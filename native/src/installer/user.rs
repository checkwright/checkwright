// spec: installer/SPEC.md §The user install — one tree-free hook arm installed for the user rather
// than for a repository: the binary and the arm's knob file in the per-user data dir, the arm's
// writes in the state dir, and the harness settings entry printed, never written
use super::{refuse, Refusal, FLAGS};
use crate::exit;
use crate::hook::{self, binding};
use crate::{install, sha256, toolfloor, walk};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};

pub const FLAG: &str = "--user";
pub const SCHEMA: &str = "checkwright-user-lock v1";
pub const LOCK_FILE: &str = "user.lock";
const APP: &str = "checkwright";
const BIN_DIR: &str = "bin";
const KNOBS_DIR: &str = "knobs";
const SCRATCH_DIR: &str = "tmp";
const WINDOWS_STATE_DIR: &str = "state";
const OBSERVE_LOG: &str = "hook-observe.log";
const SCRATCH_KNOB: &str = "GATE_SDK_TMP_DIR";
const STOP_LIVENESS: &str = "subagent-stop-liveness";
const STOP_LOG_KNOB: &str = "DELEGATION_KIT_STOP_LOG";
const WAKEUP: &str = "wakeup-guard";
const WAKEUP_LOG_KNOB: &str = "GUARD_KIT_WAKEUP_LOG";

pub(super) const USAGE: &[&str] = &[
    "usage: checkwright init --user <arm> [--dry-run] [--force]",
    "       checkwright update --user [--dry-run] [--force]",
    "       checkwright uninstall --user [--dry-run] [--force]",
    "",
    "Installs one session-hook arm for this user rather than for a repository:",
    "the gate binary and the arm's knob file go in the per-user data dir, and",
    "the arm's scratch and log in the per-user state dir. No repository is read",
    "or written. The harness settings entry is printed for you to add; nothing",
    "writes it. update re-places every recorded arm; uninstall removes them and",
    "keeps the state dir. --force rewrites or removes a knob file you edited.",
];

// spec: installer/SPEC.md §The user install — the form is chosen by the flag's presence anywhere in
// argv, so each verb hands the whole argv to this module before its own parser runs
pub fn routes(args: &[String]) -> bool {
    args.iter().any(|a| a == FLAG || a.starts_with("--user="))
}

#[derive(Debug)]
pub(super) struct Flags {
    member: Option<String>,
    dry: bool,
    force: bool,
}

// spec: installer/SPEC.md §The user install — the parse is syntactic: `init` takes the member as
// `--user`'s operand, `update` and `uninstall` take none, and every repository flag is refused
// naming it
pub(super) fn parse(verb: &str, args: &[String]) -> Result<Option<Flags>, Refusal> {
    let mut f = Flags {
        member: None,
        dry: false,
        force: false,
    };
    if args.iter().any(|a| a == "-h" || a == "--help") {
        for line in USAGE {
            println!("{}", line);
        }
        return Ok(None);
    }
    let mut seen = false;
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "--dry-run" => f.dry = true,
            "--force" => f.force = true,
            _ => {
                let (head, inline) = match a.split_once('=') {
                    Some((h, v)) => (h, Some(v)),
                    None => (a, None),
                };
                if head == FLAG {
                    if seen {
                        return Err(refuse("--user given twice", "", exit::USAGE));
                    }
                    seen = true;
                    if verb != "init" {
                        if inline.is_some() {
                            return Err(refuse(
                                format!("{} --user takes no operand", verb),
                                format!("{} --user acts on every arm {} records.", verb, LOCK_FILE),
                                exit::USAGE,
                            ));
                        }
                    } else if let Some(v) = inline {
                        f.member = Some(v.to_string());
                    } else if let Some(next) = args.get(i + 1).filter(|n| !n.starts_with('-')) {
                        f.member = Some(next.clone());
                        i += 1;
                    } else {
                        return Err(member_refusal("init --user names no arm"));
                    }
                } else if FLAGS.iter().any(|(name, _)| *name == head) {
                    return Err(refuse(
                        format!("{} does not combine with --user", head),
                        "a user install vendors nothing into a repository and commits nothing, so it takes only --dry-run and --force.",
                        exit::USAGE,
                    ));
                } else {
                    return Err(refuse(format!("unknown argument: {}", a), "", exit::USAGE));
                }
            }
        }
        i += 1;
    }
    Ok(Some(f))
}

fn member_refusal(message: &str) -> Refusal {
    refuse(
        message,
        format!(
            "a user install serves an arm that reads no repository: {}.",
            hook::tree_free_members().join(", ")
        ),
        exit::REFUSED,
    )
}

// spec: installer/SPEC.md §The user install — the operand is a member of the table's tree-free
// set, derived and never listed
fn member(named: Option<&str>) -> Result<String, Refusal> {
    let Some(m) = named else {
        return Err(member_refusal("init --user names no arm"));
    };
    if hook::tree_free_members().contains(&m) {
        return Ok(m.to_string());
    }
    Err(member_refusal(&format!("{} is not an arm a user install serves", m)))
}

pub fn run(verb: &str, args: &[String]) -> i32 {
    super::finish(
        verb,
        parse(verb, args).and_then(|f| match f {
            None => Ok(0),
            Some(f) => act(verb, &f),
        }),
    )
}

fn act(verb: &str, f: &Flags) -> Result<i32, Refusal> {
    let member = if verb == "init" { Some(member(f.member.as_deref())?) } else { None };
    let dirs = dirs(&|k| std::env::var_os(k), cfg!(windows))?;
    let out = &mut std::io::stdout();
    match (verb, member) {
        ("init", Some(m)) => init_one(&dirs, &source()?, &m, f, out).map(|_| 0),
        ("update", _) => update(&dirs, f, out),
        _ => uninstall(&dirs, f, out),
    }
}

// spec: installer/SPEC.md §The user install — what the running package supplies: the artifact
// the bootstrap verified, and the version and commit stamped at pack time
pub(super) struct Source {
    artifact: PathBuf,
    target: String,
    digest: String,
    version: String,
    commit: String,
}

