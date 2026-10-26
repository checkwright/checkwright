// spec: installer/SPEC.md §The verbs — the adopter verbs, behind the invoke: each a
// top-level `--`-prefixed arm resolved before the registry lookup and absent from `--list`, taking
// every value as argv because the caller is a bootstrap that may run no POSIX shell.
pub mod demo;
pub mod diff;
pub mod doctor;
pub mod init;
pub mod lock;
pub mod payload_recipe;
pub mod personal;
pub mod profile;
pub mod prove;
pub mod recipe;
pub mod selection;
pub mod uninstall;
pub mod update;
pub mod workflow;

use crate::programs;
use crate::{proc, walk};
use std::path::{Path, PathBuf};

// spec: installer/SPEC.md §The verbs — the roster's owner is the binary: a verb is advertised
// because this table carries it, so `checkwright --help` cannot promise one the artifact does not
// implement and an unknown verb is refused by the binary's own usage arm.
pub type Verb = fn(&[String]) -> i32;

pub const VERBS: &[(&str, Verb)] = &[
    ("--init", init::run),
    ("--doctor", doctor::run),
    ("--diff", diff::run),
    ("--update", update::run),
    ("--uninstall", uninstall::run),
    ("--demo", demo::run),
    ("--prove", prove::run),
];

// spec: installer/SPEC.md §The verbs — the same ownership for each verb's flags: a flag is
// advertisable because the parsers of the verbs named beside it accept it, and a crate test holds
// every parser and `usage:` line to this table. `-h`/`--help` takes no row, being every verb's.
pub const FLAGS: &[(&str, &[&str])] = &[
    ("--profile", &["init", "update"]),
    ("--recipe", &["init", "update"]),
    ("--no-recipe", &["init", "update"]),
    ("--dry-run", &["init", "update", "uninstall"]),
    ("--force", &["init", "update", "uninstall"]),
    ("--no-commit", &["init", "update", "uninstall"]),
    ("--personal", &["init", "update"]),
    ("--with-kit", &["init", "update"]),
    ("--without-kit", &["init", "update"]),
    ("--with-gate", &["init", "update"]),
    ("--without-gate", &["init", "update"]),
    ("--no-selection", &["init", "update"]),
];

// spec: installer/SPEC.md §What init seeds — the consumer-layout names the verbs write against,
// which are the owning kits' own defaults, held here because a second verb reads what init wrote:
// uninstall trims the agent file, demo stages its claim in the queue and state file.
// consumer-value-exempt: the layout init writes a new tree against (installer/SPEC.md §What init seeds)
pub const GATES_DIR: &str = "scripts";
// spec: installer/SPEC.md §Profiles — the payload's one reserved sibling of the kit roots: the
// packer writes the prebuilt binaries there, so it is a payload directory that is not a kit
pub const ARTIFACT_DIR: &str = "artifact";
// consumer-value-exempt: the layout init writes a new tree against (installer/SPEC.md §What init seeds)
pub const AGENT_FILE: &str = "CLAUDE.md";
// consumer-value-exempt: the layout init writes a new tree against (installer/SPEC.md §What init seeds)
pub const QUEUE_FILE: &str = "TASK-QUEUE.md";
// consumer-value-exempt: the layout init writes a new tree against (installer/SPEC.md §What init seeds)
pub const WORKFLOW_DIR: &str = ".workflow";
// consumer-value-exempt: the layout init writes a new tree against (installer/SPEC.md §What init seeds)
pub const STATE_FILE: &str = ".workflow/WORKFLOW-STATE.txt";
// spec: installer/SPEC.md §The personal install — the markers bounding the block in the clone's
// `info/exclude` that makes an install personal and is its record of being so
pub const PERSONAL_BEGIN: &str = "# checkwright:personal:begin";
pub const PERSONAL_END: &str = "# checkwright:personal:end";

