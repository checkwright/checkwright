// spec: installer/SPEC.md §demo — the adoption arc run out of the package's own payload into a
// scratch repository of its own: install, a clean battery, a caught done claim, the claim
// withdrawn. Exit status is the verdict: 0 every act held, 1 an act did not, 2 no harness.
use super::{GATES_DIR, Package, QUEUE_FILE, STATE_FILE};
use crate::walkthrough::{self as show, banner, ends_with_newline, say, Scratch};
use crate::{proc, programs};
use std::path::{Path, PathBuf};

const VERDICT: &str = "DEMO";
const USAGE: &[&str] = &[
    "usage: checkwright demo",
    "",
    "Shows the adoption arc without touching your repository: installs the full",
    "profile into a scratch repository of its own, runs the battery green, commits",
    "a task marked done with no evidence behind it and shows the claim caught,",
    "withdraws it and runs green again, then removes the scratch. Exit status is",
    "the verdict: 0 every step held.",
];

// spec: installer/SPEC.md §demo — what an agent claiming done writes: an iteration named in the
// queue header, a task under Done the seeded queue does not carry, then a validate and a close
// stamp, each stage on a session token of its own
const ITERATION: &str = "first-release";
const TASK: &str = "add-login-page";
const STAMPS: [(&str, &str); 2] = [("validate", "demo-validate"), ("close", "demo-close")];
const ITERATION_HEADER: &str = "## Iteration:";
const DONE_HEADING: &str = "## Done";

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
            say("install → clean pass → done claim caught → withdrawn → green");
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

    banner("ACT 3 — A done claim with no evidence is caught");
    say(&format!("An agent reports task {} done: it moves the task to Done and stamps", TASK));
    say(&format!("validate and close for iteration {}, but no test run was ever recorded.", ITERATION));
    let queue = step!(read(&repo, QUEUE_FILE));
    let state = step!(read(&repo, STATE_FILE));
    let head = step!(git(&repo, &["rev-parse", "--short", "HEAD"]));
    let today = crate::emit::kpi::iso_day(crate::emit::kpi::now_epoch().div_euclid(86_400));
    let (claimed_queue, claimed_state) = step!(claim(&queue, &state, head.trim(), &today).map_err(refuse));
    step!(write(&repo, QUEUE_FILE, &claimed_queue));
    step!(write(&repo, STATE_FILE, &claimed_state));
    step!(commit(&repo, &format!("done: {}", TASK)));
    let (rc, out) = step!(battery(pkg, &repo));
    if rc == 0 {
        return Outcome::Fail("the done claim did not turn the battery red".into());
    }
    let quoted: Vec<Vec<String>> = show::failed_gates(&out)
        .iter()
        .map(|g| show::excerpt(&out, g))
        .collect();
    if !quoted.iter().flatten().any(|l| l.contains(ITERATION)) {
        print!("{}", ends_with_newline(&out));
        return Outcome::Fail(format!(
            "the battery went red without naming the claimed iteration {}",
            ITERATION
        ));
    }
    for block in &quoted {
        println!();
        for line in block {
            println!("{}", line);
        }
    }
    say("→ caught. Registered as a hook, this would have refused the commit.");

    banner("ACT 4 — Withdraw the claim, re-run, green");
    say("The queue and state file go back to what they said before the claim.");
    step!(write(&repo, QUEUE_FILE, &queue));
    step!(write(&repo, STATE_FILE, &state));
    step!(commit(&repo, &format!("withdraw the done claim on {}", TASK)));
    let (rc, out) = step!(battery(pkg, &repo));
    let green = step!(green_line(&out));
    if rc != 0 || green.is_empty() {
        print!("{}", ends_with_newline(&out));
        return Outcome::Fail("the battery did not return to green after the claim was withdrawn".into());
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

// spec: installer/SPEC.md §demo — `DEMO_TMP_DIR` when set, else the platform's temp directory; a
// relative base resolves against the invoking directory, since every act spawns inside the scratch
fn scratch_base(set: Option<std::ffi::OsString>, here: &str) -> PathBuf {
    let base = set.filter(|v| !v.is_empty()).map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    PathBuf::from(crate::walk::abs_against(here, &base.to_string_lossy()))
}

// spec: installer/SPEC.md §demo — the directory is made in-process, since `mktemp` is a
// contributor program this verb may not spawn
fn make_scratch(scratch: &mut Scratch) -> Result<PathBuf, Outcome> {
    let here = crate::walk::cwd().map_err(Outcome::Refuse)?;
    let base = scratch_base(std::env::var_os("DEMO_TMP_DIR"), &here);
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

// spec: installer/SPEC.md §demo — the claim staged over the seeded queue and state file; a seed
// with no iteration header or no Done section cannot stage the act, the harness's failure and not
// the arc's
fn claim(queue: &str, state: &str, head: &str, date: &str) -> Result<(String, String), String> {
    let mut lines: Vec<String> = queue.lines().map(str::to_string).collect();
    let header = lines.iter().position(|l| l.starts_with(ITERATION_HEADER)).ok_or_else(|| {
        format!("the seeded {} carries no '{}' header to name the iteration in", QUEUE_FILE, ITERATION_HEADER)
    })?;
    lines[header] = format!("{} {}", ITERATION_HEADER, ITERATION);
    let done = lines.iter().position(|l| l.trim_end() == DONE_HEADING).ok_or_else(|| {
        format!("the seeded {} carries no '{}' section to record the task under", QUEUE_FILE, DONE_HEADING)
    })?;
    let at = if lines.get(done + 1).is_some_and(|l| l.is_empty()) { done + 2 } else { done + 1 };
    let mut insert = vec![format!("- {}", TASK)];
    if at == done + 1 {
        insert.insert(0, String::new());
    }
    if lines.get(at).is_some_and(|l| !l.starts_with("- ")) {
        insert.push(String::new());
    }
    lines.splice(at..at, insert);
    let mut claimed_queue = lines.join("\n");
    claimed_queue.push('\n');
    let mut claimed_state = if state.is_empty() { String::new() } else { ends_with_newline(state) };
    for (stage, session) in STAMPS {
        claimed_state.push_str(&format!("{} {} {} {} {}\n", ITERATION, stage, session, date, head));
    }
    Ok((claimed_queue, claimed_state))
}

fn read(repo: &Path, rel: &str) -> Result<String, Outcome> {
    std::fs::read_to_string(repo.join(rel))
        .map_err(|e| Outcome::Refuse(format!("cannot read the seeded {}: {}", rel, e)))
}

fn write(repo: &Path, rel: &str, body: &str) -> Result<(), Outcome> {
    let path = repo.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| Outcome::Refuse(format!("cannot write {}: {}", rel, e)))?;
    }
    std::fs::write(&path, body).map_err(|e| Outcome::Refuse(format!("cannot write {}: {}", rel, e)))
}

