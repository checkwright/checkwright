// spec: lifecycle-kit/SPEC.md §check-stage-entry — prior-stage invocation-stamp ordering +
// drain-entry queue-empty + audit-trigger signal
use crate::ere::Ere;
use crate::stages;
use crate::walk;
use std::path::Path;

fn knob_or(args: &[String], at: usize, knob: &str) -> Result<String, String> {
    match args.get(at).filter(|a| !a.is_empty()) {
        Some(v) => Ok(v.clone()),
        None => walk::knob_scalar(knob),
    }
}

// spec: lifecycle-kit/SPEC.md §check-stage-entry — the awk section machine: an active-section
// heading opens the window, any other `## ` heading closes it, and only a `- ` bullet inside it
// is a queue entry. Returns each such bullet with its 1-based line number.
fn active_bullets<'a>(text: &'a str, sections: &[String]) -> Vec<(usize, &'a str)> {
    let mut out = Vec::new();
    let mut inq = false;
    for (i, line) in text.lines().enumerate() {
        if let Some(rest) = line.strip_prefix("## ") {
            if sections
                .iter()
                .any(|s| rest.trim_end_matches([' ', '\t']) == s)
            {
                inq = true;
                continue;
            }
            inq = false;
        }
        if !inq || !line.starts_with("- ") {
            continue;
        }
        out.push((i + 1, line));
    }
    out
}

// spec: lifecycle-kit/SPEC.md §check-stage-entry — the `[drain-exempt: <reason>]` tag: `Some`
// carries the trimmed reason, which is empty exactly when the tag records none. An unterminated
// tag matches nothing and the line reads as untagged residue, the awk regex's own behavior.
fn drain_exempt_reason(line: &str) -> Option<&str> {
    let at = line.find("[drain-exempt:")?;
    let rest = &line[at + "[drain-exempt:".len()..];
    let end = rest.find(']')?;
    Some(rest[..end].trim_matches([' ', '\t']))
}

// spec: lifecycle-kit/SPEC.md §check-stage-entry — `grep -oE`: every non-overlapping
// leftmost-longest match on one line, the form the contract-token scan reads amendment bodies with
fn all_matches<'a>(re: &Ere, line: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut base = 0usize;
    while base <= line.len() {
        let Some(hay) = line.get(base..) else { break };
        let Some((s, e)) = re.find(hay) else { break };
        if e == s {
            base += 1;
            continue;
        }
        out.push(&hay[s..e]);
        base += e;
    }
    out
}

fn stamp_present(text: &str, iter: &str, stage: &str) -> bool {
    stages::data_lines(text).iter().any(|l| {
        let f: Vec<&str> = l.split_whitespace().collect();
        f.first() == Some(&iter) && f.get(1) == Some(&stage)
    })
}

struct Knobs {
    queue: String,
    state: String,
    stage_set: Vec<String>,
    predecessor: Vec<(String, String)>,
    drain: String,
    active_sections: Vec<String>,
    audit_stage: String,
    audit_entry_stage: String,
    waiver: String,
    roster_basename: String,
    amendment_glob: String,
    contract_tokens: Vec<String>,
}

fn knobs(args: &[String]) -> Result<Knobs, String> {
    Ok(Knobs {
        queue: knob_or(args, 0, "LIFECYCLE_KIT_QUEUE_FILE")?,
        state: knob_or(args, 1, "LIFECYCLE_KIT_STATE_FILE")?,
        stage_set: stages::stages()?,
        predecessor: walk::knob_map("LIFECYCLE_KIT_PREDECESSOR")?,
        drain: walk::knob_scalar("LIFECYCLE_KIT_DRAIN_STAGE")?,
        active_sections: walk::knob_array("LIFECYCLE_KIT_ACTIVE_SECTIONS")?,
        audit_stage: walk::knob_scalar("LIFECYCLE_KIT_AUDIT_STAGE")?,
        audit_entry_stage: walk::knob_scalar("LIFECYCLE_KIT_AUDIT_ENTRY_STAGE")?,
        waiver: walk::knob_scalar("LIFECYCLE_KIT_WAIVER_TOKEN")?,
        roster_basename: walk::knob_scalar("LIFECYCLE_KIT_ROSTER_BASENAME")?,
        amendment_glob: walk::knob_scalar("LIFECYCLE_KIT_AMENDMENT_GLOB")?,
        contract_tokens: walk::knob_array("LIFECYCLE_KIT_CONTRACT_TOKENS")?,
    })
}

