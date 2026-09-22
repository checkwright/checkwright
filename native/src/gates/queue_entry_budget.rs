// spec: queue-kit/SPEC.md §check-queue-entry-budget — a deferred entry is a costed filing:
// bounded above so it is not an inlined amendment, bounded below so it is not a flag-and-skip,
// bounded in what it may displace; an icebox entry is its heading and one sentence
use crate::queue::{self, Unit};

const COST_MARK: &str = "**Cost while deferred";

// spec: queue-kit/SPEC.md §check-queue-entry-budget — the active sections are uncapped, so no
// assertion here reads `Active`; the walk still measures those entries because a reader of one
// entry's history follows it across a promotion, and the alternative is a second walk
#[derive(PartialEq, Clone, Copy)]
pub enum Sec {
    Active,
    Deferred,
    Icebox,
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — one closed entry as the walk measured it.
// `count` (lines) and `cp` (code points) are assertion A's own quantities, so the entry-history
// and queue-index arms read the cap's measure rather than minting a second spelling of it.
pub struct Closed {
    pub slug: String,
    pub start: usize,
    pub level: usize,
    pub sec: Sec,
    pub costed: bool,
    pub nb: usize,
    pub tagged: bool,
    pub count: usize,
    pub cp: usize,
    pub dated: bool,
    pub credits: Vec<String>,
}

impl Closed {
    // spec: queue-kit/SPEC.md §check-queue-entry-budget — the size in the cap's unit
    pub fn size(&self, unit: Unit) -> usize {
        match unit {
            Unit::Cp => self.cp,
            Unit::Lines => self.count,
        }
    }
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — the cap's unit, and the unit a reader
// prints in: under `off` a size still has a reading, and it is code points
pub fn cap() -> Result<Option<(usize, Unit)>, String> {
    let raw = queue::knob_scalar("QUEUE_KIT_ENTRY_CAP")?;
    queue::parse_size(&raw).map_err(|e| format!("QUEUE_KIT_ENTRY_CAP: {}", e))
}

pub fn display_unit(cap: Option<(usize, Unit)>) -> Unit {
    cap.map_or(Unit::Cp, |(_, u)| u)
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — one pass answers both shapes the gate reads:
// closed entries, which carry assertions (A)-(C) and (E), and the assertion-(D) lines, which are a
// property of a *line* and not of an entry. Two passes would be two spellings of the same walk.
pub struct Scan {
    pub entries: Vec<Closed>,
    pub retired: Vec<(usize, String, String)>,
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — assertion (D): a retired tag on the tag line
// is refused, provenance being stated inline
const RETIRED: [&str; 1] = ["ruled"];

// spec: queue-kit/SPEC.md §check-queue-entry-budget — the scan, split from the verdict so a reader
// of the same quantity measures with the assertion's own walk rather than a second spelling of it;
// entries come back in file order, which is the order the verdict's findings are reported in. The
// size is the whole extent, heading and tag line included: nothing is discounted.
pub fn walk(text: &str, sec_cfg: &queue::Sections) -> Scan {
    let lines: Vec<&str> = text.lines().collect();
    let mut entries: Vec<Closed> = Vec::new();
    let mut retired: Vec<(usize, String, String)> = Vec::new();
    for e in queue::entries(&lines, sec_cfg) {
        let sec = if sec_cfg.is_deferred(&e.section) {
            Sec::Deferred
        } else if sec_cfg.is_icebox(&e.section) {
            Sec::Icebox
        } else {
            Sec::Active
        };
        let extent = &lines[e.start..e.end];
        let content: Vec<&str> = extent.iter().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
        let cp = content.iter().map(|l| l.chars().count()).sum::<usize>() + content.len().saturating_sub(1);
        let tags = e.tags(&lines);
        if sec != Sec::Active {
            for name in RETIRED {
                if !queue::field_tags(tags, name).is_empty() {
                    retired.push((e.tag_line.unwrap_or(e.start) + 1, e.slug.clone(), format!("[{}:]", name)));
                }
            }
        }
        // spec: queue-kit/SPEC.md §check-queue-entry-budget — assertion (E) reads the entry's own
        // body, up to its first sub-task, the scoping the icebox-candidates arm gives the same parse
        let mut marks = queue::DeferMarks::default();
        for i in e.body_lines().take_while(|i| queue::heading_level(lines[*i]).is_none()) {
            marks.observe(lines[i]);
        }
        entries.push(Closed {
            slug: e.slug.clone(),
            start: e.start + 1,
            level: e.level,
            sec,
            costed: extent.iter().any(|l| l.contains(COST_MARK)),
            nb: content.len() - 1,
            tagged: e.tag_line.is_some(),
            count: e.end - e.start,
            cp,
            dated: marks.defer_date().is_some(),
            credits: queue::field_tags(tags, "cap-credit").iter().map(|t| t.raw.to_string()).collect(),
        });
    }
    Scan { entries, retired }
}

pub fn run(args: &[String]) -> i32 {
    let sec_cfg = match queue::Sections::active_and_deferred() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-queue-entry-budget: {}", e);
            return 2;
        }
    };
    let cap = match cap() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-entry-budget: {}", e);
            return 2;
        }
    };
    let max = match queue::knob_scalar("QUEUE_KIT_ENTRY_CREDIT_MAX")
        .and_then(|raw| queue::parse_size(&raw).map_err(|e| format!("QUEUE_KIT_ENTRY_CREDIT_MAX: {}", e)))
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-entry-budget: {}", e);
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

