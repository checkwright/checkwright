// spec: gate-sdk/SPEC.md §The shell reader — the one reader of shell text for the shell content
// gates: logical lines, code and comment halves, pipelines and their stages, `pipefail` scope
use std::borrow::Cow;

pub struct Options<'a> {
    // spec: gate-sdk/SPEC.md §The shell reader — the commands whose here-doc or quoted operand is
    // read as a script of its own
    pub body: &'a [String],
    // spec: gate-sdk/SPEC.md §check-pipefail-readers — a library is linted with `pipefail` on from
    // its first line
    pub pipefail: bool,
}

#[derive(Debug, Clone)]
pub struct Word {
    pub raw: String,
    pub value: String,
    pub quoted: bool,
    pub expands: bool,
    pub whole_quote: bool,
    pub redir: bool,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub words: Vec<Word>,
    pub end: usize,
    pub shell: usize,
}

impl Command {
    pub fn args(&self) -> impl Iterator<Item = &Word> {
        self.words.iter().filter(|w| !w.redir)
    }
}

#[derive(Debug, Clone)]
pub struct Stage {
    pub end: usize,
    pub line: usize,
    pub last: usize,
    pub cmd: Option<usize>,
    pub text: String,
    pub raw: String,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub stages: Vec<Stage>,
    pub pipefail: bool,
    pub line: usize,
    pub last: usize,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub line: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub first: usize,
    pub last: usize,
    pub start: usize,
    pub end: usize,
}

// spec: gate-sdk/SPEC.md §The shell reader — a span the code half leaves out: a continuation, a
// comment, a here-doc body, and an operator-joined newline read as one blank
#[derive(Debug, Clone, Copy)]
struct Skip {
    start: usize,
    end: usize,
    blank: bool,
}

#[derive(Debug, Default)]
pub struct Script {
    pub commands: Vec<Command>,
    pub pipelines: Vec<Pipeline>,
    pub comments: Vec<Comment>,
    pub logicals: Vec<Logical>,
    pub bodies: Vec<Script>,
    pub sets_pipefail: bool,
    src: String,
    base: usize,
    skips: Vec<Skip>,
}

impl Script {
    // spec: gate-sdk/SPEC.md §The shell reader — the code half between two of the script's
    // offsets, quoted text kept
    pub fn code(&self, start: usize, end: usize) -> String {
        let b = self.src.as_bytes();
        let at = |o: usize| o.saturating_sub(self.base).min(b.len());
        let mut out: Vec<u8> = Vec::new();
        let mut k = start;
        for s in self.skips.iter().filter(|s| s.end > start && s.start < end) {
            if s.start > k {
                out.extend_from_slice(&b[at(k)..at(s.start)]);
            }
            if s.blank {
                out.push(b' ');
            }
            k = k.max(s.end);
        }
        if k < end {
            out.extend_from_slice(&b[at(k)..at(end)]);
        }
        lossy(&out)
    }

    pub fn logical_at(&self, offset: usize) -> Option<&Logical> {
        self.logicals.iter().find(|l| l.start <= offset && offset < l.end)
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub line: usize,
}

// spec: gate-sdk/SPEC.md §The shell reader — `pipefail` is set by a `set` command's `-o pipefail`
// or a combined flag word ending in `o`, never by the bare word
pub fn sets_pipefail(code: &str) -> bool {
    let words: Vec<&str> = code.split_whitespace().collect();
    words.contains(&"set")
        && words.windows(2).any(|w| {
            w[1] == "pipefail"
                && w[0].len() >= 2
                && w[0].starts_with('-')
                && !w[0].starts_with("--")
                && w[0].ends_with('o')
                && w[0][1..].chars().all(|c| c.is_ascii_alphabetic())
        })
}

pub fn read(text: &str, opts: &Options) -> Result<Script, ParseError> {
    read_at(text, opts, 0, 1)
}

fn read_at(text: &str, opts: &Options, base: usize, first_line: usize) -> Result<Script, ParseError> {
    let mut r = Reader {
        b: text.as_bytes(),
        i: 0,
        line: first_line,
        base,
        frames: vec![Frame::new(Kind::Top, first_line, 0, opts.pipefail)],
        next_shell: 1,
        pending: Vec::new(),
        docs: Vec::new(),
        lstart: 0,
        lfirst: first_line,
        out: Script::default(),
    };
    r.run()?;
    let Reader { mut out, docs, .. } = r;
    out.src = text.to_string();
    out.base = base;
    let body_opts = Options { body: opts.body, pipefail: false };
    if !opts.body.is_empty() {
        let named = |w: &Word| !w.redir && !w.quoted && opts.body.contains(&w.value);
        for d in &docs {
            if d.cmd.is_some_and(|c| out.commands[c].words.iter().any(named)) {
                out.bodies.push(read_at(&text[d.start..d.end], &body_opts, base + d.start, d.first)?);
            }
        }
        let mut operands: Vec<(String, usize, usize)> = Vec::new();
        for c in &out.commands {
            let mut seen = false;
            for w in c.args() {
                if seen && w.whole_quote {
                    let inner = &w.raw[1..w.raw.len() - 1];
                    let inner = if w.raw.starts_with('"') { unescape_dq(inner) } else { inner.to_string() };
                    operands.push((inner, w.start + 1, w.line));
                    out.skips.push(Skip { start: w.start, end: w.end, blank: true });
                }
                seen |= named(w);
            }
        }
        out.skips.sort_by_key(|s| s.start);
        for (inner, at, line) in operands {
            out.bodies.push(read_at(&inner, &body_opts, at, line)?);
        }
    }
    Ok(out)
}

fn unescape_dq(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' && matches!(chars.peek(), Some('"' | '\\' | '$' | '`')) {
            out.extend(chars.next());
        } else {
            out.push(c);
        }
    }
    out
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Top,
    Paren,
    Dollar,
    Proc,
    Brace,
    Array,
}

#[derive(Clone, Copy, PartialEq)]
enum Case {
    Head,
    Pattern,
    Body,
}

// spec: gate-sdk/SPEC.md §The shell reader — the reserved words after which the next word is still
// in command position
const KEEP: &[&str] = &["if", "then", "else", "elif", "do", "while", "until", "!", "time"];

struct WordB {
    start: usize,
    line: usize,
    value: Vec<u8>,
    quoted: bool,
    expands: bool,
    parts: u32,
    plain_run: bool,
}

struct StageB {
    start: usize,
    end: usize,
    line: usize,
    last: usize,
    cmd: Option<usize>,
    compound: bool,
    text: Vec<u8>,
}

struct Frame {
    kind: Kind,
    open_line: usize,
    shell: usize,
    pipefail: bool,
    dq: Option<usize>,
    word: Option<WordB>,
    cmd: Option<usize>,
    cmd_pos: bool,
    stages: Vec<Stage>,
    stage: Option<StageB>,
    pending_op: bool,
    test: bool,
    cases: Vec<Case>,
    redir_next: bool,
    heredoc_next: Option<bool>,
}

