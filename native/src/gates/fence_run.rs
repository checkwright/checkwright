// spec: canon-kit/SPEC.md §check-fence-run — every shell fence a doc marks runnable passes a
// narrowed static pass, then runs in a scratch copy of the tracked tree and exits as declared
use crate::bashscan::{self, Kind, Token};
use crate::emit::csmoke;
use crate::gates::fence_command_head::{self as fch, Ctx, Head};
use crate::{proc, programs, spec, walk};
use std::path::{Path, PathBuf};

pub const NAME: &str = "check-fence-run";

// spec: canon-kit/SPEC.md §check-fence-run — the nesting marker, the depth of fence scratches a
// run sits inside, and the backstop depth that ends a doc naming itself
const NESTED: &str = "FENCE_RUN_NESTED";
const DEPTH_BACKSTOP: u32 = 3;

fn nesting_depth() -> u32 {
    std::env::var(NESTED)
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0)
}

// spec: canon-kit/SPEC.md §check-fence-run — the finding carries this many trailing output lines
const TAIL: usize = 20;

// spec: canon-kit/SPEC.md §check-fence-run — the marker's two spellings
const MARKER: &str = "<!-- fence-runnable -->";
const MARKER_OPEN: &str = "<!-- fence-runnable: ";
const MARKER_CLOSE: &str = " -->";

// spec: canon-kit/SPEC.md §check-fence-run — the unroutable loopback port every proxy variable
// names, so a proxy-honoring program fails fast rather than dialing out
const DEAD_PROXY: &str = "http://127.0.0.1:9";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-fence-run — a marked fence: its opening line, the status it
// declares, and its body
pub struct Marked {
    pub open: usize,
    pub exit: i32,
    pub body: String,
}

// spec: canon-kit/SPEC.md §check-fence-run — the marker read strictly: `None` is a line that does
// not look like the marker at all, `Some(Err)` one that looks like it and misses the grammar
fn marker(line: &str) -> Option<Result<i32, String>> {
    let t = line.trim();
    let looks = t
        .strip_prefix("<!--")
        .is_some_and(|r| r.trim_start().starts_with("fence-run"));
    if !looks {
        return None;
    }
    if t == MARKER {
        return Some(Ok(0));
    }
    let operand = t.strip_prefix(MARKER_OPEN).and_then(|r| r.strip_suffix(MARKER_CLOSE));
    Some(match operand {
        Some(op) => match op.strip_prefix("exit=") {
            Some(n) if !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) => n
                .parse::<u8>()
                .map(i32::from)
                .map_err(|_| format!("marker operand '{}' is not an exit status (0-255)", op)),
            _ => Err(format!("marker operand '{}' is not exit=<decimal>", op)),
        },
        None => Err(format!("'{}' is a misspelled fence-runnable marker", t)),
    })
}

