// spec: context-kit/SPEC.md §The brevity gate — an over-budget bullet in a governed always-loaded
// section that admits its detail lives elsewhere
use crate::ere::Ere;
use crate::section;
use crate::walk;

// spec: context-kit/SPEC.md §The brevity gate — the lead's width when a bullet opens without a bold
// run, counted in characters so the cut never splits one
const LEAD_WIDTH: usize = 48;

// spec: context-kit/SPEC.md §The brevity gate — the bullet's lead: the bold run where the bullet
// opens with one (a run that never closes keeps the rest of its line), else its lead line's opening
// text
fn bullet_lead(line: &str) -> String {
    if let Some(rest) = line.strip_prefix("- **") {
        return match rest.find("**") {
            Some(at) => rest[..at].to_string(),
            None => rest.to_string(),
        };
    }
    let rest = line.strip_prefix("- ").unwrap_or(line);
    let mut chars = rest.chars();
    let head: String = chars.by_ref().take(LEAD_WIDTH).collect();
    if chars.next().is_some() {
        format!("{}…", head)
    } else {
        head
    }
}

struct Bullet {
    lead: String,
    span: usize,
    pointer: bool,
    exempt: bool,
}

// spec: context-kit/SPEC.md §The brevity gate — the span is measured to the final line carrying
// content, so a trailing blank before the next bullet never inflates the count
fn measure(lines: &[&str], at: usize, end: usize, pointer_re: &Ere) -> Bullet {
    let mut span = 1usize;
    let mut body = String::from(lines[at]);
    for (offset, line) in lines[at + 1..end].iter().enumerate() {
        body.push(' ');
        body.push_str(line);
        if !section::blank(line) {
            span = offset + 2;
        }
    }
    Bullet {
        lead: bullet_lead(lines[at]),
        span,
        pointer: pointer_re.is_match(&body),
        exempt: lines[at].contains("brevity-exempt")
            || lines[at - 1].contains("brevity-exempt"),
    }
}

// spec: context-kit/SPEC.md §The brevity gate — every top-level list item is measured, so a bullet
// needs no bold name to be in reach; an indented item belongs to its parent's extent
fn is_item(line: &str) -> bool {
    line.starts_with("- ")
}

// spec: context-kit/SPEC.md §The brevity gate — a repeated element is scanned once, so a duplicated
// heading in config cannot double a finding or the clean line's count
fn distinct(names: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for n in names {
        if !out.contains(&n) {
            out.push(n);
        }
    }
    out
}

