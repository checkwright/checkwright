// spec: canon-kit/SPEC.md §check-manifest-count — two arms over manifest prose outside an exempt
// site: (A) no bare cardinal quantifying a governed collection noun, (B) a cardinal opening a
// list's lead-in agrees with the list
use super::manifest_temporal::TemporalValve;
use crate::spec;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-manifest-count: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-manifest-count — the second sanctioned discharge rides the same
// per-site window the exempt tag opens, for both arms
const EXEMPTS: &[&str] = &["manifest-count-exempt:", spec::MEASURED_MARKER];

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }

    let manifests = spec::manifest_files_sorted_stripped(root)?;
    if manifests.is_empty() {
        println!("MANIFEST-COUNT: clean (0 manifest file(s) found)");
        return Ok(0);
    }

    // spec: canon-kit/SPEC.md §check-manifest-count — prose walk only: the shared driver
    // gates fences and per-site markers, these hooks judge the line and the wrapped paragraph
    struct Sink {
        grammar: spec::CountGrammar,
        prev: String,
        out: Vec<String>,
    }
    impl spec::ProseSink for Sink {
        fn on_line(&mut self, file: &str, fnr: usize, raw: &str) {
            if let Some(hit) = self.grammar.hit_after(&self.prev, raw) {
                self.out
                    .push(format!("  {}:{}  restated collection total: {}", file, fnr, hit));
            }
            self.prev = raw.to_string();
        }
        fn on_pflush(&mut self, file: &str, para: &spec::Para) {
            self.prev.clear();
            if let Some((fnr, hit)) = spec::para_wrapped(&self.grammar, para) {
                self.out
                    .push(format!("  {}:{}  restated collection total: {}", file, fnr, hit));
            }
        }
    }
    let mut sink = Sink {
        grammar: spec::CountGrammar::resolve()?,
        prev: String::new(),
        out: Vec::new(),
    };
    spec::walk_prose_multi(&manifests, EXEMPTS, &mut sink)?;
    let out = sink.out;

    // spec: canon-kit/SPEC.md §check-manifest-count — arm B's path test is the temporal path
    // valve alone, so the member declares no section knob
    let valve = TemporalValve::new(
        Vec::new(),
        spec::knob_array_pub("CANON_KIT_TEMPORAL_EXEMPT_PATHS")?,
    );
    let mut read = 0usize;
    let mut lead: Vec<String> = Vec::new();
    for f in manifests.iter().filter(|f| !valve.path_exempt(f)) {
        let text = spec::read_text(Path::new(f))?;
        for (ln, cardinal, items) in lead_ins(&text, &mut read) {
            lead.push(format!(
                "  {}:{}  lead-in count: '{}' over {} item(s)",
                f, ln, cardinal, items
            ));
        }
    }

    if !out.is_empty() || !lead.is_empty() {
        if !out.is_empty() {
            println!("check-manifest-count: bare count(s) quantifying a governed collection in manifest prose — the count's owner is the collection, a restated total drifts silently:");
            println!();
            for l in &out {
                println!("{}", l);
            }
        }
        if !lead.is_empty() {
            println!("check-manifest-count: lead-in count(s) disagreeing with the list below them — the list is the count's owner:");
            println!();
            for l in &lead {
                println!("{}", l);
            }
        }
        if !out.is_empty() {
            println!("  help: reword to cite the owning collection (e.g. 'the gates in gates.list') rather than pin a total; a total worth keeping takes a '<!-- measured: <key>=<value> -->' marker on the line above, or inline after the sentence, which binds it to an oracle check-measured-claim re-runs; a genuinely fixed named set joins CANON_KIT_COUNT_ALLOWED_PHRASES; a threshold/rate/partition/proportion is already exempt; else add a 'manifest-count-exempt: <reason>' comment on the line or the one above");
        }
        if !lead.is_empty() {
            println!("  help: a lead-in count disagreeing with its list — make the cardinal match, or drop it ('The cases:')");
        }
        return Ok(1);
    }
    println!("MANIFEST-COUNT: clean ({} manifest file(s), {} lead-in(s) read; no bare cardinal quantifying a governed collection, and no lead-in count disagreeing with its list, in prose outside an exempt site)", manifests.len(), read);
    Ok(0)
}

fn indent(s: &str) -> usize {
    s.bytes()
        .take_while(|b| matches!(b, b' ' | b'\t'))
        .map(|b| if b == b'\t' { 4 } else { 1 })
        .sum()
}

