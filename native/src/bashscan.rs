// spec: gate-sdk/SPEC.md §port-blockers — the command-position scanner both registry arms are
// built on: a character state machine over quoting, command-substitution frames, heredocs,
// here-strings, double-bracket state, `case` levels, arithmetic and array-literal skipping.

// spec: gate-sdk/SPEC.md §The port-candidate criteria — command position is what excludes the
// attested false positives as a class: an array-literal element, a word inside a string and an
// awk-internal token are all non-command-position, so no per-case exception list is kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Cmd,
    Expansion,
    Guard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    pub word: String,
    pub line: usize,
}

// spec: gate-sdk/SPEC.md §port-blockers — `command -v <prog>` is this tree's convention for
// announcing exactly this dependency, so a guarded program is a requirement with no inference.
const PROBE_FLAGS: [&str; 4] = ["-v", "-V", "-p", "-P"];

// spec: gate-sdk/SPEC.md §port-blockers — a word reaching command position in one of these keeps
// the next word in command position too; the shapes below it consume their own operand instead.
const RESUMING_KEYWORDS: [&str; 13] = [
    "if", "then", "else", "elif", "do", "while", "until", "!", "time", "fi", "done", "{", "}",
];

const OPERAND_KEYWORDS: [&str; 5] = ["for", "select", "in", "function", "["];

#[derive(Default)]
struct Frame {
    dq: bool,
    word: Vec<u8>,
    inword: bool,
    wcmd: bool,
    wline: usize,
}

#[derive(Default)]
struct Scan {
    out: Vec<Token>,
    cmdpos: bool,
    sq: bool,
    dq: bool,
    dbrack: bool,
    probe: u8,
    heredoc: Option<Vec<u8>>,
    hdstrip: bool,
    inword: bool,
    word: Vec<u8>,
    wcmd: bool,
    wline: usize,
    stack: Vec<Frame>,
    case_state: Vec<u8>,
    line: usize,
}

impl Scan {
    fn emit(&mut self, kind: Kind, word: &str) {
        self.out.push(Token {
            kind,
            word: word.to_string(),
            line: self.wline,
        });
    }

    fn addc(&mut self, bytes: &[u8]) {
        if !self.inword {
            self.inword = true;
            self.word.clear();
            self.wcmd = self.cmdpos;
            self.wline = self.line;
        }
        self.word.extend_from_slice(bytes);
    }

    // spec: gate-sdk/SPEC.md §port-blockers — inside `[[ … ]]` the scan tracks state and emits
    // nothing: a conditional's words are operands, and `]]` is what restores command position.
    fn endword(&mut self) {
        if !self.inword {
            return;
        }
        self.inword = false;
        let w = String::from_utf8_lossy(&std::mem::take(&mut self.word)).into_owned();
        if w == "]]" {
            self.dbrack = false;
            self.cmdpos = false;
            return;
        }
        if self.dbrack {
            return;
        }
        if w == "esac" {
            self.pop_case();
            self.cmdpos = true;
            return;
        }
        if self.wcmd {
            self.probe = 0;
            self.classify(&w);
        } else if w == "in" && self.case_state.last() == Some(&1) {
            *self.case_state.last_mut().expect("case level checked above") = 2;
            self.cmdpos = false;
        } else if self.probe > 0 {
            self.probeword(&w);
        }
    }

    fn probeword(&mut self, t: &str) {
        if PROBE_FLAGS.contains(&t) {
            self.probe = 2;
            return;
        }
        if self.probe == 2 {
            if is_program_word(t) {
                self.emit(Kind::Guard, t);
            }
            self.probe = 0;
        }
    }

    fn pop_case(&mut self) {
        self.case_state.pop();
    }

    // spec: gate-sdk/SPEC.md §port-blockers — knob resolution for a command-position expansion is
    // the third derivation input: the name is reported and the caller resolves it through the one
    // bridge resolver, so this report cannot disagree with a dispatched binary's value.
    fn classify(&mut self, t: &str) {
        if is_assignment(t) {
            self.cmdpos = true;
            return;
        }
        if t == "esac" {
            self.pop_case();
            self.cmdpos = true;
            return;
        }
        if t == "case" {
            self.case_state.push(1);
            self.cmdpos = false;
            return;
        }
        if t == "in" && self.case_state.last() == Some(&1) {
            *self.case_state.last_mut().expect("case level checked above") = 2;
            self.cmdpos = false;
            return;
        }
        if RESUMING_KEYWORDS.contains(&t) {
            self.cmdpos = true;
            return;
        }
        if t == "[[" {
            self.dbrack = true;
            self.cmdpos = false;
            return;
        }
        if OPERAND_KEYWORDS.contains(&t) {
            self.cmdpos = false;
            return;
        }
        if let Some(knob) = expansion_name(t) {
            self.emit(Kind::Expansion, &knob);
            self.cmdpos = false;
            return;
        }
        if t == "command" {
            self.cmdpos = false;
            self.probe = 1;
            return;
        }
        if is_program_word(t) {
            self.emit(Kind::Cmd, t);
        }
        self.cmdpos = false;
    }

