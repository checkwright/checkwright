// spec: guard-kit/SPEC.md §The generic ruleset — the engine: the rule table walked in dispatch order,
// each rule handed its declared views through a context and nothing else.
use super::host::Host;
use super::reader::{Reader, View};
use super::rules::TABLE;

// spec: guard-kit/SPEC.md §The shell guard — the four decisions a rule can take; each renders as
// the envelope the library printed.
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Block(String),
    Advise(String),
    Allow(String),
    Rewrite(String, String),
}

// spec: guard-kit/SPEC.md §The generic ruleset — a rule asked for a view its row does not declare.
#[derive(Debug, PartialEq, Eq)]
pub struct Fault {
    pub rule: &'static str,
    pub view: View,
}

pub type Decided = Result<Option<Verdict>, Fault>;

// spec: guard-kit/SPEC.md §The generic ruleset — the shells a rule applies to, spelled as an item's
// shells clause writes them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shell {
    Bash,
    PowerShell,
}

impl Shell {
    pub const ALL: [Shell; 2] = [Shell::Bash, Shell::PowerShell];

    pub fn spelling(self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::PowerShell => "powershell",
        }
    }

    pub fn from_spelling(s: &str) -> Option<Shell> {
        Shell::ALL.into_iter().find(|sh| sh.spelling() == s)
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — one row: the rule's name, the shells it applies to,
// the views it declares, and its test.
pub struct Rule {
    pub name: &'static str,
    pub shells: &'static [Shell],
    pub views: &'static [View],
    pub test: fn(&Ctx) -> Decided,
}

// spec: guard-kit/SPEC.md §The generic ruleset — a command a rule holds but cannot read except
// through its context: the text is private to this module.
pub struct Cmd(String);

impl Cmd {
    pub fn new(s: impl Into<String>) -> Cmd {
        Cmd(s.into())
    }
}

pub struct Ctx<'a> {
    reader: &'a dyn Reader,
    shell: Shell,
    rule: &'static Rule,
    host: &'a Host,
    cmd: &'a Cmd,
}

impl<'a> Ctx<'a> {
    pub fn cmd(&self) -> &'a Cmd {
        self.cmd
    }

    pub fn host(&self) -> &'a Host {
        self.host
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the shell `decide` was called with, which an
    // applied rule reads where its test differs by shell.
    pub fn shell(&self) -> Shell {
        self.shell
    }

    fn declared(&self, view: View) -> Result<(), Fault> {
        if self.rule.views.contains(&view) {
            Ok(())
        } else {
            Err(Fault { rule: self.rule.name, view })
        }
    }

    pub fn raw<'c>(&self, c: &'c Cmd) -> Result<&'c str, Fault> {
        self.declared(View::Raw)?;
        Ok(&c.0)
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — a skeleton view, never ending in a newline.
    pub fn view(&self, c: &Cmd, view: View) -> Result<String, Fault> {
        self.declared(view)?;
        Ok(self.reader.view(&c.0, view).unwrap_or_default())
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — a raw-command expansion decline, answered by the
    // reader: bash's own test on the raw command, or under PowerShell any `$` the `sq hdq` view keeps.
    pub fn expands(&self, c: &Cmd, bash: fn(&str) -> bool) -> Result<bool, Fault> {
        self.declared(View::Raw)?;
        Ok(match self.shell {
            Shell::Bash => bash(&c.0),
            Shell::PowerShell => self.reader.view(&c.0, View::SqHdq).unwrap_or_default().contains('$'),
        })
    }

    pub fn dequoted(&self, c: &Cmd) -> Result<Option<String>, Fault> {
        self.declared(View::Dequoted)?;
        Ok(self.reader.view(&c.0, View::Dequoted))
    }

    pub fn body(&self, c: &Cmd, k: usize) -> Result<String, Fault> {
        self.declared(View::Body)?;
        Ok(self.reader.body(&c.0, k))
    }

    pub fn segments(&self, t: &str) -> Vec<String> {
        self.reader.segments(t)
    }

    pub fn statements(&self, t: &str) -> Vec<String> {
        self.reader.statements(t)
    }

    pub fn residue_statements(&self, t: &str) -> Vec<String> {
        self.reader.residue_statements(t)
    }

    pub fn pipes(&self, t: &str) -> Vec<String> {
        self.reader.pipes(t)
    }

    pub fn redirect_pairs(&self, t: &str) -> Vec<String> {
        self.reader.redirect_pairs(t)
    }

    pub fn heredoc_terms(&self, t: &str) -> Vec<String> {
        self.reader.heredoc_terms(t)
    }

    pub fn harness_view<'t>(&self, seg: &'t str) -> &'t str {
        self.reader.harness_view(seg)
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — another rule's test taken as a predicate runs
    // under that rule's own declared views.
    pub fn under<T>(&self, rule: &'static str, f: impl FnOnce(&Ctx) -> Result<T, Fault>) -> Result<T, Fault> {
        let row = row(rule);
        f(&Ctx { reader: self.reader, shell: self.shell, rule: row, host: self.host, cmd: self.cmd })
    }
}

pub fn row(name: &str) -> &'static Rule {
    match TABLE.iter().find(|r| r.name == name) {
        Some(r) => r,
        None => panic!("no rule named {} in the rule table", name),
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — the first rule that decides ends the call; a view
// fault decides nothing and says so loudly.
pub fn decide(reader: &dyn Reader, shell: Shell, host: &Host, cmd: &Cmd) -> Option<Verdict> {
    for rule in TABLE.iter().filter(|r| r.shells.contains(&shell)) {
        let ctx = Ctx { reader, shell, rule, host, cmd };
        match (rule.test)(&ctx) {
            Ok(Some(v)) => return Some(v),
            Ok(None) => {}
            Err(f) => return Some(Verdict::Advise(fault_message(&f))),
        }
    }
    None
}

pub fn fault_message(f: &Fault) -> String {
    format!(
        "shell-guard: internal fault — rule `{}` read the `{}` view, which its row in the rule table does not declare, so the guard did not decide this call and it takes the harness's own permission path. Fix the rule or its declaration.",
        f.rule,
        f.view.spelling()
    )
}
