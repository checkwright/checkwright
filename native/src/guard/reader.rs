// spec: guard-kit/SPEC.md §The shell guard — the reader seam: everything the engine and the rules
// read of a command comes through this interface, one implementation per shell.

// spec: guard-kit/SPEC.md §The generic ruleset — a view: the command as received, a skeleton named by
// its inert classes, the dequoted view, or a heredoc body.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum View {
    Raw,
    Sq,
    Hdq,
    SqHdq,
    SqDqHd,
    SqDqHdq,
    Dequoted,
    Body,
}

impl View {
    pub const ALL: [View; 8] = [
        View::Raw,
        View::Sq,
        View::Hdq,
        View::SqHdq,
        View::SqDqHd,
        View::SqDqHdq,
        View::Dequoted,
        View::Body,
    ];

    // spec: guard-kit/SPEC.md §The generic ruleset — the view a declaration's spelling names.
    pub fn from_spelling(s: &str) -> Option<View> {
        View::ALL.into_iter().find(|v| v.spelling() == s)
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — a view spelled as a declaration writes it.
    pub fn spelling(self) -> &'static str {
        match self {
            View::Raw => "raw",
            View::Sq => "sq",
            View::Hdq => "hdq",
            View::SqHdq => "sq hdq",
            View::SqDqHd => "sq dq hd",
            View::SqDqHdq => "sq dq hdq",
            View::Dequoted => "dequoted",
            View::Body => "body",
        }
    }
}

// spec: guard-kit/SPEC.md §The reader and its views — the skeleton's placeholders: NUL, then the
// class's two letters. Every reader emits these and every rule tests these, never the bare letters.
pub const SQ_MARK: &str = "\0SQ";
pub const DQ_MARK: &str = "\0DQ";
pub const HD_MARK: &str = "\0HD";

// spec: guard-kit/SPEC.md §The reader and its views — a skeleton-derived string as it leaves the
// process: each placeholder prints as its two letters.
pub fn unmark(s: &str) -> String {
    s.replace('\0', "")
}

// spec: guard-kit/SPEC.md §The shell guard — the first C0 control byte a command carries other than
// tab, line feed and carriage return; such a command is blocked before any rule reads it.
pub fn control_byte(s: &str) -> Option<u8> {
    s.bytes().find(|&c| c < 0x20 && !matches!(c, b'\t' | b'\n' | b'\r'))
}

// spec: guard-kit/SPEC.md §The reader and its views — the word read's entry: one word as the command
// spells it, as the shell passes it, and as the skeleton holds it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Word {
    pub spelling: String,
    pub value: String,
    pub live: String,
    seams: Vec<(usize, usize, usize)>,
}

