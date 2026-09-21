// spec: gate-sdk/SPEC.md §Consumer smoke — the adoption walkthrough as an `Arm::Run`
// member: its contract is the verdict — 0 the arc behaved, 1 an act of it did not, 2 a
// precondition the harness could not meet — which `Arm::Emit` cannot carry.
// spec: gate-sdk/SPEC.md §Consumer smoke — the binary placement calls the in-crate scratch-consumer
// builder's own placement, so the walkthrough holds no second copy of it.
use crate::emit::csmoke;
use crate::proc::{self, Sink, Stderr};
use crate::programs;
use crate::walk;
use crate::walkthrough::{banner, ends_with_newline, excerpt, say, Scratch};

// spec: gate-sdk/SPEC.md §Consumer smoke — the roster is the *consumer's*: which kits the
// walkthrough vendors, and the binary the scratch consumer receives. `DEMO_TMP_DIR` is read below
// and may not join this roster (§The non-gate arm).
pub const KNOBS: &[&str] = &["GATE_SDK_KIT_DIRS", "GATE_SDK_NATIVE_BIN"];

const NAME: &str = "run-demo";
const VERDICT: &str = "DEMO";
const USAGE: &str = "usage: --run-demo";

// spec: gate-sdk/SPEC.md §Consumer smoke — the three exit classes as a type, so a finding about the
// adoption arc cannot be raised on a precondition's spelling or the reverse.
enum Outcome {
    Clean,
    Fail(String),
    Refuse(String),
}

// spec: gate-sdk/SPEC.md §Consumer smoke — an operand is a refusal rather than a silently ignored
// word: once 2 means "the harness could not be stood up", a swallowed operand is the one way a
// caller believes it selected a mode and then reads a verdict about a different run.
pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        eprintln!("{}: unknown option: {}; {}", NAME, a, USAGE);
        return 2;
    }
    let mut scratch = Scratch { dir: None };
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
    step!(place_binary(&consumer, &host, &roots));
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
            "{}: the kit roots name no kit root, so there is nothing to vendor",
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
                "{}: the kit roots name no gate-sdk root, so the craftable violation this \
                 walkthrough fires cannot be found",
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
    let made = proc::run(&programs::MKTEMP, &["-d", &template]).map_err(refuse)?;
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
    scratch.dir = Some(dir.clone().into());
    Ok(dir)
}

fn write(path: &str, body: &str) -> Result<(), Outcome> {
    std::fs::write(path, body)
        .map_err(|e| Outcome::Refuse(format!("{}: cannot write {}: {}", NAME, path, e)))
}

fn git(repo: &str, args: &[&str]) -> Result<(), Outcome> {
    let mut argv: Vec<&str> = vec!["-C", repo];
    argv.extend_from_slice(args);
    let done = proc::run(&programs::GIT, &argv).map_err(refuse)?;
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
    let done = proc::run(&programs::CP, &["-R", root, &into]).map_err(refuse)?;
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
fn place_binary(consumer: &str, host: &str, roots: &[String]) -> Result<(), Outcome> {
    csmoke::place_binary(consumer, host, roots).map_err(|e| {
        Outcome::Refuse(format!(
            "{}: the native gate binary could not be placed in the scratch consumer — {}",
            NAME,
            e.lines().join("\n")
        ))
    })
}

// spec: gate-sdk/SPEC.md §Consumer smoke — a `smoke/install.sh` runs with cwd = the scratch
// consumer and `SMOKE_KIT_ROOT` = the vendored copy, and its own narration is part of what the
// walkthrough shows, so it is inherited rather than captured. A non-zero exit is environment-class.
fn install(consumer: &str, kit: &str) -> Result<(), Outcome> {
    let script = r#"cd "$1" || exit 2; export GATE_SDK_ROOT="$1/gate-sdk" SMOKE_KIT_ROOT="$1/$2"; exec bash "$1/$2/smoke/install.sh""#;
    let code = proc::run_to_env(
        &programs::BASH,
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
        &programs::BASH,
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

fn green_line(out: &str) -> Result<String, Outcome> {
    crate::walkthrough::green_line(out).map_err(refuse)
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the crafted violation runs under the same cwd/env
// contract every `smoke/violation.sh` has, and its first stdout line is the expected gate name the
// red-phase assertion reads; its own chatter stays on the terminal as the shell form's did.
fn fire_violation(consumer: &str, sdk_kit: &str) -> Result<String, Outcome> {
    let script =
        r#"cd "$1" || exit 2; export GATE_SDK_ROOT="$1/gate-sdk" SMOKE_KIT_ROOT="$1/$2"; exec bash "$1/$2/smoke/violation.sh""#;
    let done = proc::run_streamed(&programs::BASH, &["-c", script, "bash", consumer, sdk_kit], b"", Stderr::Inherit)
        .map_err(refuse)?;
    Ok(String::from_utf8_lossy(done.stdout())
        .lines()
        .next()
        .unwrap_or("")
        .to_string())
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
    // craftable violation lives under it.
    #[test]
    fn the_gate_sdk_root_is_located_without_reordering() {
        let roots = vec!["/a/b/gate-sdk".to_string(), "/a/b/drift-kit".to_string()];
        assert!(matches!(gate_sdk_root(&roots), Ok(ref r) if r == "/a/b/gate-sdk"));
        assert!(gate_sdk_root(&["/a/b/drift-kit".to_string()]).is_err());
        assert!(matches!(parent_of("/a/b/gate-sdk"), Ok(ref h) if h == "/a/b"));
        assert!(parent_of("/gate-sdk").is_err(), "a root-level kit has no host checkout");
    }
}
