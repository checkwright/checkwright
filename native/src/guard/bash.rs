// spec: guard-kit/SPEC.md §The shell guard — the bash reader: the one holder of the normalizer,
// the splitters, the redirect scan and the harness view, which the rules, the `scan-prompts` ranker
// and `--emit-compare-settings-allow` all call.
use super::reader::{Reader, View, Word, WordAcc, DQ_MARK, HD_MARK, SQ_MARK};
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
        split_on(t, &[b"|", b"\n"])
    }

    fn redirect_pairs(&self, t: &str) -> Vec<String> {
        text::grep_o(REDIRECT_RE_SRC, t)
    }

    fn heredoc_terms(&self, t: &str) -> Vec<String> {
        heredoc_terms(t)
    }

    fn heredoc_openers(&self, t: &str) -> Vec<String> {
        heredoc_openers(t)
    }

    fn harness_view<'a>(&self, seg: &'a str) -> &'a str {
        harness_view(seg)
    }

    fn word_read(&self, t: &str) -> Option<Vec<Vec<Word>>> {
        word_read(t)
    }
}

const COMPOUND_SEPS: &[&[u8]] = &[b"||", b"&&", b"|&", b";", b"|", b"\n"];

// spec: guard-kit/SPEC.md §The shell guard — the compound split: one segment per line, split on
// the harness's statement separators. `||`, `&&` and `|&` are tested before `|`, the leftmost-longest
// alternation a `sed -E` gives for free and a scanner must spell.
pub fn split_compound(cmd: &str) -> Vec<String> {
    split_on(cmd, COMPOUND_SEPS)
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
            Some(s) if escapable(s[0]) && escaped(b, i) => {
                let open = segs.last_mut().expect("one segment is always open");
                if s[0] == b'\n' {
                    open.pop();
                } else {
                    open.push(b[i]);
                }
                i += 1;
            }
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

// spec: guard-kit/SPEC.md §The reader and its views — the escaped separator; `&&` stays bytewise, and
// a continued line drops its backslash-newline so no segment carries a newline a word reader stops at.
fn escapable(c: u8) -> bool {
    matches!(c, b';' | b'|' | b'\n')
}

fn escaped(b: &[u8], i: usize) -> bool {
    b[..i].iter().rev().take_while(|&&c| c == b'\\').count() % 2 == 1
}

// spec: guard-kit/SPEC.md §The generic ruleset — the heredoc opener as the terminator scan reads it,
// which matches from any `<` and so also finds the `<<` a herestring's second `<` begins; a
// placeholder there is a word, so a quoted herestring opens one as a bare-word herestring does.
const TERM_RE_SRC: &str = "<<-?[[:space:]]*(\"[^\"]*\"|'[^']*'|\u{0}?[A-Za-z_][A-Za-z0-9_]*)";

pub fn heredoc_openers(line: &str) -> Vec<String> {
    text::grep_o(TERM_RE_SRC, line)
}

pub fn heredoc_terms(line: &str) -> Vec<String> {
    heredoc_openers(line)
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
            if trim_start(next) == HD_MARK {
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

// spec: guard-kit/SPEC.md §The shell guard — the four inert classes the normalizer takes
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Wants {
    pub sq: bool,
    pub dq: bool,
    pub hd: bool,
    pub hdq: bool,
}

// spec: guard-kit/SPEC.md §The shell guard — the harness view's word cursor: the head word and
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

// spec: guard-kit/SPEC.md §The shell guard — `timeout`'s one duration word:
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

// spec: guard-kit/SPEC.md §The shell guard — `nice`'s glued adjustments:
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

// spec: guard-kit/SPEC.md §The shell guard — a leading assignment:
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

// spec: guard-kit/SPEC.md §The shell guard — the harness view: a segment as the permission
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

// spec: guard-kit/SPEC.md §The shell guard — `<<-?[[:space:]]*(<quoted>|<identifier>)`, anchored.
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
    AnsiC,
    Dq,
}

// spec: guard-kit/SPEC.md §The shell guard — the normalizer, the whole machinery
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
            State::Sq | State::AnsiC => {
                let close = match state {
                    State::AnsiC => ansi_c_close(&b[i..]),
                    _ => b[i..].iter().position(|&c| c == b'\''),
                };
                match close {
                    None => {
                        span.extend_from_slice(&b[i..]);
                        i = n;
                    }
                    Some(k) => {
                        span.extend_from_slice(&b[i..i + k]);
                        span.push(b'\'');
                        i += k + 1;
                        if w.sq {
                            out.extend_from_slice(SQ_MARK.as_bytes());
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
                    out.extend_from_slice(DQ_MARK.as_bytes());
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
            state = if dollar_quote(b, i) { State::AnsiC } else { State::Sq };
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
                        out.extend_from_slice(HD_MARK.as_bytes());
                        out.push(b'\n');
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
        // spec: guard-kit/SPEC.md §The shell guard — placeholder, never deletion: a construct
        // that survives the scan is live, so an unrecognized `<` is one byte of the command again.
        out.push(ch);
        i += 1;
    }
    if !span.is_empty() {
        out.extend_from_slice(&span);
    }
    (String::from_utf8_lossy(&out).into_owned(), extents, bodies)
}

// spec: guard-kit/SPEC.md §The reader and its views — the dequoted view; `None` where it and the
// skeleton cannot be aligned.
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
            out.push(b'\\');
            out.extend(at(r, j + 1, 1).into_iter().map(|b| match b {
                b' ' => 0x01,
                b'\t' => 0x02,
                o => o,
            }));
            i += 2;
            j += 2;
            continue;
        }
        let span: Vec<u8>;
        if c == b'\'' {
            if at(s, i, SQ_MARK.len()) != SQ_MARK.as_bytes() {
                return None;
            }
            let k = if dollar_quote(r, j) {
                ansi_c_close(&r[j + 1..])?
            } else {
                r[j + 1..].iter().position(|&c| c == b'\'')?
            };
            span = r[j + 1..j + 1 + k].to_vec();
            j += k + 2;
        } else {
            if at(s, i, DQ_MARK.len()) != DQ_MARK.as_bytes() {
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
        i += DQ_MARK.len();
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

// spec: guard-kit/SPEC.md §The reader and its views — a `$`-quoted word: a quote behind an odd run of
// bare `$`, less an escaped first `$`, since `$$` is the shell's pid. A continuation is removed before
// bash reads the word, so the run is counted across one.
fn dollar_quote(b: &[u8], i: usize) -> bool {
    let (mut at, mut run) = (i, 0usize);
    loop {
        while at >= 2 && b[at - 1] == b'\n' && b[at - 2] == b'\\' && !escaped(b, at - 2) {
            at -= 2;
        }
        if at == 0 || b[at - 1] != b'$' {
            break;
        }
        run += 1;
        at -= 1;
    }
    let live = if run > 0 && escaped(b, at) { run - 1 } else { run };
    live % 2 == 1
}

// spec: guard-kit/SPEC.md §The reader and its views — the first byte at or after `i` that no
// continuation removes.
fn past_continuations(b: &[u8], mut i: usize) -> usize {
    while b.get(i) == Some(&b'\\') && b.get(i + 1) == Some(&b'\n') {
        i += 2;
    }
    i
}

// spec: guard-kit/SPEC.md §The reader and its views — where bash ends an ANSI-C span: the first quote
// no backslash escapes, as an offset into the text after the opening quote.
fn ansi_c_close(b: &[u8]) -> Option<usize> {
    let mut k = 0usize;
    while k < b.len() {
        match b[k] {
            b'\'' => return Some(k),
            b'\\' => k += 2,
            _ => k += 1,
        }
    }
    None
}

// spec: guard-kit/SPEC.md §The reader and its views — an ANSI-C quoted span's value as bash resolves
// its escapes; `None` where the read refuses the span.
fn ansi_c(content: &[u8]) -> Option<Vec<u8>> {
    let hex = |s: &[u8], max: usize| -> (u32, usize) {
        let n = s.iter().take(max).take_while(|c| c.is_ascii_hexdigit()).count();
        let v = s[..n].iter().fold(0u32, |v, &c| v * 16 + (c as char).to_digit(16).unwrap_or(0));
        (v, n)
    };
    let mut out = Vec::with_capacity(content.len());
    let mut k = 0usize;
    while k < content.len() {
        let c = content[k];
        k += 1;
        if c != b'\\' {
            out.push(c);
            continue;
        }
        let &e = content.get(k)?;
        k += 1;
        let resolved: u32 = match e {
            b'n' => 0x0a,
            b't' => 0x09,
            b'r' => 0x0d,
            b'a' => 0x07,
            b'b' => 0x08,
            b'e' | b'E' => 0x1b,
            b'f' => 0x0c,
            b'v' => 0x0b,
            b'\\' | b'\'' | b'"' | b'?' => e as u32,
            b'0'..=b'7' => {
                let n = content[k - 1..].iter().take(3).take_while(|c| (b'0'..=b'7').contains(c)).count();
                let v = content[k - 1..k - 1 + n].iter().fold(0u32, |v, &c| v * 8 + (c - b'0') as u32);
                k += n - 1;
                v % 256
            }
            b'x' | b'u' | b'U' => {
                let (v, n) = hex(&content[k..], match e { b'x' => 2, b'u' => 4, _ => 8 });
                if n == 0 {
                    out.extend_from_slice(&[b'\\', e]);
                    continue;
                }
                k += n;
                if e != b'x' && v >= 0x80 {
                    return None;
                }
                v
            }
            b'c' => {
                let Some(&x) = content.get(k) else {
                    out.extend_from_slice(b"\\c");
                    continue;
                };
                k += if x == b'\\' && content.get(k + 1) == Some(&b'\\') { 2 } else { 1 };
                if !x.is_ascii() {
                    return None;
                }
                if x == b'?' { 0x7f } else { (x.to_ascii_uppercase() & 0x1f) as u32 }
            }
            _ => {
                out.extend_from_slice(&[b'\\', e]);
                continue;
            }
        };
        if resolved >= 0x80 || (resolved < 0x20 && !matches!(resolved, 0x09 | 0x0a | 0x0d)) {
            return None;
        }
        out.push(resolved as u8);
    }
    Some(out)
}

// spec: guard-kit/SPEC.md §The reader and its views — the word read; `None` where the dequoted view
// cannot be aligned outside a heredoc, or where the walk and the compound split of the skeleton part.
pub fn word_read(raw: &str) -> Option<Vec<Vec<Word>>> {
    let t = text::chomp(raw);
    let (_, extents, bodies) = scan(t, Wants::default());
    let heredoc = !extents.is_empty();
    if !heredoc {
        dequoted(raw)?;
    }
    let b = t.as_bytes();
    let mut acc = WordAcc::new();
    let mut i = 0usize;
    let (mut at, mut term_at) = (0usize, None::<usize>);
    let bare = |acc: &mut WordAcc, bytes: &[u8]| {
        for &c in bytes {
            if is_space(c) {
                acc.end_word();
            } else {
                acc.piece(&[c], &[c], &[c]);
            }
        }
    };
    while i < b.len() {
        if let Some(e) = extents.get(at) {
            if i == e.start && term_at.is_none() {
                let held = bodies[at].len();
                if held == 0 {
                    term_at = Some(i);
                } else {
                    let end = (e.start + held - 1).min(b.len());
                    acc.piece(&b[i..end], HD_MARK.as_bytes(), HD_MARK.as_bytes());
                    term_at = Some(end + 1);
                    i = end;
                    continue;
                }
            }
            if term_at == Some(i) {
                bare(&mut acc, &b[i..e.end]);
                i = e.end;
                at += 1;
                term_at = None;
                continue;
            }
        }
        match b[i] {
            b'\\' => {
                match b.get(i + 1) {
                    Some(b'\n') => {}
                    Some(c) => acc.piece(&b[i..i + 2], &[*c], &b[i..i + 2]),
                    None => acc.piece(b"\\", b"", b"\\"),
                }
                i += 2;
            }
            b'$' if b.get(past_continuations(b, i + 1)) == Some(&b'\'') && dollar_quote(b, past_continuations(b, i + 1)) => {
                let q = past_continuations(b, i + 1);
                let k = ansi_c_close(&b[q + 1..])?;
                let content = &b[q + 1..q + 1 + k];
                if content.contains(&b'\n') {
                    return None;
                }
                let live = format!("${SQ_MARK}");
                acc.ansi_c_piece(&b[i..q + k + 2], &ansi_c(content)?, live.as_bytes());
                i = q + k + 2;
            }
            b'\'' => {
                let k = b[i + 1..].iter().position(|&c| c == b'\'')?;
                if heredoc && b[i + 1..i + 1 + k].contains(&b'\n') {
                    return None;
                }
                acc.piece(&b[i..i + k + 2], &b[i + 1..i + 1 + k], SQ_MARK.as_bytes());
                i += k + 2;
            }
            b'"' => {
                if dollar_quote(b, i) {
                    return None;
                }
                let mut value: Vec<u8> = Vec::new();
                let mut k = i + 1;
                loop {
                    match *b.get(k)? {
                        b'"' => break,
                        b'\\' => {
                            match *b.get(k + 1)? {
                                b'\n' => {}
                                c @ (b'$' | b'`' | b'"' | b'\\') => value.push(c),
                                c => value.extend_from_slice(&[b'\\', c]),
                            }
                            k += 2;
                        }
                        c => {
                            value.push(c);
                            k += 1;
                        }
                    }
                }
                if heredoc && b[i..=k].contains(&b'\n') {
                    return None;
                }
                acc.piece(&b[i..=k], &value, DQ_MARK.as_bytes());
                i = k + 1;
            }
            b' ' | b'\t' => {
                acc.end_word();
                i += 1;
            }
            b'<' if b[i..].starts_with(b"<<<") => {
                bare(&mut acc, b"<<<");
                i += 3;
            }
            b'<' if heredoc_header(&b[i..]).is_some() => {
                let len = heredoc_header(&b[i..]).map_or(1, |h| h.0);
                bare(&mut acc, &b[i..i + len]);
                i += len;
            }
            c => match COMPOUND_SEPS.iter().find(|s| b[i..].starts_with(s)) {
                Some(s) => {
                    acc.end_segment();
                    i += s.len();
                }
                None => {
                    acc.piece(&[c], &[c], &[c]);
                    i += 1;
                }
            },
        }
    }
    let read = acc.finish();
    let skel = skeleton(raw, Wants { sq: true, dq: true, hd: true, hdq: false });
    let segs = split_compound(text::chomp(&skel));
    let aligned = segs.len() == read.len()
        && segs.iter().zip(&read).all(|(seg, ws)| {
            let held = text::held_words(seg);
            held.len() == ws.len() && held.iter().zip(ws).all(|(h, w)| *h == w.live)
        });
    aligned.then_some(read)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the redirect-pair pattern, handed to the crate's own
// matcher (gate-sdk/SPEC.md §The POSIX ERE matcher)
pub const REDIRECT_RE_SRC: &str = "[0-9]*>[>&]?[[:blank:]]*(&[0-9-]+|[^[:blank:]|;&<>]+)";

// spec: guard-kit/SPEC.md §The generic ruleset — one match per line, leftmost-longest and
// non-overlapping, which is `grep -o`'s contract. Line-wise, because a target class excluding only
// blanks would run a whole-string scan's pair across a newline.
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

    // spec: guard-kit/SPEC.md §The shell guard — the splitter's separator class and the
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

    // spec: guard-kit/SPEC.md §The reader and its views — the escaped `;`
    #[test]
    fn the_splitter_keeps_a_semicolon_after_an_odd_backslash_run() {
        assert_eq!(split_compound("a \\; b"), vec!["a \\; b"]);
        assert_eq!(split_compound("a \\\\; b"), vec!["a \\\\", " b"]);
        assert_eq!(split_compound("a \\\\\\; b"), vec!["a \\\\\\; b"]);
        assert_eq!(statements("a \\; b"), vec!["a \\; b"]);
        let quoted = skeleton("a '\\;' b", SQDQ);
        assert!(!quoted.contains('\\'));
        assert_eq!(split_compound(&quoted).len(), 1);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the escaped `|` and the continued line
    #[test]
    fn the_splits_keep_a_pipe_or_newline_after_an_odd_backslash_run() {
        assert_eq!(split_compound("a \\| b"), vec!["a \\| b"]);
        assert_eq!(split_compound("a \\\\| b"), vec!["a \\\\", " b"]);
        assert_eq!(split_compound("a \\|| b"), vec!["a \\|", " b"]);
        assert_eq!(split_compound("a \\|& b"), vec!["a \\|& b"]);
        assert_eq!(split_compound("a \\\nb"), vec!["a b"]);
        assert_eq!(split_compound("a\\\n-o b"), vec!["a-o b"]);
        assert_eq!(split_compound("a \\\\\nb"), vec!["a \\\\", "b"]);
        assert_eq!(split_compound("a \\\\\\\nb"), vec!["a \\\\b"]);
        assert_eq!(statements("a \\\nb"), vec!["a b"]);
        assert_eq!(split_compound("a \\&& b"), vec!["a \\", " b"]);
        let quoted = skeleton("a '\\|' b", SQDQ);
        assert!(!quoted.contains('\\'));
        assert_eq!(split_compound(&quoted).len(), 1);
        assert_eq!(Bash.pipes("a \\| b"), vec!["a \\| b"]);
        assert_eq!(Bash.pipes("a | b"), vec!["a ", " b"]);
        assert_eq!(Bash.pipes("a \\\nb"), vec!["a b"]);
    }

    // spec: guard-kit/SPEC.md §The shell guard — placeholder, never deletion
    #[test]
    fn the_skeleton_substitutes_the_inert_spans_and_leaves_the_rest_byte_identical() {
        assert_eq!(skeleton("echo 'a;b' && ls", SQDQ), format!("echo {SQ_MARK} && ls"));
        assert_eq!(skeleton("echo \"a && b\" | wc -l", SQDQ), format!("echo {DQ_MARK} | wc -l"));
        assert_eq!(skeleton("echo 'a;b' && ls", Wants::default()), "echo 'a;b' && ls");
        assert_eq!(skeleton("grep -oE \"a\\\"b\" file", SQDQ), format!("grep -oE {DQ_MARK} file"));
        assert_eq!(skeleton("echo 'unterminated", SQDQ), "echo 'unterminated");
    }

    // spec: guard-kit/SPEC.md §The reader and its views — an ANSI-C span ends where bash ends it, at the
    // first quote no backslash escapes; a plain or pid-led single-quoted span still ends at its next quote
    #[test]
    fn the_skeleton_ends_an_ansi_c_span_where_bash_does() {
        let s = |c: &str| skeleton(c, SQDQ);
        assert_eq!(s("git status $'\\'' ; git stash ; echo ''"), format!("git status ${SQ_MARK} ; git stash ; echo {SQ_MARK}"));
        assert_eq!(s("ls $'a\\\\' ; rm x"), format!("ls ${SQ_MARK} ; rm x"));
        assert_eq!(s("ls $'a\\\\\\'b' ; rm x"), format!("ls ${SQ_MARK} ; rm x"));
        assert_eq!(s("echo '\\' ; rm x"), format!("echo {SQ_MARK} ; rm x"));
        assert_eq!(s("echo $$'\\' ; rm x"), format!("echo $${SQ_MARK} ; rm x"));
        assert_eq!(s("echo \\$'\\' ; rm x"), format!("echo \\${SQ_MARK} ; rm x"));
        assert_eq!(s("echo $'\\' ; rm x"), "echo $'\\' ; rm x");
        assert_eq!(dequoted("grep $'it\\'s' f").as_deref(), Some("grep $it\\'s f"));
        assert_eq!(faces("grep $'it\\'s' f")[0][1], face("$'it\\'s'", "it's", &format!("${SQ_MARK}")));
        assert_eq!(word_read("echo $'\\' ; rm x"), None);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — a continuation is removed before bash reads
    // the word, so one between a `$` and its quote, or inside a `$` run, opens the span as the joined
    // spelling does
    #[test]
    fn a_continuation_inside_a_dollar_quote_is_joined_first() {
        let s = |c: &str| skeleton(c, SQDQ);
        assert_eq!(s("ls $\\\n'\\'' ; rm x"), format!("ls $\\\n{SQ_MARK} ; rm x"));
        assert_eq!(s("ls $\\\n\\\n'\\'' ; rm x"), format!("ls $\\\n\\\n{SQ_MARK} ; rm x"));
        assert_eq!(s("ls $\\\n$'\\' ; rm x"), format!("ls $\\\n${SQ_MARK} ; rm x"));
        assert_eq!(s("ls $\\\\\n'\\' ; rm x"), format!("ls $\\\\\n{SQ_MARK} ; rm x"));
        assert_eq!(faces("grep $\\\n'it\\'s' f")[0][1], face("$\\\n'it\\'s'", "it's", &format!("${SQ_MARK}")));
        assert_eq!(faces("grep $\\\n$'x' f")[0][1], face("$$'x'", "$$x", &format!("$${SQ_MARK}")));
    }

    // spec: guard-kit/SPEC.md §The reader and its views — command text that spells a placeholder's
    // letters is read as written, and only a real span carries the mark
    #[test]
    fn the_placeholder_letters_in_command_text_are_not_a_placeholder() {
        let s = skeleton("cmd > .tmp/DQ.log 'SQ' HD", HD);
        assert_eq!(s, format!("cmd > .tmp/DQ.log {SQ_MARK} HD"));
        assert_eq!(s.matches('\0').count(), 1);
        assert_eq!(dequoted("echo DQ 'a b'").as_deref(), Some("echo DQ a\x01b"));
        assert_eq!(heredoc_terms(&format!("cat <<<{DQ_MARK}")), vec![DQ_MARK]);
    }

    // spec: guard-kit/SPEC.md §The shell guard — an opener with no line after it has no body
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
        assert_eq!(skeleton("cat <<<\"here string\"", HD), format!("cat <<<{DQ_MARK}"));
    }

    // spec: guard-kit/SPEC.md §The shell guard — the body runs to its terminator line, which
    // stays live; `hd` blanks every body and `hdq` only a quoted-delimiter one
    #[test]
    fn a_heredoc_body_is_blanked_by_its_class_and_the_terminator_stays_live() {
        let c = "python3 - <<'PY'\nprint('a;b' | c)\nPY\nls";
        assert_eq!(skeleton(c, HD), format!("python3 - <<'PY'\n{HD_MARK}\nPY\nls"));
        assert_eq!(skeleton(c, HDQ), format!("python3 - <<'PY'\n{HD_MARK}\nPY\nls"));
        assert_eq!(skeleton(c, SQDQ), "python3 - <<'PY'\nprint('a;b' | c)\nPY\nls");
        assert_eq!(skeleton("cat <<EOF\n$HOME\nEOF", HDQ), "cat <<EOF\n$HOME\nEOF");
        assert_eq!(skeleton("cat <<-EOF\nx\n\tEOF", HD), format!("cat <<-EOF\n{HD_MARK}\n\tEOF"));
        assert_eq!(skeleton("a <<A <<B\n1\nA\n2\nB", HD), format!("a <<A <<B\n{HD_MARK}\nA\n{HD_MARK}\nB"));
        assert_eq!(skeleton("cat <<EOF\nnever ends", HD), format!("cat <<EOF\n{HD_MARK}\n"));
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

    // spec: guard-kit/SPEC.md §The shell guard — the strip walks each wrapper's own grammar,
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
        assert_eq!(redirect_pairs("cmd >& out 2>& 1 >&2x &> o").unwrap(), vec![">& out", "2>& 1", ">&2x", "> o"]);
        assert!(redirect_pairs("wc -l < in.txt").unwrap().is_empty());
        assert_eq!(
            redirect_pairs("jq . < a.json > b.json").unwrap(),
            vec!["> b.json"]
        );
        assert_eq!(redirect_pairs("cmd >&2\rx >\r| y >\x0bz").unwrap(), vec![">&2\rx", ">\r", ">\x0bz"]);
        assert_eq!(redirect_pairs("cmd >\tout").unwrap(), vec![">\tout"]);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the dequoted view
    #[test]
    fn the_dequoted_view_aligns_with_the_skeleton_or_refuses() {
        assert_eq!(dequoted("grep 'a b' f").as_deref(), Some("grep a\x01b f"));
        assert_eq!(dequoted("echo \"x;y\" | wc").as_deref(), Some("echo x\x03y | wc"));
        assert_eq!(dequoted("rm x\\ y").as_deref(), Some("rm x\\\x01y"));
        assert_eq!(dequoted("rm x\\\\ y").as_deref(), Some("rm x\\\\ y"));
        assert_eq!(dequoted("rm x\\\ty").as_deref(), Some("rm x\\\x02y"));
        assert_eq!(dequoted("rm 'x\\ y'").as_deref(), Some("rm x\\\x01y"));
        assert_eq!(dequoted("echo a\\;b").as_deref(), Some("echo a\\;b"));
        assert_eq!(dequoted("echo 'open"), None);
        assert_eq!(dequoted("cat <<EOF\nx\nEOF"), None);
        assert_eq!(dequoted("echo 'a\nb'"), None);
    }

    fn faces(t: &str) -> Vec<Vec<(String, String, String)>> {
        let read = word_read(t).unwrap_or_else(|| panic!("{:?} was not read", t));
        read.into_iter()
            .map(|seg| seg.into_iter().map(|w| (w.spelling, w.value, w.live)).collect())
            .collect()
    }

    fn face(s: &str, v: &str, l: &str) -> (String, String, String) {
        (s.to_string(), v.to_string(), l.to_string())
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the word read's three faces, index-aligned
    // with the compound split, and nothing where the dequoted view refuses
    #[test]
    fn the_word_read_carries_each_word_s_spelling_value_and_live_text() {
        assert_eq!(
            faces("rm 'two words.md' \"a\\\"b\""),
            vec![vec![
                face("rm", "rm", "rm"),
                face("'two words.md'", "two words.md", SQ_MARK),
                face("\"a\\\"b\"", "a\"b", DQ_MARK),
            ]]
        );
        assert_eq!(
            faces("cat a\\ b '.*\\.md' \\*.md"),
            vec![vec![
                face("cat", "cat", "cat"),
                face("a\\ b", "a b", "a\\ b"),
                face("'.*\\.md'", ".*\\.md", SQ_MARK),
                face("\\*.md", "*.md", "\\*.md"),
            ]]
        );
        assert_eq!(
            faces("rm pre'fix'*.md; ls \\; x | wc"),
            vec![
                vec![face("rm", "rm", "rm"), face("pre'fix'*.md", "prefix*.md", &format!("pre{SQ_MARK}*.md"))],
                vec![face("ls", "ls", "ls"), face("\\;", ";", "\\;"), face("x", "x", "x")],
                vec![face("wc", "wc", "wc")],
            ]
        );
        assert_eq!(faces("a \\\nb"), vec![vec![face("a", "a", "a"), face("b", "b", "b")]]);
        assert_eq!(faces("echo \"a\\nb\"")[0][1].1, "a\\nb");
        assert_eq!(word_read("echo 'open"), None);
        assert_eq!(word_read("echo 'a\nb'"), None);
        assert_eq!(word_read("a \\&& b"), None);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — an ANSI-C quoted word is read by its resolved
    // value, with a heredoc beside it too, and a locale-translated one reads nothing; an escaped `$`,
    // the pid `$$` and a quoted `$` stay read as written
    #[test]
    fn the_word_read_answers_nothing_on_a_dollar_quoted_word() {
        let target = format!("{DQ_MARK}${SQ_MARK}");
        let resolved = face("\".tmp/\"$'\\056\\056/README.md'", ".tmp/../README.md", &target);
        assert_eq!(faces("cat >> \".tmp/\"$'\\056\\056/README.md'")[0][2], resolved);
        assert_eq!(faces("cat >> \".tmp/\"$'\\056\\056/README.md' <<'EOF'\nx\nEOF")[0][2], resolved);
        assert_eq!(faces("echo $$$'x'")[0][1].1, "$$x");
        assert_eq!(faces("echo \\$$'x'")[0][1].1, "$x");
        assert_eq!(faces("echo \\\\$'x'")[0][1].1, "\\x");
        assert_eq!(faces("echo \\$'x'")[0][1].1, "$x");
        assert_eq!(faces("echo $$'x'")[0][1].1, "$$x");
        assert_eq!(faces("echo \"$\"'x'")[0][1].1, "$x");
        assert_eq!(word_read("cat >> \".tmp/\"$\"../README.md\""), None);
        assert_eq!(word_read("cat >> \".tmp/\"$\"../README.md\" <<'EOF'\nx\nEOF"), None);
        assert_eq!(word_read("echo $$$\"x\""), None);
        assert_eq!(word_read("echo \\$$\"x\""), None);
        assert_eq!(word_read("echo \\\\$\"x\""), None);
        assert_eq!(faces("echo \\$\"x\"")[0][1].1, "$x");
        assert_eq!(faces("echo $$\"x\"")[0][1].1, "$$x");
        assert_eq!(faces("echo '$'\"x\"")[0][1].1, "$x");
        assert_eq!(faces("echo \"a$\"\"x\"")[0][1].1, "a$x");
    }

    // spec: guard-kit/SPEC.md §The reader and its views — a word carrying an ANSI-C quoted piece is
    // marked as one, and the pid, an escaped `$` and a plain quote are not
    #[test]
    fn the_word_read_marks_a_word_carrying_an_ansi_c_piece() {
        let marks = |c: &str| -> Vec<bool> { word_read(c).unwrap()[0].iter().map(Word::ansi_c).collect() };
        assert_eq!(marks("awk $'BEGIN { \\x73ystem(\"x\") }' f"), [false, true, false]);
        assert_eq!(marks("cat >> \".tmp/\"$'\\056\\056/x'"), [false, false, true]);
        assert_eq!(marks("echo $$$'x' a"), [false, true, false]);
        assert_eq!(marks("echo $$'x' \\$'x' 'x' \"x\""), [false, false, false, false, false]);
        let w = word_read("echo a$'b'c").unwrap()[0][1].clone();
        assert!(w.slice(0, 1).is_some_and(|s| s.ansi_c()));
    }

    // spec: guard-kit/SPEC.md §The reader and its views — each escape resolved to the bytes bash gives
    // it, or refused
    #[test]
    fn an_ansi_c_escape_resolves_as_bash_resolves_it_or_refuses() {
        let read = |c: &str| ansi_c(c.as_bytes()).map(|v| String::from_utf8(v).unwrap());
        let reads = |c: &str, v: &str| assert_eq!(read(c).as_deref(), Some(v), "{:?}", c);
        let refuses = |c: &str| assert_eq!(read(c), None, "{:?}", c);
        reads("\\n", "\n");
        reads("\\t", "\t");
        reads("\\r", "\r");
        for c in ["\\a", "\\b", "\\e", "\\E", "\\f", "\\v"] {
            refuses(c);
        }
        reads("\\\\", "\\");
        reads("\\'", "'");
        reads("\\\"", "\"");
        reads("\\?", "?");
        reads("\\101", "A");
        reads("\\56x", ".x");
        refuses("\\7");
        refuses("\\0101");
        refuses("\\777");
        reads("\\x41", "A");
        refuses("\\x4");
        reads("\\x414", "A4");
        reads("\\xg", "\\xg");
        refuses("\\x80");
        reads("\\u41", "A");
        reads("\\u0041", "A");
        refuses("\\U000000e9");
        reads("\\u41g", "Ag");
        refuses("\\cA");
        refuses("\\ca");
        reads("\\cI", "\t");
        reads("\\ci", "\t");
        reads("\\c?", "\u{7f}");
        refuses("\\c\\\\x");
        reads("\\q", "\\q");
        reads("\\8", "\\8");
        reads("a\\c", "a\\c");
        reads("\\x", "\\x");
        reads("\\u", "\\u");
        reads("\\U", "\\U");
        refuses("\\0");
        refuses("\\x00");
        refuses("\\c@");
        refuses("a\\");
        reads("a\\\\", "a\\");
    }

    // spec: guard-kit/SPEC.md §The reader and its views — a heredoc read as the skeleton holds it: the
    // opener and the terminator bare, a non-empty body one mark word alone in its segment
    #[test]
    fn the_word_read_reads_a_heredoc_as_the_skeleton_holds_it() {
        let hd = |s: &str| face(s, HD_MARK, HD_MARK);
        let plain = |w: &str| face(w, w, w);
        assert_eq!(faces("cat <<EOF\nx\nEOF"), vec![vec![plain("cat"), plain("<<EOF")], vec![hd("x")], vec![plain("EOF")]]);
        assert_eq!(
            faces("cat >> \".tmp/j.md\" << \"EOF\"\na 'b'\nc\nEOF"),
            vec![
                vec![plain("cat"), plain(">>"), face("\".tmp/j.md\"", ".tmp/j.md", DQ_MARK), plain("<<"), plain("\"EOF\"")],
                vec![hd("a 'b'\nc")],
                vec![plain("EOF")],
            ]
        );
        assert_eq!(faces("cat <<'E\"F'\nx\nE\"F")[2], vec![plain("E\"F")], "a quoted delimiter keeps its quotes");
        assert_eq!(faces("cat <<'EOF'\nEOF\nls"), vec![vec![plain("cat"), plain("<<'EOF'")], vec![plain("EOF")], vec![plain("ls")]], "an empty body adds no segment");
        assert_eq!(
            faces("a <<A <<-B\n1\nA\n2\n\tB"),
            vec![vec![plain("a"), plain("<<A"), plain("<<-B")], vec![hd("1")], vec![plain("A")], vec![hd("2")], vec![plain("B")]],
            "two heredocs on one line"
        );
        assert_eq!(faces("cat <<EOF\nnever\nends"), vec![vec![plain("cat"), plain("<<EOF")], vec![hd("never\nends")]], "a body with no terminator");
        assert_eq!(word_read("cat <<EOF; echo 'a\nb'\nx\nEOF"), None, "a quoted span carrying a newline beside a heredoc");
        assert_eq!(faces("cat <<< \"x y\"")[0][2], face("\"x y\"", "x y", DQ_MARK), "a herestring is no heredoc");
    }

    // spec: guard-kit/SPEC.md §The reader and its views — a heredoc adds only its own words, over every
    // bash decision-table row one can be appended to
    #[test]
    fn a_heredoc_adds_only_its_own_words_to_every_row_it_can_follow() {
        let table = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../guard-kit/guard-tests/cases.tsv");
        let body = std::fs::read_to_string(table).unwrap();
        let heredocs: [(&str, &[&str], &str); 3] =
            [("<<'EOF'", &["x"], "x"), ("<<EOF", &["a \"b\" \\c"], "a \"b\" \\c"), ("<< \"EOF\"", &[], "")];
        let mut seen = 0usize;
        for line in body.lines() {
            let Some((verdict, cell)) = line.split_once('\t') else { continue };
            if verdict.trim().is_empty() || verdict.starts_with('#') {
                continue;
            }
            if cell.contains(['#']) || ["<<", "@NL@", "$\""].iter().any(|t| cell.contains(t)) || cell.ends_with(['&', '|', ';', '\\']) {
                continue;
            }
            let row = cell.replace("@ROOT@", "/tmp/sandbox").replace("@CR@", "\r");
            let Some(base) = word_read(&row) else {
                assert!(row.contains("$'") || row.contains("\\&&"), "{:?} was not read", row);
                continue;
            };
            for (opener, lines, spelling) in heredocs {
                let mut call = format!("{} {}\n", row, opener);
                for l in lines {
                    call.push_str(l);
                    call.push('\n');
                }
                call.push_str("EOF");
                let mut want = base.clone();
                let last = want.last_mut().expect("a read has a segment");
                last.extend(opener.split(' ').map(|w| Word::new(w, w, w)));
                if !spelling.is_empty() {
                    want.push(vec![Word::new(spelling, HD_MARK, HD_MARK)]);
                }
                want.push(vec![Word::new("EOF", "EOF", "EOF")]);
                let got = word_read(&call).unwrap_or_else(|| panic!("{:?} was not read", call));
                let faces = |r: &[Vec<Word>]| r.iter().map(|s| s.iter().map(|w| (w.spelling.clone(), w.value.clone(), w.live.clone())).collect::<Vec<_>>()).collect::<Vec<_>>();
                assert_eq!(faces(&got), faces(&want), "{:?}", call);
                seen += 1;
            }
        }
        assert!(seen > 0);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — an ANSI-C span with no escape reads as the
    // single-quoted span it re-spells, over every decision-table row carrying one
    #[test]
    fn an_ansi_c_span_reads_as_its_single_quoted_spelling() {
        let table = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../guard-kit/guard-tests/cases.tsv");
        let body = std::fs::read_to_string(table).unwrap();
        let values = |r: Vec<Vec<Word>>| r.into_iter().map(|s| s.into_iter().map(|w| w.value).collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut seen = 0usize;
        for line in body.lines() {
            let Some((verdict, cell)) = line.split_once('\t') else { continue };
            if verdict.trim().is_empty() || verdict.starts_with('#') || !cell.contains('\'') || cell.matches('\'').count() % 2 == 1 {
                continue;
            }
            if cell.contains(['\\', '"', '$', '#']) || ["<<", "@NL@"].iter().any(|t| cell.contains(t)) {
                continue;
            }
            let row = cell.replace("@ROOT@", "/tmp/sandbox").replace("@CR@", "\r");
            let mut spelled = String::new();
            for (k, part) in row.split('\'').enumerate() {
                if k > 0 {
                    spelled.push_str(if k % 2 == 1 { "$'" } else { "'" });
                }
                spelled.push_str(part);
            }
            let base = word_read(&row).unwrap_or_else(|| panic!("{:?} was not read", row));
            let got = word_read(&spelled).unwrap_or_else(|| panic!("{:?} was not read", spelled));
            assert_eq!(values(got), values(base), "{:?}", spelled);
            seen += 1;
        }
        assert!(seen > 0);
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the terminators, including the one a herestring's
    // second `<` opens, and each statement carrying only its own residue
    #[test]
    fn heredoc_residue_follows_the_one_statement_that_opened_it() {
        assert_eq!(heredoc_terms("cat <<'EOF' >> f"), vec!["EOF"]);
        assert_eq!(heredoc_terms("cat <<<\"x\""), vec!["x"]);
        assert_eq!(
            residue_statements(&format!("cat <<'EOF' >> f; ls\n{HD_MARK}\nEOF")),
            vec![format!("cat <<'EOF' >> f\n{HD_MARK}\nEOF"), " ls".to_string()]
        );
        assert_eq!(residue_statements("a && b\n\nc"), vec!["a ", " b", "c"]);
    }
}
