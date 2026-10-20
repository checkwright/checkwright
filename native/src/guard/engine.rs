// spec: guard-kit/SPEC.md §The generic ruleset — the engine: the rule table walked in dispatch order,
// each rule handed its declared views through a context and nothing else.
use super::host::Host;
use super::reader::{Reader, View, Word};
use super::rules::{literal_call, TABLE};

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

    // spec: guard-kit/SPEC.md §The reader and its views — the word read, built from the three views
    // a caller declares; it faults as an undeclared view does.
    pub fn word_read(&self, c: &Cmd) -> Result<Option<Vec<Vec<Word>>>, Fault> {
        for view in [View::Raw, View::SqDqHd, View::Dequoted] {
            self.declared(view)?;
        }
        Ok(self.reader.word_read(&c.0))
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

    pub fn heredoc_openers(&self, t: &str) -> Vec<String> {
        self.reader.heredoc_openers(t)
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
    decide_over(TABLE, reader, shell, host, cmd)
}

// spec: guard-kit/SPEC.md §The generic ruleset — a grant or a rewrite on a call the literal test
// refuses is a decline, and the walk goes on.
fn decide_over(table: &'static [Rule], reader: &dyn Reader, shell: Shell, host: &Host, cmd: &Cmd) -> Option<Verdict> {
    for rule in table.iter().filter(|r| r.shells.contains(&shell)) {
        let ctx = Ctx { reader, shell, rule, host, cmd };
        match (rule.test)(&ctx) {
            Ok(Some(Verdict::Allow(_) | Verdict::Rewrite(..))) if !literal_call(rule.name, &cmd.0) => {}
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

#[cfg(test)]
mod tests {
    use super::*;

    fn grant(_: &Ctx) -> Decided {
        Ok(Some(Verdict::Allow("stub".to_string())))
    }

    fn rewrite(_: &Ctx) -> Decided {
        Ok(Some(Verdict::Rewrite("ls".to_string(), "stub".to_string())))
    }

    fn steer(_: &Ctx) -> Decided {
        Ok(Some(Verdict::Block("stub".to_string())))
    }

    static GRANTS: &[Rule] = &[
        Rule { name: "stub_grant", shells: &[Shell::Bash], views: &[], test: grant },
        Rule { name: "stub_rewrite", shells: &[Shell::Bash], views: &[], test: rewrite },
    ];
    static STEERS: &[Rule] = &[
        Rule { name: "stub_grant", shells: &[Shell::Bash], views: &[], test: grant },
        Rule { name: "stub_steer", shells: &[Shell::Bash], views: &[], test: steer },
    ];

    // spec: guard-kit/SPEC.md §The generic ruleset — the engine holds the literal test over every
    // grant verdict, so a rule that omits it grants nothing outside it and a later rule still reads
    // the call.
    #[test]
    fn a_grant_on_a_call_the_literal_test_refuses_is_a_decline() {
        let host = Host::load(false).expect("the guard's knobs load");
        let reader = super::super::bash::Bash;
        let ask = |table: &'static [Rule], c: &str| decide_over(table, &reader, Shell::Bash, &host, &Cmd::new(c));
        assert_eq!(ask(GRANTS, "echo a >> .tmp/j.md"), Some(Verdict::Allow("stub".to_string())));
        for refused in [
            "echo \"a b\" >> .tmp/j.md",
            "echo 'a' >> .tmp/j.md",
            "echo a\\ b >> .tmp/j.md",
            "ls .tmp/o*",
            "ls .tmp/o?",
            "ls .tmp/[ab]",
            "ls ~/x",
            "echo $HOME",
            "echo `x`",
            "cat <<EOF\nx\nEOF",
            "cat <<<x",
            "ls\r",
            "(ls)",
        ] {
            assert_eq!(ask(GRANTS, refused), None, "{:?}", refused);
            assert_eq!(ask(STEERS, refused), Some(Verdict::Block("stub".to_string())), "{:?}", refused);
        }
        for literal in ["until [ -f .tmp/x ]; do sleep 5; done", "[[ -f .tmp/x ]]", "ls .tmp/a~ a=b"] {
            assert_eq!(ask(GRANTS, literal), Some(Verdict::Allow("stub".to_string())), "{:?}", literal);
        }
    }

    // spec: guard-kit/SPEC.md §The rule roster — rule `bounded_wait`'s arm (B) record statement is the
    // literal test's one exemption, byte for byte, and the rest of the call stays held to the test
    #[test]
    fn the_record_template_alone_is_set_aside_for_bounded_wait() {
        let launch = "git status > .tmp/k.log 2>&1 & echo \"pid=$! run=k\" > .tmp/k.run; wait; rm -f .tmp/k.run";
        assert!(literal_call("bounded_wait", launch));
        assert!(!literal_call("bounded_write", launch), "the exemption is rule `bounded_wait`'s alone");
        for off in [
            "git status > .tmp/k.log 2>&1 & echo  \"pid=$! run=k\" > .tmp/k.run; wait",
            "git status > .tmp/k.log 2>&1 & echo \"pid=$! run=k\" >.tmp/k.run; wait",
            "git \"status\" > .tmp/k.log 2>&1 & echo \"pid=$! run=k\" > .tmp/k.run; wait",
            "git status > .tmp/k.log 2>&1 & echo \"pid=$! run=k\" > .tmp/k.run; wait; rm -f .tmp/*.run",
        ] {
            assert!(!literal_call("bounded_wait", off), "{:?}", off);
        }
    }
}
