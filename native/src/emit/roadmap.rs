// spec: queue-kit/SPEC.md §The roadmap arm — the public roadmap projected off the queue's
// [roadmap:] tags. The arm ships the projection *grammar* only; every lane name crosses the
// config bridge as a knob (§The tag algebra), so not one enters here.
use crate::marker;
use crate::queue::{self, RoadmapEntry, Sections};

const USAGE: &str = "\
usage: --emit roadmap [--emit|--write] [queue-file]
  --emit (default): print the generated block, the surface check-roadmap-fresh byte-compares.
  --write: splice that block between QUEUE_KIT_ROADMAP_FILE's markers, touching nothing outside them.
";

const PLACEHOLDER: &str = "_Nothing is queued under this horizon._";

enum Mode {
    Emit,
    Write,
    Help,
}

fn parse(args: &[String]) -> Result<(Mode, String), String> {
    let mut mode = Mode::Emit;
    let mut file = String::new();
    for a in args {
        match a.as_str() {
            "--emit" => mode = Mode::Emit,
            "--write" => mode = Mode::Write,
            "-h" | "--help" => mode = Mode::Help,
            other if other.starts_with('-') => {
                return Err(format!("unknown option: {}", other))
            }
            other => file = other.to_string(),
        }
    }
    Ok((mode, file))
}

// spec: queue-kit/SPEC.md §The roadmap arm — every configured horizon gets its heading whether or
// not the queue fills it: an empty horizon is information, and a section that vanishes when it
// empties reads as a page that forgot it. The order is the knob array's own, never a sort.
fn body(entries: &[RoadmapEntry], horizons: &[String]) -> String {
    let mut out = String::new();
    for (i, h) in horizons.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&format!("### {}\n\n", h));
        let mut n = 0usize;
        for e in entries {
            if e.slug.is_empty() || e.tags != 1 {
                continue;
            }
            if !e.field.starts_with(&format!("{}/", h)) {
                continue;
            }
            // spec: queue-kit/SPEC.md §The tag algebra — bash's `${fieldv#*/}`: the track is
            // everything past the *first* slash, so a three-field value fails the next test
            // rather than silently projecting its middle segment
            let track = match e.field.split_once('/') {
                Some((_, t)) => t,
                None => continue,
            };
            if track.is_empty() || track.contains('/') {
                continue;
            }
            // spec: queue-kit/SPEC.md §The roadmap arm — the whitelist: an entry with no single
            // declaration contributes no prose, so unmarked body text can never reach the page
            // even when the gate is bypassed
            if e.declarations != 1 || e.summary.is_empty() {
                continue;
            }
            out.push_str(&format!(
                "- **`{}`** *({})* — {}\n",
                e.slug, track, e.summary
            ));
            n += 1;
        }
        if n == 0 {
            out.push_str(PLACEHOLDER);
            out.push('\n');
        }
    }
    // comment-tier-exempt: load-bearing blank — kramdown ends a list only on a blank
    // line, else the :end marker abuts the last bullet and is absorbed into that <li>;
    // the byte-compare strips trailing newlines on both sides, so it still holds.
    out.push('\n');
    out
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let (mode, file) = parse(args)?;
    if let Mode::Help = mode {
        return Ok(USAGE.to_string());
    }

    let file = if file.is_empty() {
        queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?
    } else {
        file
    };
    let text = std::fs::read_to_string(&file)
        .map_err(|_| format!("queue file not found: {}", file))?;
    let horizons = queue::knob_array("QUEUE_KIT_HORIZONS")?;
    if horizons.is_empty() {
        return Err(
            "QUEUE_KIT_HORIZONS is empty — no roadmap vocabulary is configured".to_string(),
        );
    }
    let sec = Sections::active_and_deferred()?;
    let text = body(&queue::roadmap_entries(&text, &sec), &horizons);

    if let Mode::Emit = mode {
        return Ok(text);
    }

    let page = queue::knob_scalar("QUEUE_KIT_ROADMAP_FILE")?;
    if page.is_empty() {
        return Err(
            "--write needs QUEUE_KIT_ROADMAP_FILE; it is empty (no projection page configured)"
                .to_string(),
        );
    }
    if !std::path::Path::new(&page).is_file() {
        return Err(format!("projection page not found: {}", page));
    }
    let name = queue::knob_scalar("QUEUE_KIT_ROADMAP_MARKER")?;
    marker::write_block(&page, &begin(&name), &end(&name), &text)?;
    Ok(format!("roadmap: replaced the {} block in {}\n", name, page))
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the marker pair's spelling, derived from the configured
// name at both readers so the arm and check-roadmap-fresh cannot disagree about what bounds the
// block they compare.
pub fn begin(name: &str) -> String {
    format!("<!-- {}:begin -->", name)
}

pub fn end(name: &str) -> String {
    format!("<!-- {}:end -->", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(tags: usize, field: &str, slug: &str, decls: usize, summary: &str) -> RoadmapEntry {
        RoadmapEntry {
            tags,
            field: field.to_string(),
            slug: slug.to_string(),
            declarations: decls,
            summary: summary.to_string(),
        }
    }

    fn horizons() -> Vec<String> {
        vec!["soon".to_string(), "someday".to_string()]
    }

    // spec: queue-kit/SPEC.md §The roadmap arm — an empty horizon still gets its heading and the
    // placeholder, and the block ends on a blank line the kramdown list needs
    #[test]
    fn every_configured_horizon_gets_a_heading_in_the_knobs_own_order() {
        let e = vec![entry(1, "soon/alpha", "a-thing", 1, "A public sentence.")];
        assert_eq!(
            body(&e, &horizons()),
            "### soon\n\n- **`a-thing`** *(alpha)* — A public sentence.\n\n### someday\n\n\
             _Nothing is queued under this horizon._\n\n"
        );
    }

    // spec: queue-kit/SPEC.md §The roadmap arm — the whitelist and the field grammar: a second
    // tag, an unparseable field, a three-field value and a missing or doubled declaration each
    // contribute nothing rather than a partial bullet
    #[test]
    fn only_a_single_tag_a_two_field_value_and_one_declaration_project() {
        let e = vec![
            entry(2, "soon/alpha", "two-tags", 1, "s"),
            entry(1, "soon", "no-slash", 1, "s"),
            entry(1, "soon/a/b", "three-fields", 1, "s"),
            entry(1, "soon/", "empty-track", 1, "s"),
            entry(1, "soon/alpha", "no-decl", 0, ""),
            entry(1, "soon/alpha", "two-decl", 2, "s"),
            entry(1, "elsewhere/alpha", "other-horizon", 1, "s"),
        ];
        let out = body(&e, &horizons());
        assert!(out.contains("### soon\n\n_Nothing is queued"), "{}", out);
        for slug in [
            "two-tags",
            "no-slash",
            "three-fields",
            "empty-track",
            "no-decl",
            "two-decl",
            "other-horizon",
        ] {
            assert!(!out.contains(slug), "{} reached the page:\n{}", slug, out);
        }
    }
}