// spec: canon-kit/SPEC.md §check-fence-run — one doc's marked fences and its marker findings; a
// marker-shaped line inside a fence is fence content and is not read
pub fn scan(text: &str) -> (Vec<Marked>, Vec<(usize, String)>) {
    let fences = fch::shell_fences(text);
    let lines: Vec<&str> = text.lines().collect();
    let mut marked = Vec::new();
    let mut bad = Vec::new();
    let mut in_fence = false;
    for (idx, raw) in lines.iter().enumerate() {
        if spec::is_fence_line(raw) {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let Some(verdict) = marker(raw) else { continue };
        let ln = idx + 1;
        match verdict {
            Err(why) => bad.push((ln, why)),
            Ok(exit) => match fences.iter().find(|(open, _)| *open == ln + 1) {
                Some((open, body)) => marked.push(Marked {
                    open: *open,
                    exit,
                    body: body.clone(),
                }),
                None => bad.push((
                    ln,
                    "marker is not immediately above a bash, sh or shell fence".to_string(),
                )),
            },
        }
    }
    (marked, bad)
}

// spec: canon-kit/SPEC.md §check-fence-run — whether a doc carries a fence this gate would run,
// the question the bash audience's third derived arm asks
pub fn carries_marked_fence(text: &str) -> bool {
    !scan(text).0.is_empty()
}

// spec: canon-kit/SPEC.md §check-fence-run — the corpus: `check-md-refs`' governed doc set under
// `anchor`, and at the default `CANON_KIT_SCAN_KIT_ROOTS` nothing under the kit roots the caller
// names, so a reader anchored on a payload prunes the payload's kits as the gate prunes vendored ones
pub fn corpus_at(anchor: &str, kit_roots: &[String]) -> Result<Vec<String>, String> {
    let exclude = spec::knob_array_pub("CANON_KIT_MDREF_EXCLUDE")?;
    let keep_kits = spec::knob_pub("CANON_KIT_SCAN_KIT_ROOTS")? == "1";
    let here = walk::cwd()?;
    let anchor_abs = walk::abs_against(&here, anchor);
    let roots: Vec<String> = kit_roots
        .iter()
        .filter(|r| !r.is_empty())
        .map(|r| walk::abs_against(&anchor_abs, r))
        .filter(|r| walk::under(&anchor_abs, r))
        .collect();
    Ok(spec::manifest_files(anchor)?
        .into_iter()
        .map(|p| p.display().to_string())
        .filter(|f| {
            let fabs = walk::abs_against(&here, f);
            let rel = walk::rel_under(&anchor_abs, &fabs)
                .map(str::to_string)
                .unwrap_or_else(|| spec::strip_dot_slash(f));
            !exclude.iter().any(|g| walk::pattern_match(g, &rel))
                && (keep_kits || !roots.iter().any(|r| walk::under(r, &fabs)))
        })
        .collect())
}

// spec: canon-kit/SPEC.md §check-fence-run — a static-pass red: body line, head, rule
type Red = (usize, String, String);

struct Runnable {
    doc: String,
    fence: Marked,
    heads: usize,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let depth = nesting_depth();
    if (depth > 0 && args.is_empty()) || depth >= DEPTH_BACKSTOP {
        println!(
            "FENCE-RUN: clean (nested {} deep inside a fence scratch; 0 marked fence(s) executed)",
            depth
        );
        return Ok(0);
    }
    let top = walk::toplevel()
        .map_err(|_| "not a git repository — cannot resolve tracked paths".to_string())?;
    let files: Vec<String> = if !args.is_empty() {
        args.to_vec()
    } else {
        corpus_at(".", &walk::kit_roots()?)?
    };
    let ctx = Ctx {
        tracked: fch::tracked_files(&top)?,
        cwd: walk::cwd()?,
        top,
        programs: spec::knob_array_pub("CANON_KIT_FENCE_RUN_PROGRAMS")?
            .into_iter()
            .collect(),
    };
    let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN")?;

    let mut bad: Vec<String> = Vec::new();
    let mut runnable: Vec<Runnable> = Vec::new();
    let mut nmarked = 0usize;
    for f in &files {
        if !Path::new(f).is_file() {
            continue;
        }
        let text = spec::read_text(Path::new(f))?;
        let (marked, findings) = scan(&text);
        for (ln, why) in findings {
            bad.push(format!("{}:{}: {}", f, ln, why));
        }
        let docdir = fch::dirname(f);
        for fence in marked {
            nmarked += 1;
            let (reds, heads) = judge(&ctx, &docdir, &fence.body, &bin)?;
            if reds.is_empty() {
                runnable.push(Runnable {
                    doc: f.clone(),
                    fence,
                    heads,
                });
                continue;
            }
            for (ln, word, why) in reds {
                bad.push(format!("{}:{}: '{}' {}", f, fence.open + ln, word, why));
            }
        }
    }

    let mut nexec = 0usize;
    let mut ndocs = 0usize;
    let mut i = 0usize;
    while i < runnable.len() {
        let doc = runnable[i].doc.clone();
        let end = runnable[i..]
            .iter()
            .position(|r| r.doc != doc)
            .map_or(runnable.len(), |n| i + n);
        ndocs += 1;
        for (r, outcome) in runnable[i..end].iter().zip(execute(&runnable[i..end], &bin)?) {
            nexec += r.heads;
            if let Some(finding) = outcome {
                bad.push(finding);
            }
        }
        i = end;
    }

    if !bad.is_empty() {
        println!("{}: marked fence(s) that do not run as declared:", NAME);
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: fix the fence or its declared status (<!-- fence-runnable: exit=<n> -->); a head");
        println!("        outside the admitted set takes a builtin, a program named in");
        println!("        CANON_KIT_FENCE_RUN_PROGRAMS, or the gate binary under a fence-safe arm, and a");
        println!("        fence not meant to run drops its marker.");
        return Ok(1);
    }
    println!(
        "FENCE-RUN: clean ({} doc(s); {} marked fence(s); {} command(s) executed in {} scratch(es))",
        files.len(),
        nmarked,
        nexec,
        ndocs
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-fence-run — the static pass: check-fence-command-head's order,
// narrowed to the fence-run program set, with the gate binary held to a fence-safe arm and every
// other tracked script refused, since its body is outside the pass
fn judge(
    ctx: &Ctx,
    docdir: &str,
    body: &str,
    bin: &str,
) -> Result<(Vec<Red>, usize), String> {
    let mut funcs = fch::defined_functions(body);
    let mut bad = Vec::new();
    let mut heads = 0usize;
    for t in bashscan::command_heads(body) {
        if matches!(t.kind, Kind::Guard) {
            continue;
        }
        heads += 1;
        if matches!(t.kind, Kind::Keyword) {
            continue;
        }
        if t.word == "." || t.word == "source" {
            if let Some(text) = fch::sourced_text(ctx, docdir, &t)? {
                funcs.extend(fch::defined_functions(&text));
            }
        }
        let word = if matches!(t.kind, Kind::Expansion) {
            format!("${}", t.word)
        } else {
            t.word.clone()
        };
        let why = if RUNS_OPERAND.contains(&t.word.as_str()) {
            Some(OPERAND_WHY.to_string())
        } else if matches!(t.kind, Kind::Expansion) || names_binary(ctx, &t.word, bin) {
            arm_verdict(&t)
        } else {
            match fch::classify(ctx, docdir, &funcs, &t.word) {
                Err(why) if why.starts_with("names no builtin") => Some(
                    "names no builtin, program in CANON_KIT_FENCE_RUN_PROGRAMS, or function"
                        .to_string(),
                ),
                Err(why) => Some(why.to_string()),
                Ok(Head::Expansion) => arm_verdict(&t),
                Ok(Head::TrackedPath) => Some(
                    "is a tracked script, whose body the static pass does not read".to_string(),
                ),
                Ok(_) => None,
            }
        };
        if let Some(why) = why {
            bad.push((t.line, word, why));
        }
    }
    for (ln, line) in body.lines().enumerate() {
        let words: Vec<String> = line
            .split_whitespace()
            .take_while(|w| !w.starts_with('#'))
            .map(fch::unquoted)
            .collect();
        for (i, w) in words.iter().enumerate() {
            let probe = words.get(i + 1).is_some_and(|n| n == "-v" || n == "-V");
            if w == "command" && !probe {
                bad.push((ln + 1, w.clone(), OPERAND_WHY.to_string()));
            }
        }
    }
    Ok((bad, heads))
}

// spec: canon-kit/SPEC.md §check-fence-run — the builtins that run an operand as a command, which
// the head scan never judges; `command` is read word by word, since the scanner emits no head for
// it, and only its `-v`/`-V` probe form passes
const RUNS_OPERAND: &[&str] = &["exec", "eval", "trap"];
const OPERAND_WHY: &str = "runs its operand as a command, which the static pass cannot judge";

fn names_binary(ctx: &Ctx, word: &str, bin: &str) -> bool {
    let w = fch::unquoted(word);
    w.contains('/') && walk::abs_against(&ctx.cwd, &w) == walk::abs_against(&ctx.cwd, bin)
}

// spec: canon-kit/SPEC.md §check-fence-run — an expansion head is read as the gate binary, so its
// first operand names the arm: a `-`-led one must be fence-safe, `--emit <name>` naming a member of
// that family, and a bare one a registered gate
fn arm_verdict(t: &Token) -> Option<String> {
    let ops: Vec<String> = t.operands.iter().map(|o| fch::unquoted(o)).collect();
    let Some(op) = ops.first() else {
        return Some("runs with no arm, which FENCE_SAFE_ARMS cannot admit".to_string());
    };
    if op == "--emit" {
        let arm = format!("--emit-{}", ops.get(1).map_or("", String::as_str));
        if crate::emit::fence_safe(&arm) {
            return None;
        }
        return Some(format!("runs '--emit {}', which names no --emit arm", ops.get(1).map_or("", String::as_str)));
    }
    if op.starts_with('-') {
        if crate::emit::fence_safe(op) {
            return None;
        }
        return Some(format!("runs arm '{}', which FENCE_SAFE_ARMS does not admit", op));
    }
    if crate::gates::lookup(op).is_some() {
        return None;
    }
    Some(format!("runs '{}', which names no registered gate", op))
}

// spec: canon-kit/SPEC.md §check-fence-run — one doc's runnable fences in one scratch, in document
// order, each its own `bash -euo pipefail` under the fixed environment; the scratch guard removes
// the directory on every exit path
fn execute(fences: &[Runnable], bin: &str) -> Result<Vec<Option<String>>, String> {
    let base = scratch_base();
    let guard = csmoke::tracked_scratch(&base, "fence-run")?;
    let dir: PathBuf = guard.dir.clone().ok_or("the scratch was not created")?;
    let root = dir.display().to_string();

    let placed = walk::abs_against(&root, bin);
    if walk::under(&root, &placed) {
        let exe = std::env::current_exe()
            .map_err(|e| format!("cannot locate the running binary: {}", e))?;
        csmoke::place_artifact(&exe.display().to_string(), &placed)?;
    }
    let home = dir.join(".git").join("fence-home");
    std::fs::create_dir_all(&home).map_err(|e| format!("cannot create {}: {}", home.display(), e))?;
    let gitconfig = home.join(".gitconfig");
    std::fs::write(&gitconfig, "").map_err(|e| format!("cannot write {}: {}", gitconfig.display(), e))?;
    let env = fixed_env(&home, &gitconfig, &placed);

    let mut out = Vec::new();
    for r in fences {
        let done = proc::run_merged_isolated(
            &programs::BASH,
            &["-euo", "pipefail", "-c", &r.fence.body],
            &env,
            &dir,
        )?;
        let rc = done.reported_code();
        if rc == r.fence.exit {
            out.push(None);
            continue;
        }
        let text = String::from_utf8_lossy(done.output()).into_owned();
        let lines: Vec<&str> = text.lines().collect();
        let tail = &lines[lines.len().saturating_sub(TAIL)..];
        let mut finding = format!(
            "{}:{}: fence exited {}, declared {}; last {} line(s) of its output:",
            r.doc,
            r.fence.open,
            rc,
            r.fence.exit,
            tail.len()
        );
        for l in tail {
            finding.push_str("\n      | ");
            finding.push_str(l);
        }
        out.push(Some(finding));
    }
    drop(guard);
    Ok(out)
}

// spec: canon-kit/SPEC.md §check-fence-run — `DEMO_TMP_DIR`, then the platform temp directory, the
// base §Consumer smoke's builder takes
pub(crate) fn scratch_base() -> PathBuf {
    std::env::var_os("DEMO_TMP_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}

// spec: canon-kit/SPEC.md §check-fence-run — the fixed environment, inherited in nothing but the
// host's `PATH` value (and on Windows the system root a process cannot start without)
fn fixed_env(home: &Path, gitconfig: &Path, bin: &str) -> Vec<(String, String)> {
    let mut env: Vec<(String, String)> = vec![
        ("PATH".into(), std::env::var("PATH").unwrap_or_default()),
        ("LC_ALL".into(), "C".into()),
        ("HOME".into(), home.display().to_string()),
        ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
        ("GIT_CONFIG_GLOBAL".into(), gitconfig.display().to_string()),
        ("GIT_ALLOW_PROTOCOL".into(), "file".into()),
        ("GATE_SDK_NATIVE_BIN".into(), bin.to_string()),
        (NESTED.into(), (nesting_depth() + 1).to_string()),
    ];
    for v in ["http_proxy", "https_proxy", "all_proxy", "HTTP_PROXY", "HTTPS_PROXY", "ALL_PROXY"] {
        env.push((v.into(), DEAD_PROXY.into()));
    }
    if cfg!(windows) {
        if let Ok(v) = std::env::var("SystemRoot") {
            env.push(("SystemRoot".into(), v));
        }
    }
    env
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: canon-kit/SPEC.md §check-fence-run — the grammar is strict: the bare marker and an
    // exit operand pass, and a near miss reds rather than silently disarming
    #[test]
    fn the_marker_is_read_strictly() {
        assert_eq!(marker("<!-- fence-runnable -->"), Some(Ok(0)));
        assert_eq!(marker("   <!-- fence-runnable: exit=1 -->"), Some(Ok(1)));
        assert!(matches!(marker("<!-- fence-runable -->"), Some(Err(_))));
        assert!(matches!(marker("<!--fence-runnable-->"), Some(Err(_))));
        assert!(matches!(marker("<!-- fence-runnable: exit=x -->"), Some(Err(_))));
        assert!(matches!(marker("<!-- fence-runnable: exit=300 -->"), Some(Err(_))));
        assert!(matches!(marker("<!-- fence-runnable: status=1 -->"), Some(Err(_))));
        assert_eq!(marker("prose naming `<!-- fence-runnable -->` inline"), None);
        assert_eq!(marker("<!-- door-contributor: x -->"), None);
    }

    // spec: canon-kit/SPEC.md §check-fence-run — a marker binds the fence on the next line only, a
    // non-shell fence or a gap is an orphan, and a marker inside a fence is content
    #[test]
    fn a_marker_binds_only_the_shell_fence_directly_below() {
        let text = "<!-- fence-runnable -->\n```bash\ntrue\n```\n\n<!-- fence-runnable -->\n\n```bash\ntrue\n```\n<!-- fence-runnable -->\n```text\nx\n```\n```markdown\n<!-- fence-runable -->\n```\n";
        let (marked, bad) = scan(text);
        assert_eq!(marked.len(), 1);
        assert_eq!((marked[0].open, marked[0].exit, marked[0].body.as_str()), (2, 0, "true\n"));
        assert_eq!(bad.iter().map(|(l, _)| *l).collect::<Vec<_>>(), vec![6, 11]);
        assert!(carries_marked_fence(text));
        assert!(!carries_marked_fence("```bash\ntrue\n```\n"));
    }

    fn ctx(programs: &[&str]) -> Ctx {
        Ctx {
            top: "/r".into(),
            cwd: "/r".into(),
            tracked: ["lib/tool.sh".to_string(), "bin/run.sh".to_string()].into_iter().collect(),
            programs: programs.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn reds(body: &str) -> Vec<String> {
        judge(&ctx(&["git"]), ".", body, "target/gates")
            .expect("the judge reads nothing from disk here")
            .0
            .into_iter()
            .map(|(_, w, _)| w)
            .collect()
    }

    // spec: canon-kit/SPEC.md §check-fence-run — the narrowing: only the fence-run program set, the
    // binary under a fence-safe arm, and no tracked script
    #[test]
    fn the_static_pass_narrows_the_command_head_rules() {
        assert!(reds("git status\ntrue\nf() { :; }\nf\n").is_empty());
        assert_eq!(reds("curl https://example.invalid\n"), vec!["curl"]);
        assert_eq!(reds("bin/run.sh\n"), vec!["bin/run.sh"]);
        assert!(reds("\"$gates\" --run-gate-tests a b\n\"$gates\" --emit knob-roster\n").is_empty());
        assert_eq!(reds("\"$gates\" --usage-poll\n"), vec!["$gates"]);
        assert_eq!(reds("\"$gates\" --emit no-such-arm\n"), vec!["$gates"]);
        assert_eq!(reds("\"$gates\"\n"), vec!["$gates"]);
        assert!(reds("\"$gates\" check-md-refs\n").is_empty());
        assert_eq!(reds("target/gates --pack-installer\n"), vec!["target/gates"]);
        assert!(reds("./target/gates --list\n").is_empty());
    }

    // spec: canon-kit/SPEC.md §check-fence-run — a builtin that runs its operand is refused, since
    // the operand is a command the head scan never sees; `command -v` stays a probe
    #[test]
    fn a_builtin_running_its_operand_is_refused() {
        assert_eq!(reds("exec curl x\n"), vec!["exec"]);
        assert_eq!(reds("eval \"curl x\"\n"), vec!["eval"]);
        assert_eq!(reds("command curl x\n"), vec!["command"]);
        assert!(reds("command -v git\ngit status  # command runs here\n").is_empty());
    }
}
