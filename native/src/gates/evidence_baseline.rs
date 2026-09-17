// spec: evidence-kit/SPEC.md §check-evidence-baseline — baseline grammar, blocking-slug
// liveness against the queue, per-suite manifest↔disk set equality, and flip causation
use crate::evidence::data_lines;
use crate::proc;
use crate::stages;
use crate::walk;
use std::path::Path;

fn knob_or(args: &[String], at: usize, knob: &str) -> Result<String, String> {
    match args.get(at).filter(|a| !a.is_empty()) {
        Some(v) => Ok(v.clone()),
        None => walk::knob_scalar(knob),
    }
}

// spec: evidence-kit/SPEC.md §check-evidence-baseline — bash `read -r suite scenario status
// slug rest`: whitespace-separated fields with everything past the fourth landing in `rest`
fn fields(line: &str) -> (String, String, String, String, String) {
    let f: Vec<&str> = line.split_whitespace().collect();
    let at = |i: usize| f.get(i).copied().unwrap_or("").to_string();
    (at(0), at(1), at(2), at(3), f[4.min(f.len())..].join(" "))
}

// spec: evidence-kit/SPEC.md §check-evidence-baseline — the queue's `<slug> <section>` walk: a
// `## ` heading names the section, and a bold-slug bullet under it claims membership of it
fn queue_entries(text: &str) -> Vec<(String, String)> {
    let mut sec = String::new();
    let mut out = Vec::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("##") {
            if rest.starts_with([' ', '\t']) {
                sec = rest.trim_start_matches([' ', '\t']).trim_end().to_string();
                continue;
            }
        }
        let Some(after_dash) = line.strip_prefix('-') else {
            continue;
        };
        if !after_dash.starts_with([' ', '\t']) {
            continue;
        }
        let rest = after_dash.trim_start_matches([' ', '\t']);
        let Some(rest) = rest.strip_prefix("**") else {
            continue;
        };
        let Some(end) = rest.find("**") else { continue };
        let slug = &rest[..end];
        if slug.is_empty() || slug.contains('*') {
            continue;
        }
        out.push((slug.to_string(), sec.clone()));
    }
    out
}

const GRAMMAR: &str = "<suite> <scenario> <status> [<slug> [reproduces-at=<rev>]]";

// spec: evidence-kit/SPEC.md §Baseline manifest — the fifth token's one legal shape, a head in
// the 7-to-40 lowercase-hex form `--enter-stage` writes
fn reproduces_at(tok: &str) -> Option<&str> {
    let rev = tok.strip_prefix("reproduces-at=")?;
    let hex = rev.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c));
    ((7..=40).contains(&rev.len()) && hex).then_some(rev)
}

struct Row<'a> {
    line: &'a str,
    suite: String,
    scenario: String,
    status: String,
    rev: Option<String>,
}

// spec: evidence-kit/SPEC.md §check-evidence-baseline — a flip is a row held red now whose
// scenario the prior baseline held `pass`; a scenario the prior baseline lacks is never one
fn untokened_flips<'r, 'a>(prior: &str, rows: &'r [Row<'a>]) -> Vec<&'r Row<'a>> {
    let passed: Vec<(String, String)> = data_lines(prior)
        .iter()
        .map(|l| fields(l))
        .filter(|f| f.2 == "pass")
        .map(|f| (f.0, f.1))
        .collect();
    rows.iter()
        .filter(|r| r.status == "fail" || r.status == "ignore")
        .filter(|r| r.rev.is_none())
        .filter(|r| passed.iter().any(|(s, sc)| *s == r.suite && *sc == r.scenario))
        .collect()
}

// spec: evidence-kit/SPEC.md §check-evidence-baseline — the flip assertion: Ok carries the clean
// line's account of which branch ran, Err the fail-closed refusal
fn flip_causation(baseline: &str, state: &str, rows: &[Row], errors: &mut Vec<String>) -> Result<String, String> {
    let start = stages::iteration_start(state);
    if start.is_empty() {
        return Ok("flip causation off: no iteration-start commit".to_string());
    }
    let closed = |what: String| {
        format!("{} — the check could not run; treating as failure (not clean)", what)
    };
    let listed = proc::run("git", &["ls-tree", "--full-name", "--name-only", &start, "--", baseline])?;
    let Some(listed) = listed.stdout() else {
        return Err(closed(format!("cannot list {} at the iteration start {}", baseline, start)));
    };
    let name = String::from_utf8_lossy(listed).trim().to_string();
    let prior = if name.is_empty() {
        String::new()
    } else {
        let shown = proc::run("git", &["show", &format!("{}:{}", start, name)])?;
        match shown.stdout() {
            Some(b) => String::from_utf8_lossy(b).into_owned(),
            None => return Err(closed(format!("cannot read {} at the iteration start {}", name, start))),
        }
    };
    let flips = untokened_flips(&prior, rows);
    for r in &flips {
        errors.push(format!(
            "{} {} passed at the iteration start {} and is now {}; a hold needs reproduces-at=<rev> at or before {}, or the red is a regression to fix: {}",
            r.suite, r.scenario, start, r.status, start, r.line
        ));
    }
    for r in rows {
        let Some(rev) = &r.rev else { continue };
        if !stages::commit_resolves(rev) {
            errors.push(format!("reproduces-at '{}' resolves to no commit: {}", rev, r.line));
            continue;
        }
        let anc = proc::run("git", &["merge-base", "--is-ancestor", rev, &start])?;
        match anc.code() {
            Some(0) => {}
            Some(1) => errors.push(format!(
                "reproduces-at '{}' names a commit inside the iteration (not at or before its start {}): {}",
                rev, start, r.line
            )),
            _ => return Err(closed(format!("cannot order {} against the iteration start {}", rev, start))),
        }
    }
    Ok(format!("flip causation judged against the iteration start {}", start))
}

