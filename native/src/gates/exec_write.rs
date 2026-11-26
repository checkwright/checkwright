// spec: gate-sdk/SPEC.md §check-exec-write — no Rust file but a declared helper writes a file with
// an execute bit
use crate::exit::{self, Fault};
use crate::rust_code::{self, Arg, Call, View};
use crate::{listing, walk};
use std::path::Path;

const NAME: &str = "check-exec-write";
const PATHS_KNOB: &str = "GATE_SDK_EXEC_WRITE_PATHS";
const HELPERS_KNOB: &str = "GATE_SDK_EXEC_WRITE_HELPERS";

// spec: gate-sdk/SPEC.md §check-exec-write — the execute bits for owner, group and other
const EXEC_BITS: u128 = 0o111;
const CONSTANTS: &[&str] = &["S_IXUSR", "S_IXGRP", "S_IXOTH", "S_IRWXU", "S_IRWXG", "S_IRWXO", "ACCESSPERMS", "ALLPERMS"];
const MODE_CALLS: &[&str] = &["from_mode", "from_raw_mode", "mode", "set_mode"];
const LOW_LEVEL: &[&str] = &["libc", "nix", "rustix"];
const SPAWNED: &[&str] = &["chmod", "install"];
const SUFFIXES: &[&str] = &["u128", "usize", "u64", "u32", "u16", "u8", "i128", "isize", "i64", "i32", "i16", "i8"];

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(fault) => {
            let fault = fault.prefixed(NAME);
            eprintln!("{}", fault);
            fault.status()
        }
    }
}

// spec: gate-sdk/SPEC.md §check-exec-write — an integer literal in any base, `_` and a type
// suffix allowed; anything else has no value the gate can know
fn literal_value(arg: &str) -> Option<u128> {
    let mut s = arg.trim();
    if let Some(suf) = SUFFIXES.iter().find(|suf| s.len() > suf.len() && s.ends_with(*suf)) {
        s = &s[..s.len() - suf.len()];
    }
    let (radix, digits) = match s.get(..2) {
        Some("0o") => (8, &s[2..]),
        Some("0x") => (16, &s[2..]),
        Some("0b") => (2, &s[2..]),
        _ => (10, s),
    };
    if !s.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    let digits: String = digits.chars().filter(|c| *c != '_').collect();
    if digits.is_empty() || !digits.chars().all(|c| c.is_digit(radix)) {
        return None;
    }
    u128::from_str_radix(&digits, radix).ok()
}

// spec: gate-sdk/SPEC.md §check-exec-write — the mode test, failed by an absent argument too
fn fails(arg: Option<&Arg>) -> bool {
    arg.and_then(|a| literal_value(&a.text)).map_or(true, |v| v & EXEC_BITS != 0)
}

// spec: gate-sdk/SPEC.md §check-exec-write — the mode a permission-setting call takes: the first
// argument of `from_mode`, the last of `mode` and `set_mode`; a bare `mode(…)` is no site
fn mode_arg(call: &Call) -> Option<Option<&Arg>> {
    match call.name() {
        "from_mode" | "from_raw_mode" => Some(call.args.first()),
        "mode" if call.method() || call.segments().len() > 1 => Some(call.args.last()),
        "set_mode" => Some(call.args.last()),
        _ => None,
    }
}

// spec: gate-sdk/SPEC.md §check-exec-write — a plain or raw string literal's body
fn string_literal(text: &str) -> Option<&str> {
    let raw = text.strip_prefix('r').unwrap_or(text);
    let hashes = raw.len() - raw.trim_start_matches('#').len();
    let body = raw[hashes..].strip_prefix('"')?.strip_suffix(&format!("\"{}", "#".repeat(hashes)))?;
    (raw.len() < text.len() || !body.contains('\\')).then_some(body)
}

