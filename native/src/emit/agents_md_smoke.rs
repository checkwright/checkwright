// spec: context-kit/SPEC.md §Testing — the AGENTS.md agent-file adapter smoke, bridged as an
// `Arm::Run` member: the contract is a verdict — 0 with the clean line, 1 with a `FAIL — <reason>`
// line, 2 a precondition the harness could not meet — which `Arm::Emit` cannot carry.
// spec: context-kit/SPEC.md §Testing — the vendoring calls `lib/consumer-smoke.sh`'s helpers in the
// library that owns them, through the shared spawn seam, so the port creates no second producer.
use crate::emit::csmoke;
use crate::ere::Ere;
use crate::proc::{self, Stderr};
use crate::walk;
use std::path::Path;

// spec: context-kit/SPEC.md §Testing — the roster is the *consumer's*: which kits to vendor, and
// the binary to place. No `CONTEXT_KIT_` knob is declared — every agent-file knob this smoke
// touches it writes into the scratch consumer's own config seams rather than resolving here.
pub const KNOBS: &[&str] = &["GATE_KIT_ROOTS_HERE", "GATE_SDK_NATIVE_BIN"];

const NAME: &str = "agents-md";
const VERDICT: &str = "AGENTS-MD-SMOKE";
const USAGE: &str = "usage: --agents-md-smoke [--keep]";

// spec: context-kit/SPEC.md §Testing — the three exit classes as a type, so a finding about the
// AGENTS.md consumer cannot be raised on a precondition's spelling or the reverse.
enum Outcome {
    Clean(String),
    Fail(Vec<String>),
    Refuse(String),
}

fn fail(line: String) -> Outcome {
    Outcome::Fail(vec![format!("{}: FAIL — {}", VERDICT, line)])
}

// spec: context-kit/SPEC.md §Testing — both scratch trees are removed on every exit path and
// `--keep` suppresses both; the shell form's re-armed `trap` could leak the first tree on a failure
// between the two arms, which is the property this shape gains rather than transcribes.
struct Scratch {
    consumer: String,
    orientation: String,
    keep: bool,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if self.consumer.is_empty() {
            return;
        }
        if self.keep {
            println!("{}: --keep, scratch retained at {}", VERDICT, self.consumer);
            return;
        }
        let _ = std::fs::remove_dir_all(&self.consumer);
        if !self.orientation.is_empty() {
            let _ = std::fs::remove_dir_all(&self.orientation);
        }
    }
}

pub fn run(args: &[String]) -> i32 {
    let mut keep = false;
    for a in args {
        if a == "--keep" {
            keep = true;
        } else {
            eprintln!("{}: unknown option: {}; {}", NAME, a, USAGE);
            return 2;
        }
    }
    let mut scratch = Scratch {
        consumer: String::new(),
        orientation: String::new(),
        keep,
    };
    let code = match smoke(&mut scratch) {
        Outcome::Clean(line) => {
            println!("{}", line);
            0
        }
        Outcome::Fail(lines) => {
            for l in &lines {
                println!("{}", l);
            }
            1
        }
        Outcome::Refuse(line) => {
            eprintln!("{}", line);
            2
        }
    };
    code
}

macro_rules! step {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(o) => return o,
        }
    };
}

// spec: context-kit/SPEC.md §Testing — the suite in the shell driver's order: vendor, convert the
// agent file, write the four config seams, regenerate under the battery env, then the four
// assertions.
fn smoke(scratch: &mut Scratch) -> Outcome {
    let roots = step!(kit_roots());
    let sdk = roots[0].clone();
    let host = step!(parent_of(&sdk));
    for r in &roots {
        if !Path::new(&format!("{}/smoke/install.sh", r)).is_file() {
            return Outcome::Refuse(format!(
                "{}: {} has no smoke/install.sh",
                NAME,
                basename(r)
            ));
        }
    }

    let consumer = step!(vendor(scratch, &sdk, &host, &roots));
    step!(convert_agent_file(&consumer));
    step!(write_config_seams(&consumer));
    step!(commit(&consumer, "convert to AGENTS.md"));
    step!(regenerate(&consumer));
    step!(commit(
        &consumer,
        "regenerate the hook + graph artifact under the AGENTS.md config"
    ));

    step!(battery_is_green(&consumer));
    step!(always_loaded_measures_the_agent_file(&consumer));
    step!(footprint_measures_the_agent_file(&consumer));
    step!(root_tiering_accepts_one_agent_file(scratch, &consumer));

    Outcome::Clean(format!(
        "{}: clean (battery green, always-loaded + footprint measure AGENTS.md, root-tiering accepts one agent file and rejects a second)",
        VERDICT
    ))
}

