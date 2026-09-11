// spec: guard-kit/SPEC.md §The guard framework — the crate's holder of the twinned `lib/guard.sh`
// primitives the compiled members are composed from, admitted by criterion 6's *unless* clause and
// held equal by `--guard-lib-parity` (gate-sdk/SPEC.md §The port-candidate criteria).
use crate::ere::{Ere, EreError};

// spec: guard-kit/SPEC.md §The guard framework — the kit-relative shell library, with two readers:
// the holder these twins are compared against, and the file whose presence *is* guard-kit being
// vendored, which is how `kpi-prompt-friction` witnesses the kit (drift-kit/SPEC.md §Bundled KPIs).
pub const LIB: &str = "lib/guard.sh";

// spec: guard-kit/SPEC.md §The guard framework — `guard_allow_match`: `[[ "$s" == ${glob//:\*/\*} ]]`,
// the settings-allow match core. One compiled holder, because two compiled members now depend on
// it and an edit to one inline copy would change one member's verdicts while the other's stay right.
pub fn allow_match(s: &str, glob: &str) -> bool {
    crate::walk::glob_match(&glob.replace(":*", "*"), s)
}

// spec: guard-kit/SPEC.md §The guard framework — `guard_split_compound`: one segment per line,
// split on the harness's statement separators. `||` and `&&` are tested before `|`, which is the
// leftmost-longest alternation the shell holder's `sed -E` gives for free and a scanner must spell.
pub fn split_compound(cmd: &str) -> Vec<String> {
    let b = cmd.as_bytes();
    let mut segs: Vec<Vec<u8>> = vec![Vec::new()];
    let mut i = 0usize;
    while i < b.len() {
        let sep = if b[i..].starts_with(b"||") || b[i..].starts_with(b"&&") {
            2
        } else if b[i] == b';' || b[i] == b'|' {
            1
        } else {
            0
        };
        if sep > 0 {
            segs.push(Vec::new());
            i += sep;
        } else {
            segs.last_mut().expect("one segment is always open").push(b[i]);
            i += 1;
        }
    }
    segs.iter()
        .map(|s| String::from_utf8_lossy(s).into_owned())
        .collect()
}

// spec: guard-kit/SPEC.md §The guard framework — the two inert classes reachable on newline-free
// input. `hd` and `hdq` are read only inside the heredoc-body machinery this twin omits, so they
// are not fields here: a field with no reader is the defect a dropped branch would be.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Wants {
    pub sq: bool,
    pub dq: bool,
}

// spec: guard-kit/SPEC.md §The guard framework — the twin's precondition, carried in the type
// rather than only in prose: a newline-bearing command is out of contract, refused rather than
// silently normalized by a machine that omits the branch which would have handled it.
#[derive(Debug, PartialEq, Eq)]
pub struct NewlineInInput;

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

fn trim_start(s: &str) -> &str {
    let i = s.bytes().position(|c| !is_space(c)).unwrap_or(s.len());
    &s[i..]
}

// spec: guard-kit/SPEC.md §The guard framework — the harness view's word cursor: `_guard_hv_pop`'s
// head word and the blanks after it, over a slice that never carries leading blanks.
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

// spec: guard-kit/SPEC.md §The guard framework — `_guard_harness_view`: a segment as the permission
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

// spec: guard-kit/SPEC.md §The guard framework — `<<-?[[:space:]]*(<quoted>|<identifier>)`, the
// opener the holder matches with an anchored ERE. One byte decides the alternative and `[[:space:]]`
// shares none of their first-character sets, so the greedy run needs no backtracking.
fn heredoc_header_len(s: &[u8]) -> Option<usize> {
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
            Some(j + 1 + k + 1)
        }
        c if c.is_ascii_alphabetic() || c == b'_' => {
            let mut e = j + 1;
            while e < s.len() && (s[e].is_ascii_alphanumeric() || s[e] == b'_') {
                e += 1;
            }
            Some(e)
        }
        _ => None,
    }
}

enum State {
    None,
    Sq,
    Dq,
}