impl Word {
    pub fn new(spelling: &str, value: &str, live: &str) -> Word {
        Word { spelling: spelling.to_string(), value: value.to_string(), live: live.to_string(), seams: Vec::new() }
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the value of what a word holds ahead of
    // the live offset `at`; `None` where `at` falls inside a piece.
    pub fn value_before(&self, at: usize) -> Option<&str> {
        if self.value == self.live {
            return self.value.get(..at);
        }
        if at == 0 {
            return Some("");
        }
        self.seams.iter().rev().find(|&&(live, _, _)| live == at).and_then(|&(_, value, _)| self.value.get(..value))
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the spelling of what a word holds ahead of
    // the live offset `at`; `None` where `at` falls inside a piece.
    pub fn spelling_before(&self, at: usize) -> Option<&str> {
        if self.spelling == self.live {
            return self.spelling.get(..at);
        }
        if at == 0 {
            return Some("");
        }
        self.seams.iter().rev().find(|&&(live, _, _)| live == at).and_then(|&(_, _, s)| self.spelling.get(..s))
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the value of what a word holds from the
    // live offset `at` on; `None` where `at` falls inside a piece.
    pub fn value_after(&self, at: usize) -> Option<&str> {
        let before = self.value_before(at)?;
        self.value.get(before.len()..)
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the word a word holds between the live
    // offsets `from` and `to`, its seams kept; `None` where either falls inside a piece.
    pub fn slice(&self, from: usize, to: usize) -> Option<Word> {
        let ends = |at: usize| {
            if at == self.live.len() {
                return Some((self.value.len(), self.spelling.len()));
            }
            Some((self.value_before(at)?.len(), self.spelling_before(at)?.len()))
        };
        let ((v0, s0), (v1, s1)) = (ends(from)?, ends(to)?);
        let seams = self
            .seams
            .iter()
            .filter(|s| s.0 > from && s.0 <= to)
            .map(|s| (s.0 - from, s.1.saturating_sub(v0), s.2.saturating_sub(s0)));
        Some(Word {
            spelling: self.spelling.get(s0..s1)?.to_string(),
            value: self.value.get(v0..v1)?.to_string(),
            live: self.live.get(from..to)?.to_string(),
            seams: seams.collect(),
        })
    }
}

// spec: guard-kit/SPEC.md §The reader and its views — the word read's accumulator: a reader feeds it
// the pieces of a word, face by face, and closes words and segments where its grammar does.
pub struct WordAcc {
    segs: Vec<Vec<Word>>,
    open: Option<[Vec<u8>; 3]>,
    seams: Vec<(usize, usize, usize)>,
}

impl WordAcc {
    pub fn new() -> WordAcc {
        WordAcc { segs: vec![Vec::new()], open: None, seams: Vec::new() }
    }

    pub fn piece(&mut self, spelling: &[u8], value: &[u8], live: &[u8]) {
        let w = self.open.get_or_insert_with(Default::default);
        w[0].extend_from_slice(spelling);
        w[1].extend_from_slice(value);
        w[2].extend_from_slice(live);
        self.seams.push((w[2].len(), w[1].len(), w[0].len()));
    }

    pub fn end_word(&mut self) {
        if let Some([s, v, l]) = self.open.take() {
            let text = |b: Vec<u8>| String::from_utf8_lossy(&b).into_owned();
            let seams = std::mem::take(&mut self.seams);
            let word = Word { spelling: text(s), value: text(v), live: text(l), seams };
            self.segs.last_mut().expect("one segment is always open").push(word);
        }
    }

    pub fn end_segment(&mut self) {
        self.end_word();
        self.segs.push(Vec::new());
    }

    pub fn finish(mut self) -> Vec<Vec<Word>> {
        self.end_word();
        self.segs
    }
}

pub trait Reader {
    // spec: guard-kit/SPEC.md §The shell guard — a view of `text`; `None` only for a dequoted view
    // the skeleton cannot be aligned with. A skeleton never ends in a newline.
    fn view(&self, text: &str, view: View) -> Option<String>;
    // spec: guard-kit/SPEC.md §The generic ruleset — the body of the `k`th heredoc, counted from one.
    fn body(&self, text: &str, k: usize) -> String;
    // spec: guard-kit/SPEC.md §The shell guard — the compound split, one segment per boundary.
    fn segments(&self, text: &str) -> Vec<String>;
    // spec: guard-kit/SPEC.md §The generic ruleset — statements, split where a pipe is not a boundary.
    fn statements(&self, text: &str) -> Vec<String>;
    // spec: guard-kit/SPEC.md §The generic ruleset — statements each carrying the heredoc residue its
    // own openers produce.
    fn residue_statements(&self, text: &str) -> Vec<String>;
    // spec: guard-kit/SPEC.md §The generic ruleset — one statement's pipeline members.
    fn pipes(&self, text: &str) -> Vec<String>;
    // spec: guard-kit/SPEC.md §The generic ruleset — every redirect, operator and target together.
    fn redirect_pairs(&self, text: &str) -> Vec<String>;
    // spec: guard-kit/SPEC.md §The generic ruleset — the terminator of each heredoc a line opens.
    fn heredoc_terms(&self, text: &str) -> Vec<String>;
    // spec: guard-kit/SPEC.md §The generic ruleset — each heredoc opener a line carries, as spelled.
    fn heredoc_openers(&self, text: &str) -> Vec<String>;
    // spec: guard-kit/SPEC.md §The shell guard — a segment as the permission matcher reads it.
    fn harness_view<'a>(&self, seg: &'a str) -> &'a str;
    // spec: guard-kit/SPEC.md §The reader and its views — the word read: each compound-split segment's
    // words; `None` where the dequoted view cannot be aligned outside a heredoc, or where the walk and
    // the compound split of the skeleton part.
    fn word_read(&self, text: &str) -> Option<Vec<Vec<Word>>>;
}
