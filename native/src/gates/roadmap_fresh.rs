// spec: queue-kit/SPEC.md §check-roadmap-fresh — every [roadmap:] tag names a configured horizon
// and track and pairs with exactly one roadmap-summary: declaration, and the projection page's
// marker block is the byte-fresh emission of the roadmap arm
use crate::emit::roadmap;
use crate::fresh;
use crate::marker;
use crate::queue::{self, Sections};
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-roadmap-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let configured = queue::knob_scalar("QUEUE_KIT_ROADMAP_FILE")?;
    let projection = fresh::positional(args, 0, &configured);
    let emit_src = args.get(1).map(String::as_str).unwrap_or("");

    // spec: queue-kit/SPEC.md §check-roadmap-fresh — no configured page is the clean skip for a
    // consumer that publishes no roadmap, matching check-queue-slug-liveness's empty-globs
    // behavior
    if projection.is_empty() {
        println!(
            "ROADMAP-FRESH: clean (QUEUE_KIT_ROADMAP_FILE empty — this consumer publishes no roadmap)"
        );
        return Ok(0);
    }
    let projection = projection.to_string();

    let queue_file = queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?;
    if !Path::new(&queue_file).is_file() {
        return Err(format!("queue file not found: {}", queue_file));
    }

    // spec: queue-kit/SPEC.md §check-roadmap-fresh — assertion B runs first, and that ordering is
    // a correctness property: an unconfigured horizon is silently dropped from the emission, so a
    // freshness verdict taken first would pass a page that quietly lost an item.
    let horizons = queue::knob_array("QUEUE_KIT_HORIZONS")?;
    let tracks = queue::knob_array("QUEUE_KIT_TRACKS")?;
    let sec = Sections::active_and_deferred()?;
    let text = fresh::read_captured(&queue_file)?;
    let entries = queue::roadmap_entries(&text, &sec);

    let mut bad: Vec<String> = Vec::new();
    let mut tagged = 0usize;
    for e in &entries {
        if e.slug.is_empty() {
            continue;
        }
        // spec: queue-kit/SPEC.md §check-roadmap-fresh — assertion C, both directions: the tag
        // decides projection and the declaration is the only prose that may be projected, so
        // neither is meaningful without the other
        if e.tags == 0 {
            bad.push(format!(
                "{}: carries a roadmap-summary: declaration but no [roadmap:] tag; a dead marking \
                 is what a dropped or reflowed tag looks like from the page's side",
                e.slug
            ));
            continue;
        }
        tagged += 1;
        // spec: queue-kit/SPEC.md §check-roadmap-fresh — the declaration count and the tag's
        // fields are independent, so both are reported in one run rather than costing the author
        // a second round trip
        if e.declarations != 1 {
            bad.push(format!(
                "{}: is [roadmap:]-tagged but carries {} roadmap-summary: declaration(s); exactly \
                 one is required",
                e.slug, e.declarations
            ));
        }
        if e.tags != 1 {
            bad.push(format!(
                "{}: carries {} [roadmap:] tags; an entry takes at most one",
                e.slug, e.tags
            ));
            continue;
        }
        let (h, t) = match e.field.split_once('/') {
            Some((h, t)) if !t.contains('/') => (h, t),
            _ => {
                bad.push(format!(
                    "{}: field '{}' does not parse as <horizon>/<track>",
                    e.slug, e.field
                ));
                continue;
            }
        };
        if !horizons.iter().any(|x| x == h) {
            bad.push(format!("{}: unknown horizon '{}'", e.slug, h));
        }
        if !tracks.iter().any(|x| x == t) {
            bad.push(format!("{}: unknown track '{}'", e.slug, t));
        }
    }

    if !bad.is_empty() {
        println!(
            "check-roadmap-fresh: invalid roadmap curation in {}",
            queue_file
        );
        println!("(an unconfigured value or an unpaired marking drops the entry off the page");
        println!("with nothing else to notice):");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: an entry joins the page with a [roadmap: <horizon>/<track>] tag on its lead");
        println!("        line, both values drawn from QUEUE_KIT_HORIZONS / QUEUE_KIT_TRACKS, plus");
        println!("        exactly one indented 'roadmap-summary: <text>' declaration in its body —");
        println!("        the only prose the public page prints (queue-kit/SPEC.md §The roadmap arm).");
        return Ok(1);
    }

    // spec: queue-kit/SPEC.md §check-roadmap-fresh — a configured path with no page, no markers,
    // or a half marker pair is a broken install, not a clean skip
    let name = queue::knob_scalar("QUEUE_KIT_ROADMAP_MARKER")?;
    let (begin, end) = (roadmap::begin(&name), roadmap::end(&name));
    if !Path::new(&projection).is_file() {
        return Err(format!("projection not found: {}", projection));
    }
    let page = fresh::read_captured(&projection)?;
    // spec: queue-kit/SPEC.md §check-roadmap-fresh — the pair count is `grep -cF`'s: a line
    // *containing* the marker counts, deliberately wider than the block reader's whole-line
    // equality, so an indented second spelling refuses rather than passing as well-formed
    let nb = page.lines().filter(|l| l.contains(begin.as_str())).count();
    let ne = page.lines().filter(|l| l.contains(end.as_str())).count();
    if nb != 1 || ne != 1 {
        return Err(format!(
            "{} needs exactly one '{}' + '{}' pair (found {} and {})",
            projection, begin, end, nb, ne
        ));
    }
    let block = marker::read_block(&page, &begin, &end);

    // spec: queue-kit/SPEC.md §check-roadmap-fresh — assertion A calls the emitter in-process:
    // the arm ported in the same commit, so there is no shell left to reach and the family's
    // bash hop is retired for this member rather than relocated.
    let emitted = if !emit_src.is_empty() {
        if !Path::new(emit_src).is_file() {
            return Err(format!("emit source not found: {}", emit_src));
        }
        fresh::read_captured(emit_src)?
    } else {
        roadmap::emit(&[])?
    };
    let emitted = emitted.trim_end_matches('\n');

    if block != emitted {
        println!(
            "check-roadmap-fresh: the {} block in {} is stale vs the roadmap arm:",
            name, projection
        );
        fresh::print_capped_diff(&format!("{}\n", emitted), &format!("{}\n", block));
        println!("  help: regenerate — bash gate-sdk/bin/run-gates.sh --emit roadmap --write");
        return Ok(1);
    }
    println!(
        "ROADMAP-FRESH: clean (the {} block in {} byte-matches the roadmap arm; {} tagged \
         entry/entries name a configured horizon and track)",
        name, projection, tagged
    );
    Ok(0)
}
