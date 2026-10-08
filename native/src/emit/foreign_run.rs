// spec: delegation-kit/SPEC.md §The foreign-vendor run — one read-only audit or mechanical sweep on
// a consumer's foreign adapter, in a scratch clone of committed `HEAD`: 0 OK, 1 REFUSED, 2 FAILED;
// a resumable adapter's `OK` opens a session `--foreign-resume` continues and closes.
use crate::hook::verdict::{self, KeyedConfig};
use crate::proc::{self, ChildEnv, Redirect};
use crate::programs::{self, Program};
use crate::walk;
use std::path::{Path, PathBuf};

pub const KNOBS: &[&str] = &[
    "DELEGATION_KIT_FOREIGN_ADAPTERS",
    "DELEGATION_KIT_FOREIGN_RESUME",
    "DELEGATION_KIT_FOREIGN_SESSION_MARKER",
    "DELEGATION_KIT_FOREIGN_TIMEOUT",
    "DELEGATION_KIT_FOREIGN_USAGE",
    "DELEGATION_KIT_FOREIGN_USAGE_CMD",
    "DELEGATION_KIT_FOREIGN_PAUSE_PCT",
    "DELEGATION_KIT_FOREIGN_PAUSE_PCT_LONG",
    "DELEGATION_KIT_STALE_AGE",
    "GATE_SDK_TMP_DIR",
];

const USAGE: &str = "usage: --foreign-run <adapter> <prompt-file> [--mode audit|sweep] [--key <key>] [--]\n       --foreign-run <adapter> --budget\n  runs one unit on the adapter DELEGATION_KIT_FOREIGN_ADAPTERS configures, in a scratch clone of committed HEAD; the key names the run's directory and defaults to the prompt file's stem. With --budget it prints the adapter's keyed budget verdict and spawns no adapter";

const RESUME_USAGE: &str = "usage: --foreign-resume <key> <prompt-file> [--]\n       --foreign-resume <key> --close\n  runs the next turn of the session a resumable adapter opened under the key, or ends it";

const PROMPT_TOKEN: &str = "@PROMPT_FILE@";

pub const SESSION_TOKEN: &str = "@SESSION_ID@";

// spec: delegation-kit/SPEC.md §The foreign-vendor run — a `GIT_*` location variable inherited from
// a hook context would point `git -C <clone>` and the adapter back at this repository
const GIT_LOCATION_VARS: &[&str] = &["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_OBJECT_DIRECTORY"];

#[derive(Debug, PartialEq)]
struct Args {
    adapter: String,
    prompt: String,
    sweep: bool,
    key: String,
}

// spec: delegation-kit/SPEC.md §The foreign-vendor run — the key is one path component, so the
// run's directory stays under the scratch dir's `foreign/`
fn one_component(arm: &str, key: &str) -> Result<(), String> {
    if key.is_empty() || key == "." || key == ".." || key.contains(['/', '\\']) {
        return Err(format!("{}: the key '{}' is not one path component", arm, key));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — a dash-led token naming no option is a refusal,
// and `--` ends option processing
#[derive(Debug, PartialEq)]
enum Form {
    Run(Args),
    Budget(String),
}

fn parse(args: &[String]) -> Result<Form, String> {
    let mut rest: Vec<&str> = Vec::new();
    let mut sweep = false;
    let mut mode = false;
    let mut budget = false;
    let mut key: Option<String> = None;
    let mut literal = false;
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if literal {
            rest.push(a);
        } else if a == "--" {
            literal = true;
        } else if a == "--mode" {
            match args.get(i + 1).map(String::as_str) {
                Some("audit") => sweep = false,
                Some("sweep") => sweep = true,
                other => return Err(format!("foreign-run: --mode takes audit or sweep (got {})", other.unwrap_or("nothing"))),
            }
            mode = true;
            i += 1;
        } else if a == "--budget" {
            budget = true;
        } else if a == "--key" {
            let k = args.get(i + 1).ok_or("foreign-run: --key needs a value")?;
            key = Some(k.clone());
            i += 1;
        } else if a.starts_with('-') {
            return Err(format!(
                "foreign-run: unrecognized option: {} — an operand beginning with \"-\" is passed after a \"--\" separator",
                a
            ));
        } else {
            rest.push(a);
        }
        i += 1;
    }
    // spec: delegation-kit/SPEC.md §The keyed verdict — the `--budget` form takes the adapter alone
    if budget {
        if mode || key.is_some() {
            return Err("foreign-run: --budget takes neither --mode nor --key".to_string());
        }
        let [adapter] = rest[..] else {
            return Err(format!("foreign-run: --budget takes an adapter alone (got {} operand(s))", rest.len()));
        };
        return Ok(Form::Budget(adapter.to_string()));
    }
    let [adapter, prompt] = rest[..] else {
        return Err(format!("foreign-run: takes an adapter and a prompt file (got {} operand(s))", rest.len()));
    };
    let key = key.unwrap_or_else(|| {
        Path::new(prompt).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default()
    });
    one_component("foreign-run", &key)?;
    Ok(Form::Run(Args { adapter: adapter.to_string(), prompt: prompt.to_string(), sweep, key }))
}

#[derive(Debug, PartialEq)]
struct ResumeArgs {
    key: String,
    prompt: Option<String>,
}

// spec: delegation-kit/SPEC.md §Resuming a session — a key and a prompt file, or a key and `--close`
fn parse_resume(args: &[String]) -> Result<ResumeArgs, String> {
    let mut rest: Vec<&str> = Vec::new();
    let mut close = false;
    let mut literal = false;
    for a in args {
        let a = a.as_str();
        if literal {
            rest.push(a);
        } else if a == "--" {
            literal = true;
        } else if a == "--close" {
            close = true;
        } else if a.starts_with('-') {
            return Err(format!(
                "foreign-resume: unrecognized option: {} — an operand beginning with \"-\" is passed after a \"--\" separator",
                a
            ));
        } else {
            rest.push(a);
        }
    }
    let (key, prompt) = match (close, &rest[..]) {
        (true, [key]) => (*key, None),
        (false, [key, prompt]) => (*key, Some(prompt.to_string())),
        (true, _) => return Err(format!("foreign-resume: --close takes a key alone (got {} operand(s))", rest.len())),
        (false, _) => return Err(format!("foreign-resume: takes a key and a prompt file (got {} operand(s))", rest.len())),
    };
    one_component("foreign-resume", key)?;
    Ok(ResumeArgs { key: key.to_string(), prompt })
}

// spec: delegation-kit/SPEC.md §Layout and configuration — an adapter's argv is its `<adapter>=<word>`
// elements' words, in element order
pub fn argv_of(adapters: &[String], name: &str) -> Vec<String> {
    adapters
        .iter()
        .filter_map(|e| e.split_once('='))
        .filter(|(a, _)| *a == name)
        .map(|(_, w)| w.to_string())
        .collect()
}

// spec: delegation-kit/SPEC.md §Layout and configuration — the id is the run of `[A-Za-z0-9._:-]`
// after the marker's first occurrence, past any blanks, in standard output, else standard error
fn session_id(marker: &str, streams: &[&Path]) -> Option<String> {
    streams.iter().find_map(|p| {
        let text = String::from_utf8_lossy(&std::fs::read(p).ok()?).into_owned();
        let at = text.find(marker)? + marker.len();
        let id: String = text[at..]
            .trim_start_matches([' ', '\t'])
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | ':' | '-'))
            .collect();
        (!id.is_empty()).then_some(id)
    })
}

pub struct Config {
    pub adapters: Vec<String>,
    pub resume: Vec<String>,
    pub markers: Vec<String>,
    pub timeout: u64,
    pub repo: String,
    pub base: String,
    pub here: String,
    pub budget: KeyedConfig,
}

struct Verdict {
    line: String,
    code: i32,
}

struct Run<'a> {
    args: &'a Args,
    dir: PathBuf,
    budget: &'static str,
    exit: String,
    report: Option<PathBuf>,
    patch: Option<PathBuf>,
}

fn show(p: &Option<PathBuf>) -> String {
    p.as_ref().map_or("none".to_string(), |p| p.display().to_string())
}

fn mode_name(sweep: bool) -> &'static str {
    if sweep {
        "sweep"
    } else {
        "audit"
    }
}

impl Run<'_> {
    fn line(&self, verdict: &str, code: i32) -> Verdict {
        Verdict {
            line: format!(
                "foreign-run: adapter={} mode={} key={} budget={} exit={} report={} patch={} -> {}",
                self.args.adapter,
                mode_name(self.args.sweep),
                self.args.key,
                self.budget,
                self.exit,
                show(&self.report),
                show(&self.patch),
                verdict
            ),
            code,
        }
    }

    fn failed(&self, why: &str) -> Verdict {
        self.line(&format!("FAILED ({})", why), 2)
    }
}

