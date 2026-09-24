// spec: guard-kit/SPEC.md §The reader and its views — the PowerShell reader: quoting, comments,
// escapes and separators in PowerShell's grammar, and every other construct read as live.
use super::reader::{Reader, View, DQ_MARK, HD_MARK, SQ_MARK};
use super::text;

pub struct PowerShell;

#[derive(Clone, Copy, Default)]
struct Wants {
    sq: bool,
    dq: bool,
    hd: bool,
    hdq: bool,
}

impl Reader for PowerShell {
    fn view(&self, t: &str, view: View) -> Option<String> {
        let w = |sq, dq, hd, hdq| Wants { sq, dq, hd, hdq };
        let wants = match view {
            View::Raw => return Some(t.to_string()),
            View::Dequoted => return dequoted(t),
            View::Body => return Some(String::new()),
            View::Sq => w(true, false, false, false),
            View::Hdq => w(false, false, false, true),
            View::SqHdq => w(true, false, false, true),
            View::SqDqHd => w(true, true, true, false),
            View::SqDqHdq => w(true, true, false, true),
        };
        Some(text::chomp(&scan(t, wants).0).to_string())
    }

    fn body(&self, t: &str, k: usize) -> String {
        let bodies = scan(t, Wants::default()).1;
        k.checked_sub(1)
            .and_then(|i| bodies.get(i))
            .map(|b| text::chomp(b).to_string())
            .unwrap_or_default()
    }

    fn segments(&self, t: &str) -> Vec<String> {
        split_on(t, &[b"||", b"&&", b";", b"|", b"\n"])
    }

    fn statements(&self, t: &str) -> Vec<String> {
        split_on(t, &[b"||", b"&&", b";", b"\n"])
    }

    // spec: guard-kit/SPEC.md §The reader and its views — a here-string is a value inside its
    // statement, so no statement carries residue.
    fn residue_statements(&self, t: &str) -> Vec<String> {
        self.statements(t)
    }

    fn pipes(&self, t: &str) -> Vec<String> {
        split_on(t, &[b"|"])
    }

    fn redirect_pairs(&self, t: &str) -> Vec<String> {
        text::grep_o(REDIRECT_RE_SRC, t)
    }

    fn heredoc_terms(&self, _: &str) -> Vec<String> {
        Vec::new()
    }

    // spec: guard-kit/SPEC.md §The reader and its views — no wrapper strip is documented for the
    // PowerShell matcher.
    fn harness_view<'a>(&self, seg: &'a str) -> &'a str {
        seg
    }
}

// spec: guard-kit/SPEC.md §The reader and its views — the redirect pair: an operator, blanks, then
// an fd-dup or a target word; PowerShell reserves `<`.
const REDIRECT_RE_SRC: &str = "[1-6*]?>>?[[:space:]]*(&[1-6]|[^[:space:]|;&<>]+)";

// spec: guard-kit/SPEC.md §The reader and its views — a split over a skeleton, the two-character
// separators taken whole; a backtick escapes the byte after it, so that byte separates nothing.
fn split_on(cmd: &str, seps: &[&[u8]]) -> Vec<String> {
    let b = cmd.as_bytes();
    let mut segs: Vec<Vec<u8>> = vec![Vec::new()];
    let mut i = 0usize;
    while i < b.len() {
        let open = segs.last_mut().expect("one segment is always open");
        if b[i] == b'`' {
            open.extend_from_slice(&b[i..(i + 2).min(b.len())]);
            i += 2;
            continue;
        }
        match seps.iter().find(|s| b[i..].starts_with(s)) {
            Some(s) => {
                segs.push(Vec::new());
                i += s.len();
            }
            None => {
                open.push(b[i]);
                i += 1;
            }
        }
    }
    segs.iter().map(|s| String::from_utf8_lossy(s).into_owned()).collect()
}

// spec: guard-kit/SPEC.md §The reader and its views — a single-quoted span's end, past its closing
// quote; a doubled `''` is one literal quote. `None` when it never closes.
fn sq_end(b: &[u8], from: usize) -> Option<usize> {
    let mut j = from + 1;
    loop {
        let k = j + b.get(j..)?.iter().position(|&c| c == b'\'')?;
        if b.get(k + 1) == Some(&b'\'') {
            j = k + 2;
            continue;
        }
        return Some(k + 1);
    }
}

// spec: guard-kit/SPEC.md §The reader and its views — an expandable span's end: a doubled `""` and
// a backtick-escaped byte do not end it, and a `$(` inside it is not modelled.
fn dq_end(b: &[u8], from: usize) -> Option<usize> {
    let mut j = from + 1;
    loop {
        let k = j + b.get(j..)?.iter().position(|&c| c == b'"' || c == b'`')?;
        if b[k] == b'`' || b.get(k + 1) == Some(&b'"') {
            j = k + 2;
            continue;
        }
        return Some(k + 1);
    }
}