fn source() -> Result<Source, Refusal> {
    let pkg = super::package("init --user places the gate binary")?;
    let version = super::init::read_package_field(&pkg, &["version"]);
    if version.is_empty() {
        return Err(refuse(
            "this package carries no version stamp",
            "the version is stamped at pack time from the release tag; a package without one was not assembled by the pack step.",
            exit::INPUT,
        ));
    }
    let digest = sha256::file_hex(&pkg.artifact).map_err(|e| refuse(e, "", 1))?;
    Ok(Source {
        commit: super::init::read_package_field(&pkg, &["checkwright", "commit"]),
        artifact: pkg.artifact,
        target: pkg.target,
        digest,
        version,
    })
}

// spec: installer/SPEC.md §The user install — the two per-user roots, each held spellable: UTF-8
// and free of a control character, since both are written into a knob file and printed JSON
#[derive(Debug)]
pub(super) struct Dirs {
    data: PathBuf,
    state: PathBuf,
}

type Env<'a> = &'a dyn Fn(&str) -> Option<OsString>;

fn dirs(env: Env, windows: bool) -> Result<Dirs, Refusal> {
    let (data, state) = if windows {
        let data = required(env, "LOCALAPPDATA", true)?.join(APP);
        let state = data.join(WINDOWS_STATE_DIR);
        (data, state)
    } else {
        let xdg = |k: &str| env(k).map(PathBuf::from).filter(|p| walk::absolute_on(&p.to_string_lossy(), false));
        let data = match xdg("XDG_DATA_HOME") {
            Some(p) => p,
            None => required(env, "HOME", false)?.join(".local").join("share"),
        };
        let state = match xdg("XDG_STATE_HOME") {
            Some(p) => p,
            None => required(env, "HOME", false)?.join(".local").join("state"),
        };
        (data.join(APP), state.join(APP))
    };
    for (which, p) in [("data", &data), ("state", &state)] {
        let Some(text) = p.to_str() else {
            return Err(refuse(
                format!("the per-user {} dir is not UTF-8: {}", which, p.to_string_lossy()),
                "the path is written into a knob file and printed as JSON, and neither can spell it. Point the variable it derives from at a UTF-8 path.",
                exit::REFUSED,
            ));
        };
        if text.chars().any(char::is_control) {
            return Err(refuse(
                format!("the per-user {} dir carries a tab or another control character: {:?}", which, text),
                "the knob file's grammar refuses such a value. Point the variable it derives from at a path without one.",
                exit::REFUSED,
            ));
        }
    }
    Ok(Dirs { data, state })
}

fn required(env: Env, var: &str, windows: bool) -> Result<PathBuf, Refusal> {
    let help = format!("the user install derives its directories from {}; set it to an absolute path, then re-run.", var);
    match env(var).filter(|v| !v.is_empty()) {
        None => Err(refuse(format!("{} is unset", var), help, exit::REFUSED)),
        Some(v) => {
            let p = PathBuf::from(v);
            if walk::absolute_on(&p.to_string_lossy(), windows) {
                Ok(p)
            } else {
                Err(refuse(format!("{} is not an absolute path: {}", var, p.to_string_lossy()), help, exit::REFUSED))
            }
        }
    }
}

fn spell(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

fn binary_rel() -> String {
    format!("{}/{}{}", BIN_DIR, env!("CARGO_PKG_NAME"), std::env::consts::EXE_SUFFIX)
}

fn binary(d: &Dirs) -> PathBuf {
    at_rel(d, &binary_rel())
}

fn knob_rel(member: &str) -> String {
    format!("{}/{}.knobs", KNOBS_DIR, member)
}

// spec: installer/SPEC.md §The user manifest — a `files` key joins onto the data dir component by
// component, so each must be a plain name on either host
fn inside_data_dir(rel: &str) -> bool {
    rel.split('/').all(|c| !c.is_empty() && c != "." && c != ".." && !c.contains(['\\', ':']))
}

fn at_rel(d: &Dirs, rel: &str) -> PathBuf {
    rel.split('/').fold(d.data.clone(), |p, c| p.join(c))
}

// spec: installer/SPEC.md §The user install — the members keeping a log, each with the knob that
// places it
fn log_knob(member: &str) -> Option<&'static str> {
    match member {
        STOP_LIVENESS => Some(STOP_LOG_KNOB),
        WAKEUP => Some(WAKEUP_LOG_KNOB),
        _ => None,
    }
}

fn log_file(d: &Dirs, member: &str) -> PathBuf {
    d.state.join(format!("{}.log", member))
}

// spec: installer/SPEC.md §The user install — absolute scalars: the scratch dir, and the log of a
// member keeping one; the observe log derives under the scratch dir, so a trial needs no other line
fn knob_text(d: &Dirs, member: &str) -> String {
    let mut text = format!("{} = {}\n", SCRATCH_KNOB, spell(&d.state.join(SCRATCH_DIR)));
    if let Some(knob) = log_knob(member) {
        text.push_str(&format!("{} = {}\n", knob, spell(&log_file(d, member))));
    }
    text
}

// spec: installer/SPEC.md §The user manifest — its own schema, reader and writer,
// SHA-256 hashes computed in-process, and no reader shared with `checkwright.lock`
#[derive(Default, Debug, PartialEq)]
struct UserLock {
    version: String,
    commit: String,
    target: String,
    digest: String,
    hooks: Vec<String>,
    state: String,
    files: BTreeMap<String, String>,
}

impl UserLock {
    fn read(path: &Path) -> Result<Option<UserLock>, Refusal> {
        if !path.exists() {
            return Ok(None);
        }
        let unknown = || {
            refuse(
                format!("{} carries a schema this build does not know: {}", LOCK_FILE, path.display()),
                "this manifest was written by a different Checkwright release. Run that release's uninstall --user, or upgrade the installer.",
                exit::INPUT,
            )
        };
        let text = std::fs::read_to_string(path).map_err(|_| unknown())?;
        let doc: Value = serde_json::from_str(&text).map_err(|_| unknown())?;
        if doc.get("schema").and_then(Value::as_str) != Some(SCHEMA) {
            return Err(unknown());
        }
        let s = |v: Option<&Value>| v.and_then(Value::as_str).unwrap_or_default().to_string();
        if let Some(Value::Object(m)) = doc.get("files") {
            if let Some(key) = m.keys().find(|k| !inside_data_dir(k)) {
                return Err(refuse(
                    format!("{} carries a files key that is not a path inside the data dir: {} ({})", LOCK_FILE, key, path.display()),
                    "init --user records only paths under the data dir, so this manifest was edited or corrupted. Restore it, or remove the user install by hand.",
                    exit::INPUT,
                ));
            }
        }
        let artifact = doc.get("artifact");
        Ok(Some(UserLock {
            version: s(doc.get("version")),
            commit: s(doc.get("commit")),
            target: s(artifact.and_then(|a| a.get("target"))),
            digest: s(artifact.and_then(|a| a.get("digest"))),
            hooks: match doc.get("hooks") {
                Some(Value::Array(items)) => items.iter().filter_map(Value::as_str).map(String::from).collect(),
                _ => Vec::new(),
            },
            state: s(doc.get("state")),
            files: match doc.get("files") {
                Some(Value::Object(m)) => m.iter().map(|(k, v)| (k.clone(), s(Some(v)))).collect(),
                _ => BTreeMap::new(),
            },
        }))
    }

