// spec: guard-kit/SPEC.md §The guard framework — the crate's holder of the three `lib/guard.sh`
// primitives §scan-prompts is composed from, admitted by criterion 6's *unless* clause and held
// equal by `--guard-lib-parity` (gate-sdk/SPEC.md §The port-candidate criteria).
use crate::ere::{Ere, EreError};

// spec: guard-kit/SPEC.md §The guard framework — the kit-relative shell library, with two readers:
// the holder these twins are compared against, and the file whose presence *is* guard-kit being
// vendored, which is how `kpi-prompt-friction` witnesses the kit (drift-kit/SPEC.md §Bundled KPIs).
pub const LIB: &str = "lib/guard.sh";

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
