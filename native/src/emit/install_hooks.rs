// spec: gate-sdk/SPEC.md §install-hooks — the per-clone hook opt-in: place both hooks in the
// clone's git directory, point core.hooksPath at them, verify the git identity once, and report
// what was placed.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because the contract is three-state and
// every code is load-bearing: `check-identity`'s 1 propagates through, and an emitting arm cannot
// carry it.
use super::hook_launcher;
use crate::{proc, programs};
use crate::registry;
use crate::walk;
use std::path::{Path, PathBuf};

// spec: gate-sdk/SPEC.md §install-hooks — this arm's own names, then those its registry-resolved
// callee declares
pub const KNOBS: &[&str] = &[
    "GATE_SDK_GATES_DIR",
    "GATE_SDK_KIT_DIRS",
    "GATE_SDK_NATIVE_BIN",
    "GATE_SDK_POWERSHELL",
    "GATE_SDK_IDENTITY_FILE",
    "GATE_SDK_GIT_EMAIL_FILE",
    "GATE_SDK_GIT_REMOTES_FILE",
    "GATE_SDK_GH_HOSTS_FILE",
    "GATE_SDK_GH_HOST",
];

const IDENTITY: &str = "check-identity";

const REFRESH: &str = "--refresh";

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the member takes one option and no positional,
// so any other token is a refusal before either knob resolves; usage itself lives on this arm's own
// front-end `case` arm, which the class gives every member holding one.
const USAGE: &str = "usage: --install-hooks [--refresh]
  --refresh re-places the hooks of a clone already opted in and changes nothing else.
  Takes no other argument: the rest of the input is the GATE_SDK_* configuration.";

pub fn run(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("install-hooks: {}", e);
            2
        }
    }
}

fn absent(bin: &str) -> String {
    format!("no gate binary at {} — place or build the binary GATE_SDK_NATIVE_BIN names first", bin)
}

// spec: gate-sdk/SPEC.md §install-hooks — the hooks directory `--refresh` re-places into: `None`
// off the work-tree top, where the binary's path and the directory would resolve against another
// tree, and in a clone not opted in, which a refresh never opts in
fn refresh_target(here: &str) -> Result<Option<PathBuf>, String> {
    let Some(top) = walk::toplevel_in_opt(here)? else {
        return Ok(None);
    };
    if !hook_launcher::same_dir(Path::new(&top), Path::new(here)) {
        return Ok(None);
    }
    Ok(hook_launcher::hooks_dir(Path::new(here)).filter(|dir| hook_launcher::opted_in(Path::new(here), dir)))
}

// spec: gate-sdk/SPEC.md §install-hooks — the files re-placed and nothing else: no config write, no
// identity rung, no receipt
fn refresh() -> Result<i32, String> {
    let Some(hooks_dir) = refresh_target(&walk::cwd()?)? else {
        return Ok(0);
    };
    let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN")?;
    if !Path::new(&bin).is_file() {
        return Err(absent(&bin));
    }
    refresh_place(Path::new(&bin), &hooks_dir)?;
    println!("install-hooks: refreshed {}", hooks_dir.display());
    Ok(0)
}

fn refresh_place(bin: &Path, hooks_dir: &Path) -> Result<(), String> {
    let chain = hook_launcher::read_chain(hooks_dir)?;
    place(bin, hooks_dir, &recorded_forwards(chain.as_ref())?)
}

