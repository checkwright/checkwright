// spec: gate-sdk/SPEC.md §git-hook — the binary under a served hook's name: it starts the binary
// `GATE_SDK_NATIVE_BIN` names on the `--git-hook` arm and returns its status, never judging itself
// spec: gate-sdk/SPEC.md §install-hooks — the hooks directory's one derivation, read by the arm that
// places it and by the installer verbs that report and remove it
use crate::knobs::{self, gate_sdk};
use crate::{knobfile, proc, programs, walk};
use std::path::{Path, PathBuf};

pub const SERVED: &[&str] = &["pre-commit", "commit-msg"];

pub const DIR: &str = "gate-hooks";

// spec: gate-sdk/SPEC.md §install-hooks — githooks(5)'s roster at the git release this build read
pub const GIT_HOOKS: &[&str] = &[
    "applypatch-msg",
    "pre-applypatch",
    "post-applypatch",
    "pre-commit",
    "pre-merge-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-rebase",
    "post-checkout",
    "post-merge",
    "pre-push",
    "pre-receive",
    "update",
    "proc-receive",
    "post-receive",
    "post-update",
    "reference-transaction",
    "push-to-checkout",
    "pre-auto-gc",
    "post-rewrite",
    "sendemail-validate",
    "fsmonitor-watchman",
    "p4-changelist",
    "p4-prepare-changelist",
    "p4-post-changelist",
    "p4-pre-submit",
    "post-index-change",
];

fn git_hook_under(name: &str, suffix: &str) -> bool {
    GIT_HOOKS.contains(&name) || GIT_HOOKS.contains(&strip_suffix_of(name, suffix))
}

pub fn git_hook_named(name: &str) -> bool {
    git_hook_under(name, std::env::consts::EXE_SUFFIX)
}

const BIN_KNOB: &str = "GATE_SDK_NATIVE_BIN";

fn strip_suffix_of<'a>(name: &'a str, suffix: &str) -> &'a str {
    if suffix.is_empty() || name.len() <= suffix.len() || !name.is_char_boundary(name.len() - suffix.len()) {
        return name;
    }
    let (stem, tail) = name.split_at(name.len() - suffix.len());
    if tail.eq_ignore_ascii_case(suffix) {
        stem
    } else {
        name
    }
}

fn served_under(started_as: &str, suffix: &str) -> Option<&'static str> {
    let name = Path::new(started_as).file_name()?.to_str()?;
    let stem = strip_suffix_of(name, suffix);
    SERVED.iter().copied().find(|h| *h == stem)
}

pub fn served(started_as: &str) -> Option<&'static str> {
    served_under(started_as, std::env::consts::EXE_SUFFIX)
}

// spec: gate-sdk/SPEC.md §install-hooks — a hook file's name with the host's executable suffix removed
pub fn stem(name: &str) -> &str {
    strip_suffix_of(name, std::env::consts::EXE_SUFFIX)
}

pub fn file_name(hook: &str) -> String {
    format!("{}{}", hook, std::env::consts::EXE_SUFFIX)
}

pub fn hooks_dir(root: &Path) -> Option<PathBuf> {
    let r = root.to_string_lossy().into_owned();
    let out = proc::run(&programs::GIT, &["-C", &r, "rev-parse", "--git-common-dir"]).ok()?;
    let dir = String::from_utf8_lossy(out.stdout()?).trim().to_string();
    if dir.is_empty() {
        return None;
    }
    Some(PathBuf::from(walk::abs_against(&r, &dir)).join(DIR))
}

// spec: gate-sdk/SPEC.md §install-hooks — two directories are one where both resolve physically to
// one path, a part that does not exist yet kept as spelled
pub fn same_dir(a: &Path, b: &Path) -> bool {
    let physical = |p: &Path| walk::physical_against(".", &p.display().to_string());
    physical(a) == physical(b)
}

// spec: gate-sdk/SPEC.md §install-hooks — opted in is `core.hooksPath` naming the hooks directory
pub fn opted_in(root: &Path, dir: &Path) -> bool {
    let r = root.to_string_lossy().into_owned();
    let Ok(out) = proc::run(&programs::GIT, &["-C", &r, "config", "--get", "core.hooksPath"]) else {
        return false;
    };
    let value = String::from_utf8_lossy(out.stdout().unwrap_or_default()).trim().to_string();
    !value.is_empty() && same_dir(Path::new(&walk::physical_against(&r, &value)), dir)
}