    fn render(&self) -> String {
        let mut doc = Map::new();
        let str_v = |s: &str| Value::String(s.to_string());
        doc.insert("schema".to_string(), str_v(SCHEMA));
        doc.insert("version".to_string(), str_v(&self.version));
        if !self.commit.is_empty() {
            doc.insert("commit".to_string(), str_v(&self.commit));
        }
        if !self.target.is_empty() {
            let mut a = Map::new();
            a.insert("target".to_string(), str_v(&self.target));
            a.insert("digest".to_string(), str_v(&self.digest));
            doc.insert("artifact".to_string(), Value::Object(a));
        }
        doc.insert("hooks".to_string(), Value::Array(self.hooks.iter().map(|h| str_v(h)).collect()));
        doc.insert("state".to_string(), str_v(&self.state));
        let files = self.files.iter().map(|(k, v)| (k.clone(), str_v(v))).collect();
        doc.insert("files".to_string(), Value::Object(files));
        let mut text = serde_json::to_string_pretty(&Value::Object(doc)).unwrap_or_default();
        text.push('\n');
        text
    }
}

fn no_lock(verb: &str, d: &Dirs) -> Refusal {
    refuse(
        format!("no {} at {}", LOCK_FILE, spell(&d.data)),
        format!(
            "{} --user manages a user install that 'checkwright init --user <arm>' made; there isn't one yet.",
            verb
        ),
        1,
    )
}

// spec: installer/SPEC.md §The install boundary — a temporary beside the target, then a rename, so
// no reader sees a half-written file and a failed write leaves what was there intact
fn write_atomically(file: &Path, body: &[u8]) -> Result<(), Refusal> {
    let tmp = sibling_tmp(file);
    std::fs::write(&tmp, body).map_err(|e| refuse(format!("cannot write {}: {}", tmp.display(), e), "", 1))?;
    std::fs::rename(&tmp, file).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        refuse(format!("cannot write {}: {}", file.display(), e), "", 1)
    })
}

fn sibling_tmp(file: &Path) -> PathBuf {
    let mut name = file.file_name().map(|n| n.to_os_string()).unwrap_or_default();
    name.push(".tmp");
    file.with_file_name(name)
}

fn make_dir(p: &Path) -> Result<(), Refusal> {
    std::fs::create_dir_all(p).map_err(|e| refuse(format!("cannot create {}: {}", p.display(), e), "", 1))
}

#[derive(Debug, PartialEq)]
enum Knob {
    Write,
    Same,
    Keep,
}

// spec: installer/SPEC.md §The user install — init's five steps for one member; a bare re-run
// leaves the data dir byte-identical
fn init_one(d: &Dirs, src: &Source, member: &str, f: &Flags, out: &mut dyn Write) -> Result<(), Refusal> {
    let lock_path = d.data.join(LOCK_FILE);
    let prior = UserLock::read(&lock_path)?.unwrap_or_default();
    // spec: installer/SPEC.md §update — the downgrade refusal, read against the user manifest
    if !f.force
        && !prior.version.is_empty()
        && prior.version != src.version
        && toolfloor::floor_met(&prior.version, &src.version) == Some(false)
    {
        return Err(refuse(
            format!(
                "this package is {} but {} records {} — refusing a silent downgrade",
                src.version, LOCK_FILE, prior.version
            ),
            "run the release you already have, or pass --force if rolling back is what you meant.",
            1,
        ));
    }
    let bin = binary(d);
    let bin_stale = prior.target != src.target
        || prior.digest != src.digest
        || !bin.is_file()
        || sha256::file_hex(&bin).ok().as_deref() != Some(src.digest.as_str());
    let rel = knob_rel(member);
    let knob = at_rel(d, &rel);
    let text = knob_text(d, member);
    let want = sha256::hex(text.as_bytes());
    let on_disk = if knob.exists() {
        Some(sha256::file_hex(&knob).map_err(|e| refuse(e, "", 1))?)
    } else {
        None
    };
    let act = match on_disk.as_deref() {
        None => Knob::Write,
        Some(h) if h == want => Knob::Same,
        Some(h) if f.force || prior.files.get(&rel).map(String::as_str) == Some(h) => Knob::Write,
        Some(_) => Knob::Keep,
    };
    let mut next = UserLock {
        version: src.version.clone(),
        commit: src.commit.clone(),
        target: src.target.clone(),
        digest: src.digest.clone(),
        hooks: prior.hooks.clone(),
        state: spell(&d.state),
        files: prior.files.clone(),
    };
    if !next.hooks.iter().any(|h| h == member) {
        next.hooks.push(member.to_string());
        next.hooks.sort();
    }
    if act != Knob::Keep {
        next.files.insert(rel.clone(), want);
    }

    if f.dry {
        let _ = writeln!(out, "would install {} for this user at {}", member, spell(&d.data));
        let _ = writeln!(
            out,
            "would {} the gate binary at {}",
            if bin_stale { "place" } else { "keep" },
            spell(&bin)
        );
        let verb = match act {
            Knob::Write => "write",
            Knob::Same => "keep",
            Knob::Keep => "keep, edited since init --user wrote it,",
        };
        let _ = writeln!(out, "would {} the knob file {}", verb, spell(&knob));
        let _ = writeln!(out, "would record {}", spell(&lock_path));
        settings_block(d, member, &bin, &knob, out);
        return Ok(());
    }

    for dir in [d.data.join(BIN_DIR), d.data.join(KNOBS_DIR), d.state.join(SCRATCH_DIR)] {
        make_dir(&dir)?;
    }
    if bin_stale {
        place_binary(&src.artifact, &bin)?;
    }
    if act == Knob::Write {
        write_atomically(&knob, text.as_bytes())?;
    }
    let rendered = next.render();
    if std::fs::read_to_string(&lock_path).ok().as_deref() != Some(rendered.as_str()) {
        write_atomically(&lock_path, rendered.as_bytes())?;
    }

    let knob_word = match act {
        Knob::Write => "written",
        Knob::Same => "unchanged",
        Knob::Keep => "kept",
    };
    let _ = writeln!(
        out,
        "INIT: {} installed for this user at {} (v{}) — the gate binary {}, its knob file {}.",
        member,
        spell(&d.data),
        src.version,
        if bin_stale { "placed" } else { "unchanged" },
        knob_word
    );
    if act == Knob::Keep {
        let _ = writeln!(
            out,
            "kept {}: edited since init --user wrote it; pass --force to rewrite it.",
            spell(&knob)
        );
    }
    settings_block(d, member, &bin, &knob, out);
    Ok(())
}

