// spec: canon-kit/SPEC.md §The reference-link grammar — docs/<kit>/{SPEC,README}.md and
// docs/doctrine-kit/DOCTRINE.md are the byte-fresh projection of the docs-mirror arm backing
// on-site reference reading; a stale, missing, or orphaned mirror page reds
use crate::fresh;
use crate::walk;
use std::path::Path;

const MIRROR_NAMES: &[&str] = &["SPEC.md", "README.md", "DOCTRINE.md"];

// spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — the generator is
// a function call, not a spawn: it ported in the same unit, so there is no shell left to reach and
// fresh::emit's bash hop is retired for this member too.
fn generate(mode: &[&str], root: &str) -> Result<String, String> {
    let mut args: Vec<String> = mode.iter().map(|s| s.to_string()).collect();
    args.push("--root".to_string());
    args.push(root.to_string());
    crate::emit::docs_mirror::emit(&args)
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-docs-mirror-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = fresh::strip_trailing_slash(fresh::positional(args, 0, "."));
    if !fresh::is_dir(root) {
        return Err(format!("not a directory: {}", root));
    }
    let listed = generate(&["--list"], root)?;

    let mut bad: Vec<String> = Vec::new();
    let mut expected: Vec<String> = Vec::new();
    let mut n = 0usize;
    for src in listed.lines() {
        let dest = format!("docs/{}", src);
        expected.push(dest.clone());
        n += 1;
        let on_disk = format!("{}/{}", root, dest);
        if !Path::new(&on_disk).is_file() {
            bad.push(format!(
                "{}: missing — the generator emits it but the tree has no such mirror page",
                dest
            ));
            continue;
        }
        let emitted = generate(&["--emit", src], root)?;
        let have = fresh::read_captured(&on_disk)?;
        if emitted.trim_end_matches('\n') != have.trim_end_matches('\n') {
            bad.push(format!("{}: stale vs the docs-mirror arm's emission of {}", dest, src));
        }
    }

    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — the orphan sweep, carrying the
    // fail-closed repair this member owed: an unreadable docs tree refuses here where the
    // shell form reported no orphans. The cohort's one designed divergence.
    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — the walk prunes nothing, because
    // the shell form reached for a bare `find`: a member that never read the prune set neither
    // narrows its corpus by it nor declares it.
    let docs_root = format!("{}/docs", root);
    let mut found: Vec<String> = walk::find_with_prune(Path::new(&docs_root), &|_| false)?
        .into_iter()
        .filter(|p| {
            p.file_name()
                .and_then(|f| f.to_str())
                .map(|f| MIRROR_NAMES.contains(&f))
                .unwrap_or(false)
        })
        .map(|p| p.display().to_string())
        .collect();
    // spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — the byte order the
    // contract states, not `sort`'s locale collation
    found.sort();
    for f in &found {
        let rel = f
            .strip_prefix(&format!("{}/", root))
            .unwrap_or(f)
            .to_string();
        if !expected.contains(&rel) {
            bad.push(format!(
                "{}: orphaned — no source doc maps to this mirror page (delete it and rerun the generator)",
                rel
            ));
        }
    }

    if !bad.is_empty() {
        println!("check-docs-mirror-fresh: the on-site SPEC mirror is out of sync with its sources:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: regenerate — bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write — and stage docs/.");
        return Ok(1);
    }
    println!(
        "DOCS-MIRROR-FRESH: clean ({} mirror page(s) byte-match the docs-mirror arm; no orphans)",
        n
    );
    Ok(0)
}
