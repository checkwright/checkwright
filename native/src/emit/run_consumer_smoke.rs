// spec: gate-sdk/SPEC.md §Consumer smoke — the scratch-consumer install, accounting and violation
// harness, an `Arm::Run` member: its contract is the 0/1/2 exit status an emitting arm collapses.
use crate::emit::csmoke;
use crate::proc::{self, Sink};
use crate::programs;
use crate::registry;
use crate::walk;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::time::Instant;

// spec: gate-sdk/SPEC.md §Consumer smoke — the roster is the consumer's: which kits to vendor and
// the binary the scratch consumer receives and the probes dispatch to.
pub const KNOBS: &[&str] = &["GATE_SDK_KIT_DIRS", "GATE_SDK_NATIVE_BIN"];

const NAME: &str = "run-consumer-smoke";
const VERDICT: &str = "CONSUMER-SMOKE";

// spec: gate-sdk/SPEC.md §Consumer smoke — the caller owns cleanup: the scratch is removed on every
// exit path, or retained and named under `--keep`.
struct Teardown {
    dir: String,
    keep: bool,
}

impl Drop for Teardown {
    fn drop(&mut self) {
        if self.dir.is_empty() {
            return;
        }
        if self.keep {
            println!("{}: --keep, scratch retained at {}", VERDICT, self.dir);
        } else {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }
}

// spec: gate-sdk/SPEC.md §Consumer smoke — exit 2 is a diagnostic on stderr, exit 1 the verdict
// lines on stdout
enum Outcome {
    Clean(String),
    Fail(Vec<String>),
    Env(Vec<String>),
}

fn env(line: String) -> Outcome {
    Outcome::Env(vec![format!("{}: {}", NAME, line)])
}

macro_rules! step {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(o) => return o,
        }
    };
}

pub fn run(args: &[String]) -> i32 {
    let mut keep = false;
    let mut given: Vec<String> = Vec::new();
    for a in args {
        if a == "--keep" {
            keep = true;
        } else if a.starts_with('-') {
            eprintln!("{}: unknown option: {}", NAME, a);
            return 2;
        } else {
            given.push(a.clone());
        }
    }
    let mut teardown = Teardown {
        dir: String::new(),
        keep,
    };
    let outcome = smoke(&given, &mut teardown);
    match outcome {
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
        Outcome::Env(lines) => {
            for l in &lines {
                eprintln!("{}", l);
            }
            2
        }
    }
}

fn basename(p: &str) -> String {
    p.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(p)
        .to_string()
}

fn parent_of(p: &str) -> String {
    match p.trim_end_matches('/').rsplit_once('/') {
        Some(("", _)) | None => "/".to_string(),
        Some((head, _)) => head.to_string(),
    }
}

// spec: gate-sdk/SPEC.md §Consumer smoke — operands name the kit roots, else the derived set;
// the invoking gate-sdk root always leads, since every later installer may assume it is installed
fn kit_roots(given: &[String]) -> Result<(String, Vec<String>), Outcome> {
    let here = walk::cwd().map_err(env)?;
    let sdk = walk::abs_against(&here, walk::sdk_root().trim_end_matches('/'));
    let roots: Vec<String> = if given.is_empty() {
        walk::kit_roots_abs().map_err(env)?
    } else {
        let mut out = Vec::new();
        for r in given {
            if !Path::new(r).is_dir() {
                return Err(env(format!("not a directory: {}", r)));
            }
            out.push(walk::abs_against(&here, r.trim_end_matches('/')));
        }
        out
    };
    let mut ordered = vec![sdk.clone()];
    ordered.extend(roots.into_iter().filter(|r| *r != sdk));
    Ok((sdk, ordered))
}

