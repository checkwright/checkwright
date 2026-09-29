// spec: gate-sdk/SPEC.md §git-hook — the arm each generated git hook hands off to: the triggered
// subset of the registry at commit time, run serially to the first failure
use crate::runner::{self, Dispatch, Selected};
use crate::{proc, programs, registry, walk};
use std::path::{Path, PathBuf};

pub const KNOBS: &[&str] = &["GATE_SDK_GATES_DIR", "GATE_SDK_KIT_DIRS"];

pub const USAGE: &str = "usage: --git-hook pre-commit | commit-msg <message-file>";

enum Hook {
    PreCommit,
    CommitMsg(String),
}

impl Hook {
    fn name(&self) -> &'static str {
        match self {
            Hook::PreCommit => "pre-commit",
            Hook::CommitMsg(_) => "commit-msg",
        }
    }
}

fn parse(args: &[String]) -> Result<Hook, String> {
    match args {
        [h] if h == "pre-commit" => Ok(Hook::PreCommit),
        [h, f] if h == "commit-msg" => Ok(Hook::CommitMsg(f.clone())),
        [h] if h == "commit-msg" => Err(format!("commit-msg: git did not pass the message-file path\n{}", USAGE)),
        _ => Err(USAGE.to_string()),
    }
}

// spec: gate-sdk/SPEC.md §git-hook — the work tree git implied: `GIT_DIR` set alone, as git leaves it
// for a hook in a linked worktree, makes the working directory the work-tree top
fn implied_work_tree(git_dir: Option<&str>, work_tree: Option<&str>, cwd: &Path) -> Option<PathBuf> {
    match (git_dir, work_tree) {
        (Some(d), None) if !d.is_empty() => Some(cwd.to_path_buf()),
        _ => None,
    }
}

// spec: gate-sdk/SPEC.md §git-hook — the pin, as an addition to each member's environment
fn pin() -> Result<Vec<(String, String)>, String> {
    let cwd = walk::cwd()?;
    let git_dir = std::env::var("GIT_DIR").ok();
    let work_tree = std::env::var("GIT_WORK_TREE").ok();
    Ok(implied_work_tree(git_dir.as_deref(), work_tree.as_deref(), Path::new(&cwd))
        .map(|top| vec![("GIT_WORK_TREE".to_string(), top.display().to_string())])
        .unwrap_or_default())
}

fn staged() -> Result<Vec<String>, String> {
    let c = proc::run(&programs::GIT, &["diff", "--cached", "--name-only", "-z", "--diff-filter=ACMR"])?;
    let out = c.stdout().ok_or_else(|| "git could not list the staged set".to_string())?;
    Ok(String::from_utf8_lossy(out)
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(String::from)
        .collect())
}