// spec: lifecycle-kit/SPEC.md §check-stage-entry — assertion C's signal, or None. templates/
// paths are excluded from both scans (a shipped stub is not a live amendment).
fn audit_signal(k: &Knobs) -> Result<Option<String>, String> {
    let prune = walk::prune_dirs()?;
    let files = walk::find_with_prune(Path::new("."), &|n| prune.iter().any(|d| d == n))?;
    let mut rel: Vec<String> = Vec::new();
    for f in &files {
        let p = f.display().to_string();
        let p = p.strip_prefix("./").unwrap_or(&p).to_string();
        if p.contains("/templates/") || p.starts_with("templates/") {
            continue;
        }
        rel.push(p);
    }
    let base = |p: &str| p.rsplit('/').next().unwrap_or(p).to_string();
    let dir = |p: &str| match p.rfind('/') {
        Some(i) => p[..i].to_string(),
        None => p.to_string(),
    };

    let mut roster: Vec<String> = Vec::new();
    let mut amend_dirs: Vec<String> = Vec::new();
    let mut amend_files: Vec<String> = Vec::new();
    for p in &rel {
        if base(p) == k.roster_basename {
            let d = dir(p);
            if !roster.contains(&d) {
                roster.push(d);
            }
        }
        if walk::pattern_match(&k.amendment_glob, &base(p)) {
            let d = dir(p);
            if !amend_dirs.contains(&d) {
                amend_dirs.push(d);
            }
            amend_files.push(p.clone());
        }
    }
    amend_dirs.sort();

    if amend_dirs.len() >= 2 {
        return Ok(Some(format!(
            "amendments span {} component dirs: {}",
            amend_dirs.len(),
            amend_dirs.join(" ")
        )));
    }

    let mut alt = String::new();
    for t in &k.contract_tokens {
        if !alt.is_empty() {
            alt.push('|');
        }
        alt.push_str(&t.replace('.', "\\."));
    }
    if alt.is_empty() {
        return Ok(None);
    }
    let re = Ere::compile(&format!("[a-z0-9][a-z0-9/_-]*/({})", alt))
        .map_err(|e| format!("cannot compile the contract-token pattern: {}", e))?;

    for af in &amend_files {
        let text = std::fs::read(af)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        let mut comps: Vec<String> = vec![dir(af)];
        for line in text.lines() {
            for tok in all_matches(&re, line) {
                let mut d = tok.to_string();
                for ct in &k.contract_tokens {
                    if let Some(s) = d.strip_suffix(&format!("/{}", ct)) {
                        d = s.to_string();
                    }
                    let bare = ct.trim_end_matches('/');
                    if let Some(s) = d.strip_suffix(&format!("/{}", bare)) {
                        d = s.to_string();
                    }
                }
                if roster.contains(&d) && !comps.contains(&d) {
                    comps.push(d);
                }
            }
        }
        if comps.len() >= 2 {
            comps.sort();
            return Ok(Some(format!(
                "amendment {} references {} components: {}",
                af,
                comps.len(),
                comps.join(" ")
            )));
        }
    }
    Ok(None)
}

