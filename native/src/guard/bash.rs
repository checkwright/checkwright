// spec: guard-kit/SPEC.md §The guard framework — the bash reader: the one holder of the normalizer,
// the splitters, the redirect scan and the harness view, which the rules, the `scan-prompts` ranker
// and `--emit-compare-settings-allow` all call.
use super::reader::{Reader, View};
use super::text::{self, is_space, trim_start};
use crate::ere::EreError;

pub struct Bash;

impl Reader for Bash {
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
        Some(text::chomp(&skeleton(t, wants)).to_string())
    }

    fn body(&self, t: &str, k: usize) -> String {
        let bodies = scan(t, Wants::default()).2;
        k.checked_sub(1)
            .and_then(|i| bodies.get(i))
            .map(|b| text::chomp(b).to_string())
            .unwrap_or_default()
    }

    fn segments(&self, t: &str) -> Vec<String> {
        split_compound(t)
    }

    fn statements(&self, t: &str) -> Vec<String> {
        statements(t)
    }

    fn residue_statements(&self, t: &str) -> Vec<String> {
        residue_statements(t)
    }

    fn pipes(&self, t: &str) -> Vec<String> {
        t.split(['|', '\n']).map(String::from).collect()
    }

    fn redirect_pairs(&self, t: &str) -> Vec<String> {
        text::grep_o(REDIRECT_RE_SRC, t)
    }

    fn heredoc_terms(&self, t: &str) -> Vec<String> {
        heredoc_terms(t)
    }

    fn harness_view<'a>(&self, seg: &'a str) -> &'a str {
        harness_view(seg)
    }
}

// spec: guard-kit/SPEC.md §The guard framework — the compound split: one segment per line, split on
// the harness's statement separators. `||`, `&&` and `|&` are tested before `|`, the leftmost-longest
// alternation a `sed -E` gives for free and a scanner must spell.
pub fn split_compound(cmd: &str) -> Vec<String> {
    split_on(cmd, &[b"||", b"&&", b"|&", b";", b"|", b"\n"])
}

// spec: guard-kit/SPEC.md §The generic ruleset — the statement split, where a pipe is not a boundary.
pub fn statements(cmd: &str) -> Vec<String> {
    split_on(cmd, &[b"||", b"&&", b";", b"\n"])
}

fn split_on(cmd: &str, seps: &[&[u8]]) -> Vec<String> {
    let b = cmd.as_bytes();
    let mut segs: Vec<Vec<u8>> = vec![Vec::new()];
    let mut i = 0usize;
    while i < b.len() {
        match seps.iter().find(|s| b[i..].starts_with(s)) {
            Some(s) => {
                segs.push(Vec::new());
                i += s.len();
            }
            None => {
                segs.last_mut().expect("one segment is always open").push(b[i]);
                i += 1;
            }
        }
    }
    segs.iter()
        .map(|s| String::from_utf8_lossy(s).into_owned())
        .collect()
}

// spec: guard-kit/SPEC.md §The generic ruleset — the heredoc opener as the terminator scan reads it,
// which matches from any `<` and so also finds the `<<` a herestring's second `<` begins.
const TERM_RE_SRC: &str = "<<-?[[:space:]]*(\"[^\"]*\"|'[^']*'|[A-Za-z_][A-Za-z0-9_]*)";

pub fn heredoc_terms(line: &str) -> Vec<String> {
    text::grep_o(TERM_RE_SRC, line)
        .into_iter()
        .map(|t| {
            let t = t.split_once("<<").map_or(t.as_str(), |(_, r)| r);
            let t = t.strip_prefix('-').unwrap_or(t);
            let t = trim_start(t);
            let t = t.strip_prefix(['"', '\'']).unwrap_or(t);
            let t = t.strip_suffix(['"', '\'']).unwrap_or(t);
            t.to_string()
        })
        .collect()
}

