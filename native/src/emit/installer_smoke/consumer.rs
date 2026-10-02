// spec: installer/SPEC.md §The consumer smoke — what every arm shares: the scratch consumer, the
// entry point under the arm's PATH, the manifest as data and the consumer-layout constants
use super::{fail, in_consumer, merged_in, refuse, text, Entry, Outcome, Run};
use crate::programs::{self, Program};
use crate::proc;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

// spec: installer/SPEC.md §The manifest — the consumer-layout constants, each the operand of an
// assertion that reds on a wrong value rather than a silent second copy
pub(super) const GATES_DIR: &str = "scripts";
pub(super) const QUEUE_FILE: &str = "TASK-QUEUE.md";
pub(super) const PROFILE_DERIVED: &str = "full";

static SEQ: AtomicUsize = AtomicUsize::new(0);

// spec: installer/SPEC.md §The consumer smoke — the PATH an arm runs its steps under, inherited
// where it masks nothing
pub(super) fn path_set(path: &Option<String>) -> Vec<(String, String)> {
    path.iter().map(|p| ("PATH".to_string(), p.clone())).collect()
}

#[cfg(unix)]
pub(super) fn sh() -> Result<Program, Outcome> {
    Ok(programs::SH)
}

#[cfg(not(unix))]
pub(super) fn sh() -> Result<Program, Outcome> {
    Err(refuse("the extracted package's POSIX bootstrap is driven on a unix host only"))
}

// spec: installer/SPEC.md §The consumer smoke — one entry point run in one directory, the arm's
// PATH and any value it sets in that child's environment
pub(super) fn entry_run(entry: &Entry, cwd: &str, args: &[&str], set: &[(String, String)]) -> Result<proc::Merged, Outcome> {
    match entry {
        Entry::Bin(p) => merged_in(&programs::CHECKWRIGHT_GATES.at(p.clone()), args, set, cwd),
        Entry::Sh(p) => {
            let mut argv = vec![p.as_str()];
            argv.extend_from_slice(args);
            merged_in(&sh()?, &argv, set, cwd)
        }
    }
}

impl Run {
    pub(super) fn verb(&self, cwd: &str, args: &[&str]) -> Result<proc::Merged, Outcome> {
        self.verb_with(cwd, args, &[])
    }

    pub(super) fn verb_with(&self, cwd: &str, args: &[&str], extra: &[(String, String)]) -> Result<proc::Merged, Outcome> {
        let mut set = path_set(&self.run_path);
        set.extend(extra.iter().cloned());
        entry_run(&self.entry, cwd, args, &set)
    }

    // spec: installer/SPEC.md §The consumer smoke — the battery or a front-end arm of the consumer
    // itself, through its vendored front-end under the arm's PATH
    pub(super) fn front_end(&self, cwd: &str, args: &[&str]) -> Result<proc::Merged, Outcome> {
        let mut argv = vec!["gate-sdk/bin/run-gates.sh"];
        argv.extend_from_slice(args);
        merged_in(&programs::BASH, &argv, &path_set(&self.run_path), cwd)
    }
}

pub(super) fn out(m: &proc::Merged) -> String {
    text(m.output())
}

// spec: installer/SPEC.md §The consumer smoke — a first-match read of a merged capture, for a line
// the arm prints or tests whole
pub(super) fn find_line(out: &str, pred: impl Fn(&str) -> bool) -> Option<&str> {
    out.split('\n').map(|l| l.strip_suffix('\r').unwrap_or(l)).find(|l| pred(l))
}

pub(super) fn first_starting<'a>(out: &'a str, prefix: &str) -> &'a str {
    find_line(out, |l| l.starts_with(prefix)).unwrap_or("")
}

// spec: installer/SPEC.md §The consumer smoke — the battery's summary line, `All <n> gates passed`
pub(super) fn all_passed(out: &str) -> Option<&str> {
    find_line(out, |l| {
        l.match_indices("All ").any(|(i, _)| {
            let rest = &l[i + 4..];
            let digits = rest.chars().take_while(char::is_ascii_digit).count();
            digits > 0 && rest[digits..].starts_with(" gates passed")
        })
    })
}

