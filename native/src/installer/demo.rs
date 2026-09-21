// spec: installer/SPEC.md §demo — the adoption arc run out of the package's own payload into a
// scratch repository of its own: install, a clean battery, a caught defect, the fix. Exit status is
// the verdict: 0 every act held, 1 an act did not, 2 the harness could not be stood up.
use super::{GATES_DIR, Package};
use crate::walkthrough::{self as show, banner, ends_with_newline, say, Scratch};
use crate::{proc, programs};
use std::path::{Path, PathBuf};

const VERDICT: &str = "DEMO";
const USAGE: &[&str] = &[
    "usage: checkwright demo",
    "",
    "Shows the adoption arc without touching your repository: installs the full",
    "profile into a scratch repository of its own, runs the battery green, plants",
    "one mistyped link and shows it caught, fixes it and runs green again, then",
    "removes the scratch. Exit status is the verdict: 0 every step held.",
];

// spec: installer/SPEC.md §demo — the consumer smoke's value-arm defect, one mistyped relative
// link in a README beside the page it meant; the fix is the link and never the corpus
const DEFECT_PAGE: &str = "docs/README.md";
const DEFECT: &str = "# Handbook\n\nStart with [the style guide](style-guid.md).\n";
const FIXED: &str = "# Handbook\n\nStart with [the style guide](style-guide.md).\n";
const TARGET_PAGE: &str = "docs/style-guide.md";
const TARGET: &str = "# Style guide\n\nWrite plainly, and link what you cite.\n";
const DEFECT_NAME: &str = "README.md";

// spec: installer/SPEC.md §demo — the three exit classes as a type, so a finding about the arc
// cannot be raised on a precondition's spelling or the reverse
enum Outcome {
    Clean,
    Fail(String),
    Refuse(String),
}

macro_rules! step {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(o) => return o,
        }
    };
}

