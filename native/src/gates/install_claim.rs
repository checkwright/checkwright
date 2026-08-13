// spec: canon-kit/SPEC.md §check-install-claim — exactly one governed doc declares the
// primary install transport, and no scanned install section leads with a different one; the
// transport vocabulary arrives as bridged data, never from an emitter spawned here
use crate::spec::{self, compile_pattern as compile, declared_id, governed_docs, skip_space};
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-claim: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }

    // spec: canon-kit/SPEC.md §check-install-claim — the transport vocabulary and the
    // install-section regex are consumer config with no kit default, so either left empty is
    // the clean skip for a consumer that documents no install path
    let section_re = spec::knob_pub("CANON_KIT_INSTALL_SECTION_RE")?;
    if spec::knob_pub("CANON_KIT_INSTALL_TRANSPORTS_CMD")?.is_empty() || section_re.is_empty() {
        println!("INSTALL-CLAIM: clean (no transport vocabulary or install-section regex configured — nothing to hold)");
        return Ok(0);
    }
    let transports = spec::claim_vocabulary(
        "CANON_KIT_INSTALL_TRANSPORT_IDS",
        "CANON_KIT_INSTALL_TRANSPORT_PATTERNS",
    )?;
    if transports.is_empty() {
        println!("INSTALL-CLAIM: clean (CANON_KIT_INSTALL_TRANSPORTS_CMD declared no transports)");
        return Ok(0);
    }

    let files = governed_docs(root, "CANON_KIT_INSTALL_CLAIM_EXCLUDE")?;
    if files.is_empty() {
        println!("INSTALL-CLAIM: clean (0 governed doc(s) found)");
        return Ok(0);
    }

    let decls = spec::declarations(&files, "install-primary:")?;
    if decls.is_empty() {
        println!("check-install-claim: no governed doc declares the primary install transport:");
        println!(
            "  0 'install-primary:' declarations across {} governed doc(s)",
            files.len()
        );
        println!("  help: an unowned primary-path claim is how two pages drift into naming different");
        println!("        transports with nothing to catch it. Put one full-line");
        println!("        '<!-- install-primary: <transport-id> -->' in the section that owns the claim,");
        println!("        naming a transport CANON_KIT_INSTALL_TRANSPORTS_CMD emits.");
        return Ok(1);
    }
    if decls.len() > 1 {
        println!(
            "check-install-claim: the primary-install-path claim has {} owners; exactly one is required:",
            decls.len()
        );
        for d in &decls {
            println!("  {}:{}:{}", d.file, d.line, d.text);
        }
        println!("  help: keep the declaration on the page that owns the claim and delete the others;");
        println!("        two owners is the same unowned-claim defect wearing a different shape.");
        return Ok(1);
    }

    let decl = &decls[0];
    let primary = decl.id.clone();
    // spec: canon-kit/SPEC.md §check-install-claim — an id outside the configured vocabulary
    // is fail-closed, not a violation: the gate then has no primary to compare a section's
    // leading transport against, so it must not run rather than pass
    if !transports.iter().any(|(id, _)| *id == primary) {
        eprintln!(
            "check-install-claim: declared primary '{}' ({}:{}) is not a configured transport:",
            primary, decl.file, decl.line
        );
        for (id, _) in &transports {
            eprintln!("  {}", id);
        }
        return Ok(2);
    }

    let sect = compile(&section_re, "CANON_KIT_INSTALL_SECTION_RE")?;
    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — delta (5)'s production reader of the
    // span API: the heading's own text is `substr($0, RSTART + RLENGTH)` off a leftmost-longest
    // match, so the port routes it through `find` rather than hand-writing a second scanner
    let head_re = compile(HEADING_RE, "check-install-claim's heading grammar")?;

    // assertion B: a scanned section's earliest transport-matching line is its leading claim
    // spec: canon-kit/SPEC.md §check-install-claim — later matches are never flagged, a fenced
    // line is scanned but never read as a heading, and the declaration is not its own evidence
    let mut findings: Vec<String> = Vec::new();
    let mut scanned = 0usize;
    for f in &files {
        let text = spec::read_text(Path::new(f))?;
        let mut fence = false;
        let mut inscope = false;
        let mut settled = false;
        let mut sect_name = String::new();
        let mut sect_fnr = 0usize;
        for (idx, raw) in text.lines().enumerate() {
            let fnr = idx + 1;
            if is_fence(raw) {
                fence = !fence;
                continue;
            }
            if !fence {
                if let Some((_, end)) = head_re.find(raw) {
                    let head = rtrim_space(&raw[end..]);
                    inscope = sect.is_match(head);
                    settled = false;
                    if inscope {
                        scanned += 1;
                        sect_name = head.to_string();
                        sect_fnr = fnr;
                    }
                    continue;
                }
            }
            if !inscope || settled || declared_id(raw, "install-primary:").is_some() {
                continue;
            }
            let mut led = String::new();
            for (id, re) in &transports {
                if !re.is_match(raw) {
                    continue;
                }
                if *id == primary {
                    led = primary.clone();
                    break;
                }
                if led.is_empty() {
                    led = id.clone();
                }
            }
            if led.is_empty() {
                continue;
            }
            settled = true;
            if led != primary {
                findings.push(format!(
                    "  {}:{}: section '{}' (line {}) leads with '{}', not the declared primary '{}'",
                    f, fnr, sect_name, sect_fnr, led, primary
                ));
            }
        }
    }

    if !findings.is_empty() {
        println!("check-install-claim: an install section leads with a transport the primary-path claim does not name:");
        for l in &findings {
            println!("{}", l);
        }
        println!("  help: the first transport a section names is the path it recommends. Lead with the");
        println!("        declared primary and name the others after it, or move the declaration");
        println!(
            "        ({}:{}) to the transport the project actually leads with.",
            decl.file, decl.line
        );
        return Ok(1);
    }

    println!(
        "INSTALL-CLAIM: clean ({} governed doc(s), {} sections scanned; '{}' declared primary at {}:{}, and no scanned section leads with another transport)",
        files.len(),
        scanned,
        primary,
        decl.file,
        decl.line
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-install-claim — `^#{2,6}[[:space:]]+`, the kit's own heading
// grammar, compiled through the engine because delta (5) makes its span the production reader
const HEADING_RE: &str = "^#{2,6}[[:space:]]+";

// spec: canon-kit/SPEC.md §check-install-claim — this member's fence test admits `~~~` beside
// the backtick form, so it is its own rather than the manifest walk's
pub fn is_fence(line: &str) -> bool {
    let b = line.as_bytes();
    let i = skip_space(b, 0);
    b[i..].starts_with(b"```") || b[i..].starts_with(b"~~~")
}

fn rtrim_space(s: &str) -> &str {
    let b = s.as_bytes();
    let mut e = b.len();
    while e > 0 && matches!(b[e - 1], b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r') {
        e -= 1;
    }
    &s[..e]
}
