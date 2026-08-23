// spec: gate-sdk/SPEC.md §run-gates — the battery runner: the registry walk, the two selectors,
// the timings, the omission accounting, the output contract and the worker pool. `bin/run-gates.sh`
// is the front-end that resolves one bridged environment and execs this arm.
use crate::gates;
use crate::proc;
use crate::registry;
use crate::walk;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

// spec: gate-sdk/SPEC.md §run-gates — the refusals keep the front-end's own name, because the
// message shapes are the tool's documented surface and the arm is what the front-end exec'd
const TOOL: &str = "run-gates";

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's own bridged reads. The roster `--knobs --run`
// prints is the *union* of these with every registry member's and every sibling arm's, derived in
// `emit::knobs` rather than maintained here.
pub const KNOBS: &[&str] = &[
    "GATE_SDK_GATES_DIR",
    "GATE_KIT_ROOTS_HERE",
    "GATE_KIT_ROOTS_REL",
    "GATE_SDK_TMP_DIR",
];

// spec: gate-sdk/SPEC.md §run-gates — one selected member: the name, and the staged-mode positional
// arguments `--for` hands it (empty for every member under `--only` and under a bare run)
struct Selected {
    name: String,
    args: Vec<String>,
}

// spec: gate-sdk/SPEC.md §run-gates — one member's finished run, buffered so the flush can be in
// registry order rather than completion order
struct Outcome {
    tail: String,
    output: Vec<u8>,
    failed: bool,
    ms: u128,
}

// spec: gate-sdk/SPEC.md §run-gates — the front-end's parsed selection, the one argv contract
// between the two halves; the user-facing grammar is the front-end's and is unchanged by it
struct Args {
    gates_dir: Option<String>,
    only: Vec<String>,
    paths: Vec<String>,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut out = Args {
        gates_dir: None,
        only: Vec::new(),
        paths: Vec::new(),
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--gates-dir" => {
                i += 1;
                match args.get(i) {
                    Some(d) => out.gates_dir = Some(d.clone()),
                    None => return Err("--gates-dir needs a directory".to_string()),
                }
                i += 1;
            }
            "--only" => {
                out.only = args[i + 1..].to_vec();
                break;
            }
            "--for" => {
                out.paths = args[i + 1..].to_vec();
                break;
            }
            other => return Err(format!("unrecognized argument: {}", other)),
        }
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the member set the declared-knob union is scoped to:
// the `gates.list` the caller named. An unreadable or unnamed registry yields nothing, so the run
// itself is what produces the `no registry at` refusal.
pub fn registered_members(args: &[String]) -> Vec<String> {
    let Ok(a) = parse(args) else { return Vec::new() };
    let Some(dir) = a.gates_dir else {
        return Vec::new();
    };
    match std::fs::read_to_string(registry::list_path(&dir)) {
        Ok(t) => registry::members(&t),
        Err(_) => Vec::new(),
    }
}

