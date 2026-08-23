// spec: gate-sdk/SPEC.md §check-gate-assertions — couple each §<gate> enumerated-assertion
// span+count to the gate code's `# assertion` markers
use crate::ere::Ere;
use crate::fresh;
use crate::gates::gate_output::{native_module, resolve_declaration};
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-gate-assertions — discovery's filters, verbatim from the shell
// form: a count-word adjacent to an enumeration noun, with one optional word between them
const COUNT_RX: &str = "(^|[^a-z])(two|three|four|five|six|seven|eight|nine)[[:space:]]+([a-z]+[[:space:]]+)?(assertion|assertions|axes|axis|checks)([^a-z]|$)";
const WORD_RX: &str = "(two|three|four|five|six|seven|eight|nine)";
const FIRST_PAREN_RX: &str = "\\([^)]*\\)";
const ONE_LABEL_RX: &str = "^\\([A-Za-z0-9]\\)$";
const LABEL_RX: &str = "\\(([A-Za-z0-9])\\)";

// spec: gate-sdk/SPEC.md §check-gate-assertions — the two marker grammars differ on purpose: this
// one accepts a multi-character label where the contract-span pattern accepts exactly one, so both
// widths are reproduced rather than unified
const MARKER_RX: &str = "(#|//)[[:space:]]*assertion[[:space:]]+[A-Za-z0-9]+:";

struct Rx {
    count: Ere,
    word: Ere,
    paren: Ere,
    one: Ere,
    label: Ere,
    marker: Ere,
}

impl Rx {
    fn new() -> Result<Rx, String> {
        let c = |p: &str| {
            Ere::compile(p)
                .map_err(|e| format!("check-gate-assertions: cannot compile {}: {}", p, e))
        };
        Ok(Rx {
            count: c(COUNT_RX)?,
            word: c(WORD_RX)?,
            paren: c(FIRST_PAREN_RX)?,
            one: c(ONE_LABEL_RX)?,
            label: c(LABEL_RX)?,
            marker: c(MARKER_RX)?,
        })
    }
}

struct Contract {
    heading: String,
    count: usize,
    labels: Vec<String>,
}

