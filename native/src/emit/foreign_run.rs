// spec: delegation-kit/SPEC.md §The foreign-vendor run — one read-only audit or mechanical sweep on
// a consumer's foreign adapter, in a scratch clone of committed `HEAD`: 0 OK, 1 REFUSED, 2 FAILED.
use crate::proc::{self, ChildEnv, Redirect};
use crate::programs::{self, Program};
use crate::walk;
use std::path::{Path, PathBuf};

pub const KNOBS: &[&str] = &[
    "DELEGATION_KIT_FOREIGN_ADAPTERS",
    "DELEGATION_KIT_FOREIGN_TIMEOUT",
    "GATE_SDK_TMP_DIR",
];

const USAGE: &str = "usage: --foreign-run <adapter> <prompt-file> [--mode audit|sweep] [--key <key>] [--]\n  runs one unit on the adapter DELEGATION_KIT_FOREIGN_ADAPTERS configures, in a scratch clone of committed HEAD; the key names the run's directory and defaults to the prompt file's stem";

const PROMPT_TOKEN: &str = "@PROMPT_FILE@";

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

// spec: gate-sdk/SPEC.md §The bin/-tool contract — a dash-led token naming no option is a refusal,
// and `--` ends option processing
fn parse(args: &[String]) -> Result<Args, String> {
    let mut rest: Vec<&str> = Vec::new();
    let mut sweep = false;
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
            i += 1;
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
    let [adapter, prompt] = rest[..] else {
        return Err(format!("foreign-run: takes an adapter and a prompt file (got {} operand(s))", rest.len()));
    };
    let key = key.unwrap_or_else(|| {
        Path::new(prompt).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default()
    });
    // spec: delegation-kit/SPEC.md §The foreign-vendor run — the key is one path component, so the
    // run's directory stays under the scratch dir's `foreign/`
    if key.is_empty() || key == "." || key == ".." || key.contains(['/', '\\']) {
        return Err(format!("foreign-run: the key '{}' is not one path component", key));
    }
    Ok(Args { adapter: adapter.to_string(), prompt: prompt.to_string(), sweep, key })
}

// spec: delegation-kit/SPEC.md §Layout and configuration — an adapter's argv is its `<adapter>=<word>`
// elements' words, in element order
fn argv_of(adapters: &[String], name: &str) -> Vec<String> {
    adapters
        .iter()
        .filter_map(|e| e.split_once('='))
        .filter(|(a, _)| *a == name)
        .map(|(_, w)| w.to_string())
        .collect()
}

pub struct Config {
    pub adapters: Vec<String>,
    pub timeout: u64,
    pub repo: String,
    pub base: String,
    pub here: String,
}

struct Verdict {
    line: String,
    code: i32,
}

struct Run<'a> {
    args: &'a Args,
    dir: PathBuf,
    exit: String,
    report: Option<PathBuf>,
    patch: Option<PathBuf>,
}

