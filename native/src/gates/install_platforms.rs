// spec: docs/site-architecture.md §Generated projections and their freshness gates — the install
// page's platform declaration block, native/targets.list and both bootstraps' host detectors hold
// lockstep, with each detector's triple count printed
// spec: installer/SPEC.md §Requirements — arm F holds the page's prerequisites block against the
// system families the platform block declares
// spec: installer/SPEC.md §The front door's verbs — arm G holds each `joined` row to the pinned
// release, admitting an unserved row as pending while this iteration will release
use super::pinned_release;
use crate::fresh;
use crate::registry;
use std::path::Path;

// spec: gate-sdk/SPEC.md §Consumer payload — the roster is this gate's own knob and never
// GATE_SDK_NATIVE_TARGETS_FILE, whose value is a smoke's narrowed host roster: a bound reading that
// would be discharged by the narrowing rather than by the declaration
const ROSTER_KNOB: &str = "GATE_LOCAL_TARGETS_ROSTER";
const BEGIN: &str = "<!-- platforms:begin -->";
const END: &str = "<!-- platforms:end -->";
// spec: docs/site-architecture.md §Generated projections and their freshness gates — two join
// states and no third, the held one carrying its precondition after the colon
const JOINED: &str = "joined";
const HELD: &str = "held:";
// spec: installer/SPEC.md §Requirements — arm F's block: every prerequisite the roster does not
// carry, each row stating a Minimum and whether it is required or optional under a condition
const PREREQ_BEGIN: &str = "<!-- prerequisites:begin -->";
const PREREQ_END: &str = "<!-- prerequisites:end -->";
const REQUIRED: &str = "required";
const OPTIONAL: &str = "optional:";
const NO_FLOOR: &str = "—";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-platforms: {}", e);
            2
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum State {
    Joined,
    Held(String),
    Unreadable(String),
}

