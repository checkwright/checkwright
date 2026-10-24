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
    Ok(Exclude { shown, path })
}

pub fn is_personal(root: &Path) -> bool {
    exclude(root)
        .ok()
        .and_then(|e| std::fs::read_to_string(&e.path).ok())
        .is_some_and(|t| crate::fresh::file_lines(&t).contains(&PERSONAL_BEGIN))
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
// manifest, a first component the index holds nothing beneath collapsed to one directory line
pub fn body(root: &Path, recorded: &[String]) -> Result<String, String> {
    let mut paths: Vec<String> = recorded.to_vec();
    paths.push(lock::FILE.to_string());
    let tops: Vec<String> = paths
        .iter()
        .filter_map(|p| p.split_once('/').map(|(c, _)| c.to_string()))
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
            Some((top, _)) if !held.contains(top) => format!("/{}/", escaped(top)),
            _ => format!("/{}", escaped(p)),
        })
        .collect();
    Ok(lines.into_iter().map(|l| l + "\n").collect())
}

// spec: installer/SPEC.md §The personal install — the block is recomputed whole on every write,
// through the installer writer, into a file and directory made when absent
pub fn write(root: &Path, recorded: &[String]) -> Result<Exclude, String> {
    let e = exclude(root)?;
    let text = body(root, recorded)?;
    if let Some(dir) = e.path.parent() {
        std::fs::create_dir_all(dir).map_err(|err| format!("cannot create {}: {}", dir.display(), err))?;
    }
    if !e.path.is_file() {
        std::fs::write(&e.path, "").map_err(|err| format!("cannot create {}: {}", e.shown, err))?;
    }
    crate::marker::install_block(&e.path.to_string_lossy(), PERSONAL_BEGIN, PERSONAL_END, &text)?;
    Ok(e)
}

// spec: installer/SPEC.md §uninstall — the block goes with the separator its append wrote
pub fn remove(root: &Path) -> Result<Exclude, String> {
    let e = exclude(root)?;
    crate::marker::uninstall_block(&e.path.to_string_lossy(), PERSONAL_BEGIN, PERSONAL_END)?;
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

    // spec: installer/SPEC.md §The personal install — the block's three line shapes over a scratch
    // repository: a root file, a component the index holds nothing beneath, and a path under a
    // tracked component, each anchored and escaped
    #[test]
    fn the_block_collapses_an_untracked_component_and_anchors_every_line() {
        let dir = std::env::temp_dir().join(format!("cw-personal-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(dir.join("scripts")).expect("cannot make the scratch repo");
        let git = |args: &[&str]| {
            let d = dir.to_string_lossy().into_owned();
            let mut argv = vec!["-C", d.as_str()];
            argv.extend_from_slice(args);
            crate::proc::run(&programs::GIT, &argv).expect("git did not spawn");
        };
        git(&["init", "-q"]);
        std::fs::write(dir.join("scripts/build.sh"), "x\n").expect("cannot write the tracked script");
        git(&["add", "scripts/build.sh"]);
        let recorded = vec![
            "gate-sdk/a.sh".to_string(),
            "gate-sdk/b/c.sh".to_string(),
            "scripts/gates.list".to_string(),
            "odd[1]*.md".to_string(),
        ];
        let text = body(&dir, &recorded).expect("the body was not derived");
        assert_eq!(
            text,
            format!("/{}\n/gate-sdk/\n/odd\\[1]\\*.md\n/scripts/gates.list\n", lock::FILE),
            "the block's lines"
        );
        let e = write(&dir, &recorded).expect("the block was not written");
        assert!(is_personal(&dir), "the written block was not read as the mode");
        let ignored = |p: &str| super::super::git_code(&dir, &["check-ignore", "-q", p]) == Some(0);
        assert!(ignored("scripts/gates.list"), "the excluded gates-directory file is not ignored");
        assert!(ignored("gate-sdk/new.sh"), "a file under the collapsed kit root is not ignored");
        assert!(!ignored("scripts/other.sh"), "a project file under the shared directory was hidden");
        remove(&dir).expect("the block was not removed");
        assert!(!is_personal(&dir), "the removed block was still read as the mode");
        assert!(e.path.is_file());
        assert_eq!(tracked(&dir, &["scripts/build.sh".to_string(), "scripts/gates.list".to_string()]), Ok(vec!["scripts/build.sh".to_string()]));
        std::fs::remove_dir_all(&dir).ok();
    }
}