fn git(repo: &Path, args: &[&str]) -> Result<String, Outcome> {
    let root = repo.to_string_lossy().into_owned();
    let mut argv: Vec<&str> = vec!["-C", &root];
    argv.extend_from_slice(args);
    let done = proc::run(&programs::GIT, &argv).map_err(refuse)?;
    match done.stdout() {
        Some(out) => Ok(String::from_utf8_lossy(out).into_owned()),
        None => Err(Outcome::Refuse(format!(
            "git {} failed in the scratch repository — {}",
            args.join(" "),
            done.failure_report().unwrap_or_default()
        ))),
    }
}

// spec: installer/SPEC.md §demo — the claim is committed past any hook, because the battery run
// below is what the act shows catching it
fn commit(repo: &Path, message: &str) -> Result<(), Outcome> {
    git(repo, &["add", "-A"])?;
    git(repo, &["commit", "-q", "--no-verify", "-m", message]).map(|_| ())
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

    // spec: installer/SPEC.md §demo — act 3 names the iteration, heads Done with the task, appends
    // a validate then a close stamp on distinct sessions, and touches nothing else
    #[test]
    fn the_claim_names_the_iteration_records_the_task_and_stamps_validate_then_close() {
        let queue = "# q\n\n## Iteration: —\n\n---\n\n## Done\n\n- old-slug\n\n## Lessons Learned\n";
        let state = "# contract\n\n---\n\n";
        let (q, s) = super::claim(queue, state, "abc1234", "2026-01-02").unwrap();
        assert_eq!(
            q,
            "# q\n\n## Iteration: first-release\n\n---\n\n## Done\n\n- add-login-page\n- old-slug\n\n## Lessons Learned\n"
        );
        assert_eq!(
            s,
            "# contract\n\n---\n\nfirst-release validate demo-validate 2026-01-02 abc1234\n\
             first-release close demo-close 2026-01-02 abc1234\n"
        );
        let (q, _) = super::claim("## Iteration: —\n## Done\n## Lessons Learned\n", "", "abc1234", "2026-01-02").unwrap();
        assert_eq!(q, "## Iteration: first-release\n## Done\n\n- add-login-page\n\n## Lessons Learned\n");
    }

    // spec: installer/SPEC.md §demo — a relative DEMO_TMP_DIR resolves against the invoking
    // directory, so the door each act spawns inside the scratch is never a relative path
    #[test]
    fn a_relative_scratch_base_resolves_against_the_invoking_directory() {
        let rooted = |p: std::path::PathBuf| crate::walk::path_root(&p.to_string_lossy()).is_some();
        let got = super::scratch_base(Some("rel/base".into()), "/invoked/here");
        assert_eq!(got, std::path::PathBuf::from("/invoked/here/rel/base"));
        assert!(rooted(super::scratch_base(Some("/abs/base".into()), "/invoked/here")));
        assert!(rooted(super::scratch_base(Some("".into()), "/invoked/here")));
        assert!(rooted(super::scratch_base(None, "/invoked/here")));
    }

    // spec: installer/SPEC.md §demo — a seeded queue act 3 cannot stage is an env refusal
    #[test]
    fn a_queue_with_no_header_or_no_done_section_cannot_stage_the_claim() {
        assert!(super::claim("## Done\n", "", "abc1234", "2026-01-02").is_err());
        assert!(super::claim("## Iteration: —\n", "", "abc1234", "2026-01-02").is_err());
    }
}
