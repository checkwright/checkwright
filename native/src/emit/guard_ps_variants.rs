// spec: guard-kit/SPEC.md §Testing — the variant lane's PowerShell deriver, a second lexer of its
// own that shares no function with the PowerShell reader.
use super::guard_variants::Variant;

pub const TRANSFORMS: &[&str] =
    &["ps-sq-arg", "ps-dq-arg", "ps-tick-lead", "ps-tick-arg", "ps-case-lead", "ps-alias-lead"];

const INELIGIBLE: &[char] = &['\'', '"', '`', '$', '#', '@', '(', ')', '{', '}', '<'];
const STATEMENT_ENDS: &[&str] = &[";", "|", "&&", "||"];
const CONTROL_LETTERS: &[u8] = b"abefnrtuv";

// spec: guard-kit/SPEC.md §Testing — the alias table is the deriver's, a cmdlet against its
// built-in aliases; an entry with no alias is legal, and the crate test holds the roster to the
// tables.
const ALIASES: &[(&str, &[&str])] = &[
    ("Get-ChildItem", &["dir", "gci", "ls"]),
    ("Get-Content", &["cat", "gc", "type"]),
    ("Remove-Item", &["del", "erase", "rd", "ri", "rm", "rmdir"]),
    ("Select-Object", &["select"]),
    ("Set-Content", &["sc"]),
    ("Set-Location", &["cd", "chdir", "sl"]),
    ("Start-Job", &["sajb"]),
    ("Start-Process", &["saps", "start"]),
    ("Start-Sleep", &["sleep"]),
    ("Write-Output", &["echo"]),
];

pub fn eligible(cell: &str) -> bool {
    !cell.contains(INELIGIBLE)
}

fn word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b"_./\\:-".contains(&b)
}

struct Site<'a> {
    start: usize,
    text: &'a str,
    lead: bool,
}

fn sites(cell: &str) -> Vec<Site<'_>> {
    let mut out = Vec::new();
    let mut prev: Option<&str> = None;
    let mut at = 0;
    for token in cell.split([' ', '\t']) {
        let start = at;
        at += token.len() + 1;
        if token.is_empty() {
            continue;
        }
        if token.len() >= 2 && !token.starts_with('-') && token.bytes().all(word) {
            let lead = prev.map_or(true, |p| STATEMENT_ENDS.iter().any(|end| p.ends_with(end)));
            out.push(Site { start, text: token, lead });
        }
        prev = Some(token);
    }
    out
}

fn verb_noun(text: &str) -> bool {
    let mut parts = text.split('-');
    let alphabetic = |part: Option<&str>| part.is_some_and(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_alphabetic()));
    alphabetic(parts.next()) && alphabetic(parts.next()) && parts.next().is_none()
}

fn aliases(text: &str) -> Option<&'static [&'static str]> {
    ALIASES.iter().find(|(cmdlet, _)| cmdlet.eq_ignore_ascii_case(text)).map(|(_, names)| *names)
}

fn ticked(text: &str) -> Option<String> {
    let at = (text.len() / 2..text.len()).find(|&i| {
        let b = text.as_bytes()[i];
        b.is_ascii_alphabetic() && !CONTROL_LETTERS.contains(&b.to_ascii_lowercase())
    })?;
    Some(format!("{}`{}", &text[..at], &text[at..]))
}