// spec: guard-kit/SPEC.md §The reader and its views — a here-string at `@`: its opener ends its line,
// and its terminator begins a later line after optional blanks. The literal's end, its body, and
// whether it is verbatim; `None` when this `@` opens no terminated here-string.
fn here_string(b: &[u8], at: usize) -> Option<(usize, String, bool)> {
    let q = *b.get(at + 1).filter(|c| matches!(c, b'\'' | b'"'))?;
    let nl = at + 2 + b[at + 2..].iter().position(|&c| c == b'\n')?;
    if !b[at + 2..nl].iter().all(|&c| matches!(c, b' ' | b'\t' | b'\r')) {
        return None;
    }
    let start = nl + 1;
    let mut ls = start;
    while ls <= b.len() {
        let le = b[ls..].iter().position(|&c| c == b'\n').map_or(b.len(), |k| ls + k);
        let lead = b[ls..le].iter().position(|&c| c != b' ' && c != b'\t').unwrap_or(le - ls);
        if b[ls + lead..le].starts_with(&[q, b'@']) {
            let body = if ls > start { &b[start..ls - 1] } else { &b[start..start] };
            return Some((ls + lead + 2, String::from_utf8_lossy(body).into_owned(), q == b'\''));
        }
        if le == b.len() {
            return None;
        }
        ls = le + 1;
    }
    None
}

// spec: guard-kit/SPEC.md §The reader and its views — a `#` opens a line comment only where it
// begins a token.
fn token_start(b: &[u8], i: usize) -> bool {
    i == 0 || matches!(b[i - 1], b'\n' | b' ' | b'\t' | b';' | b'|' | b'(')
}

// spec: guard-kit/SPEC.md §The reader and its views — a comment's extent from `i`, or `None` where no
// comment opens: a line comment runs to its line's end, a block comment through its `#>`.
fn comment_end(b: &[u8], i: usize) -> Option<usize> {
    if b[i] == b'<' && b.get(i + 1) == Some(&b'#') {
        let close = b[i + 2..].windows(2).position(|w| w == b"#>");
        return Some(close.map_or(b.len(), |k| i + 2 + k + 2));
    }
    if b[i] == b'#' && token_start(b, i) {
        return Some(b[i..].iter().position(|&c| c == b'\n').map_or(b.len(), |k| i + k));
    }
    None
}

// spec: guard-kit/SPEC.md §The reader and its views — the skeleton and every here-string body. A
// comment becomes one blank, a wanted class its placeholder, and an unterminated span stays live.
fn scan(cmd: &str, w: Wants) -> (String, Vec<String>) {
    let b = cmd.as_bytes();
    let n = b.len();
    let mut out: Vec<u8> = Vec::with_capacity(n);
    let mut bodies = Vec::new();
    let mut i = 0usize;
    while i < n {
        let c = b[i];
        if c == b'`' {
            out.extend_from_slice(&b[i..(i + 2).min(n)]);
            i += 2;
            continue;
        }
        if let Some(end) = comment_end(b, i) {
            out.push(b' ');
            i = end;
            continue;
        }
        if c == b'@' {
            if let Some((end, body, verbatim)) = here_string(b, i) {
                bodies.push(body);
                if w.hd || (w.hdq && verbatim) {
                    out.extend_from_slice(HD_MARK.as_bytes());
                } else {
                    out.extend_from_slice(&b[i..end]);
                }
                i = end;
                continue;
            }
        }
        let span = match c {
            b'\'' => Some((sq_end(b, i), w.sq, SQ_MARK)),
            b'"' => Some((dq_end(b, i), w.dq, DQ_MARK)),
            _ => None,
        };
        match span {
            Some((None, _, _)) => {
                out.extend_from_slice(&b[i..]);
                i = n;
            }
            Some((Some(end), wanted, mark)) => {
                if wanted {
                    out.extend_from_slice(mark.as_bytes());
                } else {
                    out.extend_from_slice(&b[i..end]);
                }
                i = end;
            }
            None => {
                out.push(c);
                i += 1;
            }
        }
    }
    (String::from_utf8_lossy(&out).into_owned(), bodies)
}

// spec: guard-kit/SPEC.md §The generic ruleset — a quoted span's blanks and separators held as the
// dequoted view's sentinels.
fn sentinel(c: u8) -> u8 {
    match c {
        b' ' => 0x01,
        b'\t' => 0x02,
        b';' => 0x03,
        b'|' => 0x04,
        b'&' => 0x05,
        o => o,
    }
}

