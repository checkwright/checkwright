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

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one field's value, empty when the field is
// absent; never an error on a missing field
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

// spec: gate-sdk/SPEC.md §lib/gate.sh — the couples-knob sentinel: the name a member declares in
// place of knobs written on the descriptor corpus rather than in its own entry
pub const EVERY_COUPLES_KNOB: &str = "@every-couples-knob";

// spec: gate-sdk/SPEC.md §lib/gate.sh — the sentinel's expansion for a caller already holding the
// resolve dirs: every `knob:` name the descriptor corpus carries, sorted once each
pub fn couples_knob_names(resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let mut out = corpus_knob_tokens(resolve_dirs)?;
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
                tokens.extend(
                    v.split(',')
                        .filter_map(|t| t.strip_prefix("knob:"))
                        .map(|r| crate::knobs::reference(r).0.to_string()),
                );
            }
        }
    }
    Ok(tokens)
}

fn reach_kits(name: &str, kits: &mut Vec<&'static str>, seen: &mut Vec<&'static str>) {
    let bare = name.trim_end_matches('*');
    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — an environment-only name is no knob file's
    let Some(kit) = crate::knobs::owner(bare).filter(|k| !k.is_env_only(bare)) else {
        return;
    };
    if !kits.contains(&kit.root) {
        kits.push(kit.root);
    }
    let family = name.ends_with('*');
    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a declared family reaches the kits its
    // derivation reads, as a row reaches its inputs
    if family {
        for f in kit.families.iter().filter(|f| f.prefix == bare) {
            for input in f.inputs {
                reach_kits(input, kits, seen);
            }
        }
    }
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
            for t in couples_knob_names(resolve_dirs)? {
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

// spec: gate-sdk/SPEC.md §Reading a `couples=` field's reach — the field's one matcher, which every
// reader asking what an expanded token reaches calls by this name
pub fn couple_matches(path: &str, token: &str) -> bool {
    crate::walk::pattern_match(token, path)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a `knob:` member's covering string pattern; a
// `kit:<glob>` member passes unconverted to the `kit:` pass
fn covering_pattern(member: &str) -> String {
    if member.starts_with("kit:") {
        return member.to_string();
    }
    let collapsed = member.replace("**/", "*");
    if collapsed.starts_with('*') {
        collapsed
    } else {
        format!("*{}", collapsed)
    }
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the couples/trigger expansion every reader
// shares: two passes in a fixed order, `knob:` then `kit:` over the result, one pass each
pub fn expand_couples(field: &str, kit_roots_rel: &[String]) -> Result<String, String> {
    let mut once: Vec<String> = Vec::new();
    for tok in field.split(',') {
        match tok.strip_prefix("knob:") {
            Some(name) => {
                // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a `<NAME>.<field>` token expands to
                // the field's members across the elements, and a bare token on a packed knob is refused
                let members = match crate::knobs::reference(name) {
                    (knob, Some(field)) => crate::knobs::project(knob, field),
                    (knob, None) => {
                        crate::knobs::refuse_whole_packed(knob).and_then(|_| crate::walk::knob_array(knob))
                    }
                };
                for m in members.map_err(|e| {
                    format!(
                        "couples token 'knob:{}' could not be resolved: {} — a knob token expands \
                         to the knob's members, and an empty expansion would be a lost trigger; \
                         treating as failure (not clean).\n  help: name a knob a static kit declares, \
                         and declare the '{}' sentinel on the expanding member",
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
                    once.push(covering_pattern(&m));
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

    // spec: gate-sdk/SPEC.md §Reading a `couples=` field's reach — the one semantics: `*` and `?`
    // cross `/`, and `**` is no more than `*`
    #[test]
    fn the_couples_matcher_is_a_slash_spanning_string_match() {
        assert!(couple_matches("native/src/emit/mod.rs", "native/src/*.rs"));
        assert!(!couple_matches("a/b.rs", "a/**/b.rs"));
        assert!(couple_matches("a/b", "a?b"));
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — each knob member expands to its covering
    // pattern, and a member already leading with `*` expands unchanged
    #[test]
    fn a_knob_member_expands_to_the_pattern_that_covers_it() {
        let knobs = crate::knobenv::lock();
        let d = scratch_corpus(
            &knobs,
            "cover",
            "CANON_KIT_MANIFEST_FILES[] = **/*.sh\nCANON_KIT_MANIFEST_FILES[] = SPEC.md\n\
             CANON_KIT_MANIFEST_FILES[] = templates/*.md\nCANON_KIT_MANIFEST_FILES[] = */README.md\n",
        );
        let roots = vec!["gate-sdk".to_string()];
        let got = expand_couples("knob:CANON_KIT_MANIFEST_FILES", &roots).expect("resolvable");
        assert_eq!(got, "**.sh,*SPEC.md,*templates/*.md,*/README.md");
        for (path, tok) in [
            ("x.sh", "**.sh"),
            ("a/b/x.sh", "**.sh"),
            ("docs/gate-sdk/SPEC.md", "*SPEC.md"),
            ("lifecycle-kit/templates/scope.md", "*templates/*.md"),
        ] {
            assert!(couple_matches(path, tok), "{} must cover {}", tok, path);
        }
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a scratch gates dir holding one canon-kit
    // knob file, so a `knob:` token names a static knob with a value this test controls
    fn scratch_corpus(knobs: &crate::knobenv::KnobEnv, tag: &str, body: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("checkwright-couples-{}.{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        std::fs::write(d.join("canon-config.knobs"), body).expect("write");
        knobs.set("GATE_SDK_GATES_DIR", &d.display().to_string());
        knobs.remove("CANON_KIT_KNOB_FILE");
        crate::knobs::reset(knobs);
        d
    }

    fn unscratch(knobs: &crate::knobenv::KnobEnv, d: &Path) {
        knobs.remove("GATE_SDK_GATES_DIR");
        crate::knobs::reset(knobs);
        let _ = std::fs::remove_dir_all(d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — the resolution order: `knob:` first, then
    // `kit:` over the result, so a knob member spelled `kit:<glob>` composes; and the union is a
    // union, so a descriptor's own literals survive beside the knob's members.
    #[test]
    fn a_knob_token_expands_first_and_a_kit_member_then_composes() {
        let knobs = crate::knobenv::lock();
        let d = scratch_corpus(
            &knobs,
            "compose",
            "CANON_KIT_MANIFEST_FILES[] = CLAUDE.md\nCANON_KIT_MANIFEST_FILES[] = kit:SPEC.md\n",
        );
        let roots = vec!["gate-sdk".to_string()];
        assert_eq!(
            expand_couples("*SPEC*.md,knob:CANON_KIT_MANIFEST_FILES", &roots).expect("resolvable"),
            "*SPEC*.md,*CLAUDE.md,gate-sdk/SPEC.md"
        );
        std::fs::write(d.join("canon-config.knobs"), "CANON_KIT_MANIFEST_FILES =\n").expect("write");
        crate::knobs::reset(&knobs);
        assert_eq!(
            expand_couples("CLAUDE.md,knob:CANON_KIT_MANIFEST_FILES", &roots).expect("resolved-empty"),
            "CLAUDE.md",
            "a consumer's empty knob expands to nothing, because the gate then scans nothing either"
        );
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a packed knob is addressed by a declared field,
    // read through its one parser with a short element's missing fields empty, and never whole
    #[test]
    fn a_packed_knob_expands_by_its_declared_field_and_is_refused_whole() {
        let knobs = crate::knobenv::lock();
        let d = scratch_corpus(
            &knobs,
            "packed",
            "CANON_KIT_EMBED_LANGS[] = rs|rust,rs|*.rs\n\
             CANON_KIT_EMBED_LANGS[] = make|make|Makefile,**/build/*.mk\n\
             CANON_KIT_EMBED_LANGS[] = bare\n",
        );
        let roots = vec!["gate-sdk".to_string()];
        assert_eq!(
            expand_couples("*SPEC*.md,knob:CANON_KIT_EMBED_LANGS.file-globs", &roots).expect("a declared field"),
            "*SPEC*.md,*.rs,*Makefile,*build/*.mk"
        );
        for refused in [
            "knob:CANON_KIT_EMBED_LANGS",
            "knob:CANON_KIT_EMBED_LANGS.no-such-field",
            "knob:CANON_KIT_SPEC_NAME.file-globs",
        ] {
            assert!(expand_couples(refused, &roots).is_err(), "'{}' expanded rather than refusing", refused);
        }
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the sentinel is spelled in the crate alone, and its
    // expansion is every `knob:` name the descriptor corpus carries, each a static kit's
    #[test]
    fn the_couples_knob_sentinel_expands_to_the_static_names_the_corpus_carries() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let dirs = repo_check_dirs(&repo);
        let names = couples_knob_names(&dirs).expect("the descriptor corpus is readable");
        assert!(
            !names.is_empty(),
            "the descriptor corpus names no knob token, so the expansion held over nothing — read it \
             as unverified rather than as clean"
        );
        for n in &names {
            assert!(crate::knobs::owner(n).is_some(), "the knob token {} names no static kit's knob", n);
        }
        let mut sorted = names.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(names, sorted, "the expansion is not deterministic");
    }

    // spec: gate-sdk/SPEC.md §run-gates — this repo's resolve dirs, derived in the crate: the gates
    // dir, then each kit root's `checks/`, absolute
    fn repo_check_dirs(repo: &Path) -> Vec<String> {
        let sdk = crate::walk::normalize_abs(&repo.join("gate-sdk").display().to_string());
        let roots: Vec<String> = crate::walk::kit_roots_rel_from(&sdk, "")
            .expect("the kit roots derive")
            .into_iter()
            .map(|r| repo.join(r).display().to_string())
            .collect();
        resolve_dirs(&repo.join(crate::knobs::GATES_DIR_DEFAULT).display().to_string(), &roots)
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — the derivation over every registry member:
    // a declared static name yields its kit's file, and the sentinel yields exactly the kits the
    // corpus's tokens name beside the member's own
    #[test]
    fn every_member_derives_the_knob_files_its_declaration_reaches() {
        let knobs = crate::knobenv::lock();
        knobs.remove("GATE_SDK_GATES_DIR");
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let dirs = repo_check_dirs(&repo);
        let file = |k: &crate::knobs::Kit| {
            format!("{}/{}-config.knobs", crate::knobs::GATES_DIR_DEFAULT, k.stem())
        };
        let corpus: Vec<&str> = couples_knob_names(&dirs)
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
                .map(|n| n.trim_end_matches('*'))
                .filter_map(|n| crate::knobs::owner(n).filter(|k| !k.is_env_only(n)).map(|k| k.root))
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

    // spec: gate-sdk/SPEC.md §Fail-closed contract — each refusal the knob token carries: a name no
    // static kit owns, a nested token, and a member unrepresentable after expansion. None may
    // degrade to an empty expansion, because an empty expansion is a lost trigger.
    #[test]
    fn every_knob_token_refusal_is_an_error_and_never_an_empty_expansion() {
        let knobs = crate::knobenv::lock();
        let roots = vec!["gate-sdk".to_string()];
        assert!(expand_couples("knob:PROBE_ABSENT", &roots).is_err());
        for bad in ["knob:PROBE_NEST", "a,b", "has space"] {
            let d = scratch_corpus(&knobs, "bad", &format!("CANON_KIT_MANIFEST_FILES[] = {}\n", bad));
            assert!(
                expand_couples("knob:CANON_KIT_MANIFEST_FILES", &roots).is_err(),
                "member {:?} must refuse",
                bad
            );
            unscratch(&knobs, &d);
        }
    }
}