fn site(call: &Call, calls: &[Call]) -> bool {
    let segs = call.segments();
    let low = !call.method() && segs.len() > 1;
    let under = |crates: &[&str]| low && crates.contains(&segs[0]);
    match call.name() {
        n if MODE_CALLS.contains(&n) => mode_arg(call).is_some_and(fails),
        "set_permissions" => call.args.last().map_or(true, |q| {
            !calls.iter().any(|c| c.start >= q.start && c.start < q.end && mode_arg(c).is_some())
        }),
        "chmod" | "lchmod" | "fchmod" if under(LOW_LEVEL) => fails(call.args.last()),
        "fchmodat" if under(LOW_LEVEL) => fails(call.args.get(2)),
        "open" if under(&["libc"]) => call.args.get(2).is_some_and(|a| fails(Some(a))),
        "openat" if under(&["libc"]) => call.args.get(3).is_some_and(|a| fails(Some(a))),
        "creat" if under(&["libc"]) => fails(call.args.get(1)),
        "copy" => low && segs[segs.len() - 2] == "fs",
        "new" => {
            low && segs[segs.len() - 2] == "Command"
                && call.args.first().and_then(|a| string_literal(&a.text)).is_some_and(|l| SPAWNED.contains(&l))
        }
        _ => false,
    }
}

// spec: gate-sdk/SPEC.md §check-exec-write — a site's `<call>`: the spawned program for a
// spawning site, else the callee's last segment
fn called(call: &Call) -> String {
    match call.name() {
        "new" => call.args.first().and_then(|a| string_literal(&a.text)).unwrap_or("new").to_string(),
        n => n.to_string(),
    }
}

// spec: gate-sdk/SPEC.md §check-exec-write — every site of a file, calls and mode constants, in
// offset order
fn sites(view: &View) -> Vec<(usize, String)> {
    let mut found: Vec<(usize, usize, String)> = view
        .calls
        .iter()
        .filter(|c| site(c, &view.calls))
        .map(|c| (c.start, c.line, called(c)))
        .collect();
    let code = view.code.as_bytes();
    let mut i = 0;
    while i < code.len() {
        if code[i].is_ascii_alphanumeric() || code[i] == b'_' {
            let from = i;
            while i < code.len() && (code[i].is_ascii_alphanumeric() || code[i] == b'_') {
                i += 1;
            }
            let word = &view.code[from..i];
            if CONSTANTS.contains(&word) {
                found.push((from, view.line_of(from), word.to_string()));
            }
        } else {
            i += 1;
        }
    }
    found.sort_by_key(|f| f.0);
    found.into_iter().map(|(_, line, what)| (line, what)).collect()
}

// spec: gate-sdk/SPEC.md §check-exec-write — a helper is relative, `..`-free and names a file
fn helpers() -> Result<Vec<String>, Fault> {
    let words = walk::knob_words(HELPERS_KNOB).map_err(Fault::refused)?;
    words
        .into_iter()
        .map(|h| {
            if walk::host_absolute(&h) || walk::path_root(&h).is_some() {
                return Err(Fault::refused(format!("helper '{}' is absolute — name it relative to the working directory", h)));
            }
            if h.split(['/', '\\']).any(|c| c == "..") {
                return Err(Fault::refused(format!("helper '{}' holds '..' — name it inside the tree", h)));
            }
            if !Path::new(&h).is_file() {
                return Err(Fault::refused(format!("helper '{}' names no file", h)));
            }
            Ok(h.trim_start_matches("./").to_string())
        })
        .collect()
}

fn corpus(specs: &[String], helpers: &[String]) -> Result<Vec<String>, Fault> {
    let prune = walk::prune_dirs().map_err(Fault::refused)?;
    let specs: Vec<&str> = specs.iter().map(String::as_str).collect();
    let listed = listing::tracked(None, &specs).map_err(|e| Fault::tool(e.text()))?;
    Ok(listed
        .into_iter()
        .filter(|p| p.ends_with(".rs") && !walk::path_pruned(p, &prune) && !p.split('/').any(|c| c.starts_with('.')))
        .filter(|p| !helpers.contains(p) && Path::new(p).is_file())
        .collect())
}