// spec: installer/SPEC.md §The consumer smoke — a git read a later assertion compares against: its
// stdout on success, a harness precondition otherwise
pub(super) fn git_out(c: &str, args: &[&str]) -> Result<String, Outcome> {
    let done = in_consumer(&programs::GIT, args, &[], c)?;
    match done.stdout() {
        Some(o) => Ok(text(o)),
        None => Err(refuse(format!(
            "git {} could not read {} — {}",
            args.join(" "),
            c,
            done.failure_report().unwrap_or_default()
        ))),
    }
}

pub(super) fn git(c: &str, args: &[&str]) -> Result<proc::Merged, Outcome> {
    merged_in(&programs::GIT, args, &[], c)
}

pub(super) fn tree(c: &str) -> Result<String, Outcome> {
    Ok(git_out(c, &["rev-parse", "HEAD^{tree}"])?.trim().to_string())
}

pub(super) fn head(c: &str) -> Result<String, Outcome> {
    Ok(git_out(c, &["rev-parse", "HEAD"])?.trim().to_string())
}

pub(super) fn commits(c: &str) -> Result<usize, Outcome> {
    Ok(git_out(c, &["rev-list", "--count", "HEAD"])?.trim().parse().unwrap_or(0))
}

pub(super) fn porcelain(c: &str) -> Result<String, Outcome> {
    git_out(c, &["status", "--porcelain"])
}

// spec: installer/SPEC.md §The consumer smoke — a file's blob hash in the consumer's own repository
// context, the one init's recorded-hash helper runs in
pub(super) fn hash_in(c: &str, rel: &str) -> Result<String, Outcome> {
    Ok(git_out(c, &["hash-object", "--", rel])?.trim().to_string())
}

// spec: installer/SPEC.md §The consumer smoke — a fresh scratch consumer: one empty seed commit, a
// local identity, and no background maintenance repacking .git while an arm copies it
pub(super) fn consumer(state: &Run, label: &str) -> Result<String, Outcome> {
    let c = format!("{}/consumer-{}.{}", state.scratch, label, SEQ.fetch_add(1, Ordering::Relaxed));
    let made = || -> Result<(), Outcome> {
        std::fs::create_dir(&c).map_err(|e| fail(format!("cannot create {}: {}", c, e)))?;
        for args in [
            &["init", "-q"][..],
            &["config", "user.email", "smoke@example.invalid"],
            &["config", "user.name", "smoke"],
            &["config", "maintenance.auto", "false"],
            &["config", "gc.auto", "0"],
            &["commit", "-q", "--allow-empty", "-m", "seed"],
        ] {
            git_out(&c, args)?;
        }
        Ok(())
    };
    made().map_err(|e| match e {
        Outcome::Fail(why) | Outcome::Refuse(why) => fail(format!("could not build a scratch consumer for {}: {}", label, why)),
    })?;
    Ok(c)
}

// spec: installer/SPEC.md §The consumer smoke — the manifest read as the data it is, with the
// crate's JSON reader
pub(super) struct Lock(serde_json::Value);

fn render(v: Option<&serde_json::Value>) -> String {
    match v {
        None | Some(serde_json::Value::Null) => String::new(),
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(other) => other.to_string(),
    }
}

impl Lock {
    pub(super) fn read(path: &str) -> Result<Lock, Outcome> {
        let raw = std::fs::read_to_string(path).map_err(|e| fail(format!("could not read the manifest {}: {}", path, e)))?;
        serde_json::from_str(&raw)
            .map(Lock)
            .map_err(|e| fail(format!("the manifest {} does not parse as JSON: {}", path, e)))
    }

    pub(super) fn of(c: &str) -> Result<Lock, Outcome> {
        Lock::read(&format!("{}/checkwright.lock", c))
    }

    pub(super) fn top(&self, key: &str) -> String {
        render(self.0.get(key))
    }

    pub(super) fn has(&self, key: &str) -> bool {
        self.0.get(key).is_some()
    }

    pub(super) fn artifact(&self, key: &str) -> String {
        render(self.0.get("artifact").and_then(|a| a.get(key)))
    }

    pub(super) fn files(&self) -> Vec<(String, String)> {
        self.0
            .get("files")
            .and_then(|f| f.as_object())
            .map(|m| m.iter().map(|(k, v)| (k.clone(), render(Some(v)))).collect())
            .unwrap_or_default()
    }