pub fn run(args: &[String]) -> i32 {
    let knob = |name: &str| walk::knob_scalar(name);

    let section_names = match walk::knob_array("CONTEXT_KIT_BREVITY_SECTIONS") {
        Ok(v) => distinct(v),
        Err(e) => {
            eprintln!("check-brevity: {}", e);
            return 2;
        }
    };
    if section_names.is_empty() {
        eprintln!("check-brevity: CONTEXT_KIT_BREVITY_SECTIONS is empty — the gate governs nothing");
        eprintln!(
            "check-brevity: help: name at least one heading of the governed file, or unregister \
             check-brevity to opt out of the brevity check"
        );
        return 2;
    }
    let budget: usize = match knob("CONTEXT_KIT_BREVITY_BUDGET") {
        Ok(v) => match v.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("check-brevity: CONTEXT_KIT_BREVITY_BUDGET is not an integer: {}", v);
                return 2;
            }
        },
        Err(e) => {
            eprintln!("check-brevity: {}", e);
            return 2;
        }
    };
    let pointer_re = match knob("CONTEXT_KIT_BREVITY_POINTER_RE") {
        Ok(v) => match Ere::compile(&v) {
            Ok(re) => re,
            Err(e) => {
                eprintln!("check-brevity: CONTEXT_KIT_BREVITY_POINTER_RE: {}", e);
                return 2;
            }
        },
        Err(e) => {
            eprintln!("check-brevity: {}", e);
            return 2;
        }
    };

    let brevity_file = match args.first() {
        Some(a) => a.clone(),
        None => {
            let file = match knob("CONTEXT_KIT_BREVITY_FILE") {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-brevity: {}", e);
                    return 2;
                }
            };
            // spec: context-kit/SPEC.md §The brevity gate — the knob spells the governed file
            // relative to the repo root, so the default arm resolves that root rather than cwd
            let completed = match walk::toplevel_opt() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("check-brevity: {}", e);
                    return 2;
                }
            };
            match completed {
                Some(top) => format!("{}/{}", top, file),
                None => {
                    eprintln!("check-brevity: not inside a git repository");
                    return 2;
                }
            }
        }
    };

    let text = match std::fs::read_to_string(&brevity_file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-brevity: file not found: {}", brevity_file);
            return 2;
        }
    };

    let lines = section::split_lines(&text);
    let mut resolved: Vec<(&str, Vec<section::Section>)> = Vec::new();
    let mut unmatched: Vec<&str> = Vec::new();
    for name in &section_names {
        let found = section::sections(&lines, name);
        if found.is_empty() {
            unmatched.push(name);
        } else {
            resolved.push((name, found));
        }
    }
    if !unmatched.is_empty() {
        for name in &unmatched {
            eprintln!(
                "check-brevity: no heading matches CONTEXT_KIT_BREVITY_SECTIONS element in {}: '{}'",
                brevity_file, name
            );
        }
        eprintln!(
            "check-brevity: help: a renamed or deleted section silently disarms this gate — \
             repoint the CONTEXT_KIT_BREVITY_SECTIONS element at the live heading, or restore the \
             heading it names"
        );
        return 2;
    }

    let mut total = 0usize;
    let mut within = 0usize;
    let mut counts: Vec<String> = Vec::new();
    let mut findings: Vec<String> = Vec::new();
    let mut scanned: Vec<usize> = Vec::new();
    for (name, found) in &resolved {
        let mut in_section = 0usize;
        for sec in found {
            if scanned.contains(&sec.start) {
                continue;
            }
            scanned.push(sec.start);
            let body = &lines[sec.start..sec.end];
            for item in section::items(body, is_item) {
                let at = sec.start + item.start;
                let b = measure(&lines, at, sec.start + item.end, &pointer_re);
                total += 1;
                in_section += 1;
                if b.span <= budget {
                    within += 1;
                }
                if b.span > budget && b.pointer && !b.exempt {
                    findings.push(format!(
                        "'{}' line {}: {} — {} lines AND cites a deeper doc (over the {}-line \
                         budget while admitting its detail lives elsewhere)",
                        name,
                        at + 1,
                        b.lead,
                        b.span,
                        budget
                    ));
                }
            }
        }
        counts.push(format!("'{}' {}", name, in_section));
    }

    if !findings.is_empty() {
        println!("BREVITY: {} bullet(s) over budget:", findings.len());
        for f in &findings {
            println!("  {}", f);
        }
        println!(
            "  help: cut each to ≤{} lines by pushing detail into the section it already points \
             to, or add <!-- brevity-exempt: <reason> --> on the bullet's first line / the line \
             above if every line is load-bearing",
            budget
        );
        return 1;
    }
    println!(
        "BREVITY: clean ({} bullets, {} within budget; {})",
        total,
        within,
        counts.join(", ")
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn re() -> Ere {
        Ere::compile("§").expect("the default pointer expression failed to compile")
    }

    // spec: context-kit/SPEC.md §The brevity gate — the three-way conjunction is the calibration:
    // dropping any conjunct turns the gate into a length police, so each is measured separately
    #[test]
    fn the_span_stops_at_the_last_line_carrying_content() {
        let lines = section::split_lines("## S\n- **a** — x\ncont\n\n\n");
        let b = measure(&lines, 1, lines.len(), &re());
        assert_eq!(b.span, 2);
        assert_eq!(b.lead, "a");
        assert!(!b.pointer);
    }

    #[test]
    fn the_pointer_expression_reads_the_whole_body_not_the_lead_line() {
        let lines = section::split_lines("## S\n- **a** — x\ndetail at SPEC.md §Thing\n");
        assert!(measure(&lines, 1, lines.len(), &re()).pointer);
    }

    // spec: context-kit/SPEC.md §The brevity gate — the marker is honored on the bullet's own
    // first line or the line above, which for the section's first bullet is its heading
    #[test]
    fn the_exempt_marker_is_read_from_the_bullet_line_or_the_one_above_it() {
        let lines = section::split_lines("## S\n- **a** <!-- brevity-exempt: r -->\nx\n");
        assert!(measure(&lines, 1, lines.len(), &re()).exempt);
        let lines = section::split_lines("<!-- brevity-exempt: r -->\n- **a**\nx\n");
        assert!(measure(&lines, 1, lines.len(), &re()).exempt);
        let lines = section::split_lines("## S\n- **a**\nx\n");
        assert!(!measure(&lines, 1, lines.len(), &re()).exempt);
    }

    #[test]
    fn the_lead_is_the_bold_run_and_survives_an_unclosed_one() {
        assert_eq!(bullet_lead("- **Name** — body"), "Name");
        assert_eq!(bullet_lead("- **Name"), "Name");
    }

    #[test]
    fn a_bullet_without_a_bold_run_leads_with_its_opening_text_cut_on_a_character() {
        assert_eq!(bullet_lead("- `.tmp/` is scratch"), "`.tmp/` is scratch");
        let long = format!("- {}", "é".repeat(LEAD_WIDTH + 3));
        assert_eq!(bullet_lead(&long), format!("{}…", "é".repeat(LEAD_WIDTH)));
    }

    #[test]
    fn every_top_level_item_leads_and_an_indented_one_does_not() {
        assert!(is_item("- plain"));
        assert!(is_item("- **bold**"));
        assert!(!is_item("  - nested"));
        assert!(!is_item("-no space"));
    }

    #[test]
    fn a_repeated_section_name_is_kept_once_in_first_order() {
        let got = distinct(vec!["## B".into(), "## A".into(), "## B".into()]);
        assert_eq!(got, vec!["## B".to_string(), "## A".to_string()]);
    }
}
