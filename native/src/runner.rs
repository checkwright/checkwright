// spec: gate-sdk/SPEC.md §run-gates — the battery runner: the registry walk, the two selectors,
// the timings, the omission accounting, the output contract and the worker pool. `bin/run-gates.sh`
// is the front-end that locates the binary and execs this arm.
use crate::exit;
use crate::gates;
use crate::proc;
use crate::provenance::Provenance;
use crate::registry;
use crate::sarif;
use crate::walk;
use serde_json::{json, Map, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

// spec: gate-sdk/SPEC.md §run-gates — the refusals keep the front-end's own name, because the
// message shapes are the tool's documented surface and the arm is what the front-end exec'd
const TOOL: &str = "run-gates";

// spec: gate-sdk/SPEC.md §run-gates — the one usage text, the stdout body of a help request and the
// stderr body of an unrecognized-option refusal, per §The bin/-tool contract. It lives beside the
// arm table it describes, because it grows by a paragraph per configured arm.
pub const USAGE: &str = r#"usage: --run [--json] [--gates-dir <dir>]      run every registered gate
       --only <name> [<name>...]               run only the named gates
       --only <name> -- <arg>...               run one named gate, forwarding <arg>...
       --for <path> [<path>...]                run only gates coupling to those paths
       --emit <arm> [args...]                  dispatch a ported non-gate emitter arm
       --hook <member> [--observe] [--binding <name>] [--knobs <absolute file>]  dispatch a harness hook member, payload on stdin
       --statusline                            render the harness status line, payload on stdin
       --usage-poll                            refresh the usage snapshot from its source
       --usage-verdict [paths]                 budget verdict: 0 OK/RESET-OK, 1 PAUSE, 2 STALE
       --model-verdict [--expect <class>] [transcript]  running-model verdict: 0 READ/OK, 1 BELOW/UNBOUND, 2 UNKNOWN
       --foreign-run <adapter> <prompt-file> [--mode audit|sweep] [--class <class>] [--sandbox <policy>] [--key <key>]  one unit on a foreign adapter: 0 OK, 1 REFUSED, 2 FAILED
       --foreign-run <adapter> --budget        the adapter's budget verdict, no spawn: 0 OK/RESET-OK, 1 PAUSE, 2 STALE/OFF
       --foreign-resume <key> <prompt-file> | <key> --close  the next turn of a foreign session, or its end: 0 OK/CLOSED, 1 REFUSED, 2 FAILED
       --lesson-sink <tag>                     route a lesson body on stdin to its sink
       --queue <verb> <slug> [args]            move or stamp one queue entry, then run the queue's gates
       --upgrade-smoke                         prove the FROM->TO kit upgrade in scratch
       --run-config-variant <dir> [--keep]     rehearse a config change: the gates it moves
       --run-consumer-smoke [args]             prove the kits install into a scratch consumer
       --install-lifecycle [file]              install the lifecycle resident surfaces
       --install-hooks [--refresh]             place this clone's git hooks (per-clone opt-in)
       --enter-stage <stage>                   stamp a stage entry (or --rename an iteration)
       --wait-probe <sub> [args]               the wait-primitive probe: 'sweep' is the reproducer
       --run-validate                          run the codified validate spine over the roster
       --diff-baseline <group>...              diff captured logs against the baseline slice
       --verify-gate-run [--json] [<file> | -] judge a gate-run Statement against this tree
       --import-gate-run [<file> | -]          verify a Statement and record its suites
       --verdict-lookup <tree> <kit-ver> <digest>  the verdict a run stored under that key
       --run -h | --help                      this text, on stdout, exit 0

  --run   exits 0 pass, every member green; 1 fail, a member red; 3 error, a
          member that could not judge the tree with none red. Fail outranks
          error. A request the runner refuses (an argument, a knob, an
          unknown gate) is 2, and so is a run outside a git repository. Every
          form below states its own statuses; an arm's "unavailable" is the
          front end's absent binary, exit 3.
  --gates-dir  names the registry to run in place of GATE_SDK_GATES_DIR. A
          bare <dir> after the leading options is the same request, kept as
          an accepted alias.
  --only  runs the named members in registry order whatever order they were
          typed; duplicates collapse. A name unknown to the registry is a
          refusal, except that a *sole* name is also looked up in the check
          dirs and selected if a gate declares it there; a name found in
          neither is the refusal.
          The bare <dir> alias is unavailable in this form, since every token
          after --only is a name — lead with --gates-dir <dir>. A `--` separator ends
          the name list and forwards every remaining argument to the selected
          gate, which requires the selection to resolve to exactly one member:
          two or more with a `--` is a refusal, never a broadcast.
  --for   selects by coupling: every gate whose effective trigger matches one
          of the given repo-relative paths. The placed hooks' --git-hook
          arm calls the same selector. A path no gate couples to is a note,
          not a failure.
  --json  leads the other options (--run --json --only <name>...) and prints
          the run's verdict as one versioned JSON object on stdout, written
          last: its verdict, scope, provenance, each member's kind and
          status, and the declared omissions. The transcript moves to stderr
          and the exit status is unchanged; a refused or failed run prints
          the error document instead.
  --emit  dispatches the named non-gate arm of the native binary, handing it
          every remaining argument.
  --hook  dispatches the named harness hook member: the harness payload passes
          through on stdin, the hook-JSON envelope (where the member emits one)
          comes back on stdout, and the exit status is the harness's own
          allow/block signal. Where the binary is absent or its configuration
          cannot be resolved, this arm declines through a hook envelope and
          exit 0 rather than blocking every guarded tool call. With --observe
          it runs the member as a child, logs the decision it would have made
          to GATE_SDK_HOOK_OBSERVE_LOG, and exits 0 with nothing said; any
          other trailing token outside the grammar is refused at exit 2.
  --statusline  renders the status line for the harness's statusLine hook and
          rewrites the usage snapshot; declines like --hook when unavailable.
          Takes no argument, and names one on stderr and ignores it.
  --usage-poll  runs one poll cycle against the usage source and rewrites the
          snapshot. Its caller is a refresh command or a session rather than a
          gate on a tool call, so it exits 3 when unavailable.
          Takes no argument, and refuses one with exit 2.
  --usage-verdict  emits one budget verdict line on stdout from the usage
          snapshot: exit 0 OK / RESET-OK, 1 PAUSE, 2 STALE or unreadable
          (budget-unknown, which never blocks delegation). Two optional
          positionals override the snapshot and credentials paths for test
          injection; a path beginning with a dash is passed after `--`.
          Unavailable is exit 3, which a caller reads as STALE too.
  --model-verdict  emits one line naming the model the session runs on, read
          off its transcript, and with --expect <class> judges it against the
          tier binding: exit 0 READ or OK, 1 BELOW or UNBOUND (off its
          expected tier), 2 UNKNOWN (unverified, never a refusal). The class
          is one of DELEGATION_KIT_TIER_CLASSES. The line carries the effort
          the session runs at, and an OK names one that is not the class's
          bound effort. The operand names the transcript by path or
          eight-character session id; bare, it takes the delegation-aware
          pick. Unavailable is exit 3.
  --foreign-run  runs one read-only audit or mechanical sweep on the adapter
          DELEGATION_KIT_FOREIGN_ADAPTERS configures, in a scratch clone of
          committed HEAD, and emits one verdict line: exit 0 OK, 1 REFUSED
          (the agent committed, or an audit wrote), 2 FAILED. The report, and a
          sweep's patch, land under <tmp-dir>/foreign/<key>/. An adapter
          DELEGATION_KIT_FOREIGN_RESUME gives a resume form keeps its clone on
          OK as an open session. An adapter DELEGATION_KIT_FOREIGN_HARNESS
          names requires --class and runs the model and effort its harness
          binds to that class in DELEGATION_KIT_FOREIGN_TIER_MODEL; any other
          adapter reads nothing of a class, and the line's class= says which
          happened. An adapter whose form carries @SANDBOX@ runs the word
          DELEGATION_KIT_FOREIGN_SANDBOX binds to the --sandbox policy, the
          mode word where none is named; any other adapter pins its
          confinement and refuses a named policy, and the line's sandbox=
          says which. The adapter's budget is read before the
          spawn, and a paused window spawns nothing: FAILED (budget: ...).
          With --budget it prints that budget verdict alone and spawns no
          adapter: exit 0 OK / RESET-OK, 1 PAUSE, 2 STALE or OFF
          (budget-unknown, which never blocks a run). Unavailable is exit 3.
  --foreign-resume  runs the next turn of an open session in its kept clone,
          the prompt file on stdin: exit 0 OK, 1 REFUSED (the session ends),
          2 FAILED (the session stays open). With --close it removes the clone
          and keeps every report: exit 0 CLOSED, 2 no open session.
          Unavailable is exit 3.
  --lesson-sink  reads a lesson body on stdin and runs the sink configured for
          <tag>, or appends to <workflow-dir>/<tag>-harvest.md when none is.
          The sink's exit status is this arm's, so a failing sink is visible to
          the close step that ran it; unavailable is exit 3 for the same reason.
  --queue  moves or stamps one entry of the queue file, then runs every gate
          coupling to that file as its post-check. The verb roster prints on
          stderr at exit 2 on misuse. Exit 0 written and green, 1 written and a
          post-check not green (its findings are the worklist before commit),
          2 nothing written; unavailable is 3.
  --upgrade-smoke  vendors every kit at GATE_SDK_UPGRADE_FROM into a scratch
          consumer, swaps them wholesale to GATE_SDK_UPGRADE_TO and asserts the
          sync is deterministic and the phase-B red set is declared. Takes no
          argument. Exit 0 clean with one UPGRADE-SMOKE line on stdout, 1 an
          upgrade finding, 2 a broken tag or environment; unavailable is 3.
  --run-config-variant  runs the battery twice over a scratch copy of the
          tree carrying its history, as committed and with the files in <dir>
          copied over their namesakes in the gates directory, and prints one
          row per gate whose verdict or output moves: red, green, moved,
          added, removed. `--keep` retains the scratch and prints its path.
          Exit 0 no gate newly red, 1 a gate the variant reds, 2 no
          comparison made (a refused <dir>, a scratch it could not build, a
          battery that could not run); unavailable is 3.
  --run-consumer-smoke  takes [--keep] [kit-root...]: vendors the named kit
          roots (default: every kit; gate-sdk always first) into a scratch
          consumer, runs each smoke/install.sh, asserts the battery green under
          zero config, accounts for every shipped but unregistered gate, and
          fires each smoke/violation.sh. `--keep` retains the scratch and
          prints its path. Exit 0 with one
          CONSUMER-SMOKE clean line, 1 an assertion failed, 2 usage, 3 an
          environment it could not stand up; unavailable is 3.
  --install-lifecycle  writes the lifecycle registration block into the
          always-loaded agent file, the iteration-scoped merge attributes into
          .gitattributes, and the keep-ours merge driver into this clone's git
          config. The optional positional is the agent file to write into,
          overriding LIFECYCLE_KIT_AGENT_FILE. Idempotent; exit 2 when the agent
          file is absent, 3 when a marker pair is malformed or a write fails,
          and unavailable is 3.
  --install-hooks  places the pre-commit and commit-msg hooks, each the gate
          binary under the hook's name, in gate-hooks under this clone's git
          directory and points core.hooksPath there, sets blame.ignoreRevsFile
          where that file exists, and runs check-identity once so a fresh clone
          learns of a wrong-identity mapping before its first commit. The gate
          is resolved through the registry, so a consumer shadow still wins; a
          consumer without it is skipped. Exit 0 placed and verified, 1 the
          identity gate's own finding, 2 or 3 each refusal gate-sdk/SPEC.md
          §install-hooks lists; unavailable is 3. With --refresh it only
          re-places the served hooks and a forwarding hook for each live hook
          in the recorded directory, of a clone already opted in, printing
          'install-hooks: refreshed <hooks-dir>'; in a clone not opted in, or
          off the work-tree top, it does nothing at exit 0. Takes no other
          argument.
  --enter-stage  appends the invocation stamp that IS a stage transition, after
          running the entry pre-flight; `--simulate` runs everything up to the
          write and writes nothing, `--rename <name>` renames the iteration
          across the queue header and column 1 of every stamp, and
          `--open-lead-journal` opens the lead journal under a heading keyed on
          the stage cursor and writes the session-role marker, stamping
          nothing. Exit 0 a stamp or
          a reported no-op, 1 a refusal, 2 a usage or configuration error, 3 a
          file it could not read or write; unavailable is 3, because the caller is a stage session's first step
          whose failure must be visible and a silent 0 would let a session
          believe it had stamped.
  --wait-probe  stands known-duration producers up and measures candidate wait
          forms against them, one trial line per run. The subcommand is an
          operand: produce, waiter, arm-local, record, report, sweep — the
          roster prints on stderr at exit 2 on misuse. Exit 0 on a completed
          subcommand, 1 for `report` with no trials recorded, 2 on misuse, 3 a
          read, write or spawn that failed; unavailable is 3. Hand-invoked, wired into no tier, and `sweep`
          sleeps for its declared durations.
  --run-validate  claims the producer-liveness lock, then runs each configured
          suite foreground, parses it, diffs the baseline slice per-scenario and
          folds one evidence line per suite into the tracked manifest after the
          whole roster has run. Takes no argument: the whole input is the
          EVIDENCE_KIT_* configuration. Exit 0 every suite clean, 1 a
          suite recorded new-failures, 2 a refused run (an argument, no
          suites, no run key, an absent manifest, a held lock, a missing suite
          command, a pre-hook that moved HEAD while a Statement is configured),
          3 a run that could not start or finish its writes (a manifest whose
          header is neither major, a lock it could not read, a failing
          pre-hook or spawn; a parser producing no result is 1); unavailable
          is 3, because a
          caller that read a silent 0 would believe a run it never got had
          passed. With EVIDENCE_KIT_STATEMENT_FILE set it also writes one
          in-toto Statement of the run there, and with
          EVIDENCE_KIT_STATEMENT_WRITE_CMD set it hands the Statement to that
          argv on stdin, the subject commit appended, after every other write;
          a store that fails is 3.
  --diff-baseline  parses each captured log named on argv and diffs it against
          the baseline's suite slice per-scenario, printing `new-failure` and
          `recovery` findings. Each argument group is `<suite> <logfile>
          [<status>]`, repeated; an `exit-code` suite must carry its status or
          the tool refuses rather than assuming success. A positional beginning
          with a dash is a refusal, and `--` ends option processing. Exit 0
          clean, 1 new failures against the baseline, 2 misuse or an absent
          log, 3 an unreadable log; unavailable is 3, its one functional caller being a CI leg that
          reads nothing but the status.
  --verify-gate-run  reads one gate-run Statement, bare or in a DSSE
          envelope, from <file>, from stdin for `-`, or from the stdout of the
          EVIDENCE_KIT_STATEMENT_CMD argv, and checks its shape, its subject
          against HEAD's ancestry, and each suite's binding, command and tree,
          then each EVIDENCE_KIT_VERIFY_REQUIRE word: `head` the subject is
          HEAD, `verdict` every suite clean. The DSSE signature is never checked. Exit 0 every check held, 1 a
          contradiction (one line per finding), 2 a refused request, 3 a
          Statement that could not be read or judged; unavailable is 3.
          Writes nothing. --json prints one versioned JSON object in place of
          the verdict line on exit 0 or 1, and the error document on exit 2
          or 3.
  --import-gate-run  takes the same input and checks, then records each suite
          as an origin=imported manifest line under this tree's run key,
          under the producer-liveness lock. Refused unless
          EVIDENCE_KIT_IMPORTED_RUNS is accept. Exit 0 recorded, 1 a
          contradiction with nothing recorded, 2 a refusal, 3 an input or
          tool error; unavailable is 3.
  --verdict-lookup  answers one key from the GATE_SDK_VERDICT_STORE directory
          with one JSON object on stdout, its verdict the stored pass or fail,
          or absent where no run stored that key. Reads no repository. Exit 0
          an answer, a stored fail included; 2 an operand of the wrong count
          or shape, or no absolute store set; 3 a record that could not be
          read or does not answer for its key; unavailable is 3.
  --      ends option processing, so a gates-dir spelled with a leading dash
          is still reachable.

Each form is an argument list for the gate binary, the battery its `--run`
arm; a leading --only or --for reaches that arm unnamed. The pre-build front
end takes the same forms, and a bare <dir> or -h there as well.

GATE_SDK_VERBOSE (any non-empty value) restores the per-gate banner roll the
quiet-green output contract suppresses; GATE_SDK_JOBS sets the worker count
(default: the machine's parallelism; 1 restores a serial run);
GATE_SDK_SARIF_FILE names a file the run also writes its verdict to as a SARIF
2.1.0 log, the exit status unchanged unless that file cannot be written (exit 3);
GATE_SDK_SARIF_RESULTS selects its results: all (default) or failures;
GATE_SDK_VERDICT_STORE names an absolute directory a registry run on a clean
tree keeps its pass or fail verdict in for --verdict-lookup (exit 3 when it
cannot be written).
Per-gate timings of an unfiltered run land in $GATE_SDK_TMP_DIR/gate-timings.txt (default .tmp/)."#;

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's own reads, plus the sentinel standing for
// the reads of every member the argv can dispatch
pub const KNOBS: &[&str] = &[
    "GATE_SDK_GATES_DIR",
    "GATE_SDK_KIT_DIRS",
    "GATE_SDK_TMP_DIR",
    "GATE_SDK_SPEC_BASE_URL",
    "GATE_SDK_POWERSHELL",
    crate::emit::EVERY_REGISTERED_KNOB,
];

// spec: gate-sdk/SPEC.md §run-gates — one selected member: the name, and the positional arguments
// the selection hands it — `--for`'s matching paths under a staged-mode member, and the argv after
// `--only`'s `--` separator under a single-member selection. Empty under a bare run.
pub(crate) struct Selected {
    pub(crate) name: String,
    pub(crate) args: Vec<String>,
}

// spec: gate-sdk/SPEC.md §run-gates — one member's finished run, buffered so the flush can be in
// registry order rather than completion order
pub(crate) struct Outcome {
    tail: String,
    pub(crate) output: Vec<u8>,
    pub(crate) failed: bool,
    // spec: gate-sdk/SPEC.md §run-gates — a member that failed without judging the tree: a status
    // other than 0 and 1, resolved nowhere, or never started
    pub(crate) errored: bool,
    // spec: gate-sdk/SPEC.md §run-gates — the member's status: its own where it ran, 3 where it could
    // not be started, as its dispatch tail reads, and none where it resolved nowhere
    exit: Option<i32>,
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
    json: bool,
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

// spec: gate-sdk/SPEC.md §run-gates — a refusal reports whether `--json` was already read, so an
// argv refused after the option still answers with the error document
fn parse(args: &[String]) -> Result<Args, (Refusal, bool)> {
    let mut out = Args {
        gates_dir: None,
        only: Vec::new(),
        paths: Vec::new(),
        only_args: None,
        help: false,
        json: false,
    };
    match parse_into(args, &mut out) {
        Ok(()) => Ok(out),
        Err(r) => Err((r, out.json)),
    }
}

fn parse_into(args: &[String], out: &mut Args) -> Result<(), Refusal> {
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                out.help = true;
                return Ok(());
            }
            // spec: gate-sdk/SPEC.md §run-gates — a leading option: `--only` and `--for` take every
            // token after them, so it is read only before a selector
            "--json" => {
                out.json = true;
                i += 1;
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
                return Ok(());
            }
            "--for" => {
                out.paths = args[i + 1..].to_vec();
                if out.paths.is_empty() {
                    return Err(plain("--for needs at least one path"));
                }
                return Ok(());
            }
            // spec: gate-sdk/SPEC.md §run-gates — `--` ends option processing, so a gates-dir
            // legitimately spelled with a leading dash stays reachable
            "--" => {
                if let Some(d) = args.get(i + 1) {
                    out.gates_dir = Some(d.clone());
                }
                return Ok(());
            }
            other if other.starts_with('-') => return Err(unrecognized(other)),
            other => {
                out.gates_dir = Some(other.to_string());
                return Ok(());
            }
        }
    }
    Ok(())
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

// spec: gate-sdk/SPEC.md §run-gates — a `mode=staged` member selects by git pathspec, the exact
// path or a subtree under it, a second mechanism beside the glob matcher
fn pathspec_matches(p: &str, globs: &[&str]) -> bool {
    globs
        .iter()
        .any(|g| walk::pattern_match(g, p) || walk::pattern_match(&format!("{}/*", g), p))
}

// spec: gate-sdk/SPEC.md §Reading a `couples=` field's reach — the one matcher
fn staged_matches(p: &str, globs: &[&str]) -> bool {
    globs.iter().any(|g| registry::couple_matches(p, g))
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — a resolved declaration that cannot be read is a
// refusal, never an empty manifest, which would drop the member from every selection in silence
pub(crate) fn manifest(src: &str) -> Result<Vec<(String, String)>, exit::Fault> {
    let body = std::fs::read(src).map_err(|e| exit::Fault::reading(src, format!("cannot read the declaration {}: {}", src, e)))?;
    Ok(registry::manifest_line(&String::from_utf8_lossy(&body))
        .map(registry::manifest_fields)
        .unwrap_or_default())
}

// spec: gate-sdk/SPEC.md §run-gates — one selection over a path set, and which paths it covered
pub(crate) struct Selection {
    pub(crate) run: Vec<Selected>,
    pub(crate) covered: Vec<bool>,
}

// spec: gate-sdk/SPEC.md §run-gates — the selector `--for` and the hook arm share. `tier` is the hook
// arm's filter, under which a member resolving nowhere is selected for its dispatch to red
// (§git-hook), where `--for` refuses it.
pub(crate) fn select(
    members: &[String],
    resolve_dirs: &[String],
    kit_roots_here: &[String],
    paths: &[String],
    tier: Option<&str>,
) -> Result<Selection, exit::Fault> {
    let mut covered = vec![false; paths.len()];
    let mut run: Vec<Selected> = Vec::new();
    for name in members {
        let src = match (registry::resolve(name, resolve_dirs), tier) {
            (Some(s), _) => s,
            (None, Some(_)) => {
                run.push(Selected {
                    name: name.clone(),
                    args: Vec::new(),
                });
                continue;
            }
            (None, None) => {
                return Err(exit::Fault::refused(format!("cannot resolve '{}' in: {}", name, resolve_dirs.join(" "))));
            }
        };
        let f = manifest(&src)?;
        if tier.is_some_and(|t| registry::field(&f, "tier") != t) {
            continue;
        }
        let trigger = registry::effective_trigger(&f);
        // spec: gate-sdk/SPEC.md §Fail-closed contract — an unresolvable couples token is a refusal at exit 2,
        // never a narrower selection: a silently lost trigger is a gate the selector stops running.
        let trigger = registry::expand_couples(&trigger, kit_roots_here)
            .map_err(|e| exit::Fault::refused(format!("cannot expand {}'s trigger: {}", name, e)))?;
        let mode = registry::field(&f, "mode");
        let derived = registry::derived_couples(name, resolve_dirs)
            .map_err(|e| exit::Fault::refused(format!("cannot derive {}'s knob files: {}", name, e)))?;
        let mut globs: Vec<&str> = trigger.split(',').filter(|g| !g.is_empty()).collect();
        globs.extend(derived.iter().map(String::as_str));
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
    Ok(Selection { run, covered })
}

// spec: gate-sdk/SPEC.md §run-gates — `--for`: the selection, and an uncovered path as a note on
// the transcript, never a failure
fn select_for(
    w: &mut dyn Write,
    members: &[String],
    resolve_dirs: &[String],
    kit_roots_here: &[String],
    paths: &[String],
) -> Result<Vec<Selected>, exit::Kind> {
    let s = select(members, resolve_dirs, kit_roots_here, paths, None).map_err(|e| {
        eprintln!("{}: --for {}", TOOL, e);
        e.kind
    })?;
    for (p, c) in paths.iter().zip(&s.covered) {
        if !c {
            let _ = writeln!(w, "{}: no registered gate couples to {}", TOOL, p);
        }
    }
    Ok(s.run)
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
) -> Result<Vec<Selected>, exit::Kind> {
    if only.len() == 1 && !members.iter().any(|m| m == &only[0]) {
        if registry::resolve(&only[0], resolve_dirs).is_none() {
            eprintln!("{}: --only: '{}' is not registered in {}", TOOL, only[0], list);
            return Err(exit::Kind::Refused);
        }
        return Ok(vec![Selected {
            name: only[0].clone(),
            args: forwarded.unwrap_or(&[]).to_vec(),
        }]);
    }
    for n in only {
        if !members.iter().any(|m| m == n) {
            eprintln!("{}: --only: '{}' is not registered in {}", TOOL, n, list);
            return Err(exit::Kind::Refused);
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
        return Err(exit::Kind::Usage);
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

pub(crate) struct Dispatch<'a> {
    pub(crate) resolve_dirs: &'a [String],
    pub(crate) self_exe: &'a str,
    pub(crate) list: &'a str,
    pub(crate) scratch: &'a Path,
    // spec: gate-sdk/SPEC.md §run-gates — resolved once for the run rather than per red member: the
    // value is a knob read, and a battery reddening at scale would otherwise buy the same answer
    // once per failure
    pub(crate) spec_base_url: &'a str,
    // spec: gate-sdk/SPEC.md §git-hook — added to each child's environment; the battery adds nothing
    pub(crate) env: &'a [(String, String)],
}

// spec: gate-sdk/SPEC.md §run-gates — the invariant line's reserved prefix, held here because the
// producer, §Consumer smoke's demo excerpt and the overhead meter's gate row may not spell it twice
pub const SPEC_LINE_PREFIX: &str = "    spec: ";

// spec: gate-sdk/SPEC.md §run-gates — the tail of a member the battery could not start
pub(crate) const DISPATCH_TAIL: &str = "dispatch harness error, exit 3";

// spec: gate-sdk/SPEC.md §run-gates — the failing member's own declared invariant, read off the
// descriptor that resolved it and only on a red; a descriptor carrying no `# spec:` line
// contributes nothing rather than a blank line
fn invariant(d: &Dispatch, name: &str) -> String {
    match declared(d, name) {
        Some(inv) => format!("\n{}{}", SPEC_LINE_PREFIX, inv.text),
        None => String::new(),
    }
}

// spec: gate-sdk/SPEC.md §run-gates — the invariant as the verdict line and the SARIF rule both
// carry it: the located pointer with its one-line statement, and the published location alone
fn declared(d: &Dispatch, name: &str) -> Option<sarif::Invariant> {
    let src = registry::resolve(name, d.resolve_dirs)?;
    let text = std::fs::read_to_string(&src).ok()?;
    let body = text
        .lines()
        .find_map(|l| l.trim_start().strip_prefix("# spec:"))?;
    let body = body.trim();
    // spec: canon-kit/SPEC.md §check-spec-pointer — the directive's payload: the significant head
    // ends at the em-dash prose tail, which is the one-line statement printed verbatim
    let (head, tail) = match body.split_once(" — ") {
        Some((h, t)) => (h.trim(), t.trim()),
        None => (body, ""),
    };
    let (path, frag) = match head.split_once('§') {
        Some((p, f)) => (p.trim(), f.trim()),
        None => (head, ""),
    };
    let (located, published) = resolved_location(path, frag, d.spec_base_url);
    let text = if tail.is_empty() {
        located.clone()
    } else {
        format!("{} — {}", located, tail)
    };
    Some(sarif::Invariant {
        text,
        help_uri: published.then_some(located),
    })
}

// spec: gate-sdk/SPEC.md §Layout and configuration — `GATE_SDK_SPEC_BASE_URL`: empty resolves in
// the tree, set resolves a `<dir>/SPEC.md §<heading>` pointer to `<base>/<dir>/SPEC#<anchor>`, and
// a pointer the mirror does not publish keeps its repo-relative spelling
fn resolved_location(path: &str, frag: &str, base: &str) -> (String, bool) {
    let in_tree = || {
        if frag.is_empty() {
            (path.to_string(), false)
        } else {
            (format!("{} §{}", path, frag), false)
        }
    };
    if base.is_empty() {
        return in_tree();
    }
    let Some(dir) = path.strip_suffix("/SPEC.md") else {
        return in_tree();
    };
    if dir.is_empty() || dir.contains('/') {
        return in_tree();
    }
    let mut url = format!("{}/{}/SPEC", base.trim_end_matches('/'), dir);
    if !frag.is_empty() {
        url.push('#');
        url.push_str(&crate::spec::anchor_slug(frag));
    }
    (url, true)
}

// spec: gate-sdk/SPEC.md §run-gates — one member, run as a child process; the in-process call is
// refused there, on the declared-knob discipline, fault isolation and the surviving `.sh` members
pub(crate) fn dispatch_one(d: &Dispatch, idx: usize, sel: &Selected) -> Outcome {
    let started = Instant::now();
    // spec: gate-sdk/SPEC.md §run-gates — the invariant rides the `tail` field rather than a second
    // Outcome field: the flush already writes the tail with `writeln!`, so a multi-line value needs
    // no second writer and cannot be printed by one path and dropped by another
    let fail = |tail: &str, body: String, ms: u128| Outcome {
        tail: format!("  FAIL: {} ({}){}", sel.name, tail, invariant(d, &sel.name)),
        output: body.into_bytes(),
        failed: true,
        errored: true,
        exit: (tail == DISPATCH_TAIL).then_some(exit::TOOL),
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
    let inv = match registry::invocation(&sel.name, &src, d.self_exe) {
        Ok(i) => i,
        Err(e) => return fail(DISPATCH_TAIL, e.message, started.elapsed().as_millis()),
    };
    let (program, mut args) = (inv.program, inv.args);
    args.extend(sel.args.iter().cloned());
    // spec: gate-sdk/SPEC.md §The gate-authoring surface — a member the binary does not implement
    // is handed the running binary, so it reads a knob through `--emit knob-values`
    let mut env: Vec<(String, String)> = d.env.to_vec();
    if inv.substrate != registry::Substrate::Compiled {
        env.push(("GATE_SDK_NATIVE_BIN".to_string(), d.self_exe.to_string()));
    }

    let tmpdir = d.scratch.join(format!("t{}", idx));
    if let Err(e) = std::fs::create_dir_all(&tmpdir) {
        return fail(
            DISPATCH_TAIL,
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
    match proc::dispatch(&program, &args, &tmpdir, d.scratch, &capture, &env) {
        Err(e) => fail(DISPATCH_TAIL, e, started.elapsed().as_millis()),
        Ok(done) => {
            let ms = started.elapsed().as_millis();
            if done.code == 0 {
                Outcome {
                    tail: format!("  PASS: {}", sel.name),
                    output: done.output,
                    failed: false,
                    errored: false,
                    exit: Some(done.code),
                    ms,
                }
            } else {
                Outcome {
                    tail: format!(
                        "  FAIL: {} (exit {}){}",
                        sel.name,
                        done.code,
                        invariant(d, &sel.name)
                    ),
                    output: done.output,
                    failed: true,
                    errored: done.code != 1,
                    exit: Some(done.code),
                    ms,
                }
            }
        }
    }
}

// spec: gate-sdk/SPEC.md §run-gates — a declared omission keeps `All N gates passed.` honest as the
// roster-collapse tripwire, reported on its own line beside the summary and never inside it
fn report_omissions(w: &mut dyn Write, list_text: &str) {
    let mut reasons: Vec<&str> = registry::omissions(list_text).into_iter().map(|(_, _, r)| r).collect();
    reasons.sort_unstable();
    let mut i = 0;
    while i < reasons.len() {
        let r = reasons[i];
        let n = reasons[i..].iter().take_while(|x| **x == r).count();
        // spec: gate-sdk/SPEC.md §run-gates — the class is reason-agnostic and belongs to any
        // consumer omitting a member for any cause, so the reason is reported as written and no
        // remedy is invented for it (installer/SPEC.md §The gate binary).
        let _ = writeln!(w, "{} gate(s) omitted ({}).", n, r);
        i += n;
    }
}

// spec: gate-sdk/SPEC.md §run-gates — the one maker of every runner's run scratch:
// `checkwright-run.<pid>` under the tmp dir absolutized at the invoker's root, the tmp dir made
// self-ignoring where absent
pub(crate) fn run_scratch(tmp_dir: &str) -> Result<RunScratch, String> {
    run_scratch_in(&walk::cwd()?, tmp_dir)
}

// spec: gate-sdk/SPEC.md §run-gates — the same scratch for a caller resolving the tmp dir against a
// root other than its working directory
pub(crate) fn run_scratch_in(here: &str, tmp_dir: &str) -> Result<RunScratch, String> {
    let dir = run_scratch_at(here, tmp_dir)?;
    proc::hold_temp_base(&dir);
    Ok(RunScratch(dir))
}

// spec: gate-sdk/SPEC.md §run-gates — a held run scratch, the process's temp base while it lives and
// removed on every exit path
pub(crate) struct RunScratch(PathBuf);

impl std::ops::Deref for RunScratch {
    type Target = Path;
    fn deref(&self) -> &Path {
        &self.0
    }
}

impl Drop for RunScratch {
    fn drop(&mut self) {
        proc::release_temp_base(&self.0);
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn run_scratch_at(here: &str, tmp_dir: &str) -> Result<PathBuf, String> {
    let root = PathBuf::from(walk::abs_against(here, tmp_dir));
    let scratch = root.join(format!("checkwright-run.{}", std::process::id()));
    let made = walk::make_scratch(&root).and_then(|()| {
        let _ = std::fs::remove_dir_all(&scratch);
        std::fs::create_dir_all(&scratch)
    });
    made.map_err(|e| format!("cannot create the run scratch {}: {}", scratch.display(), e))?;
    Ok(scratch)
}

// spec: gate-sdk/SPEC.md §run-gates — only an unfiltered run of the configured registry writes the
// timings file, so its one reader never sums a subset.
fn writes_timings(only: &[String], paths: &[String], explicit: bool) -> bool {
    only.is_empty() && paths.is_empty() && !explicit
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
        let _ = walk::make_scratch(dir);
    }
    let _ = std::fs::write(path, body);
}

// spec: gate-sdk/SPEC.md §run-gates — the `--json` document's version, under the verdict-contract rule
const CONTRACT: &str = "1.0.0";

pub fn run(args: &[String]) -> i32 {
    let parsed = match parse(args) {
        Ok(p) => p,
        Err((e, json)) => {
            eprintln!("{}: {}", TOOL, e.message);
            if e.usage {
                eprintln!("{}", USAGE);
            }
            return answer(json, Err(exit::Kind::Usage));
        }
    };
    // spec: gate-sdk/SPEC.md §The bin/-tool contract — a help request is the usage text on stdout at
    // exit 0, and it is answered before any knob is read: the text describes the arm roster, which is
    // a property of the binary rather than of the tree it was pointed at
    if parsed.help {
        println!("{}", USAGE);
        return 0;
    }
    answer(parsed.json, battery(&parsed))
}

// spec: gate-sdk/SPEC.md §run-gates — under `--json` the document is the arm's last write: the run's
// at its verdict status, or the error document where the arm itself refused or failed
fn answer(json: bool, result: Result<(i32, String), exit::Kind>) -> i32 {
    match result {
        Ok((code, document)) => {
            if json {
                print!("{}", document);
            }
            code
        }
        Err(kind) => {
            if json {
                print!("{}", exit::error_document(CONTRACT, kind));
            }
            kind.status()
        }
    }
}

// spec: gate-sdk/SPEC.md §run-gates — the run itself: its verdict status and, under `--json`, its
// document; every refusal and failure before the verdict names its kind
fn battery(parsed: &Args) -> Result<(i32, String), exit::Kind> {
    let json = parsed.json;
    // spec: gate-sdk/SPEC.md §run-gates — under `--json` the transcript moves to stderr, so stdout
    // carries the document alone
    let mut w: Box<dyn Write> = if json { Box::new(std::io::stderr()) } else { Box::new(std::io::stdout()) };
    // spec: gate-sdk/SPEC.md §Output contract — a run outside a git repository is refused at 2 before
    // anything is read, since its pass would carry no tree to key it
    match walk::toplevel_fault() {
        Ok(_) => {}
        Err(f) if f.kind == exit::Kind::Refused => {
            eprintln!("{}: not inside a git repository", TOOL);
            return Err(f.kind);
        }
        Err(f) => {
            eprintln!("{}: {}", TOOL, f);
            return Err(f.kind);
        }
    }
    let knob = |n: &str| walk::knob_scalar(n);
    let configured_dir = match knob("GATE_SDK_GATES_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return Err(exit::Kind::Refused);
        }
    };
    let gates_dir = parsed.gates_dir.clone().unwrap_or_else(|| configured_dir.clone());
    let explicit = gates_dir != configured_dir;
    let list = registry::list_path(&gates_dir);

    let list_text = match exit::read_configured(&list, "registry") {
        Ok(t) => t,
        Err(f) => {
            eprintln!("{}: {}", TOOL, f);
            if explicit && f.kind == exit::Kind::Refused {
                steer(&gates_dir, &configured_dir);
            }
            return Err(f.kind);
        }
    };
    let members = registry::members(&list_text);
    if members.is_empty() {
        eprintln!("{}: {} names no gates", TOOL, list);
        return Err(exit::Kind::Refused);
    }

    // spec: gate-sdk/SPEC.md §Layout and configuration — the kit roots absolutized, so the
    // unresolved-member report names the paths every dispatcher resolves
    let kit_roots = match walk::kit_roots_abs() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return Err(exit::Kind::Refused);
        }
    };
    let resolve_dirs = registry::resolve_dirs(&gates_dir, &kit_roots);

    // spec: gate-sdk/SPEC.md §run-gates — the selection is read only where a log is written, and a
    // value it refuses stops the run before the first member, an empty `--for` selection included
    let sarif_path = std::env::var(sarif::KNOB).unwrap_or_default();
    let selection = if sarif_path.is_empty() {
        sarif::Selection::All
    } else {
        match sarif::Selection::parse(&std::env::var(sarif::RESULTS_KNOB).unwrap_or_default()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}: {}", TOOL, e);
                return Err(exit::Kind::Refused);
            }
        }
    };
    let store = match crate::verdict_store::configured() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return Err(e.kind);
        }
    };

    let selected: Vec<Selected> = if !parsed.only.is_empty() {
        select_only(&members, &list, &resolve_dirs, &parsed.only, parsed.only_args.as_deref())?
    } else if !parsed.paths.is_empty() {
        // spec: gate-sdk/SPEC.md §run-gates — the kit roots relative to the working directory the door
        // set to the toplevel, since the given paths are repo-relative
        let kit_roots_here = match walk::kit_roots() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}", TOOL, e);
                return Err(exit::Kind::Refused);
            }
        };
        let s = select_for(&mut *w, &members, &resolve_dirs, &kit_roots_here, &parsed.paths)?;
        if s.is_empty() {
            let _ = write!(w, "\n===== gates summary =====\nno coupled gate for the given path(s); nothing to run.\n");
            let _ = w.flush();
            if sarif_path.is_empty() && !json {
                return Ok((0, String::new()));
            }
            let provenance = provenance_of(sarif::Scope::For, &[], &resolve_dirs);
            if !sarif_path.is_empty() {
                if let Err(e) = write_log(&sarif_path, &list, &provenance, sarif::Scope::For, selection, &[], &[]) {
                    eprintln!("{}: {}", TOOL, e);
                    return Err(exit::Kind::Tool);
                }
            }
            return Ok((0, document(0, sarif::Scope::For, &provenance, Vec::new(), &[])));
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
            return Err(exit::Kind::Refused);
        }
    };
    let scratch = match run_scratch(&tmp_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {} — treating as failure (not clean)", TOOL, e);
            return Err(exit::Kind::Tool);
        }
    };

    let self_exe = match crate::proc::self_image() {
        Ok(p) => p.display().to_string(),
        Err(e) => {
            eprintln!(
                "{}: cannot resolve this binary's own path: {} — treating as failure (not clean)",
                TOOL, e
            );
            return Err(exit::Kind::Tool);
        }
    };

    let spec_base_url = match knob("GATE_SDK_SPEC_BASE_URL") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", TOOL, e);
            return Err(exit::Kind::Refused);
        }
    };
    let scratch_path = scratch.to_path_buf();
    let d = Dispatch {
        resolve_dirs: &resolve_dirs,
        self_exe: &self_exe,
        list: &list,
        scratch: &scratch_path,
        spec_base_url: &spec_base_url,
        env: &[],
    };

    let (scope, omissions) = if !parsed.only.is_empty() {
        (sarif::Scope::Only, Vec::new())
    } else if !parsed.paths.is_empty() {
        (sarif::Scope::For, Vec::new())
    } else {
        let declared = registry::omissions(&list_text);
        let omissions = declared
            .into_iter()
            .map(|(line, name, reason)| sarif::Omission { line, name, reason })
            .collect();
        (sarif::Scope::Registry, omissions)
    };
    // spec: gate-sdk/SPEC.md §run-gates — the store's key is read before the first member runs
    let before = store
        .as_ref()
        .filter(|_| scope == sarif::Scope::Registry)
        .map(|_| provenance_of(scope, &selected, &resolve_dirs));

    let outcomes = dispatch_all(&d, &selected);
    drop(scratch);

    if writes_timings(&parsed.only, &parsed.paths, explicit) {
        write_timings(
            &PathBuf::from(&tmp_dir).join("gate-timings.txt"),
            &selected,
            &outcomes,
        );
    }

    let verbose = std::env::var("GATE_SDK_VERBOSE").map(|v| !v.is_empty()).unwrap_or(false);
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
    report_omissions(&mut *w, &list_text);
    let failed: Vec<&str> = selected
        .iter()
        .zip(&outcomes)
        .filter(|(_, o)| o.failed)
        .map(|(s, _)| s.name.as_str())
        .collect();
    let code = if failed.is_empty() {
        let _ = writeln!(w, "All {} gates passed.", selected.len());
        0
    } else {
        let _ = writeln!(
            w,
            "{} of {} gates FAILED: {}",
            failed.len(),
            selected.len(),
            failed.join(" ")
        );
        verdict_status(&outcomes)
    };
    let _ = w.flush();
    let provenance =
        (!sarif_path.is_empty() || store.is_some() || json).then(|| provenance_of(scope, &selected, &resolve_dirs));
    // spec: gate-sdk/SPEC.md §run-gates — the SARIF log is written once after the verdict lines,
    // and only an unwritable path changes the exit status
    if let Some(provenance) = provenance.as_ref().filter(|_| !sarif_path.is_empty()) {
        let lines = registry::member_lines(&list_text);
        let members: Vec<sarif::Member> = selected
            .iter()
            .zip(&outcomes)
            .map(|(s, o)| sarif::Member {
                name: &s.name,
                registry_line: lines.iter().find(|(_, n)| *n == s.name).map(|(l, _)| *l),
                failed: o.failed,
                tail: &o.tail,
                output: &o.output,
                invariant: declared(&d, &s.name),
                kit: member_kit(&s.name, &resolve_dirs, &kit_roots),
            })
            .collect();
        if let Err(e) = write_log(&sarif_path, &list, provenance, scope, selection, &members, &omissions) {
            eprintln!("{}: {}", TOOL, e);
            return Err(exit::Kind::Tool);
        }
    }
    // spec: gate-sdk/SPEC.md §run-gates — the store follows the log, keeping a judged registry run
    // whose key did not move while its members ran
    if let (Some(dir), Some(provenance)) = (&store, &provenance) {
        if let Some((name, body)) = crate::verdict_store::entry(scope, code, provenance) {
            if !before.as_ref().is_some_and(|b| crate::verdict_store::steady(b, provenance)) {
                let _ = writeln!(w, "verdict store: the tree or the gate set moved while members ran; nothing stored");
                let _ = w.flush();
            } else if let Err(e) = crate::verdict_store::write(dir, &name, &body) {
                eprintln!("{}: {}", TOOL, e);
                return Err(exit::Kind::Tool);
            }
        }
    }
    let document = match provenance.filter(|_| json) {
        Some(p) => {
            let entries = selected.iter().zip(&outcomes).map(|(s, o)| member_entry(&s.name, o)).collect();
            document(code, scope, &p, entries, &omissions)
        }
        None => String::new(),
    };
    Ok((code, document))
}