// spec: installer/SPEC.md §The user install — the artifact goes to a sibling temporary renamed over
// the target; on Windows a rename over an image in use fails, and that is the refusal's remedy
fn place_binary(artifact: &Path, bin: &Path) -> Result<(), Refusal> {
    let tmp = sibling_tmp(bin);
    std::fs::copy(artifact, &tmp).map_err(|e| refuse(format!("cannot write {}: {}", tmp.display(), e), "", 1))?;
    install::make_executable(&tmp).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        refuse(e, "", 1)
    })?;
    std::fs::rename(&tmp, bin).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        refuse(
            format!("cannot replace {}: {}", bin.display(), e),
            "a harness session running the hook holds the binary open where the platform locks a running image. End the sessions running it, then re-run.",
            1,
        )
    })
}

fn settings_file() -> String {
    if cfg!(windows) {
        format!("%USERPROFILE%\\{}\\{}", binding::CONFIG_HOME_DEFAULT, binding::SETTINGS_FILE)
    } else {
        format!("~/{}/{}", binding::CONFIG_HOME_DEFAULT, binding::SETTINGS_FILE)
    }
}

// spec: installer/SPEC.md §The user install — the `settings:` block: a banner, then indented lines,
// the JSON fragment alone on its line so a reader can lift it
fn settings_block(d: &Dirs, member: &str, bin: &Path, knob: &Path, out: &mut dyn Write) {
    let event = hook::event(member).unwrap_or_default();
    let matcher = hook::matcher(member).unwrap_or_default();
    let knob_s = spell(knob);
    let fragment =
        binding::wiring_fragment(event, &matcher, &spell(bin), &["--hook", member, hook::KNOBS_FLAG, &knob_s]);
    let scratch = spell(&d.state.join(SCRATCH_DIR));
    let _ = writeln!(out, "settings:");
    let _ = writeln!(
        out,
        "  {} {} or later: merge this entry into the \"{}\" object of the user settings file, {} (under ${} where that is set):",
        binding::HARNESS,
        binding::EXEC_FORM_FLOOR,
        binding::WIRING_HOOKS,
        settings_file(),
        binding::CONFIG_DIR_VAR
    );
    let _ = writeln!(out, "  {}", fragment);
    let _ = writeln!(
        out,
        "  trial: insert {} after \"{}\" in args to log each would-be decision to {} and enforce nothing.",
        hook::OBSERVE_FLAG,
        member,
        spell(&d.state.join(SCRATCH_DIR).join(OBSERVE_LOG))
    );
    match member {
        STOP_LIVENESS => {
            let _ = writeln!(
                out,
                "  launch records: the arm reads them from {}; point your engine there, or set {} in {} to the directory it writes.",
                scratch, SCRATCH_KNOB, knob_s
            );
        }
        WAKEUP => {
            let _ = writeln!(
                out,
                "  attempt log: the arm records each denied call in {}; set {} in {} to move it.",
                spell(&log_file(d, member)),
                WAKEUP_LOG_KNOB,
                knob_s
            );
        }
        _ => {}
    }
}

// spec: installer/SPEC.md §update — the user form: init --user for each recorded member, so the new
// binary is placed once and each member's settings block reprinted
fn update(d: &Dirs, f: &Flags, out: &mut dyn Write) -> Result<i32, Refusal> {
    let lock = UserLock::read(&d.data.join(LOCK_FILE))?.ok_or_else(|| no_lock("update", d))?;
    if lock.hooks.is_empty() {
        return Err(refuse(
            format!("{} at {} records no installed arm", LOCK_FILE, spell(&d.data)),
            "run 'checkwright init --user <arm>' to install one.",
            1,
        ));
    }
    let free = hook::tree_free_members();
    if let Some(gone) = lock.hooks.iter().find(|h| !free.contains(&h.as_str())) {
        return Err(member_refusal(&format!("{} records {}, which this release does not serve", LOCK_FILE, gone)));
    }
    let src = source()?;
    for m in &lock.hooks {
        init_one(d, &src, m, f, out)?;
    }
    Ok(0)
}

