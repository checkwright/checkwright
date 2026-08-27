// spec: gate-sdk/SPEC.md §check-action-gh-repo — a job whose `run:` bodies invoke `gh`
// establishes a repository context: a checkout ordered before the job's first invocation,
// `GH_REPO` in scope, or `--repo` on every detected invocation
use crate::actions::{self, Ev};
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-action-gh-repo — the valve's spelling is this gate's, the
// binding rule the shared walk's, so the token is handed to the walk rather than baked into it
const MARKER: &str = "gh-repo-exempt";

#[derive(Default)]
struct Audit {
    armed: usize,
    inert: usize,
    exempt: usize,
    calls: usize,
    findings: Vec<String>,
    bare: Vec<String>,
    curfile: String,
    wenv: bool,
    job: String,
    jobline: usize,
    jexempt: bool,
    jenv: bool,
    checkouts: Vec<usize>,
    iln: Vec<usize>,
    irepo: Vec<bool>,
    ienv: Vec<bool>,
    senv: bool,
    sexempt: bool,
    sln: Vec<usize>,
    srepo: Vec<bool>,
    have_job: bool,
    have_step: bool,
}

impl Audit {
    fn finish_step(&mut self) {
        if !self.have_step {
            return;
        }
        self.have_step = false;
        if !self.sexempt && !self.sln.is_empty() {
            let scoped = self.wenv || self.jenv || self.senv;
            for (line, repo) in self.sln.iter().zip(self.srepo.iter()) {
                self.iln.push(*line);
                self.irepo.push(*repo);
                self.ienv.push(scoped);
            }
        }
        self.senv = false;
        self.sexempt = false;
        self.sln.clear();
        self.srepo.clear();
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — the three arms are disjoined per job
    // and each is universally quantified over the job's detected set.
    fn finish_job(&mut self) {
        if !self.have_job {
            return;
        }
        self.finish_step();
        self.have_job = false;
        if self.jexempt {
            self.exempt += 1;
            return;
        }
        if self.iln.is_empty() {
            self.inert += 1;
            return;
        }
        self.armed += 1;
        self.calls += self.iln.len();
        let first = *self.iln.iter().min().unwrap_or(&0);
        if self.checkouts.iter().any(|c| *c < first) {
            return;
        }
        let allenv = self.ienv.iter().all(|e| *e);
        let allrepo = self.irepo.iter().all(|r| *r);
        if allenv || allrepo {
            return;
        }
        self.findings.push(format!(
            "{}:{}: job '{}' first invokes gh at line {} with no repository context",
            self.curfile, self.jobline, self.job, first
        ));
    }

    fn consume(&mut self, ev: Ev) {
        match ev {
            Ev::Job(name, line) => {
                self.finish_job();
                self.job = name;
                self.jobline = line;
                self.jexempt = false;
                self.jenv = false;
                self.checkouts.clear();
                self.iln.clear();
                self.irepo.clear();
                self.ienv.clear();
                self.have_job = true;
            }
            Ev::JobEnv => self.jenv = true,
            Ev::JobExempt => self.jexempt = true,
            Ev::Step => {
                self.finish_step();
                self.have_step = true;
            }
            Ev::StepEnv => self.senv = true,
            Ev::StepExempt => self.sexempt = true,
            Ev::Checkout(line) => self.checkouts.push(line),
            Ev::Gh(line, repo) => {
                self.sln.push(line);
                self.srepo.push(repo);
                self.have_step = true;
            }
            // spec: gate-sdk/SPEC.md §check-action-permissions — an existing consumer ignores a
            // stream member it has no arm for, which is what makes widening the stream additive.
            Ev::WorkflowEnv | Ev::WorkflowPerms(_, _) | Ev::JobPerms(_, _) | Ev::Token => {}
            Ev::BareMarker(line) => self.bare.push(format!(
                "{}:{}: a {} marker with no reason",
                self.curfile, line, MARKER
            )),
        }
    }
}

const FINDING_HELP: &[&str] = &[
    "  help: add an actions/checkout step before the first gh call, or set",
    "        GH_REPO: ${{ github.repository }} on the workflow, the job, or the",
    "        invoking step's env:, or pass --repo on every gh call in the job.",
    "        A job standing outside all three takes '# gh-repo-exempt: <reason>'.",
];

pub fn run(args: &[String]) -> i32 {
    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    let root = Path::new(scanroot);
    if !root.is_dir() {
        eprintln!("check-action-gh-repo: scan root not found: {}", scanroot);
        return 2;
    }

    let files = match walk::find_files(root, &["yml", "yaml"]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("check-action-gh-repo: {} — the check could not run; treating as failure (not clean)", e);
            return 2;
        }
    };

    if files.is_empty() {
        println!(
            "ACTION-GH-REPO: clean (no YAML under {} — 0 job(s) to check)",
            scanroot
        );
        return 0;
    }

    let mut a = Audit::default();
    let (mut walked, mut subject, mut composite, mut outside) = (0usize, 0usize, 0usize, 0usize);

