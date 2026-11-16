// spec: evidence-kit/SPEC.md §check-test-control — (A) the declarations, (B) every new baseline pass
// row named by a control, (C) on a close cursor every control witnessed red by the current run
use crate::evidence;
use crate::stages;
use crate::walk;
use std::path::Path;

fn knob_or(args: &[String], at: usize, knob: &str) -> Result<String, String> {
    match args.get(at).filter(|a| !a.is_empty()) {
        Some(v) => Ok(v.clone()),
        None => walk::knob_scalar(knob),
    }
}

fn read_or_empty(path: &str) -> String {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default()
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-test-control: {}", e);
            2
        }
    }
}

fn report(what: &str, findings: &[String], help: &str) -> i32 {
    println!("TEST-CONTROL: {} {}:", findings.len(), what);
    for f in findings {
        println!("  {}", f);
    }
    println!("  help: {}", help);
    1
}

fn rule(args: &[String]) -> Result<i32, String> {
    let names = walk::knob_array("EVIDENCE_KIT_CONTROLS")?;
    if names.is_empty() {
        println!("TEST-CONTROL: clean (not applicable: EVIDENCE_KIT_CONTROLS is empty)");
        return Ok(0);
    }
    let family = walk::knob_prefix(evidence::CONTROL_PREFIX)?;
    let suites = walk::knob_array("EVIDENCE_KIT_SUITES")?;
    let baseline = knob_or(args, 0, "EVIDENCE_KIT_BASELINE_FILE")?;
    let manifest = knob_or(args, 2, "EVIDENCE_KIT_MANIFEST_FILE")?;
    let queue = knob_or(args, 3, "EVIDENCE_KIT_QUEUE_FILE")?;
    let state = knob_or(args, 4, "EVIDENCE_KIT_STATE_FILE")?;

    // assertion A: every declaration parses, and no two name one suite and scenario
    let (controls, mut decl) = evidence::controls(&names, &family, &suites);
    for (i, c) in controls.iter().enumerate() {
        if let Some(d) = controls[..i].iter().find(|d| d.suite == c.suite && d.scenario == c.scenario) {
            decl.push(format!("controls '{}' and '{}' both name {} {}", d.name, c.name, c.suite, c.scenario));
        }
    }
    if !decl.is_empty() {
        return Ok(report(
            "declaration issue(s)",
            &decl,
            "each EVIDENCE_KIT_CONTROL_<name> is '<suite> <scenario> <VAR>=<value> [...]', its suite in EVIDENCE_KIT_SUITES, one control per suite and scenario (evidence-kit/SPEC.md §Test controls)",
        ));
    }

    let mut findings: Vec<String> = Vec::new();
    let new_account = new_tests(args, &baseline, &state, &controls, &mut findings)?;
    let close_account = close_witness(&manifest, &queue, &state, &controls, &mut findings);
    if !findings.is_empty() {
        return Ok(report(
            "issue(s)",
            &findings,
            "declare a control for each newly baselined test, and run --run-validate so every control is witnessed red before the close entry (evidence-kit/SPEC.md §check-test-control)",
        ));
    }
    println!(
        "TEST-CONTROL: clean ({} control(s) declared; {}; {})",
        controls.len(),
        new_account,
        close_account
    );
    Ok(0)
}

// spec: evidence-kit/SPEC.md §check-test-control — (B): the since baseline from the positional, else
// the baseline at the iteration start, else at EVIDENCE_KIT_CONTROL_SINCE, else disarmed
fn new_tests(
    args: &[String],
    baseline: &str,
    state: &str,
    controls: &[evidence::Control],
    findings: &mut Vec<String>,
) -> Result<String, String> {
    let (since_text, label) = match args.get(1).filter(|a| !a.is_empty()) {
        Some(file) => (read_or_empty(file), format!("the since baseline {}", file)),
        None => {
            let start = stages::iteration_start(state);
            let (commit, what) = if !start.is_empty() {
                (start, "the iteration start")
            } else {
                let rev = walk::knob_scalar("EVIDENCE_KIT_CONTROL_SINCE")?;
                if rev.is_empty() {
                    return Ok("new-test check off: no iteration-start commit and EVIDENCE_KIT_CONTROL_SINCE is empty".to_string());
                }
                if !stages::commit_resolves(&rev) {
                    return Err(format!(
                        "EVIDENCE_KIT_CONTROL_SINCE '{}' resolves to no commit — the check could not run; treating as failure (not clean)",
                        rev
                    ));
                }
                (rev, "EVIDENCE_KIT_CONTROL_SINCE")
            };
            (evidence::baseline_at(&commit, what, baseline)?, format!("{} {}", what, commit))
        }
    };
    let held: Vec<(String, String)> = evidence::data_lines(&since_text)
        .iter()
        .filter_map(|l| {
            let mut f = l.split_whitespace();
            Some((f.next()?.to_string(), f.next()?.to_string()))
        })
        .collect();
    let mut fresh = 0;
    for line in evidence::data_lines(&read_or_empty(baseline)) {
        let f: Vec<&str> = line.split_whitespace().collect();
        let (Some(suite), Some(scenario), Some(&"pass")) = (f.first(), f.get(1), f.get(2)) else {
            continue;
        };
        if held.iter().any(|(s, sc)| s == suite && sc == scenario) {
            continue;
        }
        fresh += 1;
        if !controls.iter().any(|c| c.suite == *suite && c.scenario == *scenario) {
            findings.push(format!(
                "new test '{} {}' (baselined pass since {}) is named by no control",
                suite, scenario, label
            ));
        }
    }
    Ok(format!("{} new test(s) judged against {}", fresh, label))
}

