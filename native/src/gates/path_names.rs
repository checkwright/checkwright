// spec: gate-sdk/SPEC.md §check-path-names — no tracked path has a component carrying a deny term as
// its stem
use crate::deny_terms::{self, Entry};
use crate::exit::Fault;
use crate::walk;
use std::collections::BTreeSet;

// spec: gate-sdk/SPEC.md §check-path-names — the id a hit takes when no `# rule:` line names one
const DEFAULT_ID: &str = "NAMES-DENIED";

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(fault) => {
            eprintln!("{}", fault);
            fault.status()
        }
    }
}

// spec: gate-sdk/SPEC.md §check-path-names — ASCII case folded, the component equals the term or
// opens with it followed by `-`, `_` or `.`
fn stem_hit(component: &str, term: &str) -> bool {
    let (c, t) = (component.as_bytes(), term.as_bytes());
    if t.is_empty() || c.len() < t.len() || !c[..t.len()].eq_ignore_ascii_case(t) {
        return false;
    }
    c.get(t.len()).map_or(true, |b| matches!(b, b'-' | b'_' | b'.'))
}

// spec: gate-sdk/SPEC.md §check-path-names — each path's shortest prefix whose last component
// hits, with every distinct id hitting that component; one record however many paths share it
fn records<'a>(paths: impl Iterator<Item = &'a str>, terms: &[Entry]) -> BTreeSet<(String, String)> {
    let mut out = BTreeSet::new();
    for path in paths {
        let mut end = 0;
        for component in path.split('/') {
            end += component.len();
            let ids: BTreeSet<&str> =
                terms.iter().filter(|t| stem_hit(component, &t.text)).map(|t| t.id_or(DEFAULT_ID)).collect();
            if !ids.is_empty() {
                out.extend(ids.into_iter().map(|id| (path[..end].to_string(), id.to_string())));
                break;
            }
            end += 1;
        }
    }
    out
}

fn inner(args: &[String]) -> Result<i32, Fault> {
    if !args.is_empty() {
        return Err(Fault::usage(
            "check-path-names: takes no arguments — its corpus is the tracked path set, its terms GATE_SDK_DENY_TERMS",
        ));
    }
    if walk::toplevel_opt().map_err(|e| Fault::tool(format!("check-path-names: {}", e)))?.is_none() {
        return Err(Fault::refused("check-path-names: not a git repository — cannot enumerate tracked paths"));
    }
    let term_files = deny_terms::files().map_err(|e| e.prefixed("check-path-names"))?;
    let terms = deny_terms::read(&term_files).map_err(|e| e.prefixed("check-path-names"))?;
    if terms.is_empty() {
        println!("PATH-NAMES: clean (not applicable: no deny term configured)");
        return Ok(0);
    }

    let exempt =
        walk::self_exempt_paths(&term_files, &[]).map_err(|e| Fault::refused(format!("check-path-names: {}", e)))?;
    let prune = walk::prune_dirs().map_err(|e| Fault::refused(format!("check-path-names: {}", e)))?;
    let listing = crate::listing::tracked(None, &["."])
        .map_err(|e| Fault::tool(format!("check-path-names: {}", e.text())))?;
    let paths: Vec<&str> = listing
        .iter()
        .map(String::as_str)
        .filter(|p| !walk::path_pruned(p, &prune) && !exempt.iter().any(|e| e == p))
        .collect();

    let hits = records(paths.iter().copied(), &terms);
    if !hits.is_empty() {
        println!("check-path-names: tracked path(s) carry a deny term as a component's stem:");
        for (path, id) in &hits {
            println!("{}: [{}]", path, id);
        }
        println!("  help: rename the component each record names; a directory record covers every");
        println!("        path under it. The record carries the rule id, never the term.");
        return Ok(1);
    }
    println!(
        "PATH-NAMES: clean ({} tracked path(s) read; no component carries one of the {} deny term(s) as its stem)",
        paths.len(),
        terms.len()
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(text: &str, id: Option<&str>) -> Entry {
        Entry { text: text.to_string(), id: id.map(String::from), position: 1 }
    }

    #[test]
    fn a_stem_hits_at_a_boundary_and_never_inside_a_longer_name() {
        for hit in ["foo", "FOO-cli", "foo_x", "Foo.rs", "foo.d"] {
            assert!(stem_hit(hit, "foo"), "{} should hit", hit);
        }
        for miss in ["food", "foobar.rs", "xfoo", "fo"] {
            assert!(!stem_hit(miss, "foo"), "{} should not hit", miss);
        }
    }

    #[test]
    fn a_directory_hit_is_one_record_at_its_shortest_prefix() {
        let terms = [entry("foo", None), entry("bar", Some("CODENAME")), entry("baz", Some("CODENAME"))];
        let paths = ["a/foo/x.rs", "a/foo/bar.rs", "a/bar-baz/y", "a/baz.d/z", "foo.rs"];
        let got: Vec<(String, String)> = records(paths.into_iter(), &terms).into_iter().collect();
        let want = [("a/bar-baz", "CODENAME"), ("a/baz.d", "CODENAME"), ("a/foo", "NAMES-DENIED"), ("foo.rs", "NAMES-DENIED")];
        assert_eq!(got, want.map(|(p, i)| (p.to_string(), i.to_string())));
    }
}