fn git_env() -> Vec<String> {
    GIT_LOCATION_VARS.iter().map(|s| s.to_string()).collect()
}

fn git(dir: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let unset = git_env();
    let mut argv: Vec<&str> = vec!["-C", dir.to_str().unwrap_or_default()];
    argv.extend_from_slice(args);
    let c = proc::run_with_stdin_in(&programs::GIT, &argv, b"", &ChildEnv { unset: &unset, ..ChildEnv::default() })?;
    match c.stdout() {
        Some(o) => Ok(o.to_vec()),
        None => Err(format!("git {} failed: {}", args.join(" "), c.failure_report().unwrap_or_default())),
    }
}

// spec: delegation-kit/SPEC.md §The foreign-vendor run — what "committed" compares: `HEAD` and every
// ref, so a commit on a branch the agent then left is caught too
fn refs(tree: &Path) -> Result<Vec<u8>, String> {
    let mut out = git(tree, &["rev-parse", "HEAD"])?;
    out.extend(git(tree, &["for-each-ref", "--format=%(refname) %(objectname)"])?);
    Ok(out)
}

// spec: delegation-kit/SPEC.md §The foreign-vendor run — the tree: a shared, no-checkout clone,
// detached at committed `HEAD`, with its `origin` removed so nothing in it can push back here
fn clone(repo: &str, tree: &Path) -> Result<String, String> {
    let head = git(Path::new(repo), &["rev-parse", "--verify", "HEAD^{commit}"])?;
    let head = String::from_utf8_lossy(&head).trim().to_string();
    let t = tree.to_str().ok_or("the clone path is not valid UTF-8")?;
    let unset = git_env();
    let c = proc::run_with_stdin_in(
        &programs::GIT,
        &["clone", "-q", "--shared", "--no-checkout", repo, t],
        b"",
        &ChildEnv { unset: &unset, ..ChildEnv::default() },
    )?;
    if c.stdout().is_none() {
        return Err(format!("git clone failed: {}", c.failure_report().unwrap_or_default()));
    }
    git(tree, &["checkout", "-q", "--detach", &head])?;
    git(tree, &["remote", "remove", "origin"])?;
    Ok(head)
}

// spec: delegation-kit/SPEC.md §The foreign-vendor run — a sweep's whole change, untracked files
// included, as a binary-safe patch against the session's base; an empty change is no patch
fn write_patch(tree: &Path, base: &str, patch: &Path) -> Result<Option<PathBuf>, String> {
    let _ = std::fs::remove_file(patch);
    git(tree, &["add", "-A"])?;
    let body = git(tree, &["diff", "--cached", "--binary", base])?;
    if body.is_empty() {
        return Ok(None);
    }
    std::fs::write(patch, body).map_err(|e| format!("cannot write {}: {}", patch.display(), e))?;
    Ok(Some(patch.to_path_buf()))
}

// spec: delegation-kit/SPEC.md §The foreign-vendor run — the shape, checked after every spawn that
// started: a moved ref is `committed`, and an audit's unclean status is `audit wrote`
fn shape(tree: &Path, before: &[u8], sweep: bool) -> Result<(), String> {
    match refs(tree) {
        Ok(after) if after != before => return Err("committed".to_string()),
        Ok(_) => {}
        Err(e) => return Err(format!("the clone's refs are unreadable: {}", e)),
    }
    if !sweep {
        match git(tree, &["status", "--porcelain", "--untracked-files=all"]) {
            Ok(s) if !s.is_empty() => return Err("audit wrote".to_string()),
            Ok(_) => {}
            Err(e) => return Err(format!("the clone's status is unreadable: {}", e)),
        }
    }
    Ok(())
}

struct Spawn<'a> {
    program: Program,
    words: Vec<String>,
    tree: &'a Path,
    prompt: &'a Path,
    report: &'a Path,
    stderr: &'a Path,
    timeout: u64,
}

fn spawn(s: &Spawn) -> Result<Option<i32>, String> {
    let args: Vec<&str> = s.words.iter().map(String::as_str).collect();
    let unset = git_env();
    let io = Redirect { cwd: s.tree, stdin: s.prompt, stdout: s.report, stderr: s.stderr, unset: &unset };
    proc::run_bounded_redirected(&s.program, &args, &io, s.timeout)
}

// spec: delegation-kit/SPEC.md §The foreign-vendor run — the turn's outcome once the shape held:
// a timeout or a non-zero exit fails
fn outcome(status: Option<i32>, timeout: u64) -> Result<(), String> {
    match status {
        None => Err(format!("timeout after {}s", timeout)),
        Some(c) if c != 0 => Err(format!("the adapter exited {}", c)),
        Some(_) => Ok(()),
    }
}

fn substitute(words: &[String], prompt: &str, id: Option<&str>) -> Vec<String> {
    words
        .iter()
        .map(|w| {
            let w = w.replace(PROMPT_TOKEN, prompt);
            match id {
                Some(id) => w.replace(SESSION_TOKEN, id),
                None => w,
            }
        })
        .collect()
}

fn resume_clause(key: &str) -> String {
    format!("OK — resumable: --foreign-resume {} <prompt-file>", key)
}

// spec: delegation-kit/SPEC.md §The foreign-vendor run — `session.txt`, the binding a resume reads:
// one line of `adapter= mode= base= turn= id=` fields, `-` for an id no marker produced
#[derive(Debug, PartialEq)]
struct Session {
    adapter: String,
    sweep: bool,
    base: String,
    turn: u32,
    id: Option<String>,
}

impl Session {
    fn render(&self) -> String {
        format!(
            "adapter={} mode={} base={} turn={} id={}\n",
            self.adapter,
            mode_name(self.sweep),
            self.base,
            self.turn,
            self.id.as_deref().unwrap_or("-")
        )
    }

    fn parse(text: &str) -> Result<Session, String> {
        let field = |name: &str| {
            text.split_whitespace()
                .find_map(|f| f.strip_prefix(name).and_then(|r| r.strip_prefix('=')))
                .filter(|v| !v.is_empty())
                .ok_or(format!("no {} field", name))
        };
        let sweep = match field("mode")? {
            "audit" => false,
            "sweep" => true,
            m => return Err(format!("mode '{}' is neither audit nor sweep", m)),
        };
        let turn = field("turn")?;
        Ok(Session {
            adapter: field("adapter")?.to_string(),
            sweep,
            base: field("base")?.to_string(),
            turn: turn.parse().map_err(|_| format!("turn '{}' is not a count", turn))?,
            id: Some(field("id")?).filter(|i| *i != "-").map(str::to_string),
        })
    }
}

// spec: delegation-kit/SPEC.md §Resuming a session — every turn's return survives a new open only
// as long as its session does: a new run under the key starts from no rotated report
fn clear_returns(dir: &Path) {
    for f in ["report.txt", "stderr.txt", "change.patch", "refs.txt"] {
        let _ = std::fs::remove_file(dir.join(f));
    }
    for (name, is_dir) in walk::list_dir(dir).unwrap_or_default() {
        let rotated = ["report.", "stderr."].iter().any(|p| {
            name.strip_prefix(p)
                .and_then(|r| r.strip_suffix(".txt"))
                .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
        });
        if rotated && !is_dir {
            let _ = std::fs::remove_file(dir.join(name));
        }
    }
}

// spec: delegation-kit/SPEC.md §The keyed verdict — the refusal a PAUSE becomes: the keyed line
// from its `used=` field on
fn paused(keyed: &verdict::Keyed) -> String {
    format!("budget: {}", keyed.line.find("used=").map_or(keyed.line.as_str(), |at| &keyed.line[at..]))
}

