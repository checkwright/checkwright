// spec: queue-kit/SPEC.md §check-queue-entry-budget — a deferred entry is a costed filing:
// bounded above so it is not an inlined amendment, bounded below so it is not a flag-and-skip,
// bounded in what it may displace; an icebox entry is its lead line and nothing else
use crate::queue::{self, Unit};

const COST_MARK: &str = "**Cost while deferred";

struct Open {
    slug: String,
    start: usize,
    ind: usize,
    sec: Sec,
    costed: bool,
    nb: usize,
    decls: u32,
    cp: usize,
    cpn: usize,
    credits: Vec<String>,
    marks: queue::DeferMarks,
}

impl Open {
    fn count_cp(&mut self, line: &str) {
        let t = line.trim();
        if !t.is_empty() {
            self.cp += t.chars().count();
            self.cpn += 1;
        }
    }
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — the active sections are uncapped, so no
// assertion here reads `Active`; the walk still measures those entries because a reader of one
// entry's history follows it across a promotion, and the alternative is a second walk
#[derive(PartialEq, Clone, Copy)]
pub enum Sec {
    Other,
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
    pub ind: usize,
    pub sec: Sec,
    pub costed: bool,
    pub nb: usize,
    pub count: usize,
    pub cp: usize,
    pub decls: u32,
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
const DECLARATIONS: [(&str, usize); 2] = [("recurrence:", 3), ("not-icebox-eligible:", 3)];

// spec: queue-kit/SPEC.md §check-queue-entry-budget — assertion (D): a body line led by a
// retired declaration token is refused, provenance being stated inline
const RETIRED: [&str; 1] = ["ruled:"];

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

// spec: queue-kit/SPEC.md §check-queue-entry-budget — the scan, split from the verdict so a reader
// of the same quantity measures with the assertion's own walk rather than a second spelling of it;
// entries come back in close order, which is the order the verdict's findings are reported in
pub fn walk(text: &str, sec_cfg: &queue::Sections) -> Scan {
    let mut out: Vec<Closed> = Vec::new();
    let mut retired: Vec<(usize, String, String)> = Vec::new();
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
                // spec: queue-kit/SPEC.md §check-queue-entry-budget — the count is the extent less
                // at most one line of each declaration grammar per entry
                let count = bound - o.start - o.decls.count_ones() as usize;
                out.push(Closed {
                    slug: o.slug,
                    start: o.start,
                    ind: o.ind,
                    sec: o.sec,
                    costed: o.costed,
                    nb: o.nb,
                    count,
                    cp: o.cp + o.cpn.saturating_sub(1),
                    decls: o.decls,
                    dated: o.marks.defer_date().is_some(),
                    credits: o.credits,
                });
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
            } else if sec_cfg.is_task(line) {
                Sec::Active
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
                        o.count_cp(line);
                    }
                    if let Some(o) = open.last_mut() {
                        o.marks.observe(line);
                    }
                }
                Some(slug) => {
                    let slug = slug.to_string();
                    for o in open.iter_mut() {
                        o.nb += 1;
                        o.count_cp(line);
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
                        cp: line.trim().chars().count(),
                        cpn: 1,
                        credits: queue::field_tags(line, "cap-credit").iter().map(|t| t.raw.to_string()).collect(),
                        marks: queue::DeferMarks::default(),
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
            // spec: queue-kit/SPEC.md §check-queue-entry-budget — assertion (D) binds the deferred
            // and icebox tiers, the corpus it has always scanned; the walk's reach into the active
            // sections is for measurement alone and must not widen an assertion's corpus with it
            if sec != Sec::Active {
                if let Some(tok) = line.split_whitespace().next() {
                    if RETIRED.contains(&tok) {
                        let slug = open.last().map(|o| o.slug.clone()).unwrap_or_default();
                        retired.push((fnr, slug, tok.to_string()));
                    }
                }
            }
            // spec: queue-kit/SPEC.md §check-queue-entry-budget — assertion (E) reads a mark into the
            // innermost open entry alone, the scoping the icebox-candidates arm gives the same parse
            if let Some(o) = open.last_mut() {
                o.marks.observe(line);
            }
            let decl = declaration(line).map_or(0, |i| 1u32 << i);
            // spec: queue-kit/SPEC.md §check-queue-entry-budget — the first line of each declaration
            // grammar in an entry is the discounted one, in either unit
            for o in open.iter_mut() {
                o.nb += 1;
                if o.decls & decl == 0 && decl != 0 {
                    o.decls |= decl;
                } else {
                    o.count_cp(line);
                }
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
    Scan {
        entries: out,
        retired,
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
        .map(|(fnr, slug, tok)| format!("{}:{}: {} — retired declaration grammar {}", file, fnr, slug, tok))
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
                            "{}:{}: {} — {}{} (cap {}{}{}){}",
                            file,
                            o.start,
                            o.slug,
                            n,
                            unit.suffix(),
                            c,
                            unit.suffix(),
                            credited,
                            discounted(o.decls)
                        ));
                    }
                    headroom.push((o.start, o.slug.clone(), limit.saturating_sub(n), granted));
                }
                if o.ind == 0 && !o.costed {
                    cost.push(format!("{}:{}: {}", file, o.start, o.slug));
                }
                if o.ind == 0 && !o.dated {
                    undated.push(format!("{}:{}: {}", file, o.start, o.slug));
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
            Sec::Active | Sec::Other => {}
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
            println!("retired declaration line (state who ruled, when and through what channel");
            println!("inline beside the ruling's content, then delete the line):");
            for x in &retired {
                println!("  {}", x);
            }
        }
        println!("  help: add the cost field, or evict the entry to the icebox as a one-line lead.");
        println!("        Over the cap: compress by ANSWERING grounds, never by dropping them —");
        println!("        an unanswered ground is relocated to a linked entry. Relocating it into");
        println!("        an entry that ALREADY owns its subject is self-served only for a");
        println!("        mandated write; minting a NEW entry to hold it stays authorization-");
        println!("        gated (queue-kit/SPEC.md section check-queue-entry-budget, which");
        println!("        defines the class and owns the declaration-line discount above). A");
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
    if f.len() < 4 || !is_iso_date(f[1]) {
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
    // code-point size by nothing, and one line of each declaration grammar is discounted
    #[test]
    fn the_code_point_size_is_reflow_invariant_and_discounts_one_declaration() {
        let wrapped = entry("## Deferred\n\n- **e** — alpha beta\n  gamma delta\n  recurrence: e 2026-01-01\n");
        let joined = entry("## Deferred\n\n- **e** — alpha beta gamma delta\n  recurrence: e 2026-01-01\n");
        assert_eq!(wrapped.cp, joined.cp);
        assert_eq!(joined.cp, "- **e** — alpha beta gamma delta".chars().count());
        assert_eq!(wrapped.count, 2);
    }

    #[test]
    fn a_credit_stands_only_when_granted_bounded_in_unit_and_needed() {
        let text = "## Deferred\n\n- **e** [cap-credit: +10cp 2026-09-22 lead keeps a ground] — xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n";
        let e = entry(text);
        let cap = Some((20, Unit::Cp));
        assert!(e.cp > 20);
        assert_eq!(credit_verdict(&e, cap, Some((50, Unit::Cp))), Ok(10));
        assert!(credit_verdict(&e, cap, Some((5, Unit::Cp))).is_err());
        assert!(credit_verdict(&e, cap, None).is_err());
        assert!(credit_verdict(&e, None, Some((50, Unit::Cp))).is_err());
        assert!(credit_verdict(&e, Some((20, Unit::Lines)), Some((50, Unit::Lines))).is_err());
        assert!(credit_verdict(&e, Some((10_000, Unit::Cp)), Some((50, Unit::Cp))).unwrap_err().starts_with("stale"));
        let bare = entry("## Deferred\n\n- **e** [cap-credit: +10cp] — x\n");
        assert!(credit_verdict(&bare, cap, Some((50, Unit::Cp))).unwrap_err().starts_with("malformed"));
    }
}
