// spec: installer/SPEC.md §The consumer smoke — the installer's consumer smoke as an `Arm::Run`
// member: 0 clean, 1 a finding about the payload or the tree it wrote, 2 a harness precondition,
// which an emitting arm collapses
use crate::programs::Program;
use crate::proc;
use std::collections::BTreeMap;

mod artifact;
mod companion;
mod consumer;
mod lines;
mod masked;
mod moves;
mod profiles;
mod report;
mod roster;
mod staging;
mod upgrade;

// spec: gate-sdk/SPEC.md §The non-gate arm — the declared names, each gate-sdk's; the scratch base,
// the artifact hand-off and the tarball hand-out are read undeclared, no static kit owning their
// prefix
pub const KNOBS: &[&str] = &[
    "GATE_SDK_NATIVE_BIN",
    "GATE_SDK_NATIVE_CRATE",
    "GATE_SDK_NATIVE_TARGETS_FILE",
];

const NAME: &str = "installer-smoke";
const VERDICT: &str = "INSTALLER-SMOKE";
const USAGE: &str = "usage: --installer-smoke";

// spec: installer/SPEC.md §The consumer smoke — the three exit classes as a type, so a finding about
// the payload cannot be raised on a precondition's spelling or the reverse
pub(super) enum Outcome {
    Fail(String),
    Refuse(String),
}

type Step = Result<(), Outcome>;

fn fail(why: impl Into<String>) -> Outcome {
    Outcome::Fail(why.into())
}

fn refuse(why: impl Into<String>) -> Outcome {
    Outcome::Refuse(why.into())
}

fn say(line: &str) {
    println!("  {}", line);
}

// spec: installer/SPEC.md §The consumer smoke — a failing child's whole account goes to stderr
// before the verdict that cites it
fn show(out: &str) {
    eprintln!("{}", out);
}

// spec: installer/SPEC.md §The consumer smoke — the entry point an arm drives: the npm-installed
// package, through its `.bin` entry on a unix host, or an extracted package; each is driven through
// the host's bootstrap
#[derive(Clone)]
pub(super) enum Entry {
    Installed { bin: String, package: String },
    Extracted(String),
}

impl Default for Entry {
    fn default() -> Self {
        Entry::Extracted(String::new())
    }
}

impl Entry {
    // spec: installer/SPEC.md §The consumer smoke — the host bootstrap's script inside the package
    // the entry names
    pub(super) fn bootstrap(&self) -> String {
        let package = match self {
            Entry::Installed { package, .. } => package,
            Entry::Extracted(package) => package,
        };
        format!("{}/{}", package, consumer::BOOTSTRAP)
    }
}

// spec: installer/SPEC.md §The consumer smoke — what the arms hand each other, in run order; the
// arms run in sequence, so each reads what an earlier one wrote
#[derive(Default)]
pub(super) struct Run {
    root: String,
    scratch: String,
    host: String,
    bin_name: String,
    handed: Option<String>,
    steer: Option<String>,
    roster_file: String,
    pack_artifacts: String,
    version: String,
    tarball: String,
    plant: Option<String>,
    pkg_root: String,
    cw: String,
    entry: Entry,
    run_path: Option<String>,
    payload_kits: Vec<String>,
    profiles: Vec<String>,
    order: Vec<(String, String)>,
    profile_min: String,
    registry: BTreeMap<String, Vec<String>>,
    owes_bash: BTreeMap<String, bool>,
    value_red: Vec<String>,
    seeded: Vec<String>,
    dl_package: String,
    up_version: String,
    up2_version: String,
    up: String,
    up2: String,
}

impl Drop for Run {
    fn drop(&mut self) {
        if let Some(p) = self.plant.take() {
            let _ = std::fs::remove_file(p);
        }
        if !self.scratch.is_empty() {
            let _ = std::fs::remove_dir_all(&self.scratch);
        }
    }
}

pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        eprintln!("{}: unknown argument: {}; {}", NAME, a, USAGE);
        return 2;
    }
    let mut state = Run::default();
    match smoke(&mut state) {
        Ok(()) => {
            println!("{} ({})", roster::MARKER, profiles::summary(&state));
            0
        }
        Err(Outcome::Fail(why)) => {
            println!("{}: FAIL — {}", VERDICT, why);
            1
        }
        Err(Outcome::Refuse(why)) => {
            eprintln!("{}: {}", VERDICT, why);
            2
        }
    }
}

