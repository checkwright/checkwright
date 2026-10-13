// spec: guard-kit/SPEC.md §Testing — the variant lane's deriver, its relation and its limits
// ledger; the deriver is a lexer of its own and shares no function with the bash reader.

pub const QUOTE_FORMS: &[&str] = &["sq", "dq", "esc", "part"];
pub const SITE_KINDS: &[&str] = &["lead", "arg"];
pub const SPACING: &[&str] = &["redirect-tight", "redirect-apart", "dup-apart", "word-glued", "separator-tight"];
pub const WRAP: &str = "wrap-command";

const RESERVED: &[&str] = &[
    "if", "then", "else", "elif", "fi", "for", "while", "until", "do", "done", "case", "esac", "in", "function",
    "select", "time", "coproc",
];
const NAME_TAKERS: &[&str] = &["for", "select", "case", "function"];
const STATEMENT_OPENERS: &[&str] = &["if", "then", "else", "elif", "while", "until", "do", "!", "time"];
const INELIGIBLE: &[&str] = &["'", "\"", "\\", "$", "`", "#", "(", ")", "{", "}", "[[", "<<", "@NL@"];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variant {
    pub transform: String,
    pub cell: String,
}

pub fn is_transform(name: &str) -> bool {
    name == WRAP
        || SPACING.contains(&name)
        || name
            .split_once('-')
            .is_some_and(|(form, kind)| QUOTE_FORMS.contains(&form) && SITE_KINDS.contains(&kind))
}

pub fn eligible(cell: &str) -> bool {
    !INELIGIBLE.iter().any(|mark| cell.contains(mark))
}

fn blank(b: u8) -> bool {
    b == b' ' || b == b'\t'
}

fn plain(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b"_./:%+,-".contains(&b)
}

fn operator_char(b: u8) -> bool {
    b";|&<>".contains(&b)
}

fn tokens(cell: &str) -> Vec<(usize, usize)> {
    let bytes = cell.as_bytes();
    let mut out = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if blank(bytes[at]) {
            at += 1;
            continue;
        }
        let start = at;
        while at < bytes.len() && !blank(bytes[at]) {
            at += 1;
        }
        out.push((start, at));
    }
    out
}

fn closes_statement(token: &str) -> bool {
    if token.ends_with(">&") || token.ends_with("<&") {
        return false;
    }
    token.ends_with(';') || token.ends_with('|') || token.ends_with('&')
}

struct Site {
    start: usize,
    len: usize,
    lead: bool,
}

fn sites(cell: &str) -> Vec<Site> {
    let toks = tokens(cell);
    let mut out = Vec::new();
    for (i, &(start, end)) in toks.iter().enumerate() {
        let token = &cell[start..end];
        let prev = i.checked_sub(1).map(|p| &cell[toks[p].0..toks[p].1]);
        let len = token.bytes().take_while(|&b| plain(b)).count();
        let (head, rest) = token.split_at(len);
        let descriptor = head.bytes().all(|b| b.is_ascii_digit()) && rest.starts_with(['<', '>']);
        if token.contains('@')
            || len < 2
            || RESERVED.contains(&head)
            || !rest.bytes().next().map_or(true, operator_char)
            || descriptor
            || prev.is_some_and(|p| NAME_TAKERS.contains(&p))
        {
            continue;
        }
        let lead = prev.map_or(true, |p| STATEMENT_OPENERS.contains(&p) || closes_statement(p));
        out.push(Site { start, len, lead });
    }
    out
}

fn spliced(cell: &str, from: usize, to: usize, with: &str) -> String {
    format!("{}{}{}", &cell[..from], with, &cell[to..])
}

fn quote_variants(cell: &str, out: &mut Vec<Variant>) {
    for site in sites(cell) {
        let head = &cell[site.start..site.start + site.len];
        let (front, back) = head.split_at(site.len / 2);
        let kind = if site.lead { "lead" } else { "arg" };
        for (form, text) in [
            ("sq", format!("'{}'", head)),
            ("dq", format!("\"{}\"", head)),
            ("esc", format!("\\{}", head)),
            ("part", format!("{}'{}'", front, back)),
        ] {
            out.push(Variant {
                transform: format!("{}-{}", form, kind),
                cell: spliced(cell, site.start, site.start + site.len, &text),
            });
        }
    }
}