pub(crate) struct Decl {
    pub(crate) triple: String,
    pub(crate) state: State,
    pub(crate) minimum: String,
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — a row carries
// a backticked run, its first the triple, the Minimum its second cell and the state its last cell
pub(crate) fn declarations(text: &str) -> Vec<Decl> {
    let mut inb = false;
    let mut out: Vec<Decl> = Vec::new();
    for line in text.lines() {
        if line == BEGIN {
            inb = true;
            continue;
        }
        if line == END {
            inb = false;
            continue;
        }
        if !inb || !line.trim_start().starts_with('|') {
            continue;
        }
        let Some(triple) = backticked(line) else {
            continue;
        };
        let cells = cells(line);
        let last = cells.last().map(String::as_str).unwrap_or("");
        let state = if last == JOINED {
            State::Joined
        } else if let Some(cause) = last.strip_prefix(HELD) {
            State::Held(cause.trim().to_string())
        } else if last.is_empty() {
            State::Unreadable("no join state in the Status cell".to_string())
        } else {
            State::Unreadable(format!("unknown join state `{}`", last))
        };
        out.push(Decl {
            triple: triple.to_string(),
            state,
            minimum: cells.get(1).cloned().unwrap_or_default(),
        });
    }
    out
}

struct Prereq {
    tool: String,
    minimum: String,
    needed: String,
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — a row is a
// block line opening with `|` past the first, which is the header, and not a delimiter row
fn prerequisites(text: &str) -> Option<Vec<Prereq>> {
    let mut inb = false;
    let mut seen = false;
    let mut header = true;
    let mut out: Vec<Prereq> = Vec::new();
    for line in text.lines() {
        if line == PREREQ_BEGIN {
            inb = true;
            seen = true;
            continue;
        }
        if line == PREREQ_END {
            inb = false;
            continue;
        }
        if !inb || !line.trim_start().starts_with('|') {
            continue;
        }
        if std::mem::take(&mut header) {
            continue;
        }
        let cells = cells(line);
        if cells.iter().all(|c| !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':')) {
            continue;
        }
        out.push(Prereq {
            tool: cells.first().cloned().unwrap_or_default(),
            minimum: cells.get(1).cloned().unwrap_or_default(),
            needed: cells.get(2).cloned().unwrap_or_default(),
        });
    }
    seen.then_some(out)
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — a system
// family is the first word of a platform row's System cell
fn families(text: &str) -> Vec<String> {
    let mut inb = false;
    let mut out: Vec<String> = Vec::new();
    for line in text.lines() {
        if line == BEGIN {
            inb = true;
            continue;
        }
        if line == END {
            inb = false;
            continue;
        }
        if !inb || !line.trim_start().starts_with('|') || backticked(line).is_none() {
            continue;
        }
        let system = cells(line).first().cloned().unwrap_or_default();
        if let Some(word) = system.split_whitespace().next() {
            let word = word.trim_end_matches(',');
            if !out.iter().any(|f| f == word) {
                out.push(word.to_string());
            }
        }
    }
    out
}

// spec: installer/SPEC.md §Requirements — arm F: a row with no Minimum, a row neither required
// nor optional under a condition, and a declared family no row's Needed-for cell names
fn prerequisite_findings(rows: &[Prereq], families: &[String]) -> Vec<String> {
    let mut findings: Vec<String> = Vec::new();
    for r in rows {
        if r.minimum.is_empty() || r.minimum == NO_FLOOR {
            findings.push(format!("a prerequisite with no Minimum: {}", r.tool));
        }
        if !r.needed.starts_with(REQUIRED) && !r.needed.starts_with(OPTIONAL) {
            findings.push(format!(
                "a prerequisite neither `{}` nor `{} <condition>`: {}",
                REQUIRED, OPTIONAL, r.tool
            ));
        }
    }
    for f in families {
        let named = rows.iter().any(|r| {
            r.needed
                .split(|c: char| !c.is_alphanumeric())
                .any(|w| w == f)
        });
        if !named {
            findings.push(format!(
                "a declared system no prerequisite row names: {}",
                f
            ));
        }
    }
    findings
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the first
// backticked run
fn backticked(line: &str) -> Option<&str> {
    let open = line.find('`')?;
    let close = open + 1 + line[open + 1..].find('`')?;
    if close == open + 1 {
        return None;
    }
    Some(&line[open + 1..close])
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — no cell
// carries `|`, so a row's cells are its `|`-delimited runs inside the outer pipes, each trimmed
fn cells(line: &str) -> Vec<String> {
    let body = line.trim();
    let body = body.strip_prefix('|').unwrap_or(body);
    let body = body.strip_suffix('|').unwrap_or(body);
    body.split('|').map(|c| c.trim().to_string()).collect()
}

// spec: gate-sdk/SPEC.md §Consumer payload — one target triple per live line, `#`-comments and
// blanks stripped, the line grammar scripts/gates.list uses
fn roster_triples(text: &str) -> Vec<String> {
    registry::members(text)
        .iter()
        .filter_map(|l| l.split_whitespace().next().map(String::from))
        .collect()
}

// spec: installer/SPEC.md §The gate binary — one detector, named by the function whose body owns
// its triple set and by the verb whose sole single-quoted operand each mapped triple is
struct Detector {
    opener: &'static str,
    label: &'static str,
    verb: &'static str,
}

const BASH_DETECTOR: Detector = Detector {
    opener: "target_of_host()",
    label: "target_of_host",
    verb: "printf",
};

const PWSH_DETECTOR: Detector = Detector {
    opener: "function Get-HostTarget",
    label: "Get-HostTarget",
    verb: "return",
};

// spec: installer/SPEC.md §Platform resolution — each half's fallback map, whose triples a host
// reaches as surely as the preferred ones, so they join the half's extracted set
const BASH_FALLBACK: Detector = Detector {
    opener: "fallback_of_target()",
    label: "fallback_of_target",
    verb: "printf",
};

const PWSH_FALLBACK: Detector = Detector {
    opener: "function Get-FallbackTarget",
    label: "Get-FallbackTarget",
    verb: "return",
};

// spec: installer/SPEC.md §Platform resolution — a half's emitted set is the union of both its
// functions' operands, and either extraction degrading refuses the whole half
fn half_triples(path: &str, functions: &[&Detector]) -> Result<Vec<String>, String> {
    if !Path::new(path).is_file() {
        return Err(format!("host detector not found: {}", path));
    }
    half_triples_in(&fresh::read_captured(path)?, path, functions)
}

fn half_triples_in(text: &str, path: &str, functions: &[&Detector]) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    for d in functions {
        for t in detector_triples_in(text, path, d)? {
            if !out.contains(&t) {
                out.push(t);
            }
        }
    }
    Ok(out)
}

// spec: installer/SPEC.md §The gate binary — the pinned shape: the verb's SOLE single-quoted
// operand, so only whitespace may sit between the two. The empty operand is PowerShell's
// no-mapping arm rather than a triple, and is dropped by the caller
fn sole_quoted_after<'a>(line: &'a str, verb: &str) -> Option<&'a str> {
    let at = line.find(verb)?;
    let rest = &line[at + verb.len()..];
    let open = rest.find('\'')?;
    let gap = &rest[..open];
    if gap.is_empty() || !gap.chars().all(char::is_whitespace) {
        return None;
    }
    let body = &rest[open + 1..];
    let close = body.find('\'')?;
    Some(&body[..close])
}

// spec: installer/SPEC.md §Platform resolution — every failure here is a check that could not run
// and never a pass: the function absent or renamed, its body unbounded, zero triples extracted, or
// the file unreadable. An extraction that silently degrades to nothing is the one way a lockstep
// assertion reports agreement it never tested
fn detector_triples_in(text: &str, path: &str, d: &Detector) -> Result<Vec<String>, String> {
    let mut lines = text.lines();
    if !lines.any(|l| l.trim_start().starts_with(d.opener)) {
        return Err(format!(
            "no `{}` in {} — the detector this gate reads is absent or renamed, so nothing was compared",
            d.label, path
        ));
    }
    let mut out: Vec<String> = Vec::new();
    let mut closed = false;
    for l in lines {
        if l == "}" {
            closed = true;
            break;
        }
        if l.trim_start().starts_with('#') {
            continue;
        }
        if let Some(q) = sole_quoted_after(l, d.verb) {
            if !q.is_empty() && !out.iter().any(|e| e == q) {
                out.push(q.to_string());
            }
        }
    }
    if !closed {
        return Err(format!(
            "`{}` in {} has no closing `}}` at column 0, so its body could not be bounded",
            d.label, path
        ));
    }
    if out.is_empty() {
        return Err(format!(
            "`{}` in {} extracted no triple — the pinned `{} '<triple>'` shape stopped matching",
            d.label, path, d.verb
        ));
    }
    Ok(out)
}

// spec: installer/SPEC.md §The front door's verbs — the pinned release arm G holds `joined` to:
// its roster, read at the gate's own roster path, and its install page
struct Pinned {
    label: String,
    roster: Vec<String>,
    page: String,
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — arm G's four
// inputs from files, or resolved live, where an unresolvable tag leaves the arm dormant
fn pinned_input(
    args: &[String],
    install_md: &str,
    roster: &str,
) -> Result<Option<(Pinned, String, String)>, String> {
    if args.len() == 8 {
        let p = Pinned {
            label: args[4].clone(),
            roster: roster_triples(&pinned_release::read(&args[4])?),
            page: pinned_release::read(&args[5])?,
        };
        return Ok(Some((p, args[6].clone(), args[7].clone())));
    }
    if args.len() > 4 {
        return Err("usage: check-install-platforms [install.md roster sh-bootstrap ps1-bootstrap [pinned-roster pinned-page disposition queue]]".to_string());
    }
    let Some(tag) = pinned_release::pinned_tag()? else {
        return Ok(None);
    };
    let p = Pinned {
        roster: roster_triples(&pinned_release::show_at(&tag, roster)?),
        page: pinned_release::show_at(&tag, install_md)?,
        label: tag,
    };
    let (disposition, queue) = pinned_release::live_paths()?;
    Ok(Some((p, disposition, queue)))
}

// spec: installer/SPEC.md §The front door's verbs — arm G: a `joined` row names a triple the pinned
// release publishes, at the Minimum the pinned release's page states; the Minimum half is dormant
// while the pinned page's block does not parse as the table
fn unserved(decls: &[Decl], pinned: &Pinned) -> (Vec<(String, String)>, bool) {
    let pinned_decls = declarations(&pinned.page);
    let minimum_live = !pinned_decls.is_empty();
    let mut out: Vec<(String, String)> = Vec::new();
    for d in decls.iter().filter(|d| matches!(d.state, State::Joined)) {
        if !pinned.roster.contains(&d.triple) {
            out.push((
                d.triple.clone(),
                format!("declared `joined`, and the pinned release {} publishes no {}", pinned.label, d.triple),
            ));
            continue;
        }
        if !minimum_live {
            continue;
        }
        match pinned_decls.iter().find(|p| p.triple == d.triple) {
            Some(p) if p.minimum == d.minimum => {}
            Some(p) => out.push((
                d.triple.clone(),
                format!(
                    "declared `joined` at Minimum `{}`, and the pinned release {} states `{}` for {}",
                    d.minimum, pinned.label, p.minimum, d.triple
                ),
            )),
            None => out.push((
                d.triple.clone(),
                format!("declared `joined`, and the pinned release {}'s page states no Minimum for {}", pinned.label, d.triple),
            )),
        }
    }
    (out, minimum_live)
}

fn rule(args: &[String]) -> Result<i32, String> {
    let install_md = fresh::positional_or_knob(args, 0, "GATE_LOCAL_INSTALL_PAGE")?;
    let roster = fresh::positional_or_knob(args, 1, ROSTER_KNOB)?;
    // spec: installer/SPEC.md §The gate binary — the two hand-kept host detectors, read as the
    // OWNERS of their triple sets rather than against a roster comment beside them, which would be
    // the second copy this whole binding exists to refuse
    let bash_path = fresh::positional_or_knob(args, 2, "GATE_LOCAL_BOOTSTRAP_SH")?;
    let pwsh_path = fresh::positional_or_knob(args, 3, "GATE_LOCAL_BOOTSTRAP_PS1")?;
    let (install_md, roster) = (install_md.as_str(), roster.as_str());
    let (bash_path, pwsh_path) = (bash_path.as_str(), pwsh_path.as_str());
    let pinned = pinned_input(args, install_md, roster)?;

    if !Path::new(install_md).is_file() {
        return Err(format!("install page not found: {}", install_md));
    }
    if !Path::new(roster).is_file() {
        return Err(format!("target roster not found: {}", roster));
    }
    let install_text = fresh::read_captured(install_md)?;
    if !install_text.contains(BEGIN) {
        return Err(format!(
            "no platform marker block ({}) in {}",
            BEGIN, install_md
        ));
    }

    let decls = declarations(&install_text);
    if decls.is_empty() {
        return Err(format!(
            "marker block present but no '| <system> | <minimum> | `<triple>` | <state> |' rows in {}",
            install_md
        ));
    }

    // spec: installer/SPEC.md §Requirements — a missing or empty prerequisites block is a check
    // that could not run, as the platform block's own absence is
    let prereqs = prerequisites(&install_text).ok_or_else(|| {
        format!(
            "no prerequisites marker block ({}) in {}",
            PREREQ_BEGIN, install_md
        )
    })?;
    if prereqs.is_empty() {
        return Err(format!(
            "prerequisites block present but no '| <tool> | <minimum> | <needed for> | <why> |' rows in {}",
            install_md
        ));
    }

    let roster_text = fresh::read_captured(roster)?;
    let listed = roster_triples(&roster_text);

    let detected = [
        (bash_path, half_triples(bash_path, &[&BASH_DETECTOR, &BASH_FALLBACK])?),
        (pwsh_path, half_triples(pwsh_path, &[&PWSH_DETECTOR, &PWSH_FALLBACK])?),
    ];

    let mut findings: Vec<String> = Vec::new();
    let mut held: Vec<&str> = Vec::new();
    let mut joined = 0usize;

    for d in &decls {
        let on_roster = listed.contains(&d.triple);
        // spec: docs/site-architecture.md §Generated projections and their freshness gates — a
        // support claim with no OS floor is half a claim
        if d.minimum.is_empty() {
            findings.push(format!("an empty Minimum cell: {}", d.triple));
        }
        match &d.state {
            // spec: gate-sdk/SPEC.md §Consumer payload — arm A: a `joined` declaration flipped
            // ahead of the roster write
            State::Joined => {
                joined += 1;
                if !on_roster {
                    findings.push(format!(
                        "declared `joined` but no live line in {}: {}",
                        roster, d.triple
                    ));
                }
            }
            // spec: gate-sdk/SPEC.md §Consumer payload — arm C: a hold cannot be granted silently,
            // and a held platform that is nonetheless published is a roster write whose
            // declaration flip was forgotten
            State::Held(cause) => {
                held.push(&d.triple);
                if cause.is_empty() {
                    findings.push(format!(
                        "declared `held` with no precondition: {}",
                        d.triple
                    ));
                }
                if on_roster {
                    findings.push(format!(
                        "declared `held` but carries a live line in {}: {}",
                        roster, d.triple
                    ));
                }
            }
            State::Unreadable(why) => {
                findings.push(format!("{}: {}", why, d.triple));
            }
        }
    }

    // spec: gate-sdk/SPEC.md §Consumer payload — arm B, the mechanization of the first bound: a
    // roster line may not exceed what the install page declares supported
    for t in &listed {
        if !decls.iter().any(|d| d.triple == *t) {
            findings.push(format!(
                "a live line in {} that {} declares nowhere: {}",
                roster, install_md, t
            ));
        }
    }

    // spec: installer/SPEC.md §The gate binary — arm E, EQUALITY rather than containment
    // because each direction closes a distinct failure: a triple a detector emits that nobody
    // declares, and a declared triple no detector emits
    for (path, set) in &detected {
        for t in set {
            if !decls.iter().any(|d| d.triple == *t) {
                findings.push(format!(
                    "{} detects {} and {} declares it nowhere",
                    path, t, install_md
                ));
            }
        }
        for d in &decls {
            if !set.contains(&d.triple) {
                findings.push(format!(
                    "{} declares {} and {} can never detect it",
                    install_md, d.triple, path
                ));
            }
        }
    }

    let declared_families = families(&install_text);
    findings.extend(prerequisite_findings(&prereqs, &declared_families));

    // spec: installer/SPEC.md §The front door's verbs — arm G's finding is admitted as pending
    // while this iteration will release, and reds while its disposition withholds one
    let mut pending: Vec<String> = Vec::new();
    let mut g_red = false;
    let g_state = match &pinned {
        None => "arm G dormant — the pinned tag does not resolve here".to_string(),
        Some((p, disposition_path, queue_path)) => {
            let (iteration, disp) = pinned_release::iteration_disposition(disposition_path, queue_path)?;
            let (unserved_rows, minimum_live) = unserved(&decls, p);
            for (triple, why) in unserved_rows {
                match &disp {
                    pinned_release::Disposition::Withheld(field) => {
                        g_red = true;
                        findings.push(format!("{} while iteration {}'s disposition is {}", why, iteration, field));
                    }
                    _ => {
                        if !pending.contains(&triple) {
                            pending.push(triple);
                        }
                    }
                }
            }
            format!(
                "arm G against the pinned release {}{}{}",
                p.label,
                if minimum_live { "" } else { ", its Minimum half dormant — the pinned page carries no platform table" },
                if pending.is_empty() { String::new() } else { format!(", pending release: {}", pending.join(" ")) }
            )
        }
    };

    // spec: installer/SPEC.md §The gate binary — the count rides the clean line because a source
    // scan whose extraction quietly stops matching reports an empty set as agreement, and a number
    // is what makes that visible without an audit
    let detector_report = detected
        .iter()
        .map(|(p, s)| format!("{} emits {}", p, s.len()))
        .collect::<Vec<String>>()
        .join(", ");

    if !findings.is_empty() {
        println!(
            "check-install-platforms: {}'s platform declaration, {} and the two host detectors are not in lockstep:",
            install_md, roster
        );
        for f in &findings {
            println!("  {}", f);
        }
        println!("  detected triples: {}", detector_report);
        println!("  help: every row's Status cell is `joined` — a live line in the roster — or");
        println!("        `held: <precondition>` naming the run that would join it and absent from");
        println!("        the roster, and every roster line is a declared `joined`. Flip the Status");
        println!("        cell and write the roster line together, or drop the roster line. Every");
        println!("        row's Minimum cell states the platform's OS floor.");
        println!("        Each detector's emitted triple set equals the declared set: a triple it");
        println!("        detects is declared, and a triple declared is one it can reach.");
        println!("        Every prerequisites row states a Minimum (`any` where nothing forces");
        println!("        one) and a Needed-for cell opening `required` or `optional: <condition>`,");
        println!("        and every declared system family is named by some row's Needed-for cell.");
        if g_red {
            println!("        A `joined` row names a triple the pinned release publishes, at the Minimum");
            println!("        its page states: release so the pin carries the row, or return the row");
            println!("        to `held`.");
        }
        return Ok(1);
    }

    println!(
        "INSTALL-PLATFORMS: clean ({} declared platform(s) in {}, {} joined in lockstep with {} both directions, {} held with a stated precondition; both host detectors emit exactly the declared set — {}; {} prerequisite(s) each with a Minimum and a required/optional marker, naming every declared family — {}; {})",
        decls.len(),
        install_md,
        joined,
        roster,
        held.len(),
        detector_report,
        prereqs.len(),
        declared_families.join(", "),
        g_state
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(body: &str) -> String {
        format!("{}\n\n{}\n\n{}\n", BEGIN, body, END)
    }

    const HEAD: &str = "| System | Minimum | Binary | Status |\n|---|---|---|---|\n";

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — the
    // header and delimiter rows carry no backticked run, and a line outside the table is prose
    #[test]
    fn only_a_row_carrying_a_triple_is_a_declaration() {
        let text = block(&format!(
            "{}| Linux on x86-64 | glibc 2.39 | `x86_64-unknown-linux-gnu` | joined |\n`aarch64-apple-darwin` is not declared here.",
            HEAD
        ));
        let d = declarations(&text);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].triple, "x86_64-unknown-linux-gnu");
        assert_eq!(d[0].minimum, "glibc 2.39");
        assert!(matches!(d[0].state, State::Joined));
    }

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — an empty
    // Minimum cell survives parsing as an empty one, which `rule` reds
    #[test]
    fn an_empty_minimum_cell_stays_empty() {
        let text = block(&format!("{}|  Linux |  | `x86_64-unknown-linux-gnu` | joined |", HEAD));
        assert!(declarations(&text)[0].minimum.is_empty());
    }

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — a hold
    // with no stated cause is how a pile grows silently, so the empty cause survives parsing as an
    // empty one rather than as an absent state
    #[test]
    fn a_held_state_carries_its_precondition_and_an_empty_one_stays_empty() {
        let text = block(&format!(
            "{}| Apple silicon | macOS 11 | `aarch64-apple-darwin` | held: a green leg |\n| Intel | macOS 10.12 | `x86_64-apple-darwin` | held: |",
            HEAD
        ));
        let d = declarations(&text);
        match &d[0].state {
            State::Held(c) => assert_eq!(c, "a green leg"),
            _ => panic!("first bullet is not held"),
        }
        match &d[1].state {
            State::Held(c) => assert!(c.is_empty()),
            _ => panic!("second bullet is not held"),
        }
    }

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — two
    // states and no third, so a third spelling is a finding rather than a silently ignored bullet
    #[test]
    fn a_third_join_state_is_unreadable_rather_than_dropped() {
        let text = block(&format!(
            "{}| Windows | Windows 10 | `x86_64-pc-windows-msvc` | planned |\n| Linux | glibc 2.17 | `i686-unknown-linux-gnu` | |",
            HEAD
        ));
        let d = declarations(&text);
        assert_eq!(d.len(), 2);
        assert!(matches!(d[0].state, State::Unreadable(_)));
        assert!(matches!(d[1].state, State::Unreadable(_)));
    }

