// spec: site-kit/SPEC.md §check-docs-cname-parity — the docs/CNAME host is the single gated
// source of truth for the docs host; no tracked file names a configured host alias other than
// that host in a URL
use crate::fresh;
use crate::proc;
use crate::walk;
use std::path::Path;

// spec: site-kit/SPEC.md §check-docs-cname-parity — `grep -I`: a file carrying a NUL byte is
// binary and is skipped rather than scanned, so an artifact cannot report a host it never spells
fn binary(bytes: &[u8]) -> bool {
    bytes.contains(&0)
}

// spec: site-kit/SPEC.md §check-docs-cname-parity — `grep -oE '://[A-Za-z0-9.-]+'`: every
// occurrence on the line, each yielding its own output record
fn url_hosts(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 3 <= b.len() {
        if &b[i..i + 3] == b"://" {
            let mut j = i + 3;
            while j < b.len()
                && (b[j].is_ascii_alphanumeric() || b[j] == b'.' || b[j] == b'-')
            {
                j += 1;
            }
            if j > i + 3 {
                out.push(&line[i + 3..j]);
                i = j;
                continue;
            }
        }
        i += 1;
    }
    out
}

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}", msg);
            2
        }
    }
}

fn inner(args: &[String]) -> Result<i32, String> {
    let scan_knob = walk::knob_scalar("SITE_KIT_SCAN_ROOT")
        .map_err(|e| format!("check-docs-cname-parity: {}", e))?;
    let cname_knob = walk::knob_scalar("SITE_KIT_CNAME")
        .map_err(|e| format!("check-docs-cname-parity: {}", e))?;
    let scanroot = fresh::strip_trailing_slash(fresh::positional(args, 0, &scan_knob)).to_string();
    let cname = fresh::positional(args, 1, &cname_knob).to_string();

    let probe = proc::run("git", &["rev-parse", "--git-dir"])
        .map_err(|e| format!("check-docs-cname-parity: {}", e))?;
    if probe.stdout().is_none() {
        return Err(
            "check-docs-cname-parity: not a git repository — cannot enumerate tracked files".into(),
        );
    }
    if !Path::new(&cname).is_file() {
        return Err(format!(
            "check-docs-cname-parity: CNAME not found: {}",
            cname
        ));
    }

    let raw = std::fs::read(&cname)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("check-docs-cname-parity: cannot read {}: {}", cname, e))?;
    let hlines: Vec<&str> = fresh::file_lines(&raw)
        .into_iter()
        .filter(|l| !l.trim().is_empty())
        .collect();
    if hlines.len() != 1 {
        return Err(format!(
            "check-docs-cname-parity: {} must hold exactly one host line (found {})",
            cname,
            hlines.len()
        ));
    }
    let host: String = hlines[0].chars().filter(|c| !c.is_whitespace()).collect();
    if host.is_empty() {
        return Err(format!(
            "check-docs-cname-parity: empty host in {}",
            cname
        ));
    }

    let aliases = walk::knob_array("SITE_KIT_ALIASES")
        .map_err(|e| format!("check-docs-cname-parity: {}", e))?;
    let exempt = walk::knob_array("SITE_KIT_EXEMPT_PATHS")
        .map_err(|e| format!("check-docs-cname-parity: {}", e))?;
    let prune = walk::prune_dirs().map_err(|e| format!("check-docs-cname-parity: {}", e))?;

    let ls = proc::run("git", &["ls-files", "--", &scanroot])
        .map_err(|e| format!("check-docs-cname-parity: {}", e))?;
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            return Err(format!(
                "check-docs-cname-parity: {}",
                fresh::fail_closed("git-ls-files", ls.code())
            ))
        }
    };

    let mut files: Vec<String> = Vec::new();
    for path in listing.lines() {
        if path.is_empty() || walk::path_pruned(path, &prune) {
            continue;
        }
        if exempt.iter().any(|g| walk::pattern_match(g, path)) {
            continue;
        }
        if Path::new(path).is_file() {
            files.push(path.to_string());
        }
    }

    if files.is_empty() {
        println!(
            "DOCS-CNAME-PARITY: clean (0 tracked file(s) under {}; docs host is '{}')",
            scanroot, host
        );
        return Ok(0);
    }

    let mut bad: Vec<String> = Vec::new();
    for f in &files {
        let bytes = match std::fs::read(f) {
            Ok(b) => b,
            Err(_) => continue,
        };
        if binary(&bytes) {
            continue;
        }
        let text = String::from_utf8_lossy(&bytes).into_owned();
        for (idx, line) in fresh::file_lines(&text).iter().enumerate() {
            for h in url_hosts(line) {
                if h != host && aliases.iter().any(|a| a == h) {
                    bad.push(format!(
                        "{}:{}: alias '{}' — docs host is '{}'",
                        f,
                        idx + 1,
                        h,
                        host
                    ));
                }
            }
        }
    }

    if !bad.is_empty() {
        println!("check-docs-cname-parity: tracked file(s) cite a configured host alias other than the docs/CNAME host:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: point the URL at the docs/CNAME host '{}' (a rename is a one-file edit to the CNAME", host);
        println!("        that this gate then enumerates); SITE_KIT_EXEMPT_PATHS sites are exempt.");
        return Ok(1);
    }
    println!(
        "DOCS-CNAME-PARITY: clean ({} tracked file(s) under {}; no alias but the docs host '{}' cited in a URL)",
        files.len(),
        scanroot,
        host
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: site-kit/SPEC.md §check-docs-cname-parity — the scanner takes the host run after
    // `://`, stops at the first character outside the class, and reports every occurrence
    #[test]
    fn every_url_host_on_a_line_is_reported_and_the_class_bounds_it() {
        assert_eq!(
            url_hosts("see <https://alt.example/x> and http://legacy.example:80/"),
            vec!["alt.example", "legacy.example"]
        );
        assert_eq!(url_hosts("a bare :// with no host"), Vec::<&str>::new());
        assert_eq!(url_hosts("mailto:someone@apex.example"), Vec::<&str>::new());
        assert_eq!(url_hosts("git+ssh://host-1.a.b/p"), vec!["host-1.a.b"]);
    }

    #[test]
    fn a_nul_bearing_file_is_binary_and_a_text_one_is_not() {
        assert!(binary(b"PNG\x00\x01"));
        assert!(!binary(b"https://apex.example/\n"));
    }

}