fn smoke(given: &[String], teardown: &mut Teardown) -> Outcome {
    let (sdk, roots) = step!(kit_roots(given));
    let clean = match pass(&sdk, &roots, teardown, &Sink::Inherit, true) {
        Outcome::Clean(line) => line,
        other => return other,
    };
    if roots.len() <= 2 {
        return Outcome::Clean(clean);
    }
    let started = Instant::now();
    for (i, r) in roots.iter().enumerate() {
        let alone: Vec<String> = if i == 0 {
            vec![sdk.clone()]
        } else {
            vec![sdk.clone(), r.clone()]
        };
        let mut own = Teardown {
            dir: String::new(),
            keep: false,
        };
        // spec: gate-sdk/SPEC.md §Consumer smoke — the union run already ran this installer green in
        // this environment, so a red alone is the kit's finding at exit 1, never the exit-2 band
        let lines = match pass(&sdk, &alone, &mut own, &Sink::Stderr, false) {
            Outcome::Clean(_) => continue,
            Outcome::Fail(l) | Outcome::Env(l) => l,
        };
        let mut report = vec![format!(
            "{}: FAIL — {} is not green alone (the self-sufficiency phase)",
            VERDICT,
            basename(r)
        )];
        report.extend(lines);
        report.push(format!(
            "  help: reproduce with bash gate-sdk/bin/run-gates.sh --run-consumer-smoke --keep {}",
            r
        ));
        return Outcome::Fail(report);
    }
    println!(
        "{}: alone — {} kit root(s) green alone in {}ms",
        VERDICT,
        roots.len(),
        started.elapsed().as_millis()
    );
    Outcome::Clean(clean)
}

