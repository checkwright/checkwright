// spec: gate-sdk/SPEC.md §The since-base corpus — the string literals of a file, told from its
// comments by the file's language, and the fragments a literal splits into
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lang {
    Rust,
    Hash,
    Plain,
}

// spec: gate-sdk/SPEC.md §The since-base corpus — `//` and `/* */` in a `.rs` file, `#` and single
// quotes in a `.sh` or `.py` file, double quotes alone anywhere else
pub fn lang(path: &str) -> Lang {
    if path.ends_with(".rs") {
        Lang::Rust
    } else if path.ends_with(".sh") || path.ends_with(".py") {
        Lang::Hash
    } else {
        Lang::Plain
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Comment,
    Str { escapes: bool },
    Char,
}

// spec: gate-sdk/SPEC.md §The since-base corpus — one comment or literal: its whole byte span,
// the span of its body, and the line it opens on
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
    pub body: (usize, usize),
    pub line: usize,
}

#[derive(Debug, Default)]
pub struct Lexed {
    pub spans: Vec<Span>,
    pub unterminated: Option<usize>,
}

fn ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

struct Lexer<'a> {
    b: &'a [u8],
    i: usize,
    line: usize,
    out: Lexed,
}

impl<'a> Lexer<'a> {
    fn at(&self, k: usize) -> u8 {
        self.b.get(self.i + k).copied().unwrap_or(0)
    }

    fn advance(&mut self, n: usize) {
        for _ in 0..n {
            if self.at(0) == b'\n' {
                self.line += 1;
            }
            self.i += 1;
        }
    }

    fn push(&mut self, kind: Kind, start: usize, body: (usize, usize), line: usize) {
        self.out.spans.push(Span { kind, start, end: self.i, body, line });
    }

