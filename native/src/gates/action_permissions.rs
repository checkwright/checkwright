// spec: gate-sdk/SPEC.md §check-action-permissions — a job that consumes the GitHub token has
// the scopes it takes declared rather than inherited from an invisible repository default
use crate::actions::{self, Ev, Perms};
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-action-permissions — the valve's spelling is this gate's, the
// binding rule the shared walk's, so the token is handed to the walk rather than baked into it
const MARKER: &str = "action-permissions-exempt";

// spec: gate-sdk/SPEC.md §check-action-permissions — `write` satisfies `read`, and a
// grant-everything shorthand satisfies the checkout arm without naming the scope.
fn grants_contents(p: Option<&Perms>) -> bool {
    match p {
        Some(Perms::All) => true,
        Some(Perms::Scopes(v)) => v
            .iter()
            .any(|(k, val)| k == "contents" && (val == "read" || val == "write")),
        _ => false,
    }
}

// spec: gate-sdk/SPEC.md §check-action-permissions — the declaration arm asserts that the job
// says what it takes, which an empty allowlist does not.
fn declares_scopes(p: Option<&Perms>) -> bool {
    match p {
        Some(Perms::All) => true,
        Some(Perms::Scopes(v)) => !v.is_empty(),
        _ => false,
    }
}

#[derive(Default)]
struct Audit {
    armed: usize,
    inert: usize,
    exempt: usize,
    checkout_findings: Vec<String>,
    declare_findings: Vec<String>,
    bare: Vec<String>,
    curfile: String,
    wperms: Option<Perms>,
    job: String,
    jobline: usize,
    jexempt: bool,
    jperms: Option<Perms>,
    checkout: bool,
    gh: bool,
    token: bool,
    sexempt: bool,
    scheckout: bool,
    sgh: bool,
    stoken: bool,
    have_job: bool,
    have_step: bool,
}

impl Audit {
    // spec: gate-sdk/SPEC.md §check-action-permissions — a step-bound valve drops that step's
    // evidence from the trigger set and leaves the job's other evidence held.
    fn finish_step(&mut self) {
        if !self.have_step {
            return;
        }
        self.have_step = false;
        if !self.sexempt {
            self.checkout |= self.scheckout;
            self.gh |= self.sgh;
            self.token |= self.stoken;
        }
        self.sexempt = false;
        self.scheckout = false;
        self.sgh = false;
        self.stoken = false;
    }

    // spec: gate-sdk/SPEC.md §check-action-permissions — the scopes in scope for a job are its
    // own block where it has one and the file's otherwise, and a job armed by a checkout takes
    // the stronger arm rather than both.
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
        if !(self.checkout || self.gh || self.token) {
            self.inert += 1;
            return;
        }
        self.armed += 1;
        let scopes = self.jperms.as_ref().or(self.wperms.as_ref());
        if self.checkout {
            if !grants_contents(scopes) {
                self.checkout_findings.push(format!(
                    "{}:{}: job '{}' checks out with no 'contents:' scope in scope",
                    self.curfile, self.jobline, self.job
                ));
            }
            return;
        }
        if !declares_scopes(scopes) {
            self.declare_findings.push(format!(
                "{}:{}: job '{}' consumes the GitHub token under no declared scope",
                self.curfile, self.jobline, self.job
            ));
        }
    }

    fn consume(&mut self, ev: Ev) {
        match ev {
            Ev::Job(name, line) => {
                self.finish_job();
                self.job = name;
                self.jobline = line;
                self.jexempt = false;
                self.jperms = None;
                self.checkout = false;
                self.gh = false;
                self.token = false;
                self.have_job = true;
            }
            Ev::JobExempt => self.jexempt = true,
            Ev::JobPerms(p, _) => self.jperms = Some(p),
            Ev::Step => {
                self.finish_step();
                self.have_step = true;
            }
            Ev::StepExempt => self.sexempt = true,
            Ev::Checkout(_) => {
                self.scheckout = true;
                self.have_step = true;
            }
            Ev::Gh(_, _) => {
                self.sgh = true;
                self.have_step = true;
            }
            // spec: gate-sdk/SPEC.md §check-action-permissions — a reference inside a step is
            // that step's evidence and droppable by its valve; one in the job's own keys is the
            // job's and is not.
            Ev::Token => {
                if self.have_step {
                    self.stoken = true;
                } else {
                    self.token = true;
                }
            }
            Ev::BareMarker(line) => self.bare.push(format!(
                "{}:{}: an {} marker with no reason",
                self.curfile, line, MARKER
            )),
            Ev::WorkflowPerms(_, _) | Ev::JobEnv | Ev::StepEnv | Ev::WorkflowEnv => {}
        }
    }
}