pub fn missing(dir: &Path) -> Vec<String> {
    SERVED.iter().map(|h| file_name(h)).filter(|f| !dir.join(f).is_file()).collect()
}

fn refusal(hook: &str, bin: &str, cause: &str) -> String {
    format!(
        "{}: cannot start the gate binary at {} ({}) — the hook could not run; treating as failure (not clean)\n  help: place or build the binary {} names, or bypass once with: git commit --no-verify",
        hook, bin, cause, BIN_KNOB
    )
}

// spec: gate-sdk/SPEC.md §git-hook — the launcher's read of its one knob: a scalar's precedence over
// the two knob files, each parsed only on the lines naming it, so a name this build's table lacks is
// the started build's to judge
fn bin_knob() -> Result<String, String> {
    let set = |var: String| std::env::var(var).ok().filter(|v| !v.is_empty());
    let named = set(gate_sdk::KIT.knob_file_var());
    let local = set(gate_sdk::KIT.local_knob_file_var());
    bin_from(std::env::var(BIN_KNOB).ok(), &knobs::gates_dir(), named.as_deref(), local.as_deref())
}

fn bin_from(env: Option<String>, dir: &str, named: Option<&str>, local: Option<&str>) -> Result<String, String> {
    let stem = gate_sdk::KIT.stem();
    let layer = |locator: Option<&str>, var: String, infix: &str| match locator {
        Some(f) => in_file(f, Some(&var)),
        None => in_file(&format!("{}/{}-config{}.knobs", dir, stem, infix), None),
    };
    let set = match env {
        Some(v) => Some(v),
        None => match layer(local, gate_sdk::KIT.local_knob_file_var(), ".local")? {
            Some(v) => Some(v),
            None => layer(named, gate_sdk::KIT.knob_file_var(), "")?,
        },
    };
    Ok(set.filter(|v| !v.is_empty()).unwrap_or_else(gate_sdk::host_native_bin))
}