    // spec: installer/SPEC.md §The gate binary — the SOLE single-quoted operand, so a triple
    // reached through a variable or sitting beside another operand is not extracted and the
    // detector's own shape stays the pin rather than a loose quote scan
    #[test]
    fn only_the_verbs_sole_single_quoted_operand_is_a_triple() {
        assert_eq!(
            sole_quoted_after("        Linux/x86_64)   printf 'x86_64-unknown-linux-gnu' ;;", "printf"),
            Some("x86_64-unknown-linux-gnu")
        );
        assert_eq!(
            sole_quoted_after("        '^linux/x64$' { return 'x86_64-unknown-linux-gnu' }", "return"),
            Some("x86_64-unknown-linux-gnu")
        );
        assert_eq!(sole_quoted_after("    return ''", "return"), Some(""));
        assert_eq!(sole_quoted_after("    printf \"$t\"", "printf"), None);
        assert_eq!(sole_quoted_after("    printf-ish'x'", "printf"), None);
    }

    // spec: installer/SPEC.md §The gate binary — the two halves' pinned shapes, read off the
    // function body and bounded by its own closing brace, so a triple spelled outside it is not
    // this detector's
    #[test]
    fn each_half_yields_exactly_its_functions_mapped_triples() {
        let sh = "target_of_host() {\n    case \"$(host_shape)\" in\n        Linux/x86_64) printf 'x86_64-unknown-linux-gnu' ;;\n        # printf 'never-extracted-from-a-comment'\n        *) : ;;\n    esac\n}\nother() { printf 'outside-the-body'; }\n";
        assert_eq!(
            detector_triples_in(sh, "f", &BASH_DETECTOR).unwrap(),
            vec!["x86_64-unknown-linux-gnu".to_string()]
        );
        let ps = "function Get-HostTarget {\n    switch -Regex (Get-HostShape) {\n        '^linux/x64$' { return 'x86_64-unknown-linux-gnu' }\n    }\n    return ''\n}\n";
        assert_eq!(
            detector_triples_in(ps, "f", &PWSH_DETECTOR).unwrap(),
            vec!["x86_64-unknown-linux-gnu".to_string()]
        );
    }