impl Run<'_> {
    fn line(&self, verdict: &str, code: i32) -> Verdict {
        let show = |p: &Option<PathBuf>| p.as_ref().map_or("none".to_string(), |p| p.display().to_string());
        Verdict {
            line: format!(
                "foreign-run: adapter={} mode={} key={} exit={} report={} patch={} -> {}",
                self.args.adapter,
                if self.args.sweep { "sweep" } else { "audit" },
                self.args.key,
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
fn clone(repo: &str, tree: &Path) -> Result<(), String> {
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
    Ok(())
}

// spec: delegation-kit/SPEC.md §The foreign-vendor run — a sweep's whole change, untracked files
// included, as a binary-safe patch against the checked-out commit; an empty change is no patch
fn change(tree: &Path) -> Result<Vec<u8>, String> {
    git(tree, &["add", "-A"])?;
    git(tree, &["diff", "--cached", "--binary", "HEAD"])
}

fn foreign_run(cfg: &Config, a: &Args) -> Verdict {
    let dir = PathBuf::from(&cfg.base).join("foreign").join(&a.key);
    let mut run = Run { args: a, dir, exit: "-".to_string(), report: None, patch: None };
    let argv = argv_of(&cfg.adapters, &a.adapter);
    let Some((program, words)) = argv.split_first() else {
        return run.failed(&format!("no adapter '{}' is configured in DELEGATION_KIT_FOREIGN_ADAPTERS", a.adapter));
    };
    let prompt = PathBuf::from(walk::abs_against(&cfg.here, &a.prompt));
    if std::fs::File::open(&prompt).is_err() || !prompt.is_file() {
        return run.failed(&format!("cannot read the prompt file {}", prompt.display()));
    }
    let tree = run.dir.join("tree");
    // spec: delegation-kit/SPEC.md §The foreign-vendor run — a clone an earlier refusal kept is
    // evidence awaiting inspection, never overwritten
    if tree.exists() {
        return run.failed(&format!(
            "a kept clone occupies {}; inspect and remove it, or pass another --key",
            tree.display()
        ));
    }
    if let Err(e) = std::fs::create_dir_all(&run.dir) {
        return run.failed(&format!("cannot create {}: {}", run.dir.display(), e));
    }
    let report = run.dir.join("report.txt");
    let stderr = run.dir.join("stderr.txt");
    let patch = run.dir.join("change.patch");
    for f in [&report, &stderr, &patch] {
        let _ = std::fs::remove_file(f);
    }
    if let Err(e) = clone(&cfg.repo, &tree) {
        let _ = std::fs::remove_dir_all(&tree);
        return run.failed(&format!("clone: {}", e));
    }
    let before = match refs(&tree) {
        Ok(r) => r,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tree);
            return run.failed(&format!("clone: {}", e));
        }
    };
    let prompt_abs = prompt.display().to_string();
    let args: Vec<String> = words.iter().map(|w| w.replace(PROMPT_TOKEN, &prompt_abs)).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let adapter = Program::consumer("DELEGATION_KIT_FOREIGN_ADAPTERS", program.replace(PROMPT_TOKEN, &prompt_abs));
    let unset = git_env();
    let io = Redirect { cwd: &tree, stdin: &prompt, stdout: &report, stderr: &stderr, unset: &unset };
    let status = proc::run_bounded_redirected(&adapter, &args, &io, cfg.timeout);
    let status = match status {
        Ok(s) => s,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tree);
            return run.failed(&format!("spawn: {}", e));
        }
    };
    run.report = Some(report);
    run.exit = status.map_or("timeout".to_string(), |c| c.to_string());
    // spec: delegation-kit/SPEC.md §The foreign-vendor run — the shape is checked after every run
    // that started, and a refusal outranks a failure: the kept clone is the evidence either way
    let refused = |run: &Run, why: &str| {
        run.line(&format!("REFUSED ({}) — clone kept at {}", why, tree.display()), 1)
    };
    match refs(&tree) {
        Ok(after) if after != before => return refused(&run, "committed"),
        Ok(_) => {}
        Err(e) => return refused(&run, &format!("the clone's refs are unreadable: {}", e)),
    }
    if !a.sweep {
        match git(&tree, &["status", "--porcelain", "--untracked-files=all"]) {
            Ok(s) if !s.is_empty() => return refused(&run, "audit wrote"),
            Ok(_) => {}
            Err(e) => return refused(&run, &format!("the clone's status is unreadable: {}", e)),
        }
    }
    let verdict = match status {
        None => run.failed(&format!("timeout after {}s", cfg.timeout)),
        Some(c) if c != 0 => run.failed(&format!("the adapter exited {}", c)),
        Some(_) if a.sweep => match change(&tree) {
            Ok(body) if body.is_empty() => run.line("OK", 0),
            Ok(body) => match std::fs::write(&patch, body) {
                Ok(()) => {
                    run.patch = Some(patch);
                    run.line("OK", 0)
                }
                Err(e) => run.failed(&format!("cannot write {}: {}", patch.display(), e)),
            },
            Err(e) => run.failed(&format!("the sweep's change is unreadable: {}", e)),
        },
        Some(_) => run.line("OK", 0),
    };
    let _ = std::fs::remove_dir_all(&tree);
    verdict
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
    Ok(Config {
        adapters: walk::knob_array("DELEGATION_KIT_FOREIGN_ADAPTERS")?,
        timeout,
        repo,
        base,
        here,
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
    let v = match config() {
        Ok(cfg) => foreign_run(&cfg, &a),
        Err(e) => Run { args: &a, dir: PathBuf::new(), exit: "-".to_string(), report: None, patch: None }
            .failed(&format!("config: {}", e)),
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
            Config {
                adapters: strings(adapters),
                timeout,
                repo: self.root.join("src").display().to_string(),
                base: self.root.join("tmp").display().to_string(),
                here: self.root.display().to_string(),
            }
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
        assert!(v.line.contains("mode=audit key=prompt exit=0 report="), "{}", v.line);
        assert!(v.line.ends_with("patch=none -> OK"), "{}", v.line);
        let report = std::fs::read(r.run_dir().join("report.txt")).expect("the report");
        assert_eq!(String::from_utf8_lossy(&report), hash_of(&r, b"audit the tree\n"), "stdin carries the prompt");
        assert!(!r.tree_kept(), "the clone is removed on OK");
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
        assert!(v.line.contains("exit=- report=none patch=none -> FAILED (no adapter 'nope'"), "{}", v.line);
        let mut missing = r.args("ro", false);
        missing.prompt = "absent.md".to_string();
        let v = foreign_run(&r.cfg(&["ro=git"], 60), &missing);
        assert!(v.code == 2 && v.line.contains("cannot read the prompt file"), "{}", v.line);
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
        let a = parse(&strings(&["ad", "dir/unit.prompt.md"])).expect("two operands");
        assert_eq!(a, Args { adapter: "ad".into(), prompt: "dir/unit.prompt.md".into(), sweep: false, key: "unit.prompt".into() });
        let a = parse(&strings(&["--mode", "sweep", "--key", "k1", "ad", "p.md"])).expect("options first");
        assert!(a.sweep && a.key == "k1");
        let a = parse(&strings(&["ad", "--", "-p.md"])).expect("the escape");
        assert_eq!(a.prompt, "-p.md");
        for bad in [&["ad"][..], &["ad", "p", "q"], &["--help"], &["--mode", "write", "ad", "p"], &["--key", "a/b", "ad", "p"], &["--key", "..", "ad", "p"]] {
            assert!(parse(&strings(bad)).is_err(), "{:?} must refuse", bad);
        }
    }

    // spec: delegation-kit/SPEC.md §Layout and configuration — an adapter's argv is its elements'
    // words in element order, and another adapter's words never join it
    #[test]
    fn an_adapters_argv_is_its_own_words_in_order() {
        let table = strings(&["a=prog", "b=other", "a=--flag", "a=x=y"]);
        assert_eq!(argv_of(&table, "a"), strings(&["prog", "--flag", "x=y"]));
        assert!(argv_of(&table, "c").is_empty());
    }
}