// spec: gate-sdk/SPEC.md §run-gates — worker count: `GATE_SDK_JOBS` where the environment carries a
// usable one, else the machine's parallelism, else serial. Read once, before the first dispatch.
fn jobs() -> usize {
    if let Ok(v) = std::env::var("GATE_SDK_JOBS") {
        if let Ok(n) = v.trim().parse::<usize>() {
            if n >= 1 {
                return n;
            }
        }
    }
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

// spec: gate-sdk/SPEC.md §run-gates — `pathspec_matches`: a `mode=staged` member's hook branch
// selects by git pathspec, the exact path or a subtree under it, which is a second mechanism beside
// the glob matcher and is reproduced rather than folded into it
fn pathspec_matches(p: &str, globs: &[&str]) -> bool {
    globs
        .iter()
        .any(|g| walk::pattern_match(g, p) || walk::pattern_match(&format!("{}/*", g), p))
}

// spec: gate-sdk/SPEC.md §run-gates — `gate_staged_matches`: bash's `[[ "$f" == $pat ]]` over the
// trigger globs, the matcher the generated hook's `staged_matches` splices from the same body
fn staged_matches(p: &str, globs: &[&str]) -> bool {
    globs.iter().any(|g| walk::pattern_match(g, p))
}

fn manifest(src: &str) -> Vec<(String, String)> {
    let body = std::fs::read_to_string(src).unwrap_or_default();
    registry::manifest_line(&body)
        .map(registry::manifest_fields)
        .unwrap_or_default()
}

// spec: gate-sdk/SPEC.md §run-gates — `--for` selection: every member whose effective trigger
// (`trigger=` else `couples=`, kit-expanded) matches one of the given repo-relative paths, exactly
// as the generated hook would; an uncovered path is a note on stdout, never a failure.
fn select_for(
    members: &[String],
    resolve_dirs: &[String],
    kit_roots_rel: &[String],
    paths: &[String],
) -> Result<Vec<Selected>, i32> {
    let mut covered = vec![false; paths.len()];
    let mut run: Vec<Selected> = Vec::new();
    for name in members {
        let src = match registry::resolve(name, resolve_dirs) {
            Some(s) => s,
            None => {
                eprintln!(
                    "{}: --for cannot resolve '{}' in: {}",
                    TOOL,
                    name,
                    resolve_dirs.join(" ")
                );
                return Err(2);
            }
        };
        let f = manifest(&src);
        let couples = registry::field(&f, "couples");
        let mut trigger = registry::field(&f, "trigger");
        if trigger.is_empty() {
            trigger = couples;
        }
        let trigger = registry::expand_couples(&trigger, kit_roots_rel);
        let mode = registry::field(&f, "mode");
        let globs: Vec<&str> = trigger.split(',').filter(|g| !g.is_empty()).collect();
        if trigger == "*" {
            for c in covered.iter_mut() {
                *c = true;
            }
            run.push(Selected {
                name: name.clone(),
                args: Vec::new(),
            });
            continue;
        }
        if mode == "staged" {
            let mut matched: Vec<String> = Vec::new();
            for (i, p) in paths.iter().enumerate() {
                if pathspec_matches(p, &globs) {
                    matched.push(p.clone());
                    covered[i] = true;
                }
            }
            if !matched.is_empty() {
                run.push(Selected {
                    name: name.clone(),
                    args: matched,
                });
            }
            continue;
        }
        let mut hit = false;
        for (i, p) in paths.iter().enumerate() {
            if staged_matches(p, &globs) {
                hit = true;
                covered[i] = true;
            }
        }
        if hit {
            run.push(Selected {
                name: name.clone(),
                args: Vec::new(),
            });
        }
    }
    for (i, p) in paths.iter().enumerate() {
        if !covered[i] {
            println!("{}: no registered gate couples to {}", TOOL, p);
        }
    }
    Ok(run)
}

// spec: gate-sdk/SPEC.md §run-gates — `--only` selection: set-shaped and registry-ordered, so two
// names give one transcript whichever way they were typed; an unregistered name is a refusal
// because a name is a claim about the registry, never a fact about the tree
fn select_only(members: &[String], list: &str, only: &[String]) -> Result<Vec<Selected>, i32> {
    for n in only {
        if !members.iter().any(|m| m == n) {
            eprintln!("{}: --only: '{}' is not registered in {}", TOOL, n, list);
            return Err(2);
        }
    }
    Ok(members
        .iter()
        .filter(|m| only.iter().any(|n| n == *m))
        .map(|m| Selected {
            name: m.clone(),
            args: Vec::new(),
        })
        .collect())
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the child's declared knob environment, built by *filtering*
// the union the front-end resolved: a member receives the `GATE_SDK_KNOB_*` variables its own
// registry entry declares and no others, which is what keeps the declared-knob discipline executed.
fn child_knobs(declared: &[&str], union: &[(String, String)]) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for d in declared {
        match d.strip_suffix('*') {
            Some(stem) => {
                let want = format!("GATE_SDK_KNOB_{}", stem);
                for (k, v) in union {
                    if k.starts_with(&want) {
                        out.push((k.clone(), v.clone()));
                    }
                }
            }
            None => {
                let want = format!("GATE_SDK_KNOB_{}", d);
                if let Some((k, v)) = union.iter().find(|(k, _)| *k == want) {
                    out.push((k.clone(), v.clone()));
                }
            }
        }
    }
    out
}

struct Dispatch<'a> {
    resolve_dirs: &'a [String],
    self_exe: &'a str,
    list: &'a str,
    union: &'a [(String, String)],
    union_names: &'a [String],
    scratch: &'a Path,
}

