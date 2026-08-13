// spec: canon-kit/SPEC.md §check-payload-claim — exactly one governed doc declares what a
// gate on the vendored payload discloses, and no scanned governed doc asserts a different
// disclosure class; membership over the whole document rather than position in a section
use crate::spec::{self, declared_id};
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-payload-claim: {}", e);
            2
        }
    }
}

const TAG: &str = "payload-discloses:";

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }

    // spec: canon-kit/SPEC.md §check-payload-claim — the disclosure vocabulary is consumer
    // config with no kit default, so an empty command is the clean skip for a tree whose
    // payload discloses one thing only
    if spec::knob_pub("CANON_KIT_PAYLOAD_CLAIMS_CMD")?.is_empty() {
        println!("PAYLOAD-CLAIM: clean (no disclosure vocabulary configured — nothing to hold)");
        return Ok(0);
    }
    let claims = spec::claim_vocabulary(
        "CANON_KIT_PAYLOAD_CLAIM_IDS",
        "CANON_KIT_PAYLOAD_CLAIM_PATTERNS",
    )?;
    if claims.is_empty() {
        println!("PAYLOAD-CLAIM: clean (CANON_KIT_PAYLOAD_CLAIMS_CMD declared no disclosure classes)");
        return Ok(0);
    }

    let files = spec::governed_docs(root, "CANON_KIT_PAYLOAD_CLAIM_EXCLUDE")?;
    if files.is_empty() {
        println!("PAYLOAD-CLAIM: clean (0 governed doc(s) found)");
        return Ok(0);
    }

    // assertion A: exactly one governed doc declares the disclosure class
    let decls = spec::declarations(&files, TAG)?;
    if decls.is_empty() {
        println!("check-payload-claim: no governed doc declares what the vendored payload discloses:");
        println!(
            "  0 'payload-discloses:' declarations across {} governed doc(s)",
            files.len()
        );
        println!("  help: an unowned disclosure claim is how an unbounded number of surfaces drift into");
        println!("        promising the consumer something else, with nothing watching. Put one full-line");
        println!("        '<!-- payload-discloses: <claim-id> -->' in the section that rules the fact,");
        println!("        naming a class CANON_KIT_PAYLOAD_CLAIMS_CMD emits.");
        return Ok(1);
    }
    if decls.len() > 1 {
        println!(
            "check-payload-claim: the payload-disclosure claim has {} owners; exactly one is required:",
            decls.len()
        );
        for d in &decls {
            println!("  {}:{}:{}", d.file, d.line, d.text);
        }
        println!("  help: keep the declaration in the section that rules the fact and delete the others;");
        println!("        two owners is the same unowned-claim defect wearing a different shape.");
        return Ok(1);
    }

    let decl = &decls[0];
    let declared = decl.id.clone();
    // spec: canon-kit/SPEC.md §check-payload-claim — an id outside the configured vocabulary
    // is fail-closed, not a violation: with no resolvable declared class the gate holds
    // nothing to compare a line against, so it must not run rather than pass
    if !claims.iter().any(|(id, _)| *id == declared) {
        eprintln!(
            "check-payload-claim: declared class '{}' ({}:{}) is not a configured disclosure class:",
            declared, decl.file, decl.line
        );
        for (id, _) in &claims {
            eprintln!("  {}", id);
        }
        return Ok(2);
    }

    // assertion B: no scanned line asserts a disclosure class other than the declared one
    // spec: canon-kit/SPEC.md §check-payload-claim — membership over the whole document, and
    // fenced content is scanned because a quoted recipe is where a claim shows up in passing
    let others: Vec<&(String, crate::ere::Ere)> =
        claims.iter().filter(|(id, _)| *id != declared).collect();
    let mut findings: Vec<String> = Vec::new();
    for f in &files {
        let text = spec::read_text(Path::new(f))?;
        for (idx, raw) in text.lines().enumerate() {
            if declared_id(raw, TAG).is_some() {
                continue;
            }
            for (id, re) in &others {
                if re.is_match(raw) {
                    findings.push(format!(
                        "  {}:{}: asserts '{}', not the declared '{}'",
                        f,
                        idx + 1,
                        id,
                        declared
                    ));
                    break;
                }
            }
        }
    }

    if !findings.is_empty() {
        println!("check-payload-claim: a governed doc asserts a disclosure class the declared claim does not name:");
        for l in &findings {
            println!("{}", l);
        }
        println!("  help: correct the sentence to the declared class, or move the declaration");
        println!(
            "        ({}:{}) to the class the payload actually discloses.",
            decl.file, decl.line
        );
        println!("        The declared class is what the payload ships; every other class is wrong");
        println!("        wherever it appears, whichever section it sits in.");
        return Ok(1);
    }

    println!(
        "PAYLOAD-CLAIM: clean ({} governed doc(s), {} other class(es) held; '{}' declared at {}:{}, and no scanned line asserts another class)",
        files.len(),
        others.len(),
        declared,
        decl.file,
        decl.line
    );
    Ok(0)
}
