// spec: docs/site-architecture.md §Generated projections and their freshness gates — the install
// page's platform declaration block and native/targets.list hold lockstep both directions, a
// `held` platform carries a stated precondition, and each one's omitted-member count is printed
use crate::fresh;
use crate::registry;
use crate::walk;
use std::path::Path;

const DEFAULT_INSTALL_MD: &str = "docs/install.md";
// spec: gate-sdk/SPEC.md §Consumer payload — a positional operand with a layout default and never
// a read of GATE_SDK_NATIVE_TARGETS_FILE, whose value is a smoke's narrowed host roster: a bound
// reading that would be discharged by the narrowing rather than by the declaration
const DEFAULT_ROSTER: &str = "native/targets.list";
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

    // spec: gate-sdk/SPEC.md §The port-candidate criteria — arm D rides the red line as well as
    // the clean one: a number that appears only when something is already broken is a post-mortem
    // rather than an instrument
    let per_held = omitted_report(&held);

    if !findings.is_empty() {
        println!(
            "check-install-platforms: {}'s platform declaration and {} are not in lockstep:",
            install_md, roster
        );
        for f in &findings {
            println!("  {}", f);
        }
        if !held.is_empty() {
            println!("  omitted on each held platform: {}", per_held);
        }
        println!("  help: every declared platform is `(joined)` — a live line in the roster — or");
        println!("        `(held: <precondition>)` naming the run that would join it and absent");
        println!("        from the roster, and every roster line is a declared `joined`. Flip the");
        println!("        declaration and write the roster line together, or drop the roster line.");
        return Ok(1);
    }

    println!(
        "INSTALL-PLATFORMS: clean ({} declared platform(s) in {}, {} joined in lockstep with {} both directions, {} held with a stated precondition{}{}; omitted count is the registry members dispatching to the gate binary)",
        decls.len(),
        install_md,
        joined,
        roster,
        held.len(),
        if held.is_empty() { "" } else { " — " },
        per_held
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
