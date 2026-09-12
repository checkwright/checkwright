// spec: gate-sdk/SPEC.md §lib/gate.sh — the crate-side registry layer: the `gates.list` member
// grammar, `gate_resolve`'s declaration path, the `# graph:` field read and the couples/trigger
// expansion, in one module rather than a private copy per reader
// spec: gate-sdk/SPEC.md §The non-gate arm — a universal layer, so no `.gate` descriptor couples
// it: an edit here can change every member's verdict, which is what §check-crate-arms and the
// binary's source stamp hold, exactly as they hold `walk.rs` and `proc.rs`
use crate::fresh;
use std::path::Path;

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gates_list_members`: every line that is neither blank
// nor a comment, in file order
pub fn members(text: &str) -> Vec<String> {
    fresh::file_lines(text)
        .iter()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .map(|l| (*l).to_string())
        .collect()
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_resolve`: dirs consumer-first, `.sh` beating `.gate`
// *within* a dir, so a consumer shadowing a ported member with its own shell script still wins
pub fn resolve(name: &str, dirs: &[String]) -> Option<String> {
    for d in dirs {
        for ext in ["sh", "gate"] {
            let p = format!("{}/{}.{}", d, name, ext);
            if Path::new(&p).is_file() {
                return Some(p);
            }
        }
    }
    None
}

// spec: gate-sdk/SPEC.md §run-gates — the resolve-dir order a battery reads a member's declaration
// through: the gates dir first, then each kit's `checks/`
pub fn resolve_dirs(gates_dir: &str, kit_roots: &[String]) -> Vec<String> {
    let mut dirs = vec![gates_dir.to_string()];
    for k in kit_roots {
        dirs.push(format!("{}/checks", k.trim_end_matches('/')));
    }
    dirs
}

// spec: gate-sdk/SPEC.md §port-blockers — the fixture dirs check-gate-fixture-coverage resolves,
// shared rather than duplicated, and taking its roots as a *parameter* because the gate resolves
// absolute ones where this report's evidence column must stay repo-relative.
pub fn fixture_dirs(tests_dir: &str, kit_roots: &[String]) -> Vec<String> {
    let mut dirs = vec![tests_dir.to_string()];
    for k in kit_roots {
        dirs.push(format!("{}/gate-tests", k.trim_end_matches('/')));
    }
    dirs
}

