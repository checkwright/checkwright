// spec: lifecycle-kit/SPEC.md §check-gap-inbox-neutrality — the capture surface records
// observations: every bullet is '- <YYYY-MM-DD> — <prose>', and no bullet's prose opens with
// the retired 'recurrence of `<slug>`:' verdict
use crate::walk;
use std::path::Path;

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\u{b}' || c == '\u{c}'
}

// spec: lifecycle-kit/SPEC.md §check-gap-inbox-neutrality — the one legal bullet shape,
// `- <YYYY-MM-DD> — <prose>` with non-empty prose; the date is matched digit-wise as the
// shell form's awk class does rather than parsed as a calendar date
fn well_formed(line: &str) -> bool {
    let c: Vec<char> = line.chars().collect();
    if c.len() < 3 || c[0] != '-' || !is_space(c[1]) {
        return false;
    }
    let d = &c[2..];
    let shape = "dddd-dd-dd";
    if d.len() < shape.len() {
        return false;
    }
    for (k, s) in shape.chars().enumerate() {
        let ok = if s == 'd' {
            d[k].is_ascii_digit()
        } else {
            d[k] == '-'
        };
        if !ok {
            return false;
        }
    }
    let mut i = shape.len();
    if i >= d.len() || !is_space(d[i]) {
        return false;
    }
    i += 1;
    if i >= d.len() || d[i] != '—' {
        return false;
    }
    i += 1;
    while i < d.len() && is_space(d[i]) {
        i += 1;
    }
    i < d.len()
}

// spec: lifecycle-kit/SPEC.md §check-gap-inbox-neutrality — the prose is what survives the
// `-[[:space:]][0-9-]+[[:space:]]—[[:space:]]*` lead the shell form strips
fn prose_of(line: &str) -> String {
    let c: Vec<char> = line.chars().collect();
    let mut i = 2usize;
    while i < c.len() && (c[i].is_ascii_digit() || c[i] == '-') {
        i += 1;
    }
    i += 1;
    i += 1;
    while i < c.len() && is_space(c[i]) {
        i += 1;
    }
    c[i.min(c.len())..].iter().collect()
}

// spec: lifecycle-kit/SPEC.md §check-gap-inbox-neutrality — the retired verdict opener,
// "recurrence of `<slug>`:" with a queue slug between the backticks
fn opens_with_recurrence_verdict(prose: &str) -> bool {
    let Some(rest) = prose.strip_prefix("recurrence of `") else {
        return false;
    };
    let b = rest.as_bytes();
    if b.is_empty() || !(b[0].is_ascii_lowercase() || b[0].is_ascii_digit()) {
        return false;
    }
    let mut i = 0usize;
    while i < b.len() && (b[i].is_ascii_lowercase() || b[i].is_ascii_digit() || b[i] == b'-') {
        i += 1;
    }
    rest[i..].starts_with("`:")
}

pub fn run(args: &[String]) -> i32 {
    let hermetic = args.first().filter(|a| !a.is_empty());
    let inbox = match hermetic {
        Some(p) => {
            if !Path::new(p.as_str()).is_file() {
                eprintln!("check-gap-inbox-neutrality: inbox file not found: {}", p);
                return 2;
            }
            p.clone()
        }
        None => {
            let p = match walk::knob_scalar("LIFECYCLE_KIT_GAP_INBOX_FILE") {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-gap-inbox-neutrality: {}", e);
                    return 2;
                }
            };
            // spec: lifecycle-kit/SPEC.md §check-gap-inbox-neutrality — an absent inbox is
            // clean, not fail-closed: never having filed a gap is a legal state
            if !Path::new(&p).is_file() {
                println!(
                    "GAP-INBOX-NEUTRALITY: clean (no inbox at {} — no gap filed this iteration)",
                    p
                );
                return 0;
            }
            p
        }
    };
    let text = match std::fs::read(&inbox) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => {
            eprintln!(
                "check-gap-inbox-neutrality: inbox file not readable: {}",
                inbox
            );
            return 2;
        }
    };

    let mut findings: Vec<String> = Vec::new();
    let mut bullets = 0usize;
    for (idx, line) in text.lines().enumerate() {
        let fnr = idx + 1;
        if fnr == 1 && line.starts_with('#') && line[1..].starts_with(is_space) {
            continue;
        }
        if line.chars().all(is_space) {
            continue;
        }
        bullets += 1;
        if !well_formed(line) {
            findings.push(format!(
                "{}:{}: not a gap bullet — the grammar is '- <YYYY-MM-DD> — <prose>' with non-empty prose",
                inbox, fnr
            ));
            continue;
        }
        if opens_with_recurrence_verdict(&prose_of(line)) {
            findings.push(format!(
                "{}:{}: the prose opens with a retired 'recurrence of `<slug>`:' verdict — the capture surface carries observations, not conclusions",
                inbox, fnr
            ));
        }
    }

    if !findings.is_empty() {
        println!(
            "check-gap-inbox-neutrality: {} malformed or verdict-bearing bullet(s) in {}:",
            findings.len(),
            inbox
        );
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: the gap inbox records what a filer observed, and the closing stage's drain is what judges it. File with 'bash gate-sdk/bin/run-gates.sh --emit file-gap \"<gap prose>\"', which stamps the one legal bullet shape. A bullet that re-files a live entry says so in its own prose — write why you believe it re-occurred and let the drain rule on it; never open the prose with a 'recurrence of <slug>:' verdict, which states a conclusion the capture channel has no standing to reach.");
        return 1;
    }
    println!(
        "GAP-INBOX-NEUTRALITY: clean ({} bullet(s) in {}; every bullet is dated prose and none opens with a recurrence verdict)",
        bullets, inbox
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bullet_grammar_admits_only_a_dated_line_with_prose() {
        assert!(well_formed("- 2026-08-13 — something"));
        assert!(!well_formed("- 2026-08-13 —"));
        assert!(!well_formed("- 2026-8-13 — something"));
        assert!(!well_formed("-2026-08-13 — something"));
        assert!(!well_formed("- 2026-08-13 - something"));
    }

    #[test]
    fn the_retired_verdict_opener_is_recognised_only_in_its_exact_shape() {
        assert_eq!(prose_of("- 2026-08-13 — the prose"), "the prose");
        assert!(opens_with_recurrence_verdict("recurrence of `some-slug`: x"));
        assert!(!opens_with_recurrence_verdict("recurrence of `some-slug` is likely"));
        assert!(!opens_with_recurrence_verdict("a recurrence of `x`:"));
    }
}