    // spec: installer/SPEC.md §The gate binary — every one of these is a check that could not
    // run, reaching exit 2 through `rule`'s error path rather than reporting the agreement an
    // empty extraction would otherwise look like
    #[test]
    fn a_degraded_extraction_is_a_refusal_and_never_a_pass() {
        let renamed = "resolve_host() {\n    printf 'x86_64-unknown-linux-gnu'\n}\n";
        assert!(detector_triples_in(renamed, "f", &BASH_DETECTOR)
            .unwrap_err()
            .contains("absent or renamed"));

        let unbounded = "target_of_host() {\n    printf 'x86_64-unknown-linux-gnu'\n";
        assert!(detector_triples_in(unbounded, "f", &BASH_DETECTOR)
            .unwrap_err()
            .contains("no closing"));

        let shape_moved = "target_of_host() {\n    printf \"$t\"\n}\n";
        assert!(detector_triples_in(shape_moved, "f", &BASH_DETECTOR)
            .unwrap_err()
            .contains("extracted no triple"));

        let only_the_empty_arm = "function Get-HostTarget {\n    return ''\n}\n";
        assert!(detector_triples_in(only_the_empty_arm, "f", &PWSH_DETECTOR)
            .unwrap_err()
            .contains("extracted no triple"));
    }