fn word_num(w: &str) -> usize {
    match w {
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

// spec: gate-sdk/SPEC.md §check-gate-assertions — awk's `/^[[:space:]]*$/` over the paragraph
// accumulator: POSIX space, which is narrower than Rust's Unicode-aware `trim`
fn blank(l: &str) -> bool {
    l.bytes()
        .all(|c| matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r'))
}

// spec: gate-sdk/SPEC.md §check-gate-assertions — the offsets are bytes, the C-locale reading of
// the shell form's character indices; one landing inside a multi-byte character steps back to the
// boundary below it
fn clamp(s: &str, mut i: usize) -> usize {
    if i > s.len() {
        i = s.len();
    }
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

fn cut(s: &str, a: usize, b: usize) -> &str {
    let (a, b) = (clamp(s, a), clamp(s, b));
    if a > b {
        return "";
    }
    &s[a..b]
}

fn contract(rx: &Rx, heading: &str, p: &str) -> Option<Contract> {
    // spec: gate-sdk/SPEC.md §check-gate-assertions — the first pinned port hazard: the slice below
    // indexes the original-case paragraph with the lowered copy's offsets, sound only because an
    // ASCII lowercase preserves byte length where a Unicode one does not
    let low = p.to_ascii_lowercase();
    let (ns, ne) = rx.count.find(&low)?;
    let span = cut(&low, ns, ne);
    let count = rx
        .word
        .find(span)
        .map(|(a, b)| word_num(cut(span, a, b)))
        .unwrap_or(0);

    // spec: gate-sdk/SPEC.md §check-gate-assertions — the second pinned port hazard: the slice
    // starts one position early so the trailing boundary character the pattern consumed survives
    let mut after = &p[clamp(p, ne.saturating_sub(1))..];

    let (fs, fe) = rx.paren.find(after)?;
    if !rx.one.is_match(cut(after, fs, fe)) {
        return None;
    }

    let mut labels: Vec<String> = Vec::new();
    while let Some((s, e)) = rx.label.find(after) {
        let m = cut(after, s, e);
        let lab = (m.as_bytes()[1] as char).to_string();
        if !labels.contains(&lab) {
            labels.push(lab);
        }
        after = &after[clamp(after, e)..];
    }
    if labels.len() < 2 {
        return None;
    }
    Some(Contract {
        heading: heading.to_string(),
        count,
        labels,
    })
}

// spec: gate-sdk/SPEC.md §check-gate-assertions — discovery is first-paragraph-scoped: the
// paragraph is the run of non-blank lines after a `### ` heading, joined with single spaces, and
// a `## ` heading closes the subsection without opening one
fn extract(rx: &Rx, text: &str) -> Vec<Contract> {
    let mut out: Vec<Contract> = Vec::new();
    let mut heading = String::new();
    let mut para = String::new();
    let (mut started, mut done) = (false, false);
    let emit = |h: &str, p: &str, out: &mut Vec<Contract>| {
        if !h.is_empty() {
            if let Some(c) = contract(rx, h, p) {
                out.push(c);
            }
        }
    };
    for line in fresh::file_lines(text) {
        if let Some(rest) = line.strip_prefix("### ") {
            emit(&heading, &para, &mut out);
            heading = rest.to_string();
            para.clear();
            started = false;
            done = false;
            continue;
        }
        if line.starts_with("## ") {
            emit(&heading, &para, &mut out);
            heading.clear();
            continue;
        }
        if heading.is_empty() || done {
            continue;
        }
        if !started {
            if blank(line) {
                continue;
            }
            started = true;
            para = line.to_string();
            continue;
        }
        if blank(line) {
            done = true;
            continue;
        }
        para.push(' ');
        para.push_str(line);
    }
    emit(&heading, &para, &mut out);
    out
}

// spec: gate-sdk/SPEC.md §check-gate-assertions — the comment leader is the substrate's, so both
// spellings are read; the third pinned port hazard is the sort, which is byte order and therefore
// the C-locale narrowing of the shell form's ambient-locale `sort -u`
fn markers(rx: &Rx, text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in fresh::file_lines(text) {
        let mut rest = line;
        while let Some((s, e)) = rx.marker.find(rest) {
            if let Some(lab) = label_of(cut(rest, s, e)) {
                out.push(lab);
            }
            rest = &rest[clamp(rest, e)..];
        }
    }
    out.sort();
    out.dedup();
    out
}

fn label_of(m: &str) -> Option<String> {
    let b = m.as_bytes();
    if b.last() != Some(&b':') {
        return None;
    }
    let mut i = b.len() - 1;
    while i > 0 && b[i - 1].is_ascii_alphanumeric() {
        i -= 1;
    }
    if i == b.len() - 1 {
        return None;
    }
    Some(m[i..b.len() - 1].to_string())
}

enum Resolved {
    Found(String),
    OutOfReach,
    Unresolved,
}

// spec: gate-sdk/SPEC.md §check-gate-assertions — a `.gate`-declared member carries its rule in the
// implementation module, so the markers are looked for there, and crate presence is the manifest
// rather than the directory
fn resolve_gate_file(
    gate: &str,
    scripts_dir: &str,
    dirs: &[String],
    crate_dir: &str,
    crate_manifest: &str,
) -> Resolved {
    if !scripts_dir.is_empty() {
        let p = format!("{}/{}.sh", scripts_dir, gate);
        return if Path::new(&p).is_file() {
            Resolved::Found(p)
        } else {
            Resolved::Unresolved
        };
    }
    let Some(decl) = resolve_declaration(gate, dirs) else {
        return Resolved::Unresolved;
    };
    let decl = if decl.ends_with(".gate") {
        if !Path::new(crate_manifest).is_file() {
            return Resolved::OutOfReach;
        }
        native_module(crate_dir, gate)
    } else {
        decl
    };
    if Path::new(&decl).is_file() {
        Resolved::Found(decl)
    } else {
        Resolved::Unresolved
    }
}

// spec: gate-sdk/SPEC.md §check-gate-assertions — the comma join over a sorted label set, which
// criterion 7 prices as a class (ii) incidental use of `paste -sd, -`: the compiled rule spells
// the join and the verdict is identical either side of the substitution
fn join(labels: &[String]) -> String {
    labels.join(",")
}

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}", msg);
            2
        }
    }
}

