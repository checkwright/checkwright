// spec: doctrine-kit/SPEC.md §install-doctrine — idempotent insert/replace of the doctrine
// reference block between fixed markers in the always-loaded agent file, carrying the consumer's
// declared trims across the rewrite. Here: the digest and the preservation rule, nothing else.
use crate::marker;
use crate::walk;

const BEGIN: &str = "<!-- doctrine-kit:begin -->";
const END: &str = "<!-- doctrine-kit:end -->";

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — the doctrine-side section heading is
// kit mechanism (the kit ships DOCTRINE.md), never config.
const METH_SECTION: &str = "## Methodology-maintenance rules";

fn hlevel(line: &str) -> usize {
    let n = line.chars().take_while(|c| *c == '#').count();
    if n == 0 {
        return 0;
    }
    match line.chars().nth(n) {
        Some(c) if c.is_whitespace() => n,
        _ => 0,
    }
}

// spec: doctrine-kit/SPEC.md §install-doctrine — a rule opens on `<n>. **`, read exactly as
// check-doctrine-registration assertion C reads it: the bold lead-in, its trailing period dropped.
fn rule_name(line: &str) -> Option<String> {
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    let rest = &line[digits..];
    let rest = rest.strip_prefix('.')?;
    let spaces = rest.chars().take_while(|c| c.is_whitespace()).count();
    if spaces == 0 {
        return None;
    }
    let rest = rest[spaces..].strip_prefix("**")?;
    let name = match rest.find("**") {
        Some(i) => &rest[..i],
        None => rest,
    };
    Some(name.trim_end_matches('.').to_string())
}

// spec: doctrine-kit/SPEC.md §install-doctrine — a rule's summary is read from its own `*Digest:*`
// trailer exactly as check-doctrine-registration assertion E counts it.
fn digest_trailer(line: &str) -> Option<String> {
    let body = line.trim_start().strip_prefix("*Digest:*")?;
    Some(body.trim().to_string())
}

struct Rule {
    name: String,
    trailers: usize,
    summary: String,
}

// spec: doctrine-kit/SPEC.md §install-doctrine — the rule walk, derived once rather than inside the
// digest renderer: a malformed doctrine must refuse the whole run, never leak through as a silently
// short digest. `None` is *no such section*, which the caller reports on its own terms.
fn walk_rules(text: &str) -> Option<Vec<Rule>> {
    let mut seen = false;
    let mut insec = false;
    let mut start_lvl = 0;
    let mut out: Vec<Rule> = Vec::new();
    for line in text.lines() {
        let lvl = hlevel(line);
        if !insec {
            if lvl > 0 && line.starts_with(METH_SECTION) {
                insec = true;
                seen = true;
                start_lvl = lvl;
            }
            continue;
        }
        if lvl > 0 && lvl <= start_lvl {
            insec = false;
            continue;
        }
        if let Some(name) = rule_name(line) {
            out.push(Rule {
                name,
                trailers: 0,
                summary: String::new(),
            });
        } else if let Some(value) = digest_trailer(line) {
            if let Some(cur) = out.last_mut() {
                cur.trailers += 1;
                if cur.trailers == 1 {
                    cur.summary = value;
                }
            }
        }
    }
    if !seen {
        return None;
    }
    Some(out)
}

// spec: doctrine-kit/SPEC.md §install-doctrine — the untrimmed digest: the always-loaded shape
// applied to the doctrine itself, a one-line-per-rule digest plus the markdown link to the file.
fn digest(doctrine_file: &str, rules: &[Rule]) -> Vec<String> {
    let mut out = vec![
        "## Delivery doctrine".to_string(),
        String::new(),
        format!(
            "The cross-kit delivery rules live in [{}]({}) — re-vendor",
            doctrine_file, doctrine_file
        ),
        "to upgrade. The always-loaded maintenance rules, one line each; the doctrine adds"
            .to_string(),
        "an engineering-craft section behind the link:".to_string(),
        String::new(),
    ];
    for r in rules {
        out.push(format!("- **{}** — {}", r.name, r.summary));
    }
    out
}

// spec: doctrine-kit/SPEC.md §install-doctrine — a digest bullet's rule name, extracted exactly as
// check-doctrine-registration extracts it (assertion C), so the substitution keys on the same
// string the gate keys on.
fn bullet_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("- **")?;
    let close = rest.find("**")?;
    Some(&rest[..close])
}