// spec: gate-sdk/SPEC.md §run-gates — one member, run as a child process; the in-process call is
// refused there, on the declared-knob discipline, fault isolation and the surviving `.sh` members
fn dispatch_one(d: &Dispatch, idx: usize, sel: &Selected) -> Outcome {
    let started = Instant::now();
    let fail = |tail: &str, body: String, ms: u128| Outcome {
        tail: format!("  FAIL: {} ({})", sel.name, tail),
        output: body.into_bytes(),
        failed: true,
        ms,
    };
    let src = match registry::resolve(&sel.name, d.resolve_dirs) {
        Some(s) => s,
        None => {
            return fail(
                "unresolved",
                format!(
                    "{} listed in {} but resolves in none of: {}",
                    sel.name,
                    d.list,
                    d.resolve_dirs.join(" ")
                ),
                started.elapsed().as_millis(),
            )
        }
    };
    let (argv, knobs) = if src.ends_with(".gate") {
        let declared = match gates::knobs(&sel.name) {
            Some(k) => k,
            None => {
                return fail(
                    "dispatch harness error, exit 2",
                    format!(
                        "checkwright-gates: no such gate subcommand: {} — the check could not run; \
                         treating as failure (not clean)",
                        sel.name
                    ),
                    started.elapsed().as_millis(),
                )
            }
        };
        (
            vec![d.self_exe.to_string(), sel.name.clone()],
            child_knobs(declared, d.union),
        )
    } else {
        (vec![src], Vec::new())
    };
    let mut full = argv;
    full.extend(sel.args.iter().cloned());

    let tmpdir = d.scratch.join(format!("t{}", idx));
    if let Err(e) = std::fs::create_dir_all(&tmpdir) {
        return fail(
            "dispatch harness error, exit 2",
            format!(
                "cannot create the per-gate scratch {}: {} — the check could not run; treating as \
                 failure (not clean)",
                tmpdir.display(),
                e
            ),
            started.elapsed().as_millis(),
        );
    }
    let capture = d.scratch.join(format!("c{}", idx));
    match proc::dispatch(&full, &knobs, d.union_names, &tmpdir, &capture) {
        Err(e) => fail(
            "dispatch harness error, exit 2",
            e,
            started.elapsed().as_millis(),
        ),
        Ok(done) => {
            let ms = started.elapsed().as_millis();
            if done.code == 0 {
                Outcome {
                    tail: format!("  PASS: {}", sel.name),
                    output: done.output,
                    failed: false,
                    ms,
                }
            } else {
                Outcome {
                    tail: format!("  FAIL: {} (exit {})", sel.name, done.code),
                    output: done.output,
                    failed: true,
                    ms,
                }
            }
        }
    }
}

// spec: gate-sdk/SPEC.md §run-gates — a declared omission keeps `All N gates passed.` honest as the
// roster-collapse tripwire, reported on its own line beside the summary and never inside it
fn report_omissions(list_text: &str) {
    let mut reasons: Vec<&str> = list_text
        .lines()
        .filter_map(|l| {
            let mut f = l.split_whitespace();
            match (f.next(), f.next(), f.next(), f.next()) {
                (Some("#"), Some("omitted:"), Some(_), Some(r)) => Some(r),
                _ => None,
            }
        })
        .collect();
    reasons.sort_unstable();
    let mut i = 0;
    while i < reasons.len() {
        let r = reasons[i];
        let n = reasons[i..].iter().take_while(|x| **x == r).count();
        match r {
            "substrate-unavailable" => println!(
                "{} gate(s) omitted ({}): no prebuilt gate binary is published for this platform.",
                n, r
            ),
            "digest-unverifiable" => println!(
                "{} gate(s) omitted ({}): install sha256sum or shasum, then re-run checkwright init.",
                n, r
            ),
            _ => println!("{} gate(s) omitted ({}).", n, r),
        }
        i += n;
    }
}

fn write_timings(path: &Path, order: &[Selected], outcomes: &[Outcome]) {
    let mut body = String::new();
    let mut total: u128 = 0;
    for (s, o) in order.iter().zip(outcomes) {
        body.push_str(&format!("{} {}\n", s.name, o.ms));
        total += o.ms;
    }
    body.push_str(&format!("TOTAL {}\n", total));
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(path, body);
}

