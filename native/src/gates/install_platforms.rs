// spec: docs/site-architecture.md §Generated projections and their freshness gates — the install
// page's platform declaration block, native/targets.list and both bootstraps' host detectors hold
// lockstep, with each held platform's omitted count and each detector's triple count printed
use crate::fresh;
use crate::registry;
use crate::walk;
use std::path::Path;

const DEFAULT_INSTALL_MD: &str = "docs/install.md";
// spec: gate-sdk/SPEC.md §Consumer payload — a positional operand with a layout default and never
// a read of GATE_SDK_NATIVE_TARGETS_FILE, whose value is a smoke's narrowed host roster: a bound
// reading that would be discharged by the narrowing rather than by the declaration
const DEFAULT_ROSTER: &str = "native/targets.list";
// spec: installer/README.md §The gate binary — the two hand-kept host detectors, read as the
// OWNERS of their triple sets rather than against a roster comment beside them, which would be the
// second copy this whole binding exists to refuse
const DEFAULT_BASH_BOOTSTRAP: &str = "installer/bin/checkwright.sh";
const DEFAULT_PWSH_BOOTSTRAP: &str = "installer/bin/checkwright.ps1";
const BEGIN: &str = "<!-- platforms:begin -->";
const END: &str = "<!-- platforms:end -->";
// spec: docs/site-architecture.md §Generated projections and their freshness gates — two join
// states and no third, the held one carrying its precondition after the colon
const JOINED: &str = "joined";
const HELD: &str = "held:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-platforms: {}", e);
            2
        }
    }
}

enum State {
    Joined,
    Held(String),
    Unreadable(String),
}

struct Decl {
    triple: String,
    state: State,
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the triple is
// the line's first backticked run and the state is the parenthetical read to its first `)`, the
// same positional shape check-install-toolchain's bullets take, so one grammar serves both blocks
fn declarations(text: &str) -> Vec<Decl> {
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
        if !inb || !line.starts_with("- `") {
            continue;
        }
        let Some((triple, rest)) = backticked(line) else {
            continue;
        };
        let state = match parenthetical(rest) {
            None => State::Unreadable("no join state in a parenthetical".to_string()),
            Some(p) if p.trim() == JOINED => State::Joined,
            Some(p) => match p.trim().strip_prefix(HELD) {
                Some(cause) => State::Held(cause.trim().to_string()),
                None => State::Unreadable(format!("unknown join state `{}`", p.trim())),
            },
        };
        out.push(Decl {
            triple: triple.to_string(),
            state,
        });
    }
    out
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the first
// backticked run, and everything after it
fn backticked(line: &str) -> Option<(&str, &str)> {
    let open = line.find('`')?;
    let close = open + 1 + line[open + 1..].find('`')?;
    if close == open + 1 {
        return None;
    }
    Some((&line[open + 1..close], &line[close + 1..]))
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — a
// parenthetical immediately after the triple, read to its first `)`, so it carries no nested one
fn parenthetical(rest: &str) -> Option<&str> {
    let body = rest.strip_prefix(" (")?;
    let close = body.find(')')?;
    Some(&body[..close])
}

// spec: gate-sdk/SPEC.md §Consumer payload — one target triple per live line, `#`-comments and
// blanks stripped, the line grammar scripts/gates.list uses
fn roster_triples(text: &str) -> Vec<String> {
    registry::members(text)
        .iter()
        .filter_map(|l| l.split_whitespace().next().map(String::from))
        .collect()
}

// spec: installer/README.md §The gate binary — one detector, named by the function whose body owns
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

// spec: installer/README.md §The gate binary — the pinned shape: the verb's SOLE single-quoted
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

// spec: installer/README.md §The gate binary — every failure here is a check that could not run and
// never a pass: the function absent or renamed, its body unbounded, zero triples extracted, or the
// file unreadable. An extraction that silently degrades to nothing is the one way a lockstep
// assertion reports agreement it never tested
fn detector_triples(path: &str, d: &Detector) -> Result<Vec<String>, String> {
    if !Path::new(path).is_file() {
        return Err(format!("host detector not found: {}", path));
    }
    let text = fresh::read_captured(path)?;
    detector_triples_in(&text, path, d)
}

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

// spec: gate-sdk/SPEC.md §The port-candidate criteria — arm D's corpus: the registry members that
// resolve to a `.gate` descriptor are exactly what a host with no published artifact loses, so the
// binary-less residual is a count over the live registry rather than a number carried in prose
fn omitted_members() -> Result<usize, String> {
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    let mut resolve_dirs = vec![gates_dir.clone()];
    resolve_dirs.extend(
        walk::kit_roots_rel()?
            .into_iter()
            .filter(|r| !r.is_empty())
            .map(|r| format!("{}/checks", r.trim_end_matches('/'))),
    );
    let list = registry::list_path(&gates_dir);
    if !Path::new(&list).is_file() {
        return Err(format!("no gate registry at {}", list));
    }
    let listing = fresh::read_captured(&list)?;
    Ok(registry::members(&listing)
        .into_iter()
        .filter(|m| registry::resolve(m, &resolve_dirs).map(|s| s.ends_with(".gate")) == Some(true))
        .count())
}

// spec: gate-sdk/SPEC.md §The port-candidate criteria — a count that could not be taken must not
// suppress three verdicts that do not depend on it, so the walk failure is reported once as the
// count's own value and never as this gate's exit status
fn omitted_report(held: &[&str]) -> String {
    match omitted_members() {
        Ok(n) => held
            .iter()
            .map(|t| format!("{} omits {} member(s)", t, n))
            .collect::<Vec<String>>()
            .join(", "),
        Err(e) => format!("uncounted for every held platform — {}", e),
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let install_md = fresh::positional(args, 0, DEFAULT_INSTALL_MD);
    let roster = fresh::positional(args, 1, DEFAULT_ROSTER);
    let bash_path = fresh::positional(args, 2, DEFAULT_BASH_BOOTSTRAP);
    let pwsh_path = fresh::positional(args, 3, DEFAULT_PWSH_BOOTSTRAP);

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
            "marker block present but no '- `<triple>` (<state>)' declarations in {}",
            install_md
        ));
    }

