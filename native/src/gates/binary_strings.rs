// spec: gate-sdk/SPEC.md §check-binary-strings — no named build output carries a deny term or a
// configured leak shape in a string it would ship
use crate::deny_terms::{self, Entry};
use crate::elf;
use crate::ere::Ere;
use crate::exit::{self, Fault};
use crate::fresh;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-binary-strings — the id a hit takes when no `# rule:` line names one
const DEFAULT_ID: &str = "STRINGS-DENIED";

// spec: gate-sdk/SPEC.md §check-binary-strings — the `strings(1)` default run length, the floor's
// ceiling
const MIN_RUN: usize = 4;

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(fault) => {
            eprintln!("{}", fault);
            fault.status()
        }
    }
}

fn printable(b: u8) -> bool {
    (0x20..=0x7E).contains(&b) || b == b'\t'
}

fn token_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || b == b'-'
}

// spec: gate-sdk/SPEC.md §check-binary-strings — the floor is `MIN_RUN`, or the shortest term
// able to hit if that is shorter
fn floor(terms: &[Entry]) -> usize {
    terms
        .iter()
        .filter(|t| !t.text.is_empty() && t.text.bytes().all(token_byte))
        .map(|t| t.text.len())
        .fold(MIN_RUN, usize::min)
}

// spec: gate-sdk/SPEC.md §check-binary-strings — every run of printable bytes at least `floor`
// long inside one range, with the file offset of its first byte; a run never crosses a range
fn strings(bytes: &[u8], range: std::ops::Range<usize>, floor: usize) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut at = range.start;
    while at < range.end {
        if !printable(bytes[at]) {
            at += 1;
            continue;
        }
        let start = at;
        while at < range.end && printable(bytes[at]) {
            at += 1;
        }
        if at - start >= floor {
            if let Ok(s) = std::str::from_utf8(&bytes[start..at]) {
                out.push((start, s));
            }
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §check-binary-strings — the tokens of one string, split at every byte
// outside `[A-Za-z0-9._-]`, each with its offset inside the string
fn tokens(s: &str) -> impl Iterator<Item = (usize, &str)> {
    let b = s.as_bytes();
    let mut at = 0;
    std::iter::from_fn(move || {
        while at < b.len() && !token_byte(b[at]) {
            at += 1;
        }
        if at >= b.len() {
            return None;
        }
        let start = at;
        while at < b.len() && token_byte(b[at]) {
            at += 1;
        }
        Some((start, &s[start..at]))
    })
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hit {
    offset: usize,
    list: &'static str,
    position: usize,
    id: String,
}

fn scan(bytes: &[u8], ranges: Vec<std::ops::Range<usize>>, terms: &[Entry], shapes: &[(Ere, Entry)]) -> Vec<Hit> {
    let mut hits = Vec::new();
    let floor = floor(terms);
    for range in ranges {
        for (start, s) in strings(bytes, range, floor) {
            for (off, tok) in tokens(s) {
                for t in terms.iter().filter(|t| t.text.eq_ignore_ascii_case(tok)) {
                    hits.push(Hit { offset: start + off, list: "term", position: t.position, id: t.id_or(DEFAULT_ID).to_string() });
                }
            }
            for (re, e) in shapes {
                if let Some((at, _)) = re.find(s) {
                    hits.push(Hit { offset: start + at, list: "shape", position: e.position, id: e.id_or(DEFAULT_ID).to_string() });
                }
            }
        }
    }
    hits.sort();
    hits
}

fn inner(args: &[String]) -> Result<i32, Fault> {
    if !args.is_empty() {
        return Err(Fault::usage(
            "check-binary-strings: takes no arguments — name the build outputs in GATE_SDK_STRINGS_FILES",
        ));
    }
    let outputs = walk::knob_words("GATE_SDK_STRINGS_FILES").map_err(Fault::refused)?;
    if outputs.is_empty() {
        println!("BINARY-STRINGS: clean (not applicable: no build output named)");
        return Ok(0);
    }
    let terms = deny_terms::read(&deny_terms::files()?).map_err(|e| e.prefixed("check-binary-strings"))?;
    let shape_files = walk::knob_words("GATE_SDK_STRINGS_SHAPES").map_err(Fault::refused)?;
    let shape_list = deny_terms::read_lists(&shape_files, "shape list").map_err(|e| e.prefixed("check-binary-strings"))?;

    // spec: gate-sdk/SPEC.md §check-binary-strings — the GNU escape set is refused by name at
    // compile, as check-tree-terms refuses it
    let mut shapes: Vec<(Ere, Entry)> = Vec::new();
    for e in shape_list {
        match Ere::compile(&e.text) {
            Ok(re) => shapes.push((re, e)),
            Err(err) => {
                eprintln!("check-binary-strings: {}: {}", e.text, err);
                return Err(Fault::refused(format!(
                    "BINARY-STRINGS: {}",
                    fresh::fail_closed("shape-compile", Some(exit::REFUSED))
                )));
            }
        }
    }

    let mut records: Vec<String> = Vec::new();
    for path in &outputs {
        if !Path::new(path).is_file() {
            return Err(Fault::refused(format!("check-binary-strings: no build output at {}", path)));
        }
        let bytes = std::fs::read(path)
            .map_err(|e| Fault::input(format!("check-binary-strings: cannot read the build output {}: {}", path, e)))?;
        let ranges = elf::scan_ranges(&bytes)
            .map_err(|e| Fault::input(format!("check-binary-strings: {}: {}", path, e)))?;
        // spec: gate-sdk/SPEC.md §check-binary-strings — a record locates the hit and names the
        // list and position, never the string, the term or the shape
        records.extend(
            scan(&bytes, ranges, &terms, &shapes)
                .into_iter()
                .map(|h| format!("{}@{}: [{}] {} {}", path, h.offset, h.id, h.list, h.position)),
        );
    }

    if !records.is_empty() {
        println!("check-binary-strings: build output(s) carry a deny term or a leak shape in a shipped string:");
        for r in &records {
            println!("{}", r);
        }
        println!("  help: find what put the string at the named offset (a path the build embedded, a");
        println!("        name a build script wrote) and remove it at its source; the record carries");
        println!("        the rule id and the entry's position, never the string.");
        return Ok(1);
    }
    println!(
        "BINARY-STRINGS: clean ({} build output(s) scanned; none carry the {} deny term(s) or {} shape(s))",
        outputs.len(),
        terms.len(),
        shapes.len()
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elf::tests::image;

    fn entry(text: &str, position: usize) -> Entry {
        Entry { text: text.to_string(), id: None, position }
    }

    #[test]
    fn a_term_hits_a_whole_token_and_never_one_it_only_opens() {
        let bytes = b"\x00\x00the zephyr/cfg path\x00zephyrine\x00zephyr.cfg\x00Zephyr\x00";
        let whole = 0..bytes.len();
        let hits = scan(bytes, vec![whole], &[entry("zephyr", 1)], &[]);
        let offsets: Vec<usize> = hits.iter().map(|h| h.offset).collect();
        assert_eq!(offsets, vec![6, 43]);
    }

    // spec: gate-sdk/SPEC.md §check-binary-strings — the floor is 4, lowered only by a term able
    // to hit, and a run shorter than it is no string
    #[test]
    fn the_string_floor_derives_from_the_shortest_term() {
        assert_eq!(floor(&[entry("zephyr", 1)]), 4);
        assert_eq!(floor(&[entry("zephyr", 1), entry("qux", 2)]), 3);
        assert_eq!(floor(&[entry("zephyr", 1), entry("a/b", 2)]), 4);
        assert!(strings(b"\x00abc\x00", 0..5, 4).is_empty());
        assert_eq!(strings(b"\x00abcd\x00", 0..6, 4), vec![(1, "abcd")]);
        assert_eq!(strings(b"\x00ab\x00abc\x00", 0..8, 3), vec![(4, "abc")]);
        let whole = 0..5;
        assert_eq!(scan(b"\x00qux\x00", vec![whole], &[entry("zephyr", 1), entry("qux", 2)], &[]).len(), 1);
    }

    #[test]
    fn an_executable_section_is_never_read() {
        let img = image(&[(1, 0x6, b"\x00zephyr\x00"), (1, 0x3, b"\x00quiet data\x00")]);
        let ranges = elf::scan_ranges(&img).unwrap();
        assert!(scan(&img, ranges, &[entry("zephyr", 1)], &[]).is_empty());
    }
}
