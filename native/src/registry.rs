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

// spec: gate-sdk/SPEC.md §Layout and configuration — the registration file's one declaration form,
// `# unregistered: <name> — <reason>`: §Consumer smoke's grammar on a second roster, both fields
// read, and invisible to `members` above because every reader of that one drops comment lines.
pub fn unregistered_declarations(text: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for l in fresh::file_lines(text) {
        let Some(rest) = l.trim_start().strip_prefix('#') else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix("unregistered:") else {
            continue;
        };
        let rest = rest.trim();
        let (name, reason) = match rest.find(char::is_whitespace) {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, ""),
        };
        let reason = reason
            .trim_start()
            .trim_start_matches('—')
            .trim_start_matches("--")
            .trim_start_matches('-')
            .trim();
        out.push((name.to_string(), reason.to_string()));
    }
    out
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

// spec: gate-sdk/SPEC.md §lib/gate.sh — the fixture-suite derivation: one `(suite, tests, checks)`
// triple per directory carrying a `gate-tests/` tree, the kit roots in order then the gates dir; the
// suite is the basename with `-` turned to `_`, and `checks` is empty where no sibling `checks/` is
pub fn fixture_suites_in(kit_roots_rel: &[String], gates_dir: &str) -> Vec<(String, String, String)> {
    let mut out: Vec<(String, String, String)> = Vec::new();
    for base in kit_roots_rel.iter().map(String::as_str).chain(std::iter::once(gates_dir)) {
        let base = base.trim_end_matches('/');
        let tests = format!("{}/gate-tests", base);
        if !Path::new(&tests).is_dir() {
            continue;
        }
        let suite = base.rsplit('/').next().unwrap_or(base).replace('-', "_");
        let checks = format!("{}/checks", base);
        let checks = if Path::new(&checks).is_dir() { checks } else { String::new() };
        out.push((suite, tests, checks));
    }
    out
}

pub fn fixture_suites() -> Result<Vec<(String, String, String)>, String> {
    Ok(fixture_suites_in(
        &crate::walk::kit_roots_rel()?,
        &crate::walk::knob_scalar("GATE_SDK_GATES_DIR")?,
    ))
}

// spec: gate-sdk/SPEC.md §The non-gate arm — `--emit fixture-suites`: one tab-separated line per triple
pub fn emit_fixture_suites(_args: &[String]) -> Result<String, String> {
    Ok(fixture_suites()?
        .into_iter()
        .map(|(s, t, c)| format!("{}\t{}\t{}\n", s, t, c))
        .collect())
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
// own substitution is the shell's; a static token contributes the bridged inputs its default reads
pub fn couples_knob_names(resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let tokens = corpus_knob_tokens(resolve_dirs)?;
    let mut out: Vec<String> = crate::knobs::bridged(tokens.iter().map(String::as_str))
        .into_iter()
        .map(str::to_string)
        .collect();
    out.sort();
    out.dedup();
    Ok(out)
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the sentinel's static half: the corpus's `knob:` names a
// static kit owns, which the member resolves from that kit's knob file rather than the bridge
pub fn couples_knob_static_names(resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = corpus_knob_tokens(resolve_dirs)?
        .into_iter()
        .filter(|t| crate::knobs::is_static(t))
        .collect();
    out.sort();
    out.dedup();
    Ok(out)
}

fn corpus_knob_tokens(resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
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
                tokens.extend(v.split(',').filter_map(|t| t.strip_prefix("knob:")).map(str::to_string));
            }
        }
    }
    Ok(tokens)
}