fn foreign_run(cfg: &Config, a: &Args) -> Verdict {
    let dir = PathBuf::from(&cfg.base).join("foreign").join(&a.key);
    let mut run = Run { args: a, dir, budget: "-", exit: "-".to_string(), report: None, patch: None };
    let argv = argv_of(&cfg.adapters, &a.adapter);
    let Some((program, words)) = argv.split_first() else {
        return run.failed(&format!("no adapter '{}' is configured in DELEGATION_KIT_FOREIGN_ADAPTERS", a.adapter));
    };
    let prompt = PathBuf::from(walk::abs_against(&cfg.here, &a.prompt));
    if std::fs::File::open(&prompt).is_err() || !prompt.is_file() {
        return run.failed(&format!("cannot read the prompt file {}", prompt.display()));
    }
    let tree = run.dir.join("tree");
    let session = run.dir.join("session.txt");
    // spec: delegation-kit/SPEC.md §The foreign-vendor run — an open session and a clone an earlier
    // refusal kept are each never overwritten
    if session.exists() {
        return run.failed(&format!(
            "an open session occupies {}; answer it with --foreign-resume {} <prompt-file>, or end it with --foreign-resume {} --close",
            run.dir.display(),
            a.key,
            a.key
        ));
    }
    if tree.exists() {
        return run.failed(&format!(
            "a kept clone occupies {}; inspect and remove it, or pass another --key",
            tree.display()
        ));
    }
    // spec: delegation-kit/SPEC.md §The keyed verdict — read after the pre-spawn checks and before
    // anything is written, so a paused run leaves no scratch and holds no key
    let keyed = verdict::keyed(&cfg.budget, &a.adapter);
    run.budget = keyed.word;
    if keyed.code == 1 {
        return run.failed(&paused(&keyed));
    }
    if let Err(e) = std::fs::create_dir_all(&run.dir) {
        return run.failed(&format!("cannot create {}: {}", run.dir.display(), e));
    }
    let report = run.dir.join("report.txt");
    let stderr = run.dir.join("stderr.txt");
    let patch = run.dir.join("change.patch");
    clear_returns(&run.dir);
    let base = match clone(&cfg.repo, &tree) {
        Ok(h) => h,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tree);
            return run.failed(&format!("clone: {}", e));
        }
    };
    let before = match refs(&tree) {
        Ok(r) => r,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tree);
            return run.failed(&format!("clone: {}", e));
        }
    };
    let prompt_abs = prompt.display().to_string();
    let s = Spawn {
        program: Program::consumer("DELEGATION_KIT_FOREIGN_ADAPTERS", program.replace(PROMPT_TOKEN, &prompt_abs)),
        words: substitute(words, &prompt_abs, None),
        tree: &tree,
        prompt: &prompt,
        report: &report,
        stderr: &stderr,
        timeout: cfg.timeout,
    };
    let status = match spawn(&s) {
        Ok(s) => s,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tree);
            return run.failed(&format!("spawn: {}", e));
        }
    };
    run.report = Some(report.clone());
    run.exit = status.map_or("timeout".to_string(), |c| c.to_string());
    // spec: delegation-kit/SPEC.md §The foreign-vendor run — a refusal outranks a failure: the kept
    // clone is the evidence either way
    if let Err(why) = shape(&tree, &before, a.sweep) {
        return run.line(&format!("REFUSED ({}) — clone kept at {}", why, tree.display()), 1);
    }
    let done = outcome(status, cfg.timeout).and_then(|()| {
        if a.sweep {
            run.patch = write_patch(&tree, &base, &patch).map_err(|e| format!("the sweep's change is unreadable: {}", e))?;
        }
        Ok(())
    });
    if let Err(why) = done {
        let _ = std::fs::remove_dir_all(&tree);
        return run.failed(&why);
    }
    let form = argv_of(&cfg.resume, &a.adapter);
    if form.is_empty() {
        let _ = std::fs::remove_dir_all(&tree);
        return run.line("OK", 0);
    }
    // spec: delegation-kit/SPEC.md §The foreign-vendor run — the cleanup, or the session: a resumable
    // adapter's `OK` keeps the clone and records the binding beside it
    let marker = argv_of(&cfg.markers, &a.adapter).into_iter().next();
    let id = marker.as_deref().and_then(|m| session_id(m, &[&report, &stderr]));
    let not_resumable = |why: String| {
        let _ = std::fs::remove_dir_all(&tree);
        run.line(&format!("OK — not resumable ({})", why), 0)
    };
    if id.is_none() && form.iter().any(|w| w.contains(SESSION_TOKEN)) {
        return not_resumable(format!(
            "no session id followed the marker '{}' in the turn's output",
            marker.unwrap_or_default()
        ));
    }
    let binding = Session { adapter: a.adapter.clone(), sweep: a.sweep, base, turn: 1, id };
    let recorded = std::fs::write(run.dir.join("refs.txt"), &before)
        .and_then(|()| std::fs::write(&session, binding.render()));
    if let Err(e) = recorded {
        let _ = std::fs::remove_file(&session);
        return not_resumable(format!("cannot record the session: {}", e));
    }
    run.line(&resume_clause(&a.key), 0)
}

struct Turn<'a> {
    key: &'a str,
    adapter: String,
    mode: &'static str,
    turn: String,
    budget: &'static str,
    exit: String,
    report: Option<PathBuf>,
    patch: Option<PathBuf>,
}

impl Turn<'_> {
    fn line(&self, verdict: &str, code: i32) -> Verdict {
        Verdict {
            line: format!(
                "foreign-resume: adapter={} mode={} key={} turn={} budget={} exit={} report={} patch={} -> {}",
                self.adapter,
                self.mode,
                self.key,
                self.turn,
                self.budget,
                self.exit,
                show(&self.report),
                show(&self.patch),
                verdict
            ),
            code,
        }
    }

    fn failed(&self, why: &str) -> Verdict {
        self.line(&format!("FAILED ({})", why), 2)
    }
}

// spec: delegation-kit/SPEC.md §Resuming a session — the next turn, in the kept clone and the same
// vendor session: `OK` and `FAILED` advance the turn and keep the session, `REFUSED` ends it
fn foreign_resume(cfg: &Config, key: &str, prompt: &str) -> Verdict {
    let dir = PathBuf::from(&cfg.base).join("foreign").join(key);
    let session_path = dir.join("session.txt");
    let mut t = Turn {
        key,
        adapter: "-".to_string(),
        mode: "-",
        turn: "-".to_string(),
        budget: "-",
        exit: "-".to_string(),
        report: None,
        patch: None,
    };
    let Ok(text) = std::fs::read_to_string(&session_path) else {
        return t.failed(&format!("no open session under {}", dir.display()));
    };
    let s = match Session::parse(&text) {
        Ok(s) => s,
        Err(e) => return t.failed(&format!("{} is unreadable: {}", session_path.display(), e)),
    };
    t.adapter = s.adapter.clone();
    t.mode = mode_name(s.sweep);
    t.turn = s.turn.to_string();
    let prompt = PathBuf::from(walk::abs_against(&cfg.here, prompt));
    if std::fs::File::open(&prompt).is_err() || !prompt.is_file() {
        return t.failed(&format!("cannot read the prompt file {}", prompt.display()));
    }
    if argv_of(&cfg.adapters, &s.adapter).is_empty() {
        return t.failed(&format!("no adapter '{}' is configured in DELEGATION_KIT_FOREIGN_ADAPTERS", s.adapter));
    }
    let form = argv_of(&cfg.resume, &s.adapter);
    let Some((program, words)) = form.split_first() else {
        return t.failed(&format!("adapter '{}' has no resume form in DELEGATION_KIT_FOREIGN_RESUME", s.adapter));
    };
    if s.id.is_none() && form.iter().any(|w| w.contains(SESSION_TOKEN)) {
        return t.failed(&format!("the session recorded no id, and adapter '{}' resumes by id", s.adapter));
    }
    let tree = dir.join("tree");
    if !tree.is_dir() {
        return t.failed(&format!("the session's clone {} is missing", tree.display()));
    }
    let before = match std::fs::read(dir.join("refs.txt")) {
        Ok(b) => b,
        Err(e) => return t.failed(&format!("cannot read {}: {}", dir.join("refs.txt").display(), e)),
    };
    // spec: delegation-kit/SPEC.md §Resuming a session — a paused turn precedes the rotation: the
    // counter holds, the previous report stays and the session stays open
    let keyed = verdict::keyed(&cfg.budget, &s.adapter);
    t.budget = keyed.word;
    if keyed.code == 1 {
        return t.failed(&paused(&keyed));
    }
    let report = dir.join("report.txt");
    let stderr = dir.join("stderr.txt");
    for (from, stem) in [(&report, "report"), (&stderr, "stderr")] {
        if from.exists() {
            if let Err(e) = std::fs::rename(from, dir.join(format!("{}.{}.txt", stem, s.turn))) {
                return t.failed(&format!("cannot rotate {}: {}", from.display(), e));
            }
        }
    }
    let next = Session { turn: s.turn + 1, ..s };
    t.turn = next.turn.to_string();
    let advance = |t: &Turn, why: &str| {
        if let Err(e) = std::fs::write(&session_path, next.render()) {
            return t.failed(&format!("{}; and cannot advance {}: {}", why, session_path.display(), e));
        }
        t.failed(why)
    };
    let prompt_abs = prompt.display().to_string();
    let sp = Spawn {
        program: Program::consumer(
            "DELEGATION_KIT_FOREIGN_RESUME",
            substitute(std::slice::from_ref(program), &prompt_abs, next.id.as_deref()).remove(0),
        ),
        words: substitute(words, &prompt_abs, next.id.as_deref()),
        tree: &tree,
        prompt: &prompt,
        report: &report,
        stderr: &stderr,
        timeout: cfg.timeout,
    };
    let status = match spawn(&sp) {
        Ok(st) => st,
        Err(e) => return advance(&t, &format!("spawn: {}", e)),
    };
    t.report = Some(report.clone());
    t.exit = status.map_or("timeout".to_string(), |c| c.to_string());
    if let Err(why) = shape(&tree, &before, next.sweep) {
        let _ = std::fs::remove_file(&session_path);
        return t.line(&format!("REFUSED ({}) — session ended, clone kept at {}", why, tree.display()), 1);
    }
    // spec: delegation-kit/SPEC.md §Resuming a session — every spawned sweep turn regenerates the
    // patch, a failed one included, so a session closed after it keeps that turn's change
    let patched = if next.sweep {
        write_patch(&tree, &next.base, &dir.join("change.patch"))
            .map(|p| t.patch = p)
            .map_err(|e| format!("the sweep's change is unreadable: {}", e))
    } else {
        Ok(())
    };
    if let Err(why) = outcome(status, cfg.timeout) {
        let why = match patched {
            Err(unread) => format!("{}; and {}", why, unread),
            Ok(()) => why,
        };
        return advance(&t, &why);
    }
    if let Err(why) = patched {
        return advance(&t, &why);
    }
    if let Err(e) = std::fs::write(&session_path, next.render()) {
        return t.failed(&format!("cannot advance {}: {}", session_path.display(), e));
    }
    t.line(&resume_clause(key), 0)
}

