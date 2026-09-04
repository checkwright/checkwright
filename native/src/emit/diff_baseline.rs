// spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — the situational runtime diff, not a precommit
// gate: parse each captured log named on argv, diff it against the baseline's suite slice
// per-scenario, print findings, and return the verdict as the process status.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because 1 is this tool's verdict and the
// emitting family can never carry it; its one functional caller reads nothing but the status.
use crate::evidence;
use crate::walk;
use std::path::Path;

// spec: evidence-kit/SPEC.md §Layout and configuration — the five names this arm resolves. Every
// one is defined and defaulted in the shell library the bridge sources, so a hardcoded flag would
// resolve platform defaults and ignore every consumer override.
pub const KNOBS: &[&str] = &[
    "EVIDENCE_KIT_BASELINE_FILE",
    "EVIDENCE_KIT_SKIP_FILE",
    "EVIDENCE_KIT_TMP_DIR",
    "EVIDENCE_KIT_PARSER",
    "EVIDENCE_KIT_PARSER_*",
];

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the usage a refusal prints, the `-h`/`--help`
// half having retired to the front-end. It names the `--` escape because the shape refusal it
// accompanies is the reason a caller would need one.
const USAGE: &str = "usage: run-gates.sh --diff-baseline [--] <suite> <logfile> [<status>] [<suite> <logfile> [<status>]...]
  <status> is the suite command's own exit status. A suite whose parser reads the
  log may omit it; an 'exit-code' suite may not, because the status is its verdict.
  \"--\" ends option processing, so a positional beginning with \"-\" is still reachable.";

pub fn run(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("diff-baseline: {}", e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the shape half, an *addition* this port makes
// rather than a behaviour it preserves; the scan itself is borrowed from the class's first
// free-text member rather than re-spelled, so the escape has one implementation.
fn positionals(args: &[String]) -> Result<&[String], String> {
    super::file_survey::positionals(args, "positional").map_err(|e| format!("{}\n{}", e, USAGE))
}

struct Cfg {
    baseline: String,
    skip: String,
    tmpdir: String,
    parser: String,
    parser_family: Vec<(String, String)>,
}

fn dispatch(args: &[String]) -> Result<i32, String> {
    let rest = positionals(args)?;
    if rest.len() < 2 {
        return Err(format!("too few arguments\n{}", USAGE));
    }
    let cfg = Cfg {
        baseline: walk::knob_scalar("EVIDENCE_KIT_BASELINE_FILE")?,
        skip: walk::knob_scalar("EVIDENCE_KIT_SKIP_FILE")?,
        tmpdir: walk::knob_scalar("EVIDENCE_KIT_TMP_DIR")?,
        parser: walk::knob_scalar("EVIDENCE_KIT_PARSER")?,
        parser_family: walk::knob_prefix("EVIDENCE_KIT_PARSER_"),
    };
    std::fs::create_dir_all(&cfg.tmpdir)
        .map_err(|e| format!("cannot create {}: {}", cfg.tmpdir, e))?;

    let baseline_text = read_or_empty(&cfg.baseline);
    let skip_text = if !cfg.skip.is_empty() && Path::new(&cfg.skip).is_file() {
        read_or_empty(&cfg.skip)
    } else {
        String::new()
    };

    let mut recoveries = 0usize;
    let mut rc = 0;
    let mut i = 0usize;
    while i < rest.len() {
        if rest.len() - i < 2 {
            return Err(format!("too few arguments\n{}", USAGE));
        }
        let suite = &rest[i];
        let log = &rest[i + 1];
        i += 2;
        // spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — the all-digit test stays inside the
        // group, where the shape refusal above cannot have reached it: a suite name suffixes
        // `EVIDENCE_KIT_RUN_<suite>` and so can never be all digits.
        let mut status: Option<i32> = None;
        if let Some(next) = rest.get(i) {
            if !next.is_empty() && next.bytes().all(|b| b.is_ascii_digit()) {
                status = next.parse().ok();
                i += 1;
            }
        }
        if !Path::new(log).is_file() {
            return Err(format!("log not found: {}", log));
        }
        let parser = evidence::parser_for(&cfg.parser_family, suite, &cfg.parser);
        let status = match status {
            Some(s) => s,
            None => {
                // spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — refuse rather than assume
                // success: an exit-code suite parsed against an assumed 0 reports pass for every
                // log it is ever handed, clearing a red it structurally cannot see.
                if parser == "exit-code" {
                    eprintln!(
                        "diff-baseline: suite '{}' is parsed by exit code and no status was given.",
                        suite
                    );
                    eprintln!("  help: pass the suite's own exit status as a third argument —");
                    eprintln!("        run-gates.sh --diff-baseline {} {} <status>", suite, log);
                    eprintln!("        Without it this tool cannot observe a failure in that suite at all.");
                    return Ok(2);
                }
                0
            }
        };

        let lines = evidence::parse(suite, Path::new(log), status, &parser)?;
        let parsed = format!("{}/diff-{}.parsed", cfg.tmpdir, suite);
        write_lines(&parsed, &lines)?;
        let observed = if lines.is_empty() {
            String::new()
        } else {
            lines.join("\n") + "\n"
        };

        let out = evidence::diff(&baseline_text, suite, &observed, &skip_text);
        if out.new_failure {
            rc = 1;
        }
        for f in &out.findings {
            println!("{}", f);
            if f.starts_with("recovery ") {
                recoveries += 1;
            }
        }
    }

    if rc != 0 {
        println!(
            "diff-baseline: NEW failures against {} (see 'new-failure' lines above)",
            cfg.baseline
        );
        return Ok(1);
    }
    println!(
        "diff-baseline: clean ({} unpromoted recovery finding(s); no new failure vs {})",
        recoveries, cfg.baseline
    );
    Ok(0)
}

fn read_or_empty(path: &str) -> String {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default()
}

fn write_lines(path: &str, lines: &[String]) -> Result<(), String> {
    let mut out = String::new();
    for l in lines {
        out.push_str(l);
        out.push('\n');
    }
    std::fs::write(path, out).map_err(|e| format!("cannot write {}: {}", path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the shape refusal crosses in both
    // directions: a dash-led positional in any slot is named rather than absorbed, and the
    // separator is what keeps the addition a fix rather than a capability loss.
    #[test]
    fn a_dash_led_positional_is_refused_and_a_separator_admits_it() {
        let err = positionals(&argv(&["--help", "x.log"])).expect_err("--help was absorbed");
        assert!(err.contains("--help"), "the refusal named no offender: {}", err);
        assert_eq!(
            positionals(&argv(&["--", "-suite", "x.log"])).expect("the separator did not end option processing"),
            &argv(&["-suite", "x.log"])[..]
        );
        assert_eq!(
            positionals(&argv(&["suite", "x.log", "1"])).expect("a legitimate group was refused"),
            &argv(&["suite", "x.log", "1"])[..]
        );
    }
}