// spec: installer/SPEC.md §init — the rule that an install's size is never bounded by the host's
// argv width: every git call naming the whole roster goes through `git_batched`, so the batching is
// one implementation rather than a discipline each call site must remember.
// comment-tier-exempt: a native Windows process is handed at most 32767 command-line characters and
// MinGW's git is one, the tightest ceiling this runs under; the budget is a fraction of it so the
// fixed argv and the caller's own quoting cannot close the gap
const ARGV_BATCH_BYTES: usize = 4096;

// spec: gate-sdk/SPEC.md §check-reads-couples — the enumeration is the crate's one traversal,
// reported in byte order relative to its root, so a kit's file set and a vendored directory's
// contents read the same way and an unreadable tree is an error rather than a smaller corpus.
pub fn files_under(dir: &Path) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = walk::find_with_prune(dir, &|_| false)?
        .into_iter()
        .filter_map(|p| {
            let rel = p.strip_prefix(dir).ok()?;
            Some(
                rel.components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("/"),
            )
        })
        .filter(|r| !r.is_empty())
        .collect();
    out.sort();
    Ok(out)
}

// spec: installer/SPEC.md §The manifest — the gate binary's path is owned by the seam file's
// `GATE_SDK_NATIVE_BIN`, never a stored copy, so every verb asking where the binary is reads it here.
pub fn seam_binary(root: &std::path::Path, seam: &str) -> Option<String> {
    if seam.is_empty() {
        return None;
    }
    let text = std::fs::read_to_string(root.join(seam)).ok()?;
    let rel = crate::knobfile::parse(&text, seam)
        .ok()?
        .into_iter()
        .find(|e| e.name == "GATE_SDK_NATIVE_BIN" && e.form == crate::knobfile::Form::Scalar)?
        .value;
    let rel = rel.as_str();
    if rel.is_empty() || !root.join(rel).is_file() {
        return None;
    }
    Some(rel.to_string())
}

// spec: installer/SPEC.md §init — the first component of `rel` below `root`, the last included,
// that is a link or junction; a component the tree lacks ends the walk, and one that cannot be
// read is an error
pub fn link_below(root: &std::path::Path, rel: &str) -> Result<Option<String>, String> {
    let mut at = root.to_path_buf();
    let mut seen: Vec<&str> = Vec::new();
    for c in rel.split(['/', '\\']).filter(|c| !c.is_empty() && *c != ".") {
        at.push(c);
        seen.push(c);
        match std::fs::symlink_metadata(&at) {
            Ok(m) if m.file_type().is_symlink() => return Ok(Some(seen.join("/"))),
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("cannot read {}: {}", seen.join("/"), e)),
        }
    }
    Ok(None)
}

// spec: installer/SPEC.md §The verbs — every refusal carries the verb's own prefix, an optional
// `help:` line and an exit status, which is the published idiom rather than a second one.
#[derive(Debug)]
pub struct Refusal {
    pub message: String,
    pub help: String,
    pub code: i32,
}

pub fn refuse(message: impl Into<String>, help: impl Into<String>, code: i32) -> Refusal {
    Refusal {
        message: message.into(),
        help: help.into(),
        code,
    }
}

pub fn finish(verb: &str, outcome: Result<i32, Refusal>) -> i32 {
    match outcome {
        Ok(code) => code,
        Err(r) => report(&mut std::io::stderr(), verb, &r),
    }
}

pub fn report(err: &mut dyn std::io::Write, verb: &str, r: &Refusal) -> i32 {
    let _ = writeln!(err, "checkwright {}: {}", verb, r.message);
    if !r.help.is_empty() {
        let _ = writeln!(err, "  help: {}", r.help);
    }
    r.code
}

// spec: installer/SPEC.md §The verbs — `-h`/`--help` is intercepted first and answers on its own,
// outside every repository precondition, and an unknown argument is a usage refusal rather than a
// silently ignored token.
pub fn help_only(args: &[String], usage: &[&str]) -> Option<Result<i32, Refusal>> {
    match args.first().map(String::as_str) {
        None => None,
        Some("-h") | Some("--help") => {
            for line in usage {
                println!("{}", line);
            }
            Some(Ok(0))
        }
        Some(other) => Some(Err(refuse(format!("unknown argument: {}", other), "", 2))),
    }
}

