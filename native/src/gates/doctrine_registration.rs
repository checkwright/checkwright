// spec: doctrine-kit/SPEC.md §check-doctrine-registration — the always-loaded agent file links
// the doctrine file and holds its methodology-rule digest in per-rule lockstep with DOCTRINE.md
use crate::section;
use crate::walk;

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — the doctrine-side section headings are
// kit mechanism (the kit ships DOCTRINE.md), never config
const METH_SECTION: &str = "## Methodology-maintenance rules";
const CRAFT_SECTION: &str = "## Engineering-craft rules";

fn spacey(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\u{b}' | '\u{c}' | '\r')
}

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — a numbered rule lead-in, returning the
// offset past its bold opener so the name reader and the item bounder share one grammar
fn rule_lead(line: &str) -> Option<usize> {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 || b.get(i) != Some(&b'.') {
        return None;
    }
    i += 1;
    let ws = i;
    while i < b.len() && spacey(b[i] as char) {
        i += 1;
    }
    if i == ws || !line[i..].starts_with("**") {
        return None;
    }
    Some(i + 2)
}

fn bold_run(rest: &str) -> &str {
    match rest.find("**") {
        Some(at) => &rest[..at],
        None => rest,
    }
}

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — the rule's name is its bold lead-in
// with an in-bold trailing period dropped, the spelling the digest bullet is matched against
fn rule_name(line: &str) -> String {
    let Some(at) = rule_lead(line) else {
        return String::new();
    };
    let name = bold_run(&line[at..]);
    name.strip_suffix('.').unwrap_or(name).to_string()
}

fn digest_name(line: &str) -> String {
    bold_run(line.strip_prefix("- **").unwrap_or(line)).to_string()
}

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — a declared trim names its rule between
// the marker and the em dash that opens its reason
fn trim_name(line: &str) -> String {
    const MARKER: &str = "doctrine-digest-trim:";
    let Some(at) = line.rfind(MARKER) else {
        return String::new();
    };
    let rest = line[at + MARKER.len()..].trim_start_matches(spacey);
    match rest.find('—') {
        Some(k) => rest[..k].trim_end_matches(spacey).to_string(),
        None => rest.to_string(),
    }
}

fn trailer_value<'a>(line: &'a str, trailer: &str) -> Option<&'a str> {
    let head = format!("*{}:*", trailer);
    let rest = line.trim_start_matches(spacey).strip_prefix(head.as_str())?;
    Some(rest.trim_start_matches(spacey).trim_end_matches(spacey))
}

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — assertion D's grammar: a comma list of
// lowercase stage names, or the em dash standing for none
fn stages_well_formed(val: &str) -> bool {
    if val == "—" {
        return true;
    }
    let lowercase = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase());
    let mut parts = val.split(',');
    if !lowercase(parts.next().unwrap_or("")) {
        return false;
    }
    for p in parts {
        let t = p.trim_start_matches(spacey);
        if t.len() == p.len() || !lowercase(t) {
            return false;
        }
    }
    true
}

struct Trailered {
    name: String,
    count: usize,
    bad: bool,
}

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — assertions D and E share one walk over
// a register's numbered rules, counting a named per-rule trailer and grading its value
fn trailer_walk(lines: &[&str], sect: &str, trailer: &str, stages: bool) -> Option<Vec<Trailered>> {
    let found = section::sections(lines, sect);
    if found.is_empty() {
        return None;
    }
    let mut out: Vec<Trailered> = Vec::new();
    for sec in &found {
        let body = &lines[sec.start..sec.end];
        for item in section::items(body, |l| rule_lead(l).is_some()) {
            let mut rec = Trailered {
                name: rule_name(body[item.start]),
                count: 0,
                bad: false,
            };
            for line in &body[item.start..item.end] {
                if let Some(val) = trailer_value(line, trailer) {
                    rec.count += 1;
                    if stages {
                        if !stages_well_formed(val) {
                            rec.bad = true;
                        }
                    } else if val.is_empty() {
                        rec.bad = true;
                    }
                }
            }
            out.push(rec);
        }
    }
    Some(out)
}