// spec: canon-kit/SPEC.md §check-manifest-count — arm B over one file: each lead-in whose last
// sentence opens with a cardinal disagreeing with its list, as (line, cardinal, item count);
// `read` counts every lead-in compared
fn lead_ins(text: &str, read: &mut usize) -> Vec<(usize, String, usize)> {
    let lines: Vec<&str> = text.lines().collect();
    let n = lines.len();
    let mut fence = spec::Fence::default();
    let fenced: Vec<bool> = lines
        .iter()
        .map(|l| fence.delimits(l) || fence.is_open())
        .collect();
    let marked = |i: usize| EXEMPTS.iter().any(|e| lines[i].contains(e));
    let boundary = |i: usize| {
        fenced[i]
            || spec::is_blank(lines[i])
            || spec::prose_heading_level(lines[i]) > 0
            || spec::is_table_row(lines[i])
            || marked(i)
    };
    let mut out = Vec::new();
    for i in 0..n {
        if boundary(i) || !lines[i].trim_end().ends_with(':') {
            continue;
        }
        let mut j = i + 1;
        if j < n && spec::is_blank(lines[j]) {
            j += 1;
        }
        if j >= n || fenced[j] || !spec::is_list_item(lines[j]) {
            continue;
        }
        let mut s = i;
        while s > 0 && !spec::is_list_item(lines[s]) && !boundary(s - 1) {
            s -= 1;
        }
        // spec: canon-kit/SPEC.md §check-manifest-count — a list item leads only a list nested
        // under it; its next sibling is no list it introduces
        if spec::is_list_item(lines[s]) && indent(lines[j]) <= indent(lines[s]) {
            continue;
        }
        let mut joined = String::new();
        let mut starts: Vec<usize> = Vec::new();
        for l in &lines[s..=i] {
            if !joined.is_empty() {
                joined.push(' ');
            }
            starts.push(joined.len());
            joined.push_str(l.trim());
        }
        let Some(&(st, en)) = spec::sentence_spans(&joined).last() else {
            continue;
        };
        let Some((cardinal, value)) = opening_cardinal(&joined[st..en]) else {
            continue;
        };
        let ln = s + starts.iter().rposition(|&o| o <= st).unwrap_or(0);
        if marked(ln) || (ln > 0 && marked(ln - 1)) {
            continue;
        }
        *read += 1;
        let items = list_items(&lines, &fenced, j);
        if items as u64 != value {
            out.push((ln + 1, cardinal, items));
        }
    }
    out
}

// spec: canon-kit/SPEC.md §check-manifest-count — the opener past a leading list marker,
// emphasis markers and an optional `The`, read in the count adapter's grammar outside inline code
fn opening_cardinal(sentence: &str) -> Option<(String, u64)> {
    let mut t = sentence.trim_start();
    if spec::is_list_item(t) {
        t = t.trim_start_matches(|c: char| c.is_ascii_digit());
        t = t[1..].trim_start();
    }
    let mut t = t.trim_start_matches(['*', '_']);
    if t.get(..4).is_some_and(|p| p.eq_ignore_ascii_case("the ")) {
        t = t[4..].trim_start().trim_start_matches(['*', '_']);
    }
    let word = t.bytes().take_while(|c| c.is_ascii_alphanumeric()).count();
    if word == 0 {
        return None;
    }
    let rest = &t[word..];
    let after = rest.trim_start_matches('_');
    if after.len() < rest.len() && after.chars().next().is_some_and(char::is_alphanumeric) {
        return None;
    }
    let tok = &t[..word];
    let value = if tok.bytes().all(|c| c.is_ascii_digit()) {
        tok.parse::<u64>().unwrap_or(u64::MAX)
    } else {
        spec::cardinal_word_value(&tok.to_ascii_lowercase())?
            .parse::<u64>()
            .ok()?
    };
    Some((tok.to_string(), value))
}

// spec: canon-kit/SPEC.md §check-manifest-count — the items at the first item's indent; deeper
// lines are nested content, and the list ends at a line no deeper than an item that opens none
fn list_items(lines: &[&str], fenced: &[bool], j: usize) -> usize {
    let base = indent(lines[j]);
    let mut items = 0usize;
    for (k, l) in lines.iter().enumerate().skip(j) {
        if spec::is_blank(l) || (fenced[k] && spec::fence_opening(l).is_none()) {
            continue;
        }
        let ind = indent(l);
        if spec::is_list_item(l) && ind == base {
            items += 1;
        } else if ind <= base {
            break;
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(text: &str) -> (usize, Vec<(usize, String, usize)>) {
        let mut read = 0;
        let out = lead_ins(text, &mut read);
        (read, out)
    }

    #[test]
    fn matching_and_disagreeing_lead_ins() {
        let (read, out) = scan("Two cases:\n\n- a\n- b\n\nThe three arms:\n- a\n- b\n");
        assert_eq!(read, 2);
        assert_eq!(out, vec![(6, "three".to_string(), 2)]);
    }

    #[test]
    fn nested_items_and_blank_lines_between_items_count_once() {
        let (read, out) =
            scan("Intro. Three kinds:\n\n- a\n  - a1\n\n- b\n  more\n- c\n\nAfter.\n- d\n");
        assert_eq!(read, 1);
        assert!(out.is_empty(), "{:?}", out);
    }

    #[test]
    fn mid_sentence_cardinal_and_code_opener_are_unread() {
        let (read, _) = scan("It splits in two ways:\n- a\n\n`Two` spellings:\n- a\n");
        assert_eq!(read, 0);
    }

    #[test]
    fn exempt_window_and_list_item_lead_in() {
        let (read, out) = scan(
            "<!-- manifest-count-exempt: fixed -->\nFour forms:\n- a\n\n- **Two** modes:\n  - x\n- sibling\n",
        );
        assert_eq!(read, 1);
        assert_eq!(out, vec![(5, "Two".to_string(), 1)]);
    }

    #[test]
    fn multibyte_underscore_emphasis_and_an_oversized_digit_run() {
        let (read, out) = scan(
            "Café items:\n- a\n\n日本語:\n- a\n\n_Two_ cases:\n- a\n\n__Two__ cases:\n- a\n\nTwo_ways:\n- a\n\n18446744073709551616 cases:\n- a\n",
        );
        assert_eq!(read, 3);
        assert_eq!(
            out,
            vec![
                (7, "Two".to_string(), 1),
                (10, "Two".to_string(), 1),
                (16, "18446744073709551616".to_string(), 1),
            ]
        );
    }

    #[test]
    fn a_fence_inside_an_item_is_nested_content() {
        let (_, out) = scan("Two steps:\n\n1. run\n   ```\n- not an item\n   ```\n2. read\n");
        assert!(out.is_empty(), "{:?}", out);
    }
}
