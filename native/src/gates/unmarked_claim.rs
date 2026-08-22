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
    if spec::knob_pub("CANON_KIT_CLAIM_CLASSES_CMD")?.is_empty() {
        println!("UNMARKED-CLAIM: clean (CANON_KIT_CLAIM_CLASSES_CMD unset — no declared claim class, so no sentence falls in one)");
        return Ok(0);
    }
    let classes =
        spec::claim_vocabulary("CANON_KIT_CLAIM_CLASS_IDS", "CANON_KIT_CLAIM_CLASS_PATTERNS")?;
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
    let mut files: Vec<String> = walk::glob_files(Path::new(root), &globs)?
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

    struct Sink<'a> {
        classes: &'a [(String, crate::ere::Ere)],
        out: Vec<String>,
        paras: usize,
    }
    impl spec::ProseSink for Sink<'_> {
        // spec: canon-kit/SPEC.md §check-unmarked-claim — the paragraph is the unit, the
        // marker line riding inside the block it heads
        fn on_pflush(&mut self, file: &str, para: &spec::Para) {
            if para.len() == 0 {
                return;
            }
            self.paras += 1;
            if para
                .line
                .iter()
                .any(|l| l.trim_start().starts_with(spec::MEASURED_MARKER))
            {
                return;
            }
            let flat = spec::flatten_para(para);
            // spec: canon-kit/SPEC.md §check-unmarked-claim — the subject is ASCII-lowercased
            // before matching; the fold is byte-length-preserving, so the span still maps home
            let hay = flat.text.to_ascii_lowercase();
            for (id, re) in self.classes {
                if let Some((start, _)) = re.find(&hay) {
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
        println!("  help: three remedies and the gate is indifferent between them — rewrite the sentence out of the class (a claim not made cannot go stale); attach a '<!-- measured: <key>=<value> -->' marker on the line above, binding it to an oracle check-measured-claim re-runs; or, for a deliberate keep, tag '<!-- unmarked-claim-exempt: <reason> -->' on the flagged line or directly above it (a reason is mandatory). The classes are CANON_KIT_CLAIM_CLASSES_CMD's (canon-kit/SPEC.md §Layout and configuration); this is a class assertion, never a ban on a phrase.");
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
