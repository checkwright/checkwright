// spec: gate-sdk/SPEC.md §check-exec-bit — every tracked *.sh matching an exec-glob carries git
// index mode 100755 and every tracked *.gate descriptor carries 100644, or a by-path-invoked kit
// script degrades silently to a skipped check / failed plugin in a fresh clone
use crate::fresh;
use crate::proc;
use crate::walk;

// spec: gate-sdk/SPEC.md §check-exec-bit — the prune set exempts by path *segment*, so a
// fixture tree under any depth is skipped whole
fn pruned(path: &str, prune: &[String]) -> bool {
    path.split('/').any(|seg| prune.iter().any(|p| p == seg))
}

pub fn run(args: &[String]) -> i32 {
    let globs = match walk::knob_array("GATE_EXEC_GLOBS") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-exec-bit: {}", e);
            return 2;
        }
    };
    let prune = match walk::knob_array("GATE_EXEC_PRUNE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-exec-bit: {}", e);
            return 2;
        }
    };

    // spec: gate-sdk/SPEC.md §check-exec-bit — argument mode lints a canned `git ls-files -s`
    // dump, so a fixture is hermetic against the host repo's index
    let listing = match args.first() {
        Some(dump) => match std::fs::read(dump) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(_) => {
                eprintln!("check-exec-bit: ls-files dump not readable: {}", dump);
                return 2;
            }
        },
        None => {
            let probe = match proc::run("git", &["rev-parse", "--git-dir"]) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("check-exec-bit: {}", e);
                    return 2;
                }
            };
            if probe.stdout().is_none() {
                eprintln!("check-exec-bit: not a git repository — cannot read index modes");
                return 2;
            }
            let ls = match proc::run("git", &["ls-files", "-s"]) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("check-exec-bit: {}", e);
                    return 2;
                }
            };
            match ls.stdout() {
                Some(o) => String::from_utf8_lossy(o).into_owned(),
                None => {
                    eprintln!(
                        "check-exec-bit: {}",
                        fresh::fail_closed("git-ls-files", ls.code())
                    );
                    return 2;
                }
            }
        }
    };

    let mut bad: Vec<String> = Vec::new();
    let mut notexec: Vec<String> = Vec::new();
    let (mut count, mut dcount) = (0usize, 0usize);
    for line in fresh::file_lines(&listing) {
        if line.is_empty() {
            continue;
        }
        let mode = line.split(' ').next().unwrap_or("");
        let path = line.split_once('\t').map(|(_, p)| p).unwrap_or("");
        // spec: gate-sdk/SPEC.md §check-exec-bit — a .gate descriptor is data, never sourced and
        // never run, so it is committed non-executable; stated as its own class so "not
        // executable" cannot read as "not covered"
        if path.ends_with(".gate") {
            if pruned(path, &prune) {
                continue;
            }
            dcount += 1;
            if mode != "100644" {
                notexec.push(format!("{} (index mode {})", path, mode));
            }
            continue;
        }
        if !path.ends_with(".sh") || !globs.iter().any(|g| walk::pattern_match(g, path)) {
            continue;
        }
        if pruned(path, &prune) {
            continue;
        }
        count += 1;
        if mode != "100755" {
            bad.push(format!("{} (index mode {})", path, mode));
        }
    }

    if !bad.is_empty() {
        println!("check-exec-bit: by-path-invoked script(s) not committed executable (mode 100755) — a");
        println!("100644 script degrades silently in a fresh clone (a KPI plugin to 'n/a (plugin");
        println!("failed)', a runner-invoked preflight to a skipped check):");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: git update-index --chmod=+x <path> (and chmod +x locally), then recommit.");
        return 1;
    }

    if !notexec.is_empty() {
        println!("check-exec-bit: .gate descriptor(s) committed executable — a descriptor is data");
        println!("carrying a manifest and directives, never sourced and never run; an executable one");
        println!("invites a reader to run a file with no interpreter line:");
        for b in &notexec {
            println!("  {}", b);
        }
        println!("  help: git update-index --chmod=-x <path> (and chmod -x locally), then recommit.");
        return 1;
    }

    println!(
        "EXEC-BIT: clean ({} by-path-invoked script(s) at index mode 100755; {} .gate descriptor(s) non-executable)",
        count, dcount
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prune_set_matches_a_whole_segment_at_any_depth() {
        let p = vec!["gate-tests".to_string(), "templates".to_string()];
        assert!(pruned("kit/gate-tests/x/good/check-a.sh", &p));
        assert!(pruned("templates/check-a.sh", &p));
        assert!(!pruned("kit/gate-tests-notes/check-a.sh", &p));
        assert!(!pruned("kit/checks/check-a.sh", &p));
    }

    // spec: gate-sdk/SPEC.md §check-exec-bit — the glob set is matched with `*` spanning `/`,
    // which is bash's `[[ str == pat ]]` rather than pathname expansion
    #[test]
    fn an_exec_glob_spans_slashes_and_anchors_at_both_ends() {
        assert!(walk::pattern_match("*/checks/*.sh", "a/b/checks/check-x.sh"));
        assert!(walk::pattern_match("scripts/check-*.sh", "scripts/check-x.sh"));
        assert!(!walk::pattern_match("*/checks/*.sh", "checks/check-x.sh"));
        assert!(!walk::pattern_match("*/checks/*.sh", "a/checks/check-x.sh.bak"));
    }
}