pub fn run(args: &[String]) -> i32 {
    let parsed = match parse(args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return 2;
        }
    };
    let knob = |n: &str| walk::knob_scalar(n);
    let configured_dir = match knob("GATE_SDK_GATES_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return 2;
        }
    };
    let gates_dir = parsed.gates_dir.clone().unwrap_or_else(|| configured_dir.clone());
    let explicit = gates_dir != configured_dir;
    let list = registry::list_path(&gates_dir);

    let list_text = match std::fs::read_to_string(&list) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("{}: no registry at {}", TOOL, list);
            if explicit {
                steer(&gates_dir, &configured_dir);
            }
            return 2;
        }
    };
    let members = registry::members(&list_text);
    if members.is_empty() {
        eprintln!("{}: {} names no gates", TOOL, list);
        return 2;
    }

    let kit_roots = match walk::kit_roots() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return 2;
        }
    };
    let resolve_dirs = registry::resolve_dirs(&gates_dir, &kit_roots);

    let selected: Vec<Selected> = if !parsed.only.is_empty() {
        match select_only(&members, &list, &parsed.only) {
            Ok(s) => s,
            Err(c) => return c,
        }
    } else if !parsed.paths.is_empty() {
        let kit_roots_rel = match walk::kit_roots_rel() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}", TOOL, e);
                return 2;
            }
        };
        let s = match select_for(&members, &resolve_dirs, &kit_roots_rel, &parsed.paths) {
            Ok(s) => s,
            Err(c) => return c,
        };
        if s.is_empty() {
            print!("\n===== gates summary =====\nno coupled gate for the given path(s); nothing to run.\n");
            return 0;
        }
        s
    } else {
        members
            .iter()
            .map(|m| Selected {
                name: m.clone(),
                args: Vec::new(),
            })
            .collect()
    };

    let tmp_dir = match knob("GATE_SDK_TMP_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return 2;
        }
    };
    // spec: gate-sdk/SPEC.md §run-gates — the run's own scratch: under the *system* temp dir, which
    // is where an anonymous temporary already went, and absolute because a child's cwd is its own
    // and a relative `TMPDIR` would resolve somewhere else in it. Removed on the way out.
    let scratch = std::env::temp_dir().join(format!("checkwright-run.{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    if let Err(e) = std::fs::create_dir_all(&scratch) {
        eprintln!(
            "{}: cannot create the run scratch {}: {} — treating as failure (not clean)",
            TOOL,
            scratch.display(),
            e
        );
        return 2;
    }

    let self_exe = match std::env::current_exe() {
        Ok(p) => p.display().to_string(),
        Err(e) => {
            eprintln!(
                "{}: cannot resolve this binary's own path: {} — treating as failure (not clean)",
                TOOL, e
            );
            let _ = std::fs::remove_dir_all(&scratch);
            return 2;
        }
    };

    let union: Vec<(String, String)> = std::env::vars()
        .filter(|(k, _)| k.starts_with("GATE_SDK_KNOB_"))
        .collect();
    let union_names: Vec<String> = union.iter().map(|(k, _)| k.clone()).collect();

    let d = Dispatch {
        resolve_dirs: &resolve_dirs,
        self_exe: &self_exe,
        list: &list,
        union: &union,
        union_names: &union_names,
        scratch: &scratch,
    };

    let outcomes = dispatch_all(&d, &selected);
    let _ = std::fs::remove_dir_all(&scratch);

    write_timings(
        &PathBuf::from(&tmp_dir).join("gate-timings.txt"),
        &selected,
        &outcomes,
    );

    let verbose = std::env::var("GATE_SDK_VERBOSE").map(|v| !v.is_empty()).unwrap_or(false);
    let stdout = std::io::stdout();
    let mut w = stdout.lock();
    for (s, o) in selected.iter().zip(&outcomes) {
        if !o.failed && !verbose {
            continue;
        }
        let _ = write!(w, "\n===== {} =====\n", s.name);
        let body = trim_trailing_newlines(&o.output);
        if !body.is_empty() {
            let _ = w.write_all(body);
            let _ = w.write_all(b"\n");
        }
        let _ = writeln!(w, "{}", o.tail);
    }
    let _ = write!(w, "\n===== gates summary =====\n");
    let _ = w.flush();
    report_omissions(&list_text);
    let failed: Vec<&str> = selected
        .iter()
        .zip(&outcomes)
        .filter(|(_, o)| o.failed)
        .map(|(s, _)| s.name.as_str())
        .collect();
    if failed.is_empty() {
        println!("All {} gates passed.", selected.len());
        return 0;
    }
    println!(
        "{} of {} gates FAILED: {}",
        failed.len(),
        selected.len(),
        failed.join(" ")
    );
    1
}