impl Frame {
    fn new(kind: Kind, open_line: usize, shell: usize, pipefail: bool) -> Frame {
        Frame {
            kind,
            open_line,
            shell,
            pipefail,
            dq: None,
            word: None,
            cmd: None,
            cmd_pos: true,
            stages: Vec::new(),
            stage: None,
            pending_op: false,
            test: false,
            cases: Vec::new(),
            redir_next: false,
            heredoc_next: None,
        }
    }

    // spec: gate-sdk/SPEC.md §The shell reader — an array's elements and a case pattern are words
    // that open no command
    fn code(&self) -> bool {
        self.kind != Kind::Array && self.cases.last() != Some(&Case::Pattern)
    }
}

struct HereDoc {
    cmd: Option<usize>,
    delim: Vec<u8>,
    strip: bool,
    line: usize,
    start: usize,
    end: usize,
    first: usize,
}

struct Reader<'t> {
    b: &'t [u8],
    i: usize,
    line: usize,
    base: usize,
    frames: Vec<Frame>,
    next_shell: usize,
    pending: Vec<HereDoc>,
    docs: Vec<HereDoc>,
    lstart: usize,
    lfirst: usize,
    out: Script,
}

fn lossy(b: &[u8]) -> String {
    match String::from_utf8_lossy(b) {
        Cow::Borrowed(s) => s.to_string(),
        Cow::Owned(s) => s,
    }
}

fn is_blank(c: Option<u8>) -> bool {
    matches!(c, None | Some(b' ' | b'\t' | b'\n' | b'\r'))
}

fn delimits(c: Option<u8>) -> bool {
    is_blank(c) || matches!(c, Some(b';' | b'&' | b'|' | b')' | b'<' | b'>'))
}