// spec: doctrine-kit/SPEC.md §install-doctrine — the declared-trim grammar read exactly as
// check-doctrine-registration reads it (assertion B), so installer and gate never disagree about
// which rule a marker names.
fn trim_name(line: &str) -> Option<String> {
    let i = line.find("doctrine-digest-trim:")?;
    let rest = &line[i + "doctrine-digest-trim:".len()..];
    let rest = rest.trim_start();
    let name = match rest.find('—') {
        Some(j) => &rest[..j],
        None => rest,
    };
    Some(name.trim_end().to_string())
}

pub struct Report {
    pub line: String,
    pub findings: Vec<String>,
}

pub fn install(agent_file: &str, doctrine_file: &str) -> Result<Report, String> {
    if !std::path::Path::new(agent_file).is_file() {
        return Err(format!(
            "agent file not found: {} — nothing to install into",
            agent_file
        ));
    }
    if !std::path::Path::new(doctrine_file).is_file() {
        return Err(format!(
            "doctrine file not found: {} — nothing to derive the digest from",
            doctrine_file
        ));
    }
    let text = crate::fresh::read_captured(doctrine_file)?;
    let rules = walk_rules(&text).ok_or_else(|| {
        format!(
            "no '{}' section in {} — cannot derive the digest from an unreadable rule set",
            METH_SECTION, doctrine_file
        )
    })?;
    for r in &rules {
        if r.trailers != 1 {
            return Err(format!(
                "methodology rule '{}' carries {} *Digest:* trailer(s) in {}, want exactly one — \
                 refusing to emit a digest missing or double-sourcing its bullet",
                r.name, r.trailers, doctrine_file
            ));
        }
    }
    if rules.is_empty() {
        return Err(format!(
            "no methodology rules found under '{}' in {} — refusing to install an empty digest",
            METH_SECTION, doctrine_file
        ));
    }

    // spec: doctrine-kit/SPEC.md §install-doctrine — the round-trip's read half: the block as it
    // stands is the only record of what the consumer declared, so it is harvested before the
    // rewrite that would erase it.
    let agent_text = crate::fresh::read_captured(agent_file)?;
    let current = marker::read_block(&agent_text, BEGIN, END);
    let mut order: Vec<String> = Vec::new();
    let mut lines: Vec<(String, String)> = Vec::new();
    let mut dupes: Vec<String> = Vec::new();
    for line in current.lines() {
        let Some(name) = trim_name(line) else { continue };
        if name.is_empty() {
            continue;
        }
        if order.contains(&name) {
            dupes.push(name);
            continue;
        }
        lines.push((name.clone(), line.to_string()));
        order.push(name);
    }
    let trim_of = |name: &str| {
        lines
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, l)| l.clone())
    };

    let body_lines = digest(doctrine_file, &rules);
    let orphans: Vec<String> = order
        .iter()
        .filter(|n| !rules.iter().any(|r| r.name == **n))
        .cloned()
        .collect();

    // spec: doctrine-kit/SPEC.md §install-doctrine — the emit: a trimmed rule's marker replaces its
    // bullet *in place*, because a block carrying both would satisfy the gate while handing back
    // the rule the consumer removed.
    let mut block: Vec<String> = Vec::new();
    for line in &body_lines {
        match bullet_name(line).and_then(trim_of) {
            Some(marker_line) => block.push(marker_line),
            None => block.push(line.clone()),
        }
    }
    // spec: doctrine-kit/SPEC.md §install-doctrine — a trim naming no live rule has no bullet
    // position to take, so it is carried at the digest's end rather than dropped.
    for name in &orphans {
        if let Some(l) = trim_of(name.as_str()) {
            block.push(l);
        }
    }

    let action = marker::install_block(agent_file, BEGIN, END, &format!("{}\n", block.join("\n")))?;

    // spec: doctrine-kit/SPEC.md §install-doctrine — findings go to stderr, the one channel the
    // install path does not discard, so a reconciliation the consumer owes is never silent.
    let mut findings: Vec<String> = Vec::new();
    for name in &orphans {
        findings.push(format!(
            "declared trim names '{}', which no rule in the current doctrine digest matches — \
             carried forward unchanged; adopt the renamed rule or drop the marker",
            name
        ));
    }
    for name in &dupes {
        findings.push(format!(
            "duplicate declared trim for '{}' — the first is carried, the duplicate dropped",
            name
        ));
    }

    let mut tail = String::new();
    if !orphans.is_empty() {
        tail.push_str(&format!(", {} unmatched", orphans.len()));
    }
    if !dupes.is_empty() {
        tail.push_str(&format!(", {} duplicate", dupes.len()));
    }
    Ok(Report {
        line: format!(
            "{} the doctrine reference block in {} (link → {}); {} declared trim(s) carried{}",
            action,
            agent_file,
            doctrine_file,
            order.len(),
            tail
        ),
        findings,
    })
}