// spec: evidence-kit/SPEC.md §check-test-control — (C): a control line counts only while it follows
// its suite's current line, so a later suite line, an import's among them, leaves it unwitnessed
fn close_witness(
    manifest: &str,
    queue: &str,
    state: &str,
    controls: &[evidence::Control],
    findings: &mut Vec<String>,
) -> String {
    let iter = evidence::queue_iteration(&read_or_empty(queue)).unwrap_or_default();
    let stext = read_or_empty(state);
    let stage = evidence::state_stage(&stext).unwrap_or_default();
    if iter.is_empty() || !Path::new(state).is_file() || stage.is_empty() {
        return "no lifecycle state — close-entry witness disarmed".to_string();
    }
    if stage != "close" {
        return format!("close-entry witness not due at stage '{}'", stage);
    }
    let earliest = evidence::earliest_validate(&stext, &iter).unwrap_or_default();
    let mut current: Vec<(String, String, String)> = Vec::new();
    let mut witnessed: Vec<(String, Vec<String>, (String, String))> = Vec::new();
    for line in evidence::data_lines(&read_or_empty(manifest)) {
        let f: Vec<String> = line.split_whitespace().map(String::from).collect();
        if f.len() < 3 || f[0] != iter {
            continue;
        }
        let field = |i: usize, key: &str| f.get(i).and_then(|v| v.strip_prefix(key)).unwrap_or("").to_string();
        if f[2].starts_with("sha256=") {
            current.retain(|(s, _, _)| *s != f[1]);
            current.push((f[1].clone(), field(8, "commit="), field(10, "cmd=")));
            witnessed.retain(|(_, w, _)| w[1] != f[1]);
        } else if let Some(name) = f[2].strip_prefix("control=") {
            let Some((_, commit, cmd)) = current.iter().find(|(s, _, _)| *s == f[1]) else {
                continue;
            };
            witnessed.retain(|(n, _, _)| n != name);
            witnessed.push((name.to_string(), f.clone(), (commit.clone(), cmd.clone())));
        }
    }
    for c in controls {
        let Some((_, f, (commit, cmd))) = witnessed.iter().find(|(n, w, _)| *n == c.name && w[1] == c.suite) else {
            findings.push(format!(
                "close entry: control '{}' has no witness under suite '{}''s current line for '{}'",
                c.name, c.suite, iter
            ));
            continue;
        };
        let get = |i: usize| f.get(i).map(String::as_str).unwrap_or("");
        if get(3) != format!("scenario={}", c.scenario) {
            findings.push(format!("close entry: control '{}' witnessed '{}', not its declared scenario {}", c.name, get(3), c.scenario));
        }
        if get(4) != "witnessed=red" {
            findings.push(format!("close entry: control '{}' {} — {} {} did not fail under its assignments", c.name, get(4), c.suite, c.scenario));
        }
        if get(6) != format!("commit={}", commit) || get(7) != format!("cmd={}", cmd) {
            findings.push(format!(
                "close entry: control '{}' names {} {}, not the commit and command of the suite line it follows",
                c.name,
                get(6),
                get(7)
            ));
        }
        if get(8) != format!("assign={}", evidence::assignments_digest(c)) {
            findings.push(format!(
                "close entry: control '{}' was witnessed under other assignments than its declaration's — re-run --run-validate",
                c.name
            ));
        }
        if !earliest.is_empty() && get(5) < earliest.as_str() {
            findings.push(format!(
                "close entry: control '{}' is dated {}, before the earliest validate stamp {} — stale evidence",
                c.name,
                get(5),
                earliest
            ));
        }
    }
    format!("every control witnessed red for '{}' at stage 'close'", iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: evidence-kit/SPEC.md §check-test-control — a suite line after a control line leaves the
    // control unwitnessed, which is how an imported suite line fails (C)
    #[test]
    fn a_later_suite_line_unwitnesses_the_controls_above_it() {
        let dir = std::env::temp_dir().join(format!("cw-test-control-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = |n: &str| dir.join(n).display().to_string();
        let h = "a".repeat(64);
        let suite = |origin: &str| {
            format!(
                "it unit sha256={h} pass=1 fail=0 ignore=0 verdict=clean 2026-01-02 commit=- tree=- cmd={h} origin={origin}\n"
            )
        };
        std::fs::write(p("queue"), "## Iteration: it\n").unwrap();
        std::fs::write(p("state"), "h\n---\nit validate s 2026-01-01 x\nit close s 2026-01-03 x\n").unwrap();
        let ctl = vec![evidence::Control {
            name: "c".into(),
            suite: "unit".into(),
            scenario: "t".into(),
            env: vec![("BREAK".into(), "1".into())],
        }];
        let control = format!(
            "it unit control=c scenario=t witnessed=red 2026-01-02 commit=- cmd={h} assign={}\n",
            evidence::assignments_digest(&ctl[0])
        );
        let judge = |body: String| {
            std::fs::write(p("manifest"), format!("# contract: evidence-manifest v2\n{}", body)).unwrap();
            let mut f = Vec::new();
            close_witness(&p("manifest"), &p("queue"), &p("state"), &ctl, &mut f);
            f
        };
        assert!(judge(format!("{}{}", suite("local"), control)).is_empty());
        let after = judge(format!("{}{}{}", suite("local"), control, suite("imported")));
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(after.len(), 1, "{:?}", after);
        assert!(after[0].contains("has no witness"), "{:?}", after);
    }
}