fn rule_names(lines: &[&str], sect: &str) -> Option<Vec<String>> {
    let found = section::sections(lines, sect);
    if found.is_empty() {
        return None;
    }
    let mut out: Vec<String> = Vec::new();
    for sec in &found {
        for line in &lines[sec.start..sec.end] {
            if rule_lead(line).is_some() {
                out.push(rule_name(line));
            }
        }
    }
    Some(out)
}

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — the digest side: bullet names and
// declared trims off one walk, a bullet carrying the marker reading as a bullet
fn digest_entries(lines: &[&str], sect: &str) -> Option<(Vec<String>, Vec<String>)> {
    let found = section::sections(lines, sect);
    if found.is_empty() {
        return None;
    }
    let (mut bullets, mut trims) = (Vec::new(), Vec::new());
    for sec in &found {
        for line in &lines[sec.start..sec.end] {
            if line.starts_with("- **") {
                bullets.push(digest_name(line));
            } else if line.contains("doctrine-digest-trim:") {
                trims.push(trim_name(line));
            }
        }
    }
    Some((bullets, trims))
}

fn read(path: &str) -> Result<String, ()> {
    std::fs::read_to_string(path).map_err(|_| ())
}

pub fn run(args: &[String]) -> i32 {
    let arg_or_knob = |i: usize, knob: &str| match args.get(i) {
        Some(a) => Ok(a.clone()),
        None => walk::knob_scalar(knob),
    };
    let agent_file = match arg_or_knob(0, "DOCTRINE_KIT_AGENT_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-doctrine-registration: {}", e);
            return 2;
        }
    };
    let doctrine_file = match arg_or_knob(1, "DOCTRINE_KIT_DOCTRINE_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-doctrine-registration: {}", e);
            return 2;
        }
    };
    let digest_section = match walk::knob_scalar("DOCTRINE_KIT_DIGEST_SECTION") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-doctrine-registration: {}", e);
            return 2;
        }
    };

    let Ok(agent_text) = read(&agent_file) else {
        eprintln!(
            "check-doctrine-registration: agent file not found: {}",
            agent_file
        );
        return 2;
    };

    if !agent_text.contains(&format!("]({}", doctrine_file)) {
        println!(
            "check-doctrine-registration: {} carries no markdown link to the doctrine file:",
            agent_file
        );
        println!("  {}", doctrine_file);
        println!("  help: install the doctrine reference block into the always-loaded agent file —");
        println!("        bash doctrine-kit/bin/install-doctrine.sh — so a session that loads it");
        println!("        follows the link to the delivery doctrine. Override the paths with");
        println!("        DOCTRINE_KIT_AGENT_FILE / DOCTRINE_KIT_DOCTRINE_FILE.");
        return 1;
    }

    let Ok(doctrine_text) = read(&doctrine_file) else {
        eprintln!(
            "check-doctrine-registration: doctrine file not found: {}",
            doctrine_file
        );
        return 2;
    };

    let doctrine_lines = section::split_lines(&doctrine_text);
    let agent_lines = section::split_lines(&agent_text);

    let Some(doctrine_rules) = rule_names(&doctrine_lines, METH_SECTION) else {
        eprintln!(
            "check-doctrine-registration: no '{}' section in {} — cannot certify the digest \
             against an unreadable rule set",
            METH_SECTION, doctrine_file
        );
        eprintln!(
            "  help: the methodology-maintenance section heading is kit mechanism; restore it \
             in the doctrine file"
        );
        return 2;
    };

    let Some((digest_names, trims)) = digest_entries(&agent_lines, &digest_section) else {
        eprintln!(
            "check-doctrine-registration: no heading matches DOCTRINE_KIT_DIGEST_SECTION in {}: '{}'",
            agent_file, digest_section
        );
        eprintln!(
            "  help: a renamed or deleted digest section silently disarms this gate — repoint \
             DOCTRINE_KIT_DIGEST_SECTION at the live heading, or restore the heading it names"
        );
        return 2;
    };

    let Some(craft) = trailer_walk(&doctrine_lines, CRAFT_SECTION, "Stages", true) else {
        eprintln!(
            "check-doctrine-registration: no '{}' section in {} — cannot certify the \
             stage-routing trailers against an unreadable rule set",
            CRAFT_SECTION, doctrine_file
        );
        eprintln!(
            "  help: the engineering-craft section heading is kit mechanism; restore it in the \
             doctrine file"
        );
        return 2;
    };

    let mut craft_findings: Vec<String> = Vec::new();
    for r in &craft {
        if r.count != 1 {
            craft_findings.push(format!(
                "craft rule carries {} *Stages:* trailer(s), want exactly one: {}",
                r.count, r.name
            ));
        } else if r.bad {
            craft_findings.push(format!(
                "craft rule's *Stages:* value is malformed (want a comma list of lowercase \
                 stages, or —): {}",
                r.name
            ));
        }
    }

    let mut digest_findings: Vec<String> = Vec::new();
    for r in trailer_walk(&doctrine_lines, METH_SECTION, "Digest", false).unwrap_or_default() {
        if r.count != 1 {
            digest_findings.push(format!(
                "methodology rule carries {} *Digest:* trailer(s), want exactly one: {}",
                r.count, r.name
            ));
        } else if r.bad {
            digest_findings.push(format!(
                "methodology rule's *Digest:* value is empty: {}",
                r.name
            ));
        }
    }

    let named: Vec<&String> = digest_names.iter().filter(|n| !n.is_empty()).collect();
    let trimmed: Vec<&String> = trims.iter().filter(|n| !n.is_empty()).collect();
    let mut rule_count = 0usize;
    let mut in_doctrine: Vec<&String> = Vec::new();
    let mut missing: Vec<&String> = Vec::new();
    for name in &doctrine_rules {
        if name.is_empty() {
            continue;
        }
        rule_count += 1;
        in_doctrine.push(name);
        if !named.contains(&name) && !trimmed.contains(&name) {
            missing.push(name);
        }
    }
    let orphans: Vec<&&String> = named.iter().filter(|n| !in_doctrine.contains(n)).collect();

    if missing.is_empty()
        && orphans.is_empty()
        && craft_findings.is_empty()
        && digest_findings.is_empty()
    {
        println!(
            "DOCTRINE-REGISTRATION: clean ({} links {}; {} methodology rule(s) in per-rule \
             digest lockstep, {} declared trim(s), each carrying one *Digest:* trailer; {} \
             craft rule(s) each carry one *Stages:* trailer)",
            agent_file,
            doctrine_file,
            rule_count,
            trimmed.len(),
            craft.len()
        );
        return 0;
    }

    if !missing.is_empty() || !orphans.is_empty() {
        println!(
            "check-doctrine-registration: the digest and the doctrine are out of lockstep in {}:",
            agent_file
        );
        for name in &missing {
            println!("  doctrine rule absent from the digest: {}", name);
        }
        for name in &orphans {
            println!("  digest bullet owns no doctrine rule: {}", name);
        }
        println!("  help: a re-vendored DOCTRINE.md changed the methodology-rule set — reconcile the");
        println!(
            "        '{}' digest in {}: add '- **<name>** — …' for",
            digest_section, agent_file
        );
        println!("        each absent rule (or declare a trim '<!-- doctrine-digest-trim: <name> — <reason> -->'),");
        println!("        and rename or remove any bullet that owns no rule.");
    }
    if !craft_findings.is_empty() {
        println!(
            "check-doctrine-registration: the stage-routing trailers are out of grammar in {}:",
            doctrine_file
        );
        for f in &craft_findings {
            println!("  {}", f);
        }
        println!("  help: every engineering-craft rule owns exactly one '*Stages:* <stage>[, <stage>…]'");
        println!("        trailer ('*Stages:* —' for none) so a re-vendored DOCTRINE.md that adds an");
        println!("        untagged craft rule reddens instead of silently dropping out of stage routing.");
    }
    if !digest_findings.is_empty() {
        println!(
            "check-doctrine-registration: the digest trailers are out of grammar in {}:",
            doctrine_file
        );
        for f in &digest_findings {
            println!("  {}", f);
        }
        println!("  help: every methodology rule owns exactly one non-empty '*Digest:* <one-line summary>'");
        println!("        trailer — install-doctrine.sh derives that rule's digest bullet from it, so an");
        println!("        untrailered rule would ship every consumer a digest one rule short.");
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rule_lead_needs_a_number_a_dot_whitespace_and_a_bold_opener() {
        assert_eq!(rule_name("1. **Content-tiering.** body"), "Content-tiering");
        assert_eq!(rule_name("12.  **Oracle-first** — body"), "Oracle-first");
        assert!(rule_lead("1.**Name**").is_none());
        assert!(rule_lead("1 **Name**").is_none());
        assert!(rule_lead("- **Name**").is_none());
    }

    // spec: doctrine-kit/SPEC.md §check-doctrine-registration — the trim's name stops at the em
    // dash that opens its reason, and the marker is read at its last occurrence on the line
    #[test]
    fn a_trim_names_its_rule_up_to_the_em_dash() {
        assert_eq!(
            trim_name("<!-- doctrine-digest-trim: Some Rule — not applicable here -->"),
            "Some Rule"
        );
        assert_eq!(trim_name("<!-- doctrine-digest-trim: Bare -->"), "Bare -->");
        assert_eq!(trim_name("nothing here"), "");
    }

    #[test]
    fn the_trailer_value_is_the_line_past_its_marker_trimmed_both_ends() {
        assert_eq!(trailer_value("  *Stages:*  build, close  ", "Stages"), Some("build, close"));
        assert_eq!(trailer_value("*Digest:*", "Digest"), Some(""));
        assert_eq!(trailer_value("*Stages:* x", "Digest"), None);
    }

    #[test]
    fn the_stages_grammar_takes_a_lowercase_comma_list_or_the_em_dash() {
        assert!(stages_well_formed("—"));
        assert!(stages_well_formed("build"));
        assert!(stages_well_formed("scope, build, close"));
        assert!(!stages_well_formed("Build"));
        assert!(!stages_well_formed("build,close"));
        assert!(!stages_well_formed(""));
        assert!(!stages_well_formed("build,"));
    }

    // spec: doctrine-kit/SPEC.md §check-doctrine-registration — a rule's trailers are the lines
    // between its lead and the next rule's, which is what makes a doubled trailer countable
    #[test]
    fn each_rule_owns_the_trailers_between_its_lead_and_the_next() {
        let text = "## Engineering-craft rules\n1. **A** x\n*Stages:* build\n2. **B** y\n\
                    *Stages:* build\n*Stages:* close\n## Next\n*Stages:* scope\n";
        let lines = section::split_lines(text);
        let got = trailer_walk(&lines, CRAFT_SECTION, "Stages", true).expect("section absent");
        assert_eq!(got.len(), 2);
        assert_eq!((got[0].name.as_str(), got[0].count), ("A", 1));
        assert_eq!((got[1].name.as_str(), got[1].count), ("B", 2));
    }

    #[test]
    fn an_absent_register_is_reported_as_absent_rather_than_as_empty() {
        let lines = section::split_lines("## Other\n1. **A**\n");
        assert!(trailer_walk(&lines, CRAFT_SECTION, "Stages", true).is_none());
        assert!(rule_names(&lines, METH_SECTION).is_none());
        assert!(digest_entries(&lines, "## Delivery doctrine").is_none());
    }

    // spec: doctrine-kit/SPEC.md §check-doctrine-registration — a bullet carrying the trim marker
    // is a bullet, because the digest walk tests the bullet lead-in first
    #[test]
    fn a_bullet_carrying_the_trim_marker_reads_as_a_bullet() {
        let text = "## D\n- **Kept** — x\n<!-- doctrine-digest-trim: Dropped — why -->\n";
        let lines = section::split_lines(text);
        let (bullets, trims) = digest_entries(&lines, "## D").expect("section absent");
        assert_eq!(bullets, vec!["Kept".to_string()]);
        assert_eq!(trims, vec!["Dropped".to_string()]);
    }
}