// spec: doctrine-kit/SPEC.md §install-doctrine — `--remove` is the insert path reversed over the
// same marker pair. It harvests no trims and derives no digest — a removal has nothing to carry
// forward — and reports what it did on the same channel the insert path reports on.
pub fn remove(agent_file: &str) -> Result<String, String> {
    if !std::path::Path::new(agent_file).is_file() {
        return Err(format!(
            "agent file not found: {} — nothing to remove",
            agent_file
        ));
    }
    if marker::remove_block(agent_file, BEGIN, END)? {
        Ok(format!(
            "removed the doctrine reference block from {}",
            agent_file
        ))
    } else {
        Ok(format!(
            "no doctrine reference block found in {} — nothing to remove",
            agent_file
        ))
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — a bridged `Arm::Run` on `--install-lifecycle`'s
// precedent: the member mutates a file and emits no document, and it resolves two knobs a
// hardcoded flag would ignore.
pub const KNOBS: &[&str] = &["DOCTRINE_KIT_AGENT_FILE", "DOCTRINE_KIT_DOCTRINE_FILE"];

pub fn run(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(report) => {
            for f in &report.findings {
                eprintln!("install-doctrine: {}", f);
            }
            println!("install-doctrine: {}", report.line);
            0
        }
        Err(e) => {
            eprintln!("install-doctrine: {}", e);
            2
        }
    }
}

// spec: doctrine-kit/SPEC.md §install-doctrine — the positional overrides
// `[agent-file [doctrine-file]]` and `--remove [agent-file]`; each names the file the rule writes
// into rather than selecting where configuration comes from, so each arrives as argv.
fn dispatch(args: &[String]) -> Result<Report, String> {
    let removing = args.first().map(String::as_str) == Some("--remove");
    let rest: &[String] = if removing { &args[1..] } else { args };
    if let Some(a) = rest.first().filter(|a| a.starts_with('-')) {
        return Err(format!("unknown option: {}", a));
    }
    let agent = match rest.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => walk::knob_scalar("DOCTRINE_KIT_AGENT_FILE")?,
    };
    if removing {
        return remove(&agent).map(|line| Report {
            line,
            findings: Vec::new(),
        });
    }
    let doctrine = match rest.get(1).filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => walk::knob_scalar("DOCTRINE_KIT_DOCTRINE_FILE")?,
    };
    install(&agent, &doctrine)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOCTRINE: &str = "\
# Doctrine

## Methodology-maintenance rules

1. **First rule.** Body text.

   *Digest:* the first rule's one-liner.

2. **Second rule.** More body.

   *Digest:* the second rule's one-liner.

## Something else