    pub(super) fn keys(&self) -> Vec<String> {
        self.files().into_iter().map(|(k, _)| k).collect()
    }

    pub(super) fn has_file(&self, p: &str) -> bool {
        self.0.get("files").and_then(|f| f.get(p)).is_some()
    }

    pub(super) fn file(&self, p: &str) -> String {
        render(self.0.get("files").and_then(|f| f.get(p)))
    }

    pub(super) fn list(&self, key: &str) -> Vec<String> {
        self.0
            .get(key)
            .and_then(|k| k.as_array())
            .map(|a| a.iter().map(|v| render(Some(v))).collect())
            .unwrap_or_default()
    }

    pub(super) fn selection_list(&self, key: &str) -> Vec<String> {
        self.0
            .get("selection")
            .and_then(|s| s.get(key))
            .and_then(|k| k.as_array())
            .map(|a| a.iter().map(|v| render(Some(v))).collect())
            .unwrap_or_default()
    }

    pub(super) fn value(&self) -> &serde_json::Value {
        &self.0
    }
}

// spec: installer/SPEC.md §The consumer smoke — the knob-file seam's GATE_SDK_NATIVE_BIN value, read
// in the knob-file line grammar: the head before the first `=`, blanks trimmed
pub(super) fn seam_bin(seam: &str) -> Option<String> {
    let body = std::fs::read_to_string(seam).ok()?;
    body.lines().find_map(|line| {
        let (head, value) = line.split_once('=')?;
        let head: String = head.chars().filter(|c| *c != ' ' && *c != '\t').collect();
        (head == "GATE_SDK_NATIVE_BIN").then(|| value.trim_matches([' ', '\t']).to_string())
    })
}

pub(super) fn seam_of(c: &str) -> Option<String> {
    seam_bin(&format!("{}/{}/gate-sdk-config.knobs", c, GATES_DIR))
}

pub(super) fn digest_of(file: &str) -> String {
    crate::sha256::file_hex(Path::new(file)).unwrap_or_default()
}

