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
    // spec: guard-kit/SPEC.md §The shell guard — a segment as the permission matcher reads it.
    fn harness_view<'a>(&self, seg: &'a str) -> &'a str;
}
