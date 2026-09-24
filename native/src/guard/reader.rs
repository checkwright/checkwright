// spec: guard-kit/SPEC.md §The guard framework — the reader seam: everything the engine and the rules
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

pub trait Reader {
    // spec: guard-kit/SPEC.md §The guard framework — a view of `text`; `None` only for a dequoted view
    // the skeleton cannot be aligned with. A skeleton never ends in a newline.
    fn view(&self, text: &str, view: View) -> Option<String>;
    // spec: guard-kit/SPEC.md §The generic ruleset — the body of the `k`th heredoc, counted from one.
    fn body(&self, text: &str, k: usize) -> String;
    // spec: guard-kit/SPEC.md §The guard framework — the compound split, one segment per boundary.
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
    // spec: guard-kit/SPEC.md §The guard framework — a segment as the permission matcher reads it.
    fn harness_view<'a>(&self, seg: &'a str) -> &'a str;
}