fn in_file(path: &str, named_by: Option<&str>) -> Result<Option<String>, String> {
    if !Path::new(path).is_file() {
        if let Some(var) = named_by {
            return Err(format!("{} names {}, which does not exist, so {} cannot be read", var, path, BIN_KNOB));
        }
        return Ok(None);
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    let mut found: Option<String> = None;
    for (idx, line) in text.lines().enumerate().filter(|(_, l)| knobfile::names(l, BIN_KNOB)) {
        let lno = idx + 1;
        match knobfile::parse_line(line, path, lno)? {
            Some(e) if e.form == knobfile::Form::Scalar && found.is_none() => found = Some(e.value),
            Some(e) if e.form == knobfile::Form::Scalar => {
                return Err(format!("{}:{}: {} is given twice — keep one line for it", path, lno, BIN_KNOB));
            }
            _ => {
                return Err(format!(
                    "{}:{}: {} is a scalar and this line is not that form — write `{} = value`",
                    path, lno, BIN_KNOB, BIN_KNOB
                ));
            }
        }
    }
    Ok(found)
}

// spec: gate-sdk/SPEC.md §install-hooks — the chain record's file, beside the placed hooks
pub const CHAIN: &str = "chain";

pub const CHAIN_DIR_KEY: &str = "dir=";

pub const CHAIN_RESTORE_KEY: &str = "restore=";

// spec: gate-sdk/SPEC.md §install-hooks — the prior hooks directory a placed hook chains, and the
// local `core.hooksPath` value the restore line names
#[derive(Debug, PartialEq)]
pub struct Chain {
    pub dir: PathBuf,
    pub restore: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shell {
    Posix,
    PowerShell,
}

impl Shell {
    pub fn host() -> Shell {
        if cfg!(windows) { Shell::PowerShell } else { Shell::Posix }
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — one single-quoted word in the shell form named, so the
// pasted restore sets the recorded value byte for byte
pub fn shell_quoted(value: &str, shell: Shell) -> String {
    let mut out = String::from("'");
    for c in value.chars() {
        match (shell, c) {
            (Shell::Posix, '\'') => out.push_str("'\\''"),
            (Shell::PowerShell, '\'' | '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}') => {
                out.push(c);
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out.push('\'');
    out
}

// spec: gate-sdk/SPEC.md §install-hooks — the restore is printed, never written: the receipt's and
// `uninstall`'s command, the recorded local value where there is one
pub fn restore_line(chain: Option<&Chain>) -> String {
    restore_line_in(chain, Shell::host())
}

fn restore_line_in(chain: Option<&Chain>, shell: Shell) -> String {
    match chain.map(|c| c.restore.as_str()).filter(|r| !r.is_empty()) {
        Some(value) => format!("git config core.hooksPath {}", shell_quoted(value, shell)),
        None => "git config --unset core.hooksPath".to_string(),
    }
}

// spec: gate-sdk/SPEC.md §git-hook — a record that cannot be read refuses, since reading past it
// would drop the prior hooks silently; one naming the hooks directory itself is no record
pub fn read_chain(hooks_dir: &Path) -> Result<Option<Chain>, String> {
    let path = hooks_dir.join(CHAIN);
    if path.symlink_metadata().is_err() {
        return Ok(None);
    }
    let cannot = |cause: String| format!("cannot read the chain record {} ({})", path.display(), cause);
    let text = std::fs::read_to_string(&path).map_err(|e| cannot(e.to_string()))?;
    let field = |key: &str| text.lines().find_map(|l| l.trim_end_matches('\r').strip_prefix(key).map(str::to_string));
    let dir = field(CHAIN_DIR_KEY).filter(|d| !d.is_empty()).ok_or_else(|| cannot(format!("no {} line", CHAIN_DIR_KEY)))?;
    let chain = Chain { dir: PathBuf::from(dir), restore: field(CHAIN_RESTORE_KEY).unwrap_or_default() };
    Ok((!same_dir(&chain.dir, hooks_dir)).then_some(chain))
}

// spec: gate-sdk/SPEC.md §git-hook — the start name's directory, where the chain record sits
fn start_dir(started_as: &str) -> Option<&Path> {
    Path::new(started_as).parent().filter(|p| !p.as_os_str().is_empty())
}

// spec: gate-sdk/SPEC.md §git-hook — a forwarding hook is the binary under a git hook's name it
// does not serve, with the chain record beside it; any other such start name parses arms
pub fn forward(started_as: &str) -> Option<&'static str> {
    let name = Path::new(started_as).file_name()?.to_str()?;
    let hook = GIT_HOOKS.iter().copied().find(|h| *h == stem(name) && !SERVED.contains(h))?;
    start_dir(started_as)?.join(CHAIN).symlink_metadata().ok().map(|_| hook)
}

// spec: gate-sdk/SPEC.md §git-hook — the recorded directory's live hook of this name, bare ahead of
// suffixed as git looks it up
fn live_named(dir: &Path, hook: &str) -> Result<Option<PathBuf>, String> {
    let names = super::install_hooks::live_hooks(dir)?;
    Ok(names
        .iter()
        .find(|n| stem(n) == hook)
        .map(|n| dir.join(n)))
}

fn chain_refusal(hook: &str, cause: &str) -> String {
    format!("{}: {} — the hook could not run; treating as failure (not clean)", hook, cause)
}

// spec: gate-sdk/SPEC.md §git-hook — the chained start: `Ok(None)` is no record or no live hook of
// this name, `Ok(Some(_))` the prior hook's path and status, `Err` the refusal line
fn run_chained(hook: &str, started_as: &str, args: &[String]) -> Result<Option<(PathBuf, i32)>, String> {
    let Some(dir) = start_dir(started_as) else {
        return Ok(None);
    };
    let chain = read_chain(dir).map_err(|e| chain_refusal(hook, &e))?;
    let Some(chain) = chain else {
        return Ok(None);
    };
    let Some(path) = live_named(&chain.dir, hook).map_err(|e| chain_refusal(hook, &e))? else {
        return Ok(None);
    };
    let code = start_prior(hook, &chain.dir, &path, args).map_err(|e| format!("{}: {}", hook, e))?;
    Ok(Some((path, code)))
}

// spec: gate-sdk/SPEC.md §git-hook — on unix the prior hook starts directly, as git starts it
#[cfg(not(windows))]
fn start_prior(_hook: &str, _dir: &Path, path: &Path, args: &[String]) -> Result<i32, String> {
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let prior = programs::Program::consumer(programs::CHAINED_HOOK, path.display().to_string());
    proc::run_to(&prior, &argv, &proc::Sink::Inherit)
}

// spec: gate-sdk/SPEC.md §git-hook — on native Windows git's own lookup and shebang handling start
// it, with the standard input git gave the hook handed through a scratch file
#[cfg(windows)]
fn start_prior(hook: &str, dir: &Path, _path: &Path, args: &[String]) -> Result<i32, String> {
    use std::io::Read;
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).map_err(|e| format!("cannot read the hook's standard input: {}", e))?;
    let file = scratch_input(&input)?;
    let hooks_path = format!("core.hooksPath={}", dir.display());
    let to_stdin = format!("--to-stdin={}", file.display());
    let mut argv: Vec<&str> = vec!["-c", hooks_path.as_str(), "hook", "run", "--ignore-missing", to_stdin.as_str(), hook, "--"];
    argv.extend(args.iter().map(String::as_str));
    let ran = proc::run_to(&programs::GIT, &argv, &proc::Sink::Inherit);
    let _ = std::fs::remove_file(&file);
    ran
}

// spec: gate-sdk/SPEC.md §git-hook — the scratch file under the system temp dir, created new
#[cfg(windows)]
fn scratch_input(bytes: &[u8]) -> Result<PathBuf, String> {
    use std::hash::{BuildHasher, Hasher};
    use std::io::Write;
    let drawn = std::collections::hash_map::RandomState::new().build_hasher().finish();
    let file = proc::temp_base().join(format!("checkwright-chain-{}.{:016x}", std::process::id(), drawn));
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file)
        .map_err(|e| format!("cannot create {}: {}", file.display(), e))?;
    let written = f.write_all(bytes);
    drop(f);
    if let Err(e) = written {
        let _ = std::fs::remove_file(&file);
        return Err(format!("cannot write {}: {}", file.display(), e));
    }
    Ok(file)
}

// spec: gate-sdk/SPEC.md §git-hook — the forwarding hook runs the prior hook and returns its
// status, reading no knob and starting no gate binary; with no live hook of its name it exits 0
pub fn forward_run(hook: &str, started_as: &str, args: &[String]) -> i32 {
    match run_chained(hook, started_as, args) {
        Ok(Some((_, code))) => code,
        Ok(None) => 0,
        Err(e) => {
            eprintln!("{}", e);
            2
        }
    }
}

pub fn launch(hook: &str, started_as: &str, args: &[String]) -> i32 {
    let bin = match bin_knob() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}: {} — the hook could not run; treating as failure (not clean)", hook, e);
            return 2;
        }
    };
    if served(&bin).is_some() {
        eprintln!(
            "{}: {} names {}, itself a hook's name, so the launcher would start itself — the hook could not run; treating as failure (not clean)\n  help: point {} at the gate binary",
            hook, BIN_KNOB, bin, BIN_KNOB
        );
        return 2;
    }
    match run_chained(hook, started_as, args) {
        Ok(None) | Ok(Some((_, 0))) => {}
        Ok(Some((path, code))) => {
            eprintln!("{}: the chained hook {} exited {} — the gates did not run", hook, path.display(), code);
            return code;
        }
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    }
    let mut argv: Vec<&str> = vec!["--git-hook", hook];
    argv.extend(args.iter().map(String::as_str));
    match proc::run_to(&programs::CHECKWRIGHT_GATES.at(bin.clone()), &argv, &proc::Sink::Inherit) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}", refusal(hook, &bin, &e));
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §install-hooks — a directory not placed yet, reached through a link,
    // is the one its physical spelling names, as on a host whose temp dir sits behind one
    #[cfg(unix)]
    #[test]
    fn an_absent_directory_through_a_link_is_its_physical_spelling() {
        let d = std::env::temp_dir().join(format!("checkwright-same-dir.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("real/.git")).expect("scratch");
        std::os::unix::fs::symlink(d.join("real"), d.join("alias")).expect("symlink");
        let logical = d.join("alias/.git/gate-hooks");
        let physical = PathBuf::from(walk::canonicalize(d.join("real")).expect("resolves")).join(".git/gate-hooks");
        let same = same_dir(&logical, &physical);
        let other = same_dir(&logical, &d.join("real/.git/hooks"));
        let _ = std::fs::remove_dir_all(&d);
        assert!(same, "{} and {} name one directory", logical.display(), physical.display());
        assert!(!other, "a sibling read as the same directory");
    }

    // spec: gate-sdk/SPEC.md §git-hook — the served set is closed, read off the start name's stem
    // with the host's executable suffix removed
    #[test]
    fn only_a_served_hook_name_under_the_host_suffix_is_a_launcher() {
        assert_eq!(served_under("/r/.git/gate-hooks/pre-commit", ""), Some("pre-commit"));
        assert_eq!(served_under("commit-msg", ""), Some("commit-msg"));
        assert_eq!(served_under("C:/r/.git/gate-hooks/pre-commit.exe", ".exe"), Some("pre-commit"));
        assert_eq!(served_under("commit-msg.EXE", ".exe"), Some("commit-msg"));
        assert_eq!(served_under("pre-commit", ".exe"), Some("pre-commit"));
        for other in ["pre-commit.exe", "pre-push", "checkwright-gates", "scripts/pre-commit.sh", ""] {
            assert_eq!(served_under(other, ""), None, "{}", other);
        }
        assert_eq!(served_under("pre-push.exe", ".exe"), None);
        assert_eq!(served_under(".exe", ".exe"), None);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a git hook's name, bare or under the host suffix, and
    // never a sample
    #[test]
    fn a_git_hook_name_is_read_bare_or_under_the_host_suffix() {
        assert!(git_hook_under("pre-push", ""));
        assert!(git_hook_under("pre-push", ".exe"));
        assert!(git_hook_under("pre-push.EXE", ".exe"));
        for other in ["pre-commit.sample", "pre-push.exe", "husky.sh", "gate-hooks", ""] {
            assert!(!git_hook_under(other, ""), "{}", other);
        }
        assert!(!git_hook_under("pre-commit.sample", ".exe"));
    }

    // spec: gate-sdk/SPEC.md §git-hook — a binary that cannot be started is refused naming the
    // resolved path and both remedies
    #[test]
    fn the_refusal_names_the_path_and_both_remedies() {
        let r = refusal("pre-commit", "scripts/absent", "no such file");
        assert!(r.starts_with("pre-commit: cannot start the gate binary at scripts/absent"), "{}", r);
        assert!(r.contains("place or build the binary GATE_SDK_NATIVE_BIN names"), "{}", r);
        assert!(r.contains("git commit --no-verify"), "{}", r);
    }

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("checkwright-hook-launcher-{}.{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        d
    }

    // spec: gate-sdk/SPEC.md §git-hook — the launcher reads its one knob by name: a line naming any
    // other knob, a name no table holds and a line no grammar admits among them, is skipped unread
    #[test]
    fn the_read_returns_the_named_binary_past_lines_no_table_holds() {
        let d = scratch("read");
        let dir = d.display().to_string();
        let tracked = d.join("gate-sdk-config.knobs");
        let unheld = format!("{}MINTED_AFTER_THIS_BUILD", gate_sdk::KIT.prefix());
        std::fs::write(
            &tracked,
            format!("{} = x\nnot a knob line at all\n{}_OTHER[] = y\n  {} = bin/tracked\n", unheld, BIN_KNOB, BIN_KNOB),
        )
        .expect("write");
        let from_tracked = bin_from(None, &dir, None, None);
        let from_env = bin_from(Some("bin/env".to_string()), &dir, None, None);
        let empty_env = bin_from(Some(String::new()), &dir, None, None);
        std::fs::write(d.join("gate-sdk-config.local.knobs"), format!("{}[k] = 1\n{} = bin/local\n", unheld, BIN_KNOB)).expect("write");
        let from_local = bin_from(None, &dir, None, None);
        let named = d.join("elsewhere.knobs");
        std::fs::write(&named, "GATE_SDK_NATIVE_BIN = bin/named\n").expect("write");
        let overlay = d.join("elsewhere.local.knobs");
        std::fs::write(&overlay, "").expect("write");
        let past_named_overlay = bin_from(None, &dir, None, Some(&overlay.display().to_string()));
        std::fs::write(&overlay, "GATE_SDK_NATIVE_BIN = bin/named-local\n").expect("write");
        let from_named_overlay = bin_from(None, &dir, Some(&named.display().to_string()), Some(&overlay.display().to_string()));
        let local_past_named = bin_from(None, &dir, Some(&named.display().to_string()), None);
        std::fs::write(d.join("gate-sdk-config.local.knobs"), "GATE_SDK_NATIVE_BIN =\n").expect("write");
        let empty_local = bin_from(None, &dir, None, None);
        std::fs::remove_file(d.join("gate-sdk-config.local.knobs")).expect("remove");
        let from_named = bin_from(None, &dir, Some(&named.display().to_string()), None);
        std::fs::remove_file(&tracked).expect("remove");
        let from_default = bin_from(None, &dir, None, None);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(past_named_overlay, Ok("bin/tracked".to_string()));
        assert_eq!(from_named_overlay, Ok("bin/named-local".to_string()));
        assert_eq!(local_past_named, Ok("bin/local".to_string()));
        assert_eq!(from_tracked, Ok("bin/tracked".to_string()));
        assert_eq!(from_env, Ok("bin/env".to_string()));
        assert_eq!(empty_env, Ok(gate_sdk::host_native_bin()));
        assert_eq!(from_local, Ok("bin/local".to_string()));
        assert_eq!(empty_local, Ok(gate_sdk::host_native_bin()));
        assert_eq!(from_named, Ok("bin/named".to_string()));
        assert_eq!(from_default, Ok(gate_sdk::host_native_bin()));
    }

    // spec: gate-sdk/SPEC.md §git-hook — a line setting the knob that is no scalar, the knob given
    // twice, and a named knob file or overlay that is absent each leave no binary to start, so each refuses
    // naming its file
    #[test]
    fn a_line_setting_the_knob_that_is_no_scalar_refuses_with_its_file_and_line() {
        let d = scratch("refuse");
        let dir = d.display().to_string();
        let tracked = d.join("gate-sdk-config.knobs");
        let mut got: Vec<(String, String)> = Vec::new();
        for bad in [
            "GATE_SDK_NATIVE_BIN[] = a",
            "GATE_SDK_NATIVE_BIN[k] = a",
            "GATE_SDK_NATIVE_BIN",
            "GATE_SDK_NATIVE_BIN = a\tb",
            "GATE_SDK_NATIVE_BIN = a\n# c\nGATE_SDK_NATIVE_BIN = b",
        ] {
            std::fs::write(&tracked, format!("# lead\n{}\n", bad)).expect("write");
            got.push((bad.to_string(), bin_from(None, &dir, None, None).expect_err(bad)));
        }
        let absent = d.join("absent.knobs").display().to_string();
        let missing = bin_from(None, &dir, Some(&absent), None).expect_err("a named file that is absent");
        let no_overlay = bin_from(None, &dir, None, Some(&absent)).expect_err("a named overlay that is absent");
        assert!(no_overlay.contains(&absent) && no_overlay.contains("GATE_SDK_LOCAL_KNOB_FILE"), "{}", no_overlay);
        let _ = std::fs::remove_dir_all(&d);
        let at = format!("{}/gate-sdk-config.knobs:", dir);
        for (bad, e) in &got {
            let lno = if bad.contains('#') { 4 } else { 2 };
            assert!(e.starts_with(&format!("{}{}: ", at, lno)), "{:?} gave {}", bad, e);
        }
        assert!(missing.contains(&absent) && missing.contains("GATE_SDK_KNOB_FILE"), "{}", missing);
    }

    // spec: gate-sdk/SPEC.md §git-hook — a well-formed record reads, one with no `dir=` line or that
    // cannot be read refuses naming its path, and one naming its own directory is no record
    #[test]
    fn the_chain_record_reads_refuses_or_is_none() {
        let d = scratch("chain");
        let (hooks, theirs) = (d.join("gate-hooks"), d.join("husky"));
        std::fs::create_dir_all(&hooks).expect("hooks");
        std::fs::create_dir_all(&theirs).expect("theirs");
        let absent = read_chain(&hooks);
        let record = hooks.join(CHAIN);
        std::fs::write(&record, format!("dir={}\r\nrestore=.husky\r\n", theirs.display())).expect("write");
        let formed = read_chain(&hooks);
        std::fs::write(&record, "restore=.husky\n").expect("write");
        let no_dir = read_chain(&hooks);
        std::fs::write(&record, format!("dir={}\n", hooks.display())).expect("write");
        let own = read_chain(&hooks);
        std::fs::remove_file(&record).expect("remove");
        std::fs::create_dir(&record).expect("a record that cannot be read");
        let unreadable = read_chain(&hooks);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(absent, Ok(None));
        assert_eq!(formed, Ok(Some(Chain { dir: theirs, restore: ".husky".to_string() })));
        let at = format!("cannot read the chain record {}", record.display());
        assert!(no_dir.as_ref().is_err_and(|e| e.starts_with(&at) && e.contains("no dir= line")), "{:?}", no_dir);
        assert_eq!(own, Ok(None));
        assert!(unreadable.as_ref().is_err_and(|e| e.starts_with(&at)), "{:?}", unreadable);
    }

    // spec: gate-sdk/SPEC.md §git-hook — a git hook's name the gates do not serve is a forwarding
    // hook only with the record beside it; a served name and a name that is no hook never are
    #[test]
    fn a_forwarding_hook_is_a_non_served_hook_name_beside_the_record() {
        let d = scratch("forward");
        let at = |name: &str| d.join(file_name(name)).display().to_string();
        let without = forward(&at("post-merge"));
        std::fs::write(d.join(CHAIN), "dir=/elsewhere\n").expect("record");
        let beside = forward(&at("post-merge"));
        let served_name = forward(&at("pre-commit"));
        let no_hook = forward(&at("checkwright-gates"));
        let bare = forward("post-merge");
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(without, None);
        assert_eq!(beside, Some("post-merge"));
        assert_eq!(served_name, None);
        assert_eq!(no_hook, None);
        assert_eq!(bare, None);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — the restore line names the recorded value, else unsets
    #[test]
    fn the_restore_line_names_the_recorded_value_or_unsets() {
        let with = Chain { dir: PathBuf::from("/r"), restore: ".husky".to_string() };
        let without = Chain { dir: PathBuf::from("/r"), restore: String::new() };
        assert_eq!(restore_line(Some(&with)), "git config core.hooksPath '.husky'");
        assert_eq!(restore_line(Some(&without)), "git config --unset core.hooksPath");
        assert_eq!(restore_line(None), "git config --unset core.hooksPath");
        for shell in [Shell::Posix, Shell::PowerShell] {
            assert_eq!(restore_line_in(Some(&with), shell), "git config core.hooksPath '.husky'");
            assert_eq!(restore_line_in(None, shell), "git config --unset core.hooksPath");
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a space, a metacharacter and a quote stay inside one
    // word in either shell form
    #[test]
    fn the_restore_value_is_one_single_quoted_word_in_either_shell_form() {
        let value = "/opt/team hooks;$(rm -rf x)`y`\"it's\"\u{2019}";
        assert_eq!(shell_quoted(value, Shell::Posix), "'/opt/team hooks;$(rm -rf x)`y`\"it'\\''s\"\u{2019}'");
        assert_eq!(shell_quoted(value, Shell::PowerShell), "'/opt/team hooks;$(rm -rf x)`y`\"it''s\"\u{2019}\u{2019}'");
        assert_eq!(shell_quoted("", Shell::Posix), "''");
        assert_eq!(shell_quoted("C:\\hooks", Shell::PowerShell), "'C:\\hooks'");
        assert_eq!(Shell::host() == Shell::PowerShell, cfg!(windows));
    }

    #[test]
    fn a_hooks_directory_missing_a_served_file_names_it() {
        let d = std::env::temp_dir().join(format!("checkwright-hook-launcher.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        std::fs::write(d.join(file_name("pre-commit")), "").expect("write");
        let gone = missing(&d);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(gone, vec![file_name("commit-msg")]);
    }
}