// spec: gate-sdk/SPEC.md §install-hooks — a re-run and `--refresh` place a forwarding hook for each
// live hook the recorded directory holds now, so one added there since is reached
fn recorded_forwards(chain: Option<&hook_launcher::Chain>) -> Result<Vec<String>, String> {
    match chain {
        Some(c) => Ok(forwarded(&live_hooks(&c.dir)?)),
        None => Ok(Vec::new()),
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — the hooks a forwarding hook serves: each live hook the
// gates do not serve, its suffix removed, once
fn forwarded(live: &[String]) -> Vec<String> {
    let mut out: Vec<String> = live
        .iter()
        .map(|n| hook_launcher::stem(n).to_string())
        .filter(|s| !hook_launcher::SERVED.contains(&s.as_str()))
        .collect();
    out.dedup();
    out
}

// spec: gate-sdk/SPEC.md §install-hooks — what a bare run writes beside the placed hooks
struct Plan {
    record: Option<String>,
    forwards: Vec<String>,
    chained: Option<(PathBuf, Vec<String>)>,
    replaced: Option<String>,
    restore: Option<hook_launcher::Chain>,
}

fn dispatch(args: &[String]) -> Result<i32, String> {
    if args.first().map(String::as_str) == Some(REFRESH) {
        return match args.get(1) {
            Some(a) => Err(format!("{} takes no arguments (got: {})\n{}", REFRESH, a, USAGE)),
            None => refresh(),
        };
    }
    let surplus = super::file_survey::positionals(args, "argument")
        .map_err(|e| format!("{}\n{}", e, USAGE))?;
    if let Some(a) = surplus.first() {
        return Err(format!("takes no arguments (got: {})\n{}", a, USAGE));
    }

    // spec: gate-sdk/SPEC.md §install-hooks — both knobs and the hooks directory are resolved
    // before any write, so a configuration failure refuses without half-installing.
    let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN")?;
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    if !Path::new(&bin).is_file() {
        return Err(absent(&bin));
    }
    let here = walk::cwd()?;
    let hooks_dir = hook_launcher::hooks_dir(Path::new(&here))
        .ok_or_else(|| "not inside a git repository — there is no clone to place the hooks in".to_string())?;
    let plan = plan(&here, &hooks_dir, &bin)?;

    wire(Path::new(&bin), &hooks_dir, &plan)?;
    if let Some((dir, names)) = &plan.chained {
        println!("{}", chain_line(dir, names, &hooks_dir));
    }
    if let Some(value) = plan.replaced {
        println!(
            "install-hooks: core.hooksPath was {}, which holds no hook git runs; this clone now runs {}",
            value,
            hooks_dir.display()
        );
    }
    // spec: gate-sdk/SPEC.md §install-hooks — an unwired clone is no opt-in: the refusal comes
    // ahead of the receipt, which would read as one
    config("core.hooksPath", &hooks_dir.display().to_string()).map_err(|e| {
        format!(
            "{} — the hooks are placed in {} and git is not pointed at them, so no hook runs; clear the cause and re-run",
            e,
            hooks_dir.display()
        )
    })?;
    // spec: gate-sdk/SPEC.md §install-hooks — the blame guard stays a file-existence test, so a
    // consumer without the file gets the same one-line output it always got; its failed write is
    // reported and the status unmoved, since no hook depends on it.
    if Path::new(".git-blame-ignore-revs").is_file() {
        if let Err(e) = config("blame.ignoreRevsFile", ".git-blame-ignore-revs") {
            eprintln!("install-hooks: {}", e);
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — the receipt prints whatever the rung returned: a
    // failed verification still tells the session which hooks the wiring enabled, and the status
    // is what carries the finding.
    let identity_rc = identity_rung(&gates_dir);

    println!("Active hooks:");
    for (name, _) in walk::list_dir(&hooks_dir).unwrap_or_default() {
        println!("  {}", name);
    }
    println!();
    println!("Disable with:  {}", hook_launcher::restore_line(plan.restore.as_ref()));
    Ok(identity_rc)
}

// spec: gate-sdk/SPEC.md §install-hooks — a re-run keeps the record and re-places its forwarding
// hooks; a first run records the prior directory, and over a live hook on native Windows it first
// probes that this git can chain it
fn plan(here: &str, hooks_dir: &Path, bin: &str) -> Result<Plan, String> {
    let top = walk::toplevel_in_opt(here)?.unwrap_or_else(|| here.to_string());
    let (dir, chained, replaced) = match prior(here, hooks_dir)? {
        Prior::Own => {
            let chain = hook_launcher::read_chain(hooks_dir)?;
            let forwards = recorded_forwards(chain.as_ref())?;
            return Ok(Plan { record: None, forwards, chained: None, replaced: None, restore: chain });
        }
        Prior::Quiet { dir, set } => (dir, None, set),
        Prior::Live { dir, names } => {
            if !chain_probe() {
                return Err(live_refusal(&dir, &names, bin));
            }
            (dir.clone(), Some((dir, names)), None)
        }
    };
    let restore = local_hooks_path(&top)?;
    if hook_launcher::Shell::host() == hook_launcher::Shell::PowerShell {
        if let Some(refusal) = powershell_altered(&restore) {
            return Err(refusal);
        }
    }
    let record = record_text(&dir, &restore)?;
    let forwards = chained.as_ref().map(|(_, names)| forwarded(names)).unwrap_or_default();
    Ok(Plan {
        record: Some(record),
        forwards,
        chained,
        replaced,
        restore: Some(hook_launcher::Chain { dir, restore }),
    })
}

// spec: gate-sdk/SPEC.md §install-hooks — the record lands after the hooks and before git is
// pointed at them, so a failed write leaves no clone running hooks without it
fn wire(bin: &Path, hooks_dir: &Path, plan: &Plan) -> Result<(), String> {
    place(bin, hooks_dir, &plan.forwards)?;
    let Some(text) = &plan.record else {
        return Ok(());
    };
    write_record(hooks_dir, text, &run_tag()).map_err(|e| {
        format!(
            "{} — the hooks are placed in {} and git is not pointed at them, so no hook runs; clear the cause and re-run",
            e,
            hooks_dir.display()
        )
    })
}

// spec: gate-sdk/SPEC.md §install-hooks — `core.hooksPath`'s local-scope value before the run, as
// git prints it, empty where that scope set none
fn local_hooks_path(top: &str) -> Result<String, String> {
    let got = proc::run(&programs::GIT, &["-C", top, "config", "--local", "--get", "core.hooksPath"])?;
    match (got.stdout(), got.code()) {
        (Some(out), _) => {
            let value = git_value(out);
            String::from_utf8(value.to_vec()).map_err(|_| {
                not_utf8(&format!("core.hooksPath is set locally to '{}'", escaped_bytes(value)), LOCAL_UNSET)
            })
        }
        (None, Some(1)) => Ok(String::new()),
        (None, _) => Err(format!("cannot read core.hooksPath ({})", got.failure_report().unwrap_or_default())),
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — the record holds one value per line, so a value carrying a
// line break is refused before anything is placed
fn record_text(dir: &Path, restore: &str) -> Result<String, String> {
    let shown = dir.display().to_string();
    if let Some(v) = [shown.as_str(), restore].into_iter().find(|v| v.contains(['\n', '\r'])) {
        return Err(format!(
            "core.hooksPath names {:?}, which carries a line break the chain record cannot hold — nothing placed",
            v
        ));
    }
    Ok(format!(
        "{}{}\n{}{}\n",
        hook_launcher::CHAIN_DIR_KEY,
        shown,
        hook_launcher::CHAIN_RESTORE_KEY,
        restore
    ))
}

// spec: gate-sdk/SPEC.md §install-hooks — on native Windows, the local values Windows PowerShell
// 5.1 hands git altered from the restore line, measured on the Windows leg: a double quote, and
// whitespace with a trailing backslash
fn powershell_altered(value: &str) -> Option<String> {
    let help = if value.contains('"') {
        "it carries a double quote, which Windows PowerShell 5.1 drops from the restore line's argument — nothing placed\n  help: respell it: Windows admits no double quote in a file name, so this value names no directory here; set core.hooksPath to the directory you mean, then re-run --install-hooks."
    } else if value.ends_with('\\') && value.contains(char::is_whitespace) {
        "it holds whitespace and ends in a backslash, which Windows PowerShell 5.1 turns into a double quote in the restore line's argument — nothing placed\n  help: respell it without the trailing backslash, which names the same directory, then re-run --install-hooks."
    } else {
        return None;
    };
    Some(format!("core.hooksPath is set locally to '{}', which the restore line cannot carry: {}", value, help))
}

// spec: gate-sdk/SPEC.md §install-hooks — the record is placed as a hook is: written under a sibling
// name and renamed over it, the sibling removed on every path
fn write_record(hooks_dir: &Path, text: &str, tag: &str) -> Result<(), String> {
    use std::io::Write;
    let dst = hooks_dir.join(hook_launcher::CHAIN);
    let staged = hooks_dir.join(format!("{}{}.{}", hook_launcher::CHAIN, STAGED_SUFFIX, tag));
    let written = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staged)
        .and_then(|mut f| f.write_all(text.as_bytes()))
        .map_err(|e| format!("cannot write the chain record {}: {}", staged.display(), e));
    let landed = written.and_then(|()| {
        std::fs::rename(&staged, &dst).map_err(|e| format!("cannot write the chain record {}: {}", dst.display(), e))
    });
    match (landed, remove_own(&staged)) {
        (Ok(()), None) => Ok(()),
        (Ok(()), Some(left)) => {
            eprintln!("install-hooks: {}", left);
            Ok(())
        }
        (Err(e), None) => Err(e),
        (Err(e), Some(left)) => Err(format!("{}; {}", e, left)),
    }
}

fn chain_line(dir: &Path, names: &[String], hooks_dir: &Path) -> String {
    format!(
        "install-hooks: this clone ran hooks from {} ({}); each now runs from {}, ahead of the gates where they share a hook",
        dir.display(),
        names.join(", "),
        hooks_dir.display()
    )
}

// spec: gate-sdk/SPEC.md §install-hooks — on native Windows a chained hook starts through `git hook
// run --to-stdin`, so a bare run over a live hook first asks this git to run one
#[cfg(windows)]
fn chain_probe() -> bool {
    let d = proc::temp_base().join(format!("checkwright-chain-probe.{}", run_tag()));
    let (empty, input) = (d.join("hooks"), d.join("stdin"));
    let ready = std::fs::create_dir_all(&empty).and_then(|()| std::fs::write(&input, ""));
    let ran = ready.is_ok()
        && proc::run(
            &programs::GIT,
            &[
                "-c",
                &format!("core.hooksPath={}", empty.display()),
                "hook",
                "run",
                "--ignore-missing",
                &format!("--to-stdin={}", input.display()),
                "pre-commit",
            ],
        )
        .is_ok_and(|c| c.code() == Some(0));
    let _ = std::fs::remove_dir_all(&d);
    ran
}

#[cfg(not(windows))]
fn chain_probe() -> bool {
    true
}

// spec: gate-sdk/SPEC.md §install-hooks — an empty value is set and names no hook, which the record's
// empty `restore=` cannot tell from no value, so it is refused rather than chained or recorded
const EMPTY_REFUSAL: &str = "core.hooksPath is set to the empty string, under which git finds no hook — nothing placed\n  help: unset it with 'git config --unset core.hooksPath' at the scope that set it ('git config --show-origin --get core.hooksPath' names it), or set it to the directory you meant, then re-run --install-hooks.";

// spec: gate-sdk/SPEC.md §install-hooks — git's value with the one line feed it appends stripped, so
// a line break the value keeps reaches the record's refusal
fn git_value(out: &[u8]) -> &[u8] {
    out.strip_suffix(b"\n").unwrap_or(out)
}

const EFFECTIVE_UNSET: &str = "unset it at the scope that set it ('git config --show-origin --get core.hooksPath' names it)";

const LOCAL_UNSET: &str = "unset it with 'git config --local --unset core.hooksPath'";

// spec: gate-sdk/SPEC.md §install-hooks — a value, or the prior directory it resolves to, that is not
// UTF-8 is refused: the record and the restore line carry UTF-8 alone
fn not_utf8(subject: &str, unset: &str) -> String {
    format!(
        "{}, which is not UTF-8, and the chain record and its restore line carry UTF-8 alone — nothing placed\n  help: rename the directory, or set core.hooksPath to a directory whose path is UTF-8, or {}, then re-run --install-hooks.",
        subject, unset
    )
}

fn escaped_bytes(mut b: &[u8]) -> String {
    let mut out = String::new();
    loop {
        match std::str::from_utf8(b) {
            Ok(s) => return out + s,
            Err(e) => {
                let (good, rest) = b.split_at(e.valid_up_to());
                out.push_str(std::str::from_utf8(good).unwrap_or_default());
                let bad = e.error_len().unwrap_or(rest.len());
                for byte in &rest[..bad] {
                    out.push_str(&format!("\\x{:02x}", byte));
                }
                b = &rest[bad..];
            }
        }
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — a path's own units, escaped for the message alone: its bytes
// on POSIX, each unpaired surrogate of its UTF-16 on Windows
#[cfg(not(windows))]
fn escaped_path(p: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    escaped_bytes(p.as_os_str().as_bytes())
}

#[cfg(windows)]
fn escaped_path(p: &Path) -> String {
    use std::os::windows::ffi::OsStrExt;
    char::decode_utf16(p.as_os_str().encode_wide())
        .map(|c| match c {
            Ok(c) => c.to_string(),
            Err(e) => format!("\\u{{{:04x}}}", e.unpaired_surrogate()),
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §install-hooks — the hooks directory git runs before this arm repoints it
// spec: installer/SPEC.md §init — also the directory a failed install commit's hook is named in
#[derive(Debug, PartialEq)]
pub(crate) enum Prior {
    Own,
    Quiet { dir: PathBuf, set: Option<String> },
    Live { dir: PathBuf, names: Vec<String> },
}

pub(crate) fn prior(here: &str, hooks_dir: &Path) -> Result<Prior, String> {
    let top = walk::toplevel_in_opt(here)?.unwrap_or_else(|| here.to_string());
    let got = proc::run(&programs::GIT, &["-C", &top, "config", "--path", "--get", "core.hooksPath"])?;
    let set = match (got.stdout(), got.code()) {
        (Some(out), _) => {
            let value = git_value(out);
            let value = String::from_utf8(value.to_vec()).map_err(|_| {
                not_utf8(&format!("core.hooksPath is set to '{}'", escaped_bytes(value)), EFFECTIVE_UNSET)
            })?;
            if value.is_empty() {
                return Err(EMPTY_REFUSAL.to_string());
            }
            Some(value)
        }
        (None, Some(1)) => None,
        (None, _) => return Err(format!("cannot read core.hooksPath ({})", got.failure_report().unwrap_or_default())),
    };
    let dir = match &set {
        Some(value) => PathBuf::from(walk::physical_against_strict(&top, value).map_err(|dir| {
            not_utf8(&format!("core.hooksPath '{}' resolves to '{}'", value, escaped_path(&dir)), EFFECTIVE_UNSET)
        })?),
        None => {
            let out = proc::run(&programs::GIT, &["-C", &top, "rev-parse", "--git-path", "hooks"])?;
            let path = out.stdout().ok_or_else(|| {
                format!("cannot resolve git's hooks directory ({})", out.failure_report().unwrap_or_default())
            })?;
            PathBuf::from(walk::physical_against(&top, String::from_utf8_lossy(path).trim()))
        }
    };
    if hook_launcher::same_dir(&dir, hooks_dir) {
        return Ok(Prior::Own);
    }
    let names = live_hooks(&dir)?;
    if names.is_empty() {
        Ok(Prior::Quiet { dir, set })
    } else {
        Ok(Prior::Live { dir, names })
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — a live hook is a file git would run: a hook's name, a
// symlink followed, and the executable bit where the host has one
pub fn live_hooks(dir: &Path) -> Result<Vec<String>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = walk::list_dir(dir)?
        .into_iter()
        .map(|(name, _)| name)
        .filter(|name| hook_launcher::git_hook_named(name) && proc::is_executable(&dir.join(name)))
        .collect();
    names.sort();
    Ok(names)
}

// spec: gate-sdk/SPEC.md §install-hooks — the native Windows capability refusal: the directory, the
// hooks and the cause, then the routes
fn live_refusal(dir: &Path, names: &[String], bin: &str) -> String {
    format!(
        "this clone runs hooks from {} ({}), which this git cannot chain ('git hook run --to-stdin' is not available) — nothing placed\n  help: upgrade git, or call the gates from your hook manager — '{} --git-hook pre-commit' from its pre-commit and '{} --git-hook commit-msg <message-file>' from its commit-msg — or move those hooks aside or unset core.hooksPath, then re-run --install-hooks.",
        dir.display(),
        names.join(", "),
        bin,
        bin
    )
}

// spec: gate-sdk/SPEC.md §install-hooks — each served and forwarding hook is a hard link to the
// binary under the hook's name, else a copy, written under a sibling name and renamed over the
// hook, so a placement that fails leaves the hook already there in place
fn place(bin: &Path, dir: &Path, forwards: &[String]) -> Result<(), String> {
    place_as(bin, dir, forwards, &run_tag())
}

fn place_as(bin: &Path, dir: &Path, forwards: &[String], tag: &str) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
    for hook in hook_launcher::SERVED.iter().copied().chain(forwards.iter().map(String::as_str)) {
        let dst = dir.join(hook_launcher::file_name(hook));
        let staged = dir.join(staged_name(hook, tag));
        stage(bin, &staged)?;
        let renamed =
            std::fs::rename(&staged, &dst).map_err(|e| format!("cannot replace {}: {}", dst.display(), e));
        // spec: gate-sdk/SPEC.md §install-hooks — a rename between two links to one file succeeds
        // and moves nothing, so the sibling is removed on every path; a removal that fails is
        // reported, and moves the status only where the rename already had
        match (renamed, remove_own(&staged)) {
            (Ok(()), None) => {}
            (Ok(()), Some(left)) => eprintln!("install-hooks: {}", left),
            (Err(e), None) => return Err(e),
            (Err(e), Some(left)) => return Err(format!("{}; {}", e, left)),
        }
    }
    Ok(())
}

const STAGED_SUFFIX: &str = ".new";

// spec: gate-sdk/SPEC.md §install-hooks — the sibling's name carries the placing process's id and
// a value drawn at random per run, so a placement in another process-id namespace draws another
fn run_tag() -> String {
    use std::hash::{BuildHasher, Hasher};
    let drawn = std::collections::hash_map::RandomState::new().build_hasher().finish();
    format!("{}.{:016x}", std::process::id(), drawn)
}

fn staged_name(hook: &str, tag: &str) -> String {
    format!("{}{}.{}", hook_launcher::file_name(hook), STAGED_SUFFIX, tag)
}

// spec: gate-sdk/SPEC.md §install-hooks — the sibling is created, never opened or removed first: a
// name already there is another placement's, and the link and the copy both refuse it. A stage
// that fails leaves nothing of its own behind
fn stage(bin: &Path, staged: &Path) -> Result<(), String> {
    if std::fs::hard_link(bin, staged).is_err() {
        copy_new(bin, staged)
            .map_err(|e| format!("cannot place {} from {}: {}", staged.display(), bin.display(), e))?;
    }
    crate::install::make_executable(staged).map_err(|e| match remove_own(staged) {
        Some(left) => format!("{}; {}", e, left),
        None => e,
    })
}

fn remove_own(staged: &Path) -> Option<String> {
    match std::fs::remove_file(staged) {
        Ok(()) => None,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => Some(format!(
            "cannot remove {}: {} — it serves no hook; delete it by hand",
            staged.display(),
            e
        )),
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — the copy creates its file and never opens a name already
// there: a sibling there may be a link to the file the placed hooks serve, and a write through it
// would replace or truncate them. A copy cut short removes its file or names it in the refusal
fn copy_new(bin: &Path, staged: &Path) -> std::io::Result<()> {
    copy_from(&mut std::fs::File::open(bin)?, staged)
}

fn copy_from(src: &mut impl std::io::Read, staged: &Path) -> std::io::Result<()> {
    let mut dst = std::fs::OpenOptions::new().write(true).create_new(true).open(staged)?;
    let copied = std::io::copy(src, &mut dst).map(|_| ());
    drop(dst);
    copied.map_err(|e| match remove_own(staged) {
        Some(left) => std::io::Error::new(e.kind(), format!("{}; {}", e, left)),
        None => e,
    })
}

fn config(key: &str, value: &str) -> Result<(), String> {
    let c = proc::run(&programs::GIT, &["config", key, value])?;
    if c.stdout().is_none() {
        return Err(format!("could not set {} ({})", key, c.failure_report().unwrap_or_default()));
    }
    println!("Installed: {} = {}", key, value);
    Ok(())
}

// spec: gate-sdk/SPEC.md §install-hooks — the apply-and-verify rung. The gate is resolved through
// the registry so a consumer shadow wins; an in-process call by name would resolve the *crate's*
// member and silently stop honouring it, which narrows a consumer-facing seam.
fn identity_rung(gates_dir: &str) -> i32 {
    let roots = match walk::kit_roots_abs() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("install-hooks: {}", e);
            return 2;
        }
    };
    let dirs = registry::resolve_dirs(gates_dir, &roots);
    // spec: gate-sdk/SPEC.md §install-hooks — resolving nowhere is a silent skip at the wiring's
    // own status, which a consumer shipping no such gate is entitled to; anything else that
    // cannot be dispatched fails the opt-in rather than being waved through.
    let Some(src) = registry::resolve(IDENTITY, &dirs) else {
        return 0;
    };
    println!();
    println!("Verifying git identity ({})…", IDENTITY);
    // spec: gate-sdk/SPEC.md §run-gates — a descriptor re-execs this binary as a child rather than
    // calling the member in process, on that section's fault-isolation ground
    // spec: gate-sdk/SPEC.md §install-hooks — a consumer shadow is that consumer's rule and this arm
    // is not entitled to substitute its own, so it is started as the battery starts it, with its two
    // streams in the caller's terminal and its status propagated.
    let exe = match crate::proc::self_image() {
        Ok(p) => p.display().to_string(),
        Err(e) => {
            eprintln!("install-hooks: cannot resolve this binary's own path: {}", e);
            return 2;
        }
    };
    let inv = match registry::invocation(IDENTITY, &src, &exe) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("install-hooks: {} at {} — {}", IDENTITY, src, e);
            return 2;
        }
    };
    let args: Vec<&str> = inv.args.iter().map(String::as_str).collect();
    let env: Vec<(String, String)> = match inv.substrate {
        registry::Substrate::Compiled => Vec::new(),
        _ => vec![("GATE_SDK_NATIVE_BIN".to_string(), exe.clone())],
    };
    match proc::run_to_env(&inv.program, &args, &env, &proc::Sink::Inherit) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("install-hooks: {}", e);
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the option half refuses before either knob
    // resolves, so a stray flag wires nothing
    #[test]
    fn a_flag_refuses_with_the_usage_before_any_wiring() {
        let err = dispatch(&["--help".to_string()]).expect_err("--help must refuse");
        assert!(err.contains("--help") && err.contains(USAGE), "{}", err);
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — a surplus positional, bare or after `--`,
    // refuses before any wiring rather than reading as a successful opt-in
    #[test]
    fn a_bare_positional_refuses_with_the_usage_before_any_wiring() {
        for argv in [vec!["foo"], vec!["--", "foo"]] {
            let args: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
            let err = dispatch(&args).expect_err("a positional must refuse");
            assert!(err.contains("got: foo") && err.contains(USAGE), "{}", err);
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — `--refresh` is the arm's one option and takes no
    // operand, so a token after it refuses before anything is placed
    #[test]
    fn refresh_refuses_a_trailing_token_with_the_usage() {
        for extra in ["foo", "--force"] {
            let err = dispatch(&[REFRESH.to_string(), extra.to_string()]).expect_err("an operand must refuse");
            assert!(err.contains(&format!("got: {}", extra)) && err.contains(USAGE), "{}", err);
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a refresh has a target only at the work-tree top of a
    // clone whose `core.hooksPath` names the hooks directory, so its caller never opts a clone in
    #[test]
    fn refresh_targets_only_the_top_of_a_clone_already_opted_in() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-refresh.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("repo/sub")).expect("scratch");
        let top = walk::normalize_abs(&d.join("repo").display().to_string());
        let sub = format!("{}/sub", top);
        proc::run(&programs::GIT, &["-C", &top, "init", "-q"]).expect("git init");
        let unopted = refresh_target(&top);
        let hooks = hook_launcher::hooks_dir(Path::new(&top)).expect("a hooks directory");
        let elsewhere = d.join("elsewhere").display().to_string();
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &elsewhere]).expect("git config");
        let other_path = refresh_target(&top);
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &hooks.display().to_string()]).expect("git config");
        let opted = refresh_target(&top);
        let below = refresh_target(&sub);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(unopted, Ok(None));
        assert_eq!(other_path, Ok(None));
        assert_eq!(opted, Ok(Some(hooks)));
        assert_eq!(below, Ok(None));
    }

    fn scratch_repo(tag: &str) -> (PathBuf, String, PathBuf) {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-prior-{}.{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("repo")).expect("scratch");
        let top = walk::normalize_abs(&d.join("repo").display().to_string());
        proc::run(&programs::GIT, &["-C", &top, "init", "-q"]).expect("git init");
        let hooks = hook_launcher::hooks_dir(Path::new(&top)).expect("a hooks directory");
        (d, top, hooks)
    }

    fn plant(dir: &Path, name: &str, executable: bool) {
        std::fs::create_dir_all(dir).expect("hooks dir");
        let file = dir.join(name);
        std::fs::write(&file, "#!/bin/sh\nexit 0\n").expect("hook");
        if executable {
            crate::install::make_executable(&file).expect("chmod");
        }
    }

    fn listed(dir: &Path) -> Vec<String> {
        walk::list_dir(dir).expect("lists").into_iter().map(|(n, _)| n).collect()
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a bare run's placement, record and config write
    fn bare_run(top: &str, hooks: &Path, bin: &Path) -> Result<Plan, String> {
        let plan = plan(top, hooks, &bin.display().to_string())?;
        wire(bin, hooks, &plan)?;
        proc::run(&programs::GIT, &["-C", top, "config", "core.hooksPath", &hooks.display().to_string()])?;
        Ok(plan)
    }

    fn binary(d: &Path) -> PathBuf {
        let bin = d.join("bin");
        std::fs::write(&bin, "gates").expect("binary");
        bin
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a live pre-commit in git's default directory is
    // chained: the record names that directory with an empty restore and no forwarding hook is
    // placed, the gates serving pre-commit; a directory of samples alone records and forwards nothing
    #[test]
    fn a_live_hook_in_the_default_directory_is_chained_and_samples_forward_nothing() {
        let (d, top, hooks) = scratch_repo("default");
        let default = PathBuf::from(format!("{}/.git/hooks", top));
        let samples = prior(&top, &hooks);
        let (sd, stop, shooks) = scratch_repo("samples");
        let quiet = bare_run(&stop, &shooks, &binary(&sd)).map(|p| (p.forwards, p.chained));
        let quiet_record = hook_launcher::read_chain(&shooks);
        let quiet_files = listed(&shooks);
        plant(&default, "pre-commit", true);
        let live = prior(&top, &hooks);
        let ran = bare_run(&top, &hooks, &binary(&d)).map(|p| (p.forwards, p.chained.map(|(_, n)| n)));
        let record = hook_launcher::read_chain(&hooks);
        let files = listed(&hooks);
        let default_named = record.as_ref().is_ok_and(|r| r.as_ref().is_some_and(|c| hook_launcher::same_dir(&c.dir, &default)));
        let samples_named = matches!(&quiet_record, Ok(Some(c)) if hook_launcher::same_dir(&c.dir, &PathBuf::from(format!("{}/.git/hooks", stop))));
        let _ = std::fs::remove_dir_all(&d);
        let _ = std::fs::remove_dir_all(&sd);
        assert!(matches!(&samples, Ok(Prior::Quiet { set: None, .. })), "git's template hooks are samples: {:?}", samples);
        assert!(matches!(&live, Ok(Prior::Live { names, .. }) if names == &vec!["pre-commit".to_string()]), "{:?}", live);
        assert_eq!(ran, Ok((Vec::new(), Some(vec!["pre-commit".to_string()]))));
        assert!(default_named, "{:?}", record);
        assert_eq!(record.map(|r| r.map(|c| c.restore)), Ok(Some(String::new())));
        let served_and_record = vec![hook_launcher::CHAIN.to_string(), hook_launcher::file_name("commit-msg"), hook_launcher::file_name("pre-commit")];
        assert_eq!(files, served_and_record);
        assert_eq!(quiet, Ok((Vec::new(), None)));
        assert!(samples_named, "{:?}", quiet_record);
        assert_eq!(quiet_files, served_and_record);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a local core.hooksPath naming a directory with a live
    // pre-push records that value to restore and places a forwarding pre-push; one holding no hook
    // is replaced with a note
    #[test]
    fn a_local_hooks_path_with_a_live_hook_is_recorded_and_forwarded() {
        let (d, top, hooks) = scratch_repo("set");
        let theirs = d.join("husky").display().to_string();
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &theirs]).expect("git config");
        let quiet = prior(&top, &hooks);
        plant(Path::new(&theirs), "pre-push", true);
        let ran = bare_run(&top, &hooks, &binary(&d)).map(|p| (p.forwards, p.replaced));
        let record = hook_launcher::read_chain(&hooks).map(|r| r.map(|c| c.restore));
        let forwarding = proc::is_executable(&hooks.join(hook_launcher::file_name("pre-push")));
        let _ = std::fs::remove_dir_all(&d);
        assert!(matches!(&quiet, Ok(Prior::Quiet { set: Some(v), .. }) if v == &theirs), "{:?}", quiet);
        assert_eq!(ran, Ok((vec!["pre-push".to_string()], None)));
        assert_eq!(record, Ok(Some(theirs)));
        assert!(forwarding, "no forwarding pre-push was placed");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — an effective value set to the empty string is refused
    // with nothing placed, its help naming the remedy
    #[test]
    fn an_empty_hooks_path_is_refused_with_nothing_placed() {
        let (d, top, hooks) = scratch_repo("empty");
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", ""]).expect("git config");
        let ran = bare_run(&top, &hooks, &binary(&d)).map(|p| p.record);
        let placed = hooks.exists();
        let _ = std::fs::remove_dir_all(&d);
        let said = ran.expect_err("an empty core.hooksPath was not refused");
        let lines: Vec<&str> = said.lines().collect();
        assert_eq!(lines.len(), 2, "{}", said);
        assert!(lines[0].contains("set to the empty string") && lines[0].ends_with("nothing placed"), "{}", said);
        assert!(lines[1].starts_with("  help: unset it with 'git config --unset core.hooksPath'"), "{}", said);
        assert!(!placed, "the hooks directory was created");
    }

    fn append_config(top: &str, bytes: &[u8]) {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().append(true).open(Path::new(top).join(".git/config")).expect("config");
        f.write_all(bytes).expect("appends");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a value that is not UTF-8, written into the config file
    // as bytes so the case runs on every host, is refused with nothing placed
    #[test]
    fn a_value_that_is_not_utf8_is_refused_with_nothing_placed() {
        let (d, top, hooks) = scratch_repo("not-utf8");
        append_config(&top, b"[core]\n\thooksPath = h\xffk\n");
        let ran = bare_run(&top, &hooks, &binary(&d)).map(|p| p.record);
        let placed = hooks.exists();
        let _ = std::fs::remove_dir_all(&d);
        let said = ran.expect_err("a core.hooksPath that is not UTF-8 was not refused");
        let lines: Vec<&str> = said.lines().collect();
        assert_eq!(lines.len(), 2, "{}", said);
        assert!(lines[0].starts_with("core.hooksPath is set to 'h\\xffk', which is not UTF-8"), "{}", said);
        assert!(lines[0].ends_with("nothing placed") && lines[1].contains(EFFECTIVE_UNSET), "{}", said);
        assert!(!placed, "the hooks directory was created");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a local value that is not UTF-8 under a UTF-8
    // worktree-scope value reaches the local read alone, and is refused there
    #[test]
    fn a_local_value_that_is_not_utf8_under_a_worktree_value_is_refused() {
        let (d, top, hooks) = scratch_repo("local-not-utf8");
        append_config(&top, b"[core]\n\thooksPath = h\xffk\n");
        proc::run(&programs::GIT, &["-C", &top, "config", "extensions.worktreeConfig", "true"]).expect("git config");
        let theirs = d.join("theirs").display().to_string();
        proc::run(&programs::GIT, &["-C", &top, "config", "--worktree", "core.hooksPath", &theirs]).expect("git config");
        let ran = bare_run(&top, &hooks, &binary(&d)).map(|p| p.record);
        let placed = hooks.exists();
        let _ = std::fs::remove_dir_all(&d);
        let said = ran.expect_err("a local core.hooksPath that is not UTF-8 was not refused");
        assert!(said.starts_with("core.hooksPath is set locally to 'h\\xffk', which is not UTF-8"), "{}", said);
        assert!(said.contains(LOCAL_UNSET), "{}", said);
        assert!(!placed, "the hooks directory was created");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a UTF-8 value naming a link to a directory whose name
    // is not UTF-8 is refused naming the resolved directory
    #[cfg(target_os = "linux")]
    #[test]
    fn a_value_resolving_to_a_directory_that_is_not_utf8_is_refused() {
        use std::os::unix::ffi::OsStrExt;
        let (d, top, hooks) = scratch_repo("resolves-not-utf8");
        let odd = d.join(std::ffi::OsStr::from_bytes(b"h\xffk"));
        plant(&odd, "pre-commit", true);
        let link = d.join("link");
        std::os::unix::fs::symlink(&odd, &link).expect("symlink");
        let value = link.display().to_string();
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &value]).expect("git config");
        let ran = bare_run(&top, &hooks, &binary(&d)).map(|p| p.record);
        let placed = hooks.exists();
        let _ = std::fs::remove_dir_all(&d);
        let said = ran.expect_err("a link to a directory that is not UTF-8 was not refused");
        assert!(said.starts_with(&format!("core.hooksPath '{}' resolves to '", value)), "{}", said);
        assert!(said.contains("/h\\xffk', which is not UTF-8"), "{}", said);
        assert!(!placed, "the hooks directory was created");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a value ending in a line break keeps it past the one
    // line feed git appends, so the line-break refusal sees it
    #[test]
    fn a_value_ending_in_a_line_break_is_refused() {
        let (d, top, hooks) = scratch_repo("trailing-break");
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", "hk\n"]).expect("git config");
        let ran = bare_run(&top, &hooks, &binary(&d)).map(|p| p.record);
        let placed = hooks.exists();
        let _ = std::fs::remove_dir_all(&d);
        assert!(ran.is_err_and(|e| e.contains("line break")), "a trailing line break was trimmed away");
        assert!(!placed, "the hooks directory was created");
        assert_eq!(git_value(b"hk\n\n"), b"hk\n");
        assert_eq!(git_value(b"hk\r\n"), b"hk\r");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — each byte that is not UTF-8 is escaped as `\xNN` and
    // the rest kept, a replacement character a real name holds included
    #[test]
    fn a_byte_that_is_not_utf8_is_escaped_and_the_rest_kept() {
        assert_eq!(escaped_bytes(b"h\xffk"), "h\\xffk");
        assert_eq!(escaped_bytes(b"\xe2\x82"), "\\xe2\\x82");
        assert_eq!(escaped_bytes("a\u{fffd}é".as_bytes()), "a\u{fffd}é");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a value the local scope does not itself set, here an
    // included file's, records an empty restore
    #[test]
    fn a_hooks_path_set_outside_the_local_scope_records_an_empty_restore() {
        let (d, top, hooks) = scratch_repo("global");
        let theirs = d.join("global-hooks");
        plant(&theirs, "pre-push", true);
        let included = d.join("gitconfig");
        let value = theirs.display().to_string().replace('\\', "/");
        std::fs::write(&included, format!("[core]\n\thooksPath = {}\n", value)).expect("included config");
        proc::run(&programs::GIT, &["-C", &top, "config", "include.path", &included.display().to_string()]).expect("git config");
        let ran = bare_run(&top, &hooks, &binary(&d)).map(|p| p.forwards);
        let record = hook_launcher::read_chain(&hooks);
        let named = matches!(&record, Ok(Some(c)) if hook_launcher::same_dir(&c.dir, &theirs));
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(ran, Ok(vec!["pre-push".to_string()]));
        assert!(named, "{:?}", record);
        assert_eq!(record.map(|r| r.map(|c| c.restore)), Ok(Some(String::new())));
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a re-run keeps the record byte-identical and, as a
    // refresh does, forwards a hook the recorded directory gained since the first run
    #[test]
    fn a_rerun_keeps_the_record_and_forwards_a_hook_added_since() {
        let (d, top, hooks) = scratch_repo("rechain");
        let theirs = d.join("husky");
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &theirs.display().to_string()]).expect("git config");
        plant(&theirs, "pre-push", true);
        let bin = binary(&d);
        bare_run(&top, &hooks, &bin).expect("first run");
        let first = std::fs::read(hooks.join(hook_launcher::CHAIN)).expect("the record");
        plant(&theirs, "post-merge", true);
        let rerun = bare_run(&top, &hooks, &bin).map(|p| (p.record, p.forwards));
        let kept = std::fs::read(hooks.join(hook_launcher::CHAIN)).expect("the record");
        let after_rerun = listed(&hooks);
        std::fs::remove_file(hooks.join(hook_launcher::file_name("post-merge"))).expect("remove");
        let target = refresh_target(&top);
        let refreshed = refresh_place(&bin, &hooks);
        let after_refresh = listed(&hooks);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(rerun, Ok((None, vec!["post-merge".to_string(), "pre-push".to_string()])));
        assert_eq!(kept, first, "a re-run rewrote the record");
        let every = vec![
            hook_launcher::CHAIN.to_string(),
            hook_launcher::file_name("commit-msg"),
            hook_launcher::file_name("post-merge"),
            hook_launcher::file_name("pre-commit"),
            hook_launcher::file_name("pre-push"),
        ];
        assert_eq!(after_rerun, every);
        assert_eq!(target, Ok(Some(hooks)));
        assert_eq!(refreshed, Ok(()));
        assert_eq!(after_refresh, every);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — the native Windows refusal names the directory, the
    // hooks and the cause, and its help names the routes
    #[test]
    fn the_capability_refusal_names_the_directory_the_hooks_and_the_routes() {
        let said = live_refusal(Path::new("/r/.husky"), &["pre-commit".to_string(), "pre-push".to_string()], "bin/gates");
        let lines: Vec<&str> = said.lines().collect();
        assert_eq!(lines.len(), 2, "{}", said);
        assert!(lines[0].starts_with("this clone runs hooks from /r/.husky (pre-commit, pre-push)"), "{}", said);
        assert!(lines[0].contains("'git hook run --to-stdin' is not available") && lines[0].ends_with("nothing placed"), "{}", said);
        assert!(lines[1].starts_with("  help: upgrade git") && lines[1].contains("'bin/gates --git-hook pre-commit'"), "{}", said);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — the leg's git runs `git hook run --to-stdin`
    #[cfg(windows)]
    #[test]
    fn the_chain_probe_passes_on_this_git() {
        assert!(chain_probe(), "this git cannot run 'git hook run --to-stdin'");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a value carrying a line break is refused before
    // anything is placed, the record holding one value per line
    #[test]
    fn a_value_with_a_line_break_is_refused() {
        assert!(record_text(Path::new("/r/hooks"), "a\nb").is_err_and(|e| e.contains("line break")));
        assert_eq!(record_text(Path::new("/r/hooks"), "").as_deref(), Ok("dir=/r/hooks\nrestore=\n"));
    }

    // spec: gate-sdk/SPEC.md §install-hooks — the two classes Windows PowerShell 5.1 alters are
    // refused naming the value, the class and the respelling, and the control value passes
    #[test]
    fn the_values_windows_powershell_alters_are_refused_naming_the_class() {
        let quote = powershell_altered("a\"b").expect("a double quote was not refused");
        let lines: Vec<&str> = quote.lines().collect();
        assert_eq!(lines.len(), 2, "{}", quote);
        assert!(lines[0].starts_with("core.hooksPath is set locally to 'a\"b'") && lines[0].contains("carries a double quote"), "{}", quote);
        assert!(lines[0].ends_with("nothing placed") && lines[1].contains("admits no double quote in a file name"), "{}", quote);
        let slash = powershell_altered("C:\\probe dir\\").expect("a trailing backslash after whitespace was not refused");
        assert!(slash.contains("'C:\\probe dir\\'") && slash.contains("ends in a backslash"), "{}", slash);
        assert!(slash.contains("without the trailing backslash, which names the same directory"), "{}", slash);
        for passes in ["a b'c", "C:\\hooks\\", "C:\\probe dir", ""] {
            assert_eq!(powershell_altered(passes), None, "{:?}", passes);
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a relative core.hooksPath resolves physically, as git
    // resolves it: `alias/../hooks` with `alias` a link reads the link target's sibling, never the
    // lexical `<top>/hooks`
    #[cfg(unix)]
    #[test]
    fn a_hooks_path_through_a_link_and_dotdot_reads_the_directory_git_runs() {
        let (d, top, hooks) = scratch_repo("physical");
        let outside = d.join("outside");
        std::fs::create_dir_all(outside.join("sub")).expect("scratch");
        std::os::unix::fs::symlink(outside.join("sub"), Path::new(&top).join("alias")).expect("symlink");
        plant(&outside.join("hooks"), "pre-commit", true);
        std::fs::create_dir_all(Path::new(&top).join("hooks")).expect("the lexical reading's directory");
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", "alias/../hooks"]).expect("git config");
        let got = prior(&top, &hooks);
        let runs = outside.join("hooks");
        let same = matches!(&got, Ok(Prior::Live { dir, .. }) if hook_launcher::same_dir(dir, &runs));
        let _ = std::fs::remove_dir_all(&d);
        assert!(matches!(&got, Ok(Prior::Live { names, .. }) if names == &vec!["pre-commit".to_string()]), "{:?}", got);
        assert!(same, "{:?} is not {}", got, runs.display());
    }

    // spec: gate-sdk/SPEC.md §install-hooks — on a host with an executable bit, a hook without it is
    // one git skips, so it proceeds
    #[cfg(unix)]
    #[test]
    fn a_hook_without_the_executable_bit_proceeds() {
        let (d, top, hooks) = scratch_repo("bit");
        plant(&PathBuf::from(format!("{}/.git/hooks", top)), "pre-commit", false);
        let got = prior(&top, &hooks);
        let _ = std::fs::remove_dir_all(&d);
        assert!(matches!(&got, Ok(Prior::Quiet { set: None, .. })), "{:?}", got);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a core.hooksPath already naming the hooks directory is
    // a re-run, whatever the default directory holds
    #[test]
    fn a_rerun_proceeds_past_a_live_hook_in_the_default_directory() {
        let (d, top, hooks) = scratch_repo("rerun");
        plant(&PathBuf::from(format!("{}/.git/hooks", top)), "pre-commit", true);
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &hooks.display().to_string()]).expect("git config");
        let got = prior(&top, &hooks);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(got, Ok(Prior::Own));
    }

    // spec: gate-sdk/SPEC.md §install-hooks — both served hooks are placed whatever the registry
    // holds, each the binary's bytes, and a re-run replaces a file already there
    #[test]
    fn both_hooks_are_placed_and_a_rerun_replaces_them() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        place(&bin, &hooks, &[]).expect("first placement");
        let names: Vec<String> = walk::list_dir(&hooks).expect("lists").into_iter().map(|(n, _)| n).collect();
        std::fs::remove_file(&bin).expect("the update's unlink");
        std::fs::write(&bin, "two").expect("the updated binary");
        let stale = std::fs::read_to_string(hooks.join(hook_launcher::file_name("pre-commit"))).expect("reads");
        place(&bin, &hooks, &[]).expect("second placement");
        let fresh: Vec<String> = hook_launcher::SERVED
            .iter()
            .map(|h| std::fs::read_to_string(hooks.join(hook_launcher::file_name(h))).expect("reads"))
            .collect();
        let executable = crate::proc::is_executable(&hooks.join(hook_launcher::file_name("commit-msg")));
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(names, vec![hook_launcher::file_name("commit-msg"), hook_launcher::file_name("pre-commit")]);
        assert_eq!(stale, "one", "a replaced binary must leave the placed hook as it was");
        assert_eq!(fresh, vec!["two".to_string(), "two".to_string()]);
        assert!(executable);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a replacement that cannot be written leaves both
    // hooks as they were and no sibling behind, and so does a re-run over an unchanged binary
    #[test]
    fn a_replacement_that_cannot_be_written_leaves_the_placed_hooks() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-kept.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        place(&bin, &hooks, &[]).expect("first placement");
        place(&bin, &hooks, &[]).expect("a re-run over the same file");
        let listed = |dir: &Path| -> Vec<String> { walk::list_dir(dir).expect("lists").into_iter().map(|(n, _)| n).collect() };
        let rerun = listed(&hooks);
        let refused = place(&d.join("no-such-binary"), &hooks, &[]);
        let after = listed(&hooks);
        let kept: Vec<String> = hook_launcher::SERVED
            .iter()
            .map(|h| std::fs::read_to_string(hooks.join(hook_launcher::file_name(h))).expect("the hook is still there"))
            .collect();
        let _ = std::fs::remove_dir_all(&d);
        let both = vec![hook_launcher::file_name("commit-msg"), hook_launcher::file_name("pre-commit")];
        assert_eq!(rerun, both, "a re-run left a sibling behind");
        assert!(refused.is_err_and(|e| e.contains("cannot place")), "an absent source must refuse");
        assert_eq!(after, both, "a refused placement left a sibling behind or removed a hook");
        assert_eq!(kept, vec!["one".to_string(), "one".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — two placements forced onto one sibling name: the
    // second refuses, writes nothing through the first's sibling and leaves it there, and a
    // placement under a name of its own replaces both hooks
    #[test]
    fn a_placement_drawing_a_name_already_there_refuses_and_leaves_it() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-survivor.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        place(&bin, &hooks, &[]).expect("first placement");
        std::fs::remove_file(&bin).expect("the update's unlink");
        std::fs::write(&bin, "two").expect("the updated binary");
        let served = hooks.join(hook_launcher::file_name("pre-commit"));
        let theirs = hooks.join(staged_name("pre-commit", "7.drawn"));
        std::fs::hard_link(&served, &theirs).expect("another placement's sibling");
        let copied = copy_new(&bin, &theirs);
        let second = place_as(&bin, &hooks, &[], "7.drawn");
        let through = std::fs::read_to_string(&served).expect("reads");
        let kept = std::fs::read_to_string(&theirs).unwrap_or_default();
        let replaced = place(&bin, &hooks, &[]);
        let fresh = std::fs::read_to_string(&served).expect("reads");
        let left: Vec<String> = walk::list_dir(&hooks).expect("lists").into_iter().map(|(n, _)| n).collect();
        let _ = std::fs::remove_dir_all(&d);
        assert!(copied.is_err_and(|e| e.kind() == std::io::ErrorKind::AlreadyExists), "the copy opened a name already there");
        assert!(second.is_err_and(|e| e.contains("cannot place")), "a second placement on one name must refuse");
        assert_eq!(through, "one", "the refused placement wrote through the sibling into the placed hook");
        assert_eq!(kept, "one", "the refused placement removed or replaced the other's sibling");
        assert_eq!(replaced, Ok(()));
        assert_eq!(fresh, "two");
        assert_eq!(
            left,
            vec![
                hook_launcher::file_name("commit-msg"),
                hook_launcher::file_name("pre-commit"),
                staged_name("pre-commit", "7.drawn")
            ],
            "a placement left its own sibling behind or removed another's"
        );
    }

    // spec: gate-sdk/SPEC.md §install-hooks — two runs of one process draw two names, and a
    // sibling that cannot be removed is reported with its path where an absent one is not
    #[test]
    fn a_run_draws_its_own_name_and_a_failed_removal_is_reported() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-removal.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("held")).expect("scratch");
        let (first, second) = (run_tag(), run_tag());
        let held = remove_own(&d.join("held"));
        let absent = remove_own(&d.join("absent"));
        let _ = std::fs::remove_dir_all(&d);
        assert_ne!(first, second, "two runs drew one sibling name");
        assert!(first.starts_with(&format!("{}.", std::process::id())), "{}", first);
        assert!(held.is_some_and(|r| r.contains("cannot remove") && r.contains("held")), "a failed removal went unreported");
        assert_eq!(absent, None);
    }

    struct CutShort<F: FnMut()>(F);

    impl<F: FnMut()> std::io::Read for CutShort<F> {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            (self.0)();
            Err(std::io::Error::other("the source went away"))
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a copy cut short removes the file it created, and a
    // sibling it cannot remove is named in the refusal
    #[test]
    fn a_copy_cut_short_removes_its_file_or_names_it() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-cut.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (gone, held) = (d.join("gone"), d.join("held"));
        let removed = copy_from(&mut CutShort(|| {}), &gone);
        let left = gone.exists();
        let swap = || {
            if cfg!(unix) {
                std::fs::remove_file(&held).expect("unlinks the open sibling");
                std::fs::create_dir(&held).expect("a name the removal cannot take");
            }
        };
        let kept = copy_from(&mut CutShort(swap), &held);
        let _ = std::fs::remove_dir_all(&d);
        assert!(removed.is_err_and(|e| !e.to_string().contains("cannot remove")), "a removal that worked was reported");
        assert!(!left, "a copy cut short left its file behind");
        let named = kept.is_err_and(|e| {
            let said = e.to_string();
            said.contains("the source went away") && said.contains("cannot remove") && said.contains("held")
        });
        assert_eq!(named, cfg!(unix), "a failed removal must join the refusal");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — another run's sibling is neither removed nor renamed
    #[test]
    fn another_runs_sibling_is_left_alone() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-foreign.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("git/gate-hooks")).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        let foreign: Vec<PathBuf> = hook_launcher::SERVED
            .iter()
            .map(|h| hooks.join(format!("{}{}.0", hook_launcher::file_name(h), STAGED_SUFFIX)))
            .collect();
        for f in &foreign {
            std::fs::write(f, "theirs").expect("another run's sibling");
        }
        let placed = place(&bin, &hooks, &[]);
        let kept: Vec<String> = foreign.iter().map(|f| std::fs::read_to_string(f).unwrap_or_default()).collect();
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(placed, Ok(()));
        assert_eq!(kept, vec!["theirs".to_string(), "theirs".to_string()]);
    }
}
