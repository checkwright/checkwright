// spec: gate-sdk/SPEC.md §check-projection-roster — the consumer's generated-projections roster holds
// one keyed row per gate declaring `# projection:`, and no key names any other gate
use crate::fresh;
use crate::registry;
use crate::section;
use crate::walk;
use std::path::Path;

const TOOL: &str = "check-projection-roster";
const KEY_OPEN: &str = "<!-- projection:";
const KEY_CLOSE: &str = "-->";

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}: {}", TOOL, msg);
            eprintln!("PROJECTION-ROSTER: {}", fresh::fail_closed("projection-roster", Some(2)));
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §check-projection-roster — the declaring set: every registered member whose
// header carries `# projection:`, with that member's expanded `couples=` beside it for assertion A
pub(crate) struct Declaring {
    pub(crate) name: String,
    lines: Vec<String>,
    couples: Vec<String>,
}

pub(crate) fn declaring_gates() -> Result<Vec<Declaring>, String> {
    let gates_dir = crate::knobs::gates_dir();
    let list = registry::list_path(&gates_dir);
    let text = std::fs::read_to_string(&list).map_err(|e| format!("cannot read the registry at {}: {}", list, e))?;
    let kit_roots = walk::kit_roots()?;
    let resolve_dirs = registry::resolve_dirs(&gates_dir, &kit_roots);
    let mut names = registry::members(&text);
    names.sort();
    names.dedup();
    let mut out: Vec<Declaring> = Vec::new();
    for name in names {
        let Some(src) = registry::resolve(&name, &resolve_dirs) else {
            return Err(format!("cannot resolve the registered member '{}' in: {}", name, resolve_dirs.join(" ")));
        };
        let body = std::fs::read(&src)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("cannot read {}'s header at {}: {}", name, src, e))?;
        let lines = registry::projection(&body);
        if lines.is_empty() {
            continue;
        }
        let fields = registry::manifest_line(&body).map(registry::manifest_fields).unwrap_or_default();
        let couples = registry::expand_couples(&registry::field(&fields, "couples"), &kit_roots)
            .map_err(|e| format!("cannot expand {}'s couples: {}", name, e))?;
        out.push(Declaring {
            name,
            lines,
            couples: couples.split(',').filter(|t| !t.is_empty()).map(String::from).collect(),
        });
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §check-projection-roster — assertion A: one line, a non-empty literal glob
// list, each glob reached by the gate's own expanded `couples=` under the field's one matcher
fn declaration_findings(d: &Declaring) -> Vec<String> {
    let [line] = d.lines.as_slice() else {
        return vec![format!(
            "{}: {} '{}' lines where a gate declares at most one",
            d.name,
            d.lines.len(),
            registry::PROJECTION
        )];
    };
    let globs: Vec<&str> = line.split(',').map(str::trim).filter(|g| !g.is_empty()).collect();
    if globs.is_empty() {
        return vec![format!("{}: '{}' declares no output path", d.name, registry::PROJECTION)];
    }
    let mut out: Vec<String> = Vec::new();
    for g in globs {
        if registry::COUPLES_PREFIXES.iter().any(|p| g.starts_with(p)) {
            out.push(format!(
                "{}: projection glob '{}' carries a couples prefix — an output path is the consumer's tree, written literally",
                d.name, g
            ));
        } else if !d.couples.iter().any(|t| registry::couple_matches(g, t)) {
            out.push(format!(
                "{}: projection glob '{}' is not covered by its own couples= — the hook would never re-run the gate on that output",
                d.name, g
            ));
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §check-projection-roster — every `<!-- projection: <gate> -->` key on one
// line, in order; a key with no closing marker is read to the line's end
fn keys_on(line: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find(KEY_OPEN) {
        let after = &rest[at + KEY_OPEN.len()..];
        let (inside, next) = match after.find(KEY_CLOSE) {
            Some(end) => (&after[..end], &after[end + KEY_CLOSE.len()..]),
            None => (after, ""),
        };
        out.push(inside.trim().to_string());
        rest = next;
    }
    out
}

// spec: gate-sdk/SPEC.md §check-projection-roster — the section is the heading whose text equals the
// knob, bounded by the next heading of its level or shallower; an empty knob is the whole file
fn section_lines<'a>(lines: &[&'a str], heading: &str) -> Option<Vec<(usize, &'a str)>> {
    if heading.is_empty() {
        return Some(lines.iter().enumerate().map(|(i, l)| (i + 1, *l)).collect());
    }
    let mut open: Option<usize> = None;
    let mut out: Vec<(usize, &'a str)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let lvl = section::heading_level(line);
        match open {
            Some(level) => {
                if lvl > 0 && lvl <= level {
                    return Some(out);
                }
                out.push((i + 1, line));
            }
            None => {
                if lvl > 0 && line[lvl..].trim() == heading {
                    open = Some(lvl);
                }
            }
        }
    }
    open.map(|_| out)
}

struct Row {
    line: usize,
    first_line_keys: Vec<String>,
}

// spec: gate-sdk/SPEC.md §check-projection-roster — a row is a top-level `- ` bullet and its key sits
// on the bullet's first line; a key anywhere else in the section is its own finding
fn rows(section: &[(usize, &str)]) -> (Vec<Row>, Vec<(usize, String)>) {
    let mut rows: Vec<Row> = Vec::new();
    let mut stray: Vec<(usize, String)> = Vec::new();
    for (n, line) in section {
        if line.starts_with("- ") {
            rows.push(Row {
                line: *n,
                first_line_keys: keys_on(line),
            });
        } else {
            for k in keys_on(line) {
                stray.push((*n, k));
            }
        }
    }
    (rows, stray)
}

fn inner(_args: &[String]) -> Result<i32, String> {
    let roster = walk::knob_scalar("GATE_SDK_PROJECTION_ROSTER")?;
    let heading = walk::knob_scalar("GATE_SDK_PROJECTION_ROSTER_SECTION")?;
    // spec: gate-sdk/SPEC.md §check-projection-roster — the arming knob: empty asserts nothing and says
    // so, a different sentence from a roster found complete
    if roster.is_empty() {
        println!(
            "PROJECTION-ROSTER: clean (no roster configured; GATE_SDK_PROJECTION_ROSTER is empty, so nothing was asserted)"
        );
        return Ok(0);
    }
    if !Path::new(&roster).is_file() {
        return Err(format!("the configured roster {} does not exist", roster));
    }
    let text = std::fs::read(&roster)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read the roster {}: {}", roster, e))?;
    let lines = fresh::file_lines(&text);
    let Some(sect) = section_lines(&lines, &heading) else {
        return Err(format!("{} has no heading '{}' (GATE_SDK_PROJECTION_ROSTER_SECTION)", roster, heading));
    };
    let declaring = declaring_gates()?;
    let (rows, stray) = rows(&sect);

    let mut findings: Vec<String> = Vec::new();
    for d in &declaring {
        findings.extend(declaration_findings(d));
    }
    // assertion B: exactly one keyed row per declaring gate
    for d in &declaring {
        let hits: Vec<usize> = rows
            .iter()
            .filter(|r| r.first_line_keys.contains(&d.name))
            .map(|r| r.line)
            .collect();
        if hits.len() != 1 {
            findings.push(format!(
                "{}: declares '{}' but {} row(s) in {} carry its key{}",
                d.name,
                registry::PROJECTION,
                hits.len(),
                roster,
                if hits.is_empty() {
                    String::new()
                } else {
                    format!(" (lines {})", hits.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "))
                }
            ));
        }
    }
    // assertion C: every key names a declaring gate, and sits on a row's first line
    let keyed: Vec<(usize, &String)> = rows.iter().flat_map(|r| r.first_line_keys.iter().map(move |k| (r.line, k))).collect();
    for (n, k) in &keyed {
        if !declaring.iter().any(|d| d.name == **k) {
            findings.push(format!(
                "{}:{}: key '{}' names no registered gate declaring '{}'",
                roster,
                n,
                k,
                registry::PROJECTION
            ));
        }
    }
    for (n, k) in &stray {
        findings.push(format!(
            "{}:{}: key '{}' is not on a top-level bullet's first line, so it keys no row",
            roster, n, k
        ));
    }

    if !findings.is_empty() {
        println!("check-projection-roster: {} roster finding(s):", findings.len());
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: give each gate declaring '{} <globs>' exactly one top-level bullet in", registry::PROJECTION);
        println!("        the roster section carrying '{} <gate> {}' on its first line, drop a key", KEY_OPEN, KEY_CLOSE);
        println!("        whose gate no longer declares, and keep every projection glob inside the");
        println!("        gate's own couples= (gate-sdk/SPEC.md §check-projection-roster).");
        return Ok(1);
    }
    println!(
        "PROJECTION-ROSTER: clean ({} gate(s) declaring a projection, {} keyed row key(s) in {}; every declaration covered by its couples= and keyed exactly once)",
        declaring.len(),
        keyed.len(),
        roster
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-projection-roster — one line may key several gates, and a key with no
    // close is still read
    #[test]
    fn a_line_carries_every_key_in_order() {
        assert_eq!(
            keys_on("- **x** <!-- projection: check-a --> <!-- projection: check-b -->"),
            vec!["check-a".to_string(), "check-b".to_string()]
        );
        assert_eq!(keys_on("- plain row"), Vec::<String>::new());
        assert_eq!(keys_on("<!-- projection: check-c"), vec!["check-c".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §check-projection-roster — the section closes at a heading of its level or
    // shallower, a deeper heading stays inside, and an absent heading is None
    #[test]
    fn the_section_is_bounded_by_its_own_level() {
        let lines = vec!["# t", "## Roster", "- a", "### deeper", "- b", "## Next", "- c"];
        let got: Vec<&str> = section_lines(&lines, "Roster").unwrap().iter().map(|(_, l)| *l).collect();
        assert_eq!(got, vec!["- a", "### deeper", "- b"]);
        assert!(section_lines(&lines, "Absent").is_none());
        assert_eq!(section_lines(&lines, "").unwrap().len(), lines.len());
    }

    // spec: gate-sdk/SPEC.md §check-projection-roster — assertion A reads the field's one matcher, and
    // refuses a prefixed token and a second line
    #[test]
    fn a_projection_glob_is_covered_by_its_own_couples() {
        let d = |lines: &[&str], couples: &[&str]| Declaring {
            name: "check-x".to_string(),
            lines: lines.iter().map(|s| s.to_string()).collect(),
            couples: couples.iter().map(|s| s.to_string()).collect(),
        };
        assert!(declaration_findings(&d(&["docs/x.md"], &["docs/*.md"])).is_empty());
        assert!(declaration_findings(&d(&["docs/x.md"], &["*docs/x.md"])).is_empty());
        assert!(declaration_findings(&d(&["docs/y.md"], &["docs/x.md"]))[0].contains("not covered"));
        assert!(declaration_findings(&d(&["knob:X"], &["knob:X"]))[0].contains("couples prefix"));
        assert!(declaration_findings(&d(&["a", "b"], &["*"]))[0].contains("2 '# projection:' lines"));
        assert!(declaration_findings(&d(&[""], &["*"]))[0].contains("no output path"));
    }
}
