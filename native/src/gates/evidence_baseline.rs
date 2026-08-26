// spec: evidence-kit/SPEC.md §check-evidence-baseline — baseline grammar, blocking-slug
// liveness against the queue, and per-suite manifest↔disk set equality
use crate::evidence::data_lines;
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

pub fn run(args: &[String]) -> i32 {
    let (baseline, queue, globs, permanent) = match (
        knob_or(args, 0, "EVIDENCE_KIT_BASELINE_FILE"),
        knob_or(args, 1, "EVIDENCE_KIT_QUEUE_FILE"),
        walk::knob_map("EVIDENCE_KIT_SCENARIO_GLOBS"),
        walk::knob_array("EVIDENCE_KIT_PERMANENT_SLUGS"),
    ) {
        (Ok(a), Ok(b), Ok(c), Ok(d)) => (a, b, c, d),
        (a, b, c, d) => {
            let err = [a.err(), b.err(), c.err(), d.err()]
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
        println!("  help: create {} with a comment header and one '<suite> <scenario> <status> [<slug>]' line per known scenario", baseline);
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
    for line in &blines {
        let (suite, scenario, status, slug, rest) = fields(line);
        if suite.is_empty() || scenario.is_empty() || status.is_empty() {
            errors.push(format!(
                "malformed line (want '<suite> <scenario> <status> [<slug>]'): {}",
                line
            ));
            continue;
        }
        if !rest.is_empty() {
            errors.push(format!("too many fields (a slug is a single token): {}", line));
            continue;
        }
        match status.as_str() {
            "pass" => {
                if !slug.is_empty() {
                    errors.push(format!("a 'pass' scenario takes no blocking slug: {}", line));
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

    if !errors.is_empty() {
        println!(
            "EVIDENCE-BASELINE: {} issue(s) in {}:",
            errors.len(),
            baseline
        );
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: each line is '<suite> <scenario> <status> [<slug>]'; a fail/ignore carries a live blocking slug; every configured suite owes at least one row, bought by configuring a parser rather than by hand-authoring rows; the baseline is edited by human commit only");
        return 1;
    }
    println!(
        "EVIDENCE-BASELINE: clean ({} scenario(s) over {} configured suite(s); grammar, slug liveness, suite coverage, and scenario coverage hold in {})",
        blines.len(),
        suites.len(),
        baseline
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

}
