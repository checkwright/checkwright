// spec: gate-sdk/SPEC.md §check-kit-enum — a literal hand list of >=2 kit roots sharing a glob
// must name every kit root with matching tracked files; the fix is the kit:<glob> token
use crate::fresh;
use crate::proc;
use crate::registry;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the first `# graph: ` line's `couples=`
// field, split on the shell's own unquoted word split
fn couples_of(text: &str) -> Option<String> {
    let man = fresh::file_lines(text)
        .into_iter()
        .find(|l| l.starts_with("# graph: "))?;
    man["# graph: ".len()..]
        .split_whitespace()
        .find_map(|kv| kv.strip_prefix("couples=").map(String::from))
}

pub fn run(args: &[String]) -> i32 {
    let gates_dir = match args.first() {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("GATE_SDK_GATES_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-kit-enum: {}", e);
                return 2;
            }
        },
    };
    let list = format!("{}/gates.list", gates_dir);
    if !Path::new(&list).is_file() {
        eprintln!("check-kit-enum: no registry at {}", list);
        return 2;
    }

    let repo_root = match proc::run("git", &["rev-parse", "--show-toplevel"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
        .filter(|s| !s.is_empty())
    {
        Some(r) => r,
        None => {
            eprintln!("check-kit-enum: not a git repository — cannot test tracked kit files");
            return 2;
        }
    };

    let kit_roots: Vec<String> = match walk::kit_roots_rel() {
        Ok(v) => v.into_iter().map(|r| r.trim_end_matches('/').to_string()).collect(),
        Err(e) => {
            eprintln!("check-kit-enum: {}", e);
            return 2;
        }
    };
    if kit_roots.is_empty() {
        eprintln!("check-kit-enum: no kit roots enumerated");
        return 2;
    }
    let mut resolve_dirs = vec![gates_dir.clone()];
    match walk::kit_roots_abs() {
        Ok(v) => resolve_dirs.extend(v.into_iter().map(|k| format!("{}/checks", k))),
        Err(e) => {
            eprintln!("check-kit-enum: {}", e);
            return 2;
        }
    }

    let listing = match std::fs::read(&list) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => {
            eprintln!("check-kit-enum: cannot read {}: {}", list, e);
            return 2;
        }
    };
    let members = registry::members(&listing);
    if members.is_empty() {
        eprintln!("check-kit-enum: no members parsed from {}", list);
        return 2;
    }

    let mut violations: Vec<String> = Vec::new();
    let mut groups_checked = 0usize;
    for m in &members {
        let Some(src) = registry::resolve(m, &resolve_dirs) else {
            eprintln!(
                "check-kit-enum: {} in {} resolves in none of: {}",
                m,
                list,
                resolve_dirs.join(" ")
            );
            return 2;
        };
        // spec: gate-sdk/SPEC.md §check-kit-enum — no manifest is check-graph's finding, not this
        // gate's, and a manifest with no couples= field names no roots to group
        let declared = match std::fs::read(&src) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(_) => continue,
        };
        let Some(couples) = couples_of(&declared) else {
            continue;
        };

        // spec: gate-sdk/SPEC.md §check-kit-enum — groups in first-seen order, where the shell
        // form's associative array had bash's hash order
        let mut named: Vec<(String, Vec<String>)> = Vec::new();
        for t in couples.split(',') {
            let Some((root, glob)) = t.split_once('/') else {
                continue;
            };
            if !kit_roots.iter().any(|r| r == root) {
                continue;
            }
            match named.iter_mut().find(|(g, _)| g == glob) {
                Some((_, roots)) => roots.push(root.to_string()),
                None => named.push((glob.to_string(), vec![root.to_string()])),
            }
        }

        for (glob, have) in &named {
            // spec: gate-sdk/SPEC.md §check-kit-enum — a single named root is not a hand list
            if have.len() < 2 {
                continue;
            }
            groups_checked += 1;
            let mut missing: Vec<String> = Vec::new();
            for r in &kit_roots {
                let spec = format!("{}/{}", r, glob);
                let out = match proc::run("git", &["-C", &repo_root, "ls-files", "--", &spec]) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("check-kit-enum: {}", e);
                        return 2;
                    }
                };
                let Some(bytes) = out.stdout() else {
                    eprintln!(
                        "check-kit-enum: {}",
                        fresh::fail_closed("git ls-files", out.code())
                    );
                    return 2;
                };
                if bytes.is_empty() || have.iter().any(|h| h == r) {
                    continue;
                }
                missing.push(r.clone());
            }
            if !missing.is_empty() {
                violations.push(format!(
                    "{} couples a '{}' hand list naming [{}] but omits [{}]",
                    m,
                    glob,
                    have.join(" "),
                    missing.join(" ")
                ));
            }
        }
    }

    if !violations.is_empty() {
        println!("check-kit-enum: gate(s) hand-list kit roots incompletely — the kit set drifted:");
        for v in &violations {
            println!("  {}", v);
        }
        println!("  help: replace the per-kit hand list with the 'kit:<glob>' couples token");
        println!("        (lib/gate.sh expands it to every gate_kit_roots member), so a kit");
        println!("        added later cannot silently fall out of the coupling.");
        return 1;
    }
    println!(
        "KIT-ENUM: clean ({} multi-kit hand-list group(s) complete; kit:<glob> keeps them so)",
        groups_checked
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_registry_reader_drops_comments_and_blanks_and_keeps_order() {
        assert_eq!(
            registry::members("# head\n\ncheck-b\n  \ncheck-a\n"),
            vec!["check-b".to_string(), "check-a".to_string()]
        );
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — the field is read off the first manifest
    // line whatever the declaration's substrate, and an absent field is not a finding
    #[test]
    fn the_couples_field_is_read_off_the_first_manifest_line_only() {
        let man = |body: &str| format!("# {}: {}", "graph", body);
        let two = format!(
            "#!/usr/bin/env bash\n{}\n{}\n",
            man("couples=a/x.sh,b/x.sh dir=one tier=precommit"),
            man("couples=z/y.sh")
        );
        assert_eq!(couples_of(&two), Some("a/x.sh,b/x.sh".to_string()));
        assert_eq!(couples_of("# spec: no manifest here\n"), None);
        assert_eq!(couples_of(&man("dir=one valve=none tier=precommit")), None);
    }
}
