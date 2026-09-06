// spec: gate-sdk/SPEC.md §Consumer smoke — the adoption walkthrough as a bridged `Arm::Run`
// member: its contract is the verdict — 0 the arc behaved, 1 an act of it did not, 2 a
// precondition the harness could not meet — which `Arm::Emit` cannot carry.
// spec: gate-sdk/SPEC.md §Consumer smoke — the binary placement calls `csmoke_place_binary` in the
// library that owns it, through the shared spawn seam, so the port creates no second producer.
use crate::emit::csmoke;
use crate::ere::Ere;
use crate::proc::{self, Sink, Stderr};
use crate::walk;

// spec: gate-sdk/SPEC.md §Consumer smoke — the roster is the *consumer's*: which kits the
// walkthrough vendors, and the binary the scratch consumer receives. `DEMO_TMP_DIR` is read below
// and may not join this roster (§The non-gate arm).
pub const KNOBS: &[&str] = &["GATE_KIT_ROOTS_HERE", "GATE_SDK_NATIVE_BIN"];

const NAME: &str = "run-demo";
const VERDICT: &str = "DEMO";
const USAGE: &str = "usage: --run-demo";
const RULE: &str = "════════════════════════════════════════════════════════════";

// spec: gate-sdk/SPEC.md §Consumer smoke — the three exit classes as a type, so a finding about the
// adoption arc cannot be raised on a precondition's spelling or the reverse.
enum Outcome {
    Clean,
    Fail(String),
    Refuse(String),
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the shell form's `trap cleanup EXIT`, which removed the
// scratch on every exit path. The walkthrough narrates and tears down its own scratch as part of
// its arc, so there is no mode to keep and no `--keep` to suppress this.
struct Scratch {
    dir: String,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if !self.dir.is_empty() {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }
}

// spec: gate-sdk/SPEC.md §Consumer smoke — an operand is a refusal rather than a silently ignored
// word: once 2 means "the harness could not be stood up", a swallowed operand is the one way a
// caller believes it selected a mode and then reads a verdict about a different run.
pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        eprintln!("{}: unknown option: {}; {}", NAME, a, USAGE);
        return 2;
    }
    let mut scratch = Scratch {
        dir: String::new(),
    };
    match walkthrough(&mut scratch) {
        Outcome::Clean => {
            banner("DEMO: clean — the full adoption arc behaved");
            say("vendor → clean pass → violation blocked → fix → green");
            0
        }
        Outcome::Fail(why) => {
            println!("\n{}: FAIL — {}", VERDICT, why);
            1
        }
        Outcome::Refuse(why) => {
            eprintln!("\n{}: FAIL(env) — {}", VERDICT, why);
            2
        }
    }
}

fn banner(title: &str) {
    println!("\n{}", RULE);
    println!("  {}", title);
    println!("{}", RULE);
}

fn say(line: &str) {
    println!("  {}", line);
}