// spec: installer/SPEC.md §The install boundary — the package the running artifact belongs to,
// derived from the artifact's own location rather than passed as argv: step 5's rule is
// unconditional, so it has no slot to inject a root into and the payload layout answers instead.
// spec: installer/SPEC.md §Layout — a binary reached from anywhere else, the copy `init` places
// in a consumer's tree most of all, resolves nothing here and gets the no-payload refusal.
pub struct Package {
    pub root: PathBuf,
    pub payload: PathBuf,
    pub artifact: PathBuf,
    pub target: String,
}

fn dir_named(p: Option<&Path>, name: &str) -> Option<PathBuf> {
    let d = p?;
    if d.file_name()?.to_string_lossy() == name {
        Some(d.to_path_buf())
    } else {
        None
    }
}

fn resolve_package() -> Option<Package> {
    let exe = crate::proc::self_path().ok()?;
    let target_dir = exe.parent()?;
    let target = target_dir.file_name()?.to_string_lossy().into_owned();
    let artifact_dir = dir_named(target_dir.parent(), ARTIFACT_DIR)?;
    let payload = dir_named(artifact_dir.parent(), "payload")?;
    let root = payload.parent()?.to_path_buf();
    if !root.join("package.json").is_file() {
        return None;
    }
    Some(Package {
        root,
        payload,
        artifact: exe,
        target,
    })
}

pub fn package(verb_action: &str) -> Result<Package, Refusal> {
    resolve_package().ok_or_else(|| {
        refuse(
            "this package carries no payload",
            format!(
                "{} out of the package's own payload/, assembled at pack time — run it from an installed package, not from a source checkout.",
                verb_action
            ),
            2,
        )
    })
}

// spec: installer/SPEC.md §init — the repository every verb's preconditions are about, taken
// through the crate's own crosser so the root is in one dialect (gate-sdk/SPEC.md §The crate's
// crosser) rather than in whichever one the host's git answers in.
pub fn repo_root() -> Result<Option<PathBuf>, String> {
    Ok(walk::toplevel_opt()?.map(PathBuf::from))
}

// spec: installer/SPEC.md §init — a verb needing the repository refuses outside a work tree with
// its own help, and in a repository git refuses with the crosser's sentence and a repair remedy
pub fn required_root(absent_help: &str) -> Result<PathBuf, Refusal> {
    match repo_root() {
        Ok(Some(root)) => Ok(root),
        Ok(None) => Err(refuse("not inside a git work tree", absent_help, 2)),
        Err(cause) => Err(refuse(
            cause,
            "git cannot answer for this repository, so the verb has no root to act on. Repair git's access to it (the command the cause names prints git's reason), then run the verb again.",
            2,
        )),
    }
}

pub fn git_capture(root: &Path, args: &[&str]) -> Result<String, String> {
    let root = root.to_string_lossy().into_owned();
    let mut argv: Vec<&str> = vec!["-C", &root];
    argv.extend_from_slice(args);
    let out = proc::run(&programs::GIT, &argv)?;
    Ok(out
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())
        .unwrap_or_default())
}

// spec: installer/SPEC.md §init — a git call read for its *status* rather than its stdout, which
// is what `git diff --cached --quiet` and `git ls-files --error-unmatch` are: `None` is a spawn
// failure, and folding it into either verdict is what a caller has to write down.
pub fn git_code(root: &Path, args: &[&str]) -> Option<i32> {
    let root = root.to_string_lossy().into_owned();
    let mut argv: Vec<&str> = vec!["-C", &root];
    argv.extend_from_slice(args);
    proc::run(&programs::GIT, &argv).ok()?.code()
}

