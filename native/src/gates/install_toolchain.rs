// spec: docs/site-architecture.md §Generated projections and their freshness gates — the
// install page's toolchain list holds whole-element parity (name, floor, implementation token,
// audience) with the probe roster, both directions
use crate::fresh;
use std::collections::BTreeMap;
use std::path::Path;

const DEFAULT_INSTALL_MD: &str = "docs/install.md";
const DEFAULT_ROSTER: &str = crate::toolfloor::ROSTER;
const BEGIN: &str = "<!-- toolchain:begin -->";
const END: &str = "<!-- toolchain:end -->";
// spec: docs/site-architecture.md §Generated projections and their freshness gates — the floor
// and the audience each carry a leading sigil, so this positional reader cannot mistake an
// audience for an implementation token
const GE: &str = "≥";
const AT: &str = "@";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-toolchain: {}", e);
            2
        }
    }
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the
// bullet's parenthetical carries the roster token verbatim, so each side normalizes to one
// `name:min:impl:audience` quadruple and parity is a set comparison rather than a mapping table
fn listed_quads(text: &str) -> Vec<String> {
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
        if !inb || !line.starts_with("- `") {
            continue;
        }
        let (name, rest) = match backticked(line) {
            Some(v) => v,
            None => continue,
        };
        let (mut min, mut imp, mut aud) = (String::new(), String::new(), String::new());
        if let Some(inner) = parenthetical(rest) {
            for f in inner.split(',') {
                let f = f.trim_matches(|c: char| c.is_whitespace());
                if f.is_empty() {
                    continue;
                }
                if let Some(v) = f.strip_prefix(GE) {
                    min = v.trim_matches(|c: char| c.is_whitespace()).to_string();
                } else if let Some(v) = f.strip_prefix(AT) {
                    aud = v.trim_matches(|c: char| c.is_whitespace()).to_string();
                } else {
                    imp = f.to_string();
                }
            }
        }
        out.push(format!("{}:{}:{}:{}", name, min, imp, aud));
    }
    out
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — awk's
// `match($0, /`[^`]+`/)`: the first backticked run, and everything after it
fn backticked(line: &str) -> Option<(&str, &str)> {
    let open = line.find('`')?;
    let close = open + 1 + line[open + 1..].find('`')?;
    if close == open + 1 {
        return None;
    }
    Some((&line[open + 1..close], &line[close + 1..]))
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — awk's
// `match(rest, /^ \([^)]*\)/)`: a parenthetical immediately after the name, or none
fn parenthetical(rest: &str) -> Option<&str> {
    let body = rest.strip_prefix(" (")?;
    let close = body.find(')')?;
    Some(&body[..close])
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the roster
// grammar has one crate-side parser, `toolfloor::parse`, which this gate shares with the env-probe
// arm rather than holding a second copy the two could disagree about
fn roster_quad(element: &str) -> (String, String) {
    let e = crate::toolfloor::parse(element);
    (
        e.name.clone(),
        format!("{}:{}:{}:{}", e.name, e.min, e.imp, e.audience),
    )
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the bullet
// parenthetical an element demands, `(none)` when unconstrained
fn render(quad: &str) -> String {
    let mut it = quad.splitn(4, ':');
    it.next();
    let min = it.next().unwrap_or("");
    let imp = it.next().unwrap_or("");
    let aud = it.next().unwrap_or("");
    let mut desc = String::new();
    if !min.is_empty() {
        desc = format!("{} {}", GE, min);
    }
    if !imp.is_empty() {
        if !desc.is_empty() {
            desc.push_str(", ");
        }
        desc.push_str(imp);
    }
    if !aud.is_empty() {
        if !desc.is_empty() {
            desc.push_str(", ");
        }
        desc.push_str(AT);
        desc.push_str(aud);
    }
    if desc.is_empty() {
        "(none)".to_string()
    } else {
        format!("({})", desc)
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let install_md = fresh::positional(args, 0, DEFAULT_INSTALL_MD);
    let roster = fresh::positional(args, 1, DEFAULT_ROSTER);

    if !Path::new(install_md).is_file() {
        return Err(format!("install page not found: {}", install_md));
    }
    if !Path::new(roster).is_file() {
        return Err(format!("roster file not found: {}", roster));
    }
    let install_text = fresh::read_captured(install_md)?;
    if !install_text.contains(BEGIN) {
        return Err(format!(
            "no toolchain marker block ({}) in {}",
            BEGIN, install_md
        ));
    }

    let listed = listed_quads(&install_text);
    if listed.is_empty() {
        return Err(format!(
            "marker block present but no '- `tool`' bullets in {}",
            install_md
        ));
    }

    let roster_text = fresh::read_captured(roster)?;
    let elements = crate::toolfloor::probe_set(&roster_text)
        .ok_or_else(|| format!("no PROBE_SET=(...) array in {}", roster))?;
    if elements.is_empty() {
        return Err(format!("PROBE_SET array is empty in {}", roster));
    }

    let mut roster_by_name: BTreeMap<String, String> = BTreeMap::new();
    for e in &elements {
        let (name, quad) = roster_quad(e);
        roster_by_name.insert(name, quad);
    }
    let mut listed_by_name: BTreeMap<String, String> = BTreeMap::new();
    for q in &listed {
        let name = q.split(':').next().unwrap_or("").to_string();
        listed_by_name.insert(name, q.clone());
    }

    // spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — the union in byte order,
    // the contract `sort -u` states rather than the collation a locale gives it
    let mut names: Vec<&String> = roster_by_name.keys().chain(listed_by_name.keys()).collect();
    names.sort();
    names.dedup();

    let mut findings: Vec<String> = Vec::new();
    for n in names {
        match (roster_by_name.get(n), listed_by_name.get(n)) {
            (None, _) => findings.push(format!("listed but not probed: {}", n)),
            (Some(_), None) => findings.push(format!("probed but not listed: {}", n)),
            (Some(r), Some(l)) if r != l => findings.push(format!(
                "constraint mismatch: {} — roster says {}, page says {}",
                n,
                render(r),
                render(l)
            )),
            _ => {}
        }
    }

    if !findings.is_empty() {
        println!(
            "check-install-toolchain: {} toolchain list and {} PROBE_SET disagree:",
            install_md, roster
        );
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: each bullet in the toolchain marker block renders its roster element");
        println!(
            "        verbatim — `- \\`tool\\` ({} <min-version>, <impl-token>, {}<audience>) — …`, any",
            GE, AT
        );
        println!("        field dropped where the element leaves it empty. Add the missing tool's");
        println!("        bullet, drop the stale one, or correct the parenthetical.");
        return Ok(1);
    }

    println!(
        "INSTALL-TOOLCHAIN: clean ({} roster element(s) in name+floor+impl+audience parity between {} and {} PROBE_SET)",
        roster_by_name.len(),
        install_md,
        roster
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — the
    // three spellings of an unconstrained member are one member
    #[test]
    fn every_empty_trailing_field_normalizes_to_the_same_quadruple() {
        for e in ["jq", "jq:", "jq::", "jq:::"] {
            assert_eq!(roster_quad(e).1, "jq:::");
        }
        assert_eq!(roster_quad("cargo:1.71::contributor").1, "cargo:1.71::contributor");
        assert_eq!(roster_quad("sort::coreutils").1, "sort::coreutils:");
    }

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — the
    // audience's sigil is what keeps the positional reader honest
    #[test]
    fn the_page_side_reads_each_axis_by_its_sigil() {
        let text = format!(
            "{}\n- `cargo` (≥ 1.71, gnu, @contributor) — x.\n- `jq` — y.\n{}\n",
            BEGIN, END
        );
        assert_eq!(
            listed_quads(&text),
            vec!["cargo:1.71:gnu:contributor".to_string(), "jq:::".to_string()]
        );
    }

    #[test]
    fn an_unconstrained_element_renders_as_none() {
        assert_eq!(render("jq:::"), "(none)");
        assert_eq!(render("bash:4.0::"), "(≥ 4.0)");
        assert_eq!(render("git:::contributor"), "(@contributor)");
    }
}