macro_rules! step {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(o) => return o,
        }
    };
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the four acts in the shell driver's order: vendor and
// install, assert the clean commit passes, fire one crafted violation and assert the battery names
// the gate that caught it, then restore and assert green returns.
fn walkthrough(scratch: &mut Scratch) -> Outcome {
    let roots = step!(kit_roots());
    let sdk = step!(gate_sdk_root(&roots));
    let host = step!(parent_of(&sdk));
    let consumer = step!(make_scratch(scratch));

    banner("ACT 1 — Vendor the kits into a fresh consumer");
    say("A new project adopts Checkwright by vendoring the kits and running each");
    say("kit's installer. No global install, no network — the kits are copied in.");
    step!(git(&consumer, &["init", "-q"]));
    // spec: gate-sdk/SPEC.md §Consumer smoke — the placed binary is ignored rather than tracked,
    // matching the other consumer-smoke callers, so a later `git clean -fd` spares it
    let bin = step!(native_bin());
    step!(write(
        &format!("{}/.gitignore", consumer),
        &format!(".tmp/\n{}\n", bin)
    ));
    step!(git(&consumer, &["add", "-A"]));
    step!(commit(&consumer, &["--allow-empty"], "seed"));
    for r in &roots {
        step!(vendor(r, &consumer));
    }
    step!(place_binary(&sdk, &consumer, &host, &roots));
    for r in &roots {
        let kit = basename(r);
        say(&format!("vendor + install: {}", kit));
        step!(install(&consumer, &kit));
    }
    say("→ gates.list written, pre-commit hook generated. The consumer is governed.");

    banner("ACT 2 — A clean commit passes the battery");
    say("With the kits installed and zero further config, the gate battery is green.");
    step!(git(&consumer, &["add", "-A"]));
    step!(commit(&consumer, &["--no-verify"], "installed baseline"));
    let (rc, out) = step!(run_battery(&consumer));
    let green = step!(green_line(&out));
    if rc != 0 || green.is_empty() {
        print!("{}", ends_with_newline(&out));
        return Outcome::Fail("the battery was not green on the freshly installed consumer".into());
    }
    say(&green);

    banner("ACT 3 — A violation is caught before it lands");
    say("A contributor introduces a defect — here, gate-sdk's craftable smoke");
    say("violation (an unread shell variable). The battery must turn red and name");
    say("the gate, with its finding and a help line pointing at the remedy.");
    let expected = step!(fire_violation(&consumer, &basename(&sdk)));
    if expected.is_empty() {
        return Outcome::Fail("the violation script printed no expected-gate name".into());
    }
    say(&format!("expected gate: {}", expected));
    let (rc, out) = step!(run_battery(&consumer));
    if rc == 0 {
        return Outcome::Fail(format!(
            "the violation did not turn the battery red (expected {})",
            expected
        ));
    }
    if !out.contains(&format!("FAIL: {}", expected)) {
        print!("{}", ends_with_newline(&out));
        return Outcome::Fail(format!("a gate other than {} caught it", expected));
    }
    println!();
    for line in excerpt(&out, &expected) {
        println!("{}", line);
    }
    say("→ blocked. The pre-commit hook would have rejected this commit.");

    banner("ACT 4 — Fix, re-run, green");
    say("The contributor drops the offending change and re-runs; the battery is");
    say("green again — the same gate that blocked now passes.");
    step!(git(&consumer, &["reset", "-q", "--hard"]));
    step!(git(&consumer, &["clean", "-qfd"]));
    let (rc, out) = step!(run_battery(&consumer));
    let green = step!(green_line(&out));
    if rc != 0 || green.is_empty() {
        print!("{}", ends_with_newline(&out));
        return Outcome::Fail("the battery did not return to green after the fix".into());
    }
    say(&green);

    Outcome::Clean
}

// spec: gate-sdk/SPEC.md §Consumer smoke — `gate_kit_roots`' own answer in its own order, which is
// what tells the arm which kits to vendor and in which order to install them; a compiled member has
// no `BASH_SOURCE` anchor to find gate-sdk from instead.
fn kit_roots() -> Result<Vec<String>, Outcome> {
    let roots = walk::kit_roots_abs().map_err(refuse)?;
    if roots.is_empty() {
        return Err(Outcome::Refuse(format!(
            "{}: GATE_KIT_ROOTS_HERE names no kit root, so there is nothing to vendor",
            NAME
        )));
    }
    Ok(roots)
}

fn gate_sdk_root(roots: &[String]) -> Result<String, Outcome> {
    roots
        .iter()
        .find(|r| basename(r) == "gate-sdk")
        .cloned()
        .ok_or_else(|| {
            Outcome::Refuse(format!(
                "{}: GATE_KIT_ROOTS_HERE names no gate-sdk root, so neither the consumer-smoke \
                 library nor the craftable violation this walkthrough fires can be found",
                NAME
            ))
        })
}

fn parent_of(dir: &str) -> Result<String, Outcome> {
    match dir.trim_end_matches('/').rsplit_once('/') {
        Some((head, _)) if !head.is_empty() => Ok(head.to_string()),
        _ => Err(Outcome::Refuse(format!(
            "{}: the gate-sdk root {} has no parent checkout to take a gate binary from",
            NAME, dir
        ))),
    }
}

fn basename(p: &str) -> String {
    p.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(p)
        .to_string()
}

fn refuse(e: String) -> Outcome {
    Outcome::Refuse(format!("{}: {}", NAME, e))
}

fn native_bin() -> Result<String, Outcome> {
    walk::knob_scalar("GATE_SDK_NATIVE_BIN").map_err(refuse)
}