    fn push(&mut self) {
        self.stack.push(Frame {
            dq: self.dq,
            word: std::mem::take(&mut self.word),
            inword: self.inword,
            wcmd: self.wcmd,
            wline: self.wline,
        });
        self.dq = false;
        self.inword = false;
    }

    fn pop(&mut self) {
        match self.stack.pop() {
            Some(f) => {
                self.dq = f.dq;
                self.word = f.word;
                self.inword = f.inword;
                self.wcmd = f.wcmd;
                self.wline = f.wline;
            }
            None => self.dq = false,
        }
    }

    fn in_case(&self) -> bool {
        self.dbrack || self.case_state.last() == Some(&2)
    }
}

fn at(b: &[u8], i: usize) -> u8 {
    if i < b.len() {
        b[i]
    } else {
        0
    }
}

// spec: gate-sdk/SPEC.md §port-blockers — an arithmetic or array-literal body is skipped whole
// rather than scanned, on the balance of its own delimiter; an unbalanced one runs to end of line.
fn skip_balanced(b: &[u8], mut k: usize, open: u8, close: u8, quotes: bool) -> usize {
    let mut depth = 1usize;
    while k < b.len() {
        let c = b[k];
        if quotes && (c == b'\'' || c == b'"') {
            k += 1;
            while k < b.len() && b[k] != c {
                k += 1;
            }
        } else if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return k + 1;
            }
        }
        k += 1;
    }
    k
}

fn is_assignment(t: &str) -> bool {
    let b = t.as_bytes();
    if b.is_empty() || !(b[0].is_ascii_alphabetic() || b[0] == b'_') {
        return false;
    }
    let mut i = 1usize;
    while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
        i += 1;
    }
    if at(b, i) == b'[' {
        match b[i..].iter().position(|c| *c == b']') {
            Some(j) => i += j + 1,
            None => return false,
        }
    }
    if at(b, i) == b'+' {
        i += 1;
    }
    at(b, i) == b'='
}

// spec: gate-sdk/SPEC.md §port-blockers — a program word is the shape the report names: a shell
// name, never a path, an operator or a substituted fragment.
fn is_program_word(t: &str) -> bool {
    let b = t.as_bytes();
    !b.is_empty()
        && (b[0].is_ascii_alphabetic() || b[0] == b'_')
        && b[1..]
            .iter()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'.' | b'+' | b'-'))
}

// spec: gate-sdk/SPEC.md §port-blockers — a `"${KNOB[@]}"` or `"$KNOB"` at the head of a command is
// the knob whose resolved value's command word is the requirement; the name is what is reported.
fn expansion_name(t: &str) -> Option<String> {
    let b = t.as_bytes();
    let mut i = 0usize;
    let quoted = at(b, i) == b'"';
    if quoted {
        i += 1;
    }
    if at(b, i) != b'$' {
        return None;
    }
    i += 1;
    let braced = at(b, i) == b'{';
    if braced {
        i += 1;
    }
    if !(at(b, i).is_ascii_alphabetic() || at(b, i) == b'_') {
        return None;
    }
    let start = i;
    i += 1;
    while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
        i += 1;
    }
    let name = String::from_utf8_lossy(&b[start..i]).into_owned();
    if at(b, i) == b'[' && matches!(at(b, i + 1), b'@' | b'*') && at(b, i + 2) == b']' {
        i += 3;
    }
    if braced {
        if at(b, i) != b'}' {
            return None;
        }
        i += 1;
    }
    if quoted {
        if at(b, i) != b'"' {
            return None;
        }
        i += 1;
    }
    if i == b.len() {
        Some(name)
    } else {
        None
    }
}

