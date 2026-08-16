// spec: context-kit/SPEC.md §The brevity gate — an over-budget bullet in the budgeted
// always-loaded section that admits its detail lives elsewhere
use crate::ere::Ere;
use crate::proc;
use crate::section;
use crate::walk;

// spec: context-kit/SPEC.md §The brevity gate — the bullet's name, the lead-in stripped and the
// bold run closed; a bullet whose bold never closes keeps the rest of its line
fn bullet_name(line: &str) -> &str {
    let rest = line.strip_prefix("- **").unwrap_or(line);
    match rest.find("**") {
        Some(at) => &rest[..at],
        None => rest,
    }
}

struct Bullet {
    name: String,
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
        name: bullet_name(lines[at]).to_string(),
        span,
        pointer: pointer_re.is_match(&body),
        exempt: lines[at].contains("brevity-exempt")
            || lines[at - 1].contains("brevity-exempt"),
    }
}

pub fn run(args: &[String]) -> i32 {
    let knob = |name: &str| walk::knob_scalar(name);

    let section_name = match knob("CONTEXT_KIT_BREVITY_SECTION") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-brevity: {}", e);
            return 2;
        }
    };
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
            let completed = match proc::run("git", &["rev-parse", "--show-toplevel"]) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("check-brevity: {}", e);
                    return 2;
                }
            };
            match completed.stdout() {
                Some(out) => format!("{}/{}", String::from_utf8_lossy(out).trim(), file),
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
    let found = section::sections(&lines, &section_name);
    if found.is_empty() {
        eprintln!(
            "check-brevity: no heading matches CONTEXT_KIT_BREVITY_SECTION in {}: '{}'",
            brevity_file, section_name
        );
        eprintln!(
            "check-brevity: help: a renamed or deleted section silently disarms this gate — \
             repoint CONTEXT_KIT_BREVITY_SECTION at the live heading, or restore the heading it names"
        );
        return 2;
    }

    let mut total = 0usize;
    let mut within = 0usize;
    let mut findings: Vec<String> = Vec::new();
    for sec in &found {
        let body = &lines[sec.start..sec.end];
        for item in section::items(body, |l| l.starts_with("- **")) {
            let b = measure(&lines, sec.start + item.start, sec.start + item.end, &pointer_re);
            total += 1;
            if b.span <= budget {
                within += 1;
            }
            if b.span > budget && b.pointer && !b.exempt {
                findings.push(format!(
                    "{} — {} lines AND cites a deeper doc (over the {}-line budget while \
                     admitting its detail lives elsewhere)",
                    b.name, b.span, budget
                ));
            }
        }
    }

    if !findings.is_empty() {
        println!(
            "BREVITY: {} bullet(s) over budget in '{}':",
            findings.len(),
            section_name
        );
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
    println!("BREVITY: clean ({} bullets, {} within budget)", total, within);
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
        assert_eq!(b.name, "a");
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
    fn the_name_is_the_bold_run_and_survives_an_unclosed_one() {
        assert_eq!(bullet_name("- **Name** — body"), "Name");
        assert_eq!(bullet_name("- **Name"), "Name");
    }
}
