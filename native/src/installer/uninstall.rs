// spec: installer/README.md §uninstall — reverses an install against the roster init recorded and
// nothing else: an entry still at what init wrote is removed, one edited since is kept and
// reported, and the manifest is narrowed over the survivors rather than deleted.
use super::{lock, refuse, Refusal, AGENT_FILE, GATES_DIR};
use std::collections::BTreeSet;

const USAGE: &[&str] = &[
    "usage: checkwright uninstall [--dry-run] [--force] [--no-commit]",
    "",
    "Removes the files init recorded in checkwright.lock and commits the removal.",
    "Nothing outside that roster is touched. A file you have edited since",
    "init wrote it is kept and reported; --force removes it anyway.",
];

struct Flags {
    dry: bool,
    force: bool,
    commit: bool,
}

fn parse(args: &[String]) -> Result<Option<Flags>, Refusal> {
    let mut f = Flags {
        dry: false,
        force: false,
        commit: true,
    };
    for a in args {
        match a.as_str() {
            "--dry-run" => f.dry = true,
            "--force" => f.force = true,
            "--no-commit" => f.commit = false,
            "-h" | "--help" => {
                for line in USAGE {
                    println!("{}", line);
                }
                return Ok(None);
            }
            other => return Err(refuse(format!("unknown argument: {}", other), "", 2)),
        }
    }
    Ok(Some(f))
}

pub fn run(args: &[String]) -> i32 {
    super::finish(
        "uninstall",
        parse(args).and_then(|f| match f {
            None => Ok(0),
            Some(f) => remove(&f),
        }),
    )
}

// spec: installer/README.md §uninstall — the plan groups by top-level directory, so a large removal
// reads as a shape rather than as a wall of paths.
fn by_top_dir(paths: &[String]) -> Vec<String> {
    let mut keys: Vec<String> = paths
        .iter()
        .map(|p| match p.split_once('/') {
            Some((top, _)) => format!("{}/", top),
            None => p.clone(),
        })
        .collect();
    keys.sort();
    let mut out = Vec::new();
    let mut i = 0;
    while i < keys.len() {
        let k = &keys[i];
        let n = keys[i..].iter().take_while(|x| *x == k).count();
        out.push(format!("  {:<32} {} file(s)", k, n));
        i += n;
    }
    out
}

fn residual(keep: &[(String, String)]) -> String {
    let mut e = lock::Emit::new();
    for (p, h) in keep {
        e = e.file(p, h);
    }
    e.render()
}