3. **Not a rule.** Outside the section.
";

    fn scratch(label: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "cw-doctrine-{}.{}",
            label,
            std::process::id()
        ));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("cannot make the scratch tree");
        dir
    }

    // spec: doctrine-kit/SPEC.md §install-doctrine — the walk reads a rule's name as assertion C
    // does and its summary off the rule's own trailer, and it stops at the next heading of the
    // section's own level or above.
    #[test]
    fn the_walk_reads_the_rules_of_that_section_and_no_others() {
        let rules = walk_rules(DOCTRINE).expect("the section was not found");
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].name, "First rule");
        assert_eq!(rules[0].summary, "the first rule's one-liner.");
        assert_eq!(rules[0].trailers, 1);
        assert_eq!(rules[1].name, "Second rule");
        assert!(walk_rules("# no such section\n").is_none());
        assert_eq!(rule_name("1. **Named.** rest"), Some("Named".to_string()));
        assert_eq!(rule_name("- **Not numbered**"), None);
        assert_eq!(hlevel("#no-space"), 0);
    }

    // spec: doctrine-kit/SPEC.md §install-doctrine — the round trip: a declared trim replaces its
    // bullet in place on the next run, a duplicate is dropped with a finding, and a trim naming no
    // live rule is carried at the digest's end rather than lost.
    #[test]
    fn a_declared_trim_takes_its_bullets_place_and_an_orphan_is_carried() {
        let dir = scratch("trim");
        let doc = dir.join("DOCTRINE.md").display().to_string();
        let agent = dir.join("AGENT.md").display().to_string();
        std::fs::write(&doc, DOCTRINE).expect("cannot write the scratch doctrine");
        std::fs::write(&agent, "# Agent\n\nResident.\n").expect("cannot write the scratch agent");

        let first = install(&agent, &doc).expect("the first install failed");
        assert!(first.line.starts_with("appended "), "{}", first.line);
        let body = std::fs::read_to_string(&agent).expect("cannot read back");
        assert!(body.contains("- **First rule** — the first rule's one-liner."));

        let trimmed = body.replace(
            "- **First rule** — the first rule's one-liner.",
            "<!-- doctrine-digest-trim: First rule — not our practice -->\n<!-- doctrine-digest-trim: First rule — a duplicate -->\n<!-- doctrine-digest-trim: Gone rule — an orphan -->",
        );
        std::fs::write(&agent, trimmed).expect("cannot write the trimmed agent");

        let second = install(&agent, &doc).expect("the second install failed");
        assert!(second.line.starts_with("replaced "), "{}", second.line);
        assert!(second.line.contains("2 declared trim(s) carried, 1 unmatched, 1 duplicate"));
        assert_eq!(second.findings.len(), 2);
        let body = std::fs::read_to_string(&agent).expect("cannot read back");
        assert!(
            !body.contains("- **First rule** —"),
            "the trimmed rule's bullet came back"
        );
        assert!(body.contains("doctrine-digest-trim: First rule — not our practice"));
        assert!(body.contains("doctrine-digest-trim: Gone rule — an orphan"));
        assert!(body.contains("- **Second rule** — the second rule's one-liner."));
        assert_eq!(
            body.matches("doctrine-digest-trim: First rule").count(),
            1,
            "the duplicate trim was carried too"
        );

        // spec: doctrine-kit/SPEC.md §install-doctrine — the removal path, idempotent, and a
        // reinstall afterwards restores the block.
        assert!(remove(&agent).expect("the removal failed").starts_with("removed "));
        assert!(remove(&agent)
            .expect("the second removal failed")
            .starts_with("no doctrine reference block"));
        assert!(install(&agent, &doc).is_ok());
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: doctrine-kit/SPEC.md §install-doctrine — a rule carrying no trailer, or two, refuses
    // the whole run rather than emitting a digest missing or double-sourcing its bullet.
    #[test]
    fn a_rule_without_exactly_one_trailer_refuses_the_run() {
        let dir = scratch("refuse");
        let doc = dir.join("DOCTRINE.md").display().to_string();
        let agent = dir.join("AGENT.md").display().to_string();
        std::fs::write(&agent, "# Agent\n").expect("cannot write the scratch agent");
        std::fs::write(
            &doc,
            "## Methodology-maintenance rules\n\n1. **Bare.** No trailer.\n",
        )
        .expect("cannot write the scratch doctrine");
        assert!(install(&agent, &doc).is_err());
        std::fs::write(
            &doc,
            "## Methodology-maintenance rules\n\n1. **Twice.** Body.\n\n   *Digest:* one.\n\n   *Digest:* two.\n",
        )
        .expect("cannot write the scratch doctrine");
        assert!(install(&agent, &doc).is_err());
        std::fs::write(&doc, "## Methodology-maintenance rules\n\nNo rules here.\n")
            .expect("cannot write the scratch doctrine");
        assert!(install(&agent, &doc).is_err());
        assert!(install("/checkwright-no-such-agent", &doc).is_err());
        assert_eq!(run(&["--nope".to_string()]), 2);
        std::fs::remove_dir_all(&dir).ok();
    }
}
