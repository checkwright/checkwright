// spec: guard-kit/SPEC.md §Layout and configuration — what the rules read beside the command: the
// guard's knobs, resolved in process, and the working tree they are asked about.
use super::engine::Shell;
use crate::knobs::{self, Value};
use crate::{proc, programs, walk};
use std::cell::OnceCell;

// spec: guard-kit/SPEC.md §Layout and configuration — every knob a rule or the log reads; a refusal
// on any of them is the one answer the member gives to a broken config.
pub const KNOBS: &[&str] = &[
    "GUARD_KIT_LOG",
    "GUARD_KIT_SETTINGS",
    "GUARD_KIT_RO_SCRIPTS",
    "GUARD_KIT_SCRATCH_DIRS",
    "GUARD_KIT_RO_BINS",
    "GUARD_KIT_RO_FORMS",
    "GUARD_KIT_APPEND_BINS",
    "GUARD_KIT_SEARCH_TOOLS",
    "GUARD_KIT_SCRIPT_INTERPRETERS",
    "GUARD_KIT_WORKTREE_READS",
    "GUARD_KIT_SCRATCH_POWERSHELL",
    "GUARD_KIT_CONSUMER_RULES_CMD",
    "GATE_SDK_NATIVE_BIN",
    "GATE_SDK_WORKFLOW_DIR",
];

// spec: guard-kit/SPEC.md §The generic ruleset — the working directory, the session's own worktree
// root and the main checkout's root, the last two empty outside a linked worktree.
pub struct Roots {
    pub cwd: String,
    pub own: String,
    pub main: String,
}

pub struct Host {
    pub pwd: String,
    pub log: String,
    pub settings: String,
    pub ro_scripts: Vec<String>,
    pub scratch_dirs: Vec<String>,
    pub ro_bins: Vec<String>,
    pub ro_forms: Vec<(String, String)>,
    pub append_bins: Vec<String>,
    pub search_tools: Vec<String>,
    pub interpreters: Vec<String>,
    pub worktree_reads: String,
    pub scratch_powershell: String,
    pub door: String,
    pub background: bool,
    allow: OnceCell<Vec<String>>,
    roots: OnceCell<Roots>,
}

fn scalar(name: &str) -> Result<String, String> {
    Ok(knobs::resolve(name)?.0.wire())
}

fn list(name: &str) -> Result<Vec<String>, String> {
    match knobs::resolve(name)?.0 {
        Value::Indexed(v) => Ok(v.into_iter().filter(|e| !e.is_empty()).collect()),
        Value::Scalar(s) if s.is_empty() => Ok(Vec::new()),
        Value::Scalar(s) => Ok(vec![s]),
        Value::Keyed(_) => Err(format!("{} is not an indexed knob", name)),
    }
}

fn keyed(name: &str) -> Result<Vec<(String, String)>, String> {
    match knobs::resolve(name)?.0 {
        Value::Keyed(v) => Ok(v),
        _ => Err(format!("{} is not a keyed knob", name)),
    }
}

// spec: guard-kit/SPEC.md §Consumer rules — the consumer's rule command, an argv spawned without a
// shell; empty is no consumer stage.
pub fn consumer_cmd() -> Result<Vec<String>, String> {
    list("GUARD_KIT_CONSUMER_RULES_CMD")
}