// spec: gate-sdk/SPEC.md §Consumer smoke — `DEMO_TMP_DIR`, then `TMPDIR`, then `/tmp`, read
// straight off the process environment: the base a scratch git repository and a full kit vendoring
// are built under and torn down, which is the builder's own base and not the in-repo `.tmp`.
fn make_scratch(scratch: &mut Scratch) -> Result<String, Outcome> {
    let base = std::env::var("DEMO_TMP_DIR")
        .ok()
        .filter(|v| !v.is_empty())
        .or_else(|| std::env::var("TMPDIR").ok().filter(|v| !v.is_empty()))
        .unwrap_or_else(|| "/tmp".to_string());
    let template = format!("{}/demo-consumer.XXXXXX", base.trim_end_matches('/'));
    let made = proc::run("mktemp", &["-d", &template]).map_err(refuse)?;
    let dir = made
        .stdout()
        .map(|o| String::from_utf8_lossy(o).trim().to_string())
        .filter(|d| !d.is_empty())
        .ok_or_else(|| {
            Outcome::Refuse(format!(
                "{}: cannot create the scratch consumer under {}",
                NAME, base
            ))
        })?;
    scratch.dir = dir.clone();
    Ok(dir)
}

fn write(path: &str, body: &str) -> Result<(), Outcome> {
    std::fs::write(path, body)
        .map_err(|e| Outcome::Refuse(format!("{}: cannot write {}: {}", NAME, path, e)))
}

fn git(repo: &str, args: &[&str]) -> Result<(), Outcome> {
    let mut argv: Vec<&str> = vec!["-C", repo];
    argv.extend_from_slice(args);
    let done = proc::run("git", &argv).map_err(refuse)?;
    match done.failure_report() {
        None => Ok(()),
        Some(r) => Err(Outcome::Refuse(format!(
            "{}: git {} failed in {} — {}",
            NAME,
            args.join(" "),
            repo,
            r
        ))),
    }
}

fn commit(repo: &str, extra: &[&str], message: &str) -> Result<(), Outcome> {
    let mut argv: Vec<&str> = vec![
        "-c",
        "user.email=demo@example.invalid",
        "-c",
        "user.name=demo",
        "commit",
        "-q",
    ];
    argv.extend_from_slice(extra);
    argv.extend_from_slice(&["-m", message]);
    git(repo, &argv)
}