// spec: delegation-kit/SPEC.md §Resuming a session — `--close` removes the clone and the binding and
// keeps every report and the patch
fn foreign_close(cfg: &Config, key: &str) -> Verdict {
    let dir = PathBuf::from(&cfg.base).join("foreign").join(key);
    let failed = |why: String| Verdict { line: format!("foreign-resume: key={} -> FAILED ({})", key, why), code: 2 };
    if !dir.join("session.txt").is_file() {
        return failed(format!("no open session under {}", dir.display()));
    }
    let tree = dir.join("tree");
    if tree.exists() {
        if let Err(e) = std::fs::remove_dir_all(&tree) {
            return failed(format!("cannot remove {}: {}", tree.display(), e));
        }
    }
    for f in ["session.txt", "refs.txt"] {
        let p = dir.join(f);
        if p.exists() {
            if let Err(e) = std::fs::remove_file(&p) {
                return failed(format!("cannot remove {}: {}", p.display(), e));
            }
        }
    }
    Verdict { line: format!("foreign-resume: key={} -> CLOSED", key), code: 0 }
}

fn config() -> Result<Config, String> {
    let here = walk::cwd()?;
    let timeout = walk::knob_scalar("DELEGATION_KIT_FOREIGN_TIMEOUT")?;
    let timeout: u64 = timeout
        .parse()
        .map_err(|_| format!("DELEGATION_KIT_FOREIGN_TIMEOUT is not a positive integer: {}", timeout))?;
    let tmp = walk::knob_scalar("GATE_SDK_TMP_DIR")?;
    let base = walk::abs_against(&here, &tmp);
    walk::make_scratch(&base).map_err(|e| format!("cannot create the scratch dir {}: {}", base, e))?;
    let repo = walk::toplevel_opt()?.ok_or("not inside a git work tree")?;
    let budget = KeyedConfig {
        usage: walk::knob_array("DELEGATION_KIT_FOREIGN_USAGE")?,
        producers: walk::knob_array("DELEGATION_KIT_FOREIGN_USAGE_CMD")?,
        pause: walk::knob_scalar("DELEGATION_KIT_FOREIGN_PAUSE_PCT")?,
        pause_long: walk::knob_scalar("DELEGATION_KIT_FOREIGN_PAUSE_PCT_LONG")?,
        stale_age: walk::knob_scalar("DELEGATION_KIT_STALE_AGE")?,
        timeout,
        root: repo.clone(),
    };
    Ok(Config {
        adapters: walk::knob_array("DELEGATION_KIT_FOREIGN_ADAPTERS")?,
        resume: walk::knob_array("DELEGATION_KIT_FOREIGN_RESUME")?,
        markers: walk::knob_array("DELEGATION_KIT_FOREIGN_SESSION_MARKER")?,
        timeout,
        repo,
        base,
        here,
        budget,
    })
}

pub fn run(argv: &[String]) -> i32 {
    let a = match parse(argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("{}", USAGE);
            return 2;
        }
    };
    let a = match a {
        Form::Run(a) => a,
        Form::Budget(adapter) => return budget(&adapter),
    };
    let v = match config() {
        Ok(cfg) => foreign_run(&cfg, &a),
        Err(e) => Run { args: &a, dir: PathBuf::new(), budget: "-", exit: "-".to_string(), report: None, patch: None }
            .failed(&format!("config: {}", e)),
    };
    println!("{}", v.line);
    v.code
}

// spec: delegation-kit/SPEC.md §The keyed verdict — the `--budget` form: the adapter's keyed line
// on stdout at the verdict's own exit, no clone and no adapter; an adapter the knobs do not
// configure is a shape refusal
fn budget_form(cfg: &Config, adapter: &str) -> Result<verdict::Keyed, String> {
    if argv_of(&cfg.adapters, adapter).is_empty() {
        return Err(format!(
            "foreign-run: no adapter '{}' is configured in DELEGATION_KIT_FOREIGN_ADAPTERS",
            adapter
        ));
    }
    Ok(verdict::keyed(&cfg.budget, adapter))
}

fn budget(adapter: &str) -> i32 {
    match config().map_err(|e| format!("foreign-run: config: {}", e)).and_then(|cfg| budget_form(&cfg, adapter)) {
        Ok(k) => {
            println!("{}", k.line);
            k.code
        }
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("{}", USAGE);
            2
        }
    }
}