const CHECKOUT_HELP: &[&str] = &[
    "  help: declare 'permissions:' with 'contents: read' on the job, or at workflow",
    "        level for the job to inherit. An empty allowlist grants nothing and does",
    "        not satisfy it; a job-level block replaces the workflow-level one whole.",
];

const DECLARE_HELP: &[&str] = &[
    "  help: declare 'permissions:' on the job naming the scopes its calls take — the",
    "        gate asserts that the job says what it takes, never which scope. A job",
    "        reaching the token outside this gate's theory takes",
    "        '# action-permissions-exempt: <reason>'.",
];

pub fn run(args: &[String]) -> i32 {
    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    let root = Path::new(scanroot);
    if !root.is_dir() {
        eprintln!("check-action-permissions: scan root not found: {}", scanroot);
        return 2;
    }

    let files = match walk::find_files(root, &["yml", "yaml"]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("check-action-permissions: {} — the check could not run; treating as failure (not clean)", e);
            return 2;
        }
    };

    if files.is_empty() {
        println!(
            "ACTION-PERMISSIONS: clean (no YAML under {} — 0 job(s) to check)",
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
                    "check-action-permissions: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                    f.display(),
                    e
                );
                return 2;
            }
        };
        walked += 1;
        // spec: gate-sdk/SPEC.md §check-action-permissions — the split predicate is
        // §check-action-gh-repo's, consumed unchanged: a composite action has no job and no
        // `permissions:` of its own, so the assertion belongs to its caller.
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

        // spec: gate-sdk/SPEC.md §check-action-permissions — a value the gate cannot resolve is
        // a refusal naming the construct, raised before any arm reads a grant it invented.
        for ev in &stream {
            let bad = match ev {
                Ev::WorkflowPerms(Perms::Unreadable(v), l) => Some((v, l)),
                Ev::JobPerms(Perms::Unreadable(v), l) => Some((v, l)),
                _ => None,
            };
            if let Some((v, l)) = bad {
                eprintln!(
                    "check-action-permissions: {}:{}: a permissions: value that is neither a mapping, nor read-all/write-all, nor empty ({}) — the check could not run; treating as failure (not clean)",
                    a.curfile, l, v
                );
                return 2;
            }
        }

        a.wperms = stream.iter().find_map(|e| match e {
            Ev::WorkflowPerms(p, _) => Some(p.clone()),
            _ => None,
        });
        for ev in stream {
            a.consume(ev);
        }
        a.finish_job();
    }

    let mut red = false;

    if !a.checkout_findings.is_empty() {
        red = true;
        println!("check-action-permissions: a job checks out with no 'contents:' scope in");
        println!("scope, so the fetch is refused as a 404 that reads as an absent repository —");
        println!("invisibly on a public repository, immediately on a private one:");
        for x in &a.checkout_findings {
            println!("  {}", x);
        }
        for l in CHECKOUT_HELP {
            println!("{}", l);
        }
    }

    if !a.declare_findings.is_empty() {
        red = true;
        println!("check-action-permissions: a job consumes the GitHub token under no declared");
        println!("scope, so what it takes is whatever the repository default happens to be and");
        println!("nothing in the tree says what that is:");
        for x in &a.declare_findings {
            println!("  {}", x);
        }
        for l in DECLARE_HELP {
            println!("{}", l);
        }
    }

    if !a.bare.is_empty() {
        red = true;
        println!(
            "check-action-permissions: an {} marker carries no reason, so it",
            MARKER
        );
        println!("records that a job was stood outside the assertion without saying why:");
        for x in &a.bare {
            println!("  {}", x);
        }
        println!("  help: write the marker as '# action-permissions-exempt: <reason>' naming how");
        println!("        the job reaches the token, or delete it and declare the scopes.");
    }

    if red {
        return 1;
    }

    println!(
        "ACTION-PERMISSIONS: clean ({} job(s) consuming the GitHub token across {} Actions-shaped file(s) of {} walked, all declaring the scopes they take; {} job(s) consuming none, {} exempt, {} composite-action file(s) and {} non-Actions file(s) skipped)",
        a.armed, subject, walked, a.inert, a.exempt, composite, outside
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn audit(wperms: Option<Perms>, stream: Vec<Ev>) -> Audit {
        let mut a = Audit {
            curfile: "f.yml".to_string(),
            wperms,
            ..Audit::default()
        };
        for ev in stream {
            a.consume(ev);
        }
        a.finish_job();
        a
    }

    fn scopes(pairs: &[(&str, &str)]) -> Perms {
        Perms::Scopes(
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    }

    // spec: gate-sdk/SPEC.md §check-action-permissions — a job armed by a checkout takes the
    // contents arm, which `write` and a grant-everything shorthand both satisfy.
    #[test]
    fn the_checkout_arm_names_the_scope_it_needs() {
        for granting in [
            scopes(&[("contents", "read")]),
            scopes(&[("contents", "write")]),
            Perms::All,
        ] {
            let a = audit(
                None,
                vec![
                    Ev::Job("j".into(), 1),
                    Ev::JobPerms(granting, 2),
                    Ev::Step,
                    Ev::Checkout(4),
                ],
            );
            assert!(a.checkout_findings.is_empty());
            assert_eq!(a.armed, 1);
        }
        for refusing in [
            scopes(&[("contents", "none")]),
            scopes(&[("id-token", "write")]),
            scopes(&[]),
        ] {
            let a = audit(
                None,
                vec![
                    Ev::Job("j".into(), 1),
                    Ev::JobPerms(refusing, 2),
                    Ev::Step,
                    Ev::Checkout(4),
                ],
            );
            assert_eq!(a.checkout_findings.len(), 1);
            assert!(a.declare_findings.is_empty());
        }
    }

    // spec: gate-sdk/SPEC.md §check-action-permissions — a job-level block replaces the
    // workflow-level one rather than adding to it, so inheritance applies only where the job
    // declares nothing of its own.
    #[test]
    fn a_job_block_replaces_the_workflow_block_it_does_not_extend_it() {
        let inherits = audit(
            Some(scopes(&[("contents", "read")])),
            vec![Ev::Job("j".into(), 1), Ev::Step, Ev::Checkout(3)],
        );
        assert!(inherits.checkout_findings.is_empty());

        let replaces = audit(
            Some(scopes(&[("contents", "read")])),
            vec![
                Ev::Job("j".into(), 1),
                Ev::JobPerms(scopes(&[("id-token", "write")]), 2),
                Ev::Step,
                Ev::Checkout(4),
            ],
        );
        assert_eq!(replaces.checkout_findings.len(), 1);
    }

    // spec: gate-sdk/SPEC.md §check-action-permissions — the declaration arm is taken by a job
    // armed by `gh` or a token reference and not by a checkout, and an empty allowlist fails it.
    #[test]
    fn the_declaration_arm_holds_a_gh_or_token_job_and_the_empty_allowlist_fails_both() {
        let empty = audit(
            Some(scopes(&[])),
            vec![Ev::Job("j".into(), 1), Ev::Step, Ev::Gh(3, true)],
        );
        assert_eq!(empty.declare_findings.len(), 1);

        let declared = audit(
            None,
            vec![
                Ev::Job("j".into(), 1),
                Ev::JobPerms(scopes(&[("issues", "write")]), 2),
                Ev::Token,
            ],
        );
        assert!(declared.declare_findings.is_empty());
        assert_eq!(declared.armed, 1);

        let bare_token = audit(None, vec![Ev::Job("j".into(), 1), Ev::Token]);
        assert_eq!(bare_token.declare_findings.len(), 1);
    }

    // spec: gate-sdk/SPEC.md §check-action-permissions — a job armed by none is inert and
    // counted, and the valve binds a job whole or a step narrowly.
    #[test]
    fn inertness_is_counted_and_the_valve_binds_at_two_widths() {
        let inert = audit(None, vec![Ev::Job("j".into(), 1), Ev::Step]);
        assert_eq!(inert.inert, 1);
        assert_eq!(inert.armed, 0);

        let job = audit(
            None,
            vec![
                Ev::Job("j".into(), 1),
                Ev::JobExempt,
                Ev::Step,
                Ev::Checkout(3),
            ],
        );
        assert_eq!(job.exempt, 1);
        assert!(job.checkout_findings.is_empty());

        let step = audit(
            None,
            vec![
                Ev::Job("j".into(), 1),
                Ev::Step,
                Ev::StepExempt,
                Ev::Checkout(3),
            ],
        );
        assert_eq!(step.inert, 1);
        assert!(step.checkout_findings.is_empty());

        let step_narrow = audit(
            None,
            vec![
                Ev::Job("j".into(), 1),
                Ev::Step,
                Ev::StepExempt,
                Ev::Gh(3, true),
                Ev::Step,
                Ev::Checkout(5),
            ],
        );
        assert_eq!(step_narrow.checkout_findings.len(), 1);

        let bare = audit(None, vec![Ev::Job("j".into(), 1), Ev::BareMarker(2)]);
        assert_eq!(
            bare.bare,
            vec!["f.yml:2: an action-permissions-exempt marker with no reason"]
        );
    }
}
