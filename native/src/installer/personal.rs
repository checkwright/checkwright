// spec: installer/SPEC.md §The personal install — the untracked per-clone overlay: the clone's
// exclude file, the block that names every recorded path in it, and the begin marker every verb
// reads as the install's mode
use super::{lock, PERSONAL_BEGIN, PERSONAL_END};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

// spec: installer/SPEC.md §The personal install — git's own answer for the clone's exclude file,
// kept in the spelling git printed for the report and absolutized for the read and the write
pub struct Exclude {
    pub shown: String,
    pub path: PathBuf,
    pub said: Vec<String>,
}

pub fn exclude(root: &Path) -> Result<Exclude, String> {
    let shown = super::git_capture(root, &["rev-parse", "--git-path", "info/exclude"])?
        .trim()
        .to_string();
    if shown.is_empty() {
        return Err("git named no info/exclude path for this clone".to_string());
    }
    let path = if crate::walk::path_root(&shown).is_some() {
        PathBuf::from(&shown)
    } else {
        root.join(&shown)
    };
    Ok(Exclude { shown, path, said: Vec::new() })
}

// spec: installer/SPEC.md §The personal install — the running worktree's key: none for the main
// worktree, its git directory's path under the common directory for a linked one
pub struct Worktree {
    pub key: Option<String>,
    pub common: PathBuf,
}

pub fn worktree(root: &Path) -> Result<Worktree, String> {
    let said = super::git_capture(root, &["rev-parse", "--absolute-git-dir", "--git-common-dir"])?;
    let mut lines = said.lines().map(str::trim);
    let (Some(git_dir), Some(common)) = (lines.next(), lines.next()) else {
        return Err("git named no git directory and common directory for this worktree".to_string());
    };
    let here = root.to_string_lossy();
    let git_dir = PathBuf::from(crate::walk::abs_against(&here, git_dir));
    let common = PathBuf::from(crate::walk::abs_against(&here, common));
    if crate::emit::hook_launcher::same_dir(&git_dir, &common) {
        return Ok(Worktree { key: None, common });
    }
    let canon = |p: &Path| {
        crate::walk::canonicalize(p)
            .map(|c| crate::walk::cross_arg(crate::walk::strip_extended_prefix(&c)))
            .unwrap_or_else(|| crate::walk::cross_arg(&p.to_string_lossy()))
    };
    let (g, c) = (canon(&git_dir), canon(&common));
    let key = crate::walk::rel_under(&c, &g)
        .map(|k| k.trim_end_matches('/').to_string())
        .filter(|k| !k.is_empty())
        .ok_or_else(|| format!("this worktree's git directory {} lies outside the common directory {}", g, c))?;
    Ok(Worktree { key: Some(key), common })
}

// spec: installer/SPEC.md §The personal install — the bare pair is the main worktree's, a linked
// worktree's carries its key after one space
pub fn markers(key: Option<&str>) -> (String, String) {
    match key {
        None => (PERSONAL_BEGIN.to_string(), PERSONAL_END.to_string()),
        Some(k) => (format!("{} {}", PERSONAL_BEGIN, k), format!("{} {}", PERSONAL_END, k)),
    }
}

// spec: installer/SPEC.md §The personal install — any worktree's block is the mode
pub fn is_personal(root: &Path) -> bool {
    let keyed = format!("{} ", PERSONAL_BEGIN);
    exclude(root)
        .ok()
        .and_then(|e| std::fs::read_to_string(&e.path).ok())
        .is_some_and(|t| {
            crate::fresh::file_lines(&t)
                .iter()
                .map(|l| crate::marker::bare(l))
                .any(|l| l == PERSONAL_BEGIN || l.starts_with(&keyed))
        })
}

// spec: installer/SPEC.md §The personal install — a keyed block whose key names no directory under
// the common directory is a worktree git no longer records, and goes; one whose end marker is
// missing is named and left
fn prune(e: &Exclude, wt: &Worktree) -> Result<Vec<String>, String> {
    let text = crate::fresh::read_captured(&e.path.to_string_lossy())?;
    let lines = crate::fresh::file_lines(&text);
    let keyed = format!("{} ", PERSONAL_BEGIN);
    let gone: Vec<String> = lines
        .iter()
        .filter_map(|l| crate::marker::bare(l).strip_prefix(&keyed))
        .filter(|k| !k.is_empty() && !wt.common.join(k).is_dir())
        .map(str::to_string)
        .collect();
    let mut said = Vec::new();
    for key in gone {
        let (begin, end) = markers(Some(&key));
        if !crate::marker::has(&lines, &end) {
            said.push(format!(
                "personal: left the exclude block of {} in {}, its end marker missing",
                key, e.shown
            ));
            continue;
        }
        crate::marker::uninstall_block(&e.path.to_string_lossy(), &begin, &end)?;
        said.push(format!(
            "personal: dropped the exclude block of {}, a worktree git no longer records",
            key
        ));
    }
    Ok(said)
}