impl Reader<'_> {
    fn top(&mut self) -> &mut Frame {
        let n = self.frames.len() - 1;
        &mut self.frames[n]
    }

    fn peek(&self, k: usize) -> Option<u8> {
        self.b.get(self.i + k).copied()
    }

    fn err<T>(&self, line: usize) -> Result<T, ParseError> {
        Err(ParseError { line })
    }

    fn adv(&mut self, n: usize, text: bool) {
        let end = (self.i + n).min(self.b.len());
        let bytes = &self.b[self.i..end];
        self.line += bytes.iter().filter(|c| **c == b'\n').count();
        if text {
            let n = self.frames.len() - 1;
            if let Some(s) = self.frames[n].stage.as_mut() {
                s.text.extend_from_slice(bytes);
            }
        }
        self.i = end;
    }

    fn push_value(&mut self, bytes: &[u8]) {
        if let Some(w) = self.top().word.as_mut() {
            w.value.extend_from_slice(bytes);
        }
    }

    fn seg(&mut self, quoted: bool) {
        if let Some(w) = self.top().word.as_mut() {
            if quoted {
                w.parts += 1;
                w.plain_run = false;
                w.quoted = true;
            } else if !w.plain_run {
                w.parts += 1;
                w.plain_run = true;
            }
        }
    }

    fn expands(&mut self) {
        if let Some(w) = self.top().word.as_mut() {
            w.expands = true;
        }
    }

    fn token_begin(&mut self) {
        let (base, i, line) = (self.base, self.i, self.line);
        let idx = self.out.commands.len();
        let f = self.top();
        f.pending_op = false;
        if !f.code() {
            return;
        }
        let stage = f.stage.get_or_insert(StageB {
            start: base + i,
            end: base + i,
            line,
            last: line,
            cmd: None,
            compound: false,
            text: Vec::new(),
        });
        if f.cmd.is_none() {
            if stage.cmd.is_none() && !stage.compound {
                stage.cmd = Some(idx);
            }
            let shell = f.shell;
            f.cmd = Some(idx);
            self.out.commands.push(Command { words: Vec::new(), end: base + i, shell });
        }
    }

    fn word_begin(&mut self) {
        if self.top().word.is_some() {
            return;
        }
        self.token_begin();
        let w = WordB {
            start: self.base + self.i,
            line: self.line,
            value: Vec::new(),
            quoted: false,
            expands: false,
            parts: 0,
            plain_run: false,
        };
        self.top().word = Some(w);
    }

    fn plain(&mut self, n: usize) {
        self.word_begin();
        self.seg(false);
        let bytes = self.b[self.i..(self.i + n).min(self.b.len())].to_vec();
        self.push_value(&bytes);
        self.adv(n, true);
    }

    fn mark_end(&mut self) {
        let (at, line) = (self.base + self.i, self.line);
        let f = self.top();
        if let Some(s) = f.stage.as_mut() {
            s.end = at;
            s.last = line;
        }
        if let Some(c) = f.cmd {
            self.out.commands[c].end = at;
        }
    }

    fn end_word(&mut self) {
        let Some(w) = self.top().word.take() else { return };
        let raw = lossy(&self.b[w.start - self.base..self.i]);
        let mut word = Word {
            raw,
            value: lossy(&w.value),
            quoted: w.quoted,
            expands: w.expands,
            whole_quote: w.quoted && w.parts == 1,
            redir: false,
            start: w.start,
            end: self.base + self.i,
            line: w.line,
        };
        let plain = !word.quoted;
        if !self.top().code() {
            let f = self.top();
            if f.kind != Kind::Array && plain && word.value == "esac" {
                f.cases.pop();
            }
            return;
        }
        self.mark_end();
        let f = self.frames.last_mut().expect("a frame is always open");
        let Some(c) = f.cmd else { return };
        if let Some(strip) = f.heredoc_next.take() {
            self.pending.push(HereDoc {
                cmd: Some(c),
                delim: word.value.clone().into_bytes(),
                strip,
                line: word.line,
                start: 0,
                end: 0,
                first: 0,
            });
            word.redir = true;
            self.out.commands[c].words.push(word);
            return;
        }
        if f.redir_next {
            f.redir_next = false;
            word.redir = true;
            self.out.commands[c].words.push(word);
            return;
        }
        if f.test {
            if plain && word.value == "]]" {
                f.test = false;
            }
            self.out.commands[c].words.push(word);
            return;
        }
        let at_cmd = f.cmd_pos && plain;
        if at_cmd {
            match word.value.as_str() {
                "case" => f.cases.push(Case::Head),
                "[[" => f.test = true,
                "esac" if f.cases.last() == Some(&Case::Body) => {
                    f.cases.pop();
                }
                _ => {}
            }
        }
        let is_in = plain && word.value == "in" && f.cases.last() == Some(&Case::Head);
        let keep = at_cmd && KEEP.contains(&word.value.as_str());
        let words = &mut self.out.commands[c].words;
        words.push(word);
        let function_name = words.len() == 2 && words[0].value == "function" && !words[0].quoted;
        let f = self.top();
        f.cmd_pos = keep || function_name;
        if is_in {
            self.end_pipeline();
            if let Some(top) = self.top().cases.last_mut() {
                *top = Case::Pattern;
            }
        }
    }

    fn end_command(&mut self) {
        self.end_word();
        let f = self.top();
        f.redir_next = false;
        f.heredoc_next = None;
        f.cmd_pos = true;
        if let Some(c) = f.cmd.take() {
            let code: Vec<&str> = self.out.commands[c].args().map(|w| w.value.as_str()).collect();
            if sets_pipefail(&code.join(" ")) {
                self.top().pipefail = true;
                self.out.sets_pipefail = true;
            }
        }
    }

    fn end_stage(&mut self) {
        self.end_command();
        let base = self.base;
        let b = self.b;
        let f = self.top();
        if let Some(s) = f.stage.take() {
            f.stages.push(Stage {
                end: s.end,
                line: s.line,
                last: s.last,
                cmd: s.cmd,
                text: lossy(&s.text),
                raw: lossy(&b[s.start - base..s.end - base]),
            });
        }
    }

    fn end_pipeline(&mut self) {
        self.end_stage();
        let f = self.top();
        let stages = std::mem::take(&mut f.stages);
        let pipefail = f.pipefail;
        if stages.len() >= 2 {
            let (line, last) = (stages[0].line, stages[stages.len() - 1].last);
            self.out.pipelines.push(Pipeline { stages, pipefail, line, last });
        }
    }

    fn push_frame(&mut self, kind: Kind) {
        let parent = self.top();
        let pipefail = parent.pipefail;
        let shell = if kind == Kind::Brace || kind == Kind::Array {
            parent.shell
        } else {
            self.next_shell += 1;
            self.next_shell
        };
        let line = self.line;
        self.frames.push(Frame::new(kind, line, shell, pipefail));
    }

    fn compound_begin(&mut self) {
        let (base, i, line) = (self.base, self.i, self.line);
        let f = self.top();
        f.pending_op = false;
        f.stage.get_or_insert(StageB {
            start: base + i,
            end: base + i,
            line,
            last: line,
            cmd: None,
            compound: true,
            text: Vec::new(),
        });
    }

    fn comment(&mut self) {
        let end = self.b[self.i..].iter().position(|c| *c == b'\n').map_or(self.b.len(), |p| self.i + p);
        let text = lossy(&self.b[self.i..end]);
        self.out.comments.push(Comment { line: self.line, text });
        self.skip(self.i, end, false);
        self.i = end;
    }

    fn skip(&mut self, start: usize, end: usize, blank: bool) {
        let base = self.base;
        self.out.skips.push(Skip { start: base + start, end: base + end, blank });
    }

    fn end_logical(&mut self, end: usize) {
        if end > self.lstart {
            let (base, first, last) = (self.base, self.lfirst, self.line);
            self.out.logicals.push(Logical { first, last, start: base + self.lstart, end: base + end });
        }
    }

    // comment-tier-exempt: a zero `open` ends the span at the first unescaped `close`, no nesting
    fn span_end(&self, from: usize, open: u8, close: u8) -> Option<usize> {
        let b = self.b;
        let mut depth = 0usize;
        let mut k = from;
        while k < b.len() {
            let c = b[k];
            if c == b'\\' {
                k += 2;
                continue;
            }
            if open != 0 && c == open {
                depth += 1;
            } else if c == close {
                if open == 0 || depth <= 1 {
                    return Some(k);
                }
                depth -= 1;
            } else if open != 0 && c == b'\'' && close != b'\'' {
                k = k + 1 + b[k + 1..].iter().position(|x| *x == b'\'')?;
            } else if open != 0 && c == b'"' {
                k = self.span_end(k + 1, 0, b'"')?;
            }
            k += 1;
        }
        None
    }

    fn opaque(&mut self, from: usize, open: u8, close: u8) -> Result<(), ParseError> {
        let line = self.line;
        let Some(end) = self.span_end(from, open, close) else { return self.err(line) };
        let n = end + 1 - self.i;
        let bytes = self.b[self.i..end + 1].to_vec();
        self.push_value(&bytes);
        self.adv(n, true);
        Ok(())
    }

    fn single(&mut self) -> Result<(), ParseError> {
        let line = self.line;
        self.word_begin();
        self.seg(true);
        let Some(p) = self.b[self.i + 1..].iter().position(|c| *c == b'\'') else { return self.err(line) };
        let inner = self.b[self.i + 1..self.i + 1 + p].to_vec();
        self.push_value(&inner);
        self.adv(p + 2, true);
        Ok(())
    }

    fn dollar(&mut self) -> Result<(), ParseError> {
        let dq = self.top().dq.is_some();
        match (self.peek(1), self.peek(2)) {
            (Some(b'\''), _) if !dq => {
                let line = self.line;
                self.word_begin();
                self.seg(true);
                let Some(end) = self.span_end(self.i + 2, 0, b'\'') else { return self.err(line) };
                let inner = self.b[self.i + 2..end].to_vec();
                self.push_value(&inner);
                self.adv(end + 1 - self.i, true);
            }
            (Some(b'"'), _) if !dq => {
                self.word_begin();
                self.adv(1, true);
                self.open_dq();
            }
            (Some(b'('), Some(b'(')) => {
                self.word_begin();
                self.seg(dq);
                self.expands();
                self.opaque(self.i + 1, b'(', b')')?;
            }
            (Some(b'('), _) => {
                self.word_begin();
                self.seg(dq);
                self.expands();
                self.push_value(b"$(");
                let n = self.frames.len() - 1;
                if let Some(s) = self.frames[n].stage.as_mut() {
                    s.text.extend_from_slice("$(…".as_bytes());
                }
                self.push_frame(Kind::Dollar);
                self.adv(2, false);
            }
            (Some(b'{'), _) => {
                self.word_begin();
                self.seg(dq);
                self.expands();
                self.opaque(self.i + 1, b'{', b'}')?;
            }
            (Some(b'['), _) => {
                self.word_begin();
                self.seg(dq);
                self.expands();
                self.opaque(self.i + 1, b'[', b']')?;
            }
            (Some(c), _) if c.is_ascii_alphabetic() || c == b'_' => {
                self.word_begin();
                self.seg(dq);
                self.expands();
                let n = 1 + self.b[self.i + 1..].iter().take_while(|c| c.is_ascii_alphanumeric() || **c == b'_').count();
                let bytes = self.b[self.i..self.i + n].to_vec();
                self.push_value(&bytes);
                self.adv(n, true);
            }
            (Some(c), _) if c.is_ascii_digit() || b"@*#?$!-".contains(&c) => {
                self.word_begin();
                self.seg(dq);
                self.expands();
                let bytes = self.b[self.i..self.i + 2].to_vec();
                self.push_value(&bytes);
                self.adv(2, true);
            }
            _ => {
                self.word_begin();
                self.seg(dq);
                self.push_value(b"$");
                self.adv(1, true);
            }
        }
        Ok(())
    }

    fn open_dq(&mut self) {
        self.word_begin();
        self.seg(true);
        let line = self.line;
        self.top().dq = Some(line);
        self.adv(1, true);
    }

    fn backtick(&mut self) -> Result<(), ParseError> {
        self.word_begin();
        let dq = self.top().dq.is_some();
        self.seg(dq);
        self.expands();
        self.opaque(self.i + 1, 0, b'`')
    }

    fn dq_step(&mut self) -> Result<(), ParseError> {
        match self.b[self.i] {
            b'"' => {
                self.top().dq = None;
                self.adv(1, true);
            }
            b'\\' => match self.peek(1) {
                Some(b'\n') => {
                    self.skip(self.i, self.i + 2, false);
                    self.adv(2, true);
                }
                Some(c @ (b'$' | b'`' | b'"' | b'\\')) => {
                    self.push_value(&[c]);
                    self.adv(2, true);
                }
                _ => {
                    self.push_value(b"\\");
                    self.adv(1, true);
                }
            },
            b'$' => self.dollar()?,
            b'`' => self.backtick()?,
            c => {
                self.push_value(&[c]);
                self.adv(1, true);
            }
        }
        Ok(())
    }

    fn newline(&mut self) -> Result<(), ParseError> {
        self.end_word();
        let f = self.frames.last().expect("a frame is always open");
        let joins = f.pending_op || f.test || f.kind == Kind::Array;
        let pattern = !f.code();
        if !joins && !pattern {
            self.end_pipeline();
        }
        let nl = self.i;
        if joins {
            self.skip(nl, nl + 1, true);
        } else {
            self.end_logical(nl);
        }
        self.adv(1, false);
        self.heredocs()?;
        if self.i > nl + 1 {
            self.skip(nl + 1, self.i, false);
        }
        if !joins {
            (self.lstart, self.lfirst) = (self.i, self.line);
        }
        Ok(())
    }

    fn heredocs(&mut self) -> Result<(), ParseError> {
        for mut d in std::mem::take(&mut self.pending) {
            d.start = self.i;
            d.first = self.line;
            loop {
                if self.i >= self.b.len() {
                    return self.err(d.line);
                }
                let eol = self.b[self.i..].iter().position(|c| *c == b'\n').map_or(self.b.len(), |p| self.i + p);
                let mut l = &self.b[self.i..eol];
                if d.strip {
                    while l.first() == Some(&b'\t') {
                        l = &l[1..];
                    }
                }
                let l = l.strip_suffix(b"\r").unwrap_or(l);
                let here = self.i;
                self.i = (eol + 1).min(self.b.len());
                if eol < self.b.len() {
                    self.line += 1;
                }
                if l == d.delim.as_slice() {
                    d.end = here;
                    break;
                }
            }
            self.docs.push(d);
        }
        Ok(())
    }

    fn redirect(&mut self) {
        let fd = self.top().word.as_ref().is_some_and(|w| {
            !w.quoted && !w.expands && !w.value.is_empty() && w.value.iter().all(u8::is_ascii_digit)
        });
        if fd {
            self.top().word = None;
        } else {
            self.end_word();
        }
        self.token_begin();
        let (c, n1, n2) = (self.b[self.i], self.peek(1), self.peek(2));
        let (n, heredoc) = match (c, n1, n2) {
            (b'<', Some(b'<'), Some(b'<')) => (3, None),
            (b'<', Some(b'<'), Some(b'-')) => (3, Some(true)),
            (b'<', Some(b'<'), _) => (2, Some(false)),
            (b'>', Some(b'>' | b'&' | b'|'), _) | (b'<', Some(b'&' | b'>'), _) => (2, None),
            _ => (1, None),
        };
        self.adv(n, true);
        let f = self.top();
        match heredoc {
            Some(strip) => f.heredoc_next = Some(strip),
            None => f.redir_next = true,
        }
        self.mark_end();
    }

    fn open_paren(&mut self) -> Result<(), ParseError> {
        let f = self.frames.last().expect("a frame is always open");
        if let Some(w) = f.word.as_ref() {
            let last = w.value.last().copied();
            if last == Some(b'=') && !w.quoted {
                self.push_value(b"(");
                self.push_frame(Kind::Array);
                self.adv(1, true);
                return Ok(());
            }
            if matches!(last, Some(b'@' | b'!' | b'+' | b'*' | b'?')) {
                return self.opaque(self.i, b'(', b')');
            }
            self.end_word();
        } else if self.peek(1) == Some(b'(') && f.cmd_pos {
            self.word_begin();
            self.seg(false);
            self.expands();
            return self.opaque(self.i, b'(', b')');
        }
        self.compound_begin();
        self.push_frame(Kind::Paren);
        self.adv(1, false);
        Ok(())
    }

    fn close_paren(&mut self) -> Result<(), ParseError> {
        self.end_word();
        let kind = self.top().kind;
        match kind {
            Kind::Top | Kind::Brace => return self.err(self.line),
            Kind::Array => {
                self.frames.pop();
                self.push_value(b")");
                self.adv(1, true);
                return Ok(());
            }
            _ => {}
        }
        self.end_pipeline();
        self.frames.pop();
        if kind == Kind::Paren {
            self.adv(1, false);
            self.top().cmd_pos = true;
            self.mark_end();
        } else {
            self.push_value(b")");
            self.adv(1, true);
        }
        Ok(())
    }

    fn run(&mut self) -> Result<(), ParseError> {
        while self.i < self.b.len() {
            if self.top().dq.is_some() {
                self.dq_step()?;
                continue;
            }
            let c = self.b[self.i];
            let next = self.peek(1);
            let f = self.frames.last().expect("a frame is always open");
            let (test, kind, code, cmd_pos) = (f.test, f.kind, f.code(), f.cmd_pos);
            let pattern = kind != Kind::Array && !code;
            let fresh = f.word.is_none();
            if test && b"()|&<>;".contains(&c) && f.word.as_ref().is_some_and(|w| w.value == b"]]") {
                self.end_word();
                continue;
            }
            match c {
                b' ' | b'\t' | b'\r' => {
                    self.end_word();
                    self.adv(1, true);
                }
                b'\\' if next == Some(b'\n') => {
                    self.skip(self.i, self.i + 2, false);
                    self.i += 2;
                    self.line += 1;
                }
                b'\\' => {
                    self.word_begin();
                    self.seg(true);
                    let bytes = self.b[self.i + 1..(self.i + 2).min(self.b.len())].to_vec();
                    self.push_value(&bytes);
                    self.adv(2, true);
                }
                b'\n' => self.newline()?,
                b'#' if fresh => self.comment(),
                b'\'' => self.single()?,
                b'"' => self.open_dq(),
                b'`' => self.backtick()?,
                b'$' => self.dollar()?,
                b'<' | b'>' if next == Some(b'(') => {
                    self.word_begin();
                    self.seg(false);
                    self.expands();
                    self.push_value(&[c, b'(']);
                    self.push_frame(Kind::Proc);
                    self.adv(2, true);
                }
                _ if test && b"()|&<>;".contains(&c) => self.plain(1),
                b')' if pattern => {
                    self.end_word();
                    if let Some(top) = self.top().cases.last_mut() {
                        *top = Case::Body;
                    }
                    self.top().cmd_pos = true;
                    self.adv(1, true);
                }
                b'(' | b'|' | b';' if pattern => {
                    self.end_word();
                    self.adv(1, true);
                }
                b'(' if kind == Kind::Array => self.plain(1),
                b'|' | b'&' | b';' | b'<' | b'>' if kind == Kind::Array => self.plain(1),
                b'(' => self.open_paren()?,
                b')' => self.close_paren()?,
                b';' => {
                    self.end_word();
                    let n = match (next, self.peek(2)) {
                        (Some(b';'), Some(b'&')) => 3, // status-literal-exempt: the `;;&` operator's length
                        (Some(b';' | b'&'), _) => 2, // status-literal-exempt: the `;;` or `;&` operator's length
                        _ => 1,
                    };
                    self.end_pipeline();
                    self.adv(n, true);
                    if n > 1 {
                        if let Some(top) = self.top().cases.last_mut() {
                            *top = Case::Pattern;
                        }
                    }
                }
                b'&' if next == Some(b'&') => {
                    self.end_pipeline();
                    self.adv(2, true);
                    self.top().pending_op = true;
                }
                b'&' if next == Some(b'>') => {
                    self.end_word();
                    self.token_begin();
                    let n = if self.peek(2) == Some(b'>') { 3 } else { 2 };
                    self.adv(n, true);
                    self.top().redir_next = true;
                    self.mark_end();
                }
                b'&' => {
                    self.end_pipeline();
                    self.adv(1, true);
                }
                b'|' if next == Some(b'|') => {
                    self.end_pipeline();
                    self.adv(2, true);
                    self.top().pending_op = true;
                }
                b'|' => {
                    self.end_stage();
                    self.adv(if next == Some(b'&') { 2 } else { 1 }, false);
                    self.top().pending_op = true;
                }
                b'<' | b'>' => self.redirect(),
                b'{' if fresh && cmd_pos && code && is_blank(next) => {
                    self.compound_begin();
                    self.push_frame(Kind::Brace);
                    self.adv(1, false);
                }
                b'}' if fresh && cmd_pos && kind == Kind::Brace && delimits(next) => {
                    self.end_pipeline();
                    let child = self.frames.pop().expect("a brace frame is open");
                    self.adv(1, false);
                    let f = self.top();
                    f.pipefail = child.pipefail;
                    f.cmd_pos = true;
                    self.mark_end();
                }
                _ => self.plain(1),
            }
        }
        if let Some(line) = self.top().dq {
            return self.err(line);
        }
        self.end_word();
        if self.frames.len() > 1 {
            let line = self.top().open_line;
            return self.err(line);
        }
        if let Some(d) = self.pending.first() {
            return self.err(d.line);
        }
        self.end_pipeline();
        self.end_logical(self.b.len());
        Ok(())
    }
}

