// spec: canon-kit/SPEC.md §check-unmarked-claim — a paragraph falling in a consumer-declared
// claim class carries a `measured:` marker: the class is the trigger, the marker one of three
// remedies, and the gate is indifferent between them
use crate::spec;
use crate::walk;
use std::path::Path;

const EXEMPT: &str = "unmarked-claim-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-unmarked-claim: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }

    // spec: canon-kit/SPEC.md §check-unmarked-claim — the roster is consumer config with no kit
    // default, so an unset command is the clean skip: a tree that declared no class has no
    // sentence falling in one
    if spec::command("CANON_KIT_CLAIM_CLASSES_CMD")?.is_empty() {
        println!("UNMARKED-CLAIM: clean (CANON_KIT_CLAIM_CLASSES_CMD unset — no declared claim class, so no sentence falls in one)");
        return Ok(0);
    }
    let classes = spec::claim_vocabulary("CANON_KIT_CLAIM_CLASSES_CMD")?;
    if classes.is_empty() {
        println!("UNMARKED-CLAIM: clean (CANON_KIT_CLAIM_CLASSES_CMD declared no claim classes)");
        return Ok(0);
    }

    // spec: canon-kit/SPEC.md §check-unmarked-claim — the scanned surface is
    // check-measured-claim's, shared rather than forked
    let globs = spec::knob_array_pub("CANON_KIT_MEASURED_SURFACE_GLOBS")?;
    if globs.is_empty() {
        println!("UNMARKED-CLAIM: clean (CANON_KIT_MEASURED_SURFACE_GLOBS empty — no scanned surface)");
        return Ok(0);
    }
    let mut files: Vec<String> = walk::glob_corpus(Path::new(root), &globs)?
        .into_iter()
        .filter(|p| p.is_file())
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    files.sort();
    files.dedup();
    if files.is_empty() {
        println!("UNMARKED-CLAIM: clean (0 file(s) on the measured surface)");
        return Ok(0);
    }

    let span = spec::measured_span()?;

    struct Sink<'a> {
        classes: &'a [(String, crate::ere::Ere)],
        span: spec::MeasuredSpan,
        out: Vec<String>,
        paras: usize,
    }
    impl spec::ProseSink for Sink<'_> {
        // spec: canon-kit/SPEC.md §check-unmarked-claim — the paragraph is the unit, and a
        // marker discharges exactly the claim it binds
        fn on_pflush(&mut self, file: &str, para: &spec::Para) {
            if para.len() == 0 {
                return;
            }
            self.paras += 1;
            let flat = spec::flatten_para(para);
            // spec: canon-kit/SPEC.md §check-unmarked-claim — the subject is ASCII-lowercased
            // before matching; the fold is byte-length-preserving, so the span still maps home
            let hay = flat.text.to_ascii_lowercase();
            let ranges = discharge_ranges(para, &flat, self.span);
            let discharged = |at: usize| ranges.iter().any(|(s, e)| at >= *s && at < *e);
            for (id, re) in self.classes {
                let mut from = 0usize;
                let mut hit = None;
                while from <= hay.len() {
                    let Some((start, _)) = re.find_from(&hay, from) else { break };
                    if !discharged(start) {
                        hit = Some(start);
                        break;
                    }
                    from = start + 1;
                }
                if let Some(start) = hit {
                    self.out.push(format!(
                        "  {}:{}  falls in claim class '{}' and carries no 'measured:' marker",
                        file,
                        flat.line_at(start),
                        id
                    ));
                    return;
                }
            }
        }
    }

    let mut sink = Sink {
        classes: &classes,
        span,
        out: Vec::new(),
        paras: 0,
    };
    spec::walk_prose(&files, EXEMPT, &mut sink)?;
    let out = sink.out;

    if !out.is_empty() {
        println!("check-unmarked-claim: a declared claim class is asserted with no oracle behind it — the sentence a reader trusts is the one nothing re-measures:");
        println!();
        for l in &out {
            println!("{}", l);
        }
        println!("  help: three remedies and the gate is indifferent between them — rewrite the sentence out of the class (a claim not made cannot go stale); attach a '<!-- measured: <key>=<value> -->' marker on the line above, or inline after the sentence, binding it to an oracle check-measured-claim re-runs; or, for a deliberate keep, tag '<!-- unmarked-claim-exempt: <reason> -->' on the flagged line or directly above it (a reason is mandatory). The classes are CANON_KIT_CLAIM_CLASSES_CMD's (canon-kit/SPEC.md §Layout and configuration); this is a class assertion, never a ban on a phrase.");
        return Ok(1);
    }
    println!(
        "UNMARKED-CLAIM: clean ({} file(s) on the measured surface, {} class(es) declared; {} paragraph(s) walked, none falling in a class unmarked)",
        files.len(),
        classes.len(),
        sink.paras
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-unmarked-claim — check-measured-claim's blocks: a full-line
// marker line is marker text and opens a block running to the next one or the paragraph's end,
// each block's inline markers bind within it, and a headed block's claim is the shared helper's
fn discharge_ranges(para: &spec::Para, flat: &spec::FlatPara, span: spec::MeasuredSpan) -> Vec<(usize, usize)> {
    let marker: Vec<bool> = para
        .line
        .iter()
        .map(|l| l.trim_start().starts_with(spec::MEASURED_MARKER))
        .collect();
    let seg_of = |b: usize| {
        let fnr = flat.line_at(b);
        let k = para.fnr.iter().position(|&f| f == fnr).unwrap_or(0);
        if marker[k] {
            return (Some(k), true);
        }
        ((0..k).rev().find(|&j| marker[j]), false)
    };
    let mut segs: Vec<((Option<usize>, bool), usize, usize)> = Vec::new();
    for b in 0..flat.text.len() {
        let id = seg_of(b);
        match segs.last_mut() {
            Some((last, _, e)) if *last == id => *e = b + 1,
            _ => segs.push((id, b, b + 1)),
        }
    }
    let mut out: Vec<(usize, usize)> = Vec::new();
    for ((head, is_marker), s, e) in segs {
        if is_marker {
            out.push((s, e));
            continue;
        }
        let block = &flat.text[s..e];
        for m in spec::inline_markers(block) {
            out.push((s + m.sentence.0, s + m.sentence.1));
            out.push((s + m.at, s + m.end));
        }
        if head.is_some() {
            let c = spec::full_line_claim(block, span);
            out.push((s + c.range.0, s + c.range.1));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use spec::MeasuredSpan::{Off, Paragraph, Sentence};

    fn para(lines: &[&str]) -> spec::Para {
        let mut p = spec::Para::default();
        for (k, l) in lines.iter().enumerate() {
            p.add(k + 1, l);
        }
        p
    }

    fn marked(lines: &[&str], needle: &str, span: spec::MeasuredSpan) -> bool {
        let p = para(lines);
        let flat = spec::flatten_para(&p);
        let at = flat.text.find(needle).expect("needle");
        discharge_ranges(&p, &flat, span).iter().any(|(s, e)| at >= *s && at < *e)
    }

    // spec: canon-kit/SPEC.md §check-unmarked-claim — the span narrows the discharge as it
    // narrows arm C's claim, and `off` leaves the whole block discharged
    #[test]
    fn a_full_line_marker_discharges_the_claim_its_span_binds() {
        let t = ["<!-- measured: k=3 -->", "It has three rows. Later a fast claim."];
        assert!(marked(&t, "three", Sentence));
        assert!(!marked(&t, "fast", Sentence));
        assert!(marked(&t, "fast", Paragraph));
        assert!(marked(&t, "fast", Off));
    }

    // spec: canon-kit/SPEC.md §check-unmarked-claim — a full-line marker's claim begins below it
    #[test]
    fn a_mid_paragraph_marker_does_not_discharge_the_text_above_it() {
        let t = ["A fast claim sits here.", "<!-- measured: k=3 -->", "It has three rows."];
        for s in [Paragraph, Sentence, Off] {
            assert!(!marked(&t, "fast", s));
            assert!(marked(&t, "three", s));
            assert!(marked(&t, "k=3", s));
        }
    }

    // spec: canon-kit/SPEC.md §check-unmarked-claim — an inline marker binds its own sentence
    // inside the block, and the full-line claim is the block less that sentence
    #[test]
    fn an_inline_marker_binds_its_sentence_and_the_full_line_claim_skips_it() {
        let t = ["<!-- measured: a=3 -->", "Twelve files. <!-- measured: b=12 --> It is fast. Then slow."];
        assert!(marked(&t, "Twelve", Sentence));
        assert!(marked(&t, "fast", Sentence));
        assert!(!marked(&t, "slow", Sentence));
    }
}