fn trim_trailing_newlines(b: &[u8]) -> &[u8] {
    let mut end = b.len();
    while end > 0 && b[end - 1] == b'\n' {
        end -= 1;
    }
    &b[..end]
}

// spec: gate-sdk/SPEC.md §run-gates — the `--only` steer: a positional that is really a member of
// the *default* registry earns the remedy beside the refusal, never a run it did not ask for
fn steer(arg: &str, configured_dir: &str) {
    let default_list = registry::list_path(configured_dir);
    let Ok(text) = std::fs::read_to_string(&default_list) else {
        return;
    };
    if registry::members(&text).iter().any(|m| m == arg) {
        eprintln!(
            "{}: '{}' is a gate registered in {}, not a gates dir — run it with: run-gates.sh --only {}",
            TOOL, arg, default_list, arg
        );
    }
}

// spec: gate-sdk/SPEC.md §run-gates — the worker pool: `std::thread` and `std::sync` only, no new
// crate dependency, since objective 4 makes footprint a cost paid per target on every adopter's
// machine. Members are claimed off one shared cursor, so a slow member cannot starve the queue.
fn dispatch_all(d: &Dispatch, selected: &[Selected]) -> Vec<Outcome> {
    let n = selected.len();
    let slots: Vec<Mutex<Option<Outcome>>> = (0..n).map(|_| Mutex::new(None)).collect();
    let cursor = Mutex::new(0usize);
    let workers = jobs().min(n.max(1));
    std::thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| loop {
                let i = {
                    let mut c = cursor.lock().expect("the battery's work cursor is poisoned");
                    if *c >= n {
                        return;
                    }
                    let i = *c;
                    *c += 1;
                    i
                };
                let outcome = dispatch_one(d, i, &selected[i]);
                *slots[i].lock().expect("a battery result slot is poisoned") = Some(outcome);
            });
        }
    });
    slots
        .into_iter()
        .enumerate()
        .map(|(i, m)| {
            m.into_inner()
                .expect("a battery result slot is poisoned")
                .unwrap_or_else(|| Outcome {
                    tail: format!("  FAIL: {} (dispatch harness error, exit 2)", selected[i].name),
                    output: b"the worker pool returned no result for this member".to_vec(),
                    failed: true,
                    ms: 0,
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a member receives the knobs its registry entry
    // declares and no others, prefix families included, out of the union the front-end resolved
    #[test]
    fn a_child_receives_only_the_knobs_its_entry_declares() {
        let union = vec![
            ("GATE_SDK_KNOB_A".to_string(), "1".to_string()),
            ("GATE_SDK_KNOB_B".to_string(), "2".to_string()),
            ("GATE_SDK_KNOB_P_ONE".to_string(), "3".to_string()),
            ("GATE_SDK_KNOB_P_TWO".to_string(), "4".to_string()),
        ];
        let got = child_knobs(&["A", "P_*"], &union);
        let names: Vec<&str> = got.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(
            names,
            vec!["GATE_SDK_KNOB_A", "GATE_SDK_KNOB_P_ONE", "GATE_SDK_KNOB_P_TWO"]
        );
    }

    // spec: gate-sdk/SPEC.md §run-gates — a `mode=staged` member matches by pathspec, the exact
    // path or a subtree under it, which the glob matcher beside it does not do
    #[test]
    fn a_pathspec_covers_its_own_subtree_and_a_glob_does_not() {
        assert!(pathspec_matches("docs/a/b.md", &["docs"]));
        assert!(pathspec_matches("docs", &["docs"]));
        assert!(!pathspec_matches("docsx/a.md", &["docs"]));
        assert!(!staged_matches("docs/a/b.md", &["docs"]));
        assert!(staged_matches("docs/a/b.md", &["docs/*"]));
    }
}
