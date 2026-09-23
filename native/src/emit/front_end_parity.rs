// spec: gate-sdk/SPEC.md §run-gates — the front-end parity oracle, an `Arm::Run` member: the
// contract is the 0/1/2 split — identical, divergent, could not run — which `Arm::Emit` cannot carry
use crate::proc::{self, ChildEnv, Completed};
use crate::programs::{self, Program};
use crate::walk;
use std::path::{Path, PathBuf};

// spec: gate-sdk/SPEC.md §run-gates — the arm resolves no knob: every value a case needs it writes
// into that case's own scratch repository or environment
pub const KNOBS: &[&str] = &[];

const NAME: &str = "run-front-end-parity";
const VERDICT: &str = "FRONT-END-PARITY";

// spec: gate-sdk/SPEC.md §run-gates — one corpus member: what the case sets up, and what the bash
// stub must print for the case to exercise what its name says. The reference is checked so two
// halves failing the same way cannot pass as parity.
struct Case {
    name: &'static str,
    in_repo: bool,
    argv: &'static [&'static str],
    env: &'static [(&'static str, &'static str)],
    files: &'static [(&'static str, &'static str)],
    runnable_bin: bool,
    stdin: &'static str,
    expect_code: i32,
    expect_text: &'static str,
    linked: Linked,
}

// spec: gate-sdk/SPEC.md §run-gates — whether the case runs from a hand-built linked worktree, and
// whether its main checkout carries a runnable binary at the default path
#[derive(Clone, Copy, PartialEq)]
enum Linked {
    No,
    MainBin,
    MainBare,
}

const ABSENT: &str = "is absent or not executable";
const LOCAL: &str = "scripts/gate-sdk-config.local.knobs";
const TRACKED: &str = "scripts/gate-sdk-config.knobs";

const fn absent(
    name: &'static str,
    env: &'static [(&'static str, &'static str)],
    files: &'static [(&'static str, &'static str)],
    expect_text: &'static str,
) -> Case {
    Case {
        name,
        in_repo: true,
        argv: &["--emit", "knob-values"],
        env,
        files,
        runnable_bin: false,
        stdin: "",
        expect_code: 2,
        expect_text,
        linked: Linked::No,
    }
}

const fn grammar(
    name: &'static str,
    argv: &'static [&'static str],
    expect_code: i32,
    expect_text: &'static str,
) -> Case {
    Case {
        name,
        in_repo: true,
        argv,
        env: &[],
        files: &[],
        runnable_bin: true,
        stdin: "",
        expect_code,
        expect_text,
        linked: Linked::No,
    }
}

const QUESTION: &str = "{\"tool_input\":{\"to\":\"main\",\"message\":\"Question only\"}}\n";
const ESCALATION_ADVICE: &str = "Options Recommendation Evidence";

