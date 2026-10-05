// spec: docs/site-architecture.md §Generated projections and their freshness gates — the two
// toolchain tables hold whole-element parity with the probe roster, each row on its audience's page
use super::install_docs;
use crate::fresh;
use crate::toolfloor::{CONTRIBUTOR, KIT_JOIN, REGISTERED};
use std::collections::BTreeMap;
use std::path::Path;

const BEGIN: &str = "<!-- toolchain:begin -->";
const END: &str = "<!-- toolchain:end -->";
// spec: docs/site-architecture.md §Generated projections and their freshness gates — the Version
// cell's floor lead, its field separator, and its spelling for neither axis
const GE: &str = "≥";
const SEP: &str = ", ";
const NEITHER: &str = "any";
// spec: docs/site-architecture.md §Generated projections and their freshness gates — the Needed
// cell's rendering of each audience, a kit list's lead and its last-pair joiner among them
const REQUIRED: &str = "required";
const CONTRIBUTORS: &str = "contributors";
const REGISTERED_GATES: &str = "optional: if you register a gate that runs it";
const KIT_LIST: &str = "optional: if your profile includes";
const LAST_SEP: &str = " or ";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-toolchain: {}", e);
            2
        }
    }
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — no cell
// carries `|`, so a row's cells are its `|`-delimited runs inside the outer pipes, each trimmed
fn cells(line: &str) -> Vec<String> {
    let body = line.trim();
    let body = body.strip_prefix('|').unwrap_or(body);
    let body = body.strip_suffix('|').unwrap_or(body);
    body.split('|').map(|c| c.trim().to_string()).collect()
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the first
// cell is one backticked name, so the header and delimiter rows are never rows
fn backticked_name(cell: &str) -> Option<&str> {
    let inner = cell.strip_prefix('`')?.strip_suffix('`')?;
    if inner.is_empty() || inner.contains('`') {
        return None;
    }
    Some(inner)
}

struct Row {
    name: String,
    // spec: docs/site-architecture.md §Generated projections and their freshness gates — the
    // Version and Needed cells as `render` spells them, compared verbatim
    cells: String,
    contributor: bool,
}

fn listed_rows(text: &str) -> Vec<Row> {
    let mut inb = false;
    let mut out: Vec<Row> = Vec::new();
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
        let cells = cells(line);
        let Some(name) = cells.first().and_then(|c| backticked_name(c)) else {
            continue;
        };
        let version = cells.get(1).map(String::as_str).unwrap_or("");
        let needed = cells.get(2).map(String::as_str).unwrap_or("");
        out.push(Row {
            name: name.to_string(),
            cells: format!("| {} | {} |", version, needed),
            contributor: needed == CONTRIBUTORS,
        });
    }
    out
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the roster
// grammar has one crate-side parser, `toolfloor::parse`, which this gate shares with the env-probe
// arm rather than holding a second copy the two could disagree about
// spec: context-kit/SPEC.md §bin/env-probe — a derived audience is resolved before the comparison,
// so the page renders the kits and the gate compares against a derivation rather than against a
// literal. That is what makes the parity unsatisfiable by editing both sides: one side is measured.
fn roster_quad(element: &str, derived: &[String]) -> (String, String) {
    let e = crate::toolfloor::parse(element);
    let audience = crate::toolfloor::resolved_audience(&e.audience, derived);
    (
        e.name.clone(),
        format!("{}:{}:{}:{}", e.name, e.min, e.imp, audience),
    )
}

// spec: docs/site-architecture.md §Generated projections and their freshness gates — the Version
// and Needed cells an element demands
fn render(quad: &str) -> String {
    let mut it = quad.splitn(4, ':');
    it.next();
    let min = it.next().unwrap_or("");
    let imp = it.next().unwrap_or("");
    let aud = it.next().unwrap_or("");
    let mut version: Vec<String> = Vec::new();
    if !min.is_empty() {
        version.push(format!("{} {}", GE, min));
    }
    if !imp.is_empty() {
        version.push(imp.to_string());
    }
    let version = if version.is_empty() {
        NEITHER.to_string()
    } else {
        version.join(SEP)
    };
    let needed = match aud {
        "" => REQUIRED.to_string(),
        CONTRIBUTOR => CONTRIBUTORS.to_string(),
        REGISTERED => REGISTERED_GATES.to_string(),
        kits => {
            let names: Vec<&str> = kits.split(KIT_JOIN).collect();
            let listed = match names.split_last() {
                Some((last, rest)) if !rest.is_empty() => {
                    format!("{}{}{}", rest.join(SEP), LAST_SEP, last)
                }
                _ => names.join(SEP),
            };
            format!("{} {}", KIT_LIST, listed)
        }
    };
    format!("| {} | {} |", version, needed)
}

// spec: context-kit/SPEC.md §bin/env-probe — the roster is the crate's own, so the third
// positional is an *override* rather than the ordinary path to it: a hermetic fixture needs a
// roster it can author, or the divergence cases are unreachable without editing the kit's own.
fn elements_of(roster: Option<&String>) -> Result<(Vec<String>, String), String> {
    let Some(path) = roster else {
        return Ok((
            crate::toolfloor::PROBE_SET.iter().map(|e| e.to_string()).collect(),
            crate::toolfloor::roster()?,
        ));
    };
    if !Path::new(path).is_file() {
        return Err(format!("roster file not found: {}", path));
    }
    let text = fresh::read_captured(path)?;
    let elements = crate::toolfloor::probe_set(&text)
        .ok_or_else(|| format!("no PROBE_SET=(...) array in {}", path))?;
    Ok((elements, path.clone()))
}

fn page_rows(page: &str) -> Result<Vec<Row>, String> {
    if !Path::new(page).is_file() {
        return Err(format!("toolchain page not found: {}", page));
    }
    let text = fresh::read_captured(page)?;
    if !text.contains(BEGIN) {
        return Err(format!("no toolchain marker block ({}) in {}", BEGIN, page));
    }
    Ok(listed_rows(&text))
}

// spec: installer/SPEC.md §The hosted install pin — the install page carrying the toolchain block
// is the home of every non-contributor row
fn install_rows(args: &[String]) -> Result<(String, Vec<Row>), String> {
    let pages = install_docs::read(&install_docs::paths(args, 0)?, "toolchain page not found")?;
    match install_docs::carrier(&pages, "a toolchain block", |t| t.contains(BEGIN))? {
        Some(m) => Ok((m.path.clone(), listed_rows(&m.text))),
        None => Err(format!(
            "no toolchain marker block ({}) in {}",
            BEGIN,
            install_docs::names(&pages)
        )),
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let (install_md, install_rows) = install_rows(args)?;
    let contributing_md = fresh::positional_or_knob(args, 1, "GATE_LOCAL_CONTRIBUTING_FILE")?;
    let contributing_rows = page_rows(&contributing_md)?;
    let (install_md, contributing_md) = (install_md.as_str(), contributing_md.as_str());
    if install_rows.is_empty() && contributing_rows.is_empty() {
        return Err(format!(
            "marker blocks present but no '| `tool` | … |' rows in {} or {}",
            install_md, contributing_md
        ));
    }

    let (elements, roster) = elements_of(args.get(2))?;
    if elements.is_empty() {
        return Err(format!("PROBE_SET array is empty in {}", roster));
    }

    // spec: context-kit/SPEC.md §bin/env-probe — bought only where an element asks for it, so a
    // roster carrying no derived audience needs no kit roots and a fixture may author one; an
    // unresolvable derivation fails closed, on that section's undecided rule.
    let derived = if elements
        .iter()
        .any(|e| crate::toolfloor::parse(e).audience == crate::toolfloor::DERIVED)
    {
        let d = crate::toolfloor::derived_kit_audience_here()?;
        if d.is_empty() {
            return Err(format!(
                "the roster carries a '{}' audience and no kit root under this tree satisfies its \
                 predicate — the page's row cannot be compared against nothing",
                crate::toolfloor::DERIVED
            ));
        }
        d
    } else {
        Vec::new()
    };

    let mut roster_by_name: BTreeMap<String, String> = BTreeMap::new();
    for e in &elements {
        let (name, quad) = roster_quad(e, &derived);
        roster_by_name.insert(name, render(&quad));
    }

    let mut findings: Vec<String> = Vec::new();
    let mut listed_by_name: BTreeMap<String, String> = BTreeMap::new();
    for (page, rows) in [(install_md, &install_rows), (contributing_md, &contributing_rows)] {
        for r in rows {
            // spec: docs/site-architecture.md §Generated projections and their freshness gates — a
            // contributors row only in CONTRIBUTING.md, every other row only on the install page
            let home = if r.contributor { contributing_md } else { install_md };
            if page != home {
                findings.push(format!(
                    "a row on the wrong page: {} belongs in {}, not {}",
                    r.name, home, page
                ));
            }
            if listed_by_name.insert(r.name.clone(), r.cells.clone()).is_some() {
                findings.push(format!("listed twice: {}", r.name));
            }
        }
    }

    // spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — the union in byte order,
    // the contract `sort -u` states rather than the collation a locale gives it
    let mut names: Vec<&String> = roster_by_name.keys().chain(listed_by_name.keys()).collect();
    names.sort();
    names.dedup();

    for n in names {
        match (roster_by_name.get(n), listed_by_name.get(n)) {
            (None, _) => findings.push(format!("listed but not probed: {}", n)),
            (Some(_), None) => findings.push(format!("probed but not listed: {}", n)),
            (Some(r), Some(l)) if r != l => findings.push(format!(
                "constraint mismatch: {} — roster says {}, page says {}",
                n, r, l
            )),
            _ => {}
        }
    }

    if !findings.is_empty() {
        println!(
            "check-install-toolchain: the toolchain tables in {} and {} and {} PROBE_SET disagree:",
            install_md, contributing_md, roster
        );
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: each row renders its roster element verbatim —");
        println!(
            "        `| \\`tool\\` | {} <floor>{}<impl-token> | <needed> | <why> |`, the Version cell",
            GE, SEP
        );
        println!(
            "        `{}` where the element sets neither, and a `{}` row only in {}.",
            NEITHER, CONTRIBUTORS, contributing_md
        );
        println!("        Add the missing tool's row, drop the stale one, correct its cells, or move it.");
        return Ok(1);
    }

    println!(
        "INSTALL-TOOLCHAIN: clean ({} roster element(s) in name+floor+impl+audience parity between {} + {} and {} PROBE_SET, each row on its audience's page)",
        roster_by_name.len(),
        install_md,
        contributing_md,
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
            assert_eq!(roster_quad(e, &[]).1, "jq:::");
        }
        assert_eq!(roster_quad("cargo:1.71::contributor", &[]).1, "cargo:1.71::contributor");
        assert_eq!(roster_quad("sort::coreutils", &[]).1, "sort::coreutils:");
        // spec: context-kit/SPEC.md §bin/env-probe — a derived element normalizes to the kit list
        // it resolves to, which is the quadruple the page's row is held to
        let derived = ["alpha-kit".to_string(), "beta-kit".to_string()];
        assert_eq!(
            roster_quad("bash:4.3::derived", &derived).1,
            "bash:4.3::alpha-kit+beta-kit"
        );
    }

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — the
    // header and delimiter rows skipped, each cell trimmed
    #[test]
    fn the_page_side_reads_each_row_off_its_cells() {
        let text = format!(
            "{}\n| Tool | Version | Needed | Why |\n|---|---|---|---|\n|  `cargo` |  ≥ 1.71, gnu | contributors | x |\n| `jq` | any | required | y |\n{}\n",
            BEGIN, END
        );
        let rows = listed_rows(&text);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "cargo");
        assert_eq!(rows[0].cells, "| ≥ 1.71, gnu | contributors |");
        assert!(rows[0].contributor);
        assert_eq!(rows[1].cells, render("jq:::"));
        assert!(!rows[1].contributor);
    }

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — an
    // optional row states its condition, and a kit list joins its last two names with ` or `
    #[test]
    fn render_spells_the_two_cells_an_element_demands() {
        assert_eq!(render("jq:::"), "| any | required |");
        assert_eq!(render("bash:4.0::"), "| ≥ 4.0 | required |");
        assert_eq!(render("git:::contributor"), "| any | contributors |");
        assert_eq!(
            render("shellcheck:0.6::registered"),
            "| ≥ 0.6 | optional: if you register a gate that runs it |"
        );
        assert_eq!(
            render("curl:5.9::a-kit"),
            "| ≥ 5.9 | optional: if your profile includes a-kit |"
        );
        assert_eq!(
            render("bash:4.3:gnu:a-kit+b-kit"),
            "| ≥ 4.3, gnu | optional: if your profile includes a-kit or b-kit |"
        );
        assert_eq!(
            render("bash:4.3::a-kit+b-kit+c-kit"),
            "| ≥ 4.3 | optional: if your profile includes a-kit, b-kit or c-kit |"
        );
    }
}
