// spec: evidence-kit/SPEC.md §verify-gate-run — one Statement judged against this tree and its
// commit: its shape, its subject, and each suite's binding, command and tree. It writes nothing.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because 1 a contradiction and 2 an
// unreadable Statement are different answers, which the `--emit-` collapse would merge.
use crate::evidence::{self, Statement};
use crate::programs::{self, Program};
use crate::{proc, walk};
use std::io::Read;

pub const KNOBS: &[&str] = &[
    "EVIDENCE_KIT_STATEMENT_CMD",
    "EVIDENCE_KIT_SUITES",
    "EVIDENCE_KIT_RUN_*",
    "EVIDENCE_KIT_PARSER",
    "EVIDENCE_KIT_PARSER_*",
    "EVIDENCE_KIT_VERIFY_REQUIRE",
];

// spec: evidence-kit/SPEC.md §verify-gate-run — the `--json` document's version, under gate-sdk/SPEC.md
// §run-gates' verdict-contract rule
const DOCUMENT_CONTRACT: &str = "1.0.0";

pub fn run(args: &[String]) -> i32 {
    let (json, args) = json_option(args);
    let judged = read_input("--verify-gate-run [--json]", &args).and_then(|bytes| judge(&bytes)).and_then(|mut j| {
        let words = walk::knob_scalar("EVIDENCE_KIT_VERIFY_REQUIRE")?;
        let extra = required(&j, &words);
        j.findings.extend(extra);
        Ok(j)
    });
    let judged = match judged {
        Ok(j) => j,
        Err(e) => {
            eprintln!("verify-gate-run: {}", e);
            return 2;
        }
    };
    if json {
        println!("{}", document(&judged));
    } else {
        judged.report("VERIFY-GATE-RUN");
    }
    i32::from(!judged.findings.is_empty())
}

// spec: evidence-kit/SPEC.md §verify-gate-run — one `--json` is the option wherever it sits; a second
// stays an operand, which the input grammar refuses
fn json_option(args: &[String]) -> (bool, Vec<String>) {
    match args.iter().position(|a| a == "--json") {
        Some(i) => (true, args[..i].iter().chain(&args[i + 1..]).cloned().collect()),
        None => (false, args.to_vec()),
    }
}

// spec: evidence-kit/SPEC.md §verify-gate-run — one object, keys sorted, no insignificant whitespace;
// `ahead` is absent where the subject is no ancestor
fn document(j: &Judged) -> String {
    use serde_json::{Map, Value};
    let mut doc = Map::new();
    doc.insert("contract".into(), DOCUMENT_CONTRACT.into());
    doc.insert("verdict".into(), if j.findings.is_empty() { "clean" } else { "contradiction" }.into());
    doc.insert("subject".into(), j.statement.subject.as_str().into());
    if let Ok(n) = j.ahead.parse::<u64>() {
        doc.insert("ahead".into(), n.into());
    }
    let suites = j.statement.suites.iter().map(|s| {
        let mut o = Map::new();
        o.insert("name".into(), s.name.as_str().into());
        o.insert("tree".into(), s.tree.as_str().into());
        o.insert("verdict".into(), s.verdict.as_str().into());
        Value::Object(o)
    });
    doc.insert("suites".into(), Value::Array(suites.collect()));
    doc.insert("findings".into(), Value::Array(j.findings.iter().map(|f| f.as_str().into()).collect()));
    Value::Object(doc).to_string()
}

// spec: evidence-kit/SPEC.md §verify-gate-run — check 6, the verify arm's alone: the import shares
// `judge` and records an older or failing run as history, so the requirements stay out of it
fn required(j: &Judged, words: &str) -> Vec<String> {
    let mut findings = Vec::new();
    for word in words.split_whitespace() {
        match word {
            "head" if j.ahead != "0" && j.ahead != "-" => findings.push(format!(
                "subject {} is {} commit(s) behind HEAD (EVIDENCE_KIT_VERIFY_REQUIRE: head)",
                j.statement.subject, j.ahead
            )),
            "verdict" => findings.extend(j.statement.suites.iter().filter(|s| s.verdict != "clean").map(|s| {
                format!("suite '{}' verdict: {} (EVIDENCE_KIT_VERIFY_REQUIRE: verdict)", s.name, s.verdict)
            })),
            _ => {}
        }
    }
    findings
}