// spec: guard-kit/SPEC.md §The reader and its views — the dequoted view, aligned with the `sq dq hd`
// skeleton: the quotes dropped and their escapes undone. `None` on a here-string, an unterminated
// span, a newline inside quotes, or an escape that spells a control character.
fn dequoted(raw: &str) -> Option<String> {
    let b = raw.as_bytes();
    let n = b.len();
    let mut out: Vec<u8> = Vec::with_capacity(n);
    let mut i = 0usize;
    while i < n {
        let c = b[i];
        if c == b'`' {
            out.extend_from_slice(&b[i..(i + 2).min(n)]);
            i += 2;
            continue;
        }
        if let Some(end) = comment_end(b, i) {
            out.push(b' ');
            i = end;
            continue;
        }
        if c == b'@' && here_string(b, i).is_some() {
            return None;
        }
        let content: Vec<u8> = match c {
            b'\'' => {
                let end = sq_end(b, i)?;
                let inner = &b[i + 1..end - 1];
                i = end;
                let mut v = Vec::with_capacity(inner.len());
                let mut k = 0usize;
                while k < inner.len() {
                    v.push(inner[k]);
                    k += if inner[k] == b'\'' { 2 } else { 1 };
                }
                v
            }
            b'"' => {
                let end = dq_end(b, i)?;
                let inner = &b[i + 1..end - 1];
                i = end;
                let mut v = Vec::with_capacity(inner.len());
                let mut k = 0usize;
                while k < inner.len() {
                    match inner[k] {
                        b'`' => {
                            let e = *inner.get(k + 1)?;
                            if b"0abefnrtuv".contains(&e) {
                                return None;
                            }
                            v.push(e);
                            k += 2;
                        }
                        b'"' => {
                            v.push(b'"');
                            k += 2;
                        }
                        o => {
                            v.push(o);
                            k += 1;
                        }
                    }
                }
                v
            }
            _ => {
                out.push(c);
                i += 1;
                continue;
            }
        };
        if content.contains(&b'\n') {
            return None;
        }
        out.extend(content.into_iter().map(sentinel));
    }
    Some(text::chomp(&String::from_utf8_lossy(&out)).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guard::bash::Bash;

    fn sk(t: &str) -> String {
        PowerShell.view(t, View::SqDqHd).unwrap()
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the four inert classes in PowerShell's
    // grammar: a doubled quote and a backtick escape keep a span open
    #[test]
    fn each_inert_class_maps_to_its_powershell_span() {
        assert_eq!(sk("echo 'a''; cd x' ; ls"), format!("echo {SQ_MARK} ; ls"));
        assert_eq!(sk("echo \"a`\"; cd x\" ; ls"), format!("echo {DQ_MARK} ; ls"));
        assert_eq!(sk("echo \"a\"\"; cd x\" ; ls"), format!("echo {DQ_MARK} ; ls"));
        assert_eq!(sk("echo 'a'''; cd x'"), format!("echo {SQ_MARK}; cd x'"));
        let verbatim = "@'\n$x; cd y\n'@ | git commit -F -";
        let expandable = "@\"\n$x\n  \"@ | git commit -F -";
        assert_eq!(sk(verbatim), format!("{HD_MARK} | git commit -F -"));
        assert_eq!(sk(expandable), format!("{HD_MARK} | git commit -F -"));
        assert_eq!(PowerShell.view(verbatim, View::SqHdq).unwrap(), format!("{HD_MARK} | git commit -F -"));
        assert_eq!(PowerShell.view(expandable, View::SqHdq).unwrap(), expandable);
        assert_eq!(PowerShell.view("echo 'a' \"b\"", View::Sq).unwrap(), format!("echo {SQ_MARK} \"b\""));
    }

    // spec: guard-kit/SPEC.md §The reader and its views — a here-string is one placeholder, opener to
    // terminator, so its statement stays on one skeleton line; no terminator leaves it live
    #[test]
    fn a_here_string_is_one_placeholder_or_none_at_all() {
        let c = "@'\ncd x; rm tracked.md\n'@ | git commit -F -";
        assert_eq!(PowerShell.segments(&sk(c)), vec![HD_MARK.to_string() + " ", " git commit -F -".to_string()]);
        assert_eq!(PowerShell.body(c, 1), "cd x; rm tracked.md");
        assert_eq!(PowerShell.body("@'\n'@", 1), "");
        assert_eq!(PowerShell.body(c, 2), "");
        assert_eq!(sk("@' trailing\nx\n'@"), format!("@{SQ_MARK}@"));
        assert_eq!(sk("@'\nnever closes"), "@'\nnever closes");
        assert!(PowerShell.heredoc_terms(c).is_empty());
        assert_eq!(PowerShell.residue_statements("a; b | c"), vec!["a", " b | c"]);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — a comment is one blank in every skeleton and
    // kept in the raw view; a `#` inside a word opens nothing
    #[test]
    fn a_comment_becomes_one_blank_and_raw_keeps_it() {
        assert_eq!(sk("# cd x; ls"), " ");
        assert_eq!(sk("git status # cd x; ls"), "git status  ");
        assert_eq!(sk("<# cd x; ls\n#> git status"), "  git status");
        assert_eq!(sk("ls a#b;c"), "ls a#b;c");
        assert_eq!(sk("ls;# x"), "ls; ");
        assert_eq!(sk("a<#x#>b"), "a b");
        assert_eq!(PowerShell.view("# cd x; ls", View::Raw).unwrap(), "# cd x; ls");
    }

    // spec: guard-kit/SPEC.md §The reader and its views — a backtick escapes the next byte: no span
    // opens, no separator cuts, and a line continues
    #[test]
    fn a_backtick_escape_opens_no_span_and_cuts_no_segment() {
        assert_eq!(PowerShell.segments("git status `; cd x"), vec!["git status `; cd x"]);
        assert_eq!(PowerShell.segments("a `| b"), vec!["a `| b"]);
        assert_eq!(PowerShell.segments("git log `\n --oneline"), vec!["git log `\n --oneline"]);
        assert_eq!(sk("echo `'; cd x"), "echo `'; cd x");
        assert_eq!(sk("echo 'a`'; cd x"), format!("echo {SQ_MARK}; cd x"));
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the three splits and the separators they
    // take; a trailing `&` separates nothing
    #[test]
    fn the_splits_cut_on_the_documented_separators() {
        assert_eq!(PowerShell.segments("a||b&&c;d|e\nf"), vec!["a", "b", "c", "d", "e", "f"]);
        assert_eq!(PowerShell.segments("a &"), vec!["a &"]);
        assert_eq!(PowerShell.statements("a | b;c||d"), vec!["a | b", "c", "d"]);
        assert_eq!(PowerShell.pipes("a | b"), vec!["a ", " b"]);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — PowerShell's redirect operators and no read
    // redirect
    #[test]
    fn the_redirect_pairs_take_powershell_s_operators() {
        assert_eq!(
            PowerShell.redirect_pairs("git status *> out.log 2>&1 3>> x"),
            vec!["*> out.log", "2>&1", "3>> x"]
        );
        assert!(PowerShell.redirect_pairs("sort < in.txt").is_empty());
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the dequoted view undoes each span's own
    // escapes, holds its separators as sentinels, and refuses what it cannot align
    #[test]
    fn the_dequoted_view_undoes_the_escapes_or_refuses() {
        assert_eq!(dequoted("rm 'a''b c'").as_deref(), Some("rm a'b\x01c"));
        assert_eq!(dequoted("rm \"a`\"b\"\"c;\"").as_deref(), Some("rm a\"b\"c\x03"));
        assert_eq!(dequoted("git status `; x").as_deref(), Some("git status `; x"));
        assert_eq!(dequoted("ls # 'x").as_deref(), Some("ls  "));
        assert_eq!(dequoted("@'\nx\n'@"), None);
        assert_eq!(dequoted("echo 'open"), None);
        assert_eq!(dequoted("echo \"a\nb\""), None);
        assert_eq!(dequoted("echo \"a`nb\""), None);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the harness view is the identity
    #[test]
    fn the_harness_view_strips_nothing() {
        assert_eq!(PowerShell.harness_view("timeout 5 git log"), "timeout 5 git log");
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the reader's stated bounds: typographic
    // quotes read live, and an expandable span ends at its first unescaped quote inside `$(`
    #[test]
    fn the_stated_bounds_hold() {
        assert_eq!(sk("echo ‘a; b’"), "echo ‘a; b’");
        assert_eq!(sk("echo \"$(x \"y\")\""), format!("echo {DQ_MARK}y{DQ_MARK}"));
    }

    // spec: guard-kit/SPEC.md §The reader and its views — where the two grammars differ, the same text
    // reads differently through each reader
    #[test]
    fn the_two_readers_part_where_the_grammars_do() {
        let both = |t: &str| (Bash.view(t, View::SqDqHd).unwrap(), sk(t));
        assert_eq!(both("echo 'a''b'"), (format!("echo {SQ_MARK}{SQ_MARK}"), format!("echo {SQ_MARK}")));
        assert_eq!(both("echo \"a\\\"b\"").0, format!("echo {DQ_MARK}"));
        assert_eq!(both("echo \"a\\\"b\"").1, format!("echo {DQ_MARK}b\""));
        assert_eq!(Bash.segments("a `; b").len(), 2);
        assert_eq!(PowerShell.segments("a `; b").len(), 1);
        assert_eq!(Bash.segments("a |& b").len(), 2);
        assert_eq!(PowerShell.segments("a |& b"), vec!["a ", "& b"]);
    }
}