// spec: gate-sdk/SPEC.md §run-gates — one member's entry: its kind, its status where it ran, and an
// `error` member's cause, `refused` for an exit 2 or a member resolved nowhere and `tool` otherwise
fn member_entry(name: &str, o: &Outcome) -> Value {
    let kind = if !o.failed {
        sarif::green(&o.output, &o.tail).0
    } else if !o.errored {
        "fail"
    } else {
        "error"
    };
    let mut m = Map::new();
    m.insert("name".into(), json!(name));
    m.insert("kind".into(), json!(kind));
    if let Some(c) = o.exit {
        m.insert("exit".into(), json!(c));
    }
    if o.failed && o.errored {
        let cause = match o.exit {
            None => exit::Kind::Refused,
            Some(c) if c == exit::REFUSED => exit::Kind::Refused,
            Some(_) => exit::Kind::Tool,
        };
        m.insert("error".into(), json!(cause.word()));
    }
    Value::Object(m)
}

// spec: gate-sdk/SPEC.md §run-gates — the `--json` document: one object, keys sorted, no
// insignificant whitespace
fn document(code: i32, scope: sarif::Scope, provenance: &Provenance, members: Vec<Value>, omissions: &[sarif::Omission]) -> String {
    let verdict = match code {
        0 => "pass",
        1 => "fail",
        _ => "error",
    };
    let omitted: Vec<Value> = omissions.iter().map(|o| json!({ "name": o.name, "reason": o.reason })).collect();
    let doc = json!({
        "contract": CONTRACT,
        "verdict": verdict,
        "scope": scope.word(),
        "provenance": provenance.block(),
        "members": members,
        "omitted": omitted,
    });
    format!("{}\n", doc)
}