pub fn resume(argv: &[String]) -> i32 {
    let a = match parse_resume(argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("{}", RESUME_USAGE);
            return 2;
        }
    };
    let v = match (config(), &a.prompt) {
        (Ok(cfg), Some(p)) => foreign_resume(&cfg, &a.key, p),
        (Ok(cfg), None) => foreign_close(&cfg, &a.key),
        (Err(e), Some(_)) => {
            Verdict { line: format!("foreign-resume: key={} budget=- -> FAILED (config: {})", a.key, e), code: 2 }
        }
        (Err(e), None) => Verdict { line: format!("foreign-resume: key={} -> FAILED (config: {})", a.key, e), code: 2 },
    };
    println!("{}", v.line);
    v.code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // spec: delegation-kit/SPEC.md §Testing — a throwaway source repository with one tracked config-
    // format file, so `git config --file` is a stub adapter that writes
    struct Repo {
        root: PathBuf,
    }

    impl Repo {
        fn new(tag: &str) -> Repo {
            let root = std::env::temp_dir().join(format!("checkwright-foreign-run.{}.{}", tag, std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let src = root.join("src");
            std::fs::create_dir_all(&src).expect("the sandbox must be creatable");
            std::fs::write(src.join("tracked.cfg"), "[a]\n\tb = 1\n").expect("the tracked file");
            std::fs::write(root.join("prompt.md"), "audit the tree\n").expect("the prompt file");
            std::fs::write(root.join("answer.md"), "the answer\n").expect("the answer file");
            for args in [
                &["init", "-q"][..],
                &["add", "tracked.cfg"],
                &["-c", "user.name=t", "-c", "user.email=t@example.invalid", "commit", "-q", "-m", "seed"],
            ] {
                git(&src, args).expect("the source repository must build");
            }
            Repo { root }
        }

        fn cfg(&self, adapters: &[&str], timeout: u64) -> Config {
            self.session_cfg(adapters, &[], &[], timeout)
        }

        fn session_cfg(&self, adapters: &[&str], resume: &[&str], markers: &[&str], timeout: u64) -> Config {
            Config {
                adapters: strings(adapters),
                resume: strings(resume),
                markers: strings(markers),
                timeout,
                repo: self.root.join("src").display().to_string(),
                base: self.root.join("tmp").display().to_string(),
                here: self.root.display().to_string(),
                budget: KeyedConfig {
                    usage: Vec::new(),
                    producers: Vec::new(),
                    pause: "80".to_string(),
                    pause_long: "95".to_string(),
                    stale_age: "600".to_string(),
                    timeout,
                    root: self.root.join("src").display().to_string(),
                },
            }
        }

        // spec: delegation-kit/SPEC.md §Testing — a snapshot outside the source repository, bound to
        // the adapter: `pct` used, read `age` seconds ago, in a window with an hour left
        fn budgeted(&self, mut cfg: Config, adapter: &str, pct: &str, age: i64) -> Config {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            let path = self.root.join("usage.txt");
            let body = format!(
                "five_hour_used_pct={}\nfive_hour_resets_at={}\nupdated_at={}\n",
                pct,
                now + 3600,
                now - age
            );
            std::fs::write(&path, body).expect("the snapshot");
            cfg.budget.usage = vec![format!("{}={}", adapter, path.display())];
            cfg
        }

        fn args(&self, adapter: &str, sweep: bool) -> Args {
            Args {
                adapter: adapter.to_string(),
                prompt: "prompt.md".to_string(),
                sweep,
                key: "prompt".to_string(),
            }
        }

        fn run_dir(&self) -> PathBuf {
            self.root.join("tmp").join("foreign").join("prompt")
        }

        fn tree_kept(&self) -> bool {
            self.run_dir().join("tree").exists()
        }

        fn read(&self, name: &str) -> String {
            String::from_utf8_lossy(&std::fs::read(self.run_dir().join(name)).unwrap_or_default()).into_owned()
        }

        fn head(&self) -> String {
            String::from_utf8_lossy(&git(&self.root.join("src"), &["rev-parse", "HEAD"]).expect("HEAD")).trim().to_string()
        }
    }

    impl Drop for Repo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn hash_of(repo: &Repo, body: &[u8]) -> String {
        let src = repo.root.join("src");
        let c = proc::run_with_stdin_in(
            &programs::GIT,
            &["-C", src.to_str().unwrap_or_default(), "hash-object", "--stdin"],
            body,
            &ChildEnv::default(),
        )
        .expect("git hash-object");
        String::from_utf8_lossy(c.stdout().expect("a hash")).into_owned()
    }

    // spec: delegation-kit/SPEC.md §Testing — OK, audit: the adapter prints and writes nothing, its
    // output is the report, stdin carries the prompt, and the clone is removed
    #[test]
    fn an_audit_that_writes_nothing_returns_its_output_as_the_report() {
        let r = Repo::new("audit-ok");
        let v = foreign_run(&r.cfg(&["ro=git", "ro=hash-object", "ro=--stdin"], 60), &r.args("ro", false));
        assert_eq!(v.code, 0, "{}", v.line);
        assert!(v.line.contains("mode=audit key=prompt budget=OFF exit=0 report="), "{}", v.line);
        assert!(v.line.ends_with("patch=none -> OK"), "{}", v.line);
        let report = std::fs::read(r.run_dir().join("report.txt")).expect("the report");
        assert_eq!(String::from_utf8_lossy(&report), hash_of(&r, b"audit the tree\n"), "stdin carries the prompt");
        assert!(!r.tree_kept(), "the clone is removed on OK");
        assert!(!r.run_dir().join("session.txt").exists(), "a one-shot adapter opens no session");
    }

    // spec: delegation-kit/SPEC.md §Testing — a stage's emitted contract rides an audit-mode run as
    // any prompt file does: the adapter reads its bytes whole and the clone stays clean
    #[test]
    fn a_stage_contract_document_runs_as_an_audit_prompt() {
        for stage in admitted_stages() {
            let r = Repo::new(&format!("stage-contract-{}", stage));
            let doc = crate::emit::stage_contract::compose(
                "You write nothing.\n",
                &stage,
                &format!("Execute the template at kit/templates/stages/{}.md, applying the bindings below.\n", stage),
            );
            std::fs::write(r.root.join("prompt.md"), &doc).expect("the prompt file");
            let v = foreign_run(&r.cfg(RO, 60), &r.args("ro", false));
            assert_eq!(v.code, 0, "{}: {}", stage, v.line);
            assert!(v.line.contains("mode=audit key=prompt") && v.line.ends_with("patch=none -> OK"), "{}", v.line);
            let report = std::fs::read(r.run_dir().join("report.txt")).expect("the report");
            assert_eq!(String::from_utf8_lossy(&report), hash_of(&r, doc.as_bytes()), "{}", stage);
        }
    }

    // spec: lifecycle-kit/SPEC.md §The host protocol — the admitted set is read off the shipped
    // stage templates' pointer lines, and the build stage's alone carries none
    fn admitted_stages() -> Vec<String> {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../lifecycle-kit/templates/stages");
        let mut all: Vec<String> = Vec::new();
        let mut admitted: Vec<String> = Vec::new();
        for p in walk::glob_files(&dir, &["*.md".to_string()]).expect("the shipped stage templates") {
            let Some(stage) = p.file_stem().and_then(|s| s.to_str()).map(str::to_string) else { continue };
            let text = std::fs::read_to_string(&p).expect("a stage template");
            if text.contains("lifecycle-kit/templates/host-protocol.md") {
                admitted.push(stage.clone());
            }
            all.push(stage);
        }
        all.retain(|s| !admitted.contains(s));
        assert_eq!(all, vec!["build".to_string()], "the stage templates carrying no pointer line");
        assert!(admitted.len() > 1, "{:?}", admitted);
        admitted.sort();
        admitted
    }

    // spec: delegation-kit/SPEC.md §Testing — `@PROMPT_FILE@` is substituted with the prompt's
    // absolute path
    #[test]
    fn the_prompt_token_names_the_prompt_file() {
        let r = Repo::new("token");
        let v = foreign_run(&r.cfg(&["ro=git", "ro=hash-object", "ro=@PROMPT_FILE@"], 60), &r.args("ro", false));
        assert_eq!(v.code, 0, "{}", v.line);
        let report = std::fs::read(r.run_dir().join("report.txt")).expect("the report");
        assert_eq!(String::from_utf8_lossy(&report), hash_of(&r, b"audit the tree\n"));
    }

    // spec: delegation-kit/SPEC.md §Testing — OK, sweep: a tracked and an untracked write yield a
    // patch that applies cleanly to the source repository
    #[test]
    fn a_sweep_yields_a_patch_that_applies_to_the_source() {
        let r = Repo::new("sweep-ok");
        let script = "git config --file tracked.cfg a.b 2 && git config --file new.cfg c.d 3";
        let v = foreign_run(&r.cfg(&["sw=bash", "sw=-c", &format!("sw={}", script)], 60), &r.args("sw", true));
        assert_eq!(v.code, 0, "{}", v.line);
        let patch = r.run_dir().join("change.patch");
        assert!(v.line.contains(&format!("patch={} -> OK", patch.display())), "{}", v.line);
        let src = r.root.join("src");
        git(&src, &["apply", "--check", patch.to_str().unwrap_or_default()]).expect("the patch applies cleanly");
        let body = String::from_utf8_lossy(&std::fs::read(&patch).expect("the patch")).into_owned();
        assert!(body.contains("tracked.cfg") && body.contains("new.cfg"), "{}", body);
        assert!(!r.tree_kept(), "the clone is removed on OK");
        let quiet = foreign_run(&r.cfg(&["sw=git", "sw=status"], 60), &r.args("sw", true));
        assert!(quiet.line.ends_with("patch=none -> OK"), "an empty change is no patch: {}", quiet.line);
        assert!(!patch.exists(), "an earlier run's patch is not reported as this run's");
    }

    // spec: delegation-kit/SPEC.md §Testing — REFUSED: a commit in either mode, and an audit that
    // wrote; the clone is kept on each and a kept clone blocks the next run under its key
    #[test]
    fn a_committing_or_writing_adapter_is_refused_and_its_clone_kept() {
        let commit = ["c=git", "c=-c", "c=user.name=t", "c=-c", "c=user.email=t@example.invalid", "c=commit", "c=--allow-empty", "c=-q", "c=-m", "c=x"];
        for sweep in [false, true] {
            let r = Repo::new(if sweep { "commit-sweep" } else { "commit-audit" });
            let v = foreign_run(&r.cfg(&commit, 60), &r.args("c", sweep));
            assert_eq!(v.code, 1, "{}", v.line);
            assert!(v.line.contains("-> REFUSED (committed) — clone kept at "), "{}", v.line);
            assert!(r.tree_kept());
            let again = foreign_run(&r.cfg(&commit, 60), &r.args("c", sweep));
            assert_eq!(again.code, 2, "{}", again.line);
            assert!(again.line.contains("a kept clone occupies"), "{}", again.line);
        }
        let r = Repo::new("audit-wrote");
        let v = foreign_run(&r.cfg(&["w=git", "w=config", "w=--file", "w=scratch.cfg", "w=a.b", "w=c"], 60), &r.args("w", false));
        assert_eq!(v.code, 1, "{}", v.line);
        assert!(v.line.contains("-> REFUSED (audit wrote)"), "{}", v.line);
        assert!(r.tree_kept());
    }

    // spec: delegation-kit/SPEC.md §Testing — FAILED: an unknown adapter, an unreadable prompt file,
    // a non-zero adapter exit with its report kept, and a timeout
    #[test]
    fn what_cannot_run_or_does_not_finish_fails() {
        let r = Repo::new("failed");
        let v = foreign_run(&r.cfg(&["ro=git"], 60), &r.args("nope", false));
        assert_eq!(v.code, 2, "{}", v.line);
        assert!(v.line.contains("budget=- exit=- report=none patch=none -> FAILED (no adapter 'nope'"), "{}", v.line);
        let mut missing = r.args("ro", false);
        missing.prompt = "absent.md".to_string();
        let v = foreign_run(&r.cfg(&["ro=git"], 60), &missing);
        assert!(v.code == 2 && v.line.contains("budget=- ") && v.line.contains("cannot read the prompt file"), "{}", v.line);
        let v = foreign_run(&r.cfg(&["bad=git", "bad=rev-parse", "bad=--verify", "bad=no-such-ref"], 60), &r.args("bad", false));
        assert_eq!(v.code, 2, "{}", v.line);
        assert!(v.line.contains("exit=128 report=") && v.line.contains("FAILED (the adapter exited 128)"), "{}", v.line);
        assert!(r.run_dir().join("stderr.txt").is_file() && r.run_dir().join("report.txt").is_file(), "the report is kept");
        assert!(!r.tree_kept(), "the clone is removed on FAILED");
        let v = foreign_run(&r.cfg(&["slow=bash", "slow=-c", "slow=exec sleep 5"], 1), &r.args("slow", false));
        assert_eq!(v.code, 2, "{}", v.line);
        assert!(v.line.contains("exit=timeout") && v.line.contains("FAILED (timeout after 1s)"), "{}", v.line);
    }

    // spec: delegation-kit/SPEC.md §The foreign-vendor run — the clone cannot push back: its
    // `origin` is removed
    #[test]
    fn the_clone_carries_no_remote() {
        let r = Repo::new("remote");
        let v = foreign_run(&r.cfg(&["rm=git", "rm=remote"], 60), &r.args("rm", false));
        assert_eq!(v.code, 0, "{}", v.line);
        assert!(std::fs::read(r.run_dir().join("report.txt")).expect("the report").is_empty(), "no remote listed");
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the argv shape: two operands, the two options,
    // the key's default and its one-component rule, and the `--` escape
    #[test]
    fn the_argv_shape_is_held() {
        let run = |argv: &[&str], why: &str| match parse(&strings(argv)).expect(why) {
            Form::Run(a) => a,
            Form::Budget(_) => panic!("{:?} is no budget form", argv),
        };
        let a = run(&["ad", "dir/unit.prompt.md"], "two operands");
        assert_eq!(a, Args { adapter: "ad".into(), prompt: "dir/unit.prompt.md".into(), sweep: false, key: "unit.prompt".into() });
        let a = run(&["--mode", "sweep", "--key", "k1", "ad", "p.md"], "options first");
        assert!(a.sweep && a.key == "k1");
        let a = run(&["ad", "--", "-p.md"], "the escape");
        assert_eq!(a.prompt, "-p.md");
        for bad in [&["ad"][..], &["ad", "p", "q"], &["--help"], &["--mode", "write", "ad", "p"], &["--key", "a/b", "ad", "p"], &["--key", "..", "ad", "p"]] {
            assert!(parse(&strings(bad)).is_err(), "{:?} must refuse", bad);
        }
    }

    // spec: delegation-kit/SPEC.md §The keyed verdict — the `--budget` form's argv: the adapter
    // alone, in either order, and a refusal beside a second operand, `--mode` or `--key`
    #[test]
    fn the_budget_form_takes_the_adapter_alone() {
        assert_eq!(parse(&strings(&["ad", "--budget"])), Ok(Form::Budget("ad".into())));
        assert_eq!(parse(&strings(&["--budget", "ad"])), Ok(Form::Budget("ad".into())));
        for bad in [&["--budget"][..], &["ad", "p.md", "--budget"], &["--budget", "--mode", "audit", "ad"], &["--budget", "--key", "k", "ad"]] {
            assert!(parse(&strings(bad)).is_err(), "{:?} must refuse", bad);
        }
    }

    const RO: &[&str] = &["ro=git", "ro=hash-object", "ro=--stdin"];

    // spec: delegation-kit/SPEC.md §Testing — a PAUSE spawns nothing and leaves no scratch; an
    // at-or-over reading past the stale age still pauses, beside an under-threshold one that reads
    // STALE and proceeds
    #[test]
    fn a_paused_window_spawns_nothing_and_an_unknown_budget_proceeds() {
        let r = Repo::new("budget-pause");
        for age in [0, 6000] {
            let v = foreign_run(&r.budgeted(r.cfg(RO, 60), "ro", "90", age), &r.args("ro", false));
            assert_eq!(v.code, 2, "{}", v.line);
            assert!(v.line.contains("key=prompt budget=PAUSE exit=- report=none patch=none -> FAILED (budget: used=90% age="), "{}", v.line);
            assert!(v.line.contains("-> PAUSE (short window; at or over 80% of a window that resets in "), "{}", v.line);
            assert!(!r.run_dir().exists(), "a paused run leaves no scratch and holds no key");
        }
        let v = foreign_run(&r.budgeted(r.cfg(RO, 60), "ro", "10", 6000), &r.args("ro", false));
        assert_eq!(v.code, 0, "{}", v.line);
        assert!(v.line.contains("key=prompt budget=STALE exit=0 report=") && v.line.ends_with("-> OK"), "{}", v.line);
        let v = foreign_run(&r.budgeted(r.cfg(RO, 60), "ro", "10", 0), &r.args("ro", false));
        assert!(v.code == 0 && v.line.contains("budget=OK exit=0"), "{}", v.line);
        let v = foreign_run(&r.budgeted(r.cfg(RO, 60), "other", "90", 0), &r.args("ro", false));
        assert!(v.code == 0 && v.line.contains("budget=OFF exit=0"), "an adapter with no snapshot is unbudgeted: {}", v.line);
    }

    // spec: delegation-kit/SPEC.md §Testing — a dead short window beside a live at-or-over long one
    // pauses naming the long window and spawns nothing; beside an under-threshold, absent or reset
    // long window it is RESET-OK and the run proceeds
    #[test]
    fn a_dead_short_window_pauses_only_behind_a_live_exhausted_long_one() {
        let r = Repo::new("budget-long-first");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let dead_short = |long: Option<(&str, i64)>| {
            let cfg = r.budgeted(r.cfg(RO, 60), "ro", "90", 0);
            let mut body = format!("five_hour_used_pct=90\nfive_hour_resets_at={}\nupdated_at={}\n", now - 100, now - 200);
            if let Some((pct, resets)) = long {
                body.push_str(&format!("seven_day_used_pct={}\nseven_day_resets_at={}\n", pct, resets));
            }
            std::fs::write(r.root.join("usage.txt"), body).expect("the snapshot");
            cfg
        };
        let v = foreign_run(&dead_short(Some(("95", now + 9000))), &r.args("ro", false));
        assert_eq!(v.code, 2, "{}", v.line);
        assert!(v.line.contains("budget=PAUSE exit=- report=none"), "{}", v.line);
        assert!(v.line.contains("-> PAUSE (long window; at or over 95% of a window that resets in "), "{}", v.line);
        assert!(v.line.contains("the short window rolled over "), "{}", v.line);
        assert!(!r.run_dir().exists(), "a paused run leaves no scratch and holds no key");
        for long in [Some(("94", now + 9000)), None, Some(("99", now - 1))] {
            let v = foreign_run(&dead_short(long), &r.args("ro", false));
            assert!(v.code == 0 && v.line.contains("budget=RESET-OK exit=0"), "{:?}: {}", long, v.line);
        }
    }

    // spec: delegation-kit/SPEC.md §Testing — the producer runs before the read with the snapshot
    // path substituted, and one that fails or outlives its bound leaves the snapshot read as it was
    #[test]
    fn the_producer_feeds_the_read_and_its_failure_leaves_the_snapshot() {
        let r = Repo::new("budget-producer");
        let mut cfg = r.budgeted(r.cfg(RO, 60), "ro", "10", 0);
        let write = "printf 'five_hour_used_pct=91\\nfive_hour_resets_at=99999999999\\nupdated_at=1\\n' > \"$0\"";
        cfg.budget.producers = strings(&["ro=bash", "ro=-c", &format!("ro={}", write), "ro=@USAGE_FILE@"]);
        let v = foreign_run(&cfg, &r.args("ro", false));
        assert!(v.code == 2 && v.line.contains("budget=PAUSE") && v.line.contains("used=91%"), "{}", v.line);
        for (fail, timeout) in [("exit 3", 60), ("exec sleep 5", 1)] {
            let mut cfg = r.budgeted(r.cfg(RO, timeout), "ro", "10", 0);
            cfg.budget.producers = strings(&["ro=bash", "ro=-c", &format!("ro={}", fail)]);
            let v = foreign_run(&cfg, &r.args("ro", false));
            assert!(v.code == 0 && v.line.contains("budget=OK exit=0"), "{}: {}", fail, v.line);
        }
        let mut cfg = r.budgeted(r.cfg(RO, 60), "ro", "10", 0);
        cfg.budget.producers = strings(&["ro=no-such-program-on-any-path"]);
        let v = foreign_run(&cfg, &r.args("ro", false));
        assert!(v.code == 0 && v.line.contains("budget=OK exit=0"), "a producer that cannot spawn: {}", v.line);
    }

    // spec: delegation-kit/SPEC.md §Testing — a paused resume: the turn counter holds, the previous
    // report is not rotated and the session stays open
    #[test]
    fn a_paused_resume_holds_its_turn() {
        let r = Repo::new("budget-resume");
        let open = r.session_cfg(OPEN_STDOUT, RESUME_ECHO, MARKER, 60);
        assert_eq!(foreign_run(&open, &r.args("s", false)).code, 0);
        let paused = r.budgeted(r.session_cfg(OPEN_STDOUT, RESUME_ECHO, MARKER, 60), "s", "90", 0);
        let v = foreign_resume(&paused, "prompt", "answer.md");
        assert_eq!(v.code, 2, "{}", v.line);
        assert!(v.line.starts_with("foreign-resume: adapter=s mode=audit key=prompt turn=1 budget=PAUSE exit=- report=none patch=none -> FAILED (budget: used=90% "), "{}", v.line);
        assert!(r.read("session.txt").contains(" turn=1 "), "{}", r.read("session.txt"));
        assert!(r.run_dir().join("report.txt").is_file() && !r.run_dir().join("report.1.txt").exists(), "nothing is rotated");
        let v = foreign_resume(&r.budgeted(r.session_cfg(OPEN_STDOUT, RESUME_ECHO, MARKER, 60), "s", "10", 0), "prompt", "answer.md");
        assert!(v.code == 0 && v.line.contains("turn=2 budget=OK exit=0"), "the window reset: {}", v.line);
    }

    // spec: delegation-kit/SPEC.md §Testing — the `--budget` form: the keyed line and its three
    // exits, no clone made, and an unconfigured adapter refused
    #[test]
    fn the_budget_form_prints_the_keyed_line_at_its_exit() {
        let r = Repo::new("budget-form");
        let form = |cfg: &Config, adapter: &str| budget_form(cfg, adapter).expect("a configured adapter");
        let k = form(&r.budgeted(r.cfg(RO, 60), "ro", "10", 0), "ro");
        assert_eq!((k.code, k.word), (0, "OK"), "{}", k.line);
        assert!(k.line.starts_with("foreign-budget: adapter=ro used=10% age="), "{}", k.line);
        assert!(k.line.contains(" resets_in=") && k.line.contains("s -> OK (reading within 600s and under its pause thresholds; "), "{}", k.line);
        let k = form(&r.budgeted(r.cfg(RO, 60), "ro", "80", 0), "ro");
        assert_eq!((k.code, k.word), (1, "PAUSE"), "at-or-over pauses: {}", k.line);
        let k = form(&r.budgeted(r.cfg(RO, 60), "ro", "10", 6000), "ro");
        assert_eq!((k.code, k.word), (2, "STALE"), "{}", k.line);
        assert!(k.line.contains("never blocks a foreign run"), "{}", k.line);
        let k = form(&r.cfg(RO, 60), "ro");
        assert_eq!((k.code, k.word), (2, "OFF"), "{}", k.line);
        assert!(k.line.starts_with("foreign-budget: adapter=ro -> OFF (no DELEGATION_KIT_FOREIGN_USAGE snapshot"), "{}", k.line);
        assert!(budget_form(&r.cfg(RO, 60), "nope").is_err(), "an unconfigured adapter is a shape refusal");
        assert!(!r.root.join("tmp").join("foreign").exists(), "the form makes no clone");
    }

    // spec: delegation-kit/SPEC.md §Layout and configuration — an adapter's argv is its elements'
    // words in element order, and another adapter's words never join it
    #[test]
    fn an_adapters_argv_is_its_own_words_in_order() {
        let table = strings(&["a=prog", "b=other", "a=--flag", "a=x=y"]);
        assert_eq!(argv_of(&table, "a"), strings(&["prog", "--flag", "x=y"]));
        assert!(argv_of(&table, "c").is_empty());
    }

    const OPEN_STDOUT: &[&str] = &["s=bash", "s=-c", "s=echo 'session id: abc-1.x'"];
    const OPEN_STDERR: &[&str] = &["s=bash", "s=-c", "s=echo 'session id:\terr:2' >&2"];
    const RESUME_ECHO: &[&str] = &["s=bash", "s=-c", "s=printf '%s %s\\n' \"$0\" \"$1\"; cat", "s=@SESSION_ID@", "s=@PROMPT_FILE@"];
    const MARKER: &[&str] = &["s=session id:"];

    // spec: delegation-kit/SPEC.md §Testing — open: a resumable adapter's `OK` keeps the clone and
    // writes `session.txt` and `refs.txt`, the id read from standard output or else standard error
    #[test]
    fn a_resumable_open_keeps_the_clone_and_records_the_session() {
        for (tag, open, id) in [("open-out", OPEN_STDOUT, "abc-1.x"), ("open-err", OPEN_STDERR, "err:2")] {
            let r = Repo::new(tag);
            let v = foreign_run(&r.session_cfg(open, RESUME_ECHO, MARKER, 60), &r.args("s", false));
            assert_eq!(v.code, 0, "{}", v.line);
            assert!(v.line.ends_with("-> OK — resumable: --foreign-resume prompt <prompt-file>"), "{}", v.line);
            assert!(r.tree_kept(), "an open session keeps its clone");
            assert_eq!(r.read("session.txt"), format!("adapter=s mode=audit base={} turn=1 id={}\n", r.head(), id));
            assert!(r.read("refs.txt").starts_with(&r.head()), "{}", r.read("refs.txt"));
            let again = foreign_run(&r.session_cfg(open, RESUME_ECHO, MARKER, 60), &r.args("s", false));
            assert_eq!(again.code, 2, "{}", again.line);
            assert!(again.line.contains("an open session occupies") && again.line.contains("--foreign-resume prompt --close"), "{}", again.line);
        }
    }

    // spec: delegation-kit/SPEC.md §Testing — open: a form resuming by id with no id captured opens no
    // session and removes the clone; a form resuming without an id opens one recording `-`
    #[test]
    fn an_open_without_its_needed_id_opens_no_session() {
        let r = Repo::new("no-id");
        let quiet = ["s=git", "s=status"];
        let v = foreign_run(&r.session_cfg(&quiet, RESUME_ECHO, MARKER, 60), &r.args("s", false));
        assert_eq!(v.code, 0, "{}", v.line);
        assert!(v.line.contains("-> OK — not resumable (no session id followed the marker 'session id:'"), "{}", v.line);
        assert!(!r.tree_kept() && !r.run_dir().join("session.txt").exists());
        let last = ["s=bash", "s=-c", "s=cat"];
        let v = foreign_run(&r.session_cfg(&quiet, &last, &[], 60), &r.args("s", false));
        assert!(v.line.ends_with("resumable: --foreign-resume prompt <prompt-file>"), "{}", v.line);
        assert!(r.read("session.txt").ends_with("turn=1 id=-\n"), "{}", r.read("session.txt"));
    }

    // spec: delegation-kit/SPEC.md §Testing — resume: both tokens substituted, the prompt on standard
    // input, the previous report rotated, the turn advanced; close keeps every report
    #[test]
    fn a_resume_continues_the_session_and_close_keeps_its_reports() {
        let r = Repo::new("resume");
        let cfg = r.session_cfg(OPEN_STDOUT, RESUME_ECHO, MARKER, 60);
        assert_eq!(foreign_run(&cfg, &r.args("s", false)).code, 0);
        let v = foreign_resume(&cfg, "prompt", "answer.md");
        assert_eq!(v.code, 0, "{}", v.line);
        assert!(v.line.starts_with("foreign-resume: adapter=s mode=audit key=prompt turn=2 budget=OFF exit=0 report="), "{}", v.line);
        assert!(v.line.ends_with("patch=none -> OK — resumable: --foreign-resume prompt <prompt-file>"), "{}", v.line);
        // comment-tier-exempt: a Windows bash stub prints the substituted path with forward slashes
        let answer = r.root.join("answer.md").display().to_string().replace('\\', "/");
        assert_eq!(r.read("report.txt").replace('\\', "/"), format!("abc-1.x {}\nthe answer\n", answer));
        assert_eq!(r.read("report.1.txt"), "session id: abc-1.x\n", "the open's report is rotated");
        assert!(r.run_dir().join("stderr.1.txt").is_file());
        assert!(r.read("session.txt").contains(" turn=2 id=abc-1.x"), "{}", r.read("session.txt"));
        let closed = foreign_close(&cfg, "prompt");
        assert_eq!((closed.code, closed.line.as_str()), (0, "foreign-resume: key=prompt -> CLOSED"));
        assert!(!r.tree_kept() && !r.run_dir().join("session.txt").exists() && !r.run_dir().join("refs.txt").exists());
        assert!(r.run_dir().join("report.txt").is_file() && r.run_dir().join("report.1.txt").is_file(), "close keeps every report");
        let again = foreign_close(&cfg, "prompt");
        assert!(again.code == 2 && again.line.contains("-> FAILED (no open session under"), "{}", again.line);
        let reopened = foreign_run(&cfg, &r.args("s", false));
        assert_eq!(reopened.code, 0, "{}", reopened.line);
        assert!(!r.run_dir().join("report.1.txt").exists(), "a new open starts from no rotated report");
    }

    // spec: delegation-kit/SPEC.md §Testing — resume: a sweep's patch covers the session's whole
    // change across two turns
    #[test]
    fn a_sweep_sessions_patch_is_its_whole_change() {
        let r = Repo::new("resume-sweep");
        let open = ["s=bash", "s=-c", "s=git config --file tracked.cfg a.b 2 && echo 'session id: w1'"];
        let resume = ["s=bash", "s=-c", "s=git config --file new.cfg c.d 3", "s=@SESSION_ID@"];
        let cfg = r.session_cfg(&open, &resume, MARKER, 60);
        assert_eq!(foreign_run(&cfg, &r.args("s", true)).code, 0);
        let v = foreign_resume(&cfg, "prompt", "answer.md");
        assert_eq!(v.code, 0, "{}", v.line);
        let patch = r.run_dir().join("change.patch");
        assert!(v.line.contains(&format!("mode=sweep key=prompt turn=2 budget=OFF exit=0 report={} patch={} -> OK", r.run_dir().join("report.txt").display(), patch.display())), "{}", v.line);
        let body = String::from_utf8_lossy(&std::fs::read(&patch).expect("the patch")).into_owned();
        assert!(body.contains("tracked.cfg") && body.contains("new.cfg"), "{}", body);
        git(&r.root.join("src"), &["apply", "--check", patch.to_str().unwrap_or_default()]).expect("the patch applies cleanly");
    }

    // spec: delegation-kit/SPEC.md §Testing — resume: a failed sweep turn still regenerates the patch,
    // so the session closed after it keeps that turn's change
    #[test]
    fn a_failed_sweep_turn_still_regenerates_the_patch() {
        let r = Repo::new("resume-sweep-failed");
        let open = ["s=bash", "s=-c", "s=git config --file tracked.cfg a.b 2 && echo 'session id: w1'"];
        let resume = ["s=bash", "s=-c", "s=git config --file new.cfg c.d 3; exit 3", "s=@SESSION_ID@"];
        let cfg = r.session_cfg(&open, &resume, MARKER, 60);
        assert_eq!(foreign_run(&cfg, &r.args("s", true)).code, 0);
        let v = foreign_resume(&cfg, "prompt", "answer.md");
        assert_eq!(v.code, 2, "{}", v.line);
        let patch = r.run_dir().join("change.patch");
        assert!(v.line.contains(&format!("turn=2 budget=OFF exit=3 report={} patch={} -> FAILED (the adapter exited 3)", r.run_dir().join("report.txt").display(), patch.display())), "{}", v.line);
        assert_eq!(foreign_close(&cfg, "prompt").code, 0);
        let body = String::from_utf8_lossy(&std::fs::read(&patch).expect("close keeps the patch")).into_owned();
        assert!(body.contains("tracked.cfg") && body.contains("new.cfg"), "{}", body);
        git(&r.root.join("src"), &["apply", "--check", patch.to_str().unwrap_or_default()]).expect("the patch applies cleanly");
    }

    // spec: delegation-kit/SPEC.md §Testing — the resume verdicts: a non-zero resume FAILS and keeps
    // the session, a committing one is REFUSED and ends it, and an unknown key fails without a spawn
    #[test]
    fn the_resume_verdicts() {
        let r = Repo::new("resume-verdicts");
        let fail = ["s=bash", "s=-c", "s=exit 3", "s=@SESSION_ID@"];
        let cfg = r.session_cfg(OPEN_STDOUT, &fail, MARKER, 60);
        assert_eq!(foreign_run(&cfg, &r.args("s", false)).code, 0);
        let v = foreign_resume(&cfg, "prompt", "answer.md");
        assert_eq!(v.code, 2, "{}", v.line);
        assert!(v.line.contains("turn=2 budget=OFF exit=3") && v.line.ends_with("-> FAILED (the adapter exited 3)"), "{}", v.line);
        assert!(r.read("session.txt").contains(" turn=2 "), "a failed turn keeps the session and advances");
        let commit = ["s=git", "s=-c", "s=user.name=t", "s=-c", "s=user.email=t@example.invalid", "s=commit", "s=--allow-empty", "s=-q", "s=-m", "s=@SESSION_ID@"];
        let cfg = r.session_cfg(OPEN_STDOUT, &commit, MARKER, 60);
        let v = foreign_resume(&cfg, "prompt", "answer.md");
        assert_eq!(v.code, 1, "{}", v.line);
        assert!(v.line.contains("turn=3") && v.line.contains("-> REFUSED (committed) — session ended, clone kept at "), "{}", v.line);
        assert!(!r.run_dir().join("session.txt").exists() && r.tree_kept());
        assert!(r.run_dir().join("report.2.txt").is_file(), "the failed turn's report was rotated");
        let v = foreign_resume(&cfg, "nope", "answer.md");
        assert_eq!(v.code, 2, "{}", v.line);
        assert!(v.line.starts_with("foreign-resume: adapter=- mode=- key=nope turn=- budget=- exit=- report=none"), "{}", v.line);
        assert!(v.line.contains("FAILED (no open session under"), "{}", v.line);
    }

    // spec: delegation-kit/SPEC.md §Resuming a session — the argv shape: a key and a prompt file, or a
    // key and `--close`, the key one path component, and the `--` escape
    #[test]
    fn the_resume_argv_shape_is_held() {
        assert_eq!(parse_resume(&strings(&["k", "a.md"])), Ok(ResumeArgs { key: "k".into(), prompt: Some("a.md".into()) }));
        assert_eq!(parse_resume(&strings(&["k", "--close"])), Ok(ResumeArgs { key: "k".into(), prompt: None }));
        assert_eq!(parse_resume(&strings(&["k", "--", "-a.md"])).map(|a| a.prompt), Ok(Some("-a.md".into())));
        for bad in [&["k"][..], &["k", "a", "b"], &["--close"], &["k", "a.md", "--close"], &["--help"], &["../k", "a"]] {
            assert!(parse_resume(&strings(bad)).is_err(), "{:?} must refuse", bad);
        }
    }

    // spec: delegation-kit/SPEC.md §The foreign-vendor run — `session.txt` reads back what it wrote,
    // and a line missing a field is refused
    #[test]
    fn the_session_record_round_trips() {
        let s = Session { adapter: "a".into(), sweep: true, base: "abc".into(), turn: 4, id: None };
        assert_eq!(Session::parse(&s.render()), Ok(s));
        assert!(Session::parse("adapter=a mode=audit turn=1 id=-\n").is_err());
        assert!(Session::parse("adapter=a mode=write base=b turn=1 id=-\n").is_err());
    }
}