fn blanks_before(bytes: &[u8], at: usize) -> usize {
    bytes[..at].iter().rev().take_while(|&&b| blank(b)).count()
}

fn blanks_after(bytes: &[u8], at: usize) -> usize {
    bytes[at..].iter().take_while(|&&b| blank(b)).count()
}

fn redirect_variants(cell: &str, out: &mut Vec<Variant>) {
    let bytes = cell.as_bytes();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] != b'<' && bytes[at] != b'>' {
            at += 1;
            continue;
        }
        let start = at;
        let mut end = at + 1;
        if bytes[start] == b'>' && bytes.get(end) == Some(&b'>') {
            end += 1;
        }
        at = end;
        if start > 0 && b"&<>".contains(&bytes[start - 1]) {
            continue;
        }
        let next = bytes.get(end).copied();
        if next == Some(b'&') {
            let digits = bytes[end + 1..].iter().take_while(|b| b.is_ascii_digit()).count();
            let after = bytes.get(end + 1 + digits).copied();
            if end == start + 1 && bytes[start] == b'>' && digits > 0 && after.map_or(true, |b| blank(b) || b";|&".contains(&b))
            {
                out.push(Variant { transform: "dup-apart".to_string(), cell: spliced(cell, end + 1, end + 1, " ") });
            }
            continue;
        }
        if next.is_some_and(operator_char) {
            continue;
        }
        let gap = blanks_after(bytes, end);
        if bytes.get(end + gap).is_some_and(|&b| !operator_char(b)) {
            let (name, with) = if gap > 0 { ("redirect-tight", "") } else { ("redirect-apart", " ") };
            out.push(Variant { transform: name.to_string(), cell: spliced(cell, end, end + gap, with) });
        }
        let lead = blanks_before(bytes, start);
        let word_end = start - lead;
        if lead > 0 && word_end > 0 && plain(bytes[word_end - 1]) && !bytes[word_end - 1].is_ascii_digit() {
            out.push(Variant { transform: "word-glued".to_string(), cell: spliced(cell, word_end, start, "") });
        }
    }
}

fn separator_variants(cell: &str, out: &mut Vec<Variant>) {
    let bytes = cell.as_bytes();
    let mut at = 0;
    while at < bytes.len() {
        let prev = at.checked_sub(1).map(|p| bytes[p]);
        let next = bytes.get(at + 1).copied();
        let len = match bytes[at] {
            b';' if next != Some(b';') && next != Some(b'&') && prev != Some(b';') => 1,
            b'&' if next == Some(b'&') && prev.map_or(true, |b| !b"&<>|".contains(&b)) => 2,
            b'|' if prev != Some(b'>') && prev != Some(b'|') => match next {
                Some(b'|') | Some(b'&') => 2,
                _ => 1,
            },
            _ => 0,
        };
        if len == 0 {
            at += 1;
            continue;
        }
        let (start, end) = (at, at + len);
        at = end;
        if bytes.get(end).is_some_and(|&b| b"&|;".contains(&b)) {
            continue;
        }
        let (lead, trail) = (blanks_before(bytes, start), blanks_after(bytes, end));
        if lead + trail > 0 && start > lead {
            out.push(Variant {
                transform: "separator-tight".to_string(),
                cell: format!("{}{}{}", &cell[..start - lead], &cell[start..end], &cell[end + trail..]),
            });
        }
    }
}