    let roster_text = fresh::read_captured(roster)?;
    let listed = roster_triples(&roster_text);

    let detected = [
        (bash_path, detector_triples(bash_path, &BASH_DETECTOR)?),
        (pwsh_path, detector_triples(pwsh_path, &PWSH_DETECTOR)?),
    ];

    let mut findings: Vec<String> = Vec::new();
    let mut held: Vec<&str> = Vec::new();
    let mut joined = 0usize;

    for d in &decls {
        let on_roster = listed.contains(&d.triple);
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

    // spec: installer/README.md §The gate binary — arm E, EQUALITY rather than containment
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

    // spec: installer/README.md §The gate binary — the count rides the clean line on the same
    // vacuous-pass ground arm D stands on: a source scan whose extraction quietly stops matching
    // reports an empty set as agreement, and a number is what makes that visible without an audit
    let detector_report = detected
        .iter()
        .map(|(p, s)| format!("{} emits {}", p, s.len()))
        .collect::<Vec<String>>()
        .join(", ");

    // spec: gate-sdk/SPEC.md §The port-candidate criteria — arm D rides the red line as well as
    // the clean one: a number that appears only when something is already broken is a post-mortem
    // rather than an instrument
    let per_held = omitted_report(&held);

    if !findings.is_empty() {
        println!(
            "check-install-platforms: {}'s platform declaration, {} and the two host detectors are not in lockstep:",
            install_md, roster
        );
        for f in &findings {
            println!("  {}", f);
        }
        if !held.is_empty() {
            println!("  omitted on each held platform: {}", per_held);
        }
        println!("  detected triples: {}", detector_report);
        println!("  help: every declared platform is `(joined)` — a live line in the roster — or");
        println!("        `(held: <precondition>)` naming the run that would join it and absent");
        println!("        from the roster, and every roster line is a declared `joined`. Flip the");
        println!("        declaration and write the roster line together, or drop the roster line.");
        println!("        Each detector's emitted triple set equals the declared set: a triple it");
        println!("        detects is declared, and a triple declared is one it can reach.");
        return Ok(1);
    }

    println!(
        "INSTALL-PLATFORMS: clean ({} declared platform(s) in {}, {} joined in lockstep with {} both directions, {} held with a stated precondition{}{}; both host detectors emit exactly the declared set — {}; omitted count is the registry members dispatching to the gate binary)",
        decls.len(),
        install_md,
        joined,
        roster,
        held.len(),
        if held.is_empty() { "" } else { " — " },
        per_held,
        detector_report
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(body: &str) -> String {
        format!("{}\n\n{}\n\n{}\n", BEGIN, body, END)
    }

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — the state
    // sits on the bullet's own first line and continuation lines are prose the reader never sees
    #[test]
    fn a_continuation_line_is_prose_and_never_a_second_declaration() {
        let text = block(
            "- `x86_64-unknown-linux-gnu` (joined) — Linux on x86-64.\n  `aarch64-apple-darwin` is not declared here.",
        );
        let d = declarations(&text);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].triple, "x86_64-unknown-linux-gnu");
        assert!(matches!(d[0].state, State::Joined));
    }

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — a hold
    // with no stated cause is how a pile grows silently, so the empty cause survives parsing as an
    // empty one rather than as an absent state
    #[test]
    fn a_held_state_carries_its_precondition_and_an_empty_one_stays_empty() {
        let text = block("- `aarch64-apple-darwin` (held: a green leg) — Apple silicon.\n- `x86_64-apple-darwin` (held:) — Intel.");
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
        let text = block("- `x86_64-pc-windows-msvc` (planned) — no.\n- `i686-unknown-linux-gnu` — no parenthetical at all.");
        let d = declarations(&text);
        assert_eq!(d.len(), 2);
        assert!(matches!(d[0].state, State::Unreadable(_)));
        assert!(matches!(d[1].state, State::Unreadable(_)));
    }

    // spec: installer/README.md §The gate binary — the SOLE single-quoted operand, so a triple
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

    // spec: installer/README.md §The gate binary — the two halves' pinned shapes, read off the
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

    // spec: installer/README.md §The gate binary — every one of these is a check that could not
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