    let scan = walk(&text, &sec_cfg);
    let (mut size, mut cost, mut shape, mut undated) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    // spec: queue-kit/SPEC.md §check-queue-entry-budget — headroom is the size
    // assertion's own count one subtraction away, collected for every closed
    // Deferred entry regardless of cap outcome and surfaced only on the clean path
    let mut headroom: Vec<(usize, String, usize, usize)> = Vec::new();
    let mut credit: Vec<String> = Vec::new();
    let retired: Vec<String> = scan
        .retired
        .iter()
        .map(|(fnr, slug, tok)| format!("{}:{}: {} — retired tag {}", file, fnr, slug, tok))
        .collect();
    for o in &scan.entries {
        match o.sec {
            Sec::Deferred => {
                let granted = match credit_verdict(o, cap, max) {
                    Ok(g) => g,
                    Err(why) => {
                        credit.push(format!("{}:{}: {} — {}", file, o.start, o.slug, why));
                        0
                    }
                };
                if let Some((c, unit)) = cap {
                    let n = o.size(unit);
                    let limit = c + granted;
                    if n > limit {
                        let credited = if granted > 0 { format!(" + credit {}{}", granted, unit.suffix()) } else { String::new() };
                        size.push(format!(
                            "{}:{}: {} — {}{} (cap {}{}{})",
                            file,
                            o.start,
                            o.slug,
                            n,
                            unit.suffix(),
                            c,
                            unit.suffix(),
                            credited,
                        ));
                    }
                    headroom.push((o.start, o.slug.clone(), limit.saturating_sub(n), granted));
                }
                if o.level == 3 && !o.costed {
                    cost.push(format!("{}:{}: {}", file, o.start, o.slug));
                }
                if o.level == 3 && !o.dated {
                    undated.push(format!("{}:{}: {}", file, o.start, o.slug));
                }
            }
            Sec::Icebox => {
                if o.nb != 1 || o.tagged {
                    shape.push(format!(
                        "{}:{}: {} — {} content line(s){}; an icebox entry is its heading and exactly one sentence",
                        file,
                        o.start,
                        o.slug,
                        o.nb,
                        if o.tagged { " and a tag line" } else { "" }
                    ));
                }
            }
            Sec::Active => {}
        }
    }