// spec: gate-sdk/SPEC.md §The shell reader — a waiver is `# <token>: <reason>`; the reason is the
// trimmed text after the colon
pub fn waiver_reason<'a>(comment: &'a str, token: &str) -> Option<&'a str> {
    let rest = comment.strip_prefix('#')?.trim_start().strip_prefix(token)?;
    Some(rest.strip_prefix(':')?.trim())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Waived {
    No,
    Cleared,
    Short,
}

// spec: gate-sdk/SPEC.md §The shell reader — any waiver on the physical lines clears the site when
// its reason meets the floor; a waiver under the floor clears nothing and is itself a finding
pub fn waived(script: &Script, first: usize, last: usize, tokens: &[String], floor: usize) -> Waived {
    let mut state = Waived::No;
    for c in script.comments.iter().filter(|c| c.line >= first && c.line <= last) {
        for t in tokens {
            if let Some(reason) = waiver_reason(&c.text, t) {
                if reason.chars().count() >= floor {
                    return Waived::Cleared;
                }
                state = Waived::Short;
            }
        }
    }
    state
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edit {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub line: usize,
    pub reader: &'static str,
    pub fix: Option<Vec<Edit>>,
    pub short_waiver: bool,
}

// spec: gate-sdk/SPEC.md §The shell reader — the words before a stage's command name that are not it
const PREFIX: &[&str] = &["if", "then", "else", "elif", "do", "!", "time", "command"];

fn assignment(v: &str) -> bool {
    let name_end = v.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(v.len());
    name_end > 0
        && !v.as_bytes()[0].is_ascii_digit()
        && (v[name_end..].starts_with('=') || v[name_end..].starts_with("+=") || v[name_end..].starts_with('['))
        && v.contains('=')
}

fn command_name(args: &[&Word]) -> usize {
    args.iter().take_while(|w| !w.quoted && (PREFIX.contains(&w.value.as_str()) || assignment(&w.value))).count()
}

// spec: gate-sdk/SPEC.md §The shell reader — the early-exit detector over every pipeline under
// `pipefail`, a reader cleared when every stage before it is guarded and a pipeline by its waiver
pub fn early_exits(script: &Script, tokens: &[String], floor: usize) -> Vec<Hit> {
    let mut hits = Vec::new();
    for p in script.pipelines.iter().filter(|p| p.pipefail) {
        let state = waived(script, p.line, p.last, tokens, floor);
        if state == Waived::Cleared {
            continue;
        }
        for (i, stage) in p.stages.iter().enumerate().skip(1) {
            if p.stages[..i].iter().all(guarded) {
                continue;
            }
            if let Some((reader, fix)) = early_exit(script, stage) {
                hits.push(Hit { line: p.line, reader, fix, short_waiver: state == Waived::Short });
            }
        }
    }
    for body in &script.bodies {
        hits.extend(early_exits(body, tokens, floor));
    }
    hits
}

// spec: gate-sdk/SPEC.md §The shell reader — a guarded stage is one of the shape `{ <cmd> || true; }`
fn guarded(stage: &Stage) -> bool {
    let raw = stage.raw.trim();
    let Some(inner) = raw.strip_prefix('{').and_then(|r| r.strip_suffix('}')) else { return false };
    let inner = inner.trim();
    inner.strip_suffix(';').unwrap_or(inner).trim_end().ends_with("|| true")
}

pub fn early_exit(script: &Script, stage: &Stage) -> Option<(&'static str, Option<Vec<Edit>>)> {
    let ci = stage.cmd?;
    let cmd = &script.commands[ci];
    let args: Vec<&Word> = cmd.args().collect();
    if args.first().is_some_and(|w| !w.quoted && w.value == "while") {
        let k = 1 + args[1..].iter().take_while(|w| !w.quoted && assignment(&w.value)).count();
        let reads = args.get(k).is_some_and(|w| w.value == "read");
        return (reads && loop_breaks(script, ci)).then_some(("while-read break", None));
    }
    let k = command_name(&args);
    let head = args.get(k)?;
    let rest = &args[k + 1..];
    match head.value.as_str() {
        "grep" => grep(cmd, stage, rest),
        "head" => Some(("head", head_fix(cmd, head, rest))),
        "read" => Some(("read", None)),
        "sed" => sed_quits(rest).then_some(("sed q", None)),
        "awk" | "gawk" | "mawk" | "nawk" => awk_exits(rest).then_some(("awk exit", None)),
        _ => None,
    }
}

// spec: gate-sdk/SPEC.md §The shell reader — a `while read` loop stops early when a `break` in its
// own shell reaches it
fn loop_breaks(script: &Script, ci: usize) -> bool {
    let shell = script.commands[ci].shell;
    let mut depth = 1usize;
    for c in script.commands[ci + 1..].iter().filter(|c| c.shell == shell) {
        let args: Vec<&Word> = c.args().collect();
        let k = command_name(&args);
        let Some(first) = args.get(k) else { continue };
        match first.value.as_str() {
            "while" | "until" | "for" | "select" => depth += 1,
            "done" => {
                depth -= 1;
                if depth == 0 {
                    return false;
                }
            }
            "break" => {
                let n = args.get(k + 1).and_then(|w| w.value.parse::<usize>().ok()).unwrap_or(1);
                if n >= depth {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit())
}

fn unquoted(w: &Word) -> bool {
    !w.quoted && !w.expands && w.raw == w.value
}

fn positive(s: &str) -> bool {
    digits(s) && s.bytes().any(|c| c != b'0')
}

// spec: gate-sdk/SPEC.md §The shell reader — the grep flags under which the first N output lines
// are the first N matches, so a max-count moves to `sed -n` with the output kept
const COUNT_KEEPS_SHORT: &str = "EFGPiyvwxnbHhsaUTZlmq";
const COUNT_KEEPS_LONG: &[&str] = &[
    "extended-regexp",
    "fixed-strings",
    "basic-regexp",
    "perl-regexp",
    "ignore-case",
    "no-ignore-case",
    "invert-match",
    "word-regexp",
    "line-regexp",
    "line-number",
    "byte-offset",
    "with-filename",
    "no-filename",
    "no-messages",
    "text",
    "binary",
    "initial-tab",
    "null",
    "color",
    "colour",
    "label",
    "line-buffered",
    "regexp",
    "file",
    "max-count",
    "quiet",
    "silent",
    "files-with-matches",
];

// spec: gate-sdk/SPEC.md §The shell reader — `grep` with `-q`, `-m` or `-l`, long or in a short
// cluster; a lone quiet form, and a lone nonzero digit max-count over its input alone with no flag
// shaping the output around the count, are fixable
fn grep(cmd: &Command, stage: &Stage, rest: &[&Word]) -> Option<(&'static str, Option<Vec<Edit>>)> {
    let mut quiet: Vec<(usize, Option<String>)> = Vec::new();
    let mut count: Vec<(usize, Option<String>)> = Vec::new();
    let mut max: Option<String> = None;
    let mut list = false;
    let mut fixable = true;
    let mut shaped = false;
    let mut pattern = false;
    let mut operands = 0usize;
    let mut k = 0;
    while k < rest.len() {
        let v = rest[k].value.as_str();
        if v == "--" || !v.starts_with('-') || v == "-" {
            if v == "--" {
                operands += rest.len() - k - 1;
                break;
            }
            operands += 1;
            k += 1;
            continue;
        }
        fixable &= unquoted(rest[k]);
        if let Some(long) = v.strip_prefix("--") {
            let name = long.split('=').next().unwrap_or(long);
            shaped |= !COUNT_KEEPS_LONG.contains(&name);
            pattern |= matches!(name, "regexp" | "file");
            match long {
                "quiet" | "silent" => quiet.push((k, None)),
                "files-with-matches" => list = true,
                "max-count" => {
                    max = rest.get(k + 1).map(|w| w.value.clone());
                    count.push((k, None));
                    count.push((k + 1, None));
                    k += 1;
                }
                "regexp" | "file" => k += 1,
                _ if long.starts_with("max-count=") => {
                    max = Some(long["max-count=".len()..].to_string());
                    count.push((k, None));
                }
                _ => {}
            }
            k += 1;
            continue;
        }
        let flags = &v[1..];
        let mut kept = String::from("-");
        let mut has_q = false;
        let mut arg_next = false;
        for (j, ch) in flags.char_indices() {
            match ch {
                'q' => {
                    has_q = true;
                    continue;
                }
                'l' => list = true,
                'm' => {
                    let tail = &flags[j + 1..];
                    if tail.is_empty() {
                        max = rest.get(k + 1).map(|w| w.value.clone());
                        count.push((k + 1, None));
                        arg_next = true;
                    } else {
                        max = Some(tail.to_string());
                    }
                    count.push((k, (kept.len() > 1).then(|| kept.clone())));
                    break;
                }
                'e' | 'f' | 'A' | 'B' | 'C' | 'd' | 'D' => {
                    pattern |= matches!(ch, 'e' | 'f');
                    shaped |= matches!(ch, 'A' | 'B' | 'C');
                    arg_next = j + 1 == flags.len();
                    kept.push_str(&flags[j..]);
                    break;
                }
                _ => shaped |= !COUNT_KEEPS_SHORT.contains(ch),
            }
            kept.push(ch);
        }
        if has_q {
            quiet.push((k, (kept.len() > 1).then(|| kept.clone())));
        }
        k += 1 + usize::from(arg_next);
    }
    let reader = if !quiet.is_empty() {
        "grep -q"
    } else if max.is_some() {
        "grep -m"
    } else if list {
        "grep -l"
    } else {
        return None;
    };
    let words: Vec<&Word> = cmd.args().collect();
    let base = words.len() - rest.len();
    let prev_end = |k: usize| words[base + k - 1].end;
    let fix = || -> Option<Vec<Edit>> {
        if !fixable || list || (!quiet.is_empty() && max.is_some()) {
            return None;
        }
        let (removed, tail) = if quiet.is_empty() {
            if shaped || operands > usize::from(!pattern) {
                return None;
            }
            let n = max.as_deref().filter(|n| positive(n))?;
            (&count, Edit { start: stage.end, end: stage.end, text: format!(" | sed -n '1,{}p'", n) })
        } else {
            let gone: Vec<usize> = quiet.iter().filter(|q| q.1.is_none()).map(|q| q.0).collect();
            let keep_end = (0..words.len()).filter(|i| *i < base || !gone.contains(&(i - base))).map(|i| words[i].end).max()?;
            (&quiet, Edit { start: keep_end, end: keep_end, text: " >/dev/null".to_string() })
        };
        if removed.iter().any(|(k, _)| *k >= rest.len()) {
            return None;
        }
        let mut edits: Vec<Edit> = removed
            .iter()
            .map(|(k, keep)| match keep {
                Some(text) => Edit { start: rest[*k].start, end: rest[*k].end, text: text.clone() },
                None => Edit { start: prev_end(*k), end: rest[*k].end, text: String::new() },
            })
            .collect();
        edits.push(tail);
        edits.sort_by_key(|a| std::cmp::Reverse((a.start, a.end)));
        Some(edits)
    };
    Some((reader, fix()))
}

// spec: gate-sdk/SPEC.md §The shell reader — `head` with no operand, `-n N`, `-nN` or `-N`, `N`
// nonzero, is fixable, a bare `head` being `-n 10`
fn head_fix(cmd: &Command, head: &Word, rest: &[&Word]) -> Option<Vec<Edit>> {
    if !rest.iter().all(|w| unquoted(w)) {
        return None;
    }
    let vals: Vec<&str> = rest.iter().map(|w| w.value.as_str()).collect();
    let n = match vals.as_slice() {
        [] => "10",
        ["-n", n] if positive(n) => n,
        [one] if one.starts_with("-n") && positive(&one[2..]) => &one[2..],
        [one] if one.starts_with('-') && positive(&one[1..]) => &one[1..],
        _ => return None,
    };
    let end = rest.last().map_or(head.end, |w| w.end);
    if cmd.words.iter().any(|w| w.redir && w.start > head.start && w.start < end) {
        return None;
    }
    Some(vec![Edit { start: head.start, end, text: format!("sed -n '1,{}p'", n) }])
}

// spec: gate-sdk/SPEC.md §The shell reader — a `sed` script with a command ending in `q` or `Q`
fn sed_quits(rest: &[&Word]) -> bool {
    let mut scripts: Vec<&str> = Vec::new();
    let mut explicit = false;
    let mut first: Option<&str> = None;
    let mut k = 0;
    while k < rest.len() {
        let v = rest[k].value.as_str();
        if let Some(e) = v.strip_prefix("--expression=") {
            scripts.push(e);
            explicit = true;
        } else if v == "--expression" || v == "-e" {
            scripts.extend(rest.get(k + 1).map(|w| w.value.as_str()));
            explicit = true;
            k += 1;
        } else if v == "-f" || v == "--file" {
            explicit = true;
            k += 1;
        } else if v.starts_with('-') && v.len() > 1 && !v.starts_with("--") {
            if let Some(j) = v.find('e') {
                if j + 1 < v.len() {
                    scripts.push(&v[j + 1..]);
                } else {
                    scripts.extend(rest.get(k + 1).map(|w| w.value.as_str()));
                    k += 1;
                }
                explicit = true;
            }
        } else if !v.starts_with('-') && first.is_none() {
            first = Some(v);
        }
        k += 1;
    }
    if !explicit {
        scripts.extend(first);
    }
    scripts.iter().any(|s| {
        s.split([';', '\n']).any(|piece| {
            let p = piece.trim().trim_end_matches(['}', ' ', '\t']);
            let p = p.trim_end_matches(|c: char| c.is_ascii_digit()).trim_end();
            let Some(stem) = p.strip_suffix(['q', 'Q']) else { return false };
            !stem.ends_with(|c: char| c.is_ascii_alphabetic() || c == '\\')
        })
    })
}

// spec: gate-sdk/SPEC.md §The shell reader — an `awk` program holding the identifier `exit`
fn awk_exits(rest: &[&Word]) -> bool {
    let mut k = 0;
    while k < rest.len() {
        let v = rest[k].value.as_str();
        match v {
            "-f" | "--file" => return false,
            "-F" | "-v" | "--assign" | "--field-separator" => k += 2,
            "--" => {
                k += 1;
                break;
            }
            _ if v.starts_with('-') && v.len() > 1 => k += 1,
            _ => break,
        }
    }
    let Some(program) = rest.get(k) else { return false };
    let p = program.value.as_bytes();
    let ident = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    program.value.match_indices("exit").any(|(at, _)| {
        (at == 0 || !ident(p[at - 1])) && p.get(at + 4).map_or(true, |c| !ident(*c))
    })
}

// spec: gate-sdk/SPEC.md §check-pipefail-readers — the fix arm applies a file's edits back to front
pub fn apply(text: &str, edits: &[Edit]) -> String {
    let mut sorted: Vec<&Edit> = edits.iter().collect();
    sorted.sort_by_key(|a| std::cmp::Reverse((a.start, a.end)));
    let mut out = text.to_string();
    for e in sorted {
        out.replace_range(e.start..e.end, &e.text);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn script(text: &str) -> Script {
        read(text, &Options { body: &[], pipefail: false }).expect("reads")
    }

    fn lib(text: &str) -> Script {
        read(text, &Options { body: &[], pipefail: true }).expect("reads")
    }

    fn readers(text: &str) -> Vec<&'static str> {
        early_exits(&lib(text), &["pipefail-ok".to_string()], 1).iter().map(|h| h.reader).collect()
    }

    fn fixed(text: &str) -> String {
        let s = lib(text);
        let edits: Vec<Edit> =
            early_exits(&s, &[], 1).into_iter().flat_map(|h| h.fix.expect("fixable")).collect();
        apply(text, &edits)
    }

    #[test]
    fn pipefail_is_set_by_its_flag_word() {
        assert!(sets_pipefail("set -o pipefail"));
        assert!(sets_pipefail("set -euo pipefail"));
        assert!(!sets_pipefail("echo pipefail"));
        assert!(!sets_pipefail("set +o pipefail"));
    }

    // spec: gate-sdk/SPEC.md §The shell reader — continuations and trailing operators join physical
    // lines; the logical line keeps its first line's number
    #[test]
    fn continuations_join_into_one_logical_line() {
        let s = script("a \\\n  b |\n  c &&\n  d | e\nf | g\n");
        let spans: Vec<(usize, usize)> = s.pipelines.iter().map(|p| (p.line, p.last)).collect();
        assert_eq!(spans, [(1, 3), (4, 4), (5, 5)]);
    }

    // spec: gate-sdk/SPEC.md §The shell reader — a logical line's code half keeps quoted text and
    // leaves out its comments, continuations and joined here-doc bodies
    #[test]
    fn a_logical_line_reads_its_code_half() {
        let s = script("a \\\n  b 'x # y' | # c\n  d\ncat <<EOF |\nbody\nEOF\n  e\nf # g\n");
        let lines: Vec<(usize, usize, String)> =
            s.logicals.iter().map(|l| (l.first, l.last, s.code(l.start, l.end))).collect();
        assert_eq!(
            lines,
            [
                (1, 3, "a   b 'x # y' |    d".to_string()),
                (4, 7, "cat <<EOF |   e".to_string()),
                (8, 8, "f ".to_string()),
            ]
        );
        let d = &s.commands[1].words[0];
        assert_eq!(d.value, "d");
        let l = s.logical_at(d.start).expect("in a logical line");
        assert_eq!(s.code(l.start, d.start), "a   b 'x # y' |    ");
    }

    // spec: gate-sdk/SPEC.md §The shell reader — quoted text, here-doc bodies, case patterns and
    // test brackets hold no pipeline
    #[test]
    fn quoted_and_structural_bars_are_not_pipes() {
        let s = script(concat!(
            "echo \"a|b\" 'c|d'\n",
            "cat <<'EOF'\nx | head\nit's\nEOF\n",
            "case $x in\n  a|b) echo y ;;\n  *) : ;;\nesac\n",
            "[[ $x =~ ^(a|b)$ ]] && echo z\n",
            "arr=(\n  one # c\n  two\n)\n",
            "msg='multi\nline | head'\n",
        ));
        assert!(s.pipelines.is_empty(), "{:?}", s.pipelines);
        assert_eq!(s.comments.len(), 1);
    }

    // spec: gate-sdk/SPEC.md §The shell reader — scope runs from the set to the end of the frame
    // holding it, a subshell's set never leaking out and a group's shared
    #[test]
    fn pipefail_scope_follows_its_frame() {
        let s = script("a | head\nset -o pipefail\nb | head\n( set -o pipefail; c | head )\n");
        let on: Vec<bool> = s.pipelines.iter().map(|p| p.pipefail).collect();
        assert_eq!(on, [false, true, true]);
        let s = script("( set -o pipefail )\na | head\n{ set -euo pipefail; }\nb | head\nx=$(set -o pipefail; c | head)\nd | head\n");
        let on: Vec<bool> = s.pipelines.iter().map(|p| p.pipefail).collect();
        assert_eq!(on, [false, true, true, true]);
    }

    // spec: gate-sdk/SPEC.md §The shell reader — an unparseable span names its line
    #[test]
    fn an_unparseable_span_names_its_line() {
        let opts = Options { body: &[], pipefail: false };
        for (text, line) in [
            ("ok\necho 'open\n", 2),
            ("ok\necho \"open\n", 2),
            ("cat <<EOF\nbody\n", 1),
            ("a\n( b\n", 2),
            ("a )\n", 1),
            ("x=$(b\n", 1),
            ("{ a; ) \n", 1),
        ] {
            assert_eq!(read(text, &opts).err(), Some(ParseError { line }), "{:?}", text);
        }
    }

    // spec: gate-sdk/SPEC.md §The shell reader — the reader set, each stage after the first
    #[test]
    fn each_early_exit_reader_is_found() {
        assert_eq!(readers("a | grep -q x\n"), ["grep -q"]);
        assert_eq!(readers("a | grep -Fxq -- x\n"), ["grep -q"]);
        assert_eq!(readers("a | grep --max-count=1 x\n"), ["grep -m"]);
        assert_eq!(readers("a | grep -l x\n"), ["grep -l"]);
        assert_eq!(readers("a | head -1\n"), ["head"]);
        assert_eq!(readers("a | read x\n"), ["read"]);
        assert_eq!(readers("a | sed -n '/x/{p;q}'\n"), ["sed q"]);
        assert_eq!(readers("a | sed 1q\n"), ["sed q"]);
        assert_eq!(readers("a | awk '/x/ { print; exit }'\n"), ["awk exit"]);
        assert_eq!(readers("a | while read -r l; do\n  [[ $l ]] && break\ndone\n"), ["while-read break"]);
        assert_eq!(readers("if a | grep -q x; then :; fi\n"), ["grep -q"]);
        assert!(readers("a | grep -c x\n").is_empty());
        assert!(readers("a | sed 's/q/x/'\n").is_empty());
        assert!(readers("a | awk '{ print $1 }'\n").is_empty());
        assert!(readers("a | while read -r l; do echo \"$l\"; done\n").is_empty());
        assert!(readers("a | while read -r l; do for x in y; do break; done; done\n").is_empty());
        assert!(readers("grep -q x file\n").is_empty());
    }

    // spec: gate-sdk/SPEC.md §The shell reader — a reader with every stage before it guarded, and a
    // waived pipeline, are cleared; a waiver under the floor clears nothing
    #[test]
    fn guards_and_waivers_clear() {
        assert!(readers("{ a || true; } | head -1\n").is_empty());
        assert!(readers("{ a || true; } | { cat || true; } | head -1\n").is_empty());
        assert_eq!(readers("{ a || true; } | cat | head -1\n"), ["head"]);
        assert_eq!(readers("{ a || true; } | head -1 | grep -q x\n"), ["grep -q"]);
        assert!(readers("a | head -1 # pipefail-ok: reads one line by design\n").is_empty());
        assert!(readers("a |  # pipefail-ok: one line\n  head -1\n").is_empty());
        let s = lib("a | head -1 # pipefail-ok:\n");
        let hits = early_exits(&s, &["pipefail-ok".to_string()], 1);
        assert!(hits[0].short_waiver);
        assert!(script("a | head -1\n").pipelines.iter().all(|p| !p.pipefail));
    }

    // spec: gate-sdk/SPEC.md §check-pipefail-readers — the three mechanical rewrites
    #[test]
    fn fixable_hits_rewrite() {
        assert_eq!(fixed("a | grep -q x\n"), "a | grep x >/dev/null\n");
        assert_eq!(fixed("a | grep -Fq x 2>&1\n"), "a | grep -F x >/dev/null 2>&1\n");
        assert_eq!(fixed("a | grep -m 1 x\n"), "a | grep x | sed -n '1,1p'\n");
        assert_eq!(fixed("if a | grep -Em2 x; then\n"), "if a | grep -E x | sed -n '1,2p'; then\n");
        assert_eq!(fixed("a | head -n 3\n"), "a | sed -n '1,3p'\n");
        assert_eq!(fixed("v=$(a | head -1)\n"), "v=$(a | sed -n '1,1p')\n");
        assert_eq!(fixed("a | head\n"), "a | sed -n '1,10p'\n");
        assert_eq!(fixed("a | grep -nvm1 -e x --color=never\n"), "a | grep -nv -e x --color=never | sed -n '1,1p'\n");
        let s = lib("a | grep -qm1 x\na | head -c 4\na | grep -l x\n");
        assert!(early_exits(&s, &[], 1).iter().all(|h| h.fix.is_none()));
    }

    // spec: gate-sdk/SPEC.md §The shell reader — a count of zero, a flag shaping the output around
    // the count and a file operand are hits with no rewrite, since `sed -n` would change the output
    #[test]
    fn a_count_the_rewrite_would_change_is_no_fix() {
        let shapes = [
            "a | grep -m1 -A2 x\n",
            "a | grep -m1 -C 1 x\n",
            "a | grep -m1 --after-context=2 x\n",
            "a | grep -cm1 x\n",
            "a | grep -o -m1 x\n",
            "a | grep -m1 -z x\n",
            "a | grep -m1 -2 x\n",
            "a | grep -m1 x f1 f2\n",
            "a | grep -m1 -e x f1\n",
            "a | grep -m0 x\n",
            "a | grep --max-count=00 x\n",
            "a | head -n0\n",
            "a | head -n 0\n",
            "a | head -0\n",
        ];
        for text in shapes {
            let hits = early_exits(&lib(text), &[], 1);
            assert!(hits.len() == 1 && hits[0].fix.is_none(), "{}", text);
        }
    }

    // spec: gate-sdk/SPEC.md §The shell reader — a named command's here-doc and quoted operand are
    // scripts of their own, scope off and line numbers the outer file's
    #[test]
    fn a_named_body_is_read_as_its_own_script() {
        let body = ["bash".to_string()];
        let opts = Options { body: &body, pipefail: false };
        let s = read("set -o pipefail\nbash <<'EOF'\nx\na | head -1\nEOF\nbash -c 'b | grep -q y'\n", &opts).unwrap();
        assert_eq!(s.bodies.len(), 2);
        assert_eq!(s.bodies[0].pipelines[0].line, 4);
        assert!(!s.bodies[0].pipelines[0].pipefail);
        assert_eq!(s.bodies[1].pipelines[0].line, 6);
        let codes: Vec<String> = s.logicals.iter().map(|l| s.code(l.start, l.end)).collect();
        assert_eq!(codes, ["set -o pipefail", "bash <<'EOF'", "bash -c  "]);
    }
}