pub fn derive(cell: &str) -> Vec<Variant> {
    let mut out = Vec::new();
    if !eligible(cell) {
        return out;
    }
    for site in sites(cell) {
        let kind = if site.lead { "lead" } else { "arg" };
        let mut push = |transform: String, with: String| {
            out.push(Variant {
                transform,
                cell: format!("{}{}{}", &cell[..site.start], with, &cell[site.start + site.text.len()..]),
            });
        };
        if !site.lead {
            push("ps-sq-arg".to_string(), format!("'{}'", site.text));
            push("ps-dq-arg".to_string(), format!("\"{}\"", site.text));
        }
        if let Some(with) = ticked(site.text) {
            push(format!("ps-tick-{}", kind), with);
        }
        if site.lead && verb_noun(site.text) {
            let upper = site.text.to_ascii_uppercase();
            if upper != site.text {
                push("ps-case-lead".to_string(), upper);
            }
            for alias in aliases(site.text).unwrap_or(&[]) {
                push("ps-alias-lead".to_string(), alias.to_string());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(cell: &str, transform: &str) -> Vec<String> {
        derive(cell).into_iter().filter(|v| v.transform == transform).map(|v| v.cell).collect()
    }

    #[test]
    fn an_arg_site_is_quoted_and_a_lead_is_not() {
        assert_eq!(cells("git stash list", "ps-sq-arg"), ["git 'stash' list", "git stash 'list'"]);
        assert_eq!(cells("git stash list", "ps-dq-arg"), ["git \"stash\" list", "git stash \"list\""]);
        assert_eq!(cells("git status; ls deploy", "ps-sq-arg"), ["git status; ls 'deploy'"], "a word after ; is a lead");
        assert_eq!(cells("git status | wc deploy", "ps-dq-arg"), ["git \"status\" | wc deploy", "git status | wc \"deploy\""]);
        assert_eq!(cells("cd xx && git status", "ps-sq-arg"), ["cd 'xx' && git status", "cd xx && git 'status'"]);
        assert_eq!(cells("& tools/build.ps1", "ps-sq-arg"), ["& 'tools/build.ps1'"], "the word after a call operator");
        assert_eq!(cells(". .tmp/x.ps1", "ps-sq-arg"), [". '.tmp/x.ps1'"], "the word after a dot-source operator");
        assert_eq!(cells("pwsh -File .tmp\\x.ps1", "ps-sq-arg"), ["pwsh -File '.tmp\\x.ps1'"], "a parameter is no site");
        assert_eq!(cells("git stash x", "ps-sq-arg"), ["git 'stash' x"], "a one-character token");
        assert_eq!(cells("git status > out.log 2>&1", "ps-sq-arg"), ["git 'status' > out.log 2>&1", "git status > 'out.log' 2>&1"]);
    }

    #[test]
    fn a_backtick_goes_ahead_of_a_letter_that_spells_no_control_character() {
        assert_eq!(cells("git grep foo", "ps-tick-lead"), ["g`it grep foo"]);
        assert_eq!(cells("git grep foo", "ps-tick-arg"), ["git gre`p foo", "git grep f`oo"]);
        assert_eq!(cells("git tab", "ps-tick-arg"), Vec::<String>::new(), "a, b and the rest spell a control character");
        assert_eq!(cells("git x10", "ps-tick-arg"), Vec::<String>::new(), "a digit is no letter");
        assert_eq!(cells("git sTATUS", "ps-tick-arg"), ["git sTATU`S"], "either case");
    }

    #[test]
    fn a_verb_noun_lead_is_upper_cased_and_replaced_by_each_alias() {
        assert_eq!(
            cells("Set-Location deploy; Get-ChildItem", "ps-case-lead"),
            ["SET-LOCATION deploy; Get-ChildItem", "Set-Location deploy; GET-CHILDITEM"]
        );
        assert_eq!(cells("git status", "ps-case-lead"), Vec::<String>::new(), "a native program's name is matched exactly");
        assert_eq!(cells("./build-all.ps1 deploy", "ps-case-lead"), Vec::<String>::new(), "a path is no verb-noun name");
        assert_eq!(cells("echo Remove-Item", "ps-case-lead"), Vec::<String>::new(), "an arg");
        assert_eq!(
            cells("remove-item tracked.md", "ps-alias-lead"),
            ["del tracked.md", "erase tracked.md", "rd tracked.md", "ri tracked.md", "rm tracked.md", "rmdir tracked.md"]
        );
        assert_eq!(cells("Get-Date", "ps-alias-lead"), Vec::<String>::new());
    }

    #[test]
    fn an_ineligible_row_derives_nothing() {
        for cell in [
            "git 'stash'", "git stash \"pop\"", "git sta`sh", "git status $x", "ls # note", "cat @ROOT@/notes.md",
            "git log (x)", "Start-Job { ls }", "git status < in.txt",
        ] {
            assert!(!eligible(cell), "{}", cell);
            assert!(derive(cell).is_empty(), "{}", cell);
        }
    }

    #[test]
    fn every_derived_name_is_a_transform() {
        for v in derive("Remove-Item tracked.md; git status deploy") {
            assert!(TRANSFORMS.contains(&v.transform.as_str()), "{}", v.transform);
        }
    }

    // spec: guard-kit/SPEC.md §Testing — a new row cannot arrive underived: every verb-noun lead
    // the two PowerShell tables carry has an alias-table entry.
    #[test]
    fn the_alias_table_carries_every_verb_noun_lead_of_the_tables() {
        let tests = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../guard-kit/guard-tests");
        let mut seen = 0;
        for table in ["powershell-cases.tsv", "powershell-background-cases.tsv"] {
            let body = std::fs::read_to_string(tests.join(table)).unwrap();
            for line in body.lines().filter(|l| !l.starts_with('#')) {
                let cell = line.rsplit('\t').next().unwrap();
                for site in sites(cell).into_iter().filter(|s| s.lead && verb_noun(s.text)) {
                    seen += 1;
                    assert!(aliases(site.text).is_some(), "{}: no alias-table entry for {}", table, site.text);
                }
            }
        }
        assert!(seen > 0);
    }
}
