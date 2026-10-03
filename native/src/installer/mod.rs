// spec: installer/SPEC.md §The verbs — the adopter verbs, behind the invoke: each a
// top-level `--`-prefixed arm resolved before the registry lookup and absent from `--list`, taking
// every value as argv because the caller is a bootstrap that may run no POSIX shell.
pub mod demo;
pub mod diff;
pub mod doctor;
pub mod init;
pub mod lock;
pub mod payload_recipe;
pub mod profile;
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
pub const STATE_FILE: &str = ".workflow/WORKFLOW-STATE.txt";

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
        Err(r) => {
            eprintln!("checkwright {}: {}", verb, r.message);
            if !r.help.is_empty() {
                eprintln!("  help: {}", r.help);
            }
            r.code
        }
    }
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
    let exe = std::env::current_exe().ok()?;
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
pub fn repo_root() -> Option<PathBuf> {
    walk::toplevel_opt().ok().flatten().map(PathBuf::from)
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

// spec: installer/SPEC.md §init — a failed commit is fatal, and its `Err` is git's own account,
// hook output included, since the verb's refusal line cannot say which of them stopped it
pub fn git_commit(root: &Path, message: &str) -> Result<(), String> {
    let root = root.to_string_lossy().into_owned();
    let done = proc::run(&programs::GIT, &["-C", &root, "commit", "-q", "-m", message])?;
    if done.stdout().is_some() {
        return Ok(());
    }
    let (out, err) = done.streams();
    let account: Vec<String> = [out, err]
        .iter()
        .map(|s| String::from_utf8_lossy(s).trim_end().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Err(account.join("\n"))
}

pub fn commit_or_refuse(root: &Path, message: &str, refusal: &str) -> Result<(), Refusal> {
    git_commit(root, message).map_err(|account| {
        if !account.is_empty() {
            eprintln!("{}", account);
        }
        refuse(refusal, "", 1)
    })
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
            other => panic!("VERBS carries `{}`, which this test reads no usage line for", other),
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
        let account = git_commit(&dir, "scratch").expect_err("the hook's refusal was not a failure");
        assert!(account.contains("the hook refused this commit"), "the account dropped the hook's words: {}", account);
        std::fs::remove_file(&hook).expect("cannot remove the hook");
        assert!(git_commit(&dir, "scratch").is_ok(), "a commit with no hook in the way failed");
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
    // parser is init's; `doctor`, `diff` and `demo` take no flag and answer only through `run`
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
}