    // spec: installer/SPEC.md §Platform resolution — a half's set is both functions' operands
    // united, a triple both emit counted once, and a half missing its fallback map refuses
    #[test]
    fn a_half_unites_its_detector_and_its_fallback_map() {
        let sh = "target_of_host() {\n    case \"$(host_shape)\" in\n        Linux/x86_64) printf 'x86_64-unknown-linux-gnu' ;;\n    esac\n}\nfallback_of_target() {\n    case \"$1\" in\n        x86_64-unknown-linux-gnu) printf 'x86_64-unknown-linux-musl' ;;\n        *) : ;;\n    esac\n}\n";
        assert_eq!(
            half_triples_in(sh, "f", &[&BASH_DETECTOR, &BASH_FALLBACK]).unwrap(),
            vec!["x86_64-unknown-linux-gnu".to_string(), "x86_64-unknown-linux-musl".to_string()]
        );
        let ps = "function Get-HostTarget {\n    switch -Regex (Get-HostShape) {\n        '^linux/x64$' { return 'x86_64-unknown-linux-gnu' }\n    }\n    return ''\n}\nfunction Get-FallbackTarget {\n    param([string] $Target)\n    switch ($Target) {\n        'x86_64-unknown-linux-gnu' { return 'x86_64-unknown-linux-musl' }\n    }\n    return ''\n}\n";
        assert_eq!(
            half_triples_in(ps, "f", &[&PWSH_DETECTOR, &PWSH_FALLBACK]).unwrap(),
            vec!["x86_64-unknown-linux-gnu".to_string(), "x86_64-unknown-linux-musl".to_string()]
        );
        let no_fallback = "target_of_host() {\n    printf 'x86_64-unknown-linux-gnu'\n}\n";
        assert!(half_triples_in(no_fallback, "f", &[&BASH_DETECTOR, &BASH_FALLBACK])
            .unwrap_err()
            .contains("`fallback_of_target`"));
    }