// spec: guard-kit/SPEC.md §The guard framework — `guard_skeleton` over the reachable subset: a
// friction-log line is newline-free by construction, so the holder's `$'\n'` arm cannot fire, this
// twin omits the machinery behind it, and the entry point refuses the input that would need it.
pub fn skeleton(cmd: &str, w: Wants) -> Result<String, NewlineInInput> {
    if cmd.as_bytes().contains(&b'\n') {
        return Err(NewlineInInput);
    }
    let b = cmd.as_bytes();
    let n = b.len();
    let mut out: Vec<u8> = Vec::with_capacity(n);
    let mut span: Vec<u8> = Vec::new();
    let mut state = State::None;
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

        // spec: guard-kit/SPEC.md §The guard framework — the holder jumps between significant
        // characters rather than stepping per character, and the set it jumps to is the one the
        // arms below decide on. Newline is absent from it here because the entry point refused it.
        let k = b[i..]
            .iter()
            .position(|&c| c == b'"' || c == b'\'' || c == b'\\' || c == b'<')
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
            if let Some(len) = heredoc_header_len(&b[i..]) {
                out.extend_from_slice(&b[i..i + len]);
                i += len;
                continue;
            }
        }
        // spec: guard-kit/SPEC.md §The guard framework — placeholder, never deletion: a construct
        // that survives the scan is live, so an unrecognized `<` is one byte of the command again.
        out.push(ch);
        i += 1;
    }
    if !span.is_empty() {
        out.extend_from_slice(&span);
    }
    Ok(String::from_utf8_lossy(&out).into_owned())
}

// spec: guard-kit/SPEC.md §The generic ruleset — `_guard_redirect_pairs`' pattern, cited rather
// than re-expressed: the holder hands it to `grep -oE` and this one to the crate's own matcher
// (gate-sdk/SPEC.md §The POSIX ERE matcher), so neither side re-spells it.
pub const REDIRECT_RE_SRC: &str = "[0-9]*>>?[[:space:]]*(&[0-9-]+|[^[:space:]|;&<>]+)";

// spec: guard-kit/SPEC.md §The generic ruleset — one match per line, leftmost-longest and
// non-overlapping, which is `grep -o`'s contract. Line-wise rather than whole-string, because
// `[[:space:]]` matches a newline and a whole-string scan would join two lines into one pair.
pub fn redirect_pairs(text: &str) -> Result<Vec<String>, EreError> {
    let re = Ere::compile(REDIRECT_RE_SRC)?;
    let mut out = Vec::new();
    for line in text.split('\n') {
        let mut pos = 0usize;
        while let Some(rest) = line.get(pos..) {
            if rest.is_empty() {
                break;
            }
            match re.find(rest) {
                None => break,
                Some((s, e)) if e > s => {
                    out.push(rest[s..e].to_string());
                    pos += e;
                }
                Some(_) => break,
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SQDQ: Wants = Wants { sq: true, dq: true };

    // spec: guard-kit/SPEC.md §The guard framework — the splitter's separator class and the
    // longest-match rule: `||` is one boundary, not two, and a trailing separator opens a segment
    #[test]
    fn the_splitter_takes_the_longest_separator_and_keeps_the_empty_tail() {
        assert_eq!(split_compound("a;b&&c||d|e"), vec!["a", "b", "c", "d", "e"]);
        assert_eq!(split_compound("echo trailing;"), vec!["echo trailing", ""]);
        assert_eq!(split_compound(""), vec![""]);
        assert_eq!(split_compound("a & b"), vec!["a & b"]);
    }

    // spec: guard-kit/SPEC.md §The guard framework — placeholder, never deletion, and the two
    // classes reachable on newline-free input
    #[test]
    fn the_skeleton_substitutes_the_inert_spans_and_leaves_the_rest_byte_identical() {
        assert_eq!(skeleton("echo 'a;b' && ls", SQDQ).unwrap(), "echo SQ && ls");
        assert_eq!(skeleton("echo \"a && b\" | wc -l", SQDQ).unwrap(), "echo DQ | wc -l");
        assert_eq!(
            skeleton("echo 'a;b' && ls", Wants::default()).unwrap(),
            "echo 'a;b' && ls"
        );
        assert_eq!(
            skeleton("grep -oE \"a\\\"b\" file", SQDQ).unwrap(),
            "grep -oE DQ file"
        );
        assert_eq!(skeleton("echo 'unterminated", SQDQ).unwrap(), "echo 'unterminated");
    }

    // spec: guard-kit/SPEC.md §The guard framework — the heredoc opener is emitted verbatim and
    // nothing follows it, which is what the omitted body machinery would otherwise have handled
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
            assert_eq!(skeleton(c, SQDQ).unwrap(), c, "opener {:?} did not survive", c);
        }
        assert_eq!(skeleton("cat <<<\"here string\"", SQDQ).unwrap(), "cat <<<DQ");
    }

    // spec: guard-kit/SPEC.md §The guard framework — the precondition is checked, not assumed: the
    // omitted branch's input is refused rather than silently mis-normalized
    #[test]
    fn a_newline_bearing_command_is_out_of_contract_rather_than_normalized() {
        assert_eq!(skeleton("cat <<EOF\nbody\nEOF", SQDQ), Err(NewlineInInput));
        assert!(skeleton("cat <<EOF", SQDQ).is_ok());
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
}