// spec: gate-sdk/SPEC.md §Consumer smoke — one pass over one vendoring: install, zero-config green,
// accounting, each kit's violation, final green; `verbose` gates the verdict lines a green pass prints
fn pass(sdk: &str, roots: &[String], teardown: &mut Teardown, sink: &Sink, verbose: bool) -> Outcome {
    let say = |l: &str| {
        if verbose {
            println!("{}", l);
        }
    };
    for r in roots {
        if !Path::new(&format!("{}/smoke/install.sh", r)).is_file() {
            // spec: gate-sdk/SPEC.md §Consumer smoke — the refusal names its vendored-tree cause,
            // so a tree installed from the payload reads it as a boundary and not a broken install
            return Outcome::Env(vec![
                format!(
                    "{}: {} has no smoke/install.sh — a vendored kit must ship one",
                    NAME, r
                ),
                "  help: add smoke/install.sh (+ optional smoke/violation.sh); see gate-sdk/SPEC.md §Consumer smoke.".to_string(),
                "  help: in a tree installed from the customer payload this is expected on every kit — the payload withholds smoke/ (gate-sdk/SPEC.md §Consumer payload). This arm asks whether the kits work when vendored BY COPY, so it runs against a checkout of the kit sources, not against an installed tree.".to_string(),
            ]);
        }
    }

    let host = parent_of(sdk);
    let base = std::env::var("TMPDIR")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "/tmp".to_string());
    let built = match csmoke::vendor_and_install(&host, roots, &base, sink) {
        Ok(s) => s,
        Err(e) => {
            teardown.dir = e.dir;
            return env(e.message);
        }
    };
    teardown.dir = built.dir.clone();
    let scratch = built.dir.as_str();

    let (rc, out) = step!(battery(scratch));
    if rc != 0 || !green(&out) {
        return Outcome::Fail(vec![
            format!("{}: FAIL — the battery is not green on the freshly installed consumer (zero config)", VERDICT),
            out,
            "  help: an install step left a gate red; reproduce with --keep and run gate-sdk/bin/run-gates.sh in the scratch dir.".to_string(),
        ]);
    }

    let acct = step!(account(scratch, &host, roots));
    step!(restore(scratch));

    let mut report = vec![format!(
        "{}: accounting — {} unregistered gate(s) probed in {}ms ({} self-declared, {} hand-declared, {} unaccounted)",
        VERDICT,
        acct.probed,
        acct.ms,
        acct.self_declared,
        acct.hand_declared,
        acct.unaccounted.len()
    )];
    for l in &acct.contradicted {
        report.push(format!("  contradicted declaration: {}", l));
    }
    if !acct.unaccounted.is_empty() || !acct.stale.is_empty() {
        report.push(format!(
            "{}: FAIL — the registration accounting is not satisfied",
            VERDICT
        ));
        report.extend(accounting_failure_lines(&acct));
        return Outcome::Fail(report);
    }
    for l in &report {
        say(l);
    }

    let mut fired = 0;
    for r in roots {
        let kit = basename(r);
        let vio = format!("{}/{}/smoke/violation.sh", scratch, kit);
        if !Path::new(&vio).is_file() {
            say(&format!(
                "{}: {} has no violation script — install coverage only",
                VERDICT, kit
            ));
            continue;
        }
        let expected = step!(fire(scratch, &kit, &vio));
        if expected.is_empty() {
            step!(restore(scratch));
            return Outcome::Fail(vec![format!(
                "{}: FAIL — {}/smoke/violation.sh printed no expected-gate name on line 1",
                VERDICT, kit
            )]);
        }
        let log = format!("{}/.git/consumer-smoke.sarif", scratch);
        let _ = std::fs::remove_file(&log);
        let (rc, out) = step!(battery_with(scratch, &[(crate::sarif::KNOB.to_string(), log.clone())]));
        if rc == 0 {
            step!(restore(scratch));
            return Outcome::Fail(vec![
                format!(
                    "{}: FAIL — {} violation did not turn the battery red (expected gate {})",
                    VERDICT, kit, expected
                ),
                out,
            ]);
        }
        if !out.contains(&format!("FAIL: {}", expected)) {
            step!(restore(scratch));
            return Outcome::Fail(vec![
                format!(
                    "{}: FAIL — {} violation fired, but no 'FAIL: {}' line (wrong gate caught it)",
                    VERDICT, kit, expected
                ),
                out,
            ]);
        }
        if !sarif_names(&log, &expected) {
            step!(restore(scratch));
            return Outcome::Fail(vec![
                format!(
                    "{}: FAIL — {} violation fired, but the SARIF log at {} holds no result with rule id {}",
                    VERDICT, kit, log, expected
                ),
                "  help: the --run arm writes the log GATE_SDK_SARIF_FILE names; see gate-sdk/SPEC.md §run-gates.".to_string(),
            ]);
        }
        step!(restore(scratch));
        fired += 1;
    }

    let (rc, out) = step!(battery(scratch));
    if rc != 0 || !green(&out) {
        return Outcome::Fail(vec![
            format!(
                "{}: FAIL — the battery did not return to green after the final restore",
                VERDICT
            ),
            out,
        ]);
    }

    Outcome::Clean(format!(
        "{}: clean ({} kits installed, {} violations fired, {} gates registered, {} self-declared, {} hand-declared)",
        VERDICT, built.installed, fired, acct.registered, acct.self_declared, acct.hand_declared
    ))
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the battery under test is the scratch consumer's own
// front-end, spawned in its tree with both streams merged; the output is carried with its trailing
// newlines stripped, as a command substitution holds it
fn battery(scratch: &str) -> Result<(i32, String), Outcome> {
    battery_with(scratch, &[])
}