// spec: gate-sdk/SPEC.md §run-gates — the corpus: each exit path, each precedence tier of
// `GATE_SDK_NATIVE_BIN`, a backslash-rooted value, each residual-grammar form, forwarded stdin,
// and the linked-worktree resolution on a hand-built layout
const CORPUS: &[Case] = &[
    Case {
        name: "outside a repository",
        in_repo: false,
        argv: &["--emit", "knob-values"],
        env: &[],
        files: &[],
        runnable_bin: false,
        stdin: "",
        expect_code: 2,
        expect_text: "run-gates: not inside a git repository",
        linked: Linked::No,
    },
    absent("binary absent, leading --emit", &[], &[], ABSENT),
    Case {
        name: "binary absent, leading --hook",
        argv: &["--hook", "escalation-guard"],
        expect_code: 0,
        ..absent("", &[], &[], ABSENT)
    },
    Case {
        name: "binary absent, leading --HOOK, which is no fail-open name",
        argv: &["--HOOK", "escalation-guard"],
        ..absent("", &[], &[], ABSENT)
    },
    Case {
        name: "binary absent, leading --statusline",
        argv: &["--statusline"],
        expect_code: 0,
        ..absent("", &[], &[], ABSENT)
    },
    absent(
        "GATE_SDK_NATIVE_BIN from the environment",
        &[("GATE_SDK_NATIVE_BIN", "from-env/gates")],
        &[
            (LOCAL, "GATE_SDK_NATIVE_BIN=from-local/gates\n"),
            (TRACKED, "GATE_SDK_NATIVE_BIN=from-tracked/gates\n"),
        ],
        "./from-env/gates",
    ),
    absent(
        "GATE_SDK_NATIVE_BIN from the local overlay",
        &[],
        &[
            (
                LOCAL,
                "# GATE_SDK_NATIVE_BIN=commented/gates\ngate_sdk_native_bin=other-case/gates\n \t GATE_SDK_NATIVE_BIN \t= from-local/gates \t\n",
            ),
            (TRACKED, "GATE_SDK_NATIVE_BIN=from-tracked/gates\n"),
        ],
        "./from-local/gates",
    ),
    absent(
        "GATE_SDK_NATIVE_BIN from the tracked knob file",
        &[],
        &[(TRACKED, "GATE_SDK_GATES_DIR=scripts\nGATE_SDK_NATIVE_BIN=from-tracked/gates")],
        "./from-tracked/gates",
    ),
    absent(
        "GATE_SDK_NATIVE_BIN from a GATE_SDK_KNOB_FILE override",
        &[("GATE_SDK_KNOB_FILE", "alt.knobs")],
        &[
            ("alt.knobs", "GATE_SDK_NATIVE_BIN=from-override/gates\n"),
            (TRACKED, "GATE_SDK_NATIVE_BIN=from-tracked/gates\n"),
        ],
        "./from-override/gates",
    ),
    absent(
        "GATE_SDK_NATIVE_BIN from a GATE_SDK_GATES_DIR knob directory",
        &[("GATE_SDK_GATES_DIR", "cfg")],
        &[
            ("cfg/gate-sdk-config.knobs", "GATE_SDK_NATIVE_BIN=from-gates-dir/gates\n"),
            (TRACKED, "GATE_SDK_NATIVE_BIN=from-tracked/gates\n"),
        ],
        "./from-gates-dir/gates",
    ),
    absent(
        "GATE_SDK_NATIVE_BIN rooted by a leading backslash, taking no prefix",
        &[("GATE_SDK_NATIVE_BIN", "\\from-env\\gates")],
        &[],
        "but \\from-env\\gates is absent",
    ),
    absent(
        "GATE_SDK_NATIVE_BIN defaulted",
        &[],
        &[],
        "./native/target/release/checkwright-gates",
    ),
    absent(
        "GATE_SDK_NATIVE_BIN empty in the overlay, falling through to the tracked file",
        &[],
        &[
            (LOCAL, "GATE_SDK_NATIVE_BIN=\n"),
            (TRACKED, "GATE_SDK_NATIVE_BIN=from-tracked/gates\n"),
        ],
        "./from-tracked/gates",
    ),
    absent(
        "GATE_SDK_NATIVE_BIN empty everywhere, taking the default",
        &[("GATE_SDK_NATIVE_BIN", "")],
        &[(LOCAL, "GATE_SDK_NATIVE_BIN=\n"), (TRACKED, "GATE_SDK_NATIVE_BIN= \n")],
        "./native/target/release/checkwright-gates",
    ),
    grammar("-h", &["-h"], 0, "usage: run-gates.sh"),
    grammar("--help", &["--help"], 0, "usage: run-gates.sh"),
    grammar("--only <name>", &["--only", "check-x"], 2, "scripts/gates.list"),
    grammar("--for <path>", &["--for", "README.md"], 2, "scripts/gates.list"),
    grammar("--", &["--"], 2, "scripts/gates.list"),
    grammar("-- <dir>", &["--", "elsewhere", "ignored"], 2, "elsewhere/gates.list"),
    grammar("no argument", &[], 2, "scripts/gates.list"),
    grammar("a bare positional", &["positional", "ignored"], 2, "positional/gates.list"),
    grammar(
        "a leading --emit <arm>",
        &["--emit", "knob-values", "GATE_SDK_NATIVE_BIN"],
        0,
        "GATE_SDK_NATIVE_BIN\tscalar",
    ),
    Case {
        name: "stdin forwarded to --hook <name>",
        stdin: QUESTION,
        ..grammar("", &["--hook", "escalation-guard"], 0, ESCALATION_ADVICE)
    },
    Case {
        name: "linked worktree, --hook through the main checkout's binary",
        argv: &["--hook", "escalation-guard"],
        stdin: QUESTION,
        expect_code: 0,
        linked: Linked::MainBin,
        ..absent("", &[], &[], ESCALATION_ADVICE)
    },
    Case {
        name: "linked worktree, --emit refusing to link onto an unignored door path",
        linked: Linked::MainBin,
        ..absent("", &[], &[], "is not gitignored here")
    },
    Case {
        name: "linked worktree, --emit through the main checkout's linked binary",
        argv: &["--emit", "knob-values", "GATE_SDK_NATIVE_BIN"],
        expect_code: 0,
        linked: Linked::MainBin,
        ..absent("", &[], &[(".gitignore", "native/target/\n")], "GATE_SDK_NATIVE_BIN\tscalar")
    },
    Case {
        name: "linked worktree, both binaries absent, --hook declining",
        argv: &["--hook", "escalation-guard"],
        stdin: QUESTION,
        expect_code: 0,
        linked: Linked::MainBare,
        ..absent("", &[], &[], ABSENT)
    },
];

