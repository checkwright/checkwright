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

// spec: gate-sdk/SPEC.md §run-gates — the one usage text, the stdout body of a help request and the
// stderr body of an unrecognized-option refusal, per §The bin/-tool contract. It lives beside the
// arm table it describes, because it grows by a paragraph per bridged arm.
pub const USAGE: &str = r#"usage: run-gates.sh [gates-dir]                run every registered gate
       run-gates.sh --only <name> [<name>...]  run only the named gates
       run-gates.sh --only <name> -- <arg>...  run one named gate, forwarding <arg>...
       run-gates.sh --for <path> [<path>...]   run only gates coupling to those paths
       run-gates.sh --emit <arm> [args...]     dispatch a ported non-gate emitter arm
       run-gates.sh --hook <member>            dispatch a harness hook member, payload on stdin
       run-gates.sh --statusline               render the harness status line, payload on stdin
       run-gates.sh --usage-poll               refresh the usage snapshot from its source
       run-gates.sh --usage-verdict [paths]    budget verdict: 0 OK/RESET-OK, 1 PAUSE, 2 STALE
       run-gates.sh --lesson-sink <tag>        route a lesson body on stdin to its sink
       run-gates.sh --upgrade-smoke            prove the FROM->TO kit upgrade in scratch
       run-gates.sh --install-lifecycle [file] install the lifecycle resident surfaces
       run-gates.sh --install-hooks            wire this clone's core.hooksPath (per-clone opt-in)
       run-gates.sh --enter-stage <stage>      stamp a stage entry (or --rename an iteration)
       run-gates.sh --wait-probe <sub> [args]  the wait-primitive probe: 'sweep' is the reproducer
       run-gates.sh --run-validate             run the codified validate spine over the roster
       run-gates.sh --diff-baseline <group>... diff captured logs against the baseline slice
       run-gates.sh -h | --help                this text, on stdout, exit 0

  --only  runs the named members in registry order whatever order they were
          typed; duplicates collapse. A name unknown to the registry is a
          refusal, except that a *sole* name is also looked up in the check
          dirs and selected if a gate declares it there; a name found in
          neither is the refusal.
          The [gates-dir] positional is unavailable in this form — point
          GATE_SDK_GATES_DIR at another registry instead. A `--` separator ends
          the name list and forwards every remaining argument to the selected
          gate, which requires the selection to resolve to exactly one member:
          two or more with a `--` is a refusal, never a broadcast.
  --for   selects by coupling: every gate whose effective trigger matches one
          of the given repo-relative paths, exactly as the generated hook
          would. A path no gate couples to is a note, not a failure.
  --emit  dispatches the named non-gate arm of the native binary, handing it
          every remaining argument.
  --hook  dispatches the named harness hook member: the harness payload passes
          through on stdin, the hook-JSON envelope (where the member emits one)
          comes back on stdout, and the exit status is the harness's own
          allow/block signal. Where the binary is absent or its configuration
          cannot be resolved, this arm declines with a diagnostic on stderr and
          exit 0 rather than blocking every guarded tool call.
  --statusline  renders the status line for the harness's statusLine hook and
          rewrites the usage snapshot; declines like --hook when unavailable.
  --usage-poll  runs one poll cycle against the usage source and rewrites the
          snapshot. Its caller is a refresh command or a session rather than a
          gate on a tool call, so it refuses with exit 2 when unavailable.
  --usage-verdict  emits one budget verdict line on stdout from the usage
          snapshot: exit 0 OK / RESET-OK, 1 PAUSE, 2 STALE or unreadable
          (budget-unknown, which never blocks delegation). Two optional
          positionals override the snapshot and credentials paths for test
          injection; a path beginning with a dash is passed after `--`.
          Unavailable is exit 2, the same code an unreadable snapshot takes.
  --lesson-sink  reads a lesson body on stdin and runs the sink configured for
          <tag>, or appends to <workflow-dir>/<tag>-harvest.md when none is.
          The sink's exit status is this arm's, so a failing sink is visible to
          the close step that ran it; unavailable is exit 2 for the same reason.
  --upgrade-smoke  vendors every kit at GATE_SDK_UPGRADE_FROM into a scratch
          consumer, swaps them wholesale to GATE_SDK_UPGRADE_TO and asserts the
          sync is deterministic and the phase-B red set is declared. Takes no
          argument. Exit 0 clean with one UPGRADE-SMOKE line on stdout, 1 an
          upgrade finding, 2 a broken tag or environment; unavailable is 2.
  --install-lifecycle  writes the lifecycle registration block into the
          always-loaded agent file, the iteration-scoped merge attributes into
          .gitattributes, and the keep-ours merge driver into this clone's git
          config. The optional positional is the agent file to write into,
          overriding LIFECYCLE_KIT_AGENT_FILE. Idempotent; exit 2 when the agent
          file is absent or a marker pair is malformed, and unavailable is 2.
  --install-hooks  points this clone's core.hooksPath at the generated hooks dir,
          sets blame.ignoreRevsFile where that file exists, makes the hooks
          executable and runs check-identity once so a fresh clone learns of a
          wrong-identity mapping before its first commit. The gate is resolved
          through the registry, so a consumer shadow still wins; a consumer
          without it is skipped. Takes no argument. Exit 0 wired and verified,
          1 the identity gate's own finding, 2 no hooks dir or an
          uninterpretable manifest; unavailable is 2.
  --enter-stage  appends the invocation stamp that IS a stage transition, after
          running the entry pre-flight; `--simulate` runs everything up to the
          write and writes nothing, and `--rename <name>` renames the iteration
          across the queue header and column 1 of every stamp. Exit 0 a stamp or
          a reported no-op, 1 a refusal, 2 a usage or configuration error;
          unavailable is 2, because the caller is a stage session's first step
          whose failure must be visible and a silent 0 would let a session
          believe it had stamped.
  --wait-probe  stands known-duration producers up and measures candidate wait
          forms against them, one trial line per run. The subcommand is an
          operand: produce, waiter, arm-local, record, report, sweep — the
          roster prints on stderr at exit 2 on misuse. Exit 0 on a completed
          subcommand, 1 for `report` with no trials recorded, 2 on misuse;
          unavailable is 2. Hand-invoked, wired into no tier, and `sweep`
          sleeps for its declared durations.
  --run-validate  claims the producer-liveness lock, then runs each configured
          suite foreground, parses it, diffs the baseline slice per-scenario and
          folds one evidence line per suite into the tracked manifest after the
          whole roster has run. Takes no argument: the whole input is the
          bridged EVIDENCE_KIT_* environment. Exit 0 every suite clean, 1 a
          suite recorded new-failures, 2 the run could not start (no suites, no
          run key, absent manifest, a held or unclaimable lock, a missing suite
          command, a failing pre-hook, a parser producing no result is 1);
          unavailable is 2, because a caller that read a silent 0 would believe
          a run it never got had passed.
  --diff-baseline  parses each captured log named on argv and diffs it against
          the baseline's suite slice per-scenario, printing `new-failure` and
          `recovery` findings. Each argument group is `<suite> <logfile>
          [<status>]`, repeated; an `exit-code` suite must carry its status or
          the tool refuses rather than assuming success. A positional beginning
          with a dash is a refusal, and `--` ends option processing. Exit 0
          clean, 1 new failures against the baseline, 2 misuse or an unreadable
          log; unavailable is 2, its one functional caller being a CI leg that
          reads nothing but the status.
  --      ends option processing, so a gates-dir spelled with a leading dash
          is still reachable.