fn inner(args: &[String]) -> Result<i32, Fault> {
    if !args.is_empty() {
        return Err(Fault::usage(format!("takes no arguments — its corpus is {}, its exemptions {}", PATHS_KNOB, HELPERS_KNOB)));
    }
    let specs = walk::knob_words(PATHS_KNOB).map_err(Fault::refused)?;
    if specs.is_empty() {
        println!("EXEC-WRITE: clean (not applicable: no Rust path configured)");
        return Ok(0);
    }
    let helpers = helpers()?;
    let files = corpus(&specs, &helpers)?;

    let (mut records, mut unparsed) = (Vec::new(), Vec::new());
    for f in &files {
        let bytes = std::fs::read(f).map_err(|e| Fault::input(format!("cannot read {}: {}", f, e)))?;
        match rust_code::read(&String::from_utf8_lossy(&bytes)) {
            Ok(view) => records.extend(sites(&view).into_iter().map(|(line, what)| format!("{}:{}: [EXEC-WRITE] {}", f, line, what))),
            Err(line) => unparsed.push(format!("{}:{}: [LINT-PARSE] unparseable span", f, line)),
        }
    }
    if !unparsed.is_empty() {
        for r in records.iter().chain(&unparsed) {
            println!("{}", r);
        }
        return Ok(exit::INPUT);
    }
    if records.is_empty() {
        println!("EXEC-WRITE: clean ({} Rust file(s) read, {} helper(s) exempt)", files.len(), helpers.len());
        return Ok(0);
    }
    println!("{}: executable-mode write(s) outside the declared helpers:", NAME);
    for r in &records {
        println!("{}", r);
    }
    println!("  help: move each write into a helper {} names and call the helper, or give the", HELPERS_KNOB);
    println!("        call a literal mode with no execute bit (0o644, say); the gate takes no waiver.");
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hits(text: &str) -> Vec<String> {
        sites(&rust_code::read(text).expect("reads")).into_iter().map(|(l, w)| format!("{}:{}", l, w)).collect()
    }

    // spec: gate-sdk/SPEC.md §check-exec-write — the mode test: an integer literal with no execute
    // bit passes, any other argument fails
    #[test]
    fn the_mode_test_reads_a_literal_value() {
        assert_eq!(literal_value("0o644"), Some(0o644));
        assert_eq!(literal_value("0o7_55u32"), Some(0o755));
        assert_eq!(literal_value("0x1ed"), Some(0o755));
        assert_eq!(literal_value("420_u32"), Some(0o644));
        assert_eq!(literal_value("0b110100100"), Some(0o644));
        assert_eq!(literal_value("mode"), None);
        assert_eq!(literal_value("0o644 | x"), None);
        assert_eq!(literal_value("+5"), None);
        assert!(!fails(Some(&Arg { text: "0o644".into(), start: 0, end: 0 })));
        assert!(fails(Some(&Arg { text: "0o100".into(), start: 0, end: 0 })));
        assert!(fails(None));
    }

    // spec: gate-sdk/SPEC.md §check-exec-write — each site kind, named by its last segment or its
    // constant, in offset order
    #[test]
    fn each_site_kind_hits() {
        let text = "Permissions::from_mode(0o755);\n\
                    OpenOptions::new().mode(m);\n\
                    p.set_mode(0o644);\n\
                    fs::set_permissions(a, perms);\n\
                    fs::set_permissions(a, Permissions::from_mode(0o600));\n\
                    libc::chmod(p, 0o700);\n\
                    nix::sys::stat::fchmodat(None, p, mode, f);\n\
                    libc::open(p, flags);\n\
                    libc::open(p, flags, 0o777);\n\
                    std::fs::copy(a, b);\n\
                    Command::new(\"chmod\").arg(\"+x\");\n\
                    Command::new(\"ls\");\n\
                    let m = libc::S_IXUSR;\n\
                    chmod(p, 0o755);\n";
        assert_eq!(hits(text), ["1:from_mode", "2:mode", "4:set_permissions", "6:chmod", "7:fchmodat", "9:open", "10:copy", "11:chmod", "13:S_IXUSR"]);
    }

    // spec: gate-sdk/SPEC.md §check-exec-write — a site spelled in a string, a raw string or a
    // comment is no site
    #[test]
    fn a_site_in_text_is_none() {
        let text = "let a = \"fs::copy(a, b)\";\nlet b = r#\"from_mode(0o755) \"S_IXUSR\"\"#;\n// libc::chmod(p, 0o755)\n/* Command::new(\"chmod\") */\n";
        assert!(hits(text).is_empty());
    }
}