// spec: installer/SPEC.md §uninstall — the user form: the binary goes whatever its hash, a knob file
// still at its recorded hash goes, an edited one stays named, and the state dir is the adopter's
fn uninstall(d: &Dirs, f: &Flags, out: &mut dyn Write) -> Result<i32, Refusal> {
    let lock_path = d.data.join(LOCK_FILE);
    let lock = UserLock::read(&lock_path)?.ok_or_else(|| no_lock("uninstall", d))?;
    let bin_rel = binary_rel();
    // spec: installer/SPEC.md §uninstall — every path the user form removes or prunes, read for a
    // link or junction below the data dir before the first removal
    for rel in [bin_rel.as_str(), BIN_DIR, KNOBS_DIR].into_iter().chain(lock.files.keys().map(String::as_str)) {
        let link = super::link_below(&d.data, rel)
            .map_err(|e| refuse(format!("could not read where {} lands: {}", spell(&at_rel(d, rel)), e), "", exit::INPUT))?;
        if let Some(at) = link {
            return Err(refuse(
                format!("{} reaches through a link at {}", spell(&at_rel(d, rel)), spell(&at_rel(d, &at))),
                "uninstall --user removes only what lies inside the data dir, and a link or junction there could reach a file outside it. Replace the link with the directory or file itself, or remove it, then re-run.",
                exit::REFUSED,
            ));
        }
    }
    let bin = binary(d);
    let mut remove: Vec<PathBuf> = Vec::new();
    let mut kept: BTreeMap<String, String> = BTreeMap::new();
    for (rel, h) in &lock.files {
        let p = at_rel(d, rel);
        if !p.is_file() {
            continue;
        }
        if f.force || sha256::file_hex(&p).ok().as_deref() == Some(h.as_str()) {
            remove.push(p);
        } else {
            kept.insert(rel.clone(), h.clone());
        }
    }
    if f.dry {
        if bin.exists() {
            let _ = writeln!(out, "would remove {}", spell(&bin));
        }
        for p in &remove {
            let _ = writeln!(out, "would remove {}", spell(p));
        }
        for rel in kept.keys() {
            let _ = writeln!(out, "would keep {}: edited since init --user wrote it", spell(&at_rel(d, rel)));
        }
        let _ = writeln!(out, "would {} {}", if kept.is_empty() { "remove" } else { "narrow" }, spell(&lock_path));
        return Ok(0);
    }
    if bin.exists() {
        std::fs::remove_file(&bin).map_err(|e| {
            refuse(
                format!("cannot remove {}: {}", bin.display(), e),
                "a harness session running the hook holds the binary open where the platform locks a running image. End the sessions running it, then re-run.",
                1,
            )
        })?;
    }
    for p in &remove {
        std::fs::remove_file(p).map_err(|e| refuse(format!("cannot remove {}: {}", p.display(), e), "", 1))?;
    }
    if kept.is_empty() {
        std::fs::remove_file(&lock_path).map_err(|e| refuse(format!("cannot remove {}: {}", lock_path.display(), e), "", 1))?;
    } else {
        let narrowed = UserLock {
            version: lock.version.clone(),
            state: lock.state.clone(),
            files: kept.clone(),
            ..UserLock::default()
        };
        write_atomically(&lock_path, narrowed.render().as_bytes())?;
    }
    for dir in [d.data.join(BIN_DIR), d.data.join(KNOBS_DIR), d.data.clone()] {
        let _ = std::fs::remove_dir(dir);
    }
    let _ = writeln!(
        out,
        "UNINSTALL: removed the user install at {} — the gate binary and {} knob file(s) removed, {} kept.",
        spell(&d.data),
        remove.len(),
        kept.len()
    );
    for rel in kept.keys() {
        let _ = writeln!(
            out,
            "kept {}: edited since init --user wrote it; {} names it, and --force removes it.",
            spell(&at_rel(d, rel)),
            LOCK_FILE
        );
    }
    if d.data.exists() {
        let _ = writeln!(out, "remains {}", spell(&d.data));
    }
    let state = if lock.state.is_empty() { spell(&d.state) } else { lock.state.clone() };
    if Path::new(&state).exists() {
        let _ = writeln!(out, "remains {}: the arm's logs and launch records, yours to keep or delete.", state);
    }
    for m in &lock.hooks {
        let _ = writeln!(
            out,
            "settings: remove the {} entry running {} from {} — the binary it names is gone.",
            hook::event(m).unwrap_or_default(),
            spell(&bin),
            settings_file()
        );
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEMBER: &str = "subagent-stop-liveness";

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    fn env_of(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<OsString> {
        let m: BTreeMap<String, OsString> = pairs.iter().map(|(k, v)| (k.to_string(), OsString::from(v))).collect();
        move |k| m.get(k).cloned()
    }

    struct Scratch(PathBuf);

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    impl Scratch {
        fn new(case: &str) -> Scratch {
            let dir = std::env::temp_dir().join(format!("cw-user-install-{}-{}", std::process::id(), case));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("cannot make the scratch dir");
            std::fs::write(dir.join("artifact"), b"the gate binary\n").expect("cannot write the artifact");
            Scratch(dir)
        }

        fn dirs(&self) -> Dirs {
            Dirs {
                data: self.0.join("data").join(APP),
                state: self.0.join("state").join(APP),
            }
        }

        fn source(&self, version: &str) -> Source {
            Source {
                artifact: self.0.join("artifact"),
                target: "x86_64-unknown-linux-musl".to_string(),
                digest: sha256::file_hex(&self.0.join("artifact")).expect("digest"),
                version: version.to_string(),
                commit: "abc".to_string(),
            }
        }
    }

    fn flags(dry: bool, force: bool) -> Flags {
        Flags {
            member: None,
            dry,
            force,
        }
    }

    fn snapshot(dir: &Path) -> BTreeMap<String, Vec<u8>> {
        super::super::files_under(dir)
            .expect("cannot walk the data dir")
            .into_iter()
            .map(|rel| {
                let bytes = std::fs::read(dir.join(&rel)).expect("cannot read a data-dir file");
                (rel, bytes)
            })
            .collect()
    }

    fn init(s: &Scratch, d: &Dirs, version: &str, f: &Flags) -> Result<String, Refusal> {
        let mut out = Vec::new();
        init_one(d, &s.source(version), MEMBER, f, &mut out)?;
        Ok(String::from_utf8_lossy(&out).into_owned())
    }

    // spec: installer/SPEC.md §The user install — XDG roots when absolute, `$HOME` defaults when a
    // value is relative or unset, and `%LOCALAPPDATA%` with a state subdirectory on Windows
    #[test]
    fn the_dirs_resolve_by_the_base_directory_rule() {
        let d = dirs(&env_of(&[("HOME", "/h"), ("XDG_DATA_HOME", "/xd"), ("XDG_STATE_HOME", "/xs")]), false).expect("xdg");
        assert_eq!((d.data, d.state), (PathBuf::from("/xd/checkwright"), PathBuf::from("/xs/checkwright")));
        let d = dirs(&env_of(&[("HOME", "/h"), ("XDG_DATA_HOME", "rel"), ("XDG_STATE_HOME", "")]), false).expect("home");
        assert_eq!(d.data, Path::new("/h").join(".local").join("share").join(APP));
        assert_eq!(d.state, Path::new("/h").join(".local").join("state").join(APP));
        let d = dirs(&env_of(&[("LOCALAPPDATA", "C:/l")]), true).expect("windows");
        assert_eq!((d.data.clone(), d.state), (Path::new("C:/l").join(APP), Path::new("C:/l").join(APP).join("state")));
        let d = dirs(&env_of(&[("HOME", "/h"), ("XDG_DATA_HOME", "C:/xd"), ("XDG_STATE_HOME", "\\xs")]), false)
            .expect("home");
        assert_eq!(d.data, Path::new("/h").join(".local").join("share").join(APP), "a drive spelling is relative on POSIX");
        assert_eq!(d.state, Path::new("/h").join(".local").join("state").join(APP), "a backslash is relative on POSIX");
    }

    // spec: gate-sdk/SPEC.md §The crate's crosser — a root spelling whose location depends on the
    // host's context is refused as relative: a lone leading separator on Windows, a drive spelling
    // and a leading backslash on POSIX
    #[test]
    fn a_context_dependent_root_is_refused_as_relative() {
        for (var, value, windows) in [
            ("LOCALAPPDATA", "\\l", true),
            ("LOCALAPPDATA", "/l", true),
            ("HOME", "C:/h", false),
            ("HOME", "\\h", false),
        ] {
            let r = dirs(&env_of(&[(var, value)]), windows).expect_err(value);
            let want = format!("{} is not an absolute path", var);
            assert!(r.code == 2 && r.message.contains(&want), "{}: {}", value, r.message);
        }
    }

    // spec: installer/SPEC.md §The user install — an unset root variable, a relative one, a tab and
    // a non-UTF-8 path each refuse at exit 2 naming what is at fault
    #[test]
    fn an_unusable_root_is_refused_naming_it() {
        let r = dirs(&env_of(&[("XDG_DATA_HOME", "/xd")]), false).expect_err("unset HOME passed");
        assert_eq!((r.code, r.message.as_str()), (2, "HOME is unset"));
        let r = dirs(&env_of(&[]), true).expect_err("unset LOCALAPPDATA passed");
        assert_eq!((r.code, r.message.as_str()), (2, "LOCALAPPDATA is unset"));
        let r = dirs(&env_of(&[("HOME", "rel")]), false).expect_err("relative HOME passed");
        assert!(r.code == 2 && r.message.contains("HOME is not an absolute path"), "{}", r.message);
        let r = dirs(&env_of(&[("HOME", "/h\tx")]), false).expect_err("a tab passed");
        assert!(r.code == 2 && r.message.contains("control character"), "{}", r.message);
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            let bad = OsString::from_vec(b"/h\xff".to_vec());
            let r = dirs(&move |k| (k == "HOME").then(|| bad.clone()), false).expect_err("non-UTF-8 passed");
            assert!(r.code == 2 && r.message.contains("not UTF-8"), "{}", r.message);
        }
    }

    // spec: installer/SPEC.md §The user install — the operand in both spellings, a missing or
    // non-tree-free one refused naming the tree-free members, and every repository flag refused
    #[test]
    fn the_operand_and_the_flags_are_refused_where_the_form_does_not_take_them() {
        let f = parse("init", &args(&["--user", MEMBER, "--dry-run"])).expect("parse").expect("flags");
        assert_eq!((f.member.as_deref(), f.dry), (Some(MEMBER), true));
        let f = parse("init", &args(&[&format!("--user={}", MEMBER)])).expect("parse").expect("flags");
        assert_eq!(f.member.as_deref(), Some(MEMBER));
        let r = parse("init", &args(&["--user", "--force"])).expect_err("a missing operand passed");
        assert!(r.code == 2 && r.help.contains(MEMBER), "{}", r.help);
        let r = member(Some("shell-guard")).expect_err("a tree-bound member passed");
        assert!(r.code == 2 && r.message.contains("shell-guard") && r.help.contains(MEMBER), "{}", r.message);
        let r = parse("update", &args(&["--user=x"])).expect_err("an operand on update passed");
        assert_eq!(r.code, 2);
        for verb in ["init", "update", "uninstall"] {
            for (flag, _) in FLAGS.iter().filter(|(f, _)| !["--dry-run", "--force", FLAG].contains(f)) {
                let mut a = args(&["--user"]);
                if verb == "init" {
                    a.push(MEMBER.to_string());
                }
                a.push(flag.to_string());
                let r = parse(verb, &a).expect_err("a repository flag passed");
                assert_eq!((r.code, r.message.clone()), (2, format!("{} does not combine with --user", flag)), "{}", verb);
            }
            let r = parse(verb, &args(&["--user", "--bogus"])).expect_err("an unknown argument passed");
            assert_eq!(r.code, 2, "{}", verb);
        }
    }

    // spec: installer/SPEC.md §The user install — the install's layout, its knob file's two
    // absolute scalars, the printed fragment carrying the absolute binary and file, and a bare
    // re-run leaving the data dir byte-identical
    #[test]
    fn init_places_the_arm_prints_its_entry_and_a_rerun_changes_nothing() {
        let s = Scratch::new("init");
        let d = s.dirs();
        let report = init(&s, &d, "1.2.0", &flags(false, false)).expect("init");
        let bin = binary(&d);
        let knob = at_rel(&d, &knob_rel(MEMBER));
        assert_eq!(std::fs::read(&bin).expect("binary"), b"the gate binary\n");
        assert_eq!(std::fs::read_to_string(&knob).expect("knob file"), knob_text(&d, MEMBER));
        assert!(d.state.join(SCRATCH_DIR).is_dir(), "the scratch dir was not made");
        let lock = UserLock::read(&d.data.join(LOCK_FILE)).expect("lock").expect("a lock");
        assert_eq!((lock.hooks.clone(), lock.version.as_str()), (vec![MEMBER.to_string()], "1.2.0"));
        let fragment = report
            .lines()
            .skip_while(|l| *l != "settings:")
            .map(str::trim)
            .find(|l| l.starts_with('{'))
            .expect("no fragment line");
        let doc: Value = serde_json::from_str(fragment).expect("the fragment is not JSON");
        let entry = &doc[binding::EVENT_CHILD_TURN_END][0]["hooks"][0];
        assert_eq!(entry["command"], Value::String(spell(&bin)));
        assert_eq!(entry["args"], serde_json::json!(["--hook", MEMBER, "--knobs", spell(&knob)]));
        assert!(report.contains(binding::EXEC_FORM_FLOOR) && !report.contains("inferred"), "{}", report);
        let before = snapshot(&d.data);
        let again = init(&s, &d, "1.2.0", &flags(false, false)).expect("re-run");
        assert_eq!(snapshot(&d.data), before, "a bare re-run changed the data dir");
        assert!(again.contains("binary unchanged") && again.contains("knob file unchanged"), "{}", again);
    }

    // spec: installer/SPEC.md §The user install — for every tree-free member the printed group
    // carries its event and matcher, and the knob file it writes reports no fault when the member
    // reads it with no repository
    #[test]
    fn every_tree_free_member_installs_a_fault_free_knob_file_under_its_matcher() {
        let s = Scratch::new("members");
        let d = s.dirs();
        let members: Vec<&str> = crate::hook::HOOKS.iter().filter(|r| r.4).map(|r| r.0).collect();
        assert_eq!(members.len(), 5, "{:?}", members);
        for m in members {
            let mut out = Vec::new();
            init_one(&d, &s.source("1.2.0"), m, &flags(false, false), &mut out).expect("init");
            let report = String::from_utf8_lossy(&out).into_owned();
            let fragment = report
                .lines()
                .skip_while(|l| *l != "settings:")
                .map(str::trim)
                .find(|l| l.starts_with('{'))
                .expect("no fragment line");
            let doc: Value = serde_json::from_str(fragment).expect("the fragment is not JSON");
            let group = &doc[crate::hook::event(m).expect("a row")][0];
            match crate::hook::matcher(m).filter(|s| !s.is_empty()) {
                Some(want) => assert_eq!(group[binding::WIRING_MATCHER], Value::String(want), "{}", m),
                None => assert!(group.get(binding::WIRING_MATCHER).is_none(), "{}", m),
            }
            let knob = at_rel(&d, &knob_rel(m));
            let text = std::fs::read_to_string(&knob).expect("knob file");
            assert_eq!(text, knob_text(&d, m));
            assert_eq!(crate::knobs::check_hook_file(&spell(&knob)), Ok(()), "{}", m);
            assert_eq!(text.contains(STOP_LOG_KNOB), m == STOP_LIVENESS, "{}", m);
            assert_eq!(text.contains(WAKEUP_LOG_KNOB), m == WAKEUP, "{}", m);
            assert_eq!(report.contains("launch records:"), m == STOP_LIVENESS, "{}", m);
            assert_eq!(report.contains("attempt log:"), m == WAKEUP, "{}", m);
            let _marked = crate::hook::Marked::new(&format!("user-{}", m), Some(&text));
            assert_eq!(crate::hook::knob_reads(m), Ok(()), "{}", m);
        }
    }

    // spec: installer/SPEC.md §The user install — a knob file edited since init wrote it is kept and
    // named, its recorded hash carried, and `--force` rewrites it; a dry run writes nothing
    #[test]
    fn an_edited_knob_file_is_kept_unless_forced() {
        let s = Scratch::new("edited");
        let d = s.dirs();
        let dry = init(&s, &d, "1.2.0", &flags(true, false)).expect("dry run");
        assert!(!d.data.exists() && dry.contains("would place"), "{}", dry);
        init(&s, &d, "1.2.0", &flags(false, false)).expect("init");
        let knob = at_rel(&d, &knob_rel(MEMBER));
        std::fs::write(&knob, "GATE_SDK_TMP_DIR = /engine/records\n").expect("edit");
        let report = init(&s, &d, "1.2.0", &flags(false, false)).expect("re-run");
        assert!(report.contains("knob file kept") && report.contains("--force"), "{}", report);
        assert_eq!(std::fs::read_to_string(&knob).expect("knob"), "GATE_SDK_TMP_DIR = /engine/records\n");
        init(&s, &d, "1.2.0", &flags(false, true)).expect("forced");
        assert_eq!(std::fs::read_to_string(&knob).expect("knob"), knob_text(&d, MEMBER));
    }

    // spec: installer/SPEC.md §update — a recorded version newer than the package refuses at exit 1,
    // and `--force` makes the rollback deliberate
    #[test]
    fn an_older_package_is_a_refused_downgrade() {
        let s = Scratch::new("downgrade");
        let d = s.dirs();
        init(&s, &d, "2.0.0", &flags(false, false)).expect("init");
        let r = init(&s, &d, "1.9.0", &flags(false, false)).expect_err("a downgrade passed");
        assert!(r.code == 1 && r.message.contains("refusing a silent downgrade"), "{}", r.message);
        init(&s, &d, "1.9.0", &flags(false, true)).expect("a forced rollback refused");
    }

    // spec: installer/SPEC.md §update — with no user manifest both managing verbs refuse at exit 1
    // naming init --user
    #[test]
    fn update_and_uninstall_need_a_user_install() {
        let s = Scratch::new("nolock");
        let d = s.dirs();
        let r = update(&d, &flags(false, false), &mut Vec::new()).expect_err("update passed");
        assert!(r.code == 1 && r.help.contains("init --user"), "{}", r.help);
        let r = uninstall(&d, &flags(false, false), &mut Vec::new()).expect_err("uninstall passed");
        assert!(r.code == 1 && r.help.contains("init --user"), "{}", r.help);
    }

    // spec: installer/SPEC.md §The user manifest — a `files` key outside the data dir refuses the
    // manifest at exit 3 before anything is removed, a forced uninstall included; the backslash keys
    // are the Windows traversal
    #[test]
    fn a_files_key_outside_the_data_dir_refuses_the_manifest() {
        let s = Scratch::new("traversal");
        let d = s.dirs();
        init(&s, &d, "1.2.0", &flags(false, false)).expect("init");
        let victim = s.0.join("victim");
        std::fs::write(&victim, b"not the install's\n").expect("victim");
        let hash = sha256::file_hex(&victim).expect("hash");
        let lock_path = d.data.join(LOCK_FILE);
        let good = std::fs::read_to_string(&lock_path).expect("lock");
        for key in ["../../victim", "..\\..\\victim", "knobs\\..\\..\\..\\victim", "/abs", "C:/x", "C:x", "knobs//x", "knobs/./x", "./x", ""] {
            let mut doc: Value = serde_json::from_str(&good).expect("lock JSON");
            doc["files"] = serde_json::json!({ key: hash });
            std::fs::write(&lock_path, doc.to_string()).expect("tamper");
            let r = uninstall(&d, &flags(false, true), &mut Vec::new()).expect_err(key);
            assert!(r.code == exit::INPUT && r.message.contains("not a path inside the data dir"), "{:?}: {}", key, r.message);
            assert!(victim.is_file() && binary(&d).is_file(), "{:?}: a refused manifest removed a file", key);
            let r = update(&d, &flags(false, false), &mut Vec::new()).expect_err(key);
            assert_eq!(r.code, exit::INPUT, "{:?}", key);
        }
        std::fs::write(&lock_path, &good).expect("restore");
        uninstall(&d, &flags(false, true), &mut Vec::new()).expect("the restored manifest refused");
        assert!(!d.data.exists() && victim.is_file());
    }

    fn link_dir(target: &Path, at: &Path) {
        #[cfg(unix)]
        std::os::unix::fs::symlink(target, at).expect("cannot link a directory");
        #[cfg(windows)]
        {
            let status = std::process::Command::new("cmd")
                .args(["/C", "mklink", "/J"])
                .arg(at)
                .arg(target)
                .status()
                .expect("cmd did not spawn");
            assert!(status.success(), "mklink /J failed: {}", status);
        }
    }

    fn unlink_dir(at: &Path) {
        #[cfg(unix)]
        std::fs::remove_file(at).expect("cannot remove the link");
        #[cfg(windows)]
        std::fs::remove_dir(at).expect("cannot remove the junction");
    }

    // spec: installer/SPEC.md §uninstall — a link or junction below the data dir on a path the user
    // form removes or prunes refuses at exit 2 before anything is removed, a forced or dry run
    // included, and nothing outside the data dir is touched
    #[test]
    fn a_link_inside_the_data_dir_refuses_the_removal() {
        let s = Scratch::new("contained");
        let d = s.dirs();
        init(&s, &d, "1.2.0", &flags(false, false)).expect("init");
        let outside = s.0.join("outside");
        std::fs::create_dir_all(&outside).expect("outside");
        let victim = outside.join("victim");
        std::fs::write(&victim, b"not the install's\n").expect("victim");
        let hash = sha256::file_hex(&victim).expect("hash");
        let lock_path = d.data.join(LOCK_FILE);
        let good = std::fs::read_to_string(&lock_path).expect("lock");
        let mut doc: Value = serde_json::from_str(&good).expect("lock JSON");
        doc["files"]["escape/victim"] = Value::String(hash);
        std::fs::write(&lock_path, doc.to_string()).expect("tamper");
        let escape = d.data.join("escape");
        link_dir(&outside, &escape);
        for f in [flags(false, true), flags(true, true), flags(false, false)] {
            let mut out = Vec::new();
            let r = uninstall(&d, &f, &mut out).expect_err("a removal through a link passed");
            assert!(r.code == exit::REFUSED && r.message.contains(&format!("a link at {}", spell(&escape))), "{}", r.message);
            assert!(out.is_empty() && victim.is_file() && binary(&d).is_file(), "a refused removal wrote or removed");
        }
        unlink_dir(&escape);
        std::fs::write(&lock_path, &good).expect("restore");
        for (dir, rel) in [(BIN_DIR, binary_rel()), (KNOBS_DIR, knob_rel(MEMBER))] {
            let real = d.data.join(dir);
            let aside = s.0.join(format!("aside-{}", dir));
            std::fs::rename(&real, &aside).expect("move the dir aside");
            link_dir(&aside, &real);
            let r = uninstall(&d, &flags(false, true), &mut Vec::new()).expect_err(dir);
            assert!(r.code == exit::REFUSED && r.message.contains(&format!("a link at {}", spell(&real))), "{}: {}", dir, r.message);
            let leaf = rel.rsplit('/').next().expect("a leaf");
            assert!(aside.join(leaf).is_file(), "{}: the link's target lost {}", dir, leaf);
            unlink_dir(&real);
            std::fs::rename(&aside, &real).expect("move the dir back");
        }
        uninstall(&d, &flags(false, false), &mut Vec::new()).expect("the plain install refused");
        assert!(!d.data.exists() && victim.is_file());
    }

    // spec: installer/SPEC.md §uninstall — the binary and an unedited knob file go, an edited one
    // stays named with the manifest narrowed to it, the state dir is kept, and the entry is recalled
    #[test]
    fn uninstall_reports_its_residue() {
        let s = Scratch::new("residue");
        let d = s.dirs();
        init(&s, &d, "1.2.0", &flags(false, false)).expect("init");
        let knob = at_rel(&d, &knob_rel(MEMBER));
        std::fs::write(&knob, "edited\n").expect("edit");
        let mut out = Vec::new();
        uninstall(&d, &flags(false, false), &mut out).expect("uninstall");
        let report = String::from_utf8_lossy(&out);
        assert!(!binary(&d).exists(), "the binary stayed");
        assert!(knob.is_file() && report.contains(&format!("kept {}", spell(&knob))), "{}", report);
        let lock = UserLock::read(&d.data.join(LOCK_FILE)).expect("lock").expect("a narrowed lock");
        assert_eq!(lock.files.keys().cloned().collect::<Vec<_>>(), vec![knob_rel(MEMBER)]);
        assert!(lock.hooks.is_empty() && lock.target.is_empty(), "{:?}", lock);
        assert!(d.state.is_dir() && report.contains(&format!("remains {}", spell(&d.state))), "{}", report);
        assert!(report.contains(&format!("remove the {} entry", binding::EVENT_CHILD_TURN_END)), "{}", report);
        let mut out = Vec::new();
        uninstall(&d, &flags(false, true), &mut out).expect("forced uninstall");
        assert!(!d.data.exists(), "the data dir stayed: {}", String::from_utf8_lossy(&out));
        assert!(d.state.is_dir(), "the state dir went");
    }
}