// spec: installer/SPEC.md §init — the hooks whose non-zero exit stops a commit at once; another
// hook's exit may be one git ignores, so it names nothing
const STOPPING_HOOKS: &[&str] = &["pre-commit", "prepare-commit-msg", "commit-msg"];

static TRACE_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

// spec: installer/SPEC.md §init — a failed commit: git's own account, hook output included, and the
// hook git's trace records refusing it, where the trace establishes one
#[derive(Debug)]
pub struct Unmade {
    pub account: String,
    pub hook: Option<String>,
}

pub fn git_commit(root: &Path, message: &str) -> Result<(), Unmade> {
    let root = root.to_string_lossy().into_owned();
    let trace = trace_file();
    let env: Vec<(String, String)> = trace
        .iter()
        .map(|t| ("GIT_TRACE2_EVENT".to_string(), t.display().to_string()))
        .collect();
    let done = proc::run_merged_in(&programs::GIT, &["-C", &root, "commit", "-q", "-m", message], &env, None);
    let hook = trace.and_then(|t| {
        let text = std::fs::read_to_string(&t).ok();
        let _ = std::fs::remove_file(&t);
        refusing_hook(&text?)
    });
    match done {
        Ok(m) if m.succeeded() => Ok(()),
        Ok(m) => Err(Unmade {
            account: String::from_utf8_lossy(m.output()).trim_end().to_string(),
            hook,
        }),
        Err(account) => Err(Unmade { account, hook: None }),
    }
}

