// spec: gate-sdk/SPEC.md §check-test-hermetic — a bespoke test is pinned to kit defaults and a
// credential-managing smoke script cannot reach the ambient credential
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-test-hermetic — `^#[[:space:]]*hermetic-exempt:`, hand-written
// because the pattern is a kit literal the rule owns rather than a consumer-configured ERE the
// rule interprets
pub fn has_marker(text: &str, marker: &str) -> bool {
    text.lines().any(|l| {
        let Some(rest) = l.strip_prefix('#') else {
            return false;
        };
        rest.trim_start_matches([' ', '\t', '\r', '\x0b', '\x0c'])
            .starts_with(marker)
    })
}

struct Outcome {
    fail: bool,
    tests_total: usize,
    smoke_total: usize,
}

// assertion A: every bespoke gate-tests/*.test.sh sources lib/test-hermetic.sh or carries a
// `# hermetic-exempt:` marker
fn scan_tests(dirs: &[String], out: &mut Outcome) -> Result<(), i32> {
    let scan_dirs: Vec<String> = if !dirs.is_empty() {
        dirs.to_vec()
    } else {
        match walk::kit_roots_abs() {
            Ok(v) => v.into_iter().map(|k| format!("{}/gate-tests", k)).collect(),
            Err(e) => {
                eprintln!("check-test-hermetic: {}", e);
                return Err(2);
            }
        }
    };

    let mut files: Vec<String> = Vec::new();
    for d in &scan_dirs {
        match walk::glob_files(Path::new(d), &["*.test.sh".to_string()]) {
            Ok(hits) => files.extend(hits.iter().map(|p| p.display().to_string())),
            Err(e) => {
                eprintln!(
                    "check-test-hermetic: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return Err(2);
            }
        }
    }
    if files.is_empty() {
        eprintln!(
            "check-test-hermetic: no *.test.sh under: {}",
            scan_dirs.join(" ")
        );
        return Err(2);
    }

    let mut leaky: Vec<String> = Vec::new();
    for f in &files {
        out.tests_total += 1;
        let text = match std::fs::read(f) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => {
                eprintln!(
                    "check-test-hermetic: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                    f, e
                );
                return Err(2);
            }
        };
        if text.contains("lib/test-hermetic.sh") {
            continue;
        }
        if has_marker(&text, "hermetic-exempt:") {
            continue;
        }
        leaky.push(f.clone());
    }

    if !leaky.is_empty() {
        println!("check-test-hermetic: bespoke test(s) neither source lib/test-hermetic.sh nor");
        println!("carry a '# hermetic-exempt:' marker (gate-sdk/SPEC.md §check-test-hermetic — a");
        println!("test on the invoker's cwd config can green wrongly on the consumer's posture):");
        for f in &leaky {
            println!("  {}", f);
        }
        println!("  help: source the bootstrap as the test's first act —");
        println!("        source \"$(dirname \"${{BASH_SOURCE[0]}}\")/../../gate-sdk/lib/test-hermetic.sh\"");
        println!("  (per-case config overrides after the source still win by ordering), OR add a");
        println!("  '# hermetic-exempt: <reason>' line for a test that proves hermeticity otherwise.");
        out.fail = true;
    }
    Ok(())
}