impl Host {
    // spec: guard-kit/SPEC.md §The shell guard — the knob load: every value resolved from the
    // static table and the consumer's knob file, with no spawn; the first refusal is the answer.
    pub fn load(background: bool) -> Result<Host, String> {
        let door = crate::installer::init::command_token(&knobs::wire("GATE_SDK_NATIVE_BIN")?);
        knobs::wire("GATE_SDK_WORKFLOW_DIR")?;
        Ok(Host {
            pwd: logical_pwd(),
            log: scalar("GUARD_KIT_LOG")?,
            settings: scalar("GUARD_KIT_SETTINGS")?,
            ro_scripts: list("GUARD_KIT_RO_SCRIPTS")?,
            scratch_dirs: list("GUARD_KIT_SCRATCH_DIRS")?,
            ro_bins: list("GUARD_KIT_RO_BINS")?,
            ro_forms: keyed("GUARD_KIT_RO_FORMS")?,
            append_bins: list("GUARD_KIT_APPEND_BINS")?,
            search_tools: list("GUARD_KIT_SEARCH_TOOLS")?,
            interpreters: list("GUARD_KIT_SCRIPT_INTERPRETERS")?,
            worktree_reads: scalar("GUARD_KIT_WORKTREE_READS")?,
            scratch_powershell: scalar("GUARD_KIT_SCRATCH_POWERSHELL")?,
            door,
            background,
            allow: OnceCell::new(),
            roots: OnceCell::new(),
        })
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the rules' `$PWD/`, matched against command text
    pub fn pwd_prefix(&self) -> String {
        // path-dialect-exempt: a needle for the command as the shell spelled it, never a path this crate produces
        format!("{}/", self.pwd)
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — a scratch member as command text spells it,
    // bare and `./`-led, for the tests that match the command as written
    pub fn scratch_prefixes(&self) -> Vec<(String, String)> {
        self.scratch_dirs
            .iter()
            // path-dialect-exempt: needles for the command as the shell spelled it, never paths this crate produces
            .map(|d| (format!("{}/", d), format!("./{}/", d)))
            .collect()
    }

    pub fn first_scratch(&self) -> &str {
        self.scratch_dirs.first().map_or("", String::as_str)
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the committed allow entries, read once per call;
    // a missing or unreadable settings file reads as none, and every reader declines.
    pub fn allow_entries(&self) -> &[String] {
        self.allow.get_or_init(|| {
            if !std::path::Path::new(&self.settings).is_file() {
                return Vec::new();
            }
            match crate::emit::compare_settings_allow::read_allow(&self.settings) {
                crate::emit::compare_settings_allow::AllowRead::Entries(e) => e,
                _ => Vec::new(),
            }
        })
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the committed `Bash(...)` allow inners.
    pub fn allow_inners(&self) -> Vec<String> {
        self.allow_entries()
            .iter()
            .filter_map(|e| e.strip_prefix("Bash(")?.strip_suffix(')'))
            .filter(|i| !i.is_empty())
            .map(String::from)
            .collect()
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the roots, resolved once per call; the walk up to
    // the nearest `.git` keeps git unspawned wherever that `.git` is a directory.
    pub fn roots(&self) -> &Roots {
        self.roots.get_or_init(|| {
            let cwd = walk::cwd().unwrap_or_default();
            let mut r = Roots { cwd: cwd.clone(), own: String::new(), main: String::new() };
            let mut d = cwd.trim_end_matches('/').to_string();
            while !d.is_empty() && !std::path::Path::new(&format!("{}/.git", d)).exists() {
                d = match d.rfind('/') {
                    Some(i) => d[..i].to_string(),
                    None => String::new(),
                };
            }
            if d.is_empty() || !std::path::Path::new(&format!("{}/.git", d)).is_file() {
                return r;
            }
            let git_dir = rev_parse(&cwd, "--git-dir");
            let common = rev_parse(&cwd, "--git-common-dir");
            if common.is_empty() || git_dir == common || !common.ends_with("/.git") {
                return r;
            }
            r.main = common[..common.len() - "/.git".len()].to_string();
            r.own = d;
            r
        })
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — lexical against the working directory, `.` and
    // `..` folded, the filesystem never read.
    pub fn lexical(&self, p: &str) -> String {
        let p = if walk::path_root(p).is_some() {
            p.to_string()
        } else {
            format!("{}/{}", self.roots().cwd, p)
        };
        let mut stack: Vec<&str> = Vec::new();
        for part in p.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                s => stack.push(s),
            }
        }
        format!("/{}", stack.join("/"))
    }

    // spec: guard-kit/SPEC.md §Layout and configuration — the directory each scratch member names for
    // this session: from a linked worktree a relative member resolves against the main checkout.
    pub fn scratch_homes(&self) -> Vec<String> {
        let main = &self.roots().main;
        self.scratch_dirs
            .iter()
            .map(|d| {
                if !main.is_empty() && walk::path_root(d).is_none() {
                    self.lexical(&format!("{}/{}", main, d))
                } else {
                    d.clone()
                }
            })
            .collect()
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — from a linked worktree, the directory each scratch
    // member names in the session's own worktree; none outside one.
    pub fn own_scratch_homes(&self) -> Vec<String> {
        let r = self.roots();
        if r.main.is_empty() {
            return Vec::new();
        }
        self.scratch_dirs
            .iter()
            .map(|d| {
                if walk::path_root(d).is_some() {
                    self.lexical(d)
                } else {
                    self.lexical(&format!("{}/{}", r.own, d))
                }
            })
            .collect()
    }

    pub fn in_scratch(&self, p: &str) -> bool {
        let t = self.lexical(p);
        self.scratch_homes().iter().any(|h| under(&t, &self.lexical(h)))
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — in the main checkout and outside the session's
    // own worktree, so a sibling worktree nested there counts as the main checkout's.
    pub fn in_main(&self, t: &str) -> bool {
        let r = self.roots();
        !r.main.is_empty() && under(t, &r.main) && !under(t, &r.own)
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the ignored-target test, asked of the checkout
    // that holds the target.
    pub fn ignored(&self, p: &str) -> bool {
        let main = self.roots().main.clone();
        if !main.is_empty() {
            let t = self.lexical(p);
            if self.in_main(&t) {
                return git_ok(&["-C", &main, "check-ignore", "--quiet", "--", &t]);
            }
        }
        git_ok(&["check-ignore", "--quiet", "--", p])
    }

    pub fn tracked(&self, p: &str) -> bool {
        git_ok(&["ls-files", "--error-unmatch", "--", p])
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the recorded producers still alive, each as the
    // block names it; a record that does not parse, or has no line end, is no record.
    pub fn live_run_records(&self) -> Vec<String> {
        let mut out = Vec::new();
        for d in self.scratch_homes() {
            let Ok(entries) = walk::list_dir(std::path::Path::new(&d)) else { continue };
            for (n, _) in entries.into_iter().filter(|(n, _)| n.ends_with(".run")) {
                let rec = format!("{}/{}", d, n);
                if !std::path::Path::new(&rec).is_file() {
                    continue;
                }
                let Ok(body) = std::fs::read_to_string(&rec) else { continue };
                let Some((line, _)) = body.split_once('\n') else { continue };
                let Some((pid, key)) = parse_record(line) else { continue };
                if matches!(crate::evidence::pid_alive(pid), Ok(true)) {
                    out.push(format!("'{}' (pid {}, recorded in {})", key, pid, rec));
                }
            }
        }
        out
    }

    // spec: guard-kit/SPEC.md §The shell guard — the fall-through line, cut to the harness's
    // analysis bound plus one and encoded so one call stays one decodable line; a PowerShell call
    // leads with its tool name and the one raw tab no encoded command carries.
    pub fn log_fallthrough(&self, cmd: &str, shell: Shell) {
        use std::io::Write;
        let cut: String = cmd.chars().take(10001).collect();
        let line = cut.replace('\\', "\\\\").replace('\n', "\\n").replace('\t', "\\t");
        let line = match shell {
            Shell::Bash => line,
            Shell::PowerShell => format!("PowerShell\t{}", line),
        };
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&self.log) {
            let _ = writeln!(f, "{}", line);
        }
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — `^pid=([1-9][0-9]*)[[:space:]]run=([^[:space:]]+)$`
fn parse_record(line: &str) -> Option<(&str, &str)> {
    let rest = line.strip_prefix("pid=")?;
    let n = rest.bytes().take_while(u8::is_ascii_digit).count();
    let (pid, rest) = rest.split_at(n);
    if pid.is_empty() || pid.starts_with('0') {
        return None;
    }
    let mut chars = rest.chars();
    if !chars.next().is_some_and(|c| c.is_ascii_whitespace() || c == '\x0b') {
        return None;
    }
    let key = chars.as_str().strip_prefix("run=")?;
    if key.is_empty() || key.bytes().any(super::text::is_space) {
        return None;
    }
    Some((pid, key))
}

pub fn under(a: &str, b: &str) -> bool {
    walk::at_or_under(b, a)
}

fn git_ok(args: &[&str]) -> bool {
    proc::run(&programs::GIT, args).is_ok_and(|c| c.code() == Some(0))
}

fn rev_parse(cwd: &str, flag: &str) -> String {
    let Ok(c) = proc::run(&programs::GIT, &["rev-parse", flag]) else {
        return String::new();
    };
    let Some(o) = c.stdout() else { return String::new() };
    let p = String::from_utf8_lossy(o).trim_end_matches(['\n', '\r']).to_string();
    if p.is_empty() {
        return String::new();
    }
    walk::canonicalize(walk::abs_against(cwd, &p))
        .map(|c| walk::normalize_abs(c.strip_prefix(r"\\?\").unwrap_or(&c)))
        .unwrap_or_default()
}

// spec: guard-kit/SPEC.md §The generic ruleset — the rules' `$PWD`: the inherited logical spelling
// where it names the working directory, the physical one otherwise.
fn logical_pwd() -> String {
    let cwd = walk::cwd().unwrap_or_default();
    match std::env::var("PWD") {
        Ok(p) if walk::path_root(&p).is_some()
            && walk::canonicalize(&p).is_some()
            && walk::canonicalize(&p) == walk::canonicalize(&cwd) =>
        {
            p
        }
        _ => cwd,
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `git_c_root`'s knob read: a static scalar's
// resolved value, or nothing for a name no kit owns.
pub fn knob_scalar_value(name: &str) -> Option<String> {
    let out = knobs::values(&[name.to_string()]).ok()?;
    let line = out.lines().next()?;
    let mut f = line.splitn(3, '\t');
    let _ = f.next()?;
    let shape = f.next()?;
    let resolved = f.next().unwrap_or("");
    (shape == "scalar").then(|| resolved.to_string())
}

// spec: guard-kit/SPEC.md §The generic ruleset — `test -ef`: the same file by device and inode.
pub fn same_file(a: &str, b: &str) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        match (std::fs::metadata(a), std::fs::metadata(b)) {
            (Ok(x), Ok(y)) => x.dev() == y.dev() && x.ino() == y.ino(),
            _ => false,
        }
    }
    #[cfg(not(unix))]
    {
        walk::canonicalize(a).is_some() && walk::canonicalize(a) == walk::canonicalize(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_line_parses_only_in_its_one_grammar() {
        assert_eq!(parse_record("pid=12 run=k"), Some(("12", "k")));
        assert_eq!(parse_record("pid=012 run=k"), None);
        assert_eq!(parse_record("pid=12  run=k"), None);
        assert_eq!(parse_record("pid=12 run=k x"), None);
        assert_eq!(parse_record("garbage"), None);
    }

    #[test]
    fn containment_is_by_whole_component() {
        assert!(under("/a/b", "/a"));
        assert!(under("/a", "/a"));
        assert!(!under("/ab", "/a"));
    }
}