// spec: gate-sdk/SPEC.md §run-gates — the scratch root is removed on every exit path
struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        eprintln!("{}: unknown argument: {}; usage: --run-front-end-parity", NAME, a);
        return 2;
    }
    match check() {
        Ok(true) => 0,
        Ok(false) => 1,
        Err(e) => {
            eprintln!(
                "{}: {} — the check could not run; treating as failure (not clean)",
                NAME, e
            );
            2
        }
    }
}

fn check() -> Result<bool, String> {
    let here = walk::cwd()?;
    let sdk = PathBuf::from(walk::abs_against(&here, walk::sdk_root().trim_end_matches('/')));
    let stub = sdk.join("bin").join("run-gates.sh");
    let twin = sdk.join("bin").join("run-gates.ps1");
    for f in [&stub, &twin] {
        if !f.is_file() {
            return Err(format!("{} is not a file", f.display()));
        }
    }
    let hosts = twin_hosts();
    for p in std::iter::once(&programs::BASH).chain(hosts.iter()) {
        if !proc::on_path(p) {
            return Err(format!("no {} resolves on PATH", p));
        }
    }
    let exe = forward_slashed(
        std::env::current_exe()
            .map_err(|e| format!("cannot locate this binary: {}", e))?
            .display()
            .to_string(),
    );
    let scratch = Scratch(scratch_root()?);
    let unset = inherited_knobs();
    let mut clean = true;
    for (i, case) in CORPUS.iter().enumerate() {
        let dir = scratch.0.join(format!("case-{:02}", i));
        let (stub_arg, twin_arg) = prepare(case, &dir, &sdk, &stub, &twin, Path::new(&exe))?;
        let mut set: Vec<(String, String)> = vec![(
            "GIT_CEILING_DIRECTORIES".to_string(),
            forward_slashed(scratch.0.display().to_string()),
        )];
        set.extend(case.env.iter().map(|(k, v)| (k.to_string(), v.to_string())));
        if case.runnable_bin {
            set.push(("GATE_SDK_NATIVE_BIN".to_string(), exe.clone()));
        }
        let env = ChildEnv {
            set: &set,
            unset: &unset,
            cwd: Some(&dir),
        };
        let mut bash_argv: Vec<&str> = vec![stub_arg.as_str()];
        bash_argv.extend(case.argv);
        let mut twin_argv: Vec<&str> = vec!["-NoProfile", "-NonInteractive", "-File", twin_arg.as_str()];
        twin_argv.extend(case.argv);
        let bash = Transcript::of(&programs::BASH, &bash_argv, case, &env)?;
        if bash.code != case.expect_code || !bash.text().contains(case.expect_text) {
            return Err(format!(
                "case '{}': the bash stub did not do what the case names (want exit {} carrying {:?}), so the corpus exercised nothing\n{}",
                case.name,
                case.expect_code,
                case.expect_text,
                bash.render("bash")
            ));
        }
        for host in &hosts {
            let who = host.name();
            let theirs = Transcript::of(host, &twin_argv, case, &env)?;
            if let Some(diff) = bash.first_difference(&theirs, &who) {
                clean = false;
                println!("{}: FAIL — case '{}' diverges under {}: {}", VERDICT, case.name, who, diff);
                print!("{}{}", bash.render("bash"), theirs.render(&who));
            }
        }
    }
    if clean {
        let names: Vec<String> = hosts.iter().map(|h| h.name()).collect();
        println!(
            "{}: clean — {} cases, both front-ends identical after CRLF becomes LF, the twin under {}",
            VERDICT,
            CORPUS.len(),
            names.join(" and ")
        );
    }
    Ok(clean)
}

// spec: gate-sdk/SPEC.md §run-gates — every PowerShell host the platform ships; `cfg!` rather than
// `#[cfg]` keeps the Windows-only row referenced on every target
fn twin_hosts() -> Vec<Program> {
    if cfg!(windows) {
        vec![programs::PWSH, programs::POWERSHELL]
    } else {
        vec![programs::PWSH]
    }
}

// spec: gate-sdk/SPEC.md §run-gates — a Windows path handed to both halves in the one spelling
// Git-for-Windows bash and PowerShell each read unconverted
fn forward_slashed(p: String) -> String {
    if cfg!(windows) {
        p.replace('\\', "/")
    } else {
        p
    }
}