pub fn run(args: &[String]) -> i32 {
    let (baseline, queue, state, globs, permanent) = match (
        knob_or(args, 0, "EVIDENCE_KIT_BASELINE_FILE"),
        knob_or(args, 1, "EVIDENCE_KIT_QUEUE_FILE"),
        knob_or(args, 2, "EVIDENCE_KIT_STATE_FILE"),
        walk::knob_map("EVIDENCE_KIT_SCENARIO_GLOBS"),
        walk::knob_array("EVIDENCE_KIT_PERMANENT_SLUGS"),
    ) {
        (Ok(a), Ok(b), Ok(e), Ok(c), Ok(d)) => (a, b, e, c, d),
        (a, b, e, c, d) => {
            let err = [a.err(), b.err(), e.err(), c.err(), d.err()]
                .into_iter()
                .flatten()
                .next()
                .unwrap_or_default();
            eprintln!("check-evidence-baseline: {}", err);
            return 2;
        }
    };

    if !Path::new(&baseline).is_file() {
        println!("EVIDENCE-BASELINE: baseline not found: {}", baseline);
        println!("  help: create {} with a comment header and one '{}' line per known scenario", baseline, GRAMMAR);
        return 1;
    }
    let btext = match std::fs::read(&baseline) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => {
            eprintln!(
                "check-evidence-baseline: cannot read {}: {} — the check could not run; treating as failure (not clean)",
                baseline, e
            );
            return 2;
        }
    };
    let blines = data_lines(&btext);

    let mut errors: Vec<String> = Vec::new();
    let mut blocking: Vec<String> = Vec::new();
    let mut rows: Vec<Row> = Vec::new();
    for line in &blines {
        let (suite, scenario, status, slug, rest) = fields(line);
        if suite.is_empty() || scenario.is_empty() || status.is_empty() {
            errors.push(format!("malformed line (want '{}'): {}", GRAMMAR, line));
            continue;
        }
        let rev = if rest.is_empty() {
            None
        } else if let Some(rev) = reproduces_at(&rest) {
            Some(rev.to_string())
        } else {
            errors.push(format!(
                "too many fields (a slug is a single token, and the only fifth field is reproduces-at=<rev>): {}",
                line
            ));
            continue;
        };
        match status.as_str() {
            "pass" => {
                if !slug.is_empty() {
                    errors.push(format!("a 'pass' scenario takes no blocking slug: {}", line));
                }
                if rev.is_some() {
                    errors.push(format!("a 'pass' scenario takes no reproduces-at token: {}", line));
                }
            }
            "fail" | "ignore" => {
                if slug.is_empty() {
                    errors.push(format!(
                        "a '{}' scenario requires a blocking slug (a live task or permanent marker): {}",
                        status, line
                    ));
                } else if !blocking.contains(&slug) {
                    blocking.push(slug);
                }
            }
            _ => errors.push(format!(
                "bad status '{}' (want pass|fail|ignore): {}",
                status, line
            )),
        }
        rows.push(Row { line, suite, scenario, status, rev });
    }

    if !blocking.is_empty() {
        let qtext = std::fs::read(&queue)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        let entries = queue_entries(&qtext);
        let live: Vec<&String> = entries
            .iter()
            .filter(|(_, sec)| sec != "Done")
            .map(|(s, _)| s)
            .collect();
        blocking.sort();
        for slug in &blocking {
            if permanent.contains(slug) || live.contains(&slug) {
                continue;
            }
            if entries.iter().any(|(s, sec)| s == slug && sec == "Done") {
                errors.push(format!(
                    "blocking slug '{}' is a Done task — stale; promote the scenario or repoint the slug",
                    slug
                ));
            } else {
                errors.push(format!(
                    "blocking slug '{}' resolves to no live task in {} and no permanent marker",
                    slug, queue
                ));
            }
        }
    }

    for (suite, glob) in &globs {
        let parts: Vec<String> = glob.split_whitespace().map(String::from).collect();
        let files = match walk::glob_files(Path::new("."), &parts) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "check-evidence-baseline: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        };
        let mut on_disk: Vec<String> = files
            .iter()
            .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(String::from))
            .collect();
        on_disk.sort();
        on_disk.dedup();
        let mut in_base: Vec<String> = blines
            .iter()
            .filter_map(|l| {
                let f: Vec<&str> = l.split_whitespace().collect();
                match (f.first(), f.get(1)) {
                    (Some(s), Some(sc)) if *s == suite => Some(sc.to_string()),
                    _ => None,
                }
            })
            .collect();
        in_base.sort();
        in_base.dedup();
        for s in &in_base {
            if !on_disk.contains(s) {
                errors.push(format!(
                    "suite '{}': baseline scenario '{}' matches no file under glob '{}'",
                    suite, s, glob
                ));
            }
        }
        for s in &on_disk {
            if !in_base.contains(s) {
                errors.push(format!(
                    "suite '{}': on-disk scenario '{}' (glob '{}') has no baseline line",
                    suite, s, glob
                ));
            }
        }
    }

    // spec: evidence-kit/SPEC.md §check-evidence-baseline — every configured suite carries at
    // least one baseline row; the roster is EVIDENCE_KIT_SUITES, so a suite acquires and drops
    // the obligation with no edit here, and a suite set that will not resolve is unjudgeable
    let suites = match walk::knob_array("EVIDENCE_KIT_SUITES") {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "check-evidence-baseline: {} — the check could not run; treating as failure (not clean)",
                e
            );
            return 2;
        }
    };
    if !suites.is_empty() {
        let covered: Vec<&str> = blines
            .iter()
            .filter_map(|l| l.split_whitespace().next())
            .collect();
        for s in &suites {
            if !covered.contains(&s.as_str()) {
                errors.push(format!(
                    "configured suite '{}' carries no baseline row — a scenario going absent in it reds nothing",
                    s
                ));
            }
        }
    }

    let flip = match flip_causation(&baseline, &state, &rows, &mut errors) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("check-evidence-baseline: {}", e);
            return 2;
        }
    };

    if !errors.is_empty() {
        println!(
            "EVIDENCE-BASELINE: {} issue(s) in {}:",
            errors.len(),
            baseline
        );
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: each line is '{}'; a fail/ignore carries a live blocking slug; a row held red that passed at the iteration start carries reproduces-at=<rev> naming a commit at or before that start where its red reproduces (else it is a regression to fix); every configured suite owes at least one row, bought by configuring a parser rather than by hand-authoring rows; the baseline is edited by human commit only", GRAMMAR);
        return 1;
    }
    println!(
        "EVIDENCE-BASELINE: clean ({} scenario(s) over {} configured suite(s); grammar, slug liveness, suite coverage, and scenario coverage hold in {}; {})",
        blines.len(),
        suites.len(),
        baseline,
        flip
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: evidence-kit/SPEC.md §check-evidence-baseline — bash `read`'s field split: the fifth
    // and later tokens land in `rest`, which is what makes a multi-token slug a grammar error
    #[test]
    fn a_fifth_field_lands_in_rest_and_a_short_line_leaves_the_tail_empty() {
        assert_eq!(
            fields("u a fail slug extra tail"),
            (
                "u".into(),
                "a".into(),
                "fail".into(),
                "slug".into(),
                "extra tail".into()
            )
        );
        assert_eq!(
            fields("u a pass"),
            ("u".into(), "a".into(), "pass".into(), String::new(), String::new())
        );
    }

    // spec: evidence-kit/SPEC.md §check-evidence-baseline — a bold-slug bullet claims the
    // section it sits under, and a `### ` heading is not a section line
    #[test]
    fn a_bullet_claims_the_section_heading_above_it() {
        let q = "## New Features\n- **live-one** — x\n### Sub\n## Done\n- **gone** — y\nnot a bullet\n";
        assert_eq!(
            queue_entries(q),
            vec![
                ("live-one".to_string(), "New Features".to_string()),
                ("gone".to_string(), "Done".to_string()),
            ]
        );
    }

    // spec: evidence-kit/SPEC.md §Baseline manifest — the fifth token takes one shape only
    #[test]
    fn only_a_hex_head_is_a_reproduces_at_token() {
        assert_eq!(reproduces_at("reproduces-at=af66137c"), Some("af66137c"));
        assert_eq!(reproduces_at("reproduces-at=AF66137C"), None);
        assert_eq!(reproduces_at("reproduces-at=af661"), None);
        assert_eq!(reproduces_at("reproduces-at=af66137c extra"), None);
        assert_eq!(reproduces_at("other=af66137c"), None);
    }

    // spec: evidence-kit/SPEC.md §check-evidence-baseline — a flip is prior `pass` to held red with
    // no token; a tokened row, a row red already, and a scenario new since the start are not
    #[test]
    fn a_flip_is_a_prior_pass_held_red_without_a_token() {
        let row = |line: &'static str| {
            let (suite, scenario, status, _, rest) = fields(line);
            Row { line, suite, scenario, status, rev: reproduces_at(&rest).map(String::from) }
        };
        let rows = vec![
            row("u flipped fail t"),
            row("u held ignore t reproduces-at=af66137c"),
            row("u was-red fail t"),
            row("u new fail t"),
            row("u still pass"),
        ];
        let prior = "# h\nu flipped pass\nu held pass\nu was-red fail t\nu still pass\n";
        let got: Vec<&str> = untokened_flips(prior, &rows).iter().map(|r| r.line).collect();
        assert_eq!(got, vec!["u flipped fail t"]);
    }
}
