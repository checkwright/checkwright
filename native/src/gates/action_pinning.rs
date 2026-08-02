// spec: gate-sdk/SPEC.md §check-action-pinning — every `uses:` ref in a scanned YAML
// file is immutable: a full 40-hex commit SHA, or a repo-local ./ path git pins at
// checkout. Ported to the binary substrate; the fixture pair is the parity oracle and
// runs against whichever substrate the member resolves to (§run-gate-tests).
use crate::walk;
use std::path::Path;

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | 0x0b | 0x0c)
}

// spec: gate-sdk/SPEC.md §check-action-pinning — the ref-bearing line grammar:
// optional indent, an optional comment leader (a commented-out `uses:` still names a
// ref a reviewer may uncomment), an optional list dash, then `uses:`.
fn uses_value(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    let mut i = 0;
    while i < b.len() && is_space(b[i]) {
        i += 1;
    }
    if i < b.len() && b[i] == b'#' {
        i += 1;
        while i < b.len() && is_space(b[i]) {
            i += 1;
        }
    }
    if i < b.len() && b[i] == b'-' {
        let save = i;
        i += 1;
        let mut n = 0;
        while i < b.len() && is_space(b[i]) {
            i += 1;
            n += 1;
        }
        if n == 0 {
            i = save;
        }
    }
    if !line.get(i..)?.starts_with("uses:") {
        return None;
    }
    i += "uses:".len();
    while i < b.len() && is_space(b[i]) {
        i += 1;
    }
    let rest = line.get(i..)?;
    let end = rest
        .bytes()
        .position(is_space)
        .unwrap_or(rest.len());
    Some(rest[..end].trim_matches(|c| c == '\'' || c == '"'))
}

// spec: gate-sdk/SPEC.md §check-action-pinning — immutable iff a repo-local ./ path
// (the checkout pins it) or a trailing @ plus exactly 40 lowercase hex.
fn is_immutable(r: &str) -> bool {
    if r.starts_with("./") {
        return true;
    }
    let b = r.as_bytes();
    if b.len() < 41 {
        return false;
    }
    let at = b.len() - 41;
    b[at] == b'@' && b[at + 1..].iter().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

pub fn run(args: &[String]) -> i32 {
    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    let root = Path::new(scanroot);
    if !root.is_dir() {
        eprintln!("check-action-pinning: scan root not found: {}", scanroot);
        return 2;
    }

    let files = match walk::find_files(root, &["yml", "yaml"]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("check-action-pinning: {} — the check could not run; treating as failure (not clean)", e);
            return 2;
        }
    };

    if files.is_empty() {
        println!(
            "ACTION-PINNING: clean (no YAML under {} — 0 uses: refs to pin)",
            scanroot
        );
        return 0;
    }

    let mut refs = 0usize;
    let mut stray: Vec<String> = Vec::new();
    for f in &files {
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "check-action-pinning: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                    f.display(),
                    e
                );
                return 2;
            }
        };
        for (n, line) in text.lines().enumerate() {
            if let Some(v) = uses_value(line) {
                if v.is_empty() {
                    continue;
                }
                refs += 1;
                if !is_immutable(v) {
                    stray.push(format!("{}:{}: {}", f.display(), n + 1, v));
                }
            }
        }
    }

    if !stray.is_empty() {
        println!("check-action-pinning: mutable action ref — a tag or branch is repointable by");
        println!("whoever owns it, so the code a run executes is not the code that was reviewed:");
        for s in &stray {
            println!("  {}", s);
        }
        println!("  help: replace the ref with the full 40-hex commit SHA the tag resolves to,");
        println!("        keeping the tag as a trailing comment (uses: owner/repo@<sha> # v1.2.3).");
        println!("        A repo-local ./ action needs no pin — the checkout already pins it.");
        return 1;
    }

    println!(
        "ACTION-PINNING: clean ({} uses: ref(s) across {} YAML file(s) under {}, all immutable)",
        refs,
        files.len(),
        scanroot
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

    #[test]
    fn line_grammar_matches_the_shell_prefix() {
        assert_eq!(uses_value("uses: a/b@v1"), Some("a/b@v1"));
        assert_eq!(uses_value("  uses: a/b@v1"), Some("a/b@v1"));
        assert_eq!(uses_value("  - uses: a/b@v1"), Some("a/b@v1"));
        assert_eq!(uses_value("  # uses: a/b@v1"), Some("a/b@v1"));
        assert_eq!(uses_value("  #uses: a/b@v1"), Some("a/b@v1"));
        assert_eq!(uses_value("  # - uses: a/b@v1"), Some("a/b@v1"));
        // a dash with no following space is not the list-item arm
        assert_eq!(uses_value("-uses: a/b@v1"), None);
        assert_eq!(uses_value("name: uses: not-a-ref"), None);
        assert_eq!(uses_value("run: echo uses:"), None);
    }

    #[test]
    fn value_stops_at_whitespace_and_sheds_quotes() {
        assert_eq!(uses_value("uses: 'a/b@v1'  # comment"), Some("a/b@v1"));
        assert_eq!(uses_value("uses: \"a/b@v1\""), Some("a/b@v1"));
        assert_eq!(uses_value("uses:"), Some(""));
    }

    #[test]
    fn immutability_is_forty_lowercase_hex_or_a_local_path() {
        assert!(is_immutable(&format!("owner/action@{}", SHA)));
        assert!(is_immutable("./.github/actions/local"));
        assert!(!is_immutable("owner/action@v5"));
        assert!(!is_immutable("owner/action@main"));
        // 39 hex is a near-miss the shell regex also rejects
        assert!(!is_immutable(&format!("owner/action@{}", &SHA[..39])));
        // uppercase hex is not the lowercase class the shell regex names
        assert!(!is_immutable(&format!("owner/action@{}", SHA.to_uppercase())));
    }
}
