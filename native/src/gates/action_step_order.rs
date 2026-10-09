// spec: gate-sdk/SPEC.md §check-action-step-order — a job whose `run:` bodies spell a listed
// program has a step on the listed action ordered before the job's first such spelling
use crate::actions::{self, Ev};
use crate::walk;
use std::path::Path;

const ACTION_KNOB: &str = "GATE_LOCAL_STEP_ORDER_ACTION";
const PROGRAMS_KNOB: &str = "GATE_LOCAL_STEP_ORDER_PROGRAMS";

#[derive(Default)]
struct Audit {
    armed: usize,
    inert: usize,
    findings: Vec<String>,
    curfile: String,
    job: String,
    have_job: bool,
    step: Option<usize>,
    spelling: Option<(usize, String)>,
}

impl Audit {
    // spec: gate-sdk/SPEC.md §check-action-step-order — the order is by line: the job's first
    // step on the action against its first spelling
    fn finish_job(&mut self, action: &str) {
        if !self.have_job {
            return;
        }
        self.have_job = false;
        let step = self.step.take();
        let Some((line, program)) = self.spelling.take() else {
            self.inert += 1;
            return;
        };
        self.armed += 1;
        if step.is_some_and(|s| s < line) {
            return;
        }
        self.findings.push(format!(
            "{}:{}: job {} spells {} with no earlier step on {}",
            self.curfile, line, self.job, program, action
        ));
    }

    fn consume(&mut self, ev: Ev, action: &str) {
        match ev {
            Ev::Job(name, _) => {
                self.finish_job(action);
                self.job = name;
                self.have_job = true;
            }
            Ev::Uses(r, line) if self.have_job && r == action && self.step.map_or(true, |s| line < s) => {
                self.step = Some(line);
            }
            Ev::Word(w, line) if self.have_job && self.spelling.as_ref().map_or(true, |(s, _)| line < *s) => {
                self.spelling = Some((line, w));
            }
            _ => {}
        }
    }
}

fn refusal(e: &str) -> i32 {
    eprintln!("check-action-step-order: {} — the check could not run; treating as failure (not clean)", e);
    2
}

pub fn run(args: &[String]) -> i32 {
    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    let root = Path::new(scanroot);
    if !root.is_dir() {
        eprintln!("check-action-step-order: scan root not found: {}", scanroot);
        return 2;
    }

    let action = match walk::knob_scalar(ACTION_KNOB) {
        Ok(a) => a,
        Err(e) => return refusal(&e),
    };
    let programs = match walk::knob_array(PROGRAMS_KNOB) {
        Ok(p) => p,
        Err(e) => return refusal(&e),
    };
    // spec: gate-sdk/SPEC.md §check-action-step-order — either value empty is a clean skip naming
    // the empty knob, a different sentence from a nothing-found clean
    for (knob, empty) in [(ACTION_KNOB, action.is_empty()), (PROGRAMS_KNOB, programs.is_empty())] {
        if empty {
            println!("ACTION-STEP-ORDER: clean ({} is empty — no order is declared, so no job is checked)", knob);
            return 0;
        }
    }

    let files = match walk::find_files(root, &["yml", "yaml"]) {
        Ok(f) => f,
        Err(e) => return refusal(&e),
    };

    let mut a = Audit::default();
    let (mut walked, mut subject, mut composite, mut outside) = (0usize, 0usize, 0usize, 0usize);

    for f in &files {
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => return refusal(&format!("cannot read {} ({})", f.display(), e)),
        };
        walked += 1;
        // spec: gate-sdk/SPEC.md §check-action-step-order — the subject split is
        // check-action-gh-repo's: a composite action has no job and is skipped and counted
        if text.lines().any(|l| l.starts_with("jobs:")) {
            subject += 1;
        } else if text.lines().any(|l| l.starts_with("runs:")) {
            composite += 1;
            continue;
        } else {
            outside += 1;
            continue;
        }

        a.curfile = f.display().to_string();
        for ev in actions::walk_file(&text, actions::NO_VALVE, &programs) {
            a.consume(ev, &action);
        }
        a.finish_job(&action);
    }

    if !a.findings.is_empty() {
        println!("check-action-step-order: a job spells a listed program before any step on the");
        println!("listed action, so that call runs without what the action provides:");
        for x in &a.findings {
            println!("  {}", x);
        }
        println!("  help: add a step 'uses: {}' ahead of the job's first listed program", action);
        println!("        ({}), or reword a line that only mentions one.", programs.join(", "));
        return 1;
    }

    println!(
        "ACTION-STEP-ORDER: clean ({} job(s) spelling a listed program across {} Actions-shaped file(s) of {} walked, each with a step on {} first; {} job(s) spelling none, {} composite-action file(s) and {} non-Actions file(s) skipped)",
        a.armed, subject, walked, action, a.inert, composite, outside
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTION: &str = "./a";

    fn audit(stream: Vec<Ev>) -> Audit {
        let mut a = Audit { curfile: "f.yml".to_string(), ..Audit::default() };
        for ev in stream {
            a.consume(ev, ACTION);
        }
        a.finish_job(ACTION);
        a
    }

    // spec: gate-sdk/SPEC.md §check-action-step-order — a step on the action satisfies only its
    // own job, and only from a line ahead of the job's first spelling
    #[test]
    fn the_step_counts_only_ahead_of_the_first_spelling_in_its_own_job() {
        let early = audit(vec![Ev::Job("j".into(), 1), Ev::Uses(ACTION.into(), 3), Ev::Word("cargo".into(), 5)]);
        assert!(early.findings.is_empty());
        assert_eq!((early.armed, early.inert), (1, 0));

        let late = audit(vec![Ev::Job("j".into(), 1), Ev::Word("cargo".into(), 3), Ev::Uses(ACTION.into(), 5)]);
        assert_eq!(late.findings, vec!["f.yml:3: job j spells cargo with no earlier step on ./a"]);

        let other_job = audit(vec![
            Ev::Job("a".into(), 1),
            Ev::Uses(ACTION.into(), 3),
            Ev::Job("b".into(), 5),
            Ev::Word("rustc".into(), 7),
        ]);
        assert_eq!(other_job.findings, vec!["f.yml:7: job b spells rustc with no earlier step on ./a"]);
        assert_eq!((other_job.armed, other_job.inert), (1, 1));

        let other_action = audit(vec![Ev::Job("j".into(), 1), Ev::Uses("./b".into(), 3), Ev::Word("cargo".into(), 5)]);
        assert_eq!(other_action.findings.len(), 1);
    }

    // spec: gate-sdk/SPEC.md §check-action-step-order — one finding per job, on its first spelling
    #[test]
    fn a_job_reds_once_on_its_first_spelling() {
        let a = audit(vec![Ev::Job("j".into(), 1), Ev::Word("rustc".into(), 9), Ev::Word("cargo".into(), 4)]);
        assert_eq!(a.findings, vec!["f.yml:4: job j spells cargo with no earlier step on ./a"]);
    }
}