    // spec: installer/SPEC.md §The front door's verbs — arm G: a joined triple the pinned roster
    // lacks, a joined Minimum the pinned page states otherwise, a held row never read, and the
    // Minimum half dormant against a pinned page with no table
    #[test]
    fn arm_g_holds_joined_rows_to_the_pinned_release() {
        let page = block(&format!(
            "{}| Linux | glibc 2.39 | `x86_64-unknown-linux-gnu` | joined |\n| Linux | Linux 3.2 | `x86_64-unknown-linux-musl` | joined |\n| Linux | glibc 2.39 | `aarch64-unknown-linux-gnu` | held: a run |",
            HEAD
        ));
        let decls = declarations(&page);
        let pinned = |page: String| Pinned {
            label: "v1.0.0".to_string(),
            roster: vec!["x86_64-unknown-linux-gnu".to_string()],
            page,
        };
        let table = block(&format!("{}| Linux | glibc 2.17 | `x86_64-unknown-linux-gnu` | joined |", HEAD));
        let (rows, live) = unserved(&decls, &pinned(table));
        assert!(live);
        assert_eq!(rows.len(), 2, "{:?}", rows);
        assert!(rows[0].1.contains("states `glibc 2.17`"));
        assert!(rows[1].1.contains("publishes no x86_64-unknown-linux-musl"));
        let bullets = format!("{}\n- `x86_64-unknown-linux-gnu` (joined) — Linux\n{}\n", BEGIN, END);
        let (rows, live) = unserved(&decls, &pinned(bullets));
        assert!(!live);
        assert_eq!(rows.len(), 1, "{:?}", rows);
        assert_eq!(rows[0].0, "x86_64-unknown-linux-musl");
    }