// spec: installer/SPEC.md §init — a fresh scratch file for the one spawned commit's trace, absolute
// because git ignores a relative trace target
fn trace_file() -> Option<PathBuf> {
    let base = PathBuf::from(walk::abs_against(&walk::cwd().ok()?, &proc::temp_base().display().to_string()));
    let t = base.join(format!(
        "checkwright-commit-trace.{}.{}",
        std::process::id(),
        TRACE_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_file(&t);
    Some(t)
}

// spec: installer/SPEC.md §init — the stopping hook the commit's own session ran and saw exit
// non-zero; the first event is the commit's, and a hook's nested git writes under a child session
fn refusing_hook(trace: &str) -> Option<String> {
    use serde_json::Value;
    let events: Vec<Value> = trace.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
    let sid = events.first()?.get("sid")?.as_str()?;
    let field = |e: &Value, k: &str| e.get(k).and_then(Value::as_str).map(str::to_string);
    let mut hooks: std::collections::BTreeMap<i64, String> = std::collections::BTreeMap::new();
    for e in events.iter().filter(|e| e.get("sid").and_then(Value::as_str) == Some(sid)) {
        let Some(id) = e.get("child_id").and_then(Value::as_i64) else { continue };
        match field(e, "event").as_deref() {
            Some("child_start") if field(e, "child_class").as_deref() == Some("hook") => {
                if let Some(name) = field(e, "hook_name") {
                    hooks.insert(id, name);
                }
            }
            Some("child_exit") => {
                let refused = e.get("code").and_then(Value::as_i64).is_some_and(|c| c != 0);
                match hooks.get(&id) {
                    Some(name) if refused && STOPPING_HOOKS.contains(&name.as_str()) => return Some(name.clone()),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    None
}

// spec: installer/SPEC.md §init — the directory a refusing hook ran from, read as install-hooks
// reads the prior one; the gate hooks directory, or a read that fails, establishes no adopter's hook
fn judged_in(root: &Path) -> Option<PathBuf> {
    let gate = crate::emit::hook_launcher::hooks_dir(root)?;
    match crate::emit::install_hooks::prior(&root.to_string_lossy(), &gate) {
        Ok(crate::emit::install_hooks::Prior::Quiet { dir, .. }) | Ok(crate::emit::install_hooks::Prior::Live { dir, .. }) => {
            Some(dir)
        }
        _ => None,
    }
}

// spec: installer/SPEC.md §init — what a verb's failed commit says: its line as it stands, and,
// once a refusing hook is established, the line naming it with the kit directories it must pass
pub struct Failed<'a> {
    pub plain: &'a str,
    pub staged: &'a str,
    pub judged: &'a str,
    pub valve: &'a str,
    pub kits: &'a [String],
}

pub fn commit_or_refuse(root: &Path, message: &str, failed: &Failed) -> Result<(), Refusal> {
    commit_or_refuse_to(&mut std::io::stderr(), root, message, failed)
}

// spec: installer/SPEC.md §init — the directory is read before the commit, so a hook that repoints
// `core.hooksPath` before refusing is named where it ran
pub fn commit_or_refuse_to(err: &mut dyn std::io::Write, root: &Path, message: &str, failed: &Failed) -> Result<(), Refusal> {
    let ran_in = judged_in(root);
    git_commit(root, message).map_err(|u| {
        if !u.account.is_empty() {
            let _ = writeln!(err, "{}", u.account);
        }
        match u.hook.zip(ran_in) {
            Some((hook, dir)) => refuse(
                format!("the commit failed, and your {} hook in {} judged it — {}", hook, dir.display(), failed.staged),
                format!(
                    "that hook judges every file {}. The kit directories ({}) are a vendored dependency: let it pass them, then commit the staged files yourself{}.",
                    failed.judged,
                    failed.kits.join(", "),
                    failed.valve
                ),
                1,
            ),
            None => refuse(failed.plain, "", 1),
        }
    })
}

// spec: installer/SPEC.md §init — the identity precondition: the commit needs the author and the
// committer both, read as git's strict read takes them, and a failed read relays git's account
pub fn identity_or_refuse(err: &mut dyn std::io::Write, root: &Path, valve: &str) -> Result<(), Refusal> {
    let root_s = root.to_string_lossy().into_owned();
    for half in ["GIT_AUTHOR_IDENT", "GIT_COMMITTER_IDENT"] {
        let account = match proc::run_merged_in(&programs::GIT, &["-C", &root_s, "var", half], &[], None) {
            Ok(m) if m.succeeded() => continue,
            Ok(m) => String::from_utf8_lossy(m.output()).trim_end().to_string(),
            Err(spawn) => spawn,
        };
        if !account.is_empty() {
            let _ = writeln!(err, "{}", account);
        }
        return Err(refuse(
            "git has no identity to make the commit with",
            format!(
                "set user.name and user.email with 'git config --global' (or without --global for this repository alone), or pass --no-commit to stage {} and commit it yourself.",
                valve
            ),
            1,
        ));
    }
    Ok(())
}

// spec: installer/SPEC.md §init — the roster goes to git in batches, so a large profile's install
// is not bounded by the host's argv width. The status is captured rather than discarded: a read
// that failed and returned nothing would look exactly like a clean tree.
pub fn git_batched(root: &Path, fixed: &[&str], paths: &[String]) -> Result<Vec<u8>, String> {
    let root_s = root.to_string_lossy().into_owned();
    let mut collected: Vec<u8> = Vec::new();
    for chunk in chunks(paths) {
        let mut argv: Vec<&str> = vec!["-C", &root_s];
        argv.extend_from_slice(fixed);
        argv.push("--");
        argv.extend(chunk.iter().map(String::as_str));
        let out = proc::run(&programs::GIT, &argv)?;
        match out.stdout() {
            Some(o) => collected.extend_from_slice(o),
            None => {
                return Err(out
                    .failure_report()
                    .unwrap_or_else(|| "git refused the call".to_string()))
            }
        }
    }
    Ok(collected)
}

// spec: installer/SPEC.md §init — the batching itself, separated from the spawn so the boundary
// rule is asserted over a corpus rather than over a hundred processes: an empty roster is no call
// at all, and a single path wider than the budget still gets its own call rather than none.
fn chunks(paths: &[String]) -> Vec<&[String]> {
    let mut out: Vec<&[String]> = Vec::new();
    let (mut start, mut used) = (0usize, 0usize);
    for (i, p) in paths.iter().enumerate() {
        if i > start && used + p.len() + 1 > ARGV_BATCH_BYTES {
            out.push(&paths[start..i]);
            start = i;
            used = 0;
        }
        used += p.len() + 1;
    }
    if start < paths.len() {
        out.push(&paths[start..]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn usage_line(verb: &str) -> String {
        match verb {
            "init" => init::USAGE.to_string(),
            "update" => update::USAGE[0].to_string(),
            "uninstall" => uninstall::USAGE[0].to_string(),
            "doctor" => doctor::USAGE[0].to_string(),
            "diff" => diff::usage()[0].clone(),
            "demo" => demo::USAGE[0].to_string(),
            "prove" => prove::USAGE[0].to_string(),
            other => panic!("VERBS carries `{}`, which this test reads no usage line for", other),
        }
    }

    // spec: installer/SPEC.md §init — a link at a directory component or at the file itself is
    // named, and a plain path or one the tree lacks passes
    #[cfg(unix)]
    #[test]
    fn a_destination_through_a_link_is_named_at_its_first_link() {
        let dir = std::env::temp_dir().join(format!("cw-link-below-{}", std::process::id()));
        let outside = std::env::temp_dir().join(format!("cw-link-below-{}-outside", std::process::id()));
        for d in [&dir, &outside] {
            std::fs::remove_dir_all(d).ok();
            std::fs::create_dir_all(d).expect("cannot make the scratch dirs");
        }
        std::fs::create_dir_all(dir.join("kit/sub")).expect("cannot make the kit dir");
        std::fs::write(dir.join("kit/sub/a.sh"), "x\n").expect("cannot write a plain file");
        std::fs::write(outside.join("f.sh"), "x\n").expect("cannot write the link target");
        std::os::unix::fs::symlink(&outside, dir.join("linked")).expect("cannot link a directory");
        std::os::unix::fs::symlink(outside.join("f.sh"), dir.join("kit/f.sh")).expect("cannot link a file");
        assert_eq!(link_below(&dir, "kit/sub/a.sh"), Ok(None), "a plain path");
        assert_eq!(link_below(&dir, "absent/x.sh"), Ok(None), "a path the tree lacks");
        assert_eq!(link_below(&dir, "linked/deeper/x.sh"), Ok(Some("linked".to_string())), "a directory link");
        assert_eq!(link_below(&dir, "kit/f.sh"), Ok(Some("kit/f.sh".to_string())), "a file link");
        for d in [&dir, &outside] {
            std::fs::remove_dir_all(d).ok();
        }
    }

    // spec: installer/SPEC.md §init — a junction, native Windows' unprivileged directory link, is
    // named at the junction under either separator
    #[cfg(windows)]
    #[test]
    fn a_destination_through_a_junction_is_named_at_the_junction() {
        let dir = std::env::temp_dir().join(format!("cw-junction-below-{}", std::process::id()));
        let outside = std::env::temp_dir().join(format!("cw-junction-below-{}-outside", std::process::id()));
        for d in [&dir, &outside] {
            std::fs::remove_dir_all(d).ok();
            std::fs::create_dir_all(d).expect("cannot make the scratch dirs");
        }
        std::fs::create_dir_all(dir.join("kit")).expect("cannot make the kit dir");
        let junction = dir.join("kit").join("linked");
        let status = std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&junction)
            .arg(&outside)
            .status()
            .expect("cmd did not spawn");
        assert!(status.success(), "mklink /J failed: {}", status);
        assert_eq!(link_below(&dir, "kit/linked/x.sh"), Ok(Some("kit/linked".to_string())), "a junction, `/`-separated");
        assert_eq!(link_below(&dir, "kit\\linked\\x.sh"), Ok(Some("kit/linked".to_string())), "a junction, `\\`-separated");
        std::fs::remove_dir(&junction).expect("cannot remove the junction");
        for d in [&dir, &outside] {
            std::fs::remove_dir_all(d).ok();
        }
    }

    // spec: installer/SPEC.md §init — a hook's refusal reaches the caller as the commit's account,
    // rather than as a bare failure that cannot say what stopped it
    #[test]
    fn a_refused_commit_carries_the_hooks_own_account() {
        let dir = std::env::temp_dir().join(format!("cw-commit-account-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(dir.join("hooks")).expect("cannot make the scratch repo");
        let git = |args: &[&str]| {
            let d = dir.to_string_lossy().into_owned();
            let mut argv = vec!["-C", d.as_str()];
            argv.extend_from_slice(args);
            proc::run(&programs::GIT, &argv).expect("git did not spawn");
        };
        git(&["init", "-q"]);
        git(&["config", "user.name", "Scratch"]);
        git(&["config", "user.email", "scratch@example.com"]);
        git(&["config", "core.hooksPath", "hooks"]);
        let hook = dir.join("hooks").join("pre-commit");
        std::fs::write(&hook, "#!/bin/sh\necho 'the hook refused this commit' >&2\nexit 1\n")
            .expect("cannot write the hook");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755))
                .expect("cannot mark the hook executable");
        }
        std::fs::write(dir.join("f.txt"), "x\n").expect("cannot write the staged file");
        git(&["add", "f.txt"]);
        let account = git_commit(&dir, "scratch").expect_err("the hook's refusal was not a failure").account;
        assert!(account.contains("the hook refused this commit"), "the account dropped the hook's words: {}", account);
        std::fs::remove_file(&hook).expect("cannot remove the hook");
        assert!(git_commit(&dir, "scratch").is_ok(), "a commit with no hook in the way failed");
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §init — a local identity passes, and an empty local `user.name` refuses
    // under any host's global config, with git's account and the verb's own valve
    #[test]
    fn the_identity_precondition_refuses_what_gits_strict_read_refuses() {
        let dir = std::env::temp_dir().join(format!("cw-identity-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("cannot make the scratch repo");
        let git = |args: &[&str]| {
            let d = dir.to_string_lossy().into_owned();
            let mut argv = vec!["-C", d.as_str()];
            argv.extend_from_slice(args);
            proc::run(&programs::GIT, &argv).expect("git did not spawn");
        };
        git(&["init", "-q"]);
        git(&["config", "user.name", "Scratch"]);
        git(&["config", "user.email", "scratch@example.com"]);
        let mut err = Vec::new();
        assert!(identity_or_refuse(&mut err, &dir, "the vendoring").is_ok(), "a set identity was refused");
        assert!(err.is_empty(), "a passing read printed: {}", String::from_utf8_lossy(&err));
        git(&["config", "user.name", ""]);
        let r = identity_or_refuse(&mut err, &dir, "the vendoring").expect_err("an empty name passed");
        assert_eq!((r.message.as_str(), r.code), ("git has no identity to make the commit with", 1));
        assert!(r.help.contains("--no-commit to stage the vendoring and commit it yourself"), "{}", r.help);
        assert!(String::from_utf8_lossy(&err).contains("fatal: "), "git's account was not relayed");
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §The verbs — a usage line's flags, each with whether a `<operand>`
    // follows it
    fn usage_flags(line: &str) -> Vec<(String, bool)> {
        let toks: Vec<String> = line
            .split_whitespace()
            .map(|t| t.trim_start_matches('[').trim_end_matches("...").trim_end_matches(']').to_string())
            .collect();
        toks.iter()
            .enumerate()
            .filter(|(_, t)| t.starts_with("--") && t.as_str() != "--help")
            .map(|(i, t)| (t.clone(), toks.get(i + 1).is_some_and(|n| n.starts_with('<'))))
            .collect()
    }

    // spec: installer/SPEC.md §The verbs — `update` forwards its argv to `init` verbatim, so its
    // parser is init's; `doctor`, `diff`, `demo` and `prove` take no flag and answer only through `run`
    fn refused_at(verb: &str, args: &[String]) -> Option<i32> {
        match verb {
            "init" | "update" => init::parse(args, None).err().map(|r| r.code),
            "uninstall" => uninstall::parse(args).err().map(|r| r.code),
            _ => {
                let (_, run) = VERBS.iter().find(|(v, _)| v.trim_start_matches("--") == verb)?;
                Some(run(args)).filter(|c| *c != 0)
            }
        }
    }

    // spec: installer/SPEC.md §The verbs — each row's flag parses for every verb it names, in both
    // spellings where it takes an operand; a flag no row names for a verb is refused at exit 2 by
    // that verb; and each verb's `usage:` line lists exactly its rows
    #[test]
    fn the_flag_roster_is_every_parsers_and_usage_lines() {
        for (verb, _) in VERBS {
            let verb = verb.trim_start_matches("--");
            let rows: BTreeSet<&str> = FLAGS.iter().filter(|(_, vs)| vs.contains(&verb)).map(|(f, _)| *f).collect();
            let listed = usage_flags(&usage_line(verb));
            let names: BTreeSet<&str> = listed.iter().map(|(f, _)| f.as_str()).collect();
            assert_eq!(names, rows, "{}'s usage line and its FLAGS rows disagree", verb);
            for (flag, operand) in &listed {
                let mut spellings = vec![vec![flag.clone()]];
                if *operand {
                    spellings = vec![vec![flag.clone(), "x".to_string()], vec![format!("{}=x", flag)]];
                }
                for args in spellings {
                    assert_eq!(refused_at(verb, &args), None, "{} refused {:?}", verb, args);
                }
            }
            for (flag, _) in FLAGS.iter().filter(|(_, vs)| !vs.contains(&verb)) {
                assert_eq!(refused_at(verb, &[flag.to_string()]), Some(2), "{} did not refuse {}", verb, flag);
            }
        }
    }

    // spec: installer/SPEC.md §init — every path lands in exactly one chunk, no chunk exceeds
    // the budget, an empty roster is no call at all, and a single path wider than the budget still
    // gets a call of its own rather than none.
    #[test]
    fn every_path_lands_in_one_chunk_and_no_chunk_exceeds_the_budget() {
        let paths: Vec<String> = (0..400)
            .map(|i| format!("kit/file-{:04}-with-a-long-enough-name.txt", i))
            .collect();
        let total: usize = paths.iter().map(|p| p.len() + 1).sum();
        assert!(total > ARGV_BATCH_BYTES, "the corpus does not force a chunk");

        let cut = chunks(&paths);
        assert!(cut.len() > 1, "a corpus past the budget was issued in one call");
        let flat: Vec<&String> = cut.iter().flat_map(|c| c.iter()).collect();
        assert_eq!(flat.len(), paths.len(), "a chunk was dropped");
        assert!(
            flat.iter().zip(paths.iter()).all(|(a, b)| *a == b),
            "the roster's order did not survive chunking"
        );
        for c in &cut {
            let width: usize = c.iter().map(|p| p.len() + 1).sum();
            assert!(
                width <= ARGV_BATCH_BYTES || c.len() == 1,
                "a chunk of {} paths exceeds the budget at {} bytes",
                c.len(),
                width
            );
        }

        assert!(chunks(&[]).is_empty(), "an empty roster is not a call");
        let wide = vec!["x".repeat(ARGV_BATCH_BYTES * 2)];
        assert_eq!(chunks(&wide).len(), 1, "an over-wide path lost its call");
    }

    // spec: installer/SPEC.md §The personal install — the state file is a process record, so the
    // workflow directory the refusal tests against holds it
    #[test]
    fn the_state_file_sits_under_the_workflow_directory() {
        assert!(walk::under(WORKFLOW_DIR, STATE_FILE), "{} is not under {}", STATE_FILE, WORKFLOW_DIR);
    }
}