// spec: evidence-kit/SPEC.md §verify-gate-run — the input forms: a file, stdin for `-`, else the
// stdout of the consumer's command knob, its argv spawned with no shell
pub(crate) fn read_input(spelling: &str, args: &[String]) -> Result<Vec<u8>, String> {
    let usage = format!("usage: {} [<file> | -]", spelling);
    match args {
        [] => {}
        [one] if one == "-" => {
            let mut buf = Vec::new();
            std::io::stdin()
                .read_to_end(&mut buf)
                .map_err(|e| format!("cannot read stdin: {}", e))?;
            return Ok(buf);
        }
        [one] if !one.starts_with("--") => {
            return std::fs::read(one).map_err(|e| format!("cannot read {}: {}", one, e));
        }
        _ => return Err(format!("takes at most one operand (got: {}) — {}", args.join(" "), usage)),
    }
    let cmd = walk::knob_array("EVIDENCE_KIT_STATEMENT_CMD")?;
    if cmd.is_empty() {
        return Err(format!(
            "no Statement given: name a <file> or - on the command line, or set EVIDENCE_KIT_STATEMENT_CMD — {}",
            usage
        ));
    }
    let program = Program::consumer("EVIDENCE_KIT_STATEMENT_CMD", cmd[0].as_str());
    let rest: Vec<&str> = cmd[1..].iter().map(String::as_str).collect();
    let out = proc::run_streamed(&program, &rest, b"", proc::Stderr::Inherit)?;
    if out.code() != 0 {
        return Err(format!("EVIDENCE_KIT_STATEMENT_CMD exited {} — no Statement read", out.code()));
    }
    Ok(out.stdout().to_vec())
}

pub(crate) struct Judged {
    pub statement: Statement,
    pub findings: Vec<String>,
    pub ahead: String,
}

impl Judged {
    pub(crate) fn report(&self, tag: &str) {
        let summary = format!(
            "subject {}, {} suite(s), ahead={}; signature not checked",
            self.statement.subject,
            self.statement.suites.len(),
            self.ahead
        );
        if self.findings.is_empty() {
            println!("{}: clean ({})", tag, summary);
            return;
        }
        println!("{}: {} contradiction(s) ({}):", tag, self.findings.len(), summary);
        for f in &self.findings {
            println!("  {}", f);
        }
    }
}

fn git(args: &[&str]) -> Result<proc::Completed, String> {
    proc::run(&programs::GIT, args)
}

// spec: evidence-kit/SPEC.md §verify-gate-run — the checks in their order: shape refuses (`Err`,
// exit 2), and every later check is a finding naming the suite and the field
pub(crate) fn judge(bytes: &[u8]) -> Result<Judged, String> {
    let st = evidence::read_statement(bytes).map_err(|e| format!("not a gate-run Statement this reader can judge: {}", e))?;
    let mut findings: Vec<String> = Vec::new();
    let subject = st.subject.clone();
    let head = crate::provenance::head()
        .ok_or("HEAD names no commit here — the subject cannot be ordered against it")?;
    let mut ahead = "-".to_string();
    let resolves = git(&["rev-parse", "--verify", "--quiet", &format!("{}^{{commit}}", subject)])?;
    match resolves.code() {
        Some(0) => {
            let anc = git(&["merge-base", "--is-ancestor", &subject, &head])?;
            match anc.code() {
                Some(0) => {
                    let count = git(&["rev-list", "--count", &format!("{}..{}", subject, head)])?;
                    ahead = count
                        .stdout()
                        .map(|b| String::from_utf8_lossy(b).trim().to_string())
                        .filter(|n| evidence::is_digits(n))
                        .ok_or("cannot count the commits HEAD is ahead of the subject")?;
                }
                Some(1) => findings.push(format!("subject {} is neither HEAD nor an ancestor of it", subject)),
                _ => return Err(format!("cannot order the subject {} against HEAD", subject)),
            }
        }
        Some(1) => findings.push(format!("subject {} names no commit in this repository", subject)),
        _ => return Err(format!("cannot look the subject {} up", subject)),
    }
    let cfg = Resolution {
        suites: walk::knob_array("EVIDENCE_KIT_SUITES")?,
        run_family: walk::knob_prefix("EVIDENCE_KIT_RUN_")?,
        parser_family: walk::knob_prefix("EVIDENCE_KIT_PARSER_")?,
        parser: walk::knob_scalar("EVIDENCE_KIT_PARSER")?,
    };
    findings.extend(field_findings(&st, &cfg));
    Ok(Judged { statement: st, findings, ahead })
}

struct Resolution {
    suites: Vec<String>,
    run_family: Vec<(String, String)>,
    parser_family: Vec<(String, String)>,
    parser: String,
}