// spec: installer/SPEC.md §The consumer smoke — the roster is declared first, from the table the
// headers are printed from, then the preflight, then each arm under its header in table order
fn smoke(state: &mut Run) -> Step {
    roster::declare();
    roster::preflight(state)?;
    for row in roster::ARMS {
        println!("{}", roster::header(row, state));
        (row.arm)(state)?;
    }
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — a spawn into a scratch consumer, an extracted
// package or a throwaway copy of either runs under the invoking environment less every name a
// static kit's prefix owns, plus the values the arm sets for that call
fn in_consumer(
    program: &Program,
    args: &[&str],
    set: &[(String, String)],
    cwd: &str,
) -> Result<proc::Completed, Outcome> {
    in_consumer_fed(program, args, set, cwd, b"")
}

fn in_consumer_fed(
    program: &Program,
    args: &[&str],
    set: &[(String, String)],
    cwd: &str,
    input: &[u8],
) -> Result<proc::Completed, Outcome> {
    let unset = crate::knobs::inherited_under_static_prefixes();
    let child = proc::ChildEnv {
        set,
        unset: &unset,
        cwd: Some(std::path::Path::new(cwd)),
    };
    proc::run_with_stdin_in(program, args, input, &child).map_err(refuse)
}

// spec: installer/SPEC.md §The consumer smoke — the same scrub with both streams merged, the capture
// every verb's verdict is read from
fn merged_in(program: &Program, args: &[&str], set: &[(String, String)], cwd: &str) -> Result<proc::Merged, Outcome> {
    let unset = crate::knobs::inherited_under_static_prefixes();
    proc::run_merged_scrubbed(program, args, set, std::path::Path::new(cwd), &unset).map_err(refuse)
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The consumer smoke — the arm takes no operand, so any word is a
    // refusal rather than a mode the caller believes it selected
    #[test]
    fn the_arm_takes_no_operand() {
        assert_eq!(run(&["--keep".to_string()]), 2);
    }

    // spec: installer/SPEC.md §The consumer smoke — the scrub is the static tables' prefix set,
    // never a list: a name under every static kit's prefix is stripped, and a name under none,
    // the smoke's own undeclared ones included, reaches the child
    #[test]
    fn the_scrub_strips_every_static_prefix_and_nothing_else() {
        let guard = crate::knobenv::lock();
        let planted: Vec<String> = crate::knobs::STATIC_KITS
            .iter()
            .map(|k| format!("{}SCRUB_PROBE", k.prefix()))
            .collect();
        for p in &planted {
            guard.set(p, "redirected");
        }
        guard.set("INSTALLER_SMOKE_SCRUB_PROBE", "kept");
        let got = crate::knobs::inherited_under_static_prefixes();
        for p in &planted {
            guard.remove(p);
        }
        guard.remove("INSTALLER_SMOKE_SCRUB_PROBE");
        for p in &planted {
            assert!(got.contains(p), "{} reached a scratch consumer's child", p);
        }
        assert!(
            !got.iter().any(|n| n == "INSTALLER_SMOKE_SCRUB_PROBE"),
            "a name under no static kit's prefix was scrubbed"
        );
    }

    // spec: installer/SPEC.md §The consumer smoke — the scrub reaches the child itself, on both
    // capture faces: an exported kit knob is absent from a scratch consumer's environment and an
    // arm's own value is present
    #[cfg(unix)]
    #[test]
    fn a_scratch_consumers_child_meets_no_exported_kit_knob() {
        let guard = crate::knobenv::lock();
        guard.set("EVIDENCE_KIT_MANIFEST_FILE", ".tmp/no-such-manifest.md");
        let dir = std::env::temp_dir().display().to_string();
        let probe = ["-c", "printf '%s|%s' \"${EVIDENCE_KIT_MANIFEST_FILE-unset}\" \"$DEMO_TMP_DIR\""];
        let set = [("DEMO_TMP_DIR".to_string(), "set".to_string())];
        let split = in_consumer(&crate::programs::SH, &probe, &set, &dir);
        let joined = merged_in(&crate::programs::SH, &probe, &set, &dir);
        guard.remove("EVIDENCE_KIT_MANIFEST_FILE");
        let Ok(done) = split else { panic!("the scrubbed spawn did not run") };
        assert_eq!(text(done.stdout().unwrap_or_default()), "unset|set");
        let Ok(done) = joined else { panic!("the scrubbed merged spawn did not run") };
        assert_eq!(text(done.output()), "unset|set");
    }
}