fn vendor(root: &str, consumer: &str) -> Result<(), Outcome> {
    let into = format!("{}/{}", consumer, basename(root));
    let done = proc::run("cp", &["-R", root, &into]).map_err(refuse)?;
    match done.failure_report() {
        None => Ok(()),
        Some(r) => Err(Outcome::Refuse(format!(
            "{}: could not vendor {} into the scratch consumer — {}",
            NAME, root, r
        ))),
    }
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the binary placement is environment-class: a scratch
// consumer that could not receive the artifact its vendored descriptors dispatch to is a harness
// that could not be stood up, never a statement about the adoption arc.
fn place_binary(sdk: &str, consumer: &str, host: &str, roots: &[String]) -> Result<(), Outcome> {
    let code = csmoke::place_binary(sdk, consumer, host, roots).map_err(refuse)?;
    if code != 0 {
        return Err(Outcome::Refuse(format!(
            "{}: the native gate binary could not be placed in the scratch consumer",
            NAME
        )));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §Consumer smoke — a `smoke/install.sh` runs with cwd = the scratch
// consumer and `SMOKE_KIT_ROOT` = the vendored copy, and its own narration is part of what the
// walkthrough shows, so it is inherited rather than captured. A non-zero exit is environment-class.
fn install(consumer: &str, kit: &str) -> Result<(), Outcome> {
    let script = r#"cd "$1" || exit 2; export SMOKE_KIT_ROOT="$1/$2"; exec bash "$1/$2/smoke/install.sh""#;
    let code = proc::run_to_env(
        "bash",
        &["-c", script, "bash", consumer, kit],
        &[],
        &Sink::Inherit,
    )
    .map_err(refuse)?;
    if code != 0 {
        return Err(Outcome::Refuse(format!(
            "{}: {} installer errored",
            NAME, kit
        )));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the battery under test is the *vendored consumer's own*
// front-end, spawned in its tree: an in-process call would run the host's registry against the
// scratch tree, which is the pairing defect the placement's own record names.
fn run_battery(consumer: &str) -> Result<(i32, String), Outcome> {
    let m = proc::run_merged_in(
        "bash",
        &["gate-sdk/bin/run-gates.sh"],
        &[("GATE_SDK_VERBOSE".to_string(), "1".to_string())],
        Some(std::path::Path::new(consumer)),
    )
    .map_err(refuse)?;
    Ok((
        m.reported_code(),
        String::from_utf8_lossy(m.output()).into_owned(),
    ))
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the positive green token, matched on the summary line's
// own grammar rather than on a gate count, and echoed back as the act's one-line result.
fn green_line(out: &str) -> Result<String, Outcome> {
    let re = Ere::compile("All [0-9]+ gates passed").map_err(|e| refuse(e.to_string()))?;
    let hits: Vec<&str> = out.lines().filter(|l| re.is_match(l)).collect();
    Ok(hits.join("\n"))
}

fn ends_with_newline(out: &str) -> String {
    if out.ends_with('\n') {
        out.to_string()
    } else {
        format!("{}\n", out)
    }
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the crafted violation runs under the same cwd/env
// contract every `smoke/violation.sh` has, and its first stdout line is the expected gate name the
// red-phase assertion reads; its own chatter stays on the terminal as the shell form's did.
fn fire_violation(consumer: &str, sdk_kit: &str) -> Result<String, Outcome> {
    let script =
        r#"cd "$1" || exit 2; export SMOKE_KIT_ROOT="$1/$2"; exec bash "$1/$2/smoke/violation.sh""#;
    let done = csmoke::spawn(script, &[consumer, sdk_kit], Stderr::Inherit).map_err(refuse)?;
    Ok(String::from_utf8_lossy(done.stdout())
        .lines()
        .next()
        .unwrap_or("")
        .to_string())
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the reddened gate's own block quoted back, from its
// section header through its `FAIL:` line, so the reader sees the finding and the help line rather
// than being told they exist.
fn excerpt(out: &str, gate: &str) -> Vec<String> {
    let head = format!("===== {} =====", gate);
    let tail = format!("FAIL: {}", gate);
    let mut quoted = Vec::new();
    let mut on = false;
    for line in out.lines() {
        if line.contains(&head) {
            on = true;
        }
        if !on {
            continue;
        }
        quoted.push(format!("  | {}", line));
        if line.contains(&tail) {
            break;
        }
    }
    quoted
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §Consumer smoke — the walkthrough takes no argument, so any word is a
    // refusal rather than a mode the caller believes it selected.
    #[test]
    fn the_member_takes_no_operand() {
        assert_eq!(run(&["--keep".to_string()]), 2);
        assert_eq!(run(&["--nope".to_string()]), 2);
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — the gate-sdk root leads nothing here: the vendoring
    // keeps `gate_kit_roots`' own order, and the root is located rather than reordered because the
    // violation and the library both live under it.
    #[test]
    fn the_gate_sdk_root_is_located_without_reordering() {
        let roots = vec!["/a/b/gate-sdk".to_string(), "/a/b/drift-kit".to_string()];
        assert!(matches!(gate_sdk_root(&roots), Ok(ref r) if r == "/a/b/gate-sdk"));
        assert!(gate_sdk_root(&["/a/b/drift-kit".to_string()]).is_err());
        assert!(matches!(parent_of("/a/b/gate-sdk"), Ok(ref h) if h == "/a/b"));
        assert!(parent_of("/gate-sdk").is_err(), "a root-level kit has no host checkout");
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — the excerpt runs from the gate's section header to
    // its own `FAIL:` line and stops there, so a later gate's block never joins the quote.
    #[test]
    fn the_excerpt_stops_at_its_own_fail_line() {
        let out = "before\n===== check-x =====\nfinding\nFAIL: check-x (exit 1)\n\
                   ===== check-y =====\nFAIL: check-y (exit 1)\n";
        assert_eq!(
            excerpt(out, "check-x"),
            vec![
                "  | ===== check-x =====".to_string(),
                "  | finding".to_string(),
                "  | FAIL: check-x (exit 1)".to_string(),
            ]
        );
        assert!(excerpt(out, "check-absent").is_empty());
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — the green token is the summary line's own grammar
    // and not a gate count, so a battery whose roster grew still matches and a red one does not.
    #[test]
    fn the_green_token_matches_the_summary_grammar_alone() {
        assert!(matches!(
            green_line("noise\nAll 108 gates passed\n"),
            Ok(ref l) if l == "All 108 gates passed"
        ));
        assert!(matches!(
            green_line("1 of 108 gates FAILED: check-x\n"),
            Ok(ref l) if l.is_empty()
        ));
    }
}