// spec: gate-sdk/SPEC.md §run-gates — the run's verdict: `fail` where any member exited 1, else
// `error` where any other member failed without judging, else `pass`; a violation found outranks
// an error beside it
pub(crate) fn verdict_status(outcomes: &[Outcome]) -> i32 {
    if outcomes.iter().any(|o| o.failed && !o.errored) {
        1
    } else if outcomes.iter().any(|o| o.errored) {
        exit::TOOL
    } else {
        0
    }
}

// spec: gate-sdk/SPEC.md §run-gates — the selection as `gateSet` reads it, each member's resolved
// file hashed; a resolved file the run cannot read leaves the set unfillable, so it is absent
fn set_members(selected: &[Selected], resolve_dirs: &[String]) -> Option<Vec<crate::provenance::SetMember>> {
    selected
        .iter()
        .map(|s| {
            let file = match registry::resolve(&s.name, resolve_dirs) {
                Some(p) => Some(crate::sha256::hex(&std::fs::read(p).ok()?)),
                None => None,
            };
            Some(crate::provenance::SetMember {
                name: s.name.clone(),
                file,
                args: s.args.clone(),
            })
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §run-gates — the block collected once per run, for the log and the store
fn provenance_of(scope: sarif::Scope, selected: &[Selected], resolve_dirs: &[String]) -> Provenance {
    let set = set_members(selected, resolve_dirs);
    // spec: gate-sdk/SPEC.md §run-gates — the block's kit roots in their repo-relative spelling
    let roots_here = walk::kit_roots().unwrap_or_default();
    crate::provenance::collect(&roots_here, set.as_deref().map(|m| (scope.word(), m)))
}

fn write_log(
    path: &str,
    list: &str,
    provenance: &Provenance,
    scope: sarif::Scope,
    selection: sarif::Selection,
    members: &[sarif::Member],
    omissions: &[sarif::Omission],
) -> Result<(), String> {
    let run = sarif::Run {
        registry: list,
        provenance,
        scope,
        selection,
    };
    let body = sarif::render(&run, members, omissions, &|p| match Path::new(p).metadata() {
        Ok(m) if m.is_file() => Some(sarif::Artifact::File),
        Ok(m) if m.is_dir() => Some(sarif::Artifact::Directory),
        _ => None,
    });
    sarif::write(path, &body)
}

// spec: gate-sdk/SPEC.md §run-gates — a rule's `properties.kit`: the kit root its member resolved
// from, none for the consumer's gates dir at index 0
fn member_kit(name: &str, resolve_dirs: &[String], kit_roots: &[String]) -> Option<String> {
    let (i, _) = registry::resolve_at(name, resolve_dirs)?;
    let root = kit_roots.get(i.checked_sub(1)?)?;
    Some(crate::provenance::kit_name(root))
}

pub(crate) fn trim_trailing_newlines(b: &[u8]) -> &[u8] {
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
    if !registry::members(&text).iter().any(|m| m == arg) {
        return;
    }
    if let Ok(door) = gates::door_command(&format!("--only {}", arg)) {
        eprintln!(
            "{}: '{}' is a gate registered in {}, not a gates dir — run it with: {}",
            TOOL, arg, default_list, door
        );
    }
}

// spec: gate-sdk/SPEC.md §run-gates — the worker pool: `std::thread` and `std::sync` only, no new
// crate dependency, since §The adopter constraints makes footprint a cost paid per target on every
// adopter's machine. Members are claimed off one shared cursor, so a slow member cannot starve the queue.
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
                    tail: format!(
                        "  FAIL: {} ({}){}",
                        selected[i].name,
                        DISPATCH_TAIL,
                        invariant(d, &selected[i].name)
                    ),
                    output: b"the worker pool returned no result for this member".to_vec(),
                    failed: true,
                    errored: true,
                    exit: Some(exit::TOOL),
                    ms: 0,
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §run-gates — the run scratch lies under the tmp dir absolutized at the
    // invoker's root, a relative value included, and a dispatched child's `TMPDIR` lies under it
    // with the run scratch heading its git ceiling
    #[test]
    fn the_run_scratch_roots_under_the_tmp_dir_and_heads_each_childs_ceiling() {
        let here = std::env::temp_dir().join(format!("checkwright-run-scratch-test.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&here);
        std::fs::create_dir_all(&here).unwrap();
        let here_s = walk::abs_against(&here.display().to_string(), ".");
        let root = PathBuf::from(walk::abs_against(&here_s, "rel/tmp"));
        let scratch = run_scratch_at(&here_s, "rel/tmp").unwrap();
        assert_eq!(scratch.parent(), Some(root.as_path()), "the run scratch is a child of the absolutized tmp dir");
        assert!(walk::path_root(&scratch.display().to_string()).is_some() && scratch.is_dir());
        assert!(root.join(".gitignore").is_file(), "the tmp dir the run made ignores itself");

        let tmpdir = scratch.join("t0");
        std::fs::create_dir_all(&tmpdir).unwrap();
        let env = proc::scratch_env(&tmpdir, &scratch);
        let get = |k: &str| env.iter().find(|(n, _)| n == k).map(|(_, v)| v.clone()).unwrap();
        assert_eq!(get("TMPDIR"), tmpdir.display().to_string());
        assert_eq!(get("TMP"), tmpdir.display().to_string(), "a Windows child reads TMP, so it names the same private dir");
        let sep = if cfg!(windows) { ';' } else { ':' };
        assert_eq!(get("GIT_CEILING_DIRECTORIES").split(sep).next(), Some(scratch.display().to_string().as_str()));

        #[cfg(unix)]
        {
            let script = r#"printf '%s\n%s\n' "$TMPDIR" "$GIT_CEILING_DIRECTORIES""#;
            let args: Vec<String> = ["-c", script].iter().map(|s| s.to_string()).collect();
            let done = proc::dispatch(&crate::programs::BASH, &args, &tmpdir, &scratch, &scratch.join("c0"), &[]).unwrap();
            let out = String::from_utf8(done.output).unwrap();
            let mut lines = out.lines();
            assert_eq!(lines.next(), Some(tmpdir.display().to_string().as_str()));
            let ceiling = lines.next().unwrap();
            assert_eq!(ceiling.split(sep).next(), Some(scratch.display().to_string().as_str()), "{}", ceiling);
        }
        let _ = std::fs::remove_dir_all(&here);
    }

    // spec: gate-sdk/SPEC.md §run-gates — the write predicate over a bare run and the three
    // filtered ones
    #[test]
    fn only_a_bare_run_of_the_configured_registry_writes_timings() {
        let one = vec!["x".to_string()];
        assert!(writes_timings(&[], &[], false));
        assert!(!writes_timings(&one, &[], false));
        assert!(!writes_timings(&[], &one, false));
        assert!(!writes_timings(&[], &[], true));
    }

    // spec: gate-sdk/SPEC.md §Layout and configuration — both branches of the resolution, and the
    // two shapes the mirror does not publish: a nested path and a non-SPEC document, each keeping
    // its repo-relative spelling rather than naming a page that is not there
    #[test]
    fn a_pointer_resolves_to_the_mirror_only_where_the_mirror_publishes_one() {
        let resolved = |p: &str, f: &str, b: &str| resolved_location(p, f, b).0;
        assert!(!resolved_location("gate-sdk/SPEC.md", "X", "").1);
        assert!(resolved_location("gate-sdk/SPEC.md", "X", "https://example.test").1);
        assert!(!resolved_location("vendor/gate-sdk/SPEC.md", "X", "https://example.test").1);
        assert_eq!(
            resolved("gate-sdk/SPEC.md", "Consumer payload", ""),
            "gate-sdk/SPEC.md §Consumer payload"
        );
        assert_eq!(resolved("gate-sdk/SPEC.md", "", ""), "gate-sdk/SPEC.md");
        assert_eq!(
            resolved("gate-sdk/SPEC.md", "Consumer payload", "https://example.test"),
            "https://example.test/gate-sdk/SPEC#consumer-payload"
        );
        // comment-tier-exempt: a trailing slash is a local property of a hand-written knob value
        assert_eq!(
            resolved("gate-sdk/SPEC.md", "", "https://example.test/"),
            "https://example.test/gate-sdk/SPEC"
        );
        assert_eq!(
            resolved("canon-kit/SPEC.md", "bin/env-probe — the floor", "https://example.test"),
            format!(
                "https://example.test/canon-kit/SPEC#{}",
                crate::spec::anchor_slug("bin/env-probe — the floor")
            )
        );
        assert_eq!(
            resolved("vendor/gate-sdk/SPEC.md", "X", "https://example.test"),
            "vendor/gate-sdk/SPEC.md §X"
        );
        assert_eq!(
            resolved("docs/site-architecture.md", "X", "https://example.test"),
            "docs/site-architecture.md §X"
        );
    }

    // spec: gate-sdk/SPEC.md §run-gates — the prefix is reserved AGAINST the two verdict prefixes
    // and against the overhead meter's content marker, both asserted rather than eyeballed
    #[test]
    fn the_invariant_lines_prefix_misses_every_verdict_reader() {
        assert!(!SPEC_LINE_PREFIX.starts_with("  PASS: "));
        assert!(!SPEC_LINE_PREFIX.starts_with("  FAIL: "));
        assert!(!SPEC_LINE_PREFIX.contains("FAIL: check-"));
        let line = format!("{}gate-sdk/SPEC.md §X — the rule it holds", SPEC_LINE_PREFIX);
        assert!(!line.starts_with("  PASS: ") && !line.starts_with("  FAIL: "));
        assert!(!line.contains("FAIL: check-"));
    }

    // spec: gate-sdk/SPEC.md §Reading a `couples=` field's reach — POSIX `case` matching: `*` and
    // `?` cross `/`, a bracket expression, a leading `*`, and an exact path matching only itself
    #[test]
    fn the_staged_matcher_reads_a_pattern_as_posix_case_matching() {
        assert!(staged_matches("docs/a/b.md", &["docs/*.md"]));
        assert!(staged_matches("kit/deep/x.rs", &["kit/*.rs"]));
        assert!(staged_matches("abc/one.txt", &["a?c/*.txt"]));
        assert!(!staged_matches("ac/one.txt", &["a?c/*.txt"]));
        assert!(staged_matches("alpha/one", &["[ab]lpha/*"]));
        assert!(!staged_matches("clpha/one", &["[ab]lpha/*"]));
        assert!(staged_matches("clpha/one", &["[!ab]lpha/*"]));
        assert!(staged_matches("vendor/scripts/run.sh", &["*scripts/*.sh"]));
        assert!(staged_matches("README.md", &["*.md"]));
        assert!(staged_matches("docs/index.md", &["docs/index.md"]));
        assert!(!staged_matches("docs/index.mdx", &["docs/index.md"]));
        assert!(!staged_matches("docs", &["docs/*"]));
        assert!(staged_matches("kit/run.sh", &["scripts/*.sh", "kit/*.sh"]));
        assert!(!staged_matches("", &["docs/*.md", "nothing/at/all"]));
        assert!(staged_matches("", &["*"]));
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

    // spec: gate-sdk/SPEC.md §run-gates — pass, fail and error exit 0, 1 and 3, and a violation one
    // member found outranks another member's error
    #[test]
    fn the_verdict_is_three_way_and_fail_outranks_error() {
        let o = |failed: bool, errored: bool| Outcome {
            tail: String::new(),
            output: Vec::new(),
            failed,
            errored,
            exit: None,
            ms: 0,
        };
        assert_eq!(verdict_status(&[o(false, false), o(false, false)]), 0);
        assert_eq!(verdict_status(&[o(false, false), o(true, false)]), 1);
        assert_eq!(verdict_status(&[o(false, false), o(true, true)]), exit::TOOL);
        assert_eq!(verdict_status(&[o(true, true), o(true, false)]), 1);
        assert_eq!(verdict_status(&[]), 0);
    }

    // spec: gate-sdk/SPEC.md §run-gates — the four goldens are the documents byte for byte, each
    // kind, cause and scope carried by at least one
    #[test]
    fn the_goldens_are_the_documents() {
        let p = crate::sarif::fixed();
        let ran = |code: i32, output: &str| Outcome {
            tail: String::new(),
            output: output.as_bytes().to_vec(),
            failed: code != 0,
            errored: code != 0 && code != 1,
            exit: Some(code),
            ms: 0,
        };
        let unjudged = |exit: Option<i32>| Outcome {
            tail: String::new(),
            output: Vec::new(),
            failed: true,
            errored: true,
            exit,
            ms: 0,
        };
        let names = ["check-a", "check-b"];
        let doc = |scope: sarif::Scope, outcomes: [Outcome; 2], omissions: &[sarif::Omission]| {
            let entries = names.iter().zip(&outcomes).map(|(n, o)| member_entry(n, o)).collect();
            document(verdict_status(&outcomes), scope, &p, entries, omissions)
        };
        let pass = doc(
            sarif::Scope::Registry,
            [ran(0, "CHECK-A: clean (3 files)\n"), ran(0, "CHECK-B: clean (not applicable: CHECK_B_GLOBS is unset)\n")],
            &[sarif::Omission { line: 3, name: "check-c", reason: "unshipped" }],
        );
        let fail = doc(sarif::Scope::Only, [ran(1, "a.md:1: finding\n"), ran(2, "check-b: refused\n")], &[]);
        let error = doc(sarif::Scope::For, [unjudged(None), unjudged(Some(exit::TOOL))], &[]);
        let refusal = exit::error_document(CONTRACT, exit::Kind::Refused);
        for (got, golden) in [
            (pass, include_str!("../../docs/contracts/run/v1/pass.json")),
            (fail, include_str!("../../docs/contracts/run/v1/fail.json")),
            (error, include_str!("../../docs/contracts/run/v1/error.json")),
            (refusal, include_str!("../../docs/contracts/run/v1/refusal.json")),
        ] {
            assert_eq!(got, golden);
        }
    }
}