// spec: guard-kit/SPEC.md §The generic ruleset — each statement of a skeleton followed by the residue
// its own openers produce; a line whose openers sit in more than one statement attributes its residue
// to none.
pub fn residue_statements(s: &str) -> Vec<String> {
    let lines: Vec<&str> = s.split('\n').collect();
    let at = |i: usize| lines.get(i).copied().unwrap_or("");
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        i += 1;
        let mut residue = String::new();
        for t in heredoc_terms(line) {
            let mut next = at(i);
            if trim_start(next) == "HD" {
                residue.push('\n');
                residue.push_str(next);
                i += 1;
                next = at(i);
            }
            if trim_start(next) != t {
                break;
            }
            residue.push('\n');
            residue.push_str(next);
            i += 1;
        }
        let parts = statements(line);
        let carriers = parts.iter().filter(|p| p.contains("<<")).count();
        for part in parts {
            if part.bytes().all(is_space) {
                continue;
            }
            if part.contains("<<") && carriers == 1 {
                out.push(format!("{}{}", part, residue));
            } else {
                out.push(part);
            }
        }
    }
    out
}

// spec: guard-kit/SPEC.md §The guard framework — the four inert classes the normalizer takes
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Wants {
    pub sq: bool,
    pub dq: bool,
    pub hd: bool,
    pub hdq: bool,
}

// spec: guard-kit/SPEC.md §The guard framework — the harness view's word cursor: the head word and
// the blanks after it, over a slice that never carries leading blanks.
struct Words<'a>(&'a str);

impl<'a> Words<'a> {
    fn peek(&self) -> Option<&'a str> {
        if self.0.is_empty() {
            return None;
        }
        let i = self.0.bytes().position(is_space).unwrap_or(self.0.len());
        Some(&self.0[..i])
    }

    fn pop(&mut self) -> Option<&'a str> {
        let w = self.peek()?;
        self.0 = trim_start(&self.0[w.len()..]);
        Some(w)
    }
}

fn glued(w: &str, prefix: &str) -> bool {
    w.len() > prefix.len() && w.starts_with(prefix)
}

fn is_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit())
}

fn is_int(w: &str) -> bool {
    is_digits(w.strip_prefix(['+', '-']).unwrap_or(w))
}

// spec: guard-kit/SPEC.md §The guard framework — `timeout`'s one duration word:
// `^([0-9]+\.?[0-9]*|\.[0-9]+)[smhd]?$`.
fn is_duration(w: &str) -> bool {
    let n = w.strip_suffix(['s', 'm', 'h', 'd']).unwrap_or(w);
    if let Some(frac) = n.strip_prefix('.') {
        return is_digits(frac);
    }
    let int = n.bytes().take_while(u8::is_ascii_digit).count();
    if int == 0 {
        return false;
    }
    let rest = &n[int..];
    let rest = rest.strip_prefix('.').unwrap_or(rest);
    rest.is_empty() || is_digits(rest)
}

fn walk_timeout(cur: &mut Words) -> bool {
    while let Some(w) = cur.peek() {
        match w {
            "--preserve-status" | "--foreground" | "-v" | "--verbose" => {
                cur.pop();
            }
            "-s" | "-k" | "--signal" | "--kill-after" => {
                cur.pop();
                if cur.pop().is_none() {
                    return false;
                }
            }
            _ if glued(w, "-s") || glued(w, "-k") || glued(w, "--signal=")
                || glued(w, "--kill-after=") =>
            {
                cur.pop();
            }
            _ => break,
        }
    }
    cur.pop().is_some_and(is_duration)
}

// spec: guard-kit/SPEC.md §The guard framework — `nice`'s glued adjustments:
// `^(-n[+-]?|--adjustment=[+-]?|-)[0-9]+$`.
fn walk_nice(cur: &mut Words) -> bool {
    while let Some(w) = cur.peek() {
        if w == "-n" {
            cur.pop();
            if !cur.pop().is_some_and(is_int) {
                return false;
            }
        } else if w
            .strip_prefix("-n")
            .or_else(|| w.strip_prefix("--adjustment="))
            .is_some_and(is_int)
            || w.strip_prefix('-').is_some_and(is_digits)
        {
            cur.pop();
        } else {
            break;
        }
    }
    true
}

