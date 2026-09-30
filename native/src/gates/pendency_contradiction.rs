// spec: canon-kit/SPEC.md §check-pendency-contradiction — no governed manifest file binds one code
// span both to a pending construction and to a landed one
use super::citation_link::prose_only;
use super::spec_pointer::paragraphs;
use crate::spec;
use std::collections::BTreeMap;
use std::path::Path;

const PENDING: &str = "CANON_KIT_PENDENCY_PENDING";
const PENDING_EXTRA: &str = "CANON_KIT_PENDENCY_PENDING_EXTRA";
const LANDED: &str = "CANON_KIT_PENDENCY_LANDED";
const LANDED_EXTRA: &str = "CANON_KIT_PENDENCY_LANDED_EXTRA";
const VALVE: &str = "pendency-exempt:";

// spec: canon-kit/SPEC.md §check-pendency-contradiction — a span becomes one sentinel in the
// folded prose, so a phrase never matches inside code and a slot reads a list by position alone
const BINDABLE: char = '\u{1}';
const INERT: char = '\u{2}';

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-pendency-contradiction: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let pending = phrases(&spec::vocabulary(PENDING, PENDING_EXTRA)?);
    let landed = phrases(&spec::vocabulary(LANDED, LANDED_EXTRA)?);
    for (set, knob) in [(&pending, PENDING), (&landed, LANDED)] {
        if set.is_empty() {
            println!("PENDENCY-CONTRADICTION: clean (skipped: {} and its extra are empty)", knob);
            return Ok(0);
        }
    }
    let files = spec::manifest_files_sorted_stripped(root)?;
    let mut out: Vec<String> = Vec::new();
    let (mut n_pending, mut n_landed) = (0usize, 0usize);
    for f in &files {
        let text = spec::read_text(Path::new(f))?;
        let scan = scan(&text, &pending, &landed);
        n_pending += scan.pending;
        n_landed += scan.landed;
        for (token, (p, l)) in scan.both() {
            out.push(format!(
                "  {}:{}  `{}` pending here and landed at line {}",
                f, p, token, l
            ));
        }
    }
    if !out.is_empty() {
        println!("check-pendency-contradiction: a file binds one code span both to a pending construction and to a landed one:");
        println!();
        for l in &out {
            println!("{}", l);
        }
        println!("  help: the file says both: make the pending sentence past, or drop it; a deliberate pair takes '<!-- pendency-exempt: <reason> -->' on the binding's line or the one above (a reason is mandatory)");
        return Ok(1);
    }
    println!(
        "PENDENCY-CONTRADICTION: clean ({} manifest file(s), {} pending and {} landed binding(s); no code span bound both ways in one file)",
        files.len(),
        n_pending,
        n_landed
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-pendency-contradiction — one construction: the folded words on
// each side of its `{}` slot, either side possibly empty
#[derive(Debug, PartialEq)]
struct Phrase {
    before: String,
    after: String,
}

fn fold_words(s: &str) -> String {
    s.split_whitespace()
        .map(|w| w.to_ascii_lowercase())
        .collect::<Vec<String>>()
        .join(" ")
}

// spec: canon-kit/SPEC.md §Layout and configuration — the slot validator refuses a malformed member
// at every canon-kit gate, so a member reaching this reader carries one slot and a word beside it
fn phrases(members: &[String]) -> Vec<Phrase> {
    members
        .iter()
        .filter_map(|m| m.split_once("{}"))
        .map(|(b, a)| Phrase {
            before: fold_words(b),
            after: fold_words(a),
        })
        .filter(|p| !p.before.is_empty() || !p.after.is_empty())
        .collect()
}

// spec: canon-kit/SPEC.md §Layout and configuration — a construction member carries exactly one
// `{}` slot and at least one word beside it
pub(crate) fn slot_refusal(member: &str) -> Option<&'static str> {
    if member.matches("{}").count() != 1 {
        return Some("must carry exactly one '{}' slot");
    }
    if member.replace("{}", "").trim().is_empty() {
        return Some("carries nothing beside its '{}' slot");
    }
    None
}

#[derive(Default)]
struct Scan {
    pending: usize,
    landed: usize,
    first_pending: BTreeMap<String, usize>,
    first_landed: BTreeMap<String, usize>,
}

impl Scan {
    fn both(&self) -> Vec<(String, (usize, usize))> {
        self.first_pending
            .iter()
            .filter_map(|(t, p)| self.first_landed.get(t).map(|l| (t.clone(), (*p, *l))))
            .collect()
    }
}

// spec: canon-kit/SPEC.md §check-pendency-contradiction — a span holds an identifier-shaped token,
// one run of `[A-Za-z0-9._/-]`
fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'_' | b'/' | b'-'))
}

