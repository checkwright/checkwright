// spec: gate-sdk/SPEC.md §check-core-files — every path in the core-files manifest exists in the
// worktree and is tracked, a `kit:` line deriving one path per kit root
use crate::fresh;
use crate::proc;
use crate::registry;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-core-files — `gate_expand_couples_var`'s expansion narrowed to
// this reader: one `<kit-root>/<token>` per root, a non-`kit:` line verbatim, and a wildcard
// token refused fail-closed rather than expanded, returned as the offending line
fn expand(lines: &[&str], roots: &[String]) -> Result<Vec<String>, String> {
    let mut paths: Vec<String> = Vec::new();
    for line in lines {
        let Some(token) = line.strip_prefix("kit:") else {
            paths.push((*line).to_string());
            continue;
        };
        if token.contains('*') || token.contains('?') || token.contains('[') {
            return Err((*line).to_string());
        }
        for r in roots {
            paths.push(format!("{}/{}", r.trim_end_matches('/'), token));
        }
    }
    Ok(paths)
}

pub fn run(args: &[String]) -> i32 {
    let default = match walk::knob_scalar("GATE_SDK_CORE_FILES_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-core-files: {}", e);
            return 2;
        }
    };
    let manifest = fresh::positional(args, 0, &default).to_string();

    if !Path::new(&manifest).exists() {
        println!(
            "CORE-FILES: clean (no manifest at {} — optional consumer config absent)",
            manifest
        );
        return 0;
    }
    let text = match std::fs::read(&manifest) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => {
            eprintln!("check-core-files: manifest not readable: {}", manifest);
            return 2;
        }
    };
    let lines: Vec<String> = registry::members(&text);
    if lines.is_empty() {
        println!("CORE-FILES: clean (manifest {} lists no paths)", manifest);
        return 0;
    }

    // spec: gate-sdk/SPEC.md §check-core-files — the kit-root set is resolved only where a
    // `kit:` line asks for it, so a token-free manifest costs no bridge read
    let roots = if lines.iter().any(|l| l.starts_with("kit:")) {
        match walk::kit_roots_rel() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-core-files: {}", e);
                return 2;
            }
        }
    } else {
        Vec::new()
    };
    let line_refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let paths = match expand(&line_refs, &roots) {
        Ok(p) => p,
        Err(line) => {
            eprintln!(
                "check-core-files: kit: token carries a wildcard in {}: {}",
                manifest, line
            );
            eprintln!("  help: this manifest requires every expanded path to exist and be tracked,");
            eprintln!("        which a wildcard cannot express — name an exact per-kit basename");
            eprintln!("        (e.g. kit:SPEC.md), or hand-list the paths.");
            return 2;
        }
    };

    let probe = match proc::run("git", &["rev-parse", "--git-dir"]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("check-core-files: {}", e);
            return 2;
        }
    };
    if probe.stdout().is_none() {
        eprintln!("check-core-files: not a git repository — cannot verify tracked status");
        return 2;
    }

    let (mut missing, mut untracked) = (Vec::new(), Vec::new());
    let mut present = 0usize;
    for p in &paths {
        if !Path::new(p).exists() {
            missing.push(p.clone());
            continue;
        }
        let tracked = proc::run("git", &["ls-files", "--error-unmatch", "--", p])
            .map(|c| c.stdout().is_some())
            .unwrap_or(false);
        if tracked {
            present += 1;
        } else {
            untracked.push(p.clone());
        }
    }

    if !missing.is_empty() || !untracked.is_empty() {
        println!(
            "check-core-files: core file(s) listed in {} but missing or untracked",
            manifest
        );
        println!("(the silent-deletion class downstream gates catch only incidentally):");
        for p in &missing {
            println!("  missing:   {}", p);
        }
        for p in &untracked {
            println!("  untracked: {}", p);
        }
        println!("  help: restore the file (git checkout / git add), or — if the removal is");
        println!(
            "        intentional — delete its line from {} in the same commit.",
            manifest
        );
        return 1;
    }

    println!(
        "CORE-FILES: clean ({} manifest path(s) present and tracked in {})",
        present, manifest
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kit_token_derives_one_path_per_root_and_a_plain_line_passes_through() {
        let roots = vec!["alpha-kit".to_string(), "beta-kit/".to_string()];
        assert_eq!(
            expand(&["kit:SPEC.md", "plain.txt"], &roots),
            Ok(vec![
                "alpha-kit/SPEC.md".to_string(),
                "beta-kit/SPEC.md".to_string(),
                "plain.txt".to_string(),
            ])
        );
    }

    // spec: gate-sdk/SPEC.md §check-core-files — one expansion, two readers, one stated
    // restriction: this reader requires each expanded path to exist, so a glob is refused
    #[test]
    fn every_wildcard_metacharacter_is_refused_and_names_its_line() {
        for tok in ["kit:checks/*.sh", "kit:SPEC?.md", "kit:[ab].md"] {
            assert_eq!(expand(&[tok], &["a-kit".to_string()]), Err(tok.to_string()));
        }
        assert!(expand(&["scripts/*.sh"], &["a-kit".to_string()]).is_ok());
    }

    #[test]
    fn a_token_free_manifest_needs_no_root_set() {
        assert_eq!(
            expand(&["a.md", "b/c.md"], &[]),
            Ok(vec!["a.md".to_string(), "b/c.md".to_string()])
        );
    }
}
