// spec: gate-sdk/SPEC.md §check-root-tiering — the repo root holds only allowlisted orientation
// entries; workflow machinery stays under the configured dirs
use crate::gates::template_registry_parity::list_members;
use crate::{fresh, proc, walk};

// spec: gate-sdk/SPEC.md §check-root-tiering — the fallback orientation set, generic by
// construction plus the two configured document knobs: the consumer-shaped roster is the
// optional allowlist file, never a literal the crate carries (the graph-vocab.sh pattern)
fn builtin_allow(queue: &str, agent: &str) -> Vec<String> {
    vec![
        "README.md".to_string(),
        "LICENSE".to_string(),
        queue.to_string(),
        agent.to_string(),
        ".gitignore".to_string(),
        "SPEC-*.md".to_string(),
    ]
}

// spec: gate-sdk/SPEC.md §check-root-tiering — an allowlist entry matches as bash's
// `[[ "$e" == $p ]]` does: an exact name, or a glob over the whole entry
fn covered(allow: &[String], entry: &str) -> bool {
    allow.iter().any(|p| walk::pattern_match(p, entry))
}

pub fn run(args: &[String]) -> i32 {
    if args.len() > 2 {
        eprintln!("check-root-tiering: unexpected argument: {}", args[2]);
        return 2;
    }

    let allowfile = match args.first() {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("GATE_SDK_ROOT_ALLOWLIST") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-root-tiering: {}", e);
                return 2;
            }
        },
    };
    let scanroot = args.get(1).cloned().unwrap_or_else(|| ".".to_string());

    // spec: gate-sdk/SPEC.md §check-root-tiering — the tracked set is the subject, so a
    // non-repo cwd is fail-closed: there is no listing to be clean about
    match proc::run("git", &["rev-parse", "--git-dir"]) {
        Ok(c) if c.stdout().is_some() => {}
        Ok(_) => {
            eprintln!(
                "check-root-tiering: not a git repository — cannot enumerate tracked root entries"
            );
            return 2;
        }
        Err(e) => {
            eprintln!("check-root-tiering: {}", e);
            return 2;
        }
    }

    let (allow, src) = if std::path::Path::new(&allowfile).is_file() {
        match std::fs::read_to_string(&allowfile) {
            Ok(t) => (list_members(&t), allowfile.clone()),
            Err(_) => {
                eprintln!("check-root-tiering: allowlist not readable: {}", allowfile);
                return 2;
            }
        }
    } else {
        // spec: gate-sdk/SPEC.md §check-root-tiering — an absent allowlist is the
        // not-yet-curated state, which degrades to the built-in set rather than refusing
        let queue = match walk::knob_scalar("GATE_SDK_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-root-tiering: {}", e);
                return 2;
            }
        };
        let agent = match walk::knob_scalar("GATE_SDK_AGENT_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-root-tiering: {}", e);
                return 2;
            }
        };
        (
            builtin_allow(&queue, &agent),
            "built-in minimal orientation set".to_string(),
        )
    };

    let prefix = if scanroot == "." {
        String::new()
    } else {
        format!("{}/", scanroot.trim_end_matches('/'))
    };

    let ls = match proc::run("git", &["ls-files", "--", &scanroot]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("check-root-tiering: {}", e);
            return 2;
        }
    };
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            eprintln!(
                "check-root-tiering: {}",
                fresh::fail_closed("git-ls-files", ls.code())
            );
            return 2;
        }
    };

    let mut seen: Vec<String> = Vec::new();
    let mut stray: Vec<String> = Vec::new();
    for path in listing.lines() {
        if path.is_empty() {
            continue;
        }
        let rest = path.strip_prefix(prefix.as_str()).unwrap_or(path);
        let entry = rest.split('/').next().unwrap_or(rest).to_string();
        if seen.contains(&entry) {
            continue;
        }
        if !covered(&allow, &entry) {
            stray.push(entry.clone());
        }
        seen.push(entry);
    }

    if !stray.is_empty() {
        println!(
            "check-root-tiering: tracked top-level entry not in the allowlist ({}) —",
            src
        );
        println!("the repo root is the orientation surface; workflow machinery belongs under the");
        println!("configured workflow/gates dirs, not scattered at root:");
        for s in &stray {
            println!("  {}", s);
        }
        println!("  help: move the entry under an existing dir, or — if it is a deliberate new");
        println!(
            "        root surface — add it to {} in the same commit.",
            allowfile
        );
        return 1;
    }

    println!(
        "ROOT-TIERING: clean ({} tracked top-level entr(y|ies) all allowlisted via {})",
        seen.len(),
        src
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-root-tiering — entries match as exact names or globs, the
    // whole-entry rule bash's `[[ == ]]` applies; the fixture pair fixes the roster, this fixes
    // the two spellings a roster of exact names alone would never exercise
    #[test]
    fn an_allowlist_entry_matches_exactly_or_as_a_whole_entry_glob() {
        let allow = builtin_allow("TASK-QUEUE.md", "AGENTS.md");
        assert!(covered(&allow, "README.md"));
        assert!(covered(&allow, "AGENTS.md"));
        assert!(!covered(&allow, "CLAUDE.md"));
        assert!(covered(&allow, "SPEC-third-batch.md"));
        assert!(!covered(&allow, "notes/SPEC-x.md"));
    }
}
