// spec: installer/README.md §diff — classifies every `files` entry against the tree using the same
// hash comparison the claim makes; changed and missing are named apart because a deletion and an
// edit have different remedies. Exit status is the verdict: 0 all match, 1 at least one does not.
use super::{lock, refuse, Refusal};

fn usage() -> Vec<String> {
    vec![
        "usage: checkwright diff".to_string(),
        String::new(),
        format!(
            "Compares the tree against what init recorded in {}, using the same",
            lock::FILE
        ),
        "hash comparison the ownership claim makes. Exit status is the verdict: 0".to_string(),
        "every recorded file matches, 1 at least one has changed or gone missing.".to_string(),
    ]
}

pub fn run(args: &[String]) -> i32 {
    let lines = usage();
    let borrowed: Vec<&str> = lines.iter().map(String::as_str).collect();
    if let Some(outcome) = super::help_only(args, &borrowed) {
        return super::finish("diff", outcome);
    }
    super::finish("diff", compare())
}

fn compare() -> Result<i32, Refusal> {
    // spec: installer/README.md §init — every precondition refuses rather than warns, and is
    // checked before anything is compared: diff's subject is exactly the roster init recorded, so
    // it needs the repository that roster is in.
    let root = super::repo_root().ok_or_else(|| {
        refuse(
            "not inside a git work tree",
            "diff compares the tree against the manifest init committed there, so it needs the repository that manifest is in.",
            2,
        )
    })?;
    let path = lock::path(&root);
    if !path.is_file() {
        return Err(refuse(
            format!("no {} at {}", lock::FILE, root.display()),
            "init is the verb that makes an install, and the manifest it writes is what diff compares against. Without one there is nothing here to compare.",
            2,
        ));
    }
    let manifest = lock::Manifest::read(&path)
        .filter(lock::Manifest::schema_ok)
        .ok_or_else(|| {
            refuse(
                format!("{} carries a schema this build does not know", lock::FILE),
                "this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for.",
                2,
            )
        })?;

    // spec: installer/README.md §diff — a recorded path already off the tree is reported apart from
    // one whose content differs: the roster's exit rule means the next init silently drops a missing
    // path and rewrites it fresh, which is worth a warning before it happens rather than after.
    let (mut changed, mut missing) = (Vec::new(), Vec::new());
    let mut same = 0usize;
    let entries: Vec<(String, String, bool)> = manifest
        .files()
        .into_iter()
        .map(|(p, h)| {
            let present = root.join(&p).is_file();
            (p, h, present)
        })
        .collect();
    let present: Vec<_> = entries
        .iter()
        .filter(|e| e.2)
        .map(|e| root.join(&e.0))
        .collect();
    let mut hashes = lock::hash_all(&present).into_iter();
    for (p, h, present) in entries {
        if !present {
            missing.push(p);
        } else if hashes.next().unwrap_or_default() == h {
            same += 1;
        } else {
            changed.push(p);
        }
    }

    println!("checking {} against the tree", lock::FILE);
    if !changed.is_empty() {
        println!(
            "\nchanged ({}) — content differs from what init wrote:",
            changed.len()
        );
        for p in &changed {
            println!("  {}", p);
        }
    }
    if !missing.is_empty() {
        println!("\nmissing ({}) — recorded by init but no longer on disk; the next init will silently drop these from the roster and rewrite them fresh:", missing.len());
        for p in &missing {
            println!("  {}", p);
        }
    }

    if changed.is_empty() && missing.is_empty() {
        println!("\nDIFF: clean — {} file(s) match what init wrote.", same);
        return Ok(0);
    }
    println!(
        "\nDIFF: {} changed, {} missing, {} unchanged",
        changed.len(),
        missing.len(),
        same
    );
    Ok(1)
}

#[cfg(test)]
mod tests {
    // spec: installer/README.md §The verbs — `--help` answers on its own, outside every repository
    // precondition, and an unknown argument is a usage refusal.
    #[test]
    fn help_answers_outside_every_precondition() {
        assert_eq!(super::run(&["--help".to_string()]), 0);
        assert_eq!(super::run(&["--nope".to_string()]), 2);
    }
}