// spec: context-kit/SPEC.md §Testing — `gate_kit_roots`' own answer with gate-sdk first, which is
// what tells the arm which kits to vendor; a compiled member has no `BASH_SOURCE` anchor to find
// gate-sdk from instead.
fn kit_roots() -> Result<Vec<String>, Outcome> {
    let roots = walk::kit_roots_abs().map_err(|e| Outcome::Refuse(format!("{}: {}", NAME, e)))?;
    let sdk = roots
        .iter()
        .find(|r| basename(r) == "gate-sdk")
        .cloned()
        .ok_or_else(|| {
            Outcome::Refuse(format!(
                "{}: GATE_KIT_ROOTS_HERE names no gate-sdk root, so the consumer-smoke library this suite vendors through cannot be found",
                NAME
            ))
        })?;
    let mut ordered = vec![sdk.clone()];
    ordered.extend(roots.into_iter().filter(|r| *r != sdk));
    Ok(ordered)
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

// spec: context-kit/SPEC.md §Testing — `csmoke_vendor_and_install` communicates by setting its
// caller's `SCRATCH`, which no process boundary carries, so the seam is the library's own contract
// read off stdout and the helper's own chatter moves to stderr to clear that channel.
fn vendor(
    scratch: &mut Scratch,
    sdk: &str,
    host: &str,
    roots: &[String],
) -> Result<String, Outcome> {
    let script = format!(
        "{} host=\"$2\"; shift 2; csmoke_vendor_and_install \"$host\" \"$@\" 1>&2; \
         st=$?; printf '%s' \"$SCRATCH\"; exit $st",
        csmoke::SOURCE
    );
    let refs: Vec<&str> = vec![sdk, host]
        .into_iter()
        .chain(roots.iter().map(String::as_str))
        .collect();
    let done = spawn(&script, &refs)?;
    let consumer = String::from_utf8_lossy(done.stdout()).trim().to_string();
    scratch.consumer = consumer.clone();
    if done.code() != 0 || consumer.is_empty() {
        return Err(Outcome::Refuse(format!(
            "{}: could not vendor and install the scratch consumer",
            NAME
        )));
    }
    Ok(consumer)
}

fn spawn(script: &str, args: &[&str]) -> Result<proc::Streamed, Outcome> {
    csmoke::spawn(script, args, Stderr::Inherit)
        .map_err(|e| Outcome::Refuse(format!("{}: {}", NAME, e)))
}

// spec: context-kit/SPEC.md §Testing — the installers write the consumer's agent file as
// `CLAUDE.md`; the smoke converts it, as an AGENTS.md adopter would, after asserting there was one.
fn convert_agent_file(consumer: &str) -> Result<(), Outcome> {
    if !Path::new(&format!("{}/CLAUDE.md", consumer)).is_file() {
        return Err(fail(
            "the installed baseline wrote no CLAUDE.md to convert".to_string(),
        ));
    }
    git(consumer, &["mv", "CLAUDE.md", "AGENTS.md"])
}

// spec: context-kit/SPEC.md §Testing — the six knob writes into four config seams, written as file
// content because they are the *consumer's* configuration and not this arm's.
fn write_config_seams(consumer: &str) -> Result<(), Outcome> {
    let seams: [(&str, &str, bool); 4] = [
        (
            "scripts/gate-sdk-config.sh",
            "# shellcheck shell=bash\n# shellcheck disable=SC2034  # read by check-root-tiering after lib/gate.sh sources this seam\nGATE_SDK_AGENT_FILE=\"AGENTS.md\"\n",
            false,
        ),
        (
            "scripts/context-config.sh",
            "# shellcheck disable=SC2034  # read by context-kit bins and check-brevity\nCONTEXT_KIT_SURFACES=(\"AGENTS.md\")\nCONTEXT_KIT_BREVITY_FILE=\"AGENTS.md\"\n",
            true,
        ),
        (
            "scripts/doctrine-config.sh",
            "# shellcheck shell=bash disable=SC2034\nDOCTRINE_KIT_AGENT_FILE=\"AGENTS.md\"\n",
            false,
        ),
        (
            "scripts/canon-config.sh",
            "# shellcheck shell=bash disable=SC2034\nCANON_KIT_MANIFEST_FILES=(\"AGENTS.md\" \"README.md\" \"*/SPEC.md\" \"*/README.md\")\n",
            false,
        ),
    ];
    for (rel, body, append) in seams {
        let path = format!("{}/{}", consumer, rel);
        let written = if append {
            std::fs::read_to_string(&path).unwrap_or_default() + body
        } else {
            body.to_string()
        };
        std::fs::write(&path, written)
            .map_err(|e| Outcome::Refuse(format!("{}: cannot write {}: {}", NAME, path, e)))?;
    }
    Ok(())
}

// spec: context-kit/SPEC.md §Testing — the two battery-env values ride every spawned battery run:
// the lifecycle knob is scalar with no default config file, and canon resolves its manifest only
// through its config file.
fn battery_env() -> Vec<(String, String)> {
    vec![
        (
            "LIFECYCLE_KIT_AGENT_FILE".to_string(),
            "AGENTS.md".to_string(),
        ),
        (
            "CANON_KIT_CONFIG_FILE".to_string(),
            "scripts/canon-config.sh".to_string(),
        ),
    ]
}

// spec: context-kit/SPEC.md §Testing — the regeneration ordering: each kit's install.sh already
// wrote the hook and the graph artifact, but before `canon-config.sh` existed, so both are
// rewritten here under the same env the battery will run with.
fn regenerate(consumer: &str) -> Result<(), Outcome> {
    let hook = proc::run_merged_in(
        "bash",
        &["gate-sdk/bin/gen-pre-commit.sh", "--write"],
        &battery_env(),
        Some(Path::new(consumer)),
    )
    .map_err(|e| Outcome::Refuse(format!("{}: {}", NAME, e)))?;
    if !hook.succeeded() {
        return Err(Outcome::Refuse(format!(
            "{}: could not regenerate the hook under the AGENTS.md config:\n{}",
            NAME,
            String::from_utf8_lossy(hook.output()).trim_end()
        )));
    }
    // spec: context-kit/SPEC.md §Testing — the graph artifact is a redirect rather than a capture,
    // so the emit's own stderr stays out of the committed HTML.
    let graph = spawn(
        r#"cd "$1" && exec env LIFECYCLE_KIT_AGENT_FILE="$2" CANON_KIT_CONFIG_FILE="$3" bash gate-sdk/bin/run-gates.sh --emit graph > scripts/CHECK-GRAPH.html"#,
        &[consumer, "AGENTS.md", "scripts/canon-config.sh"],
    )?;
    if graph.code() != 0 {
        return Err(Outcome::Refuse(format!(
            "{}: could not regenerate scripts/CHECK-GRAPH.html under the AGENTS.md config",
            NAME
        )));
    }
    Ok(())
}

fn commit(consumer: &str, message: &str) -> Result<(), Outcome> {
    git(consumer, &["add", "-A"])?;
    git(
        consumer,
        &[
            "-c",
            "user.email=smoke@example.invalid",
            "-c",
            "user.name=smoke",
            "commit",
            "-q",
            "--no-verify",
            "-m",
            message,
        ],
    )
}

fn git(repo: &str, args: &[&str]) -> Result<(), Outcome> {
    let mut argv: Vec<&str> = vec!["-C", repo];
    argv.extend_from_slice(args);
    let done = proc::run("git", &argv).map_err(|e| Outcome::Refuse(format!("{}: {}", NAME, e)))?;
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

// spec: context-kit/SPEC.md §Testing — assertion 1: the battery green on the AGENTS.md consumer,
// matched on the summary line's own grammar rather than on a gate count.
fn battery_is_green(consumer: &str) -> Result<(), Outcome> {
    let m = proc::run_merged_in(
        "bash",
        &["gate-sdk/bin/run-gates.sh"],
        &battery_env(),
        Some(Path::new(consumer)),
    )
    .map_err(|e| Outcome::Refuse(format!("{}: {}", NAME, e)))?;
    let out = String::from_utf8_lossy(m.output()).into_owned();
    let green = compiled("All [0-9]+ gates passed")?.is_match(&out);
    if m.reported_code() == 0 && green {
        return Ok(());
    }
    Err(Outcome::Fail(vec![
        format!("{}: FAIL — the battery is not green on the AGENTS.md consumer", VERDICT),
        out.trim_end().to_string(),
        "  help: reproduce with --keep and run the battery in the scratch dir with the knobs set."
            .to_string(),
    ]))
}

// spec: context-kit/SPEC.md §Testing — assertion 2: `always-loaded` measures a surface count equal
// to `AGENTS.md`'s line count, which is what proves it read the converted file and not the default.
fn always_loaded_measures_the_agent_file(consumer: &str) -> Result<(), Outcome> {
    let agent = format!("{}/AGENTS.md", consumer);
    let lines = std::fs::read_to_string(&agent)
        .map_err(|e| Outcome::Refuse(format!("{}: cannot read {}: {}", NAME, agent, e)))?
        .matches('\n')
        .count();
    let done = spawn(
        r#"cd "$1" && exec bash gate-sdk/bin/run-gates.sh --emit always-loaded"#,
        &[consumer],
    )?;
    let out = String::from_utf8_lossy(done.stdout()).into_owned();
    if compiled(&format!("surfaces {}( |·)", lines))?.is_match(&out) {
        return Ok(());
    }
    Err(fail(format!(
        "always-loaded did not measure the AGENTS.md surface ({}l): {}",
        lines,
        out.trim_end()
    )))
}

// spec: context-kit/SPEC.md §Testing — assertion 3: a non-zero always-loaded total; against the
// `CLAUDE.md` default the emitter would find no surface file and measure zero.
fn footprint_measures_the_agent_file(consumer: &str) -> Result<(), Outcome> {
    let done = spawn(
        r#"cd "$1" && exec bash gate-sdk/bin/run-gates.sh --emit footprint"#,
        &[consumer],
    )?;
    let out = String::from_utf8_lossy(done.stdout()).into_owned();
    let head = compiled(r"^\| \*\*total\*\*")?;
    let total: Vec<&str> = out.lines().filter(|l| head.is_match(l)).collect();
    let joined = total.join("\n");
    if compiled(r"\*\*total\*\* \| [1-9][0-9]*l")?.is_match(&joined) {
        return Ok(());
    }
    Err(fail(format!(
        "footprint measured no AGENTS.md always-loaded surface: {}",
        joined
    )))
}

// spec: context-kit/SPEC.md §Testing — assertion 4 runs in a dedicated orientation-clean repo: the
// vendored scratch is a kit monorepo whose built-in root check reds on the kit dirs whatever the
// agent file is, so the knob's real surface is the zero-allowlist fallback set.
fn root_tiering_accepts_one_agent_file(
    scratch: &mut Scratch,
    consumer: &str,
) -> Result<(), Outcome> {
    let rt = orientation_repo(scratch)?;
    let argv = root_tiering_dispatch(consumer)?;

    let (rc, out) = run_dispatch(&argv, &rt)?;
    if rc != 0 {
        return Err(fail(format!(
            "check-root-tiering rejected an orientation-clean AGENTS.md root: {}",
            out
        )));
    }

    std::fs::write(format!("{}/CLAUDE.md", rt), "")
        .map_err(|e| Outcome::Refuse(format!("{}: cannot write the stray agent file: {}", NAME, e)))?;
    git(&rt, &["add", "CLAUDE.md"])?;
    let (rc, out) = run_dispatch(&argv, &rt)?;
    if rc == 0 {
        return Err(fail(
            "check-root-tiering accepted a stray second agent file (CLAUDE.md beside AGENTS.md)"
                .to_string(),
        ));
    }
    if !out.contains("CLAUDE.md") {
        return Err(fail(format!(
            "check-root-tiering rejected the wrong entry (expected the stray CLAUDE.md): {}",
            out
        )));
    }
    Ok(())
}

fn orientation_repo(scratch: &mut Scratch) -> Result<String, Outcome> {
    let tmp = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
    let template = format!("{}/agents-md-rt.XXXXXX", tmp.trim_end_matches('/'));
    let made = proc::run("mktemp", &["-d", &template])
        .map_err(|e| Outcome::Refuse(format!("{}: {}", NAME, e)))?;
    let rt = made
        .stdout()
        .map(|o| String::from_utf8_lossy(o).trim().to_string())
        .filter(|r| !r.is_empty())
        .ok_or_else(|| {
            Outcome::Refuse(format!("{}: cannot create the orientation repo", NAME))
        })?;
    scratch.orientation = rt.clone();
    git(&rt, &["init", "-q"])?;
    for f in ["README.md", "AGENTS.md"] {
        std::fs::write(format!("{}/{}", rt, f), "")
            .map_err(|e| Outcome::Refuse(format!("{}: cannot seed {}: {}", NAME, f, e)))?;
    }
    git(&rt, &["add", "-A"])?;
    git(
        &rt,
        &[
            "-c",
            "user.email=smoke@example.invalid",
            "-c",
            "user.name=smoke",
            "commit",
            "-q",
            "-m",
            "seed",
        ],
    )?;
    Ok(rt)
}

// spec: context-kit/SPEC.md §Testing — the dispatch is resolved once against the *vendored* tree,
// whose knobs and binary are the consumer's, and that resolution stays on the shell side: a literal
// `checks/<gate>.sh` path would name a substrate the port has moved.
fn root_tiering_dispatch(consumer: &str) -> Result<Vec<String>, Outcome> {
    let mut bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN")
        .map_err(|e| Outcome::Refuse(format!("{}: {}", NAME, e)))?;
    if !bin.starts_with('/') {
        bin = format!("{}/{}", consumer, bin);
    }
    let done = spawn(
        r#"cd "$1" && exec env GATE_SDK_AGENT_FILE="AGENTS.md" GATE_SDK_NATIVE_BIN="$2" bash -c 'source gate-sdk/lib/gate.sh; gate_command check-root-tiering gate-sdk/checks'"#,
        &[consumer, &bin],
    )?;
    if done.code() != 0 {
        return Err(fail(
            "could not resolve check-root-tiering's dispatch in the vendored tree".to_string(),
        ));
    }
    let argv: Vec<String> = String::from_utf8_lossy(done.stdout())
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();
    if argv.is_empty() {
        return Err(fail(
            "check-root-tiering resolves to nothing in the vendored tree".to_string(),
        ));
    }
    Ok(argv)
}

fn run_dispatch(argv: &[String], cwd: &str) -> Result<(i32, String), Outcome> {
    let rest: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    let m = proc::run_merged_in(&argv[0], &rest, &[], Some(Path::new(cwd)))
        .map_err(|e| Outcome::Refuse(format!("{}: {}", NAME, e)))?;
    Ok((
        m.reported_code(),
        String::from_utf8_lossy(m.output())
            .trim_end_matches('\n')
            .to_string(),
    ))
}

fn compiled(pattern: &str) -> Result<Ere, Outcome> {
    Ere::compile(pattern).map_err(|e| {
        Outcome::Refuse(format!(
            "{}: cannot compile the assertion pattern {}: {}",
            NAME, pattern, e
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §Testing — `--keep` is an argument the rule itself consumes, so
    // anything else is a refusal rather than a silently ignored word.
    #[test]
    fn the_member_takes_only_keep() {
        assert_eq!(run(&["--nope".to_string()]), 2);
    }

    // spec: context-kit/SPEC.md §Testing — the gate-sdk root leads the vendoring order and appears
    // once, which is what `csmoke_vendor_and_install`'s gate-sdk-first contract reads.
    #[test]
    fn the_parent_checkout_is_the_gate_sdk_roots_own() {
        assert!(matches!(parent_of("/a/b/gate-sdk"), Ok(ref h) if h == "/a/b"));
        assert!(matches!(parent_of("/a/b/gate-sdk/"), Ok(ref h) if h == "/a/b"));
        assert!(parent_of("/gate-sdk").is_err(), "a root-level kit has no host checkout");
    }
}