// spec: evidence-kit/SPEC.md §verify-gate-run — the checks that read the Statement and this tree's
// configuration alone: the provenance commit, then each suite's binding, command and tree
fn field_findings(st: &Statement, cfg: &Resolution) -> Vec<String> {
    let subject = &st.subject;
    let mut findings = Vec::new();
    match &st.provenance_commit {
        Some(c) if c == subject => {}
        Some(c) => findings.push(format!("provenance.commit {} differs from the subject {}", c, subject)),
        None => findings.push(format!("provenance carries no commit (the subject is {})", subject)),
    }
    for s in &st.suites {
        let bound = evidence::binding_digest(&s.name, &s.cmd, subject, s.counts, &s.verdict, &s.log);
        if bound != s.binding {
            findings.push(format!(
                "suite '{}' binding.sha256: does not recompute from its own fields (want {})",
                s.name, bound
            ));
        }
        let here = cfg
            .suites
            .contains(&s.name)
            .then(|| evidence::resolved_command_digest(&cfg.run_family, &cfg.parser_family, &cfg.parser, &s.name))
            .flatten();
        match here {
            None => findings.push(format!(
                "suite '{}' command.sha256: this tree configures no such suite (EVIDENCE_KIT_SUITES, EVIDENCE_KIT_RUN_{})",
                s.name, s.name
            )),
            Some(d) if d != s.cmd => findings.push(format!(
                "suite '{}' command.sha256: differs from this tree's resolution of its command ({})",
                s.name, d
            )),
            Some(_) => {}
        }
        if s.tree != "clean" {
            findings.push(format!(
                "suite '{}' tree: {} — it ran on content that is not the subject commit",
                s.name, s.tree
            ));
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Statement, Resolution) {
        let cfg = Resolution {
            suites: vec!["unit".into()],
            run_family: vec![("unit".into(), "cargo  test".into())],
            parser_family: Vec::new(),
            parser: "libtest".into(),
        };
        let subject = "c".repeat(40);
        let cmd = evidence::command_digest("cargo test", "libtest");
        let log = "a".repeat(64);
        let st = Statement {
            subject: subject.clone(),
            run: "it".into(),
            provenance_commit: Some(subject.clone()),
            suites: vec![evidence::StatementSuite {
                name: "unit".into(),
                binding: evidence::binding_digest("unit", &cmd, &subject, (3, 0, 1), "clean", &log),
                cmd,
                log,
                counts: (3, 0, 1),
                verdict: "clean".into(),
                date: "2026-01-01".into(),
                tree: "clean".into(),
            }],
        };
        (st, cfg)
    }

    // spec: evidence-kit/SPEC.md §verify-gate-run — a consistent Statement holds every field check
    #[test]
    fn a_consistent_statement_has_no_field_finding() {
        let (st, cfg) = fixture();
        assert!(field_findings(&st, &cfg).is_empty(), "{:?}", field_findings(&st, &cfg));
    }

    // spec: evidence-kit/SPEC.md §verify-gate-run — each field check reds on its own field
    #[test]
    fn each_field_check_reds_its_own_field() {
        let red = |edit: &dyn Fn(&mut Statement, &mut Resolution), want: &str| {
            let (mut st, mut cfg) = fixture();
            edit(&mut st, &mut cfg);
            let f = field_findings(&st, &cfg);
            assert!(f.len() == 1 && f[0].contains(want), "want one '{}' finding, got {:?}", want, f);
        };
        red(&|st, _| st.provenance_commit = Some("d".repeat(40)), "provenance.commit");
        red(&|st, _| st.provenance_commit = None, "provenance carries no commit");
        red(&|st, _| st.suites[0].counts.1 = 1, "binding.sha256");
        red(&|_, cfg| cfg.parser = "exit-code".into(), "differs from this tree's resolution");
        red(&|_, cfg| cfg.suites.clear(), "configures no such suite");
        red(&|_, cfg| cfg.run_family.clear(), "configures no such suite");
        red(&|st, _| st.suites[0].tree = "modified".into(), "tree: modified");
    }

    // spec: evidence-kit/SPEC.md §verify-gate-run — each requirement word reds its own case and holds
    // on a run of `HEAD` with every suite clean; an empty selection requires nothing
    #[test]
    fn each_requirement_reds_its_own_case() {
        let judged = |ahead: &str, verdict: &str| {
            let (mut st, _) = fixture();
            st.suites[0].verdict = verdict.into();
            Judged { statement: st, findings: Vec::new(), ahead: ahead.into() }
        };
        assert!(required(&judged("0", "clean"), "head verdict").is_empty());
        assert!(required(&judged("3", "new-failures"), "").is_empty());
        let behind = required(&judged("3", "clean"), "head");
        assert!(
            behind.len() == 1 && behind[0].contains("is 3 commit(s) behind HEAD (EVIDENCE_KIT_VERIFY_REQUIRE: head)"),
            "{:?}",
            behind
        );
        assert!(required(&judged("-", "clean"), "head").is_empty(), "check 2 already names a subject off HEAD's line");
        let failing = required(&judged("0", "new-failures"), "verdict");
        assert_eq!(failing, vec!["suite 'unit' verdict: new-failures (EVIDENCE_KIT_VERIFY_REQUIRE: verdict)".to_string()]);
        assert!(required(&judged("0", "new-failures"), "head").is_empty());
        assert_eq!(required(&judged("2", "new-failures"), "verdict head").len(), 2);
    }

    #[test]
    fn more_than_one_operand_or_a_flag_is_refused() {
        let two = ["a".to_string(), "b".to_string()];
        assert!(read_input("--verify-gate-run", &two).unwrap_err().contains("at most one operand"));
        let flag = ["--help".to_string()];
        assert!(read_input("--verify-gate-run", &flag).unwrap_err().contains("usage"));
    }

    // spec: evidence-kit/SPEC.md §verify-gate-run — `--json` is taken in either position and once; a
    // second, and any other `--` word, reaches the operand rules and is refused there
    #[test]
    fn one_json_option_is_taken_and_a_second_is_refused() {
        let words = |w: &[&str]| w.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(json_option(&words(&["--json", "s.json"])), (true, words(&["s.json"])));
        assert_eq!(json_option(&words(&["s.json", "--json"])), (true, words(&["s.json"])));
        assert_eq!(json_option(&words(&["-"])), (false, words(&["-"])));
        let (on, rest) = json_option(&words(&["--json", "--json"]));
        assert!(on);
        assert!(read_input("--verify-gate-run [--json]", &rest).unwrap_err().contains("usage: --verify-gate-run [--json] [<file> | -]"));
    }

    fn second_suite(st: &mut Statement, tree: &str) {
        let (cmd, log, counts) = (evidence::command_digest("cargo test", "libtest"), "b".repeat(64), (5, 1, 0));
        st.suites.push(evidence::StatementSuite {
            name: "smoke".into(),
            binding: evidence::binding_digest("smoke", &cmd, &st.subject, counts, "new-failures", &log),
            cmd,
            log,
            counts,
            verdict: "new-failures".into(),
            date: "2026-01-01".into(),
            tree: tree.into(),
        });
    }

    // spec: evidence-kit/SPEC.md §verify-gate-run — the goldens are the renderer's output on a fixed
    // judgment: a two-suite Statement clean at `ahead` 0, and a subject off HEAD's line with a suite
    // whose tree is `modified`
    #[test]
    fn the_document_matches_its_goldens() {
        let (mut st, mut cfg) = fixture();
        second_suite(&mut st, "clean");
        cfg.suites.push("smoke".into());
        cfg.run_family.push(("smoke".into(), "cargo test".into()));
        let clean = Judged { findings: field_findings(&st, &cfg), statement: st, ahead: "0".into() };
        assert!(clean.findings.is_empty(), "{:?}", clean.findings);
        let (mut st, mut cfg) = fixture();
        second_suite(&mut st, "modified");
        cfg.suites.push("smoke".into());
        cfg.run_family.push(("smoke".into(), "cargo test".into()));
        let mut findings = vec![format!("subject {} is neither HEAD nor an ancestor of it", st.subject)];
        findings.extend(field_findings(&st, &cfg));
        assert_eq!(findings.len(), 2, "{:?}", findings);
        let contradiction = Judged { statement: st, findings, ahead: "-".into() };
        for (judged, golden, name) in [
            (clean, include_str!("../../../docs/contracts/verify-gate-run/v1/clean.json"), "clean.json"),
            (contradiction, include_str!("../../../docs/contracts/verify-gate-run/v1/contradiction.json"), "contradiction.json"),
        ] {
            let rendered = format!("{}\n", document(&judged));
            assert_eq!(rendered, golden, "docs/contracts/verify-gate-run/v1/{} is stale; the renderer now writes:\n{}", name, rendered);
        }
    }
}