// spec: canon-kit/SPEC.md §check-pendency-contradiction — the paragraph's prose folded to
// lower case with whitespace runs made one space, each code span a sentinel; every span keeps its
// token and its offset in the joined paragraph, so a binding reports on its physical line
fn fold(joined: &str) -> (Vec<char>, Vec<(String, usize)>) {
    let b = joined.as_bytes();
    let mut out: Vec<char> = Vec::new();
    let mut spans: Vec<(String, usize)> = Vec::new();
    let mut i = 0usize;
    let mut space = false;
    while i < b.len() {
        if b[i] == b'`' {
            let mut n = 0usize;
            while i + n < b.len() && b[i + n] == b'`' {
                n += 1;
            }
            if let Some(close) = closing_run(b, i + n, n) {
                if space {
                    out.push(' ');
                    space = false;
                }
                let body = joined[i + n..close].trim();
                if identifier(body) {
                    out.push(BINDABLE);
                    spans.push((body.to_string(), i));
                } else {
                    out.push(INERT);
                }
                i = close + n;
                continue;
            }
            if space {
                out.push(' ');
                space = false;
            }
            out.extend(std::iter::repeat('`').take(n));
            i += n;
            continue;
        }
        let c = joined[i..].chars().next().unwrap_or(' ');
        if c.is_whitespace() {
            space = !out.is_empty();
        } else {
            if space {
                out.push(' ');
                space = false;
            }
            out.extend(c.to_lowercase());
        }
        i += c.len_utf8();
    }
    (out, spans)
}

fn closing_run(b: &[u8], from: usize, n: usize) -> Option<usize> {
    let mut j = from;
    while j < b.len() {
        if b[j] == b'`' {
            let mut m = 0usize;
            while j + m < b.len() && b[j + m] == b'`' {
                m += 1;
            }
            if m == n {
                return Some(j);
            }
            j += m;
            continue;
        }
        j += 1;
    }
    None
}

// spec: canon-kit/SPEC.md §check-pendency-contradiction — the separators a span list is joined by
const SEPARATORS: &[&str] = &[", and ", ", or ", ", ", " and ", " or "];

// spec: canon-kit/SPEC.md §check-pendency-contradiction — each maximal list of bindable spans: its
// first and last sentinel position in the folded prose and the span ordinals it covers
fn span_lists(folded: &[char]) -> Vec<(usize, usize, Vec<usize>)> {
    let sentinels: Vec<usize> = (0..folded.len()).filter(|&i| folded[i] == BINDABLE).collect();
    let mut out: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    let mut k = 0usize;
    while k < sentinels.len() {
        let start = sentinels[k];
        let mut members = vec![k];
        while k + 1 < sentinels.len() {
            let gap: String = folded[sentinels[k] + 1..sentinels[k + 1]].iter().collect();
            if !SEPARATORS.contains(&gap.as_str()) {
                break;
            }
            k += 1;
            members.push(k);
        }
        out.push((start, sentinels[k], members));
        k += 1;
    }
    out
}

fn word_char(c: Option<&char>) -> bool {
    c.is_some_and(|c| c.is_alphanumeric())
}

// spec: canon-kit/SPEC.md §check-pendency-contradiction — the slot binds the list directly after
// the words before it and directly before the words after it, a word boundary at each outer end
fn binds(folded: &[char], first: usize, last: usize, p: &Phrase) -> bool {
    if !p.before.is_empty() {
        let want: Vec<char> = format!("{} ", p.before).chars().collect();
        if first < want.len() || folded[first - want.len()..first] != want[..] {
            return false;
        }
        let at = first - want.len();
        if at > 0 && word_char(folded.get(at - 1)) {
            return false;
        }
    }
    if !p.after.is_empty() {
        let want: Vec<char> = format!(" {}", p.after).chars().collect();
        let from = last + 1;
        if from + want.len() > folded.len() || folded[from..from + want.len()] != want[..] {
            return false;
        }
        if word_char(folded.get(from + want.len())) {
            return false;
        }
    }
    true
}

fn valve_reason(line: &str) -> Option<String> {
    let at = line.find(VALVE)?;
    let rest = &line[at + VALVE.len()..];
    let rest = rest.split("-->").next().unwrap_or(rest);
    Some(rest.trim().to_string())
}