pub fn run(args: &[String]) -> i32 {
    let k = match knobs(args) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("check-stage-entry: {}", e);
            return 2;
        }
    };

    for path in [&k.queue, &k.state] {
        if !Path::new(path).is_file() {
            eprintln!("check-stage-entry: file not found: {}", path);
            return 2;
        }
    }

    let qtext = match std::fs::read(&k.queue) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => {
            eprintln!("check-stage-entry: cannot read {}: {}", k.queue, e);
            return 2;
        }
    };
    let Some(hdr) = stages::header(&qtext) else {
        println!("STAGE-ENTRY: no '## Iteration:' header in {}", k.queue);
        println!("  help: add '## Iteration: <name>' to {}", k.queue);
        return 1;
    };

    // spec: lifecycle-kit/SPEC.md §check-stage-entry — the two axes, two surfaces: the header
    // names the iteration, the state file's last stamp is the entered stage. An empty cursor
    // is unreachable by construction here and stays a hard parse error rather than a disarm.
    let iter = stages::header_iter(hdr);
    let stext = match std::fs::read(&k.state) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => {
            eprintln!("check-stage-entry: cannot read {}: {}", k.state, e);
            return 2;
        }
    };
    let stage = stages::current_stage(&stext);
    if iter.is_empty() {
        println!("STAGE-ENTRY: could not parse the iteration from: {}", hdr);
        println!("  help: header must read '## Iteration: <name>'");
        return 1;
    }
    if stage.is_empty() {
        println!(
            "STAGE-ENTRY: could not parse the entered stage — no stamp line in {}",
            k.state
        );
        println!("  help: the entered stage is the last '<iter> <stage> <session-id> <YYYY-MM-DD> <head>' stamp; run the stage skill (it stamps as its first step)");
        return 1;
    }
    if !stages::stage_known(&k.stage_set, &stage) {
        println!(
            "STAGE-ENTRY: cursor stage '{}' is not a lifecycle stage ({})",
            stage,
            k.stage_set.join(" ")
        );
        println!(
            "  help: the last stamp in {} must name one of the configured lifecycle stages",
            k.state
        );
        return 1;
    }
    let pred = walk::knob_in_family(&k.predecessor, &stage).unwrap_or_default();

    let mut errors: Vec<String> = Vec::new();
    let mut c_fired = false;

    // assertion A: the entered stage's mandatory-predecessor stamp exists for this iteration
    if !pred.is_empty() && !stamp_present(&stext, &iter, &pred) {
        errors.push(format!(
            "entering '{}' but no '{} {}' stamp in {} — the mandatory predecessor stage was never invoked (run /{}, or correct the entry stamp)",
            stage, iter, pred, k.state, pred
        ));
    }

    // assertion B: drain-entry queue-empty — [drain-exempt:] exempts at drain entry only; the
    // drain successor's entry backstops with no exemption
    let b_drain = !k.drain.is_empty() && stage == k.drain;
    let b_successor = !k.drain.is_empty()
        && !b_drain
        && k.predecessor
            .iter()
            .any(|(s, p)| *s == stage && *p == k.drain);
    let mut exempt_detail = String::new();
    if b_drain || b_successor {
        let (mut leftover, mut malformed) = (String::new(), String::new());
        for (ln, text) in active_bullets(&qtext, &k.active_sections) {
            match drain_exempt_reason(text) {
                Some("") => malformed.push_str(&format!("\n    {}: {}", ln, text)),
                Some(reason) if b_drain => {
                    if !exempt_detail.is_empty() {
                        exempt_detail.push_str("; ");
                    }
                    exempt_detail.push_str(&format!("{}: {}", ln, reason));
                }
                _ => leftover.push_str(&format!("\n    {}: {}", ln, text)),
            }
        }
        if !leftover.is_empty() {
            if b_drain {
                errors.push(format!(
                    "entering '{}' but the active queue is non-empty (the prior stage is not drained):{}",
                    stage, leftover
                ));
            } else {
                errors.push(format!(
                    "entering '{}' (the drain successor) but the active queue is non-empty — nothing may remain active past '{}', [drain-exempt:] included:{}",
                    stage, k.drain, leftover
                ));
            }
        }
        if !malformed.is_empty() {
            errors.push(format!(
                "[drain-exempt:] with an empty reason is malformed (the reason is the audit trail):{}",
                malformed
            ));
        }
    }

    // assertion C: audit-entry with a cross-component amendment signal and no audit stamp demands
    // that stamp or a recorded waiver (lifecycle-kit/SPEC.md §check-stage-entry)
    if !k.audit_stage.is_empty()
        && stage == k.audit_entry_stage
        && !stamp_present(&stext, &iter, &k.audit_stage)
    {
        match audit_signal(&k) {
            Err(e) => {
                eprintln!(
                    "check-stage-entry: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
            Ok(Some(detail)) => {
                if !stamp_present(&stext, &iter, &k.waiver) {
                    c_fired = true;
                    errors.push(format!(
                        "entering '{}' with a cross-component amendment signal and no '{} {}' stamp ({}) — the audit trigger (≥2 components' contracts) was not verified for this iteration",
                        stage, iter, k.audit_stage, detail
                    ));
                }
            }
            Ok(None) => {}
        }
    }

    if !errors.is_empty() {
        println!(
            "STAGE-ENTRY: {} prior-stage readiness issue(s) entering '{}' of '{}':",
            errors.len(),
            stage,
            iter
        );
        for e in &errors {
            println!("  {}", e);
        }
        if c_fired {
            println!("  help: a cross-component {} entry must run /{} (stamps '{} {} <session> <date> <head>'), or — on an explicit user ruling, never self-issued by the entering session — record a deliberate waiver line '{} {} <session> <date> <head>' in {}", k.audit_entry_stage, k.audit_stage, iter, k.audit_stage, iter, k.waiver, k.state);
        } else {
            println!("  help: a stage entry re-verifies the prior stage's static exit — invoke the predecessor skill (it stamps {}) and drain the active queue before entering {}", k.state, k.drain);
        }
        return 1;
    }

    let detail = if pred.is_empty() {
        format!("'{}' has no mandatory predecessor", stage)
    } else if b_drain {
        let mut d = format!("predecessor '{}' stamped; active queue drained", pred);
        if !exempt_detail.is_empty() {
            d.push_str(&format!(" (drain-exempt residue — {})", exempt_detail));
        }
        d
    } else if b_successor {
        format!("predecessor '{}' stamped; active queue empty (drain-successor backstop, no exemption)", pred)
    } else {
        format!("predecessor '{}' stamped", pred)
    };
    println!("STAGE-ENTRY: clean ('{}' / '{}' — {})", iter, stage, detail);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §check-stage-entry — the section window is the awk machine's:
    // an active heading opens it, any other `## ` closes it, and only a `- ` bullet counts
    #[test]
    fn the_active_window_opens_on_a_section_and_closes_on_any_other_heading() {
        let text = "## New Features\n- a\n## Done\n- b\n## Technical Debt  \n- c\nnot a bullet\n";
        let secs = vec!["New Features".to_string(), "Technical Debt".to_string()];
        let got: Vec<&str> = active_bullets(text, &secs).iter().map(|(_, l)| *l).collect();
        assert_eq!(got, vec!["- a", "- c"]);
    }

    // spec: lifecycle-kit/SPEC.md §check-stage-entry — an empty reason is malformed rather than
    // absent, and an unterminated tag is untagged residue rather than an exemption
    #[test]
    fn a_drain_exempt_tag_reports_its_reason_and_distinguishes_empty_from_absent() {
        assert_eq!(drain_exempt_reason("- x [drain-exempt: why]"), Some("why"));
        assert_eq!(drain_exempt_reason("- x [drain-exempt: ]"), Some(""));
        assert_eq!(drain_exempt_reason("- x [drain-exempt:]"), Some(""));
        assert_eq!(drain_exempt_reason("- x"), None);
        assert_eq!(drain_exempt_reason("- x [drain-exempt: unterminated"), None);
    }

    // spec: lifecycle-kit/SPEC.md §check-stage-entry — `grep -oE` takes every non-overlapping
    // match on the line, not the first: a body naming two components must yield both
    #[test]
    fn every_contract_token_on_a_line_is_taken_not_just_the_first() {
        let re = Ere::compile("[a-z0-9][a-z0-9/_-]*/(SPEC\\.md|proto/)").expect("compiles");
        assert_eq!(
            all_matches(&re, "folds panel-facade/SPEC.md into widget-service/SPEC.md"),
            vec!["panel-facade/SPEC.md", "widget-service/SPEC.md"]
        );
        assert!(all_matches(&re, "no token here").is_empty());
    }
}