// spec: installer/SPEC.md §The personal install — each of git's pattern metacharacters is escaped,
// so a line matches the one path it names
fn escaped(p: &str) -> String {
    let mut out = String::with_capacity(p.len());
    for c in p.chars() {
        if matches!(c, '\\' | '*' | '?' | '[') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

// spec: installer/SPEC.md §The personal install — one anchored line per recorded path and the
// manifest, a kit directory the index holds nothing beneath collapsed to one directory line
pub fn body(root: &Path, recorded: &[String], kits: &[String]) -> Result<String, String> {
    let mut paths: Vec<String> = recorded.to_vec();
    paths.push(lock::FILE.to_string());
    let tops: Vec<String> = paths
        .iter()
        .filter_map(|p| p.split_once('/').map(|(c, _)| c.to_string()))
        .filter(|c| kits.contains(c))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let listed = super::git_batched(root, &["--literal-pathspecs", "ls-files", "-z"], &tops)?;
    let held: BTreeSet<String> = crate::listing::decode(&listed)
        .map_err(|e| e.text())?
        .into_iter()
        .filter_map(|p| p.split_once('/').map(|(c, _)| c.to_string()))
        .collect();
    let lines: BTreeSet<String> = paths
        .iter()
        .map(|p| match p.split_once('/') {
            Some((top, _)) if tops.iter().any(|t| t == top) && !held.contains(top) => format!("/{}/", escaped(top)),
            _ => format!("/{}", escaped(p)),
        })
        .collect();
    Ok(lines.into_iter().map(|l| l + "\n").collect())
}

// spec: installer/SPEC.md §The personal install — this worktree's block is recomputed whole on
// every write, through the installer writer, into a file and directory made when absent
pub fn write(root: &Path, recorded: &[String], kits: &[String]) -> Result<Exclude, String> {
    let mut e = exclude(root)?;
    let wt = worktree(root)?;
    let text = body(root, recorded, kits)?;
    if let Some(dir) = e.path.parent() {
        std::fs::create_dir_all(dir).map_err(|err| format!("cannot create {}: {}", dir.display(), err))?;
    }
    if !e.path.is_file() {
        std::fs::write(&e.path, "").map_err(|err| format!("cannot create {}: {}", e.shown, err))?;
    }
    let (begin, end) = markers(wt.key.as_deref());
    crate::marker::install_block(&e.path.to_string_lossy(), &begin, &end, &text)?;
    e.said = prune(&e, &wt)?;
    Ok(e)
}

// spec: installer/SPEC.md §The personal install — the writer's begin-without-end refusal, read over
// this worktree's pair before the run's first write
pub fn check(root: &Path) -> Result<(), String> {
    let e = exclude(root)?;
    if !e.path.is_file() {
        return Ok(());
    }
    let (begin, end) = markers(worktree(root)?.key.as_deref());
    crate::marker::installable(&e.path.to_string_lossy(), &begin, &end)
}

// spec: installer/SPEC.md §uninstall — this worktree's block goes with the separator its append wrote
pub fn remove(root: &Path) -> Result<Exclude, String> {
    let mut e = exclude(root)?;
    let wt = worktree(root)?;
    let (begin, end) = markers(wt.key.as_deref());
    crate::marker::uninstall_block(&e.path.to_string_lossy(), &begin, &end)?;
    e.said = prune(&e, &wt)?;
    Ok(e)
}

// spec: installer/SPEC.md §The personal install — the planned paths the index tracks, asked in one
// batched listing
pub fn tracked(root: &Path, planned: &[String]) -> Result<Vec<String>, String> {
    let listed = super::git_batched(root, &["--literal-pathspecs", "ls-files", "-z"], planned)?;
    let mut out = crate::listing::decode(&listed).map_err(|e| e.text())?;
    out.sort();
    out.dedup();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs;

    // spec: installer/SPEC.md §The personal install — a root file, a kit directory with and one
    // without tracked content beneath, and shared directories holding only untracked files
    #[test]
    fn the_block_collapses_only_an_untracked_kit_directory_and_anchors_every_line() {
        let dir = std::env::temp_dir().join(format!("cw-personal-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        for d in ["scripts", "docs", "vendored"] {
            std::fs::create_dir_all(dir.join(d)).expect("cannot make the scratch repo");
        }
        let git = |args: &[&str]| {
            let d = dir.to_string_lossy().into_owned();
            let mut argv = vec!["-C", d.as_str()];
            argv.extend_from_slice(args);
            crate::proc::run(&programs::GIT, &argv).expect("git did not spawn");
        };
        git(&["init", "-q"]);
        std::fs::write(dir.join("vendored/own.txt"), "x\n").expect("cannot write the tracked file");
        git(&["add", "vendored/own.txt"]);
        let recorded = vec![
            "gate-sdk/a.sh".to_string(),
            "gate-sdk/b/c.sh".to_string(),
            "vendored/x.sh".to_string(),
            "scripts/gates.list".to_string(),
            "docs/graph.html".to_string(),
            "odd[1]*.md".to_string(),
        ];
        let kits = vec!["gate-sdk".to_string(), "vendored".to_string()];
        let text = body(&dir, &recorded, &kits).expect("the body was not derived");
        assert_eq!(
            text,
            format!(
                "/{}\n/docs/graph.html\n/gate-sdk/\n/odd\\[1]\\*.md\n/scripts/gates.list\n/vendored/x.sh\n",
                lock::FILE
            ),
            "the block's lines"
        );
        let e = write(&dir, &recorded, &kits).expect("the block was not written");
        assert!(is_personal(&dir), "the written block was not read as the mode");
        let ignored = |p: &str| super::super::git_code(&dir, &["check-ignore", "-q", p]) == Some(0);
        assert!(ignored("scripts/gates.list"), "the excluded gates-directory file is not ignored");
        assert!(ignored("gate-sdk/new.sh"), "a file under the collapsed kit root is not ignored");
        assert!(!ignored("scripts/other.sh"), "an untracked project file under the gates directory was hidden");
        assert!(!ignored("docs/other.md"), "an untracked project file under a shared directory was hidden");
        assert!(!ignored("vendored/new.sh"), "a file under a kit directory the index tracks was hidden");
        remove(&dir).expect("the block was not removed");
        assert!(!is_personal(&dir), "the removed block was still read as the mode");
        assert!(e.path.is_file());
        assert_eq!(tracked(&dir, &["vendored/own.txt".to_string(), "scripts/gates.list".to_string()]), Ok(vec!["vendored/own.txt".to_string()]));
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §The personal install — one block per worktree over a scratch
    // repository with one linked worktree: the key, the bare block adopted, each removal taking its
    // own block, and the prune of a worktree git no longer records
    #[test]
    fn each_worktree_keeps_its_own_block_and_a_gone_worktrees_block_is_pruned() {
        let main = std::env::temp_dir().join(format!("cw-personal-wt-{}", std::process::id()));
        let linked = std::env::temp_dir().join(format!("cw-personal-wt-{}-linked", std::process::id()));
        for d in [&main, &linked] {
            std::fs::remove_dir_all(d).ok();
        }
        std::fs::create_dir_all(&main).expect("cannot make the scratch repo");
        let git = |args: &[&str]| {
            let d = main.to_string_lossy().into_owned();
            let mut argv = vec!["-C", d.as_str(), "-c", "user.name=t", "-c", "user.email=t@example.invalid"];
            argv.extend_from_slice(args);
            crate::proc::run(&programs::GIT, &argv).expect("git did not spawn");
        };
        git(&["init", "-q"]);
        // comment-tier-exempt: auto maintenance stays off, since the seed commit's detached run
        // prunes the worktree admin dir below while it is still half-written
        git(&["config", "maintenance.auto", "false"]);
        git(&["commit", "-q", "--allow-empty", "-m", "seed"]);
        let linked_s = linked.to_string_lossy().into_owned();
        let name = linked.file_name().expect("a named worktree").to_string_lossy().into_owned();
        // spec: gate-sdk/SPEC.md §upgrade-smoke — the linked worktree is laid out by hand, git's
        // own admin-directory shape, since crate source never adds one through git
        let admin = main.join(".git/worktrees").join(&name);
        std::fs::create_dir_all(&admin).expect("cannot make the admin directory");
        std::fs::create_dir_all(&linked).expect("cannot make the linked worktree");
        let head = super::super::git_capture(&main, &["rev-parse", "HEAD"]).expect("no seed commit");
        for (file, body) in [
            ("HEAD", head),
            ("commondir", "../..\n".to_string()),
            ("gitdir", format!("{}/.git\n", linked_s)),
        ] {
            std::fs::write(admin.join(file), body).expect("cannot write the admin directory");
        }
        std::fs::write(linked.join(".git"), format!("gitdir: {}\n", admin.display())).expect("cannot link the worktree");
        let key = format!("worktrees/{}", name);
        assert_eq!(worktree(&main).expect("no main key").key, None, "the main worktree's key");
        assert_eq!(worktree(&linked).expect("no linked key").key, Some(key.clone()), "the linked worktree's key");

        let e = exclude(&main).expect("no exclude file");
        std::fs::create_dir_all(e.path.parent().expect("an info dir")).expect("cannot make info/");
        std::fs::write(&e.path, "").expect("cannot seed the exclude file");
        crate::marker::install_block(&e.path.to_string_lossy(), PERSONAL_BEGIN, PERSONAL_END, "/stale\n")
            .expect("cannot seed a bare block");
        let one = |p: &str| vec![p.to_string()];
        write(&main, &one("main-only.txt"), &[]).expect("the main block was not written");
        let read = || std::fs::read_to_string(&e.path).expect("cannot read the exclude file");
        let bare = read().lines().filter(|l| *l == PERSONAL_BEGIN).count();
        assert_eq!(bare, 1, "the bare block was duplicated rather than replaced");
        assert!(!read().contains("/stale"), "the bare block was not replaced");

        write(&linked, &one("linked-only.txt"), &[]).expect("the linked block was not written");
        let ignored = |at: &Path, p: &str| super::super::git_code(at, &["check-ignore", "-q", p]) == Some(0);
        for at in [&main, &linked] {
            for p in ["main-only.txt", "linked-only.txt", lock::FILE] {
                assert!(ignored(at, p), "{} does not read ignored from {}", p, at.display());
            }
        }
        let main_block = crate::marker::read_block(&read(), PERSONAL_BEGIN, PERSONAL_END);
        remove(&linked).expect("the linked block was not removed");
        assert_eq!(crate::marker::read_block(&read(), PERSONAL_BEGIN, PERSONAL_END), main_block, "the main block moved");
        assert!(ignored(&linked, "main-only.txt"), "the main worktree's path stopped reading ignored");
        assert!(!ignored(&main, "linked-only.txt"), "the removed block's path still reads ignored");
        assert!(is_personal(&linked), "a worktree without its own block lost the mode");

        write(&linked, &one("linked-only.txt"), &[]).expect("the linked block was not rewritten");
        // spec: installer/SPEC.md §The personal install — a CRLF-converted exclude file keeps both
        // blocks: the mode reads, the bare block is replaced, and the live key is not pruned
        std::fs::write(&e.path, read().replace('\n', "\r\n")).expect("cannot convert the exclude file");
        assert!(is_personal(&main), "a CRLF bare block was not read as the mode");
        let said = write(&main, &one("main-only.txt"), &[]).expect("the CRLF main block was not rewritten").said;
        assert!(said.is_empty(), "a live worktree's CRLF block was pruned: {:?}", said);
        assert_eq!(read().matches(PERSONAL_BEGIN).count(), 2, "a CRLF block was duplicated or dropped");
        assert!(ignored(&main, "linked-only.txt"), "the linked block stopped reading ignored");
        git(&["worktree", "remove", "--force", &linked_s]);
        let mut grown = read();
        grown.push_str(&format!("{} worktrees/ghost\n/ghost\n", PERSONAL_BEGIN));
        std::fs::write(&e.path, grown).expect("cannot plant a half-open block");
        let said = write(&main, &one("main-only.txt"), &[]).expect("the main block was not rewritten").said;
        assert_eq!(
            said,
            vec![
                format!("personal: dropped the exclude block of {}, a worktree git no longer records", key),
                format!("personal: left the exclude block of worktrees/ghost in {}, its end marker missing", e.shown),
            ],
            "the prune's report"
        );
        assert!(!read().contains(&key), "the gone worktree's block was not pruned");
        std::fs::remove_dir_all(&main).ok();
    }
}