pub fn list_path(gates_dir: &str) -> String {
    format!("{}/gates.list", gates_dir)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the first `# graph: ` line's whitespace-split
// `<key>=<value>` tokens; an unknown token is the caller's to report, so this keeps them all
pub fn manifest_line(text: &str) -> Option<&str> {
    fresh::file_lines(text)
        .into_iter()
        .find(|l| l.starts_with("# graph: "))
}

pub fn manifest_fields(man: &str) -> Vec<(String, String)> {
    man.trim_start_matches("# graph: ")
        .split_whitespace()
        .map(|kv| match kv.split_once('=') {
            Some((k, v)) => (k.to_string(), v.to_string()),
            None => (kv.to_string(), String::new()),
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — `gate_manifest_field`: the value, empty when
// the field is absent; never an error on a missing field
pub fn field(fields: &[(String, String)], key: &str) -> String {
    fields
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .unwrap_or_default()
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the closed set of special `couples=` prefixes,
// read from here by every reader that recognises one, because a prefix one reader knows and another
// does not falls through as an inert literal glob
pub const COUPLES_PREFIXES: &[&str] = &["knob:", "kit:"];

// spec: gate-sdk/SPEC.md §lib/gate.sh — the couples-knob union sentinel: the name a member declares
// in place of knobs written on the descriptor corpus rather than in its own entry
pub const EVERY_COUPLES_KNOB: &str = "@every-couples-knob";

// spec: gate-sdk/SPEC.md §lib/gate.sh — the sentinel's expansion for a caller already holding the
// resolve dirs, held to `_gate_couples_knob_names`' derivation by a unit test because the bridge's
// own substitution is the shell's
pub fn couples_knob_names(resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    for d in resolve_dirs {
        let decls = crate::walk::glob_files(
            Path::new(d),
            &["*.gate".to_string(), "*.sh".to_string()],
        )?;
        for p in decls {
            let text = match std::fs::read(&p) {
                Ok(b) => String::from_utf8_lossy(&b).into_owned(),
                Err(_) => continue,
            };
            let Some(man) = manifest_line(&text) else {
                continue;
            };
            for (k, v) in manifest_fields(man) {
                if k != "couples" && k != "trigger" {
                    continue;
                }
                for name in v.split(',').filter_map(|t| t.strip_prefix("knob:")) {
                    out.push(name.to_string());
                }
            }
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the couples/trigger expansion every reader
// shares: two passes in a fixed order, `knob:` then `kit:` over the result, one pass each
pub fn expand_couples(field: &str, kit_roots_rel: &[String]) -> Result<String, String> {
    let mut once: Vec<String> = Vec::new();
    for tok in field.split(',') {
        match tok.strip_prefix("knob:") {
            Some(name) => {
                for m in crate::walk::knob_array(name).map_err(|e| {
                    format!(
                        "couples token 'knob:{}' could not be resolved: {} — a knob token expands \
                         to the knob's members, and an empty expansion would be a lost trigger; \
                         treating as failure (not clean).\n  help: the expanding member must \
                         declare the '{}' sentinel so the bridge carries every knob the descriptor \
                         corpus names",
                        name, e, EVERY_COUPLES_KNOB
                    )
                })? {
                    if m.starts_with("knob:") {
                        return Err(format!(
                            "couples token 'knob:{}' has the member '{}', itself a knob token — \
                             expansion is one pass each, so a nested token is a refusal rather \
                             than a second pass",
                            name, m
                        ));
                    }
                    if m.contains(',') || m.chars().any(char::is_whitespace) {
                        return Err(format!(
                            "couples token 'knob:{}' has the member '{}', which carries a comma or \
                             whitespace — `couples=` is comma-separated and its manifest line \
                             splits on unquoted whitespace, so such a member is unrepresentable \
                             after expansion and would truncate the trigger silently",
                            name, m
                        ));
                    }
                    once.push(m);
                }
            }
            None => once.push(tok.to_string()),
        }
    }
    let mut out: Vec<String> = Vec::new();
    for tok in &once {
        match tok.strip_prefix("kit:") {
            Some(glob) => {
                for r in kit_roots_rel {
                    out.push(format!("{}/{}", r.trim_end_matches('/'), glob));
                }
            }
            None => out.push(tok.clone()),
        }
    }
    Ok(out.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_member_line_is_neither_blank_nor_a_comment() {
        let got = members("check-a\n\n  # a comment\ncheck-b\n");
        assert_eq!(got, vec!["check-a".to_string(), "check-b".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §check-graph — a `kit:` token expands once per repo-relative kit
    // root and a plain token passes through, which is the expansion both substrates must agree on
    #[test]
    fn kit_tokens_expand_and_plain_tokens_pass_through() {
        let roots = vec!["gate-sdk".to_string(), "queue-kit/".to_string()];
        assert_eq!(
            expand_couples("scripts/gates.list,kit:*.sh", &roots).expect("no knob token"),
            "scripts/gates.list,gate-sdk/*.sh,queue-kit/*.sh"
        );
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — the resolution order: `knob:` first, then
    // `kit:` over the result, so a knob member spelled `kit:<glob>` composes; and the union is a
    // union, so a descriptor's own literals survive beside the knob's members.
    #[test]
    fn a_knob_token_expands_first_and_a_kit_member_then_composes() {
        let knobs = crate::knobenv::lock();
        knobs.set("GATE_SDK_KNOB_PROBE_CORPUS", "CLAUDE.md\tkit:SPEC.md");
        let roots = vec!["gate-sdk".to_string()];
        assert_eq!(
            expand_couples("*SPEC*.md,knob:PROBE_CORPUS", &roots).expect("resolvable"),
            "*SPEC*.md,CLAUDE.md,gate-sdk/SPEC.md"
        );
        knobs.set("GATE_SDK_KNOB_PROBE_CORPUS", "");
        assert_eq!(
            expand_couples("CLAUDE.md,knob:PROBE_CORPUS", &roots).expect("resolved-empty"),
            "CLAUDE.md",
            "a consumer's empty knob expands to nothing, because the gate then scans nothing either"
        );
        knobs.remove("GATE_SDK_KNOB_PROBE_CORPUS");
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the sentinel crosses the dispatch seam, so its two
    // spellings and the two derivations behind it are held together rather than remembered
    #[test]
    fn the_couples_knob_sentinel_and_its_derivation_agree_across_the_dispatch_seam() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let lib = repo.join("gate-sdk/lib/gate.sh");
        let text = std::fs::read_to_string(&lib)
            .unwrap_or_else(|e| panic!("cannot read {}: {}", lib.display(), e));
        assert!(
            text.contains(EVERY_COUPLES_KNOB),
            "{} no longer carries the sentinel literal {} — the binary would declare a name the \
             bridge does not substitute, and every knob token would refuse",
            lib.display(),
            EVERY_COUPLES_KNOB
        );
        let shell = |snippet: &str| -> Vec<String> {
            let out = std::process::Command::new("bash")
                .arg("-c")
                .arg(format!(". gate-sdk/lib/gate.sh; {}", snippet))
                .current_dir(&repo)
                .output()
                .expect("cannot run the shell library");
            assert!(
                out.status.success(),
                "gate-sdk/lib/gate.sh failed on {:?}: {}",
                snippet,
                String::from_utf8_lossy(&out.stderr).trim()
            );
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter(|l| !l.is_empty())
                .map(str::to_string)
                .collect()
        };
        let dirs: Vec<String> = shell("gate_check_dirs")
            .into_iter()
            .map(|d| {
                if d.starts_with('/') {
                    d
                } else {
                    repo.join(d).display().to_string()
                }
            })
            .collect();
        assert!(!dirs.is_empty(), "no resolve dir to derive over");
        let mine = couples_knob_names(&dirs).expect("the descriptor corpus is readable");
        assert_eq!(
            mine,
            shell("_gate_couples_knob_names"),
            "the two substrates read one descriptor corpus differently, so a member bridged by the \
             shell and a child bridged by the crate would see different knob sets"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — each refusal the knob token carries: an
    // unbridged knob, a nested token, and a member unrepresentable after expansion. None may
    // degrade to an empty expansion, because an empty expansion is a lost trigger.
    #[test]
    fn every_knob_token_refusal_is_an_error_and_never_an_empty_expansion() {
        let knobs = crate::knobenv::lock();
        let roots = vec!["gate-sdk".to_string()];
        knobs.remove("GATE_SDK_KNOB_PROBE_ABSENT");
        assert!(expand_couples("knob:PROBE_ABSENT", &roots).is_err());
        for bad in ["knob:PROBE_NEST", "a,b", "has space"] {
            knobs.set("GATE_SDK_KNOB_PROBE_BAD", bad);
            assert!(
                expand_couples("knob:PROBE_BAD", &roots).is_err(),
                "member {:?} must refuse",
                bad
            );
        }
        knobs.remove("GATE_SDK_KNOB_PROBE_BAD");
    }
}