fn remove(f: &Flags) -> Result<i32, Refusal> {
    // spec: installer/README.md §init — every precondition refuses rather than warns and all of
    // them are checked before anything is removed, for init's own reason: a partial removal is the
    // outcome none of them may produce.
    let root = super::repo_root().ok_or_else(|| {
        refuse(
            "not inside a git work tree",
            "uninstall stages and commits the removal the same way init committed the install, so it needs the repository it is reversing.",
            2,
        )
    })?;
    let lock_path = lock::path(&root);
    if !lock_path.is_file() {
        return Err(refuse(
            format!("no {} at {}", lock::FILE, root.display()),
            "init is the verb that makes an install, and the manifest it writes is the only record of which files are this installer's to remove. Without one there is nothing here to reverse.",
            2,
        ));
    }
    let manifest = lock::Manifest::read(&lock_path)
        .filter(lock::Manifest::schema_ok)
        .ok_or_else(|| {
            refuse(
                format!("{} carries a schema this build does not know", lock::FILE),
                "this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for.",
                2,
            )
        })?;

    // spec: installer/README.md §init — the clean-worktree precondition is init's, on the same
    // terms: one commit is made, and a dirty tree would fold your work into it; --no-commit is the
    // same valve.
    if !f.dry && f.commit {
        let dirty = super::git_capture(&root, &["status", "--porcelain"]).unwrap_or_default();
        if !dirty.trim().is_empty() {
            return Err(refuse(
                "the worktree is not clean",
                "uninstall makes one commit, and a dirty tree would fold your work into it — so a reviewer's diff would no longer be the whole of what was removed. Commit or stash first, or pass --no-commit to stage the removal yourself.",
                1,
            ));
        }
    }

    let profile = manifest.field("profile");
    let version = manifest.field("version");
    let kits: Vec<String> = manifest
        .field("kits")
        .split_whitespace()
        .map(String::from)
        .collect();
    let gates_list = manifest.own_file(&format!("{}/gates.list", GATES_DIR));

    // spec: installer/README.md §uninstall — the removal rule is the ownership claim seen from the
    // other side and needs no new data: a hash that still matches marks a file init's to remove, one
    // that differs marks yours to keep, and a path already off the tree is a no-op.
    let (mut remove_set, mut keep, mut gone) = (Vec::new(), Vec::new(), Vec::new());
    let mut roster: BTreeSet<String> = BTreeSet::new();
    for (p, h) in manifest.files() {
        roster.insert(p.clone());
        let file = root.join(&p);
        if !file.is_file() {
            gone.push(p);
            continue;
        }
        if f.force || lock::hash(&file).unwrap_or_default() == h {
            remove_set.push(p);
        } else {
            keep.push((p, h));
        }
    }

    // spec: installer/README.md §uninstall — the agent file is the one entry that is a span rather
    // than a file, so the branch keeping it still owes the doctrine block a removal: that block is
    // prose you did not write, pointing at a doctrine file this verb just removed.
    let trim_agent = keep.iter().any(|(p, _)| p == AGENT_FILE)
        && kits.iter().any(|k| k == "doctrine-kit");

    // spec: installer/README.md §uninstall — the hook opt-in is reported, not rewritten: git config
    // is outside the ownership roster, and a `core.hooksPath` naming a directory that no longer
    // exists is inert rather than breaking.
    let mut hooks_line = String::new();
    if let Ok(hp) = super::git_capture(&root, &["config", "--get", "core.hooksPath"]) {
        let hp = hp.trim();
        if let Some((gates_dir, _)) = gates_list.rsplit_once('/') {
            let rel = hp
                .strip_prefix(&format!("{}/", root.display()))
                .unwrap_or(hp);
            if !hp.is_empty() && (rel == gates_dir || rel.starts_with(&format!("{}/", gates_dir))) {
                hooks_line = "git config --unset core.hooksPath".to_string();
            }
        }
    }

    // spec: installer/README.md §uninstall — a run with nothing to remove says so and exits 0: the
    // install is still there, so narrowing the manifest here would disown an install that has not
    // ended.
    if remove_set.is_empty() {
        println!(
            "UNINSTALL: nothing to remove — of {} recorded file(s), {} have changed since init wrote them and {} are already gone.",
            keep.len() + gone.len(),
            keep.len(),
            gone.len()
        );
        if !keep.is_empty() {
            println!("  help: they are yours, so uninstall leaves them. Pass --force to remove them anyway.");
        }
        return Ok(0);
    }

    // spec: installer/README.md §uninstall — a file you added inside a vendored directory is not on
    // the roster, so it is never removed; the plan names it because a directory left behind holding
    // only your own files is a surprise worth spending a line on before the run rather than after.
    let mut added: Vec<String> = Vec::new();
    for kit in &kits {
        if !root.join(kit).is_dir() {
            continue;
        }
        let found = super::files_under(&root.join(kit)).map_err(|e| refuse(e, "", 2))?;
        added.extend(
            found
                .into_iter()
                .map(|p| format!("{}/{}", kit, p))
                .filter(|p| !roster.contains(p)),
        );
    }

    if f.dry {
        println!(
            "checkwright uninstall --dry-run (profile: {}, version: {})\n",
            profile, version
        );
        println!("would remove {} file(s):", remove_set.len());
        for l in by_top_dir(&remove_set) {
            println!("{}", l);
        }
        println!("would then remove every directory those files empty.");
        if !keep.is_empty() {
            println!(
                "\nwould keep {} file(s) you have changed since init wrote them (--force to remove them anyway):",
                keep.len()
            );
            for (p, _) in &keep {
                println!("  {}", p);
            }
        }
        if !gone.is_empty() {
            println!(
                "\n{} recorded file(s) are already off the tree — nothing to do for them.",
                gone.len()
            );
        }
        if !added.is_empty() {
            println!(
                "\nwould leave {} file(s) you added inside a vendored directory:",
                added.len()
            );
            for p in &added {
                println!("  {}", p);
            }
        }
        if trim_agent {
            println!(
                "\nwould trim the doctrine block out of {}, which is being kept, and leave the rest of it alone.",
                AGENT_FILE
            );
        }
        if keep.is_empty() {
            println!(
                "\nwould delete {} — nothing recorded would survive.",
                lock::FILE
            );
        } else {
            println!(
                "\nwould rewrite {} over the {} kept file(s), at the hashes init recorded:",
                lock::FILE,
                keep.len()
            );
            print!("{}", residual(&keep));
        }
        if !hooks_line.is_empty() {
            println!("\nwould print this for you to run yourself:\n  {}", hooks_line);
        }
        println!("\nDRY RUN: nothing was written.");
        return Ok(0);
    }

    for p in &remove_set {
        std::fs::remove_file(root.join(p))
            .map_err(|e| refuse(format!("could not remove {}: {}", p, e), "", 2))?;
    }

    // spec: installer/README.md §uninstall — pruning is bottom-up and only ever removes a directory
    // that is now empty: uninstall removes files it owns, never directories it merely emptied
    // around, so one left holding anything at all is left alone.
    let mut dirs: BTreeSet<String> = BTreeSet::new();
    for p in &remove_set {
        let mut d = p.as_str();
        while let Some((parent, _)) = d.rsplit_once('/') {
            dirs.insert(parent.to_string());
            d = parent;
        }
    }
    for d in dirs.iter().rev() {
        let _ = std::fs::remove_dir(root.join(d));
    }

    let mut trimmed = false;
    if trim_agent {
        // spec: doctrine-kit/SPEC.md §install-doctrine — the trim runs through the kit's own
        // installer, which after the relocation is this binary's own doctrine module, so the marker
        // strings keep their one writer and the removal works once the vendored kit is gone.
        crate::doctrine::remove(&root.join(AGENT_FILE).to_string_lossy())
            .map_err(|e| refuse(
                format!("the doctrine block could not be trimmed out of {}: {}", AGENT_FILE, e),
                "",
                2,
            ))?;
        trimmed = true;
    }

    if keep.is_empty() {
        std::fs::remove_file(&lock_path)
            .map_err(|e| refuse(format!("could not remove {}: {}", lock::FILE, e), "", 2))?;
    } else {
        std::fs::write(&lock_path, residual(&keep)).map_err(|e| {
            refuse(
                format!("could not rewrite {} over the kept file(s): {}", lock::FILE, e),
                "",
                2,
            )
        })?;
    }

    // spec: installer/README.md §uninstall — the staged set is the removals and the manifest
    // disposition, never a kept file: staging one left for the adopter is the defect init's
    // written-set/roster split exists to prevent, and it stays one when the write is a removal.
    // spec: installer/README.md §init — the read's status must survive: every file above is already
    // deleted, so a read that failed and returned nothing would stage the manifest alone and commit
    // it under a message claiming the removal.
    let listed = super::git_batched(&root, &["ls-files", "-z"], &remove_set).map_err(|_| {
        refuse(
            "could not read which of the removed files this repository tracks",
            "nothing was staged and the removal is not committed; the files are gone from the worktree and 'git status' shows them, so stage and commit that yourself, or 'git restore' them to undo.",
            1,
        )
    })?;
    let mut stage: Vec<String> = listed
        .split(|b| *b == 0)
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf8_lossy(s).into_owned())
        .collect();
    let tracked_lock = super::git_code(&root, &["ls-files", "--error-unmatch", "--", lock::FILE])
        == Some(0);
    if lock_path.is_file() || tracked_lock {
        stage.push(lock::FILE.to_string());
    }
    if !stage.is_empty() {
        super::git_batched(&root, &["add"], &stage)
            .map_err(|e| refuse(format!("could not stage the removal: {}", e), "", 2))?;
    }

    if !keep.is_empty() {
        println!(
            "\n{} file(s) have changed since init wrote them and were kept:",
            keep.len()
        );
        for (p, _) in &keep {
            println!("  {}", p);
        }
        println!(
            "  help: they are yours. {} now records those paths and nothing else, at the hashes init wrote, so a future init still protects them rather than writing through them.",
            lock::FILE
        );
    }
    if trimmed {
        println!(
            "\ntrimmed the doctrine block out of {} and left the change unstaged — the rest of that file is yours to review.",
            AGENT_FILE
        );
    }

    if super::git_code(&root, &["diff", "--cached", "--quiet"]) == Some(0) {
        println!(
            "\nUNINSTALL: removed {} file(s), none of which this repository was tracking — nothing to commit.",
            remove_set.len()
        );
    } else if f.commit {
        let message = format!("chore: remove Checkwright kits ({}, v{})", profile, version);
        if super::git_code(&root, &["commit", "-q", "-m", &message]) != Some(0) {
            return Err(refuse(
                "the commit failed — the removal is staged; commit it yourself to finish",
                "",
                1,
            ));
        }
        println!(
            "\nUNINSTALL: removed {} file(s), kept {}, and committed the removal.",
            remove_set.len(),
            keep.len()
        );
    } else {
        println!(
            "\nUNINSTALL: removed {} file(s), kept {} and staged the removal — --no-commit, so the commit is yours.",
            remove_set.len(),
            keep.len()
        );
    }

    if !hooks_line.is_empty() {
        println!(
            "\nthis clone still points at the hooks directory that was just removed. Undo that yourself with:\n  {}",
            hooks_line
        );
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/README.md §uninstall — the plan's shape summary counts by top-level directory,
    // and a bare file at the root is its own key.
    #[test]
    fn the_plan_groups_by_top_level_directory() {
        let lines = by_top_dir(&[
            "gate-sdk/a.sh".to_string(),
            "gate-sdk/b/c.sh".to_string(),
            "checkwright.lock".to_string(),
        ]);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("checkwright.lock") && lines[0].ends_with("1 file(s)"));
        assert!(lines[1].contains("gate-sdk/") && lines[1].ends_with("2 file(s)"));
    }

    // spec: installer/README.md §The verbs — `--help` answers on its own and an unknown argument is
    // a usage refusal rather than an ignored token.
    #[test]
    fn help_answers_and_an_unknown_argument_refuses() {
        assert_eq!(run(&["--help".to_string()]), 0);
        assert_eq!(run(&["--nope".to_string()]), 2);
    }
}