fn walk_stdbuf(cur: &mut Words) -> bool {
    while let Some(w) = cur.peek() {
        match w {
            "-i" | "-o" | "-e" => {
                cur.pop();
                if cur.pop().is_none() {
                    return false;
                }
            }
            _ if glued(w, "-i") || glued(w, "-o") || glued(w, "-e") || glued(w, "--input=")
                || glued(w, "--output=") || glued(w, "--error=") =>
            {
                cur.pop();
            }
            _ => break,
        }
    }
    true
}

// spec: guard-kit/SPEC.md §The guard framework — a leading assignment:
// `^[A-Za-z_][A-Za-z0-9_]*=[^"']*$`.
fn is_assignment(w: &str) -> bool {
    let Some((name, value)) = w.split_once('=') else {
        return false;
    };
    let b = name.as_bytes();
    !b.is_empty()
        && (b[0].is_ascii_alphabetic() || b[0] == b'_')
        && b.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_')
        && !value.contains(['"', '\''])
}

// spec: guard-kit/SPEC.md §The guard framework — the harness view: a segment as the permission
// matcher reads it, its documented leading wrappers stripped from the head repeatedly. An option a
// wrapper's walk does not recognize, or a wrapper left with nothing to wrap, stops the strip there.
pub fn harness_view(seg: &str) -> &str {
    let mut rest = trim_start(seg);
    loop {
        let mut cur = Words(rest);
        let Some(head) = cur.pop() else { break };
        let walked = match head {
            "time" => {
                if cur.peek() == Some("-p") {
                    cur.pop();
                }
                true
            }
            "timeout" => walk_timeout(&mut cur),
            "nice" => walk_nice(&mut cur),
            "stdbuf" => walk_stdbuf(&mut cur),
            "nohup" | "builtin" | "noglob" | "command" | "xargs" => true,
            h => is_assignment(h),
        };
        match cur.peek() {
            Some(w) if walked && !w.starts_with('-') => rest = cur.0,
            _ => break,
        }
    }
    rest
}

// spec: guard-kit/SPEC.md §The guard framework — `<<-?[[:space:]]*(<quoted>|<identifier>)`, anchored.
// One byte decides the alternative and `[[:space:]]` shares none of their first-character sets, so the
// greedy run needs no backtracking.
fn heredoc_header(s: &[u8]) -> Option<(usize, &[u8], bool)> {
    if !s.starts_with(b"<<") {
        return None;
    }
    let mut j = 2;
    if s.get(j) == Some(&b'-') {
        j += 1;
    }
    while j < s.len() && is_space(s[j]) {
        j += 1;
    }
    let first = *s.get(j)?;
    match first {
        b'"' | b'\'' => {
            let k = s[j + 1..].iter().position(|&c| c == first)?;
            Some((j + 1 + k + 1, &s[j + 1..j + 1 + k], true))
        }
        c if c.is_ascii_alphabetic() || c == b'_' => {
            let mut e = j + 1;
            while e < s.len() && (s[e].is_ascii_alphanumeric() || s[e] == b'_') {
                e += 1;
            }
            Some((e, &s[j..e], false))
        }
        _ => None,
    }
}

enum State {
    None,
    Sq,
    Dq,
}

// spec: guard-kit/SPEC.md §The guard framework — the normalizer, the whole machinery
pub fn skeleton(cmd: &str, w: Wants) -> String {
    scan(cmd, w).0
}

// spec: guard-kit/SPEC.md §scan-prompts — each heredoc body with its terminator line, as byte ranges
// of the command: the extent the skeleton's `hd` arm blanks, which the ranker's grant test drops
pub fn heredoc_extents(cmd: &str) -> Vec<std::ops::Range<usize>> {
    scan(cmd, Wants::default()).1
}

type Scanned = (String, Vec<std::ops::Range<usize>>, Vec<String>);