    fn open(&mut self, line: usize) -> bool {
        self.out.unterminated = Some(line);
        false
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — a quoted run closed by `quote` with
    // backslash escapes; `line_bound` drops a run its line does not close, which no literal is
    fn quoted(&mut self, quote: u8, escapes: bool, line_bound: bool, skip: usize) -> bool {
        let (start, line) = (self.i, self.line);
        self.advance(skip);
        let from = self.i;
        while self.i < self.b.len() {
            match self.at(0) {
                b'\\' if escapes => self.advance(2),
                b'\n' if line_bound => {
                    (self.i, self.line) = (start + skip, line);
                    return true;
                }
                c if c == quote => {
                    let to = self.i;
                    self.advance(1);
                    self.push(Kind::Str { escapes }, start, (from, to), line);
                    return true;
                }
                _ => self.advance(1),
            }
        }
        if line_bound {
            (self.i, self.line) = (start + skip, line);
            return true;
        }
        self.open(line)
    }

    fn line_comment(&mut self) {
        let (start, line) = (self.i, self.line);
        while self.i < self.b.len() && self.at(0) != b'\n' {
            self.i += 1;
        }
        self.push(Kind::Comment, start, (start, self.i), line);
    }

    fn block_comment(&mut self) -> bool {
        let (start, line) = (self.i, self.line);
        let mut depth = 0usize;
        while self.i < self.b.len() {
            if self.at(0) == b'/' && self.at(1) == b'*' {
                depth += 1;
                self.advance(2);
            } else if self.at(0) == b'*' && self.at(1) == b'/' {
                depth -= 1;
                self.advance(2);
                if depth == 0 {
                    self.push(Kind::Comment, start, (start, self.i), line);
                    return true;
                }
            } else {
                self.advance(1);
            }
        }
        self.open(line)
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — `r"…"`, `r#"…"#` and their `b`/`c` forms,
    // closed by a quote and the same count of hashes
    fn raw(&mut self, prefix: usize) -> Option<bool> {
        let mut k = prefix;
        while self.at(k) == b'#' {
            k += 1;
        }
        if self.at(k) != b'"' {
            return None;
        }
        let hashes = k - prefix;
        let (start, line) = (self.i, self.line);
        self.advance(k + 1);
        let from = self.i;
        while self.i < self.b.len() {
            if self.at(0) == b'"' && (1..=hashes).all(|h| self.at(h) == b'#') {
                let to = self.i;
                self.advance(1 + hashes);
                self.push(Kind::Str { escapes: false }, start, (from, to), line);
                return Some(true);
            }
            self.advance(1);
        }
        Some(self.open(line))
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — a char literal is a quote, an escape or one
    // character, and a quote; anything else after a quote is a lifetime or a label
    fn char_or_lifetime(&mut self) -> bool {
        let (start, line) = (self.i, self.line);
        if self.at(1) == b'\\' {
            self.advance(3);
            while self.i < self.b.len() && self.at(0) != b'\'' && self.at(0) != b'\n' {
                self.advance(1);
            }
            if self.at(0) != b'\'' {
                return self.open(line);
            }
            self.advance(1);
            self.push(Kind::Char, start, (start + 1, self.i - 1), line);
            return true;
        }
        let width = std::str::from_utf8(&self.b[self.i + 1..]).ok().and_then(|s| s.chars().next()).map_or(1, char::len_utf8);
        if self.at(1) != b'\n' && self.at(1 + width) == b'\'' {
            self.advance(2 + width);
            self.push(Kind::Char, start, (start + 1, self.i - 1), line);
        } else {
            self.advance(1);
        }
        true
    }

    fn rust(&mut self) {
        while self.i < self.b.len() {
            let c = self.at(0);
            let word_start = self.i == 0 || !ident(self.b[self.i - 1]);
            let ok = match c {
                b'/' if self.at(1) == b'/' => {
                    self.line_comment();
                    true
                }
                b'/' if self.at(1) == b'*' => self.block_comment(),
                b'"' => self.quoted(b'"', true, false, 1),
                b'\'' => self.char_or_lifetime(),
                b'r' if word_start => match self.raw(1) {
                    Some(ok) => ok,
                    None => {
                        self.advance(1);
                        true
                    }
                },
                b'b' | b'c' if word_start && self.at(1) == b'r' => match self.raw(2) {
                    Some(ok) => ok,
                    None => {
                        self.advance(1);
                        true
                    }
                },
                b'b' | b'c' if word_start && self.at(1) == b'"' => self.quoted(b'"', true, false, 2),
                b'b' if word_start && self.at(1) == b'\'' => {
                    self.advance(1);
                    self.char_or_lifetime()
                }
                _ => {
                    self.advance(1);
                    true
                }
            };
            if !ok {
                return;
            }
        }
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — `#` opens a comment where a word starts;
    // Python's triple quotes are its one literal spanning lines
    fn hash(&mut self) {
        while self.i < self.b.len() {
            let c = self.at(0);
            let prev = if self.i == 0 { b'\n' } else { self.b[self.i - 1] };
            match c {
                b'#' if prev.is_ascii_whitespace() || matches!(prev, b';' | b'|' | b'&' | b'(') => self.line_comment(),
                b'\\' => self.advance(2),
                b'"' | b'\'' if self.at(1) == c && self.at(2) == c => {
                    let (start, line) = (self.i, self.line);
                    self.advance(3);
                    let from = self.i;
                    while self.i < self.b.len() && !(self.at(0) == c && self.at(1) == c && self.at(2) == c) {
                        let step = if self.at(0) == b'\\' { 2 } else { 1 };
                        self.advance(step);
                    }
                    if self.i >= self.b.len() {
                        self.i = start + 3;
                        self.line = line;
                        continue;
                    }
                    let to = self.i;
                    self.advance(3);
                    self.push(Kind::Str { escapes: true }, start, (from, to), line);
                }
                b'"' => {
                    self.quoted(b'"', true, true, 1);
                }
                b'\'' => {
                    self.quoted(b'\'', false, true, 1);
                }
                _ => self.advance(1),
            }
        }
    }

    fn plain(&mut self) {
        while self.i < self.b.len() {
            if self.at(0) == b'"' {
                self.quoted(b'"', true, true, 1);
            } else {
                self.advance(1);
            }
        }
    }
}

// spec: gate-sdk/SPEC.md §The since-base corpus — every comment and literal span of a file; an
// unterminated one ends the read and names its line
pub fn lex(text: &str, lang: Lang) -> Lexed {
    let mut l = Lexer { b: text.as_bytes(), i: 0, line: 1, out: Lexed::default() };
    match lang {
        Lang::Rust => l.rust(),
        Lang::Hash => l.hash(),
        Lang::Plain => l.plain(),
    }
    l.out
}

// spec: gate-sdk/SPEC.md §The since-base corpus — a literal's body, the line it opens on, and
// whether it is an assertion's message
#[derive(Debug, PartialEq)]
pub struct Literal<'a> {
    pub line: usize,
    pub body: &'a str,
    pub escapes: bool,
    pub message: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Frame {
    Other,
    Expect,
    Panic,
    Assert(usize),
}

impl Frame {
    fn message_at(self, commas: usize) -> bool {
        match self {
            Frame::Other => false,
            Frame::Expect => commas == 0,
            Frame::Panic => true,
            Frame::Assert(operands) => commas >= operands,
        }
    }
}

// spec: gate-sdk/SPEC.md §The since-base corpus — the call a `(` opens, read from the code before
// it: `.expect(`, `panic!(` and an `assert…!(` taking two operands when it compares, else one
fn callee(code: &[u8], open: usize) -> Frame {
    let mut k = open;
    while k > 0 && code[k - 1].is_ascii_whitespace() {
        k -= 1;
    }
    let bang = k > 0 && code[k - 1] == b'!';
    if bang {
        k -= 1;
    }
    let end = k;
    while k > 0 && ident(code[k - 1]) {
        k -= 1;
    }
    let name = std::str::from_utf8(&code[k..end]).unwrap_or("");
    let method = k > 0 && code[k - 1] == b'.';
    match (name, bang, method) {
        ("expect", false, true) => Frame::Expect,
        ("panic", true, _) => Frame::Panic,
        (n, true, _) if n.starts_with("assert") => {
            Frame::Assert(if n.ends_with("_eq") || n.ends_with("_ne") || n.ends_with("_matches") { 2 } else { 1 })
        }
        _ => Frame::Other,
    }
}

// spec: gate-sdk/SPEC.md §The since-base corpus — a literal anywhere inside a message argument,
// a nested call's included, is a message
fn messages(text: &str, spans: &[Span]) -> Vec<bool> {
    let mut code = text.as_bytes().to_vec();
    for s in spans {
        code[s.start..s.end].iter_mut().filter(|b| **b != b'\n').for_each(|b| *b = b' ');
    }
    let mut stack: Vec<(Frame, usize, bool)> = Vec::new();
    let mut out = Vec::new();
    let mut next = spans.iter().filter(|s| matches!(s.kind, Kind::Str { .. })).peekable();
    for (i, &c) in code.iter().enumerate() {
        while next.next_if(|s| s.start <= i).is_some() {
            out.push(stack.last().is_some_and(|&(f, n, inherited)| inherited || f.message_at(n)));
        }
        match c {
            b'(' | b'[' | b'{' => {
                let inherited = stack.last().is_some_and(|&(f, n, inh)| inh || f.message_at(n));
                let frame = if c == b'(' { callee(&code, i) } else { Frame::Other };
                stack.push((frame, 0, inherited));
            }
            b')' | b']' | b'}' => {
                stack.pop();
            }
            b',' => {
                if let Some(top) = stack.last_mut() {
                    top.1 += 1;
                }
            }
            _ => {}
        }
    }
    out.extend(next.map(|_| false));
    out
}

// spec: gate-sdk/SPEC.md §The since-base corpus — the literals of a file, comments skipped; a
// Rust literal's message flag is its call's
pub fn literals(text: &str, lang: Lang) -> Vec<Literal<'_>> {
    let lexed = lex(text, lang);
    let strs: Vec<&Span> = lexed.spans.iter().filter(|s| matches!(s.kind, Kind::Str { .. })).collect();
    let flags = if lang == Lang::Rust { messages(text, &lexed.spans) } else { vec![false; strs.len()] };
    strs.iter()
        .zip(flags)
        .map(|(s, message)| Literal {
            line: s.line,
            body: &text[s.body.0..s.body.1],
            escapes: matches!(s.kind, Kind::Str { escapes: true }),
            message,
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §The since-base corpus — the counting floor: 8 characters, 4 ASCII letters
pub fn counts(fragment: &str) -> bool {
    fragment.chars().count() >= 8 && fragment.chars().filter(char::is_ascii_alphabetic).count() >= 4
}

// spec: gate-sdk/SPEC.md §The since-base corpus — the width of a two-byte cut: an escape, `$(`,
// a special parameter and `%s`
const PAIR: usize = 2;

// spec: gate-sdk/SPEC.md §The since-base corpus — the counted runs between placeholders (`{…}`,
// `$x`, `${…}`, `$(`, `%s`) and escapes
pub fn fragments(body: &str, escapes: bool) -> Vec<&str> {
    let b = body.as_bytes();
    let mut out = Vec::new();
    let (mut from, mut i) = (0, 0);
    while i < b.len() {
        let cut = match b[i] {
            b'\\' if escapes => Some(PAIR),
            b'{' => b[i..].iter().position(|&c| c == b'}').map(|e| e + 1),
            b'$' if b.get(i + 1) == Some(&b'{') => b[i..].iter().position(|&c| c == b'}').map(|e| e + 1),
            b'$' if b.get(i + 1) == Some(&b'(') => Some(PAIR),
            b'$' if b.get(i + 1).is_some_and(|&c| ident(c)) => Some(1 + b[i + 1..].iter().take_while(|&&c| ident(c)).count()),
            b'$' if b.get(i + 1).is_some_and(|c| b"@*#?$!-".contains(c)) => Some(PAIR),
            b'%' if b.get(i + 1) == Some(&b's') => Some(PAIR),
            _ => None,
        };
        match cut {
            Some(n) => {
                out.push(&body[from..i]);
                i = (i + n).min(b.len());
                while !body.is_char_boundary(i) {
                    i += 1;
                }
                from = i;
            }
            None => i += 1,
        }
    }
    out.push(&body[from..]);
    out.retain(|f| counts(f));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bodies(text: &str, lang: Lang) -> Vec<(usize, &str, bool)> {
        literals(text, lang).into_iter().map(|l| (l.line, l.body, l.message)).collect()
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — comments are no literal, a raw string closes
    // only at its own hash count, and a lifetime is no char literal
    #[test]
    fn rust_literals_skip_comments_and_read_raw_strings() {
        let text = "// \"in a comment\"\n/* outer /* \"nested\" */ still */ let a = \"plain \\\" quote\";\nfn f<'a>(x: &'a str) -> char { 'x' }\nlet r = r#\"raw \"inner\" text\"#;\nlet b = br##\"bytes\"##; let q = '\\'';\n";
        assert_eq!(
            bodies(text, Lang::Rust),
            [(2, "plain \\\" quote", false), (4, "raw \"inner\" text", false), (5, "bytes", false)]
        );
        assert_eq!(lex("let s = \"open\n", Lang::Rust).unterminated, Some(1));
        assert_eq!(lex("x /* a /* b */\n", Lang::Rust).unterminated, Some(1));
        assert_eq!(lex("let s = r#\"open\"\n", Lang::Rust).unterminated, Some(1));
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — an assertion's message is never listed;
    // its operands are
    #[test]
    fn an_assertion_message_is_flagged_and_its_operands_are_not() {
        let text = "assert_eq!(out, \"expected words\", \"message {}\", x);\nassert!(s.contains(\"operand text\"), \"why {}\", format!(\"nested words\"));\nv.expect(\"cannot read it\");\npanic!(\"broke badly\");\nf(\"plain call\", \"second\");\n";
        assert_eq!(
            bodies(text, Lang::Rust),
            [
                (1, "expected words", false),
                (1, "message {}", true),
                (2, "operand text", false),
                (2, "why {}", true),
                (2, "nested words", true),
                (3, "cannot read it", true),
                (4, "broke badly", true),
                (5, "plain call", false),
                (5, "second", false),
            ]
        );
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — `#` opens a comment only at a word start,
    // and a quote its line does not close is no literal
    #[test]
    fn hash_and_plain_literals_are_line_bound() {
        let text = "echo \"double $x\" 'single' # \"comment\"\nn=${#arr} \"after\"\necho don't stop\n\"\"\"triple\nline\"\"\"\n";
        assert_eq!(
            bodies(text, Lang::Hash),
            [(1, "double $x", false), (1, "single", false), (2, "after", false), (4, "triple\nline", false)]
        );
        assert_eq!(bodies("a \"one\" b \"two\n\"three\"\n", Lang::Plain), [(1, "one", false), (2, "three", false)]);
        assert_eq!((lang("a/b.rs"), lang("t.sh"), lang("x.py"), lang("README.md")), (Lang::Rust, Lang::Hash, Lang::Hash, Lang::Plain));
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — placeholders and escapes cut a literal, and
    // a fragment under the floor is dropped
    #[test]
    fn a_literal_splits_at_placeholders_and_escapes() {
        assert_eq!(fragments("cannot read {} config file", true), ["cannot read ", " config file"]);
        assert_eq!(fragments("missing ${name} value here\\nsecond line text", true), ["missing ", " value here", "second line text"]);
        assert_eq!(fragments("checking $name now %s please", true), ["checking "]);
        assert_eq!(fragments("run $(make all) twice", true), ["make all) twice"]);
        assert_eq!(fragments("raw \\n is one fragment", false), ["raw \\n is one fragment"]);
        assert!(counts("abcd1234") && !counts("abc12345") && !counts("abcdefg"));
    }
}
