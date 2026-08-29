// spec: gate-sdk/SPEC.md §check-template-registry-parity — a kit's shipped `.list` registry
// template names exactly the artifacts of its sibling directory, both directions
use crate::gates::smoke_entry_guard::{kit_name, scan_root};
use crate::proc;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gates_list_members`: a comment-and-blank line filter
// over a `.list` file. A content grammar rather than a corpus derivation, so criterion 6 does
// not reach it and this is the rule itself, not a second copy of one.
pub fn list_members(text: &str) -> Vec<String> {
    text.lines()
        .filter(|l| {
            let t = l.trim_start_matches([' ', '\t']);
            !(t.is_empty() || t.starts_with('#'))
        })
        .map(String::from)
        .collect()
}

// spec: gate-sdk/SPEC.md §check-template-registry-parity — native dispatch is a shipping mechanism:
// a kit whose registry members the binary carries names its sibling directory here, and that entry
// is what holds the template in population once no `.sh` file answers the registry any more.
pub type NativeRegistry = (&'static str, &'static str, fn() -> Vec<&'static str>);

pub const NATIVE_REGISTRIES: &[NativeRegistry] = &[(
    crate::emit::kpi::REGISTRY_KIT,
    crate::emit::kpi::REGISTRY_DIR,
    crate::emit::kpi::names,
)];

fn native_members(kit: &str, dir_base: &str) -> Option<Vec<String>> {
    NATIVE_REGISTRIES
        .iter()
        .find(|(k, d, _)| *k == kit && *d == dir_base)
        .map(|(_, _, f)| f().into_iter().map(String::from).collect())
}

fn sorted_unique(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

// spec: gate-sdk/SPEC.md §check-template-registry-parity — the two `comm` arms, computed by set
// membership rather than by a sorted merge: `sort`'s collation is locale-dependent and a set
// difference is not
fn only_in(a: &[String], b: &[String]) -> Vec<String> {
    a.iter().filter(|x| !b.contains(x)).cloned().collect()
}

pub fn run(args: &[String]) -> i32 {
    let root = match scan_root(args, "check-template-registry-parity") {
        Some(r) => r,
        None => return 2,
    };
    if !Path::new(&root).is_dir() {
        eprintln!("check-template-registry-parity: root not found: {}", root);
        return 2;
    }

    let kit_roots = match walk::kit_roots() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-template-registry-parity: {}", e);
            return 2;
        }
    };
    if kit_roots.is_empty() {
        eprintln!("check-template-registry-parity: no kit roots enumerated");
        return 2;
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — every kit root's `templates/*.list` listing
    // goes through one `glob_files` call anchored at the scan root, so the member observes a
    // single walk root rather than one per configured kit
    let mut relative: Vec<String> = Vec::new();
    let mut absolute: Vec<String> = Vec::new();
    for raw in &kit_roots {
        let r = raw.trim_end_matches('/');
        if r.starts_with('/') {
            absolute.push(r.to_string());
        } else {
            relative.push(format!("{}/templates/*.list", r));
        }
    }
    let mut templates: Vec<String> = Vec::new();
    if !relative.is_empty() {
        match walk::glob_files(Path::new(&root), &relative) {
            Ok(hits) => templates.extend(hits.iter().map(|p| p.display().to_string())),
            Err(e) => {
                eprintln!(
                    "check-template-registry-parity: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        }
    }
    for r in &absolute {
        match walk::glob_files(Path::new(r), &["templates/*.list".to_string()]) {
            Ok(hits) => templates.extend(hits.iter().map(|p| p.display().to_string())),
            Err(e) => {
                eprintln!(
                    "check-template-registry-parity: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        }
    }

    let mut findings: Vec<String> = Vec::new();
    let mut served: Vec<(String, String)> = Vec::new();
    let mut registries = 0usize;
    let mut skipped = 0usize;

    for tpl in &templates {
        // spec: gate-sdk/SPEC.md §check-template-registry-parity — the kit root and the sibling
        // directory are read back off the template's own path, which is what lets the listing
        // above be one call: `<kit-root>/templates/<name>.list` names both.
        let cut = match tpl.rfind("/templates/") {
            Some(i) => i,
            None => continue,
        };
        let abs = &tpl[..cut];
        let kit = kit_name(abs);
        let base = &tpl[cut + "/templates/".len()..];
        let dir = format!("{}/{}", abs, base.trim_end_matches(".list"));

        // spec: gate-sdk/SPEC.md §check-template-registry-parity — the population predicate: a
        // sibling directory of kit-shipped artifacts, **or** native dispatch of that registry;
        // neither is a skip-and-count, and that section owns why a widening needs both
        let dir_base = base.trim_end_matches(".list");
        let native = native_members(kit, dir_base);
        if !Path::new(&dir).is_dir() && native.is_none() {
            skipped += 1;
            continue;
        }
        let tpl_text = match std::fs::read(tpl) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(_) => {
                eprintln!(
                    "check-template-registry-parity: template not readable: {}",
                    tpl
                );
                return 2;
            }
        };
        let mut shipped: Vec<String> = Vec::new();
        if Path::new(&dir).is_dir() {
            if !walk::dir_readable(Path::new(&dir)) {
                eprintln!(
                    "check-template-registry-parity: sibling directory not readable: {}",
                    dir
                );
                return 2;
            }
            let completed = match proc::run("git", &["-C", &dir, "ls-files", "--", "*.sh"]) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("check-template-registry-parity: {}", e);
                    return 2;
                }
            };
            let listing = match completed.stdout() {
                Some(o) => String::from_utf8_lossy(o).into_owned(),
                None => {
                    eprintln!(
                        "check-template-registry-parity: git ls-files failed under {}",
                        dir
                    );
                    return 2;
                }
            };
            shipped.extend(
                listing
                    .lines()
                    .filter(|f| !f.is_empty() && !f.contains('/'))
                    .map(|f| f.strip_suffix(".sh").unwrap_or(f).to_string()),
            );
        }
        // spec: gate-sdk/SPEC.md §check-template-registry-parity — a natively dispatched member is
        // shipped: it answers the registry name at the same transition a `.sh` file did, so the two
        // mechanisms join into one shipped set rather than the second replacing the first
        if let Some(n) = &native {
            shipped.extend(n.iter().cloned());
        }
        registries += 1;
        let rel_tpl = tpl
            .strip_prefix(&format!("{}/", root))
            .unwrap_or(tpl)
            .to_string();

        let shipped = sorted_unique(shipped);
        let registered = sorted_unique(list_members(&tpl_text));

        // assertion A: every shipped artifact is registered
        for m in only_in(&shipped, &registered) {
            findings.push(format!(
                "{}: {} does not register shipped artifact: {}",
                kit, rel_tpl, m
            ));
        }
        // assertion B: every registry line resolves to a shipped artifact
        for m in only_in(&registered, &shipped) {
            findings.push(format!(
                "{}: {} registers a name no shipped artifact answers: {}",
                kit, rel_tpl, m
            ));
        }
        served.push((kit.to_string(), dir_base.to_string()));
    }

    // spec: gate-sdk/SPEC.md §check-template-registry-parity — assertion C, the widening's own
    // guard: where the declaring kit **is** in this scan's root set, its registry template must have
    // been reached. A declaration outliving its template is how the widened predicate would come to
    // hold a template nothing ships; scoping the assertion to the enumerated roots is what keeps a
    // sandboxed fixture, which vendors neither the kit nor its template, out of it.
    for (kit, dir_base, _) in NATIVE_REGISTRIES {
        let vendored = kit_roots
            .iter()
            .any(|r| r.trim_end_matches('/').rsplit('/').next() == Some(*kit));
        if vendored && !served.iter().any(|(k, d)| k == kit && d == dir_base) {
            findings.push(format!(
                "{}: the binary declares native dispatch for a {}.list registry, but the scan reached no such template",
                kit, dir_base
            ));
        }
    }

    if !findings.is_empty() {
        println!("check-template-registry-parity: a kit's shipped registry template is out of parity with the directory it registers:");
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: a shipped registry template names the kit's whole bundled set — add the");
        println!("        missing line (the consumer prunes its own copy, the kit ships all of it),");
        println!("        or drop the line whose artifact the kit no longer ships. An untracked file");
        println!("        is not shipped and forces nothing: commit it first.");
        return 1;
    }

    println!(
        "TEMPLATE-REGISTRY-PARITY: clean ({} shipped registry template(s) in name-set parity with the sibling directory each registers; {} .list template(s) with neither a sibling directory nor native dispatch skipped)",
        registries, skipped
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-template-registry-parity — the widening's fixture registry is
    // a transcription of a derived roster, so it is pinned to that roster here: a member added to
    // the binary without its fixture line would otherwise red the pair with no statement of why
    #[test]
    fn the_widening_fixture_registers_exactly_the_natively_dispatched_roster() {
        let tpl = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../gate-sdk/gate-tests/check-template-registry-parity/good/drift-kit/templates/kpis.list");
        let text = std::fs::read_to_string(&tpl)
            .unwrap_or_else(|e| panic!("cannot read {}: {}", tpl.display(), e));
        let registered = sorted_unique(list_members(&text));
        let dispatched = sorted_unique(
            native_members(crate::emit::kpi::REGISTRY_KIT, crate::emit::kpi::REGISTRY_DIR)
                .expect("the native registry declaration went missing"),
        );
        assert_eq!(
            registered, dispatched,
            "the good/ fixture's registry and the binary's dispatched roster disagree — add the \
             member's line to the fixture and to the kit's shipped templates/kpis.list"
        );
    }

    #[test]
    fn the_line_grammar_drops_comments_and_blanks_and_keeps_the_rest_verbatim() {
        let t = "# header\n\n  \nwidget-one\n  widget-two\n\t# indented comment\n";
        assert_eq!(list_members(t), vec!["widget-one", "  widget-two"]);
    }

    #[test]
    fn a_set_difference_is_both_directions_and_not_a_sorted_merge() {
        let a: Vec<String> = ["one", "two"].iter().map(|s| s.to_string()).collect();
        let b: Vec<String> = ["two", "three"].iter().map(|s| s.to_string()).collect();
        assert_eq!(only_in(&a, &b), vec!["one"]);
        assert_eq!(only_in(&b, &a), vec!["three"]);
    }
}