fn first_seen(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for m in registry::members(text) {
        if !out.contains(&m) {
            out.push(m);
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §git-hook — the pre-commit members: the `--for` selector over the staged set,
// restricted to `tier=precommit`, each `mode=staged` member keeping only its regular-file operands
pub(crate) fn precommit_selection(
    members: &[String],
    resolve_dirs: &[String],
    kit_roots_here: &[String],
    staged: &[String],
    top: &Path,
) -> Result<Vec<Selected>, String> {
    let s = runner::select(members, resolve_dirs, kit_roots_here, staged, Some("precommit"))?;
    let mut out: Vec<Selected> = Vec::new();
    for mut sel in s.run {
        if !sel.args.is_empty() {
            sel.args.retain(|p| top.join(p).is_file());
            if sel.args.is_empty() {
                continue;
            }
        }
        out.push(sel);
    }
    Ok(out)
}

fn commit_msg_selection(members: &[String], resolve_dirs: &[String], file: &str) -> Vec<Selected> {
    members
        .iter()
        .filter(|m| {
            registry::resolve(m, resolve_dirs).is_some_and(|src| {
                let body = std::fs::read_to_string(&src).unwrap_or_default();
                let f = registry::manifest_line(&body).map(registry::manifest_fields).unwrap_or_default();
                registry::field(&f, "tier") == "commit-msg"
            })
        })
        .map(|m| Selected {
            name: m.clone(),
            args: vec![file.to_string()],
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §git-hook — the quiet-green report: a failing member's output, then the
// uniform failure lines; a passing one's only under `GATE_SDK_VERBOSE`
fn report(hook: &str, d: &Dispatch, selected: &[Selected], verbose: bool, out: &mut String) -> i32 {
    let reprint = |out: &mut String, bytes: &[u8]| {
        let body = runner::trim_trailing_newlines(bytes);
        if !body.is_empty() {
            out.push_str(&String::from_utf8_lossy(body));
            out.push('\n');
        }
    };
    for (i, sel) in selected.iter().enumerate() {
        let o = runner::dispatch_one(d, i, sel);
        if o.failed {
            reprint(out, &o.output);
            out.push_str(&format!(
                "\n{}: {} failed (see above).\n  Bypass once (use sparingly): git commit --no-verify\n",
                hook, sel.name
            ));
            return 1;
        }
        if verbose {
            reprint(out, &o.output);
            out.push_str(&format!("  PASS: {}\n", sel.name));
        }
    }
    out.push_str(&format!("{}: {} gate(s) passed.\n", hook, selected.len()));
    0
}

fn hook_run(hook: &Hook) -> Result<(i32, String), String> {
    walk::toplevel_opt()?.ok_or_else(|| "not inside a git repository".to_string())?;
    let paths = match hook {
        Hook::PreCommit => staged()?,
        Hook::CommitMsg(_) => Vec::new(),
    };
    run_over(hook, &paths)
}

// spec: gate-sdk/SPEC.md §git-hook — an empty staged set exits 0 printing nothing, before the registry is read
fn run_over(hook: &Hook, paths: &[String]) -> Result<(i32, String), String> {
    if matches!(hook, Hook::PreCommit) && paths.is_empty() {
        return Ok((0, String::new()));
    }
    let env = pin()?;
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    let list = registry::list_path(&gates_dir);
    let text = std::fs::read_to_string(&list).map_err(|e| format!("cannot read the registry {}: {}", list, e))?;
    let members = first_seen(&text);
    let resolve_dirs = registry::resolve_dirs(&gates_dir, &walk::kit_roots_abs()?);
    let selected = match hook {
        Hook::PreCommit => precommit_selection(&members, &resolve_dirs, &walk::kit_roots()?, paths, Path::new("."))?,
        Hook::CommitMsg(file) => commit_msg_selection(&members, &resolve_dirs, file),
    };
    let self_exe = std::env::current_exe()
        .map_err(|e| format!("cannot resolve this binary's own path: {}", e))?
        .display()
        .to_string();
    let scratch = std::env::temp_dir().join(format!("checkwright-git-hook.{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).map_err(|e| format!("cannot create {}: {}", scratch.display(), e))?;
    let d = Dispatch {
        resolve_dirs: &resolve_dirs,
        self_exe: &self_exe,
        list: &list,
        scratch: &scratch,
        spec_base_url: "",
        env: &env,
    };
    let verbose = std::env::var("GATE_SDK_VERBOSE").is_ok_and(|v| !v.is_empty());
    let mut out = String::new();
    let code = report(hook.name(), &d, &selected, verbose, &mut out);
    let _ = std::fs::remove_dir_all(&scratch);
    Ok((code, out))
}

pub fn run(args: &[String]) -> i32 {
    let hook = match parse(args) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    match hook_run(&hook) {
        Ok((code, out)) => {
            print!("{}", out);
            code
        }
        Err(e) => {
            eprintln!("{}: {} — the hook could not run; treating as failure (not clean)", hook.name(), e);
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: gate-sdk/SPEC.md §git-hook — no operand, an unknown hook, an extra operand and a
    // commit-msg with no message file are refused before anything is read
    #[test]
    fn a_malformed_argv_is_a_usage_refusal() {
        for bad in [&[][..], &["pre-push"][..], &["pre-commit", "x"][..], &["commit-msg"][..], &["commit-msg", "m", "x"][..]] {
            let e = parse(&argv(bad)).err().unwrap_or_else(|| panic!("{:?} was accepted", bad));
            assert!(e.ends_with(USAGE), "{:?} refused without the usage line: {}", bad, e);
        }
        assert!(matches!(parse(&argv(&["pre-commit"])), Ok(Hook::PreCommit)));
        assert!(matches!(parse(&argv(&["commit-msg", "m"])), Ok(Hook::CommitMsg(f)) if f == "m"));
        assert_eq!(run(&argv(&["nope"])), 2);
    }

    #[test]
    fn an_empty_staged_set_passes_printing_nothing() {
        assert_eq!(run_over(&Hook::PreCommit, &[]), Ok((0, String::new())));
    }

    #[test]
    fn only_git_dir_set_alone_implies_the_working_directory() {
        let here = Path::new("/w");
        assert_eq!(implied_work_tree(Some("/r/.git/worktrees/w"), None, here), Some(here.to_path_buf()));
        assert_eq!(implied_work_tree(Some("/r/.git"), Some("/elsewhere"), here), None);
        assert_eq!(implied_work_tree(None, None, here), None);
        assert_eq!(implied_work_tree(Some(""), None, here), None);
    }

    // spec: gate-sdk/SPEC.md §git-hook — the pin against a scratch repository with a hand-built
    // linked worktree: a member-shaped `git -C <subdir> ls-files | hash-object` fails under git's
    // hook environment and succeeds once the implied work tree is pinned
    #[test]
    fn the_pin_makes_a_subdirectory_git_call_resolve_in_a_linked_worktree() {
        let base = std::env::temp_dir().join(format!("checkwright-git-hook-pin.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let (main, wt) = (base.join("main"), base.join("wt"));
        let put = |p: PathBuf, body: &str| {
            std::fs::create_dir_all(p.parent().expect("a parent")).expect("mkdir");
            std::fs::write(p, body).expect("write");
        };
        let git = |args: &[&str]| {
            proc::run(&programs::GIT, args).expect("git runs").stdout().expect("git succeeds").to_vec()
        };
        put(main.join("sub/a.txt"), "hi\n");
        let m = main.display().to_string();
        git(&["-C", &m, "init", "-q"]);
        git(&["-C", &m, "add", "sub/a.txt"]);
        git(&["-C", &m, "-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", "i"]);
        let head = String::from_utf8_lossy(&git(&["-C", &m, "rev-parse", "HEAD"])).into_owned();
        let admin = main.join(".git/worktrees/wt");
        put(admin.join("HEAD"), &head);
        put(admin.join("commondir"), "../..\n");
        put(admin.join("gitdir"), &format!("{}\n", wt.join(".git").display()));
        std::fs::copy(main.join(".git/index"), admin.join("index")).expect("the worktree's index");
        put(wt.join(".git"), &format!("gitdir: {}\n", admin.display()));
        put(wt.join("sub/a.txt"), "hi\n");

        let hashes = |pinned: bool| -> bool {
            let mut env = vec![("GIT_DIR".to_string(), admin.display().to_string())];
            if let Some(top) = implied_work_tree(Some(&env[0].1), None, &wt).filter(|_| pinned) {
                env.push(("GIT_WORK_TREE".to_string(), top.display().to_string()));
            }
            let ls = proc::run_with_env_in(&programs::GIT, &["-C", "sub", "ls-files"], &env, Some(&wt))
                .expect("git runs");
            let listed = String::from_utf8_lossy(ls.stdout().expect("ls-files succeeds")).into_owned();
            let mut args = vec!["-C", "sub", "hash-object", "--"];
            args.extend(listed.lines());
            proc::run_with_env_in(&programs::GIT, &args, &env, Some(&wt))
                .expect("git runs")
                .stdout()
                .is_some()
        };
        let (unpinned, pinned) = (hashes(false), hashes(true));
        let _ = std::fs::remove_dir_all(&base);
        assert!(!unpinned, "the unpinned hook environment hashed the listing, so the scratch does not reproduce the red");
        assert!(pinned, "the pinned environment still cannot hash what ls-files listed");
    }

    // spec: gate-sdk/SPEC.md §git-hook — the tier filter in registry order, a `mode=staged` member's
    // regular-file operands, a member left with none not run, and an unresolved member selected to red
    #[test]
    fn the_selection_keeps_the_precommit_tier_and_regular_file_operands() {
        let base = std::env::temp_dir().join(format!("checkwright-git-hook-select.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let checks = base.join("checks");
        std::fs::create_dir_all(&checks).expect("checks dir");
        let gate = |n: &str, manifest: &str| {
            std::fs::write(checks.join(format!("{}.sh", n)), format!("#!/bin/sh\n# graph: {}\n", manifest))
                .expect("write");
        };
        gate("check-pre", "couples=src/* dir=one valve=none tier=precommit");
        gate("check-align", "couples=src/* dir=one valve=none tier=align-only");
        gate("check-msg", "couples=src/* dir=one valve=none tier=commit-msg");
        gate("check-staged", "couples=src dir=one valve=none tier=precommit mode=staged");
        gate("check-other", "couples=docs/* dir=one valve=none tier=precommit");
        std::fs::create_dir_all(base.join("src")).expect("src");
        std::fs::write(base.join("src/a.rs"), "x").expect("write");
        let members: Vec<String> =
            ["check-absent", "check-pre", "check-align", "check-msg", "check-staged", "check-other"]
                .iter()
                .map(|s| s.to_string())
                .collect();
        let dirs = vec![checks.display().to_string()];
        let pick = |staged: &[&str]| -> Result<Vec<(String, Vec<String>)>, String> {
            let staged: Vec<String> = staged.iter().map(|s| s.to_string()).collect();
            Ok(precommit_selection(&members, &dirs, &[], &staged, &base)?
                .into_iter()
                .map(|s| (s.name, s.args))
                .collect())
        };
        let (both, gone_only) = (pick(&["src/a.rs", "src/gone.rs"]), pick(&["src/gone.rs"]));
        let _ = std::fs::remove_dir_all(&base);
        let own = |v: &[(&str, &[&str])]| -> Vec<(String, Vec<String>)> {
            v.iter().map(|(n, a)| (n.to_string(), a.iter().map(|s| s.to_string()).collect())).collect()
        };
        assert_eq!(
            both.expect("selects"),
            own(&[("check-absent", &[]), ("check-pre", &[]), ("check-staged", &["src/a.rs"])])
        );
        assert_eq!(gone_only.expect("selects"), own(&[("check-absent", &[]), ("check-pre", &[])]));
    }

    // spec: gate-sdk/SPEC.md §git-hook — the first failure stops the run with the uniform failure
    // lines, and a green run ends on the summary line
    #[cfg(unix)]
    #[test]
    fn a_failing_member_stops_the_run_and_a_green_one_prints_the_summary() {
        let base = std::env::temp_dir().join(format!("checkwright-git-hook-report.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let checks = base.join("checks");
        std::fs::create_dir_all(base.join("scratch")).expect("scratch");
        std::fs::create_dir_all(&checks).expect("checks");
        let gate = |n: &str, body: &str| {
            let p = checks.join(format!("{}.sh", n));
            std::fs::write(&p, format!("#!/bin/sh\n# graph: couples=* dir=one valve=none tier=precommit\n{}\n", body))
                .expect("write");
            crate::install::make_executable(&p).expect("exec bit");
        };
        gate("check-ok", "echo quiet");
        gate("check-red", "echo 'x.rs:1: finding'; exit 1");
        gate("check-never", "echo reached");
        // spec: gate-sdk/SPEC.md §check-crate-arms — a script is run only once no sibling thread's
        // fork can still hold it open for writing, which a spawn meanwhile meets as ETXTBSY
        for n in ["check-ok", "check-red", "check-never"] {
            let p = programs::Program::consumer(programs::GATE_DECLARATION, checks.join(format!("{}.sh", n)).display().to_string());
            let mut tries = 0;
            while proc::run(&p, &[]).is_err_and(|e| e.contains("busy")) && tries < 200 {
                tries += 1;
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        let dirs = vec![checks.display().to_string()];
        let scratch = base.join("scratch");
        let d = Dispatch { resolve_dirs: &dirs, self_exe: "", list: "gates.list", scratch: &scratch, spec_base_url: "", env: &[] };
        let sel = |n: &[&str]| -> Vec<Selected> {
            n.iter().map(|s| Selected { name: s.to_string(), args: Vec::new() }).collect()
        };
        let (mut red, mut green, mut loud) = (String::new(), String::new(), String::new());
        let red_code = report("pre-commit", &d, &sel(&["check-ok", "check-red", "check-never"]), false, &mut red);
        let green_code = report("commit-msg", &d, &sel(&["check-ok"]), false, &mut green);
        let loud_code = report("pre-commit", &d, &sel(&["check-ok"]), true, &mut loud);
        let _ = std::fs::remove_dir_all(&base);
        assert_eq!(red_code, 1);
        assert_eq!(
            red,
            "x.rs:1: finding\n\npre-commit: check-red failed (see above).\n  Bypass once (use sparingly): git commit --no-verify\n"
        );
        assert_eq!((green_code, green.as_str()), (0, "commit-msg: 1 gate(s) passed.\n"));
        assert_eq!((loud_code, loud.as_str()), (0, "quiet\n  PASS: check-ok\npre-commit: 1 gate(s) passed.\n"));
    }
}