fn scan(text: &str, pending: &[Phrase], landed: &[Phrase]) -> Scan {
    let raw: Vec<&str> = text.lines().collect();
    let valved = |line: usize| {
        [Some(line - 1), (line - 1).checked_sub(1)]
            .iter()
            .flatten()
            .filter_map(|&i| raw.get(i).and_then(|l| valve_reason(l)))
            .any(|r| !r.is_empty())
    };
    let mut out = Scan::default();
    for para in paragraphs(&prose_only(text)) {
        let (folded, spans) = fold(&para.joined);
        for (first, last, members) in span_lists(&folded) {
            for (class, phrases) in [(0, pending), (1, landed)] {
                if !phrases.iter().any(|p| binds(&folded, first, last, p)) {
                    continue;
                }
                for &m in &members {
                    let (token, at) = &spans[m];
                    let line = para.line_of(*at);
                    if valved(line) {
                        continue;
                    }
                    let (count, firsts) = if class == 0 {
                        (&mut out.pending, &mut out.first_pending)
                    } else {
                        (&mut out.landed, &mut out.first_landed)
                    };
                    *count += 1;
                    firsts.entry(token.clone()).or_insert(line);
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sets() -> (Vec<Phrase>, Vec<Phrase>) {
        let v = |xs: &[&str]| phrases(&xs.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        (
            v(&["waits on {}", "lands after {}", "until {} lands", "{} does not exist yet"]),
            v(&["implemented by {}", "{} landed", "{} ships"]),
        )
    }

    fn both(text: &str) -> Vec<(String, (usize, usize))> {
        let (p, l) = sets();
        scan(text, &p, &l).both()
    }

    // spec: canon-kit/SPEC.md §check-pendency-contradiction — the incident's shape: a coordinated
    // list pending in one section and landed in another, one binding broken by a wrap
    #[test]
    fn a_list_pending_and_landed_in_one_file_reds_each_token() {
        let t = "## A\n\nIt lands after `a-x` and\n`b-y`.\n\n## B\n\nImplemented by `a-x`, `b-y`.\n";
        assert_eq!(
            both(t),
            vec![("a-x".to_string(), (3, 8)), ("b-y".to_string(), (4, 8))]
        );
    }

    // spec: canon-kit/SPEC.md §check-pendency-contradiction — adjacency: a pronoun, a word between
    // and a span quoting the phrase bind nothing
    #[test]
    fn only_a_directly_adjacent_span_binds() {
        assert!(both("`x` landed; `y` still waits on it.\n").is_empty());
        assert!(both("`x` landed.\n\nIt waits on the `x` gate.\n").is_empty());
        assert!(both("`x` landed.\n\nThe knob reads `waits on {}` and `x`.\n").is_empty());
        assert_eq!(both("`x` landed.\n\n`y` still waits on `x`.\n").len(), 1);
    }

    // spec: canon-kit/SPEC.md §check-pendency-contradiction — a slot between two words binds the
    // list both enclose, and matching folds case and whitespace
    #[test]
    fn a_middle_slot_and_case_and_whitespace_are_folded() {
        assert_eq!(both("Until  `x`\nLANDS, wait.\n\n`x` ships.\n").len(), 1);
        assert!(both("until `x` is ready.\n\n`x` ships.\n").is_empty());
    }

    // spec: canon-kit/SPEC.md §check-pendency-contradiction — a phrase needs a word boundary at
    // each outer end
    #[test]
    fn a_phrase_inside_a_longer_word_binds_nothing() {
        assert!(both("`x` landed.\n\nIt awaits on `x`.\n").is_empty());
        assert!(both("`x` landedness.\n\nIt waits on `x`.\n").is_empty());
    }

    // spec: canon-kit/SPEC.md §check-pendency-contradiction — fences, comments and generated
    // regions are held out, and the valve drops a binding only with a reason
    #[test]
    fn held_out_regions_and_the_valve() {
        assert!(both("`x` landed.\n\n```\nwaits on `x`\n```\n").is_empty());
        assert!(both("`x` landed.\n\n<!-- waits on `x` -->\n").is_empty());
        assert!(both("`x` landed.\n\n<!-- r:begin -->\nwaits on `x`\n<!-- r:end -->\n").is_empty());
        assert!(both("`x` landed.\n<!-- pendency-exempt: quoted -->\nwaits on `x`\n").is_empty());
        assert_eq!(both("`x` landed.\n<!-- pendency-exempt: -->\nwaits on `x`\n").len(), 1);
    }

    #[test]
    fn a_member_carries_one_slot_and_a_word_beside_it() {
        assert!(slot_refusal("waits on {}").is_none());
        assert!(slot_refusal("{} landed").is_none());
        assert!(slot_refusal("waits on").is_some());
        assert!(slot_refusal("{} and {}").is_some());
        assert!(slot_refusal(" {} ").is_some());
    }
}