// spec: gate-sdk/SPEC.md §port-blockers — the standing blind spot carried forward, not discharged:
// the scan reads a member's own declaration text and does not follow a call into a kit library, so
// a requirement reached through a shared helper stays invisible (port-blockers-library-mediated-scan).
pub fn command_positions(text: &str) -> Vec<Token> {
    let mut s = Scan {
        cmdpos: true,
        ..Scan::default()
    };
    let mut cont = false;
    for (idx, raw) in text.lines().enumerate() {
        s.line = idx + 1;
        let b = raw.as_bytes();
        let n = b.len();

        // spec: gate-sdk/SPEC.md §port-blockers — a heredoc body is skipped to its delimiter, the
        // `<<-` form matching a delimiter after leading blanks; the body is data, never source.
        if let Some(delim) = s.heredoc.clone() {
            let t = if s.hdstrip {
                raw.trim_start_matches([' ', '\t'])
            } else {
                raw
            };
            if t.as_bytes() == delim.as_slice() {
                s.heredoc = None;
                s.cmdpos = true;
            }
            continue;
        }

        let mut pend: Option<Vec<u8>> = None;
        let mut i = 0usize;
        while i < n {
            let c = b[i];

            if s.sq {
                s.addc(&b[i..i + 1]);
                if c == b'\'' {
                    s.sq = false;
                }
                i += 1;
                continue;
            }

            if s.dq {
                if c == b'\\' {
                    let end = (i + 2).min(n);
                    s.addc(&b[i..end]);
                    i += 2;
                    continue;
                }
                if c == b'$' && at(b, i + 1) == b'(' && at(b, i + 2) == b'(' {
                    i = skip_balanced(b, i + 3, b'(', b')', false);
                    continue;
                }
                if c == b'$' && at(b, i + 1) == b'{' {
                    let j = skip_balanced(b, i + 2, b'{', b'}', false);
                    s.addc(&b[i..j]);
                    i = j;
                    continue;
                }
                if c == b'$' && at(b, i + 1) == b'(' {
                    s.addc(b"$(");
                    s.push();
                    s.cmdpos = true;
                    i += 2;
                    continue;
                }
                if c == b'`' {
                    s.addc(b"`");
                    s.push();
                    s.cmdpos = true;
                    i += 1;
                    continue;
                }
                s.addc(&b[i..i + 1]);
                if c == b'"' {
                    s.dq = false;
                }
                i += 1;
                continue;
            }

            if c == b' ' || c == b'\t' {
                s.endword();
                i += 1;
                continue;
            }
            if c == b'\\' {
                if i + 1 == n {
                    cont = true;
                    i += 1;
                    continue;
                }
                s.addc(&b[i..i + 2]);
                i += 2;
                continue;
            }
            if c == b'#' && !s.inword {
                break;
            }
            if c == b'\'' {
                s.addc(b"'");
                s.sq = true;
                i += 1;
                continue;
            }
            if c == b'"' {
                s.addc(b"\"");
                s.dq = true;
                i += 1;
                continue;
            }

            // spec: gate-sdk/SPEC.md §port-blockers — a here-string is consumed whole as a single
            // redirection operator, **ahead of** the heredoc branch: an operator consumed whole
            // cannot be re-entered part-way, which is what makes that repair total.
            if c == b'<' && at(b, i + 1) == b'<' && at(b, i + 2) == b'<' {
                s.endword();
                i += 3;
                s.cmdpos = false;
                continue;
            }

            if c == b'<' && at(b, i + 1) == b'<' && at(b, i + 2) != b'<' {
                s.endword();
                let mut j = i + 2;
                s.hdstrip = at(b, j) == b'-';
                if s.hdstrip {
                    j += 1;
                }
                while at(b, j) == b' ' {
                    j += 1;
                }
                let q = at(b, j);
                let mut delim: Vec<u8> = Vec::new();
                if q == b'\'' || q == b'"' {
                    j += 1;
                    while j < n && b[j] != q {
                        delim.push(b[j]);
                        j += 1;
                    }
                    j += 1;
                } else {
                    while j < n && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
                        delim.push(b[j]);
                        j += 1;
                    }
                }
                pend = Some(delim);
                i = j;
                s.cmdpos = false;
                continue;
            }

            if c == b'$' && at(b, i + 1) == b'(' && at(b, i + 2) == b'(' {
                i = skip_balanced(b, i + 3, b'(', b')', false);
                continue;
            }
            if c == b'$' && at(b, i + 1) == b'{' {
                let j = skip_balanced(b, i + 2, b'{', b'}', false);
                s.addc(&b[i..j]);
                i = j;
                continue;
            }
            if c == b'$' && at(b, i + 1) == b'(' {
                s.addc(b"$(");
                s.push();
                s.cmdpos = true;
                i += 2;
                continue;
            }
            if c == b'<' && at(b, i + 1) == b'(' {
                s.endword();
                s.push();
                s.cmdpos = true;
                i += 2;
                continue;
            }
            if c == b'`' {
                s.addc(b"`");
                s.push();
                s.cmdpos = true;
                i += 1;
                continue;
            }
            if c == b'(' && at(b, i + 1) == b'(' && !s.inword {
                i = skip_balanced(b, i + 2, b'(', b')', false);
                s.cmdpos = false;
                continue;
            }
            if c == b'(' && s.inword && s.word.last() == Some(&b'=') {
                s.endword();
                i = skip_balanced(b, i + 1, b'(', b')', true);
                s.cmdpos = true;
                continue;
            }

            if c == b';' && at(b, i + 1) == b';' {
                s.endword();
                if s.case_state.last() == Some(&3) {
                    *s.case_state.last_mut().expect("case level checked above") = 2;
                }
                s.cmdpos = !s.in_case();
                i += 2;
                continue;
            }
            if c == b';' || c == b'&' || c == b'|' {
                s.endword();
                s.cmdpos = !s.in_case();
                i += 1;
                continue;
            }
            if c == b'{' || c == b'(' {
                s.endword();
                if c == b'(' {
                    s.push();
                }
                s.cmdpos = true;
                i += 1;
                continue;
            }
            // spec: gate-sdk/SPEC.md §port-blockers — **inside `[[ … ]]` only**, a `)` pops a
            // pushed substitution frame ahead of any case-pattern reading: the unscoped form was
            // tried and caught stealing a genuine `case` pattern's frame.
            if c == b')' {
                s.endword();
                if s.dbrack && !s.stack.is_empty() {
                    s.pop();
                    s.cmdpos = false;
                } else if s.in_case() {
                    if let Some(st) = s.case_state.last_mut() {
                        *st = 3;
                    }
                    s.cmdpos = true;
                } else {
                    s.pop();
                    s.cmdpos = false;
                }
                i += 1;
                continue;
            }
            if c == b'}' {
                s.endword();
                s.cmdpos = true;
                i += 1;
                continue;
            }
            if c == b'>' || c == b'<' {
                s.endword();
                s.cmdpos = false;
                i += 1;
                continue;
            }

            s.addc(&b[i..i + 1]);
            i += 1;
        }
        s.endword();
        if let Some(d) = pend {
            s.heredoc = Some(d);
        }
        if !s.sq && !s.dq && !cont {
            s.cmdpos = !s.in_case();
        }
        cont = false;
    }
    s.out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(text: &str, kind: Kind) -> Vec<String> {
        command_positions(text)
            .into_iter()
            .filter(|t| t.kind == kind)
            .map(|t| t.word)
            .collect()
    }

    // spec: gate-sdk/SPEC.md §The port-candidate criteria — command position excludes the attested
    // false positives as a class: an array-literal element, a word inside a string, an awk-internal
    // token. One test over all three, because one rule removes all three.
    #[test]
    fn only_a_word_at_the_head_of_a_simple_command_is_a_command() {
        assert_eq!(words("jq --raw-output .\n", Kind::Cmd), vec!["jq"]);
        assert_eq!(words("ARR=(jq shellcheck ruby)\n", Kind::Cmd), Vec::<String>::new());
        assert_eq!(words("echo \"run jq here\"\n", Kind::Cmd), vec!["echo"]);
        assert_eq!(
            words("awk '{ print length($0) }' file\n", Kind::Cmd),
            vec!["awk"]
        );
        assert_eq!(
            words("printf '%s' \"$x\" | jq .\n", Kind::Cmd),
            vec!["printf", "jq"]
        );
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the `command -v` guard is a requirement with no
    // inference, and `declare -F`'s mirror is the caller's filter rather than the scan's.
    #[test]
    fn a_command_v_guard_reports_its_program_however_it_is_spelled() {
        assert_eq!(words("command -v jq >/dev/null\n", Kind::Guard), vec!["jq"]);
        assert_eq!(words("command -V shellcheck\n", Kind::Guard), vec!["shellcheck"]);
        assert_eq!(words("command -v jq\n", Kind::Cmd), Vec::<String>::new());
    }

    // spec: gate-sdk/SPEC.md §port-blockers — a command-position expansion is reported by knob
    // name, in every spelling the report resolves: bare, braced, quoted, array-subscripted.
    #[test]
    fn a_command_position_expansion_is_reported_by_knob_name() {
        assert_eq!(words("\"$RENDERER\" --check\n", Kind::Expansion), vec!["RENDERER"]);
        assert_eq!(words("${RENDERER} --check\n", Kind::Expansion), vec!["RENDERER"]);
        assert_eq!(
            words("\"${RENDERER[@]}\" --check\n", Kind::Expansion),
            vec!["RENDERER"]
        );
        assert_eq!(
            words("VAR=1 \"$RENDERER\" x\n", Kind::Expansion),
            vec!["RENDERER"],
            "an assignment prefix keeps the next word in command position"
        );
        assert_eq!(
            words("echo \"$RENDERER\"\n", Kind::Expansion),
            Vec::<String>::new()
        );
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the here-string repair, carried forward by name: an
    // operator consumed whole cannot be re-entered part-way, and the pre-repair scan abandoned the
    // rest of the declaration and reported the unread remainder as *clean*.
    #[test]
    fn a_here_string_is_consumed_whole_and_the_scan_continues_past_it() {
        let text = "grep -q x <<<\"$payload\"\njq --version\n";
        assert_eq!(words(text, Kind::Cmd), vec!["grep", "jq"]);
        let heredoc = "cat <<'EOF'\njq is not a command here\nEOF\nshellcheck x\n";
        assert_eq!(words(heredoc, Kind::Cmd), vec!["cat", "shellcheck"]);
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the double-bracket scoping is load-bearing: inside
    // `[[ … ]]` a `)` is never a case-pattern close, while a genuine `case` pattern inside a
    // command substitution must keep its frame. The negative control rides the same test.
    #[test]
    fn a_paren_pops_a_frame_only_inside_double_brackets() {
        let text = "if [[ \"$(printf x)\" == x ]]; then jq .; fi\nshellcheck y\n";
        assert_eq!(
            words(text, Kind::Cmd),
            vec!["jq", "shellcheck"],
            "the conditional stole the frame and the rest of the declaration was lost"
        );
        let control = "v=$( case \"$x\" in a) echo hi ;; esac )\njq .\n";
        assert_eq!(words(control, Kind::Cmd), vec!["echo", "jq"]);
    }

    // spec: gate-sdk/SPEC.md §port-blockers — an end-of-file balance check cannot see an early
    // pop: the real closing `)` lands on an empty stack and the file balances by coincidence while
    // everything after the `case` is lost. So the assertion is on what follows, not on the depth.
    #[test]
    fn a_case_block_restores_command_position_for_everything_after_it() {
        let text = "case \"$x\" in\n  a) jq . ;;\n  b) shellcheck y ;;\nesac\nruby -e 1\n";
        assert_eq!(words(text, Kind::Cmd), vec!["jq", "shellcheck", "ruby"]);
    }

    // spec: gate-sdk/SPEC.md §port-blockers — a backslash continuation keeps the next line in the
    // operand position the current command left it in, rather than restarting a command.
    #[test]
    fn a_continuation_does_not_restart_a_command() {
        let text = "jq \\\n  --raw-output \\\n  .\nshellcheck y\n";
        assert_eq!(words(text, Kind::Cmd), vec!["jq", "shellcheck"]);
    }

    // spec: gate-sdk/SPEC.md §port-blockers — a comment runs to end of line and a substitution
    // frame restores the quoting state it was pushed under, so the scan never leaks either.
    #[test]
    fn a_comment_and_a_substitution_frame_both_close_cleanly() {
        assert_eq!(words("jq . # shellcheck is prose here\n", Kind::Cmd), vec!["jq"]);
        assert_eq!(
            words("x=\"$(shellcheck -f json y)\"\njq .\n", Kind::Cmd),
            vec!["shellcheck", "jq"]
        );
        assert_eq!(words("x=`ruby -e 1`\njq .\n", Kind::Cmd), vec!["ruby", "jq"]);
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the row's line number is the word's own start line,
    // which is what makes the evidence column a `<file>:<line>` a reader can open.
    #[test]
    fn a_token_carries_the_line_its_word_started_on() {
        let toks = command_positions("jq .\n\nshellcheck \\\n  y\n");
        assert_eq!(toks.len(), 2);
        assert_eq!(toks[0].line, 1);
        assert_eq!(toks[1].line, 3);
    }
}