// spec: installer/SPEC.md §demo — `--help` answers on its own and any other word is a usage
// refusal: the verb fixes its own profile and scratch, so an operand has nothing to select
pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        if a == "-h" || a == "--help" {
            for line in USAGE {
                println!("{}", line);
            }
            return 0;
        }
        eprintln!("checkwright demo: unknown argument: {}", a);
        eprintln!("  {}", USAGE[0]);
        return 2;
    }
    let pkg = match super::package("demo installs") {
        Ok(p) => p,
        Err(mut r) => {
            r.help.push_str(
                " From a source checkout, the contributor's walkthrough is `run-gates.sh --run-demo`.",
            );
            return super::finish("demo", Err(r));
        }
    };
    let mut scratch = Scratch { dir: None };
    match walkthrough(&pkg, &mut scratch) {
        Outcome::Clean => {
            banner("DEMO: clean — the adoption arc behaved, and your repository was not touched");
            say("install → clean pass → defect caught → fix → green");
            say("To adopt for real, run `checkwright init` inside your repository.");
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

fn walkthrough(pkg: &Package, scratch: &mut Scratch) -> Outcome {
    let repo = step!(make_scratch(scratch));
    step!(git(&repo, &["init", "-q"]));
    // spec: installer/SPEC.md §demo — the scratch carries its own identity, so an unconfigured
    // host's missing user.name stops neither the seed commit nor the one `init` makes; and no
    // detached maintenance repacks the objects the teardown is removing
    for (k, v) in [
        ("user.name", "checkwright demo"),
        ("user.email", "demo@example.invalid"),
        ("maintenance.auto", "false"),
        ("gc.auto", "0"),
    ] {
        step!(git(&repo, &["config", k, v]));
    }
    step!(git(&repo, &["commit", "-q", "--allow-empty", "-m", "seed"]));

    banner("ACT 1 — Install into a fresh repository");
    say("`checkwright init --profile full`, exactly as you would run it in yours,");
    say("but in a scratch repository this demo made and will remove.");
    step!(install(pkg, &repo));

    banner("ACT 2 — A clean tree passes the battery");
    let (rc, out) = step!(battery(pkg, &repo));
    let green = step!(green_line(&out));
    if rc != 0 || green.is_empty() {
        print!("{}", ends_with_newline(&out));
        return Outcome::Fail("the battery was not green on the freshly installed repository".into());
    }
    say(&green);

    banner("ACT 3 — A defect is caught before it lands");
    say(&format!("A page links to its neighbour with a typo: {} → style-guid.md.", DEFECT_PAGE));
    step!(write(&repo, TARGET_PAGE, TARGET));
    step!(write(&repo, DEFECT_PAGE, DEFECT));
    step!(commit(&repo, "handbook"));
    let (rc, out) = step!(battery(pkg, &repo));
    if rc == 0 {
        return Outcome::Fail("the mistyped link did not turn the battery red".into());
    }
    let quoted: Vec<Vec<String>> = show::failed_gates(&out)
        .iter()
        .map(|g| show::excerpt(&out, g))
        .collect();
    if !quoted.iter().flatten().any(|l| l.contains(DEFECT_NAME)) {
        print!("{}", ends_with_newline(&out));
        return Outcome::Fail(format!("the battery went red without naming {}", DEFECT_NAME));
    }
    for block in &quoted {
        println!();
        for line in block {
            println!("{}", line);
        }
    }
    say("→ caught. Registered as a hook, this would have refused the commit.");

    banner("ACT 4 — Fix, re-run, green");
    step!(write(&repo, DEFECT_PAGE, FIXED));
    step!(commit(&repo, "fix the link"));
    let (rc, out) = step!(battery(pkg, &repo));
    let green = step!(green_line(&out));
    if rc != 0 || green.is_empty() {
        print!("{}", ends_with_newline(&out));
        return Outcome::Fail("the battery did not return to green after the fix".into());
    }
    say(&green);

    Outcome::Clean
}

fn refuse(e: String) -> Outcome {
    Outcome::Refuse(e)
}

fn green_line(out: &str) -> Result<String, Outcome> {
    show::green_line(out).map_err(refuse)
}

// spec: installer/SPEC.md §demo — `DEMO_TMP_DIR` when set, else the platform's temp directory; the
// directory is made in-process, since `mktemp` is a contributor program this verb may not spawn
fn make_scratch(scratch: &mut Scratch) -> Result<PathBuf, Outcome> {
    let base = std::env::var_os("DEMO_TMP_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    for n in 0..64u32 {
        let dir = base.join(format!("checkwright-demo.{}.{}.{}", std::process::id(), stamp, n));
        match std::fs::create_dir(&dir) {
            Ok(()) => {
                scratch.dir = Some(dir.clone());
                return Ok(dir);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => {
                return Err(Outcome::Refuse(format!(
                    "cannot create the scratch repository under {}: {}",
                    base.display(),
                    e
                )))
            }
        }
    }
    Err(Outcome::Refuse(format!(
        "cannot create the scratch repository under {}: every candidate name was taken",
        base.display()
    )))
}

fn write(repo: &Path, rel: &str, body: &str) -> Result<(), Outcome> {
    let path = repo.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| Outcome::Refuse(format!("cannot write {}: {}", rel, e)))?;
    }
    std::fs::write(&path, body).map_err(|e| Outcome::Refuse(format!("cannot write {}: {}", rel, e)))
}

fn git(repo: &Path, args: &[&str]) -> Result<(), Outcome> {
    let root = repo.to_string_lossy().into_owned();
    let mut argv: Vec<&str> = vec!["-C", &root];
    argv.extend_from_slice(args);
    let done = proc::run(&programs::GIT, &argv).map_err(refuse)?;
    match done.failure_report() {
        None => Ok(()),
        Some(r) => Err(Outcome::Refuse(format!(
            "git {} failed in the scratch repository — {}",
            args.join(" "),
            r
        ))),
    }
}

// spec: installer/SPEC.md §demo — the defect is committed past any hook, because the battery run
// below is what the act shows catching it
fn commit(repo: &Path, message: &str) -> Result<(), Outcome> {
    git(repo, &["add", "-A"])?;
    git(repo, &["commit", "-q", "--no-verify", "-m", message])
}

// spec: installer/SPEC.md §demo — act 1 is the adopter's own `init`, spawned from this very
// executable with its narration on the terminal; a non-zero exit is a harness that could not be
// stood up, never a statement about the arc
fn install(pkg: &Package, repo: &Path) -> Result<(), Outcome> {
    let me = programs::CHECKWRIGHT_GATES.at(pkg.artifact.to_string_lossy().into_owned());
    let code = proc::run_to_in(&me, &["--init", "--profile", super::profile::DERIVED], &[], Some(repo), &proc::Sink::Inherit)
        .map_err(refuse)?;
    if code != 0 {
        return Err(Outcome::Refuse(format!("init exited {} in the scratch repository", code)));
    }
    Ok(())
}

// spec: installer/SPEC.md §demo — the battery is the door `init` placed in the scratch tree,
// spawned there: the subject is the installed consumer's own door, not this process's registry
fn battery(pkg: &Package, repo: &Path) -> Result<(i32, String), Outcome> {
    let name = pkg.artifact.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let door = repo.join(GATES_DIR).join(name);
    let m = proc::run_merged_in(
        &programs::CHECKWRIGHT_GATES.at(door.to_string_lossy().into_owned()),
        &["--run"],
        &[],
        Some(repo),
    )
    .map_err(refuse)?;
    Ok((m.reported_code(), String::from_utf8_lossy(m.output()).into_owned()))
}

#[cfg(test)]
mod tests {
    // spec: installer/SPEC.md §demo — `--help` answers before the package precondition, and any
    // other word is a usage refusal rather than a mode the caller believes it selected
    #[test]
    fn help_answers_and_an_operand_is_refused() {
        assert_eq!(super::run(&["--help".to_string()]), 0);
        assert_eq!(super::run(&["extra".to_string()]), 2);
    }

    // spec: installer/SPEC.md §demo — run from a source checkout there is no payload, so the verb
    // refuses at exit 2 rather than reaching for the contributor's kit roots
    #[test]
    fn a_source_checkout_gets_the_no_payload_refusal() {
        assert_eq!(super::run(&[]), 2);
    }
}