The battery itself is the binary's `--run` arm; run-gates.sh resolves its
bridged environment and execs it.

GATE_SDK_VERBOSE (any non-empty value) restores the per-gate banner roll the
quiet-green output contract suppresses; GATE_SDK_JOBS sets the worker count
(default: the machine's parallelism; 1 restores a serial run). Per-gate timings
land in $GATE_SDK_TMP_DIR/gate-timings.txt (default .tmp/)."#;

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's own bridged reads, plus the union sentinel:
// `--knobs --run` prints these with every registry member's added, expanded in `emit::knobs`. The
// sentinel is what expresses the dispatch union per member rather than per `Arm` variant.
pub const KNOBS: &[&str] = &[
    "GATE_SDK_GATES_DIR",
    "GATE_KIT_ROOTS_HERE",
    "GATE_KIT_ROOTS_REL",
    "GATE_SDK_TMP_DIR",
    crate::emit::EVERY_REGISTERED_KNOB,
];

// spec: gate-sdk/SPEC.md §run-gates — one selected member: the name, and the positional arguments
// the selection hands it — `--for`'s matching paths under a staged-mode member, and the argv after
// `--only`'s `--` separator under a single-member selection. Empty under a bare run.
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

// spec: gate-sdk/SPEC.md §run-gates — the whole user-facing argument grammar, which is this arm's
// after the front-end cut: the front-end resolves the gates dir and execs, and every other form
// lands here, beside the arm table and the usage text.
struct Args {
    gates_dir: Option<String>,
    only: Vec<String>,
    paths: Vec<String>,
    // spec: gate-sdk/SPEC.md §run-gates — `Some` exactly when a `--` separator followed `--only`,
    // which is what makes an empty forwarded vector distinguishable from no channel at all: the
    // separator is the caller's assertion that one gate is selected, and it is checked as one.
    only_args: Option<Vec<String>>,
    help: bool,
}

// spec: gate-sdk/SPEC.md §run-gates — a refusal decidable from argv alone, and whether the usage
// text is its body: an unrecognized option prints it (§The bin/-tool contract), an arity or shape
// refusal names the argument instead, because a usage dump is not an answer to a well-formed flag.
struct Refusal {
    message: String,
    usage: bool,
}

fn plain(message: &str) -> Refusal {
    Refusal {
        message: message.to_string(),
        usage: false,
    }
}

fn unrecognized(option: &str) -> Refusal {
    Refusal {
        message: format!("unrecognized option: {}", option),
        usage: true,
    }
}

fn parse(args: &[String]) -> Result<Args, Refusal> {
    let mut out = Args {
        gates_dir: None,
        only: Vec::new(),
        paths: Vec::new(),
        only_args: None,
        help: false,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                out.help = true;
                return Ok(out);
            }
            "--gates-dir" => {
                i += 1;
                match args.get(i) {
                    Some(d) => out.gates_dir = Some(d.clone()),
                    None => return Err(plain("--gates-dir needs a directory")),
                }
                i += 1;
            }
            // spec: gate-sdk/SPEC.md §run-gates — the emitter front-end's arity refusal, kept at its
            // own message rather than collapsed into the unrecognized-option one: `--emit` is a
            // recognized flag missing its operand, and the two are different mistakes
            "--emit" => return Err(plain("--emit needs an arm name")),
            "--only" => {
                let mut j = i + 1;
                while j < args.len() && args[j] != "--" {
                    // spec: gate-sdk/SPEC.md §run-gates — a name beginning with '-' is an
                    // unrecognized option wherever it stands, so `--only --for` refuses at the name
                    // instead of taking it for a gate and reporting it unregistered
                    if args[j].starts_with('-') {
                        return Err(unrecognized(&args[j]));
                    }
                    out.only.push(args[j].clone());
                    j += 1;
                }
                if out.only.is_empty() {
                    return Err(plain("--only needs at least one gate name"));
                }
                if j < args.len() {
                    out.only_args = Some(args[j + 1..].to_vec());
                }
                return Ok(out);
            }
            "--for" => {
                out.paths = args[i + 1..].to_vec();
                if out.paths.is_empty() {
                    return Err(plain("--for needs at least one path"));
                }
                return Ok(out);
            }
            // spec: gate-sdk/SPEC.md §run-gates — `--` ends option processing, so a gates-dir
            // legitimately spelled with a leading dash stays reachable
            "--" => {
                if let Some(d) = args.get(i + 1) {
                    out.gates_dir = Some(d.clone());
                }
                return Ok(out);
            }
            other if other.starts_with('-') => return Err(unrecognized(other)),
            other => {
                out.gates_dir = Some(other.to_string());
                return Ok(out);
            }
        }
    }
    Ok(out)
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
// names give one transcript whichever way they were typed; a *sole* name the registry omits
// resolves against the check dirs, and a name resolving nowhere is the refusal
fn select_only(
    members: &[String],
    list: &str,
    resolve_dirs: &[String],
    only: &[String],
    forwarded: Option<&[String]>,
) -> Result<Vec<Selected>, i32> {
    if only.len() == 1 && !members.iter().any(|m| m == &only[0]) {
        if registry::resolve(&only[0], resolve_dirs).is_none() {
            eprintln!("{}: --only: '{}' is not registered in {}", TOOL, only[0], list);
            return Err(2);
        }
        return Ok(vec![Selected {
            name: only[0].clone(),
            args: forwarded.unwrap_or(&[]).to_vec(),
        }]);
    }
    for n in only {
        if !members.iter().any(|m| m == n) {
            eprintln!("{}: --only: '{}' is not registered in {}", TOOL, n, list);
            return Err(2);
        }
    }
    let picked: Vec<&String> = members
        .iter()
        .filter(|m| only.iter().any(|n| n == *m))
        .collect();
    // spec: gate-sdk/SPEC.md §run-gates — the `--` channel is single-member-or-refuse, and the check
    // sits here rather than in the parser because only the registry resolves the cardinality the
    // bound is about.
    if forwarded.is_some() && picked.len() != 1 {
        eprintln!(
            "{}: --only: a '--' separator forwards its arguments to one selected gate, and this selection resolves to {}: {}",
            TOOL,
            picked.len(),
            picked.iter().map(|m| m.as_str()).collect::<Vec<_>>().join(" ")
        );
        eprintln!(
            "  help: name exactly one gate before the '--'; broadcasting one argument vector across a selection is refused, not narrowed"
        );
        return Err(2);
    }
    let args = forwarded.unwrap_or(&[]);
    Ok(picked
        .into_iter()
        .map(|m| Selected {
            name: m.clone(),
            args: args.to_vec(),
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
            eprintln!("{}: {}", TOOL, e.message);
            if e.usage {
                eprintln!("{}", USAGE);
            }
            return 2;
        }
    };
    // spec: gate-sdk/SPEC.md §The bin/-tool contract — a help request is the usage text on stdout at
    // exit 0, and it is answered before any knob is read: the text describes the arm roster, which is
    // a property of the binary rather than of the tree it was pointed at
    if parsed.help {
        println!("{}", USAGE);
        return 0;
    }
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

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged roots cross spelled relative to the invoking
    // directory and are re-absolutised here, which is how the reader recovers exactly the paths the
    // shell library computed — and what keeps the two dispatchers' unresolved-member report equal
    let kit_roots = match walk::kit_roots_abs() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return 2;
        }
    };
    let resolve_dirs = registry::resolve_dirs(&gates_dir, &kit_roots);

    let selected: Vec<Selected> = if !parsed.only.is_empty() {
        match select_only(&members, &list, &resolve_dirs, &parsed.only, parsed.only_args.as_deref()) {
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

    // spec: gate-sdk/SPEC.md §The port-candidate criteria — the criterion-6 discharge for the
    // `staged_matches` twin the port created: one canned corpus of glob/path pairs put to
    // `gate_staged_matches` and to this matcher, verdicts compared byte for byte.
    #[test]
    fn the_staged_matcher_agrees_with_the_shell_library_on_a_canned_corpus() {
        let globs: &[&[&str]] = &[
            &["docs/*.md"],
            &["docs/*"],
            &["*.md"],
            &["*"],
            &["kit/**/x.rs"],
            &["a?c/*.txt"],
            &["[ab]lpha/*"],
            &["[!ab]lpha/*"],
            &["docs/index.md"],
            &["scripts/*.sh", "kit/*.sh"],
            &["docs/*.md", "nothing/at/all"],
        ];
        let paths = [
            "docs/index.md",
            "docs/a/b.md",
            "docs",
            "docsx/index.md",
            "README.md",
            "kit/deep/x.rs",
            "kit/x.rs",
            "abc/one.txt",
            "ac/one.txt",
            "alpha/one",
            "blpha/one",
            "clpha/one",
            "scripts/run.sh",
            "kit/run.sh",
            "",
        ];

        let mut corpus = String::new();
        let mut mine: Vec<bool> = Vec::new();
        for g in globs {
            for p in paths {
                corpus.push('P');
                corpus.push_str(p);
                for one in *g {
                    corpus.push('\t');
                    corpus.push_str(one);
                }
                corpus.push('\n');
                mine.push(staged_matches(p, g));
            }
        }

        let lib = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("gate-sdk/lib/gate.sh");
        // spec: gate-sdk/SPEC.md §The port-candidate criteria — each record carries a `P` sentinel
        // ahead of the path, because `read` will not hand back an empty leading field and the empty
        // path is a corpus row the two matchers must be compared on like any other
        let script = concat!(
            "source \"$1\"; ",
            "while IFS=$'\\t' read -ra f; do ",
            "  p=\"${f[0]#P}\"; staged_all=(\"$p\"); ",
            "  if gate_staged_matches \"${f[@]:1}\"; then echo 1; else echo 0; fi; ",
            "done"
        );
        let out = crate::proc::run_with_stdin(
            "bash",
            &["-c", script, "bash", &lib.display().to_string()],
            corpus.as_bytes(),
        )
        .expect("cannot run the shell matcher");
        let body = out
            .stdout()
            .expect("the shell matcher exited non-zero over the canned corpus");
        let theirs: Vec<bool> = String::from_utf8_lossy(body)
            .lines()
            .map(|l| l.trim() == "1")
            .collect();

        assert_eq!(
            theirs.len(),
            mine.len(),
            "the shell matcher answered {} of {} corpus rows",
            theirs.len(),
            mine.len()
        );
        let mut i = 0;
        for g in globs {
            for p in paths {
                assert_eq!(
                    mine[i], theirs[i],
                    "the two staged matchers disagree on path {:?} against globs {:?}:                      the crate says {} and gate_staged_matches says {}",
                    p, g, mine[i], theirs[i]
                );
                i += 1;
            }
        }
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