// spec: gate-sdk/SPEC.md §run-gates — a long-name, prefix-free absolute root: Windows hands the
// temporary directory out in its 8.3 spelling and `canonicalize` adds a verbatim prefix, and either
// would make the stub and the twin see two spellings of one directory
fn scratch_root() -> Result<PathBuf, String> {
    let root = std::env::temp_dir().join(format!("checkwright-front-end-parity.{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).map_err(|e| format!("cannot create {}: {}", root.display(), e))?;
    let s = walk::canonicalize(&root).ok_or_else(|| format!("cannot resolve {}", root.display()))?;
    Ok(PathBuf::from(s.strip_prefix(r"\\?\").unwrap_or(&s)))
}

// spec: gate-sdk/SPEC.md §run-gates — every name under a static kit's prefix, plus git's own
// repository locators, is stripped from both halves' environment: the case sets what it measures
fn inherited_knobs() -> Vec<String> {
    let prefixes: Vec<String> = crate::knobs::STATIC_KITS.iter().map(|k| k.prefix()).collect();
    std::env::vars_os()
        .filter_map(|(k, _)| k.into_string().ok())
        .filter(|k| {
            prefixes.iter().any(|p| k.starts_with(p.as_str())) || crate::proc::GIT_REPO_LOCATORS.contains(&k.as_str())
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §run-gates — a case's scratch: a fresh repository vendoring the tree's own
// `gate-sdk/bin/` and `gate-sdk/lib/`, run by relative path as a user types it; the outside case
// runs the tree's own pair from a directory no repository contains
fn prepare(
    case: &Case,
    dir: &Path,
    sdk: &Path,
    stub: &Path,
    twin: &Path,
    exe: &Path,
) -> Result<(String, String), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
    if !case.in_repo {
        return Ok((
            forward_slashed(stub.display().to_string()),
            forward_slashed(twin.display().to_string()),
        ));
    }
    if case.linked == Linked::No {
        git_init(dir)?;
    } else {
        link(dir, case.linked, exe)?;
    }
    for sub in ["bin", "lib"] {
        let dest = dir.join("gate-sdk").join(sub);
        std::fs::create_dir_all(&dest).map_err(|e| format!("cannot create {}: {}", dest.display(), e))?;
        let from = sdk.join(sub);
        for (name, is_dir) in walk::list_dir(&from)? {
            if !is_dir {
                std::fs::copy(from.join(&name), dest.join(&name))
                    .map_err(|e| format!("cannot vendor {}: {}", from.join(&name).display(), e))?;
            }
        }
    }
    for (rel, body) in case.files {
        write(&dir.join(rel), body)?;
    }
    Ok((
        "gate-sdk/bin/run-gates.sh".to_string(),
        "gate-sdk/bin/run-gates.ps1".to_string(),
    ))
}

fn git_init(dir: &Path) -> Result<(), String> {
    let dir_s = dir.display().to_string();
    let init = proc::run(&programs::GIT, &["init", "-q", &dir_s])?;
    match init.failure_report() {
        Some(r) => Err(format!("git init {}: {}", dir_s, r)),
        None => Ok(()),
    }
}

fn write(p: &Path, body: &str) -> Result<(), String> {
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {}", parent.display(), e))?;
    }
    std::fs::write(p, body).map_err(|e| format!("cannot write {}: {}", p.display(), e))
}

// spec: gate-sdk/SPEC.md §run-gates — a linked worktree built by hand on git's documented on-disk
// format, because a crate source adding one is refused; the main checkout sits beside the case
// directory
fn link(dir: &Path, linked: Linked, exe: &Path) -> Result<(), String> {
    let leaf = dir
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("{} has no name", dir.display()))?;
    let main = dir.with_file_name(format!("{}-main", leaf));
    git_init(&main)?;
    let admin = main.join(".git").join("worktrees").join("linked");
    write(&admin.join("HEAD"), "ref: refs/heads/linked\n")?;
    write(&admin.join("commondir"), "../..\n")?;
    write(
        &admin.join("gitdir"),
        &format!("{}\n", forward_slashed(dir.join(".git").display().to_string())),
    )?;
    write(
        &dir.join(".git"),
        &format!("gitdir: {}\n", forward_slashed(admin.display().to_string())),
    )?;
    if linked == Linked::MainBin {
        let bin = main
            .join("native")
            .join("target")
            .join("release")
            .join(format!("checkwright-gates{}", std::env::consts::EXE_SUFFIX));
        if let Some(parent) = bin.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {}", parent.display(), e))?;
        }
        std::fs::copy(exe, &bin).map_err(|e| format!("cannot place {}: {}", bin.display(), e))?;
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §run-gates — one half's result after the one normalization: CRLF becomes
// LF, PowerShell's console writer ending lines with CRLF on Windows being no contract difference
struct Transcript {
    code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn crlf_to_lf(b: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(b.len());
    for (i, c) in b.iter().enumerate() {
        if *c == b'\r' && b.get(i + 1) == Some(&b'\n') {
            continue;
        }
        out.push(*c);
    }
    out
}

impl Transcript {
    fn of(program: &Program, argv: &[&str], case: &Case, env: &ChildEnv) -> Result<Transcript, String> {
        let done: Completed = proc::run_with_stdin_in(program, argv, case.stdin.as_bytes(), env)?;
        let (out, err) = done.streams();
        Ok(Transcript {
            code: done.reported_code(),
            stdout: crlf_to_lf(out),
            stderr: crlf_to_lf(err),
        })
    }

    fn text(&self) -> String {
        format!(
            "{}{}",
            String::from_utf8_lossy(&self.stdout),
            String::from_utf8_lossy(&self.stderr)
        )
    }

    // spec: gate-sdk/SPEC.md §run-gates — the first place the two halves part: the status, else the
    // first differing line of stdout, then of stderr
    fn first_difference(&self, other: &Transcript, who: &str) -> Option<String> {
        if self.code != other.code {
            return Some(format!("exit {} (bash) against {} ({})", self.code, other.code, who));
        }
        for (label, a, b) in [
            ("stdout", &self.stdout, &other.stdout),
            ("stderr", &self.stderr, &other.stderr),
        ] {
            if a == b {
                continue;
            }
            let (a, b) = (String::from_utf8_lossy(a), String::from_utf8_lossy(b));
            let (al, bl): (Vec<&str>, Vec<&str>) = (a.split('\n').collect(), b.split('\n').collect());
            let n = (0..al.len().max(bl.len()))
                .find(|i| al.get(*i) != bl.get(*i))
                .unwrap_or(0);
            return Some(format!(
                "{} line {}: {:?} (bash) against {:?} ({})",
                label,
                n + 1,
                al.get(n).copied().unwrap_or("<end>"),
                bl.get(n).copied().unwrap_or("<end>"),
                who
            ));
        }
        None
    }

    fn render(&self, who: &str) -> String {
        let block = |b: &[u8]| {
            let s = String::from_utf8_lossy(b);
            if s.is_empty() {
                "    <empty>\n".to_string()
            } else {
                s.lines().map(|l| format!("    {}\n", l)).collect()
            }
        };
        format!(
            "  {}: exit {}\n  {} stdout:\n{}  {} stderr:\n{}",
            who,
            self.code,
            who,
            block(&self.stdout),
            who,
            block(&self.stderr)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §run-gates — the one normalization is CRLF alone: a status, a stray
    // byte or a line either half lacks is a divergence the arm names by its first line
    #[test]
    fn transcripts_differ_only_past_the_crlf_normalization() {
        let t = |code: i32, out: &str| Transcript {
            code,
            stdout: crlf_to_lf(out.as_bytes()),
            stderr: Vec::new(),
        };
        assert!(t(0, "a\nb\n").first_difference(&t(0, "a\r\nb\r\n"), "pwsh").is_none());
        let d = t(0, "a\nb\n").first_difference(&t(0, "a\nc\n"), "powershell").expect("diverges");
        assert!(d.starts_with("stdout line 2:"), "{}", d);
        assert!(d.ends_with("(powershell)"), "{}", d);
        assert!(t(2, "").first_difference(&t(0, ""), "pwsh").expect("diverges").starts_with("exit 2"));
    }

    // spec: gate-sdk/SPEC.md §run-gates — Windows PowerShell joins the host set on Windows alone
    #[test]
    fn the_twin_runs_under_every_powershell_host_the_platform_ships() {
        let names: Vec<String> = twin_hosts().iter().map(Program::name).collect();
        let want: &[&str] = if cfg!(windows) { &["pwsh", "powershell"] } else { &["pwsh"] };
        assert_eq!(names, want);
    }

    // spec: gate-sdk/SPEC.md §run-gates — every case is named, and no two share a name, so a red
    // run's case line points at exactly one member
    #[test]
    fn every_case_carries_a_distinct_name() {
        let mut names: Vec<&str> = CORPUS.iter().map(|c| c.name).collect();
        assert!(names.iter().all(|n| !n.is_empty()));
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), CORPUS.len());
    }
}