fn scan(cmd: &str, w: Wants) -> Scanned {
    let b = cmd.as_bytes();
    let n = b.len();
    let mut out: Vec<u8> = Vec::with_capacity(n);
    let mut span: Vec<u8> = Vec::new();
    let mut state = State::None;
    let mut pending: std::collections::VecDeque<(&[u8], bool)> = std::collections::VecDeque::new();
    let mut extents = Vec::new();
    let mut bodies = Vec::new();
    let mut i = 0usize;
    while i < n {
        match state {
            State::Sq => {
                match b[i..].iter().position(|&c| c == b'\'') {
                    None => {
                        span.extend_from_slice(&b[i..]);
                        i = n;
                    }
                    Some(k) => {
                        span.extend_from_slice(&b[i..i + k]);
                        span.push(b'\'');
                        i += k + 1;
                        if w.sq {
                            out.extend_from_slice(b"SQ");
                        } else {
                            out.extend_from_slice(&span);
                        }
                        span.clear();
                        state = State::None;
                    }
                }
                continue;
            }
            State::Dq => {
                let k = b[i..]
                    .iter()
                    .position(|&c| c == b'"' || c == b'\\')
                    .unwrap_or(n - i);
                span.extend_from_slice(&b[i..i + k]);
                i += k;
                if i >= n {
                    continue;
                }
                if b[i] == b'\\' {
                    span.extend_from_slice(&b[i..(i + 2).min(n)]);
                    i += 2;
                    continue;
                }
                span.push(b'"');
                i += 1;
                if w.dq {
                    out.extend_from_slice(b"DQ");
                } else {
                    out.extend_from_slice(&span);
                }
                span.clear();
                state = State::None;
                continue;
            }
            State::None => {}
        }

        let k = b[i..]
            .iter()
            .position(|&c| c == b'"' || c == b'\'' || c == b'\\' || c == b'<' || c == b'\n')
            .unwrap_or(n - i);
        if k > 0 {
            out.extend_from_slice(&b[i..i + k]);
            i += k;
            if i >= n {
                continue;
            }
        }
        let ch = b[i];
        if ch == b'\'' {
            state = State::Sq;
            span.clear();
            span.push(b'\'');
            i += 1;
            continue;
        }
        if ch == b'"' {
            state = State::Dq;
            span.clear();
            span.push(b'"');
            i += 1;
            continue;
        }
        if ch == b'\\' {
            out.extend_from_slice(&b[i..(i + 2).min(n)]);
            i += 2;
            continue;
        }
        if ch == b'<' {
            if b[i..].starts_with(b"<<<") {
                out.extend_from_slice(b"<<<");
                i += 3;
                continue;
            }
            if let Some((len, term, quoted)) = heredoc_header(&b[i..]) {
                out.extend_from_slice(&b[i..i + len]);
                i += len;
                pending.push_back((term, quoted));
                continue;
            }
        }
        if ch == b'\n' {
            out.push(b'\n');
            i += 1;
            while let Some((term, quoted)) = pending.pop_front() {
                let start = i;
                let mut body: Vec<u8> = Vec::new();
                while i < n {
                    let end = b[i..].iter().position(|&c| c == b'\n').map_or(n, |k| i + k);
                    let line = &b[i..end];
                    let lead = line.iter().position(|&c| !is_space(c)).unwrap_or(line.len());
                    if &line[lead..] == term {
                        break;
                    }
                    body.extend_from_slice(line);
                    body.push(b'\n');
                    i = (end + 1).min(n);
                }
                bodies.push(String::from_utf8_lossy(&body).into_owned());
                if !body.is_empty() {
                    if w.hd || (w.hdq && quoted) {
                        out.extend_from_slice(b"HD\n");
                    } else {
                        out.extend_from_slice(&body);
                    }
                }
                if i < n {
                    let end = b[i..].iter().position(|&c| c == b'\n').map_or(n, |k| i + k);
                    out.extend_from_slice(&b[i..end]);
                    i = end;
                    extents.push(start..end);
                    if i < n {
                        out.push(b'\n');
                        i += 1;
                    }
                } else {
                    extents.push(start..n);
                }
            }
            continue;
        }
        // spec: guard-kit/SPEC.md §The guard framework — placeholder, never deletion: a construct
        // that survives the scan is live, so an unrecognized `<` is one byte of the command again.
        out.push(ch);
        i += 1;
    }
    if !span.is_empty() {
        out.extend_from_slice(&span);
    }
    (String::from_utf8_lossy(&out).into_owned(), extents, bodies)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the dequoted view, walked in lockstep with the
// `sq dq hd` skeleton, a quoted span's blanks and separators held as sentinels; `None` where the two
// cannot be aligned.
pub fn dequoted(raw: &str) -> Option<String> {
    let skel = skeleton(raw, Wants { sq: true, dq: true, hd: true, hdq: false });
    let s = text::chomp(&skel).as_bytes();
    let r = raw.as_bytes();
    let mut out: Vec<u8> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    let lit = |c: u8| c == b'"' || c == b'\'' || c == b'\\';
    let at = |v: &[u8], from: usize, len: usize| v.get(from..(from + len).min(v.len())).unwrap_or(&[]).to_vec();
    while j < r.len() {
        let k = r[j..].iter().position(|&c| lit(c)).unwrap_or(r.len() - j);
        if at(s, i, k) != r[j..j + k] {
            return None;
        }
        out.extend_from_slice(&r[j..j + k]);
        i += k;
        j += k;
        if j >= r.len() {
            break;
        }
        let c = r[j];
        if c == b'\\' {
            if at(s, i, 2) != at(r, j, 2) {
                return None;
            }
            out.extend_from_slice(&at(r, j, 2));
            i += 2;
            j += 2;
            continue;
        }
        let span: Vec<u8>;
        if c == b'\'' {
            if at(s, i, 2) != b"SQ" {
                return None;
            }
            let k = r[j + 1..].iter().position(|&c| c == b'\'')?;
            span = r[j + 1..j + 1 + k].to_vec();
            j += k + 2;
        } else {
            if at(s, i, 2) != b"DQ" {
                return None;
            }
            let mut k = j + 1;
            let mut acc: Vec<u8> = Vec::new();
            loop {
                let m = r[k..].iter().position(|&c| c == b'"' || c == b'\\')?;
                acc.extend_from_slice(&r[k..k + m]);
                k += m;
                if r[k] != b'\\' {
                    break;
                }
                acc.extend_from_slice(&at(r, k, 2));
                k += 2;
                if k > r.len() {
                    return None;
                }
            }
            span = acc;
            j = k + 1;
        }
        i += 2;
        if span.contains(&b'\n') {
            return None;
        }
        for b in span {
            out.push(match b {
                b' ' => 0x01,
                b'\t' => 0x02,
                b';' => 0x03,
                b'|' => 0x04,
                b'&' => 0x05,
                o => o,
            });
        }
    }
    if i != s.len() {
        return None;
    }
    Some(text::chomp(&String::from_utf8_lossy(&out)).to_string())
}

// spec: guard-kit/SPEC.md §The generic ruleset — the redirect-pair pattern, handed to the crate's own
// matcher (gate-sdk/SPEC.md §The POSIX ERE matcher)
pub const REDIRECT_RE_SRC: &str = "[0-9]*>>?[[:space:]]*(&[0-9-]+|[^[:space:]|;&<>]+)";

// spec: guard-kit/SPEC.md §The generic ruleset — one match per line, leftmost-longest and
// non-overlapping, which is `grep -o`'s contract. Line-wise rather than whole-string, because
// `[[:space:]]` matches a newline and a whole-string scan would join two lines into one pair.
pub fn redirect_pairs(text: &str) -> Result<Vec<String>, EreError> {
    crate::ere::Ere::compile(REDIRECT_RE_SRC)?;
    Ok(text::grep_o(REDIRECT_RE_SRC, text))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SQDQ: Wants = Wants { sq: true, dq: true, hd: false, hdq: false };
    const HD: Wants = Wants { sq: true, dq: true, hd: true, hdq: false };
    const HDQ: Wants = Wants { sq: true, dq: false, hd: false, hdq: true };

    // spec: guard-kit/SPEC.md §The guard framework — the splitter's separator class and the
    // longest-match rule: `||` is one boundary, not two, and a trailing separator opens a segment
    #[test]
    fn the_splitter_takes_the_longest_separator_and_keeps_the_empty_tail() {
        assert_eq!(split_compound("a;b&&c||d|e"), vec!["a", "b", "c", "d", "e"]);
        assert_eq!(split_compound("echo trailing;"), vec!["echo trailing", ""]);
        assert_eq!(split_compound(""), vec![""]);
        assert_eq!(split_compound("a & b"), vec!["a & b"]);
        assert_eq!(split_compound("a |& b"), vec!["a ", " b"]);
        assert_eq!(split_compound("a\nb;c\n"), vec!["a", "b", "c", ""]);
        assert_eq!(statements("a | b;c||d |& e"), vec!["a | b", "c", "d |& e"]);
    }

    // spec: guard-kit/SPEC.md §The guard framework — placeholder, never deletion
    #[test]
    fn the_skeleton_substitutes_the_inert_spans_and_leaves_the_rest_byte_identical() {
        assert_eq!(skeleton("echo 'a;b' && ls", SQDQ), "echo SQ && ls");
        assert_eq!(skeleton("echo \"a && b\" | wc -l", SQDQ), "echo DQ | wc -l");
        assert_eq!(skeleton("echo 'a;b' && ls", Wants::default()), "echo 'a;b' && ls");
        assert_eq!(skeleton("grep -oE \"a\\\"b\" file", SQDQ), "grep -oE DQ file");
        assert_eq!(skeleton("echo 'unterminated", SQDQ), "echo 'unterminated");
    }

    // spec: guard-kit/SPEC.md §The guard framework — an opener with no line after it has no body
    #[test]
    fn the_heredoc_opener_survives_verbatim_and_never_becomes_a_placeholder() {
        for c in [
            "cat <<EOF",
            "cat <<-EOF",
            "cat <<'EOF'",
            "cat <<\"EOF\"",
            "x <<  SPACED",
            "x << \"Q S\"",
            "x <<9BAD",
            "x <<",
        ] {
            assert_eq!(skeleton(c, HD), c, "opener {:?} did not survive", c);
        }
        assert_eq!(skeleton("cat <<<\"here string\"", HD), "cat <<<DQ");
    }

    // spec: guard-kit/SPEC.md §The guard framework — the body runs to its terminator line, which
    // stays live; `hd` blanks every body and `hdq` only a quoted-delimiter one
    #[test]
    fn a_heredoc_body_is_blanked_by_its_class_and_the_terminator_stays_live() {
        let c = "python3 - <<'PY'\nprint('a;b' | c)\nPY\nls";
        assert_eq!(skeleton(c, HD), "python3 - <<'PY'\nHD\nPY\nls");
        assert_eq!(skeleton(c, HDQ), "python3 - <<'PY'\nHD\nPY\nls");
        assert_eq!(skeleton(c, SQDQ), "python3 - <<'PY'\nprint('a;b' | c)\nPY\nls");
        assert_eq!(skeleton("cat <<EOF\n$HOME\nEOF", HDQ), "cat <<EOF\n$HOME\nEOF");
        assert_eq!(skeleton("cat <<-EOF\nx\n\tEOF", HD), "cat <<-EOF\nHD\n\tEOF");
        assert_eq!(skeleton("a <<A <<B\n1\nA\n2\nB", HD), "a <<A <<B\nHD\nA\nHD\nB");
        assert_eq!(skeleton("cat <<EOF\nnever ends", HD), "cat <<EOF\nHD\n");
        assert_eq!(skeleton("cat <<EOF\nnever ends", SQDQ), "cat <<EOF\nnever ends\n");
        assert_eq!(skeleton("cat <<EOF\nEOF", HD), "cat <<EOF\nEOF");
        assert_eq!(Bash.body("a <<A <<B\n1\nA\n2\n3\nB", 2), "2\n3");
        assert_eq!(Bash.body("a <<A\n1\nA", 2), "");
    }

    // spec: guard-kit/SPEC.md §scan-prompts — the extent is the body plus its terminator line
    #[test]
    fn the_heredoc_extent_covers_the_body_and_the_terminator_line() {
        let c = "python3 - <<'PY'\nx = 1\nPY\nperl -e 1";
        let e = heredoc_extents(c);
        assert_eq!(e.len(), 1);
        assert_eq!(&c[e[0].clone()], "x = 1\nPY");
        let u = "cat <<EOF\nnever ends";
        assert_eq!(&u[heredoc_extents(u)[0].clone()], "never ends");
        assert!(heredoc_extents("cat <<EOF").is_empty());
    }

    // spec: guard-kit/SPEC.md §The guard framework — the strip walks each wrapper's own grammar,
    // nests, and stops at the wrapper whose arguments it cannot walk or that wraps nothing
    #[test]
    fn the_harness_view_strips_the_documented_wrappers_and_stops_where_a_walk_fails() {
        assert_eq!(harness_view("nice -n 5 timeout -s KILL 1.5s time -p git log"), "git log");
        assert_eq!(harness_view("FOO=1 stdbuf -oL xargs grep x"), "grep x");
        assert_eq!(harness_view("sudo git log"), "sudo git log");
        assert_eq!(harness_view("command -v git"), "command -v git");
        assert_eq!(harness_view("xargs -0 grep x"), "xargs -0 grep x");
        assert_eq!(harness_view("timeout abc git log"), "timeout abc git log");
        assert_eq!(harness_view("time"), "time");
        assert_eq!(harness_view("FOO='a b' git log"), "FOO='a b' git log");
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — operator and target together, fd-dups
    // included, and a read redirect is not a pair
    #[test]
    fn the_redirect_scan_reports_operator_and_target_and_keeps_fd_dups() {
        assert_eq!(redirect_pairs("sort -rn > out.txt").unwrap(), vec!["> out.txt"]);
        assert_eq!(redirect_pairs("cmd 1>>log 2>&-").unwrap(), vec!["1>>log", "2>&-"]);
        assert_eq!(redirect_pairs("cmd 2>&1").unwrap(), vec!["2>&1"]);
        assert!(redirect_pairs("wc -l < in.txt").unwrap().is_empty());
        assert_eq!(
            redirect_pairs("jq . < a.json > b.json").unwrap(),
            vec!["> b.json"]
        );
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the dequoted view keeps content, drops quote
    // characters, holds a quoted span's separators as sentinels, and refuses what it cannot align
    #[test]
    fn the_dequoted_view_aligns_with_the_skeleton_or_refuses() {
        assert_eq!(dequoted("grep 'a b' f").as_deref(), Some("grep a\x01b f"));
        assert_eq!(dequoted("echo \"x;y\" | wc").as_deref(), Some("echo x\x03y | wc"));
        assert_eq!(dequoted("echo a\\ b").as_deref(), Some("echo a\\ b"));
        assert_eq!(dequoted("echo 'open"), None);
        assert_eq!(dequoted("cat <<EOF\nx\nEOF"), None);
        assert_eq!(dequoted("echo 'a\nb'"), None);
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the terminators, including the one a herestring's
    // second `<` opens, and each statement carrying only its own residue
    #[test]
    fn heredoc_residue_follows_the_one_statement_that_opened_it() {
        assert_eq!(heredoc_terms("cat <<'EOF' >> f"), vec!["EOF"]);
        assert_eq!(heredoc_terms("cat <<<\"x\""), vec!["x"]);
        assert_eq!(
            residue_statements("cat <<'EOF' >> f; ls\nHD\nEOF"),
            vec!["cat <<'EOF' >> f\nHD\nEOF", " ls"]
        );
        assert_eq!(residue_statements("a && b\n\nc"), vec!["a ", " b", "c"]);
    }
}
