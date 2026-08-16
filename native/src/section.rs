// spec: gate-sdk/SPEC.md §Porting a gate to the binary substrate — the heading-bounded section
// walk, held once for the members whose shell forms each hand-rolled it

// spec: context-kit/SPEC.md §The brevity gate — a heading is a `#` run followed by POSIX
// whitespace and its level is that run's length; every other line is level 0
pub fn heading_level(line: &str) -> usize {
    let b = line.as_bytes();
    let mut n = 0usize;
    while n < b.len() && b[n] == b'#' {
        n += 1;
    }
    if n == 0 {
        return 0;
    }
    match b.get(n) {
        Some(c) if is_posix_space(*c) => n,
        _ => 0,
    }
}

fn is_posix_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

// spec: gate-sdk/SPEC.md §Porting a gate to the binary substrate — the primitive yields a line
// range and nothing else; the opening level bounds the walk but is never a field a caller reads
pub struct Section {
    pub start: usize,
    pub end: usize,
}

// spec: context-kit/SPEC.md §The brevity gate — the section opens at a heading whose line begins
// with the caller's name and closes at the next heading of the same level or shallower; an empty
// result is the absent-section verdict each caller fails closed on
pub fn sections(lines: &[&str], name: &str) -> Vec<Section> {
    let mut out: Vec<Section> = Vec::new();
    let mut open: Option<(usize, usize)> = None;
    for (i, line) in lines.iter().enumerate() {
        let lvl = heading_level(line);
        match open {
            Some((level, start)) => {
                // spec: doctrine-kit/SPEC.md §check-doctrine-registration — the closing heading is
                // consumed, so it cannot itself reopen a section whose name it happens to carry
                if lvl > 0 && lvl <= level {
                    out.push(Section { start, end: i });
                    open = None;
                }
            }
            None => {
                if lvl > 0 && line.starts_with(name) {
                    open = Some((lvl, i + 1));
                }
            }
        }
    }
    if let Some((_, start)) = open {
        out.push(Section {
            start,
            end: lines.len(),
        });
    }
    out
}

pub struct Item {
    pub start: usize,
    pub end: usize,
}

// spec: doctrine-kit/SPEC.md §check-doctrine-registration — the same bounding rule one level down:
// an item opens at a caller-recognised lead line and closes at the next lead or the range's end
pub fn items(lines: &[&str], is_lead: impl Fn(&str) -> bool) -> Vec<Item> {
    let mut out: Vec<Item> = Vec::new();
    let mut open: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        if is_lead(line) {
            if let Some(start) = open {
                out.push(Item { start, end: i });
            }
            open = Some(i);
        }
    }
    if let Some(start) = open {
        out.push(Item {
            start,
            end: lines.len(),
        });
    }
    out
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — awk's record split, so a file with or without a final
// newline yields the same record count in both substrates
pub fn split_lines(text: &str) -> Vec<&str> {
    let mut v: Vec<&str> = text.split('\n').collect();
    if v.last() == Some(&"") {
        v.pop();
    }
    v
}

pub fn blank(line: &str) -> bool {
    !line.bytes().any(|b| !is_posix_space(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_deeper_heading_does_not_close_a_section_and_a_shallower_one_does() {
        let text = "# a\n## Target\nbody\n### deeper\nmore\n## Sibling\nout\n";
        let lines = split_lines(text);
        let secs = sections(&lines, "## Target");
        assert_eq!(secs.len(), 1);
        assert_eq!(&lines[secs[0].start..secs[0].end], &["body", "### deeper", "more"]);
    }

    #[test]
    fn an_absent_section_yields_nothing_and_a_trailing_one_runs_to_the_end() {
        let lines = split_lines("## Other\nx\n");
        assert!(sections(&lines, "## Target").is_empty());
        let lines = split_lines("## Target\na\nb\n");
        let secs = sections(&lines, "## Target");
        assert_eq!(&lines[secs[0].start..secs[0].end], &["a", "b"]);
    }

    // spec: doctrine-kit/SPEC.md §check-doctrine-registration — the closing heading is consumed
    // rather than re-tested, and a later match opens a second range
    #[test]
    fn a_closing_heading_carrying_the_name_is_consumed_and_a_later_one_reopens() {
        let lines = split_lines("## T\na\n## T\nb\n## T\nc\n");
        let secs = sections(&lines, "## T");
        assert_eq!(secs.len(), 2);
        assert_eq!(&lines[secs[0].start..secs[0].end], &["a"]);
        assert_eq!(&lines[secs[1].start..secs[1].end], &["c"]);
    }

    #[test]
    fn a_hash_run_without_trailing_whitespace_is_not_a_heading() {
        assert_eq!(heading_level("## x"), 2);
        assert_eq!(heading_level("##x"), 0);
        assert_eq!(heading_level("#"), 0);
        assert_eq!(heading_level("text ## x"), 0);
        assert_eq!(heading_level("###### x"), 6);
    }

    #[test]
    fn an_item_runs_to_the_next_lead_or_the_ranges_end() {
        let lines = split_lines("- **a**\nx\n\n- **b**\ny\n");
        let got = items(&lines, |l| l.starts_with("- **"));
        assert_eq!(got.len(), 2);
        assert_eq!(&lines[got[0].start..got[0].end], &["- **a**", "x", ""]);
        assert_eq!(&lines[got[1].start..got[1].end], &["- **b**", "y"]);
        assert!(items(&lines, |l| l.starts_with("1. ")).is_empty());
    }
}