    if !size.is_empty() || !credit.is_empty() || !cost.is_empty() || !shape.is_empty() || !retired.is_empty() || !undated.is_empty() {
        println!("check-queue-entry-budget: deferred-pool entry budget violation(s):");
        println!();
        if !size.is_empty() {
            println!("over the per-entry size cap (a body that long is an amendment inlined where");
            println!("the amendment gates cannot see it):");
            for x in &size {
                println!("  {}", x);
            }
        }
        if !credit.is_empty() {
            println!("a [cap-credit:] that does not stand (a credit is granted, bounded, and deleted");
            println!("once the entry fits the cap without it):");
            for x in &credit {
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
        if !undated.is_empty() {
            println!("no defer date (neither a 'Surfaced <date>' nor a 'Filed <date>' mark, label");
            println!("and date on one body line — an undated entry never ages out and no KPI sees it):");
            for x in &undated {
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
        if !retired.is_empty() {
            println!("retired tag (state who ruled, when and through what channel inline beside");
            println!("the ruling's content, then delete the tag):");
            for x in &retired {
                println!("  {}", x);
            }
        }
        println!("  help: add the cost field, or evict the entry to the icebox as its heading and one sentence.");
        println!("        Over the cap: compress by ANSWERING grounds, never by dropping them —");
        println!("        an unanswered ground is relocated to a linked entry. Relocating it into");
        println!("        an entry that ALREADY owns its subject is self-served only for a");
        println!("        mandated write; minting a NEW entry to hold it stays authorization-");
        println!("        gated (queue-kit/SPEC.md section check-queue-entry-budget, which");
        println!("        defines the class). A");
        println!("        record whose compression would lose what its reader needs may be");
        println!("        granted a [cap-credit:] by the same authority; a session never takes one.");
        // spec: queue-kit/SPEC.md §check-queue-entry-budget — the arm's named reader is this
        // failure's reader, who is by construction the session about to compress; it is routed
        // here because no other trigger reaches that session at that moment
        println!("        Before compressing, read what has already left the entry:");
        println!("        bash gate-sdk/bin/run-gates.sh --emit entry-history <slug> — the commits");
        println!("        in which its counted extent fell. Advisory, no verdict: it is how you");
        println!("        avoid re-answering an answered ground or dropping an unanswered one.");
        return 1;
    }

    let Some((c, unit)) = cap else {
        println!(
            "QUEUE-ENTRY-BUDGET: clean (size cap off; every {} entry carrying a cost field and resolving a defer date in {})",
            sec_cfg.deferred, file
        );
        return 0;
    };
    let u = unit.suffix();
    println!(
        "QUEUE-ENTRY-BUDGET: clean (every {} entry within {}{}, carrying a cost field and resolving a defer date in {})",
        sec_cfg.deferred, c, u, file
    );
    if !headroom.is_empty() {
        println!();
        println!("headroom under the {}{} cap, per entry:", c, u);
        headroom.sort_by_key(|(start, _, _, _)| *start);
        for (_, slug, h, granted) in &headroom {
            if *granted > 0 {
                println!("  {}: {}{} of headroom (credit +{}{})", slug, h, u, granted, u);
            } else {
                println!("  {}: {}{} of headroom", slug, h, u);
            }
        }
    }
    0
}

// spec: queue-kit/SPEC.md §The tag algebra — `[cap-credit: +<n><unit> <YYYY-MM-DD> <grantor>
// <reason>]`: the amount, a date, a one-token grantor and a non-empty reason
fn credit_grant(raw: &str) -> Result<(usize, Unit), String> {
    let f: Vec<&str> = raw.split_whitespace().collect();
    let shape = "malformed [cap-credit:] (want +<n><unit> <YYYY-MM-DD> <grantor> <reason>)";
    if f.len() < 4 || !queue::is_iso_date(f[1]) {
        return Err(shape.to_string());
    }
    match f[0].strip_prefix('+').map(queue::parse_size) {
        Some(Ok(Some(g))) => Ok(g),
        _ => Err(shape.to_string()),
    }
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — a deferred entry's credit, in the cap's unit:
// `Ok(0)` where it carries none, the amount where it stands, and the reason where it is red
fn credit_verdict(o: &Closed, cap: Option<(usize, Unit)>, max: Option<(usize, Unit)>) -> Result<usize, String> {
    let raw = match o.credits.as_slice() {
        [] => return Ok(0),
        [one] => one,
        _ => return Err("more than one [cap-credit:] (at most one per entry)".to_string()),
    };
    let (n, unit) = credit_grant(raw)?;
    let Some((c, cap_unit)) = cap else {
        return Err("a credit under QUEUE_KIT_ENTRY_CAP=off (there is no cap to credit)".to_string());
    };
    if unit != cap_unit {
        return Err(format!("credit in {} but the cap is in {}", unit.suffix(), cap_unit.suffix()));
    }
    match max {
        None => return Err("credits are off (QUEUE_KIT_ENTRY_CREDIT_MAX=off)".to_string()),
        Some((_, mu)) if mu != cap_unit => {
            return Err(format!(
                "QUEUE_KIT_ENTRY_CREDIT_MAX is in {} but the cap is in {}, so it admits no credit",
                mu.suffix(),
                cap_unit.suffix()
            ))
        }
        Some((m, _)) if n > m => {
            return Err(format!("credit +{}{} exceeds QUEUE_KIT_ENTRY_CREDIT_MAX ({}{})", n, unit.suffix(), m, unit.suffix()))
        }
        Some(_) => {}
    }
    let size = o.size(cap_unit);
    if size <= c {
        return Err(format!(
            "stale credit: the entry measures {}{}, within the {}{} cap without it — delete the tag",
            size,
            unit.suffix(),
            c,
            unit.suffix()
        ));
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sections() -> queue::Sections {
        queue::Sections {
            active: vec!["New Features".to_string()],
            deferred: "Deferred".to_string(),
            icebox: String::new(),
            done: String::new(),
        }
    }

    fn entry(text: &str) -> Closed {
        walk(text, &sections()).entries.into_iter().find(|e| e.slug == "e").expect("entry")
    }

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — a reflow of the same text moves the
    // code-point size by nothing, and the whole extent counts, heading and tag line included
    #[test]
    fn the_code_point_size_is_reflow_invariant_and_counts_the_whole_extent() {
        let wrapped = entry("## Deferred\n\n### e\n\n[recurrence: 2026-01-01]\n\nalpha beta\ngamma delta\n");
        let joined = entry("## Deferred\n\n### e\n\n[recurrence: 2026-01-01]\n\nalpha beta gamma delta\n");
        assert_eq!(wrapped.cp, joined.cp);
        assert_eq!(joined.cp, "### e [recurrence: 2026-01-01] alpha beta gamma delta".chars().count());
        assert_eq!(joined.count, 5);
    }

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — assertion B's shape: the heading and one
    // sentence, no tag line
    #[test]
    fn an_icebox_entry_is_its_heading_and_one_sentence() {
        let sec = queue::Sections {
            active: vec!["New Features".to_string()],
            deferred: "Deferred".to_string(),
            icebox: "Icebox".to_string(),
            done: String::new(),
        };
        let shape = |t: &str| {
            let e = walk(t, &sec).entries.remove(0);
            (e.nb, e.tagged)
        };
        assert_eq!(shape("## Icebox\n\n### e\n\nOne sentence.\n"), (1, false));
        assert_eq!(shape("## Icebox\n\n### e\n\n[cost: once/low]\n\nOne sentence.\n"), (2, true));
        assert_eq!(shape("## Icebox\n\n### e\n\nOne.\n\nTwo.\n"), (2, false));
    }

    #[test]
    fn a_credit_stands_only_when_granted_bounded_in_unit_and_needed() {
        let text = "## Deferred\n\n### e\n\n[cap-credit: +10cp 2026-09-22 lead keeps a ground]\n\nxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n";
        let e = entry(text);
        let cap = Some((20, Unit::Cp));
        assert!(e.cp > 20);
        assert_eq!(credit_verdict(&e, cap, Some((50, Unit::Cp))), Ok(10));
        assert!(credit_verdict(&e, cap, Some((5, Unit::Cp))).is_err());
        assert!(credit_verdict(&e, cap, None).is_err());
        assert!(credit_verdict(&e, None, Some((50, Unit::Cp))).is_err());
        assert!(credit_verdict(&e, Some((20, Unit::Lines)), Some((50, Unit::Lines))).is_err());
        assert!(credit_verdict(&e, Some((10_000, Unit::Cp)), Some((50, Unit::Cp))).unwrap_err().starts_with("stale"));
        let bare = entry("## Deferred\n\n### e\n\n[cap-credit: +10cp]\n\nx\n");
        assert!(credit_verdict(&bare, cap, Some((50, Unit::Cp))).unwrap_err().starts_with("malformed"));
    }
}