    for f in &files {
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "check-action-gh-repo: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                    f.display(),
                    e
                );
                return 2;
            }
        };
        walked += 1;
        // spec: gate-sdk/SPEC.md §check-action-gh-repo — the Actions-shape predicate is
        // split in two: the unit is a job under `jobs:`, and a `runs:`-shaped composite
        // action inherits its caller's repository context.
        if text.lines().any(|l| l.starts_with("jobs:")) {
            subject += 1;
        } else if text.lines().any(|l| l.starts_with("runs:")) {
            composite += 1;
            continue;
        } else {
            outside += 1;
            continue;
        }

        let stream = actions::walk_file(&text, MARKER);
        a.curfile = f.display().to_string();
        a.wenv = stream.iter().any(|e| matches!(e, Ev::WorkflowEnv));
        for ev in stream {
            a.consume(ev);
        }
        a.finish_job();
    }

    let mut red = false;

    if !a.findings.is_empty() {
        red = true;
        println!("check-action-gh-repo: a job invokes gh with no way to resolve a target");
        println!("repository, so every call in it dies before its first request — on a tag,");
        println!("where nothing else in the battery runs:");
        for x in &a.findings {
            println!("  {}", x);
        }
        for l in FINDING_HELP {
            println!("{}", l);
        }
    }

    if !a.bare.is_empty() {
        red = true;
        println!("check-action-gh-repo: a gh-repo-exempt marker carries no reason, so it records");
        println!("that an arm was stood outside of without saying which one or why:");
        for x in &a.bare {
            println!("  {}", x);
        }
        println!("  help: write the marker as '# gh-repo-exempt: <reason>' naming the arm the");
        println!("        job stands outside of, or delete it and satisfy an arm.");
    }

    if red {
        return 1;
    }

    println!(
        "ACTION-GH-REPO: clean ({} job(s) invoking gh across {} Actions-shaped file(s) of {} walked, all resolving a repository; {} invocation(s) detected, {} job(s) invoking none, {} exempt, {} composite-action file(s) and {} non-Actions file(s) skipped)",
        a.armed, subject, walked, a.calls, a.inert, a.exempt, composite, outside
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn audit(stream: Vec<Ev>) -> Audit {
        let mut a = Audit {
            curfile: "f.yml".to_string(),
            ..Audit::default()
        };
        a.wenv = stream.iter().any(|e| matches!(e, Ev::WorkflowEnv));
        for ev in stream {
            a.consume(ev);
        }
        a.finish_job();
        a
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — the arms are disjoined per job and each
    // is quantified over the whole detected set, so a job pairing a `GH_REPO` step with a
    // `--repo` step satisfies neither.
    #[test]
    fn mixing_the_arms_within_one_job_satisfies_neither() {
        let mixed = audit(vec![
            Ev::Job("j".into(), 1),
            Ev::Step,
            Ev::StepEnv,
            Ev::Gh(3, false),
            Ev::Step,
            Ev::Gh(5, true),
        ]);
        assert_eq!(mixed.findings.len(), 1);
        assert_eq!(mixed.armed, 1);
        assert_eq!(mixed.calls, 2);

        let all_repo = audit(vec![
            Ev::Job("j".into(), 1),
            Ev::Step,
            Ev::Gh(3, true),
            Ev::Step,
            Ev::Gh(5, true),
        ]);
        assert!(all_repo.findings.is_empty());
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — a checkout satisfies the arm only where
    // its line precedes the job's first detected invocation.
    #[test]
    fn a_checkout_ordered_after_the_first_call_establishes_nothing() {
        let late = audit(vec![
            Ev::Job("j".into(), 1),
            Ev::Step,
            Ev::Gh(3, false),
            Ev::Step,
            Ev::Checkout(5),
        ]);
        assert_eq!(late.findings.len(), 1);

        let early = audit(vec![
            Ev::Job("j".into(), 1),
            Ev::Step,
            Ev::Checkout(2),
            Ev::Step,
            Ev::Gh(4, false),
        ]);
        assert!(early.findings.is_empty());
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — a job-bound marker skips the job, a
    // step-bound one drops that step's invocations and leaves the job's others held.
    #[test]
    fn the_valve_binds_a_job_whole_and_a_step_narrowly() {
        let job = audit(vec![
            Ev::Job("j".into(), 1),
            Ev::JobExempt,
            Ev::Step,
            Ev::Gh(3, false),
        ]);
        assert!(job.findings.is_empty());
        assert_eq!(job.exempt, 1);
        assert_eq!(job.armed, 0);

        let step = audit(vec![
            Ev::Job("j".into(), 1),
            Ev::Step,
            Ev::StepExempt,
            Ev::Gh(3, false),
            Ev::Step,
            Ev::Gh(5, false),
        ]);
        assert_eq!(step.findings.len(), 1);
        assert_eq!(step.calls, 1);

        let bare = audit(vec![Ev::Job("j".into(), 1), Ev::BareMarker(2)]);
        assert_eq!(bare.bare, vec!["f.yml:2: a gh-repo-exempt marker with no reason"]);
    }
}