    fn prereq_block(body: &str) -> String {
        format!(
            "{}\n\n| Tool | Minimum | Needed for | Why |\n|---|---|---|---|\n{}\n\n{}\n",
            PREREQ_BEGIN, body, PREREQ_END
        )
    }

    // spec: installer/SPEC.md §Requirements — arm F reads the header and delimiter as no row, and
    // tells an absent block from an empty one
    #[test]
    fn the_prerequisites_block_reads_its_rows_and_its_absence() {
        let text = prereq_block("| `tar` | any | required to install on Linux and macOS | x |");
        let rows = prerequisites(&text).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].tool, "`tar`");
        assert_eq!(rows[0].minimum, "any");
        assert!(prerequisites("no block here").is_none());
        assert!(prerequisites(&prereq_block("")).unwrap().is_empty());
    }

    // spec: installer/SPEC.md §Requirements — arm F's three row findings, each reached, and a
    // family matched as a word of the Needed-for cell
    #[test]
    fn arm_f_reds_a_missing_minimum_an_unmarked_row_and_an_unnamed_family() {
        let platforms = block(&format!(
            "{}| Linux on arm64 | glibc 2.39 | `aarch64-unknown-linux-gnu` | joined |\n| macOS on Intel | macOS 10.12 | `x86_64-apple-darwin` | joined |\n| Windows on x86-64 | Windows 10 | `x86_64-pc-windows-msvc` | joined |",
            HEAD
        ));
        let fams = families(&platforms);
        assert_eq!(fams, vec!["Linux", "macOS", "Windows"]);

        let clean = prereq_block(
            "| `sh` | any | required to install on Linux and macOS | x |\n| PowerShell | 5.1 | required to install on Windows | y |\n| Node | 8.2 | optional: only to install with npx | z |",
        );
        assert!(prerequisite_findings(&prerequisites(&clean).unwrap(), &fams).is_empty());

        let red = prereq_block(
            "| `sh` | — | required to install on Linux and macOS | x |\n| PowerShell |  | needed on Windowsish hosts | y |",
        );
        let f = prerequisite_findings(&prerequisites(&red).unwrap(), &fams);
        assert_eq!(f.len(), 4, "{:?}", f);
        assert!(f[0].contains("no Minimum: `sh`"));
        assert!(f[1].contains("no Minimum: PowerShell"));
        assert!(f[2].contains("neither"));
        assert!(f[3].ends_with("names: Windows"));
    }

    // spec: gate-sdk/SPEC.md §Consumer payload — the roster's line grammar, so a commented triple
    // is not a live line and a runner-mapped second field does not become part of the triple
    #[test]
    fn the_roster_reads_the_first_token_of_every_live_line() {
        assert_eq!(
            roster_triples("# x86_64-apple-darwin\n\nx86_64-unknown-linux-gnu  ubuntu-latest\n"),
            vec!["x86_64-unknown-linux-gnu".to_string()]
        );
    }
}