fn battery_with(scratch: &str, env_extra: &[(String, String)]) -> Result<(i32, String), Outcome> {
    let m = proc::run_merged_in(
        &programs::BASH,
        &["gate-sdk/bin/run-gates.sh"],
        env_extra,
        Some(Path::new(scratch)),
    )
    .map_err(env)?;
    Ok((
        m.reported_code(),
        String::from_utf8_lossy(m.output())
            .trim_end_matches('\n')
            .to_string(),
    ))
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the violation run's SARIF log holds a result carrying
// the expected gate's rule id
fn sarif_names(log: &str, gate: &str) -> bool {
    let Ok(text) = std::fs::read_to_string(log) else {
        return false;
    };
    let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else {
        return false;
    };
    doc["runs"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|r| r["results"].as_array())
        .flatten()
        .any(|r| r["ruleId"] == gate)
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the positive green token, matched on the summary line's
// grammar rather than on a gate count
fn green(out: &str) -> bool {
    out.lines().any(|l| {
        l.split("All ").skip(1).any(|rest| {
            let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
            digits > 0 && rest[digits..].starts_with(" gates passed")
        })
    })
}

// spec: gate-sdk/SPEC.md §Consumer smoke — a hard reset, not a checkout, so a violation that staged
// its shape is unstaged too
fn restore(scratch: &str) -> Result<(), Outcome> {
    for args in [&["reset", "-q", "--hard"][..], &["clean", "-qfd"][..]] {
        let mut argv: Vec<&str> = vec!["-C", scratch];
        argv.extend_from_slice(args);
        let done = proc::run(&programs::GIT, &argv).map_err(env)?;
        if let Some(r) = done.failure_report() {
            return Err(env(format!("git {} failed in {} — {}", args.join(" "), scratch, r)));
        }
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the violation runs under the install recipe's cwd/env
// contract, and its first stdout line is the expected gate
fn fire(scratch: &str, kit: &str, vio: &str) -> Result<String, Outcome> {
    let recipe_env = [
        ("GATE_SDK_ROOT".to_string(), format!("{}/gate-sdk", scratch)),
        ("SMOKE_KIT_ROOT".to_string(), format!("{}/{}", scratch, kit)),
    ];
    let done =
        proc::run_stdout_in(&programs::BASH, &[vio], &recipe_env, Path::new(scratch)).map_err(env)?;
    Ok(String::from_utf8_lossy(done.stdout())
        .lines()
        .next()
        .unwrap_or("")
        .to_string())
}

// spec: gate-sdk/SPEC.md §Consumer smoke — a probe's two readings: the gate's own exit status, or
// the harness's inability to dispatch it at all, which is never a verdict about the gate
#[derive(Debug, PartialEq)]
enum Probe {
    Exit(i32),
    HarnessError(String),
}

// spec: gate-sdk/SPEC.md §Consumer smoke — resolve in the one checks dir, spawn in the probed tree:
// a gate reads its knob files relative to its working directory, so an in-process call would pair
// the invoking tree's knob state with the probed tree
fn probe(tree: &str, checks_dir: &str, gate: &str) -> Result<Probe, Outcome> {
    let Some(decl) = registry::resolve(gate, &[checks_dir.to_string()]) else {
        return Ok(Probe::Exit(2));
    };
    let (program, args): (programs::Program, Vec<&str>) = if decl.ends_with(".gate") {
        let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN").map_err(env)?;
        let path = format!("{}/{}", tree, bin);
        if !proc::is_executable(Path::new(&path)) {
            return Ok(Probe::HarnessError(format!(
                "{} dispatches to the native binary, but {} is absent or not executable in {} — the probe could not run. Build it: bash gate-sdk/bin/build-native.sh",
                gate, path, tree
            )));
        }
        (programs::CHECKWRIGHT_GATES.at(path), vec![gate])
    } else {
        (programs::Program::consumer(programs::GATE_DECLARATION, decl), Vec::new())
    };
    let locator = [("GATE_SDK_ROOT".to_string(), format!("{}/gate-sdk", tree))];
    match proc::run_with_env_in(&program, &args, &locator, Some(Path::new(tree))) {
        Ok(done) => Ok(Probe::Exit(done.reported_code())),
        Err(_) => Ok(Probe::Exit(126)),
    }
}

struct Accounting {
    registered: usize,
    probed: usize,
    self_declared: usize,
    hand_declared: usize,
    ms: u128,
    unaccounted: Vec<String>,
    stale: Vec<String>,
    contradicted: Vec<String>,
}

// spec: gate-sdk/SPEC.md §Consumer smoke — `# smoke-unregistered: <gate> — <reason>`: the first word
// is the name, and one each of `—`, `--`, `-` and a space is stripped off the reason in that order
fn declaration(line: &str) -> Option<(String, String)> {
    let rest = line.split_once("smoke-unregistered:")?.1;
    let rest = rest.trim();
    let (name, reason) = match rest.find(char::is_whitespace) {
        Some(i) => (&rest[..i], rest[i..].trim_start()),
        None => (rest, ""),
    };
    let mut reason = reason;
    for p in ["—", "--", "-", " "] {
        reason = reason.strip_prefix(p).unwrap_or(reason);
    }
    Some((name.to_string(), reason.to_string()))
}

fn is_declaration_line(line: &str) -> bool {
    line.trim_start()
        .strip_prefix('#')
        .map(|r| r.trim_start().starts_with("smoke-unregistered:"))
        .unwrap_or(false)
}

// spec: gate-sdk/SPEC.md §The install disposition — the first token of a gate's `# install:` line,
// read off its shell spelling before its descriptor
fn install_token(checks: &str, gate: &str) -> String {
    for ext in ["sh", "gate"] {
        let Ok(text) = std::fs::read_to_string(format!("{}/{}.{}", checks, gate, ext)) else {
            continue;
        };
        for l in text.lines() {
            if let Some(rest) = l.strip_prefix("# install:") {
                if rest.starts_with([' ', '\t']) {
                    return rest.split_whitespace().next().unwrap_or("").to_string();
                }
            }
        }
    }
    String::new()
}

// spec: gate-sdk/SPEC.md §Consumer smoke — each `unaccounted:` line gains its own `help:`
// continuation naming its remedy, since the reader is a validate session and the build session
// that owns the defect, one stage removed from the landing obligation
fn accounting_failure_lines(acct: &Accounting) -> Vec<String> {
    let mut lines = Vec::new();
    for l in &acct.unaccounted {
        lines.push(format!("  unaccounted: {}", l));
        lines.push("  help: register it in the smoke/install.sh of the kit that seeds this gate's subject surface — which need not be the shipping kit named above — or in the shipping kit's own smoke/install.sh with '# smoke-unregistered: <gate-name> — <reason>' for a vacuous pass (gate-sdk/SPEC.md §Consumer smoke).".to_string());
    }
    for l in &acct.stale {
        lines.push(format!("  stale declaration: {}", l));
    }
    if !acct.stale.is_empty() {
        lines.push("  help: a stale '# smoke-unregistered:' line names a gate that is now registered, mis-shipped, or already probe-exempt — correct or remove it (gate-sdk/SPEC.md §Consumer smoke).".to_string());
    }
    lines
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the registration accounting: one pass over the union of
// the vendored kits' declarations against the scratch registry, probe first and reasons second
fn account(scratch: &str, host: &str, roots: &[String]) -> Result<Accounting, Outcome> {
    let list = format!("{}/scripts/gates.list", scratch);
    let text = std::fs::read_to_string(&list)
        .map_err(|_| env(format!("no gate registry at {} after install", list)))?;
    let registered: BTreeSet<String> = registry::members(&text).into_iter().collect();

    let shipped = shipped_gates(scratch, roots);
    let kit_root: BTreeMap<String, String> = roots.iter().map(|r| (basename(r), r.clone())).collect();

    let mut reasons: BTreeMap<String, (String, String)> = BTreeMap::new();
    for r in roots {
        let kit = basename(r);
        let recipe = format!("{}/{}/smoke/install.sh", scratch, kit);
        let body = std::fs::read_to_string(&recipe).unwrap_or_default();
        for l in body.lines().filter(|l| is_declaration_line(l)) {
            let (g, reason) = declaration(l).unwrap_or_default();
            if g.is_empty() || reason.is_empty() {
                return Err(Outcome::Fail(vec![
                    format!("{}: FAIL — {}/smoke/install.sh has a '# smoke-unregistered:' line missing its gate name or its reason", VERDICT, kit),
                    "  help: the shape is '# smoke-unregistered: <gate-name> — <reason>'; both fields are read (gate-sdk/SPEC.md §Consumer smoke).".to_string(),
                ]));
            }
            reasons.insert(g, (kit.clone(), reason));
        }
    }

    let unregistered: Vec<(&String, &String)> =
        shipped.iter().filter(|(g, _)| !registered.contains(*g)).collect();

    let mut acct = Accounting {
        registered: registered.len(),
        probed: unregistered.len(),
        self_declared: 0,
        hand_declared: 0,
        ms: 0,
        unaccounted: Vec::new(),
        stale: Vec::new(),
        contradicted: Vec::new(),
    };
    let mut exempt: BTreeSet<String> = BTreeSet::new();
    let started = Instant::now();
    for (g, kit) in &unregistered {
        let scratch_checks = format!("{}/{}/checks", scratch, kit);
        let rc_s = step_probe(probe(scratch, &scratch_checks, g)?)?;
        let mut rc_h: Option<i32> = None;
        if rc_s == 2 {
            let host_checks = format!("{}/checks", kit_root[*kit]);
            let h = step_probe(probe(host, &host_checks, g)?)?;
            rc_h = Some(h);
            if h != 2 {
                if install_token(&scratch_checks, g) == "zero-config" {
                    acct.contradicted.push(format!(
                        "{} ships {} declaring zero-config, yet the probe finds its surface absent in the scratch consumer",
                        kit, g
                    ));
                }
                exempt.insert((*g).clone());
                acct.self_declared += 1;
                continue;
            }
        }
        if reasons.get(*g).map(|(k, _)| k == *kit).unwrap_or(false) {
            acct.hand_declared += 1;
            continue;
        }
        acct.unaccounted.push(match rc_h {
            Some(h) => format!("{} ships {} — scratch exit {}, invoking-repo exit {}", kit, g, rc_s, h),
            None => format!("{} ships {} — scratch exit {}", kit, g, rc_s),
        });
    }
    acct.ms = started.elapsed().as_millis();

    for (g, (dk, _)) in &reasons {
        if registered.contains(g) {
            acct.stale.push(format!("{} declares {} unregistered, but it is registered", dk, g));
        } else if shipped.get(g) != Some(dk) {
            acct.stale.push(format!("{} declares {}, which that kit does not ship", dk, g));
        } else if exempt.contains(g) {
            acct.stale.push(format!(
                "{} declares {}, which the probe already exempts — probe first, reasons second",
                dk, g
            ));
        }
    }
    Ok(acct)
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the shipped set, keyed by gate name over both declaration
// spellings, so a gate declared as `.sh` and `.gate` at once is one gate and one probe
fn shipped_gates(scratch: &str, roots: &[String]) -> BTreeMap<String, String> {
    let mut shipped: BTreeMap<String, String> = BTreeMap::new();
    for r in roots {
        let kit = basename(r);
        let checks = format!("{}/{}/checks", scratch, kit);
        for (name, _) in walk::list_dir(Path::new(&checks)).unwrap_or_default() {
            let stem = name.strip_suffix(".sh").or_else(|| name.strip_suffix(".gate"));
            if let Some(stem) = stem.filter(|s| s.starts_with("check-")) {
                shipped.insert(stem.to_string(), kit.clone());
            }
        }
    }
    shipped
}

// spec: gate-sdk/SPEC.md §Consumer smoke — a probe the harness could not dispatch ends the run at
// exit 2 naming the tree, rather than entering the corroboration table as a verdict
fn step_probe(p: Probe) -> Result<i32, Outcome> {
    match p {
        Probe::Exit(code) => Ok(code),
        Probe::HarnessError(message) => Err(env(message)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §Consumer smoke — `--keep` is the one flag, so any other dash-led word
    // is a refusal rather than a kit root
    #[test]
    fn an_unknown_option_is_a_usage_refusal() {
        assert_eq!(run(&["--nope".to_string()]), 2);
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — both declaration fields are read, and the separator
    // is trimmed off the reason in the order the grammar names
    #[test]
    fn a_declaration_reads_its_name_and_its_trimmed_reason() {
        assert!(is_declaration_line("  # smoke-unregistered: check-x — why"));
        assert!(!is_declaration_line("echo smoke-unregistered: check-x"));
        assert_eq!(
            declaration("# smoke-unregistered: check-x — why not"),
            Some(("check-x".to_string(), "why not".to_string()))
        );
        assert_eq!(
            declaration("# smoke-unregistered: check-x -- why"),
            Some(("check-x".to_string(), "why".to_string()))
        );
        assert_eq!(
            declaration("# smoke-unregistered: check-x"),
            Some(("check-x".to_string(), String::new()))
        );
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — the green token is the summary grammar, never a count
    #[test]
    fn the_green_token_matches_the_summary_grammar_alone() {
        assert!(green("noise\nAll 108 gates passed.\n"));
        assert!(!green("All gates passed"));
        assert!(!green("1 of 108 gates FAILED: check-x"));
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — a gate no checks dir declares is exit 2, and a `.gate`
    // declaration whose tree holds no binary is a harness error, never a probe verdict
    #[test]
    fn a_probe_separates_an_absent_declaration_from_an_absent_binary() {
        let base = std::env::temp_dir().join(format!("csmoke-probe.{}", std::process::id()));
        let checks = base.join("checks");
        std::fs::create_dir_all(&checks).unwrap();
        std::fs::write(checks.join("check-ported.gate"), "").unwrap();
        let tree = base.display().to_string();
        let dir = checks.display().to_string();
        assert!(matches!(probe(&tree, &dir, "check-absent"), Ok(Probe::Exit(2))));
        assert!(matches!(
            probe(&tree, &dir, "check-ported"),
            Ok(Probe::HarnessError(_))
        ));
        let _ = std::fs::remove_dir_all(&base);
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — the union over both declaration spellings counts a
    // gate once however many spellings declare it, and counts a non-`check-` file never
    #[test]
    fn the_shipped_set_counts_a_gate_once_across_both_spellings() {
        let base = std::env::temp_dir().join(format!("csmoke-shipped.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let checks = base.join("alpha-kit/checks");
        let other = base.join("beta-kit/checks");
        std::fs::create_dir_all(&checks).unwrap();
        std::fs::create_dir_all(&other).unwrap();
        for f in ["check-shell.sh", "check-native.gate", "check-both.sh", "check-both.gate", "helper.sh", "check-x.md"] {
            std::fs::write(checks.join(f), "").unwrap();
        }
        std::fs::write(other.join("check-beta.gate"), "").unwrap();
        let roots = vec!["/host/alpha-kit".to_string(), "/host/beta-kit".to_string()];
        let shipped = shipped_gates(&base.display().to_string(), &roots);
        let names: Vec<&str> = shipped.keys().map(String::as_str).collect();
        assert_eq!(names, ["check-beta", "check-both", "check-native", "check-shell"]);
        assert_eq!(shipped["check-both"], "alpha-kit");
        assert_eq!(shipped["check-beta"], "beta-kit");
        let _ = std::fs::remove_dir_all(&base);
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — the landing obligation's remedy: an unaccounted
    // line names its own help, a stale one names a different help, and a clean field adds neither
    #[test]
    fn an_unaccounted_gate_names_its_own_help_line() {
        let acct = Accounting {
            registered: 0,
            probed: 1,
            self_declared: 0,
            hand_declared: 0,
            ms: 0,
            unaccounted: vec!["some-kit ships check-x — scratch exit 1".to_string()],
            stale: Vec::new(),
            contradicted: Vec::new(),
        };
        let lines = accounting_failure_lines(&acct);
        assert_eq!(lines[0], "  unaccounted: some-kit ships check-x — scratch exit 1");
        assert!(lines[1].starts_with("  help: register it in the smoke/install.sh"));
        assert!(!lines.iter().any(|l| l.contains("stale '# smoke-unregistered:'")));
    }
}