fn inner(args: &[String]) -> Result<i32, String> {
    let rx = Rx::new()?;
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")
        .map_err(|e| format!("check-gate-assertions: {}", e))?;
    let kit_roots = walk::kit_roots_abs().map_err(|e| format!("check-gate-assertions: {}", e))?;
    let crate_dir = walk::knob_scalar("GATE_SDK_NATIVE_CRATE")
        .map_err(|e| format!("check-gate-assertions: {}", e))?;
    let crate_manifest = format!("{}/Cargo.toml", crate_dir);

    let specs: Vec<String> = match args.first() {
        Some(a) => vec![a.clone()],
        None => {
            let mut v: Vec<String> = Vec::new();
            let own = format!("{}/SPEC.md", gates_dir);
            if Path::new(&own).is_file() {
                v.push(own);
            }
            for k in &kit_roots {
                let s = format!("{}/SPEC.md", k);
                if Path::new(&s).is_file() {
                    v.push(s);
                }
            }
            v
        }
    };
    let scripts_dir = args.get(1).cloned().unwrap_or_default();

    if specs.is_empty() {
        return Err("check-gate-assertions: no SPEC.md found (run from repo root)".into());
    }
    for s in &specs {
        if !Path::new(s).is_file() {
            return Err(format!(
                "check-gate-assertions: not found: {} (run from repo root)",
                s
            ));
        }
    }

    let mut resolve_dirs = vec![gates_dir];
    resolve_dirs.extend(kit_roots.iter().map(|k| format!("{}/checks", k)));

    let mut findings: Vec<String> = Vec::new();
    let mut out_of_reach: Vec<String> = Vec::new();
    let mut coupled = 0usize;

    for spec in &specs {
        let text = std::fs::read(spec)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|_| {
                format!(
                    "check-gate-assertions: {}",
                    fresh::fail_closed("awk", Some(2))
                )
            })?;
        for c in extract(&rx, &text) {
            coupled += 1;

            // assertion A: the count-word equals the size of the label span it introduces
            if c.labels.len() != c.count {
                findings.push(format!(
                    "{} §{}: count-word says {} but the (X) span enumerates {} label(s) [{}] — the contract is internally inconsistent",
                    spec, c.heading, c.count, c.labels.len(), join(&c.labels)
                ));
            }

            let file = match resolve_gate_file(
                &c.heading,
                &scripts_dir,
                &resolve_dirs,
                &crate_dir,
                &crate_manifest,
            ) {
                // spec: gate-sdk/SPEC.md §check-gate-assertions — a tree with no crate skips those
                // members and counts them: an internal sentinel, not a failure path
                Resolved::OutOfReach => {
                    out_of_reach.push(c.heading.clone());
                    continue;
                }
                // assertion B: the heading resolves to gate code through the registry
                Resolved::Unresolved => {
                    findings.push(format!(
                        "{} §{}: enumerated contract but no gate code resolves for '{}' (heading must name the script)",
                        spec, c.heading, c.heading
                    ));
                    continue;
                }
                Resolved::Found(f) => f,
            };

            let code = std::fs::read(&file)
                .map(|b| String::from_utf8_lossy(&b).into_owned())
                .unwrap_or_default();
            let have = markers(&rx, &code);

            // assertion C: a resolved file carrying no marker at all is the retrofit obligation
            if have.is_empty() {
                findings.push(format!(
                    "{} §{}: contract enumerates [{}] but {} carries zero `# assertion` markers (retrofit obligation)",
                    spec, c.heading, join(&c.labels), file
                ));
                continue;
            }

            let mut want = c.labels.clone();
            want.sort();
            want.dedup();

            // assertion D: the marker label set equals the contract's label span, reported through
            // its `missing` and `extra` sub-branches
            if want != have {
                let missing: Vec<String> = want
                    .iter()
                    .filter(|l| !have.contains(l))
                    .cloned()
                    .collect();
                let extra: Vec<String> = have
                    .iter()
                    .filter(|l| !want.contains(l))
                    .cloned()
                    .collect();
                let mut msg = format!(
                    "{} §{}: marker set [{}] != contract span [{}]",
                    spec,
                    c.heading,
                    join(&have),
                    join(&want)
                );
                if !missing.is_empty() {
                    msg.push_str(&format!("; missing marker(s): {}", join(&missing)));
                }
                if !extra.is_empty() {
                    msg.push_str(&format!("; extra marker(s): {}", join(&extra)));
                }
                findings.push(msg);
            }
        }
    }

    if !findings.is_empty() {
        println!("check-gate-assertions: §<gate> assertion enumeration ↔ gate-code marker mismatch:");
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: add/align a '# assertion <label>: <tag>' marker per enumerated assertion in the gate code so its label set matches the §<gate> contract span (and the count-word), or fix the contract's enumeration");
        return Ok(1);
    }

    let declared = if out_of_reach.is_empty() {
        String::new()
    } else {
        format!(
            ", {} declared out of reach with no crate at {} — {}",
            out_of_reach.len(),
            crate_manifest,
            out_of_reach.join(" ")
        )
    };
    println!(
        "GATE-ASSERTIONS: clean ({} of {} enumerated contract(s) coupled{})",
        coupled - out_of_reach.len(),
        coupled,
        declared
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-gate-assertions — this module is the resolution target of the
    // gate's own contract, so a marker shape is composed here and never spelled: a literal one would
    // join the module's own marker set and red the gate against itself
    fn marker(label: &str, tag: &str) -> String {
        format!("// {} {}: {}", "assertion", label, tag)
    }

    fn rx() -> Rx {
        Rx::new().expect("the gate's own patterns must compile")
    }

    // spec: gate-sdk/SPEC.md §check-gate-assertions — the slice keeps one boundary character, so a
    // noun followed immediately by a parenthesis keeps its first label. Slicing from the match end
    // drops the `(` the pattern consumed and the span silently loses (A).
    #[test]
    fn the_slice_keeps_the_boundary_character_the_pattern_consumed() {
        let c = contract(&rx(), "check-x", "Invariant: two axes(A) the first; (B) the second.")
            .expect("the contract must be discovered");
        assert_eq!(c.labels, vec!["A", "B"]);
        assert_eq!(c.count, 2);
    }

    // spec: gate-sdk/SPEC.md §check-gate-assertions — the lowercasing must be byte-length
    // preserving, because the slice indexes the original-case paragraph with the lowered copy's
    // offsets. Rust's Unicode lowercase is not, which is the silent-divergence hazard.
    #[test]
    fn the_lowercasing_is_ascii_and_preserves_every_offset() {
        let p = "Invariant: İstanbul holds on TWO AXES: (A) the first; (B) the second.";
        assert_eq!(p.to_ascii_lowercase().len(), p.len());
        assert_ne!(p.to_lowercase().len(), p.len());
        let c = contract(&rx(), "check-x", p).expect("an upper-case count-word is discovered");
        assert_eq!(c.labels, vec!["A", "B"]);
    }

    // spec: gate-sdk/SPEC.md §check-gate-assertions — the four discovery filters, each proved by a
    // paragraph that must NOT yield a contract
    #[test]
    fn discovery_excludes_what_the_four_filters_exclude() {
        let rx = rx();
        assert!(contract(&rx, "check-x", "Invariant: bar is internally coherent.").is_none());
        assert!(contract(&rx, "check-x", "Held on three grounds: (A) one; (B) two.").is_none());
        assert!(contract(&rx, "check-x", "Two axes (see below): (A) one; (B) two.").is_none());
        assert!(contract(&rx, "check-x", "Held on two axes: (A) the only one.").is_none());
    }

    // spec: gate-sdk/SPEC.md §check-gate-assertions — the two marker grammars differ on purpose:
    // the extraction pattern accepts a multi-character label, the contract span does not, so a
    // multi-character marker can only ever surface as an extra marker
    #[test]
    fn the_marker_grammar_is_wider_than_the_contract_span_grammar() {
        let rx = rx();
        let text = format!("{}\n    {}\n", marker("AB", "a wide label"), marker("C", "one"));
        assert_eq!(markers(&rx, &text), vec!["AB", "C"]);
        assert!(contract(&rx, "check-x", "Held on two axes: (AB) one; (C) two.").is_none());
    }

    // spec: gate-sdk/SPEC.md §check-gate-assertions — the marker set is sorted by byte, which is
    // C-locale order, and deduplicated; the leader is the substrate's and either spelling counts
    #[test]
    fn the_marker_set_is_byte_sorted_deduplicated_and_leader_agnostic() {
        let rx = rx();
        let text = format!(
            "{}\n{}\n{}\n# {} a: a hash-led marker\n",
            marker("b", "lower"),
            marker("A", "upper"),
            marker("A", "a repeat of the same label"),
            "assertion"
        );
        assert_eq!(markers(&rx, &text), vec!["A", "a", "b"]);
    }

    // spec: gate-sdk/SPEC.md §check-gate-assertions — a marker whose label is not alphanumeric is
    // not a marker, and a `# assertion <label>:` help string is therefore not one either
    #[test]
    fn a_placeholder_label_is_not_a_marker() {
        assert!(markers(&rx(), "  help: add a '# assertion <label>: <tag>' marker\n").is_empty());
    }

    // spec: canon-kit/SPEC.md §check-comment-tier — the arms are proved by the gate's own fixture
    // pair rather than by the live tree, the sentence a self-auditing member inherits
    #[test]
    fn the_fixture_pair_yields_the_contracts_its_cases_enumerate() {
        let rx = rx();
        let count = |case: &str| {
            let dir = walk::fixture_case_dirs("check-gate-assertions")
                .into_iter()
                .find(|d| d.ends_with(case))
                .expect("the check-gate-assertions fixture pair is missing a case dir");
            let text = std::fs::read_to_string(dir.join("scripts/SPEC.md"))
                .expect("the fixture case has no scripts/SPEC.md");
            extract(&rx, &text).len()
        };
        assert_eq!(count("good"), 3, "three coupled contracts and four excluded headings");
        assert_eq!(count("bad"), 6);
    }
}
