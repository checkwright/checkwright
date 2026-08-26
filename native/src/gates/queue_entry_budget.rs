// spec: queue-kit/SPEC.md §check-queue-entry-budget — a deferred entry is a costed filing:
// bounded above so it is not an inlined amendment, bounded below so it is not a flag-and-skip,
// bounded in what it may displace; an icebox entry is its lead line and nothing else
use crate::queue;

const COST_MARK: &str = "**Cost while deferred";

struct Open {
    slug: String,
    start: usize,
    ind: usize,
    sec: Sec,
    costed: bool,
    nb: usize,
    decls: u32,
}

#[derive(PartialEq, Clone, Copy)]
enum Sec {
    Other,
    Deferred,
    Icebox,
}

fn is_iso_date(tok: &str) -> bool {
    let b = tok.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && [0, 1, 2, 3, 5, 6, 8, 9]
            .iter()
            .all(|&i| b[i].is_ascii_digit())
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — at most one line of EACH declaration
// grammar the queue format defines is discounted, each matched by its own grammar: lead token,
// slug, then at least one ISO date past the slug, with no entry-boundary or self-slug condition
const DECLARATIONS: [(&str, usize); 2] = [("recurrence:", 3), ("ruled:", 5)];

fn declaration(line: &str) -> Option<usize> {
    let f: Vec<&str> = line.split_whitespace().collect();
    DECLARATIONS
        .iter()
        .position(|&(tok, min)| f.len() >= min && f[0] == tok && f[2..].iter().any(|t| is_iso_date(t)))
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — the finding names which grammars were
// discounted, so a reader checks the arithmetic against the extent without re-deriving the set
fn discounted(decls: u32) -> String {
    let toks: Vec<&str> = DECLARATIONS
        .iter()
        .enumerate()
        .filter(|(i, _)| decls & (1 << i) != 0)
        .map(|(_, (tok, _))| *tok)
        .collect();
    if toks.is_empty() {
        String::new()
    } else {
        format!(", after discounting one {} line", toks.join(" and one "))
    }
}

fn is_rule(line: &str) -> bool {
    match line.strip_prefix("---") {
        Some(rest) => rest.bytes().all(|b| b == b' ' || b == b'\t'),
        None => false,
    }
}

pub fn run(args: &[String]) -> i32 {
    let sec_cfg = match queue::Sections::active_and_deferred() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-queue-entry-budget: {}", e);
            return 2;
        }
    };
    let cap_raw = match queue::knob_scalar("QUEUE_KIT_ENTRY_LINE_CAP") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-entry-budget: {}", e);
            return 2;
        }
    };
    let cap: usize = match cap_raw.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!(
                "check-queue-entry-budget: QUEUE_KIT_ENTRY_LINE_CAP is not a positive integer: {}",
                cap_raw
            );
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-queue-entry-budget: {}", e);
                return 2;
            }
        },
    };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-queue-entry-budget: file not found: {}", file);
            return 2;
        }
    };

    let (mut size, mut cost, mut shape) = (Vec::new(), Vec::new(), Vec::new());
    // spec: queue-kit/SPEC.md §check-queue-entry-budget — headroom is the size
    // assertion's own count one subtraction away, collected for every closed
    // Deferred entry regardless of cap outcome and surfaced only on the clean path
    let mut headroom: Vec<(usize, String, usize)> = Vec::new();
    let mut open: Vec<Open> = Vec::new();
    let mut sec = Sec::Other;
    let mut bound;
    // spec: queue-kit/SPEC.md §check-queue-entry-budget — closing to depth 0 closes every open
    // entry, the shape a heading and end-of-file both need
    let all = 0usize;

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — an extent runs from the lead line to
    // the line before the next bullet at the same or shallower indent; a sub-task nests inside
    // its parent and is measured as its own entry too
    macro_rules! close_to {
        ($ind:expr) => {{
            while let Some(o) = open.last() {
                if o.ind < $ind {
                    break;
                }
                let o = open.pop().unwrap();
                match o.sec {
                    Sec::Deferred => {
                        // spec: queue-kit/SPEC.md §check-queue-entry-budget — the count is the
                        // extent less at most one line of each declaration grammar per entry
                        let n = bound - o.start - o.decls.count_ones() as usize;
                        if n > cap {
                            size.push(format!(
                                "{}:{}: {} — {} lines (cap {}){}",
                                file,
                                o.start,
                                o.slug,
                                n,
                                cap,
                                discounted(o.decls)
                            ));
                        }
                        if o.ind == 0 && !o.costed {
                            cost.push(format!("{}:{}: {}", file, o.start, o.slug));
                        }
                        headroom.push((o.start, o.slug.clone(), cap.saturating_sub(n)));
                    }
                    Sec::Icebox => {
                        if o.nb > 1 {
                            shape.push(format!(
                                "{}:{}: {} — {} content lines; an icebox entry is exactly one",
                                file, o.start, o.slug, o.nb
                            ));
                        }
                    }
                    Sec::Other => {}
                }
            }
        }};
    }

    let mut last = 0usize;
    for (i, line) in text.lines().enumerate() {
        let fnr = i + 1;
        last = fnr;

        if line.starts_with('#') || is_rule(line) {
            bound = fnr;
            close_to!(all);
        }
        if queue::is_section_line(line) {
            sec = if sec_cfg.is_deferred(line) {
                Sec::Deferred
            } else if sec_cfg.is_icebox(line) {
                Sec::Icebox
            } else {
                Sec::Other
            };
            continue;
        }
        if sec == Sec::Other {
            continue;
        }

        if queue::is_bullet(line) {
            let ind = queue::indent(line);
            bound = fnr;
            close_to!(ind);
            match queue::bullet_slug(line) {
                None => {
                    // spec: queue-kit/SPEC.md §check-queue-entry-budget — a prose-note bullet
                    // is a content line of every entry it sits inside
                    for o in open.iter_mut() {
                        o.nb += 1;
                    }
                }
                Some(slug) => {
                    let slug = slug.to_string();
                    for o in open.iter_mut() {
                        o.nb += 1;
                    }
                    let costed = line.contains(COST_MARK);
                    open.push(Open {
                        slug,
                        start: fnr,
                        ind,
                        sec,
                        costed: false,
                        nb: 1,
                        decls: 0,
                    });
                    if costed {
                        for o in open.iter_mut() {
                            o.costed = true;
                        }
                    }
                }
            }
            continue;
        }

        if !open.is_empty() && !line.trim().is_empty() {
            let decl = declaration(line).map_or(0, |i| 1u32 << i);
            for o in open.iter_mut() {
                o.nb += 1;
                o.decls |= decl;
            }
        }
        if !open.is_empty() && line.contains(COST_MARK) {
            for o in open.iter_mut() {
                o.costed = true;
            }
        }
    }
    bound = last + 1;
    close_to!(all);

    if !size.is_empty() || !cost.is_empty() || !shape.is_empty() {
        println!("check-queue-entry-budget: deferred-pool entry budget violation(s):");
        println!();
        if !size.is_empty() {
            println!("over the per-entry line cap (a body that long is an amendment inlined where");
            println!("the amendment gates cannot see it):");
            for x in &size {
                println!("  {}", x);
            }
        }
        if !cost.is_empty() {
            println!("no 'Cost while deferred' field (a gap you defer is costed and filed, never");
            println!("flagged-and-skipped):");
            for x in &cost {
                println!("  {}", x);
            }
        }
        if !shape.is_empty() {
            println!("icebox entry carrying a body (the tier's whole purpose is minimum residency;");
            println!("membership in it is itself the cost declaration):");
            for x in &shape {
                println!("  {}", x);
            }
        }
        println!("  help: add the cost field, or evict the entry to the icebox as a one-line lead.");
        println!("        Over the cap: compress by ANSWERING grounds, never by dropping them —");
        println!("        an unanswered ground is relocated to a linked entry. Relocating it into");
        println!("        an entry that ALREADY owns its subject is self-served only for a");
        println!("        mandated write; minting a NEW entry to hold it stays authorization-");
        println!("        gated (queue-kit/SPEC.md section check-queue-entry-budget, which");
        println!("        defines the class and owns the declaration-line discount above).");
        return 1;
    }

    println!(
        "QUEUE-ENTRY-BUDGET: clean (every {} entry within {} lines and carrying a cost field in {})",
        sec_cfg.deferred, cap, file
    );
    if !headroom.is_empty() {
        println!();
        println!("headroom under the {}-line cap, per entry:", cap);
        headroom.sort_by_key(|(start, _, _)| *start);
        for (_, slug, h) in &headroom {
            println!("  {}: {} lines of headroom", slug, h);
        }
    }
    0
}
