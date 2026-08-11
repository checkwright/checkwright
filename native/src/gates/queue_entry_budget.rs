// spec: queue-kit/SPEC.md §check-queue-entry-budget — a deferred entry is a costed filing:
// bounded above so it is not an inlined amendment, bounded below so it is not a flag-and-skip;
// an icebox entry is its lead line and nothing else
use crate::queue;

const COST_MARK: &str = "**Cost while deferred";

struct Open {
    slug: String,
    start: usize,
    ind: usize,
    sec: Sec,
    costed: bool,
    nb: usize,
}

#[derive(PartialEq, Clone, Copy)]
enum Sec {
    Other,
    Deferred,
    Icebox,
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
                let n = bound - o.start;
                match o.sec {
                    Sec::Deferred => {
                        if n > cap {
                            size.push(format!(
                                "{}:{}: {} — {} lines (cap {})",
                                file, o.start, o.slug, n, cap
                            ));
                        }
                        if o.ind == 0 && !o.costed {
                            cost.push(format!("{}:{}: {}", file, o.start, o.slug));
                        }
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
            for o in open.iter_mut() {
                o.nb += 1;
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
        println!("        an unanswered ground is relocated to a linked entry, and that split is");
        println!("        authorization-gated, not self-served (queue-kit/SPEC.md");
        println!("        section check-queue-entry-budget).");
        return 1;
    }

    println!(
        "QUEUE-ENTRY-BUDGET: clean (every {} entry within {} lines and carrying a cost field in {})",
        sec_cfg.deferred, cap, file
    );
    0
}