// assertion B: a credential-managing smoke script (one that assigns *_CRED_FILE) must pin every
// own-kit bin call ("$SMOKE_KIT_ROOT/bin/*") to a *_CRED_FILE path, else the bin resolves its
// credential file from the ambient ~/.claude
fn scan_smoke(dirs: &[String], out: &mut Outcome) -> Result<(), i32> {
    let smoke_dirs: Vec<String> = if !dirs.is_empty() {
        dirs.to_vec()
    } else {
        match walk::kit_roots_abs() {
            Ok(v) => v
                .into_iter()
                .map(|k| format!("{}/smoke", k))
                .filter(|d| Path::new(d).is_dir())
                .collect(),
            Err(e) => {
                eprintln!("check-test-hermetic: {}", e);
                return Err(2);
            }
        }
    };

    let mut leaky: Vec<String> = Vec::new();
    for d in &smoke_dirs {
        for name in ["install.sh", "violation.sh"] {
            let f = format!("{}/{}", d, name);
            if !Path::new(&f).exists() {
                continue;
            }
            let text = match std::fs::read(&f) {
                Ok(b) => String::from_utf8_lossy(&b).into_owned(),
                Err(_) => {
                    eprintln!("check-test-hermetic: unreadable smoke script: {}", f);
                    return Err(2);
                }
            };
            out.smoke_total += 1;
            if has_marker(&text, "hermetic-exempt:") {
                continue;
            }
            if !text.contains("_CRED_FILE=") {
                continue;
            }
            for (n, line) in text.lines().enumerate() {
                if !line.contains("SMOKE_KIT_ROOT/bin/") {
                    continue;
                }
                if line.contains("_CRED_FILE=") {
                    continue;
                }
                leaky.push(format!("{}:{}", f, n + 1));
            }
        }
    }

    if !leaky.is_empty() {
        println!("check-test-hermetic: credential-managing smoke script(s) invoke an own-kit bin");
        println!("($SMOKE_KIT_ROOT/bin/…) with no *_CRED_FILE= pin on the line — the bin resolves");
        println!("its credential file from the ambient $HOME/.claude, so the smoke reads live");
        println!("credential state (gate-sdk/SPEC.md §check-test-hermetic):");
        for f in &leaky {
            println!("  {}", f);
        }
        println!("  help: prefix the invocation with a hermetic pin to an absent path, e.g.");
        println!("        <KIT>_CRED_FILE=\"$pp/absent.json\" bash \"$SMOKE_KIT_ROOT/bin/…\" …");
        println!("  (an absent path zeroes the login timestamp so no ambient auth event leaks in),");
        println!("  OR add a '# hermetic-exempt: <reason>' line for a script hermetic otherwise.");
        out.fail = true;
    }
    Ok(())
}

pub fn run(args: &[String]) -> i32 {
    // spec: gate-sdk/SPEC.md §check-test-hermetic — `--smoke` selects assertion B, any other
    // positional selects assertion A over the named dirs, and no argument runs both
    let (smoke_mode, tests_mode, rest): (bool, bool, Vec<String>) =
        match args.first().map(String::as_str) {
            Some("--smoke") => (true, false, args[1..].to_vec()),
            Some(_) => (false, true, args.to_vec()),
            None => (false, false, Vec::new()),
        };

    let mut out = Outcome {
        fail: false,
        tests_total: 0,
        smoke_total: 0,
    };

    if smoke_mode {
        if let Err(rc) = scan_smoke(&rest, &mut out) {
            return rc;
        }
    } else if tests_mode {
        if let Err(rc) = scan_tests(&rest, &mut out) {
            return rc;
        }
    } else {
        if let Err(rc) = scan_tests(&[], &mut out) {
            return rc;
        }
        if let Err(rc) = scan_smoke(&[], &mut out) {
            return rc;
        }
    }

    if out.fail {
        return 1;
    }
    println!(
        "TEST-HERMETIC: clean ({} bespoke test(s) pinned to kit defaults; {} smoke script(s) checked for ambient-credential leaks)",
        out.tests_total, out.smoke_total
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_marker_is_a_comment_leader_then_optional_space_then_the_token() {
        assert!(has_marker("# hermetic-exempt: reason\n", "hermetic-exempt:"));
        assert!(has_marker("#hermetic-exempt: reason\n", "hermetic-exempt:"));
        assert!(has_marker(
            "set -e\n#\thermetic-exempt: reason\n",
            "hermetic-exempt:"
        ));
        assert!(!has_marker(
            "  # hermetic-exempt: reason\n",
            "hermetic-exempt:"
        ));
        assert!(!has_marker(
            "echo hermetic-exempt: nope\n",
            "hermetic-exempt:"
        ));
    }
}