pub(super) fn is_hash(v: &str) -> bool {
    v.len() == 40 && v.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

// spec: installer/SPEC.md §The consumer smoke — a tree copied with its links and modes, the shape
// `cp -Rp` gave the copies an arm runs a printed command in
pub(super) fn copy_tree(src: &Path, dst: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dst).map_err(|e| format!("cannot create {}: {}", dst.display(), e))?;
    for (name, _) in crate::walk::list_dir(src)? {
        let (from, to) = (src.join(&name), dst.join(&name));
        let kind = std::fs::symlink_metadata(&from)
            .map_err(|e| format!("cannot stat {}: {}", from.display(), e))?
            .file_type();
        if kind.is_symlink() {
            link(&from, &to)?;
        } else if kind.is_dir() {
            copy_tree(&from, &to)?;
        } else {
            std::fs::copy(&from, &to).map_err(|e| format!("cannot copy {}: {}", from.display(), e))?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn link(from: &Path, to: &Path) -> Result<(), String> {
    let target = std::fs::read_link(from).map_err(|e| format!("cannot read the link {}: {}", from.display(), e))?;
    std::os::unix::fs::symlink(target, to).map_err(|e| format!("cannot link {}: {}", to.display(), e))
}

#[cfg(not(unix))]
fn link(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::copy(from, to).map(|_| ()).map_err(|e| format!("cannot copy {}: {}", from.display(), e))
}

// spec: installer/SPEC.md §The consumer smoke — every file of one extension under a directory,
// relative to it and sorted, the walk `find` gave the lifecycle overlay
pub(super) fn files_under(root: &Path, ext: &str) -> Result<Vec<String>, Outcome> {
    let found = crate::walk::find_files(root, &[ext]).map_err(refuse)?;
    let mut rel: Vec<String> = found
        .iter()
        .filter_map(|f| f.strip_prefix(root).ok().map(|r| r.to_string_lossy().replace('\\', "/")))
        .collect();
    rel.sort();
    Ok(rel)
}

// spec: installer/SPEC.md §The consumer smoke — where a program resolves on a PATH an arm built,
// the crate's own PATH search with no spawn
pub(super) fn resolves(program: &str, path: &str) -> Option<String> {
    #[cfg(windows)]
    let pathext = Some(std::env::var("PATHEXT").unwrap_or_default());
    #[cfg(not(windows))]
    let pathext: Option<String> = None;
    proc::resolve_on_path(program, Some(std::ffi::OsStr::new(path)), pathext.as_deref(), proc::is_executable)
}

pub(super) fn invoking_path() -> String {
    std::env::var("PATH").unwrap_or_default()
}

// spec: installer/SPEC.md §init — a printed command's head: a path is the binary init placed, run
// from the directory the line is run in, and a bare word is the interpreter the line names
pub(super) fn argv_program(cwd: &str, head: &str) -> Result<Program, Outcome> {
    if head.contains('/') {
        return Ok(programs::CHECKWRIGHT_GATES.at(crate::walk::abs_against(cwd, head)));
    }
    match head {
        "bash" => Ok(programs::BASH),
        "sh" => sh(),
        other => Err(refuse(format!(
            "the printed command's head '{}' is neither a path nor an interpreter this arm can run",
            other
        ))),
    }
}

pub(super) fn run_line(cwd: &str, toks: &[&str], set: &[(String, String)]) -> Result<proc::Merged, Outcome> {
    let (head, rest) = toks.split_first().ok_or_else(|| refuse("an empty command line"))?;
    merged_in(&argv_program(cwd, head)?, rest, set, cwd)
}

pub(super) fn exists(p: &str) -> bool {
    Path::new(p).exists()
}

pub(super) fn is_file(p: &str) -> bool {
    Path::new(p).is_file()
}

pub(super) fn write(p: &str, body: &str) -> Result<(), Outcome> {
    if let Some(parent) = Path::new(p).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(p, body).map_err(|e| fail(format!("could not write {}: {}", p, e)))
}

pub(super) fn append(p: &str, body: &str) -> Result<(), Outcome> {
    use std::io::Write;
    std::fs::OpenOptions::new()
        .append(true)
        .open(p)
        .and_then(|mut f| f.write_all(body.as_bytes()))
        .map_err(|e| fail(format!("could not append to {}: {}", p, e)))
}

pub(super) fn mkdir(p: &str) -> Result<(), Outcome> {
    std::fs::create_dir_all(p).map_err(|e| fail(format!("could not create {}: {}", p, e)))
}

// spec: installer/SPEC.md §The consumer smoke — a tarball extracted with tar into a directory, the
// Node-free transport's own step
pub(super) fn extract(tarball: &str, into: &str) -> Result<(), Outcome> {
    let done = proc::run(&programs::TAR, &["-xzf", tarball, "-C", into]).map_err(refuse)?;
    match done.failure_report() {
        None => Ok(()),
        Some(r) => Err(fail(format!("tar could not extract {}: {}", tarball, r))),
    }
}

// spec: installer/SPEC.md §The consumer smoke — a verb's failure account and its verdict together
pub(super) fn failed(m: &proc::Merged, why: impl Into<String>) -> Outcome {
    super::show(&out(m));
    fail(why)
}

pub(super) fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The consumer smoke — the seam value is read in the knob-file line
    // grammar, blanks around the head and the value trimmed
    #[test]
    fn the_seam_value_is_read_in_the_knob_line_grammar() {
        let f = std::env::temp_dir().join(format!("cw-installer-smoke-seam-{}", std::process::id()));
        std::fs::write(&f, "# GATE_SDK_NATIVE_BIN = no\nGATE_SDK_KIT_DIRS = a\n GATE_SDK_NATIVE_BIN\t= scripts/checkwright-gates \n").expect("write");
        let got = seam_bin(&f.display().to_string());
        let _ = std::fs::remove_file(&f);
        assert_eq!(got.as_deref(), Some("scripts/checkwright-gates"));
    }

    // spec: installer/SPEC.md §The consumer smoke — the summary line is found unanchored, and a
    // count is required
    #[test]
    fn the_battery_summary_needs_a_count() {
        assert_eq!(all_passed("x\nAll 12 gates passed.\n"), Some("All 12 gates passed."));
        assert_eq!(all_passed("All gates passed.\n"), None);
    }
}