fn reach_kits(name: &str, kits: &mut Vec<&'static str>, seen: &mut Vec<&'static str>) {
    let bare = name.trim_end_matches('*');
    let Some(kit) = crate::knobs::owner(bare) else {
        return;
    };
    if !kits.contains(&kit.root) {
        kits.push(kit.root);
    }
    let family = name.ends_with('*');
    for row in kit.rows {
        let hit = if family { row.name.starts_with(bare) } else { row.name == bare };
        if !hit || seen.contains(&row.name) {
            continue;
        }
        seen.push(row.name);
        for input in row.inputs {
            reach_kits(input, kits, seen);
        }
    }
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a static kit's knob file is a derived couple:
// the tracked file at its default location for every static kit the member's declaration reaches,
// in the static-kit table's order; a staged member reaching one is refused, never handed the file
pub fn knob_files(member: &str, resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let Some(declared) = crate::gates::declared(member) else {
        return Ok(Vec::new());
    };
    let mut kits: Vec<&'static str> = Vec::new();
    let mut seen: Vec<&'static str> = Vec::new();
    for name in declared {
        if *name == EVERY_COUPLES_KNOB {
            for t in couples_knob_static_names(resolve_dirs)? {
                reach_kits(&t, &mut kits, &mut seen);
            }
        } else {
            reach_kits(name, &mut kits, &mut seen);
        }
    }
    if kits.is_empty() {
        return Ok(Vec::new());
    }
    let dir = crate::knobs::gates_dir();
    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a reference line in a reached kit's tracked
    // file reaches its referent's kit, read from the file now; the local overlay is never read
    for root in kits.clone() {
        let Some(kit) = crate::knobs::STATIC_KITS.iter().find(|k| k.root == root) else {
            continue;
        };
        let path = format!("{}/{}-config.knobs", dir, kit.stem());
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for e in crate::knobfile::parse(&text, &path)? {
            if let crate::knobfile::Form::Reference(other) = &e.form {
                reach_kits(other, &mut kits, &mut seen);
            }
        }
    }
    if let Some(src) = resolve(member, resolve_dirs) {
        let text = std::fs::read(&src)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        let mode = manifest_line(&text).map(|m| field(&manifest_fields(m), "mode")).unwrap_or_default();
        if mode == "staged" {
            return Err(format!(
                "{} is mode=staged and reads the knob file of {} — the staged hook passes matched \
                 paths to the gate as arguments, so a derived knob-file couple would reach it as a \
                 file to scan; treating as failure (not clean).\n  help: read no static knob from a \
                 mode=staged member, or drop mode=staged",
                member,
                kits.join(", ")
            ));
        }
    }
    Ok(crate::knobs::STATIC_KITS
        .iter()
        .filter(|k| kits.contains(&k.root))
        .map(|k| format!("{}/{}-config.knobs", dir, k.stem()))
        .collect())
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
        // spec: gate-sdk/SPEC.md §lib/gate.sh — run under the caller's own `set -euo pipefail`, which
        // `gen-pre-commit.sh` sets: a derivation that aborts there returns *nothing* rather than
        // failing, and an empty derivation bridges no knob and refuses every token. Attested.
        let shell = |snippet: &str| -> Vec<String> {
            let out = std::process::Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "set -euo pipefail; . gate-sdk/lib/gate.sh; {}",
                    snippet
                ))
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
        assert!(
            !mine.is_empty(),
            "the descriptor corpus names no knob token, so the agreement above held over nothing — \
             read it as unverified rather than as clean"
        );
        // spec: gate-sdk/SPEC.md §lib/gate.sh — the bridge's own output for the derived set, so the
        // path from a declared sentinel to an exported value is asserted end to end rather than in
        // two halves that could each pass while the join between them produces nothing
        let bridged = shell("_gate_couples_knob_bridge; env | grep '^GATE_SDK_KNOB_' | cut -d= -f1");
        for k in &mine {
            assert!(
                bridged.contains(&format!("GATE_SDK_KNOB_{}", k)),
                "the bridge resolved no value for {}, which a knob token names — an empty expansion \
                 is the lost trigger the token exists to prevent",
                k
            );
        }
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — the derivation over every registry member:
    // a declared static name yields its kit's file, and the sentinel yields exactly the kits the
    // corpus's static tokens name beside the member's own
    #[test]
    fn every_member_derives_the_knob_files_its_declaration_reaches() {
        let knobs = crate::knobenv::lock();
        knobs.remove("GATE_SDK_GATES_DIR");
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let listed = std::process::Command::new("bash")
            .arg("-c")
            .arg("set -euo pipefail; . gate-sdk/lib/gate.sh; gate_check_dirs")
            .current_dir(&repo)
            .output()
            .expect("cannot run the shell library");
        assert!(listed.status.success(), "gate_check_dirs failed");
        let dirs: Vec<String> = String::from_utf8_lossy(&listed.stdout)
            .lines()
            .filter(|l| !l.is_empty())
            .map(|d| if d.starts_with('/') { d.to_string() } else { repo.join(d).display().to_string() })
            .collect();
        let file = |k: &crate::knobs::Kit| {
            format!("{}/{}-config.knobs", crate::knobs::GATES_DIR_DEFAULT, k.stem())
        };
        let corpus: Vec<&str> = couples_knob_static_names(&dirs)
            .expect("the descriptor corpus is readable")
            .iter()
            .filter_map(|t| crate::knobs::owner(t).map(|k| k.root))
            .collect();
        assert!(
            !corpus.is_empty(),
            "the descriptor corpus names no static knob token, so the sentinel half held over nothing"
        );
        let (mut declaring, mut sentinel) = (0, 0);
        for (member, _) in crate::gates::names_with_owners() {
            let got = knob_files(member, &dirs).expect("no registry member is staged");
            let declared = crate::gates::declared(member).unwrap_or(&[]);
            let own: Vec<&str> = declared
                .iter()
                .filter_map(|n| crate::knobs::owner(n.trim_end_matches('*')).map(|k| k.root))
                .collect();
            for kit in crate::knobs::STATIC_KITS.iter().filter(|k| own.contains(&k.root)) {
                declaring += 1;
                assert!(
                    got.contains(&file(kit)),
                    "{} declares a {} knob but derives no couple on {}",
                    member,
                    kit.root,
                    file(kit)
                );
            }
            if declared.contains(&EVERY_COUPLES_KNOB) {
                sentinel += 1;
                let reached: Vec<&str> = crate::knobs::STATIC_KITS
                    .iter()
                    .filter(|k| corpus.contains(&k.root) || own.contains(&k.root))
                    .map(|k| k.root)
                    .collect();
                let referents = referent_kits(&repo, &reached);
                let want: Vec<String> = crate::knobs::STATIC_KITS
                    .iter()
                    .filter(|k| reached.contains(&k.root) || referents.contains(&k.root))
                    .map(|k| file(k))
                    .collect();
                assert_eq!(got, want, "{}'s sentinel derivation", member);
            }
        }
        assert!(declaring > 0 && sentinel > 0, "one half of the derivation held over no member");
    }

    fn referent_kits(repo: &Path, reached: &[&str]) -> Vec<&'static str> {
        let mut out: Vec<&'static str> = Vec::new();
        for kit in crate::knobs::STATIC_KITS.iter().filter(|k| reached.contains(&k.root)) {
            let path = repo.join(crate::knobs::GATES_DIR_DEFAULT).join(format!("{}-config.knobs", kit.stem()));
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            for e in crate::knobfile::parse(&text, "tracked").expect("a tracked knob file parses") {
                if let crate::knobfile::Form::Reference(other) = e.form {
                    out.extend(crate::knobs::owner(&other).map(|k| k.root));
                }
            }
        }
        out
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a reference line in a reached kit's tracked
    // file reaches its referent's kit; the same line in the local overlay reaches nothing
    #[test]
    fn a_tracked_reference_reaches_its_referents_kit_and_the_overlay_does_not() {
        let knobs = crate::knobenv::lock();
        let d = std::env::temp_dir().join(format!("checkwright-knob-refs.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        knobs.set("GATE_SDK_GATES_DIR", &d.display().to_string());
        let dirs: Vec<String> = Vec::new();
        let member = "check-trajectory-fresh";
        let lifecycle = format!("{}/lifecycle-config.knobs", d.display());
        std::fs::write(d.join("drift-config.local.knobs"), "DRIFT_KIT_STAGES[] <- LIFECYCLE_KIT_STAGES\n").expect("write");
        assert!(!knob_files(member, &dirs).expect("derives").contains(&lifecycle));
        std::fs::write(d.join("drift-config.knobs"), "DRIFT_KIT_STAGES[] <- LIFECYCLE_KIT_STAGES\n").expect("write");
        assert!(knob_files(member, &dirs).expect("derives").contains(&lifecycle));
        knobs.remove("GATE_SDK_GATES_DIR");
        let _ = std::fs::remove_dir_all(&d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a staged member reaching a static kit is a
    // refusal, because the staged hook would hand it the derived file as a path to scan
    #[test]
    fn a_staged_member_reaching_a_static_kit_is_refused() {
        let _knobs = crate::knobenv::lock();
        let d = std::env::temp_dir().join(format!("checkwright-knob-files.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let dirs = vec![d.display().to_string()];
        let member = "check-queue-sections";
        let write = |mode: &str| {
            std::fs::write(
                d.join(format!("{}.gate", member)),
                format!("# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit{}\n", mode),
            )
            .expect("write");
        };
        write("");
        let whole = knob_files(member, &dirs);
        write(" mode=staged");
        let staged = knob_files(member, &dirs);
        let _ = std::fs::remove_dir_all(&d);
        assert!(whole.is_ok_and(|f| !f.is_empty()), "the whole-tree member derives its kit's file");
        assert!(staged.is_err(), "the staged member must refuse rather than derive");
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