pub fn derive(cell: &str) -> Vec<Variant> {
    let mut out = Vec::new();
    if !eligible(cell) {
        return out;
    }
    quote_variants(cell, &mut out);
    redirect_variants(cell, &mut out);
    separator_variants(cell, &mut out);
    let first = tokens(cell).first().map(|t| t.0);
    if sites(cell).first().is_some_and(|s| Some(s.start) == first) {
        out.push(Variant { transform: WRAP.to_string(), cell: format!("command {}", cell) });
    }
    out
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Held {
    Holds,
    Limitable,
    Never,
}

pub fn relation(base: &str, got: &str) -> Held {
    let holds: &[&str] = match base {
        "advise" => &["advise", "block"],
        "fallthrough" => &["fallthrough", "block"],
        "allow" | "rewrite" => &["allow", "rewrite", "fallthrough", "block"],
        _ => &["block"],
    };
    if holds.contains(&got) {
        Held::Holds
    } else if granted(got) || got == "fallthrough" {
        Held::Limitable
    } else {
        Held::Never
    }
}

fn granted(got: &str) -> bool {
    got == "allow" || got == "rewrite"
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Admit {
    Admitted,
    NoLimit,
    BelowFloor,
}

struct Limit {
    grants: bool,
    rule: String,
    transform: String,
    failed: bool,
    granted: bool,
}

pub struct Ledger {
    limits: Vec<Limit>,
}

impl Ledger {
    pub fn parse(rows: &[Vec<String>], path: &str) -> Result<Ledger, String> {
        let mut limits: Vec<Limit> = Vec::new();
        for row in rows {
            let (floor, rule, transform) = (row[0].as_str(), row[1].as_str(), row[2].as_str());
            let grants = match floor {
                "allow" => true,
                "fallthrough" => false,
                _ => return Err(format!("{}: floor '{}' is neither 'fallthrough' nor 'allow'", path, floor)),
            };
            if !is_transform(transform) {
                return Err(format!("{}: '{}' (rule {}) names no transform of the variant lane", path, transform, rule));
            }
            if limits.iter().any(|l| l.rule == rule && l.transform == transform) {
                return Err(format!("{}: rule {} and transform {} are named by two rows", path, rule, transform));
            }
            limits.push(Limit {
                grants,
                rule: rule.to_string(),
                transform: transform.to_string(),
                failed: false,
                granted: false,
            });
        }
        Ok(Ledger { limits })
    }

    pub fn len(&self) -> usize {
        self.limits.len()
    }

    pub fn admit(&mut self, section: Option<&str>, transform: &str, got: &str) -> Admit {
        let Some(limit) = self
            .limits
            .iter_mut()
            .find(|l| Some(l.rule.as_str()) == section && l.transform == transform)
        else {
            return Admit::NoLimit;
        };
        limit.failed = true;
        limit.granted |= granted(got);
        if granted(got) && !limit.grants {
            Admit::BelowFloor
        } else {
            Admit::Admitted
        }
    }

    pub fn stale(&self) -> Vec<String> {
        let mut out = Vec::new();
        for l in &self.limits {
            if !l.failed {
                out.push(format!(
                    "limit row '{} {}' admits a re-spelling that no longer fails: delete the row",
                    l.rule, l.transform
                ));
            } else if l.grants && !l.granted {
                out.push(format!(
                    "limit row '{} {}' has floor 'allow' and no failing re-spelling under it is granted: tighten it to 'fallthrough'",
                    l.rule, l.transform
                ));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(cell: &str, transform: &str) -> Vec<String> {
        derive(cell).into_iter().filter(|v| v.transform == transform).map(|v| v.cell).collect()
    }

    #[test]
    fn each_quote_form_respells_a_lead_and_an_arg_site() {
        assert_eq!(cells("cd deploy", "sq-lead"), ["'cd' deploy"]);
        assert_eq!(cells("cd deploy", "dq-lead"), ["\"cd\" deploy"]);
        assert_eq!(cells("cd deploy", "esc-lead"), ["\\cd deploy"]);
        assert_eq!(cells("cd deploy", "part-lead"), ["c'd' deploy"]);
        assert_eq!(cells("cd deploy", "sq-arg"), ["cd 'deploy'"]);
        assert_eq!(cells("cd deploy", "dq-arg"), ["cd \"deploy\""]);
        assert_eq!(cells("cd deploy", "esc-arg"), ["cd \\deploy"]);
        assert_eq!(cells("cd deploy", "part-arg"), ["cd dep'loy'"]);
        assert_eq!(cells("git stash", "part-arg"), ["git st'ash'"], "the cut is half the length, rounded down");
    }

    #[test]
    fn a_site_is_a_lead_where_its_token_opens_a_statement() {
        assert_eq!(cells("cd deploy && ls", "sq-lead"), ["'cd' deploy && ls", "cd deploy && 'ls'"]);
        assert_eq!(cells("ls; cat notes.md", "sq-lead"), ["'ls'; cat notes.md", "ls; 'cat' notes.md"]);
        assert_eq!(cells("grep -q x f | wc -l", "sq-lead").len(), 2);
        assert_eq!(cells("sleep 60 & echo hi", "sq-lead").len(), 2);
        assert_eq!(
            cells("until test -f x.run; do sleep 5; done", "sq-lead"),
            ["until 'test' -f x.run; do sleep 5; done", "until test -f x.run; do 'sleep' 5; done"]
        );
        assert_eq!(cells("! grep -q x f", "sq-lead"), ["! 'grep' -q x f"]);
        assert_eq!(cells("ls >& out.txt", "sq-lead"), ["'ls' >& out.txt"], "a dup operator closes no statement");
        assert_eq!(cells("ls >& out.txt", "sq-arg"), ["ls >& 'out.txt'"]);
    }

    #[test]
    fn a_head_keeps_the_separator_or_redirect_glued_after_it() {
        assert_eq!(cells("ls; pwd", "sq-lead"), ["'ls'; pwd", "ls; 'pwd'"]);
        assert_eq!(cells("echo hi>out.txt", "dq-arg"), ["echo \"hi\">out.txt"]);
    }

    #[test]
    fn what_is_no_site() {
        assert!(cells("for item in aa bb; do ls; done", "sq-arg").iter().all(|c| !c.contains("'item'")), "a name after for");
        assert!(derive("if ls; then pwd; fi").iter().all(|v| !v.cell.contains("'if'") && !v.cell.contains("'then'")));
        assert_eq!(cells("ls a", "sq-arg"), Vec::<String>::new(), "a one-character head");
        assert_eq!(cells("ls 22>out.txt", "sq-arg"), Vec::<String>::new(), "a file descriptor");
        assert_eq!(cells("ls 22;", "sq-arg"), ["ls '22';"]);
        assert_eq!(cells("ls notes*.md", "sq-arg"), Vec::<String>::new(), "a head with a glob after it");
        assert_eq!(cells("FOO=bar", "sq-lead"), Vec::<String>::new(), "an assignment");
        assert_eq!(cells("cat @ROOT@/notes.md", "sq-arg"), Vec::<String>::new(), "a word carrying a substitution token");
        assert_eq!(cells("ls;@ROOT@/x", "sq-lead"), Vec::<String>::new());
    }

    #[test]
    fn an_ineligible_row_derives_nothing() {
        for cell in [
            "echo 'a b'", "echo \"a\"", "echo a\\ b", "echo $HOME", "echo `pwd`", "ls # note", "(ls)", "ls {a,b}",
            "[[ -f x ]]", "cat <<EOF@NL@x@NL@EOF", "printf x@NL@ls",
        ] {
            assert!(!eligible(cell), "{}", cell);
            assert!(derive(cell).is_empty(), "{}", cell);
        }
    }

    #[test]
    fn the_spacing_transforms_each_respell_one_occurrence() {
        assert_eq!(cells("ls > out.txt 2> err.txt", "redirect-tight"), ["ls >out.txt 2> err.txt", "ls > out.txt 2>err.txt"]);
        assert_eq!(cells("ls >>out.txt <in.txt", "redirect-apart"), ["ls >> out.txt <in.txt", "ls >>out.txt < in.txt"]);
        assert_eq!(cells("ls >out.txt 2>&1", "dup-apart"), ["ls >out.txt 2>& 1"]);
        assert_eq!(cells("ls 2>&1 | wc -l", "redirect-apart"), Vec::<String>::new(), "a dup is no redirect operator");
        assert_eq!(cells("ls &>out.txt", "redirect-apart"), Vec::<String>::new());
        assert_eq!(cells("ls >| out.txt", "redirect-tight"), Vec::<String>::new());
        assert_eq!(cells("echo hi > out.txt", "word-glued"), ["echo hi> out.txt"]);
        assert_eq!(cells("echo 5 > out.txt", "word-glued"), Vec::<String>::new(), "a digit would become a descriptor");
        assert_eq!(cells("echo hi 2> out.txt", "word-glued"), Vec::<String>::new(), "a descriptor would join the word");
        assert_eq!(cells("ls ; > out.txt", "word-glued"), Vec::<String>::new(), "a separator is no word");
        assert_eq!(
            cells("ls ; pwd && id || ls | wc |& cat", "separator-tight"),
            [
                "ls;pwd && id || ls | wc |& cat",
                "ls ; pwd&&id || ls | wc |& cat",
                "ls ; pwd && id||ls | wc |& cat",
                "ls ; pwd && id || ls|wc |& cat",
                "ls ; pwd && id || ls | wc|&cat",
            ]
        );
        assert_eq!(cells("ls;pwd", "separator-tight"), Vec::<String>::new());
        assert_eq!(cells("sleep 5 & ls", "separator-tight"), Vec::<String>::new(), "a lone & is not in the set");
    }

    #[test]
    fn command_leads_a_row_whose_first_token_is_a_lead_site() {
        assert_eq!(cells("cd deploy && ls", WRAP), ["command cd deploy && ls"]);
        assert_eq!(cells("! ls", WRAP), Vec::<String>::new());
        assert_eq!(cells("if ls; then pwd; fi", WRAP), Vec::<String>::new());
        assert_eq!(cells("FOO=bar ls", WRAP), Vec::<String>::new());
    }

    #[test]
    fn every_derived_name_is_a_transform() {
        for v in derive("cd deploy && echo hi > out.txt 2>&1 ; ls >err.txt") {
            assert!(is_transform(&v.transform), "{}", v.transform);
        }
        assert!(!is_transform("sq"));
        assert!(!is_transform("sq-word"));
        assert!(!is_transform("wrap"));
    }

    #[test]
    fn the_relation_is_one_sided() {
        use Held::*;
        for (base, got, want) in [
            ("block", "block", Holds),
            ("block", "fallthrough", Limitable),
            ("block", "allow", Limitable),
            ("block", "advise", Never),
            ("advise", "advise", Holds),
            ("advise", "block", Holds),
            ("advise", "fallthrough", Limitable),
            ("fallthrough", "fallthrough", Holds),
            ("fallthrough", "block", Holds),
            ("fallthrough", "allow", Limitable),
            ("fallthrough", "rewrite", Limitable),
            ("fallthrough", "advise", Never),
            ("allow", "fallthrough", Holds),
            ("allow", "block", Holds),
            ("allow", "rewrite", Holds),
            ("rewrite", "allow", Holds),
            ("rewrite", "advise", Never),
            ("allow", "exit3", Never),
            ("allow", "unknown", Never),
        ] {
            assert_eq!(relation(base, got), want, "{} -> {}", base, got);
        }
    }

    fn row(floor: &str, rule: &str, transform: &str) -> Vec<String> {
        vec![floor.to_string(), rule.to_string(), transform.to_string()]
    }

    #[test]
    fn a_limit_admits_up_to_its_floor_under_its_own_section_and_transform() {
        let mut ledger =
            Ledger::parse(&[row("fallthrough", "cd_compound", "sq-lead"), row("allow", "cat_file", "esc-arg")], "t").unwrap();
        assert_eq!(ledger.admit(Some("cd_compound"), "sq-lead", "fallthrough"), Admit::Admitted);
        assert_eq!(ledger.admit(Some("cd_compound"), "sq-lead", "allow"), Admit::BelowFloor);
        assert_eq!(ledger.admit(Some("cd_compound"), "dq-lead", "fallthrough"), Admit::NoLimit);
        assert_eq!(ledger.admit(None, "sq-lead", "fallthrough"), Admit::NoLimit);
        assert_eq!(ledger.admit(Some("cat_file"), "esc-arg", "allow"), Admit::Admitted);
        assert_eq!(ledger.admit(Some("cat_file"), "esc-arg", "fallthrough"), Admit::Admitted);
        assert!(ledger.stale().is_empty());
    }

    #[test]
    fn a_limit_nothing_needs_and_an_allow_floor_nothing_reaches_are_stale() {
        let mut ledger =
            Ledger::parse(&[row("fallthrough", "cd_compound", "sq-lead"), row("allow", "cat_file", "esc-arg")], "t").unwrap();
        assert_eq!(ledger.stale().len(), 2);
        ledger.admit(Some("cat_file"), "esc-arg", "fallthrough");
        let stale = ledger.stale();
        assert!(stale[0].contains("delete the row"), "{:?}", stale);
        assert!(stale[1].contains("tighten it"), "{:?}", stale);
    }

    #[test]
    fn a_table_the_lane_cannot_read_is_refused() {
        assert!(Ledger::parse(&[row("block", "cd_compound", "sq-lead")], "t").is_err());
        assert!(Ledger::parse(&[row("allow", "cd_compound", "sq")], "t").is_err());
        assert!(Ledger::parse(&[row("allow", "cd_compound", "sq-lead"), row("fallthrough", "cd_compound", "sq-lead")], "t").is_err());
    }
}
