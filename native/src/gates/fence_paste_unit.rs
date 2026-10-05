// spec: canon-kit/SPEC.md §check-fence-paste-unit — on a declared page every shell fence is one paste,
// and every PowerShell fence of more than one statement is one script block
use super::fence_command_head::{fences_of, SHELL_LANGS};
use crate::bashscan::{self, Sep};
use crate::spec;
use crate::walk;
use std::path::Path;

const PAGES_KNOB: &str = "CANON_KIT_FENCE_PASTE_PAGES";

// spec: canon-kit/SPEC.md §check-fence-paste-unit — the fence languages arm B reads
const POWERSHELL_LANGS: [&str; 3] = ["powershell", "pwsh", "ps1"];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-fence-paste-unit: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let globs = spec::knob_array_pub(PAGES_KNOB)?;
    let mut pages: Vec<String> = if globs.is_empty() {
        Vec::new()
    } else {
        walk::glob_corpus(Path::new(root), &globs)?
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| spec::strip_dot_slash(&p.display().to_string()))
            .collect()
    };
    pages.sort();
    pages.dedup();

    let mut unstopped: Vec<String> = Vec::new();
    let mut unwrapped: Vec<String> = Vec::new();
    let (mut nshell, mut npwsh) = (0usize, 0usize);
    for page in &pages {
        let text = spec::read_text(Path::new(page))
            .map_err(|e| format!("cannot read declared page {}: {}", page, e))?;
        for (open, body) in fences_of(&text, &SHELL_LANGS) {
            nshell += 1;
            for ln in unstopped_lists(&body) {
                unstopped.push(format!(
                    "  {}:{}: in the shell fence opened at line {}, a failure here leaves the next command running",
                    page,
                    open + ln,
                    open
                ));
            }
        }
        for (open, body) in fences_of(&text, &POWERSHELL_LANGS) {
            npwsh += 1;
            if let Some(n) = unwrapped_statements(&body) {
                unwrapped.push(format!(
                    "  {}:{}: a PowerShell fence of {} statement lines, not wrapped as `. {{ … }}`",
                    page, open, n
                ));
            }
        }
    }

    if !unstopped.is_empty() || !unwrapped.is_empty() {
        println!(
            "check-fence-paste-unit: {} fence finding(s) on the declared pages:",
            unstopped.len() + unwrapped.len()
        );
        for f in unstopped.iter().chain(unwrapped.iter()) {
            println!("{}", f);
        }
        if !unstopped.is_empty() {
            println!("  help: join the commands with `&&`, or split the block into one per step.");
        }
        if !unwrapped.is_empty() {
            println!("  help: wrap the statements as `. {{ … }}`.");
        }
        println!("  help: a fence shown for reading rather than running takes another info string (text, console).");
        return Ok(1);
    }
    println!(
        "FENCE-PASTE-UNIT: clean ({} page(s); {} shell fence(s); {} PowerShell fence(s))",
        pages.len(),
        nshell,
        npwsh
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-fence-paste-unit — arm A: the body line of every top-level list a
// further list follows, unless every word of it is an assignment carrying no command substitution
fn unstopped_lists(body: &str) -> Vec<usize> {
    let mut lists: Vec<(usize, bool)> = Vec::new();
    for p in bashscan::command_placements(body) {
        if !p.top {
            continue;
        }
        let starts = matches!(p.sep, Sep::Start | Sep::Newline | Sep::Semi | Sep::Amp);
        match lists.last_mut() {
            Some(last) if !starts => last.1 &= p.bare_assignment,
            _ => lists.push((p.line, p.bare_assignment)),
        }
    }
    let keep = lists.len().saturating_sub(1);
    lists
        .into_iter()
        .take(keep)
        .filter(|(_, bare)| !bare)
        .map(|(line, _)| line)
        .collect()
}

// spec: canon-kit/SPEC.md §check-fence-paste-unit — arm B: a fence of two or more statement lines
// opens with the line `. {` and closes with the line `}`; `Some` carries the count of one that does not
fn unwrapped_statements(body: &str) -> Option<usize> {
    let lines: Vec<&str> = body
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    let wrapped = lines.first() == Some(&". {") && lines.last() == Some(&"}");
    if lines.len() >= 2 && !wrapped {
        Some(lines.len())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_list_followed_by_another_is_red_unless_it_is_a_bare_assignment() {
        assert_eq!(unstopped_lists("a\nb\n"), vec![1]);
        assert_eq!(unstopped_lists("a; b\n"), vec![1]);
        assert_eq!(unstopped_lists("a & b\n"), vec![1]);
        assert_eq!(unstopped_lists("v=1\nw=2 x=3\na\n"), Vec::<usize>::new());
        assert_eq!(unstopped_lists("cw=\"$(mktemp -d)\"\ncurl x\n"), vec![1]);
        assert_eq!(unstopped_lists("cw=`mktemp -d`\ncurl x\n"), vec![1]);
        assert_eq!(unstopped_lists("a\ncw=\"$(b)\"\nc\n"), vec![1, 2]);
        assert_eq!(unstopped_lists("a\nw=`b` c\nd\n"), vec![1, 2]);
        assert_eq!(unstopped_lists("export P=1\na\n"), vec![1]);
        assert_eq!(unstopped_lists("V=1 a\nb\n"), vec![1]);
        assert_eq!(unstopped_lists("arr=(a \"$(b)\")\nc\n"), vec![1]);
        assert_eq!(unstopped_lists("arr=(a b)\nc\n"), Vec::<usize>::new());
    }

    #[test]
    fn joined_continued_and_compound_commands_are_one_list() {
        let joined = "brew install bash \\\n  && echo x >> f \\\n  && export P=1\n";
        assert_eq!(unstopped_lists(joined), Vec::<usize>::new());
        assert_eq!(unstopped_lists("a &&\n  b ||\n  c | d |& e\n"), Vec::<usize>::new());
        let sub = "( cd x \\\n  && { a || b; } \\\n  && tar x )\nsh y\n";
        assert_eq!(unstopped_lists(sub), vec![1]);
        let sub_joined = "( cd x \\\n  && { a || b; } \\\n  && tar x ) \\\n  && sh y\n";
        assert_eq!(unstopped_lists(sub_joined), Vec::<usize>::new());
        let compound = "if a; then\n  b\n  c\nfi\n";
        assert_eq!(unstopped_lists(compound), Vec::<usize>::new());
        let looped = "for f in *; do\n  a \"$f\"\ndone && b\n";
        assert_eq!(unstopped_lists(looped), Vec::<usize>::new());
        let cased = "case \"$x\" in\n  a) b ;;\n  c) d ;;\nesac\n";
        assert_eq!(unstopped_lists(cased), Vec::<usize>::new());
        assert_eq!(unstopped_lists("f() { a; b; }\n"), Vec::<usize>::new());
        assert_eq!(unstopped_lists("[[ -f x ]] && a\n"), Vec::<usize>::new());
    }

    #[test]
    fn a_redirection_ampersand_a_heredoc_and_a_comment_start_no_list() {
        assert_eq!(unstopped_lists("a 2>&1 | b\n"), Vec::<usize>::new());
        assert_eq!(unstopped_lists("a &>/dev/null\n"), Vec::<usize>::new());
        assert_eq!(unstopped_lists("cat <<'EOF'\nb\nc\nEOF\n"), Vec::<usize>::new());
        assert_eq!(unstopped_lists("# a\n\na # b\n# c\n"), Vec::<usize>::new());
        assert_eq!(unstopped_lists("cat <<'EOF'\nb\nEOF\nc\n"), vec![1]);
    }

    #[test]
    fn the_finding_names_the_line_the_list_starts_on() {
        let body = "v=1\n( cd x \\\n  && y )\nz\n";
        assert_eq!(unstopped_lists(body), vec![2]);
    }

    #[test]
    fn a_powershell_sequence_is_wrapped_as_a_dot_sourced_block() {
        assert_eq!(unwrapped_statements("irm x | iex\n"), None);
        assert_eq!(unwrapped_statements("# note\n\nirm x | iex\n"), None);
        assert_eq!(unwrapped_statements("$a = 1\nb $a\n"), Some(2));
        assert_eq!(unwrapped_statements(". {\n  $a = 1\n  b $a\n}\n"), None);
        assert_eq!(unwrapped_statements("& {\n  $a = 1\n}\n"), Some(3));
        assert_eq!(unwrapped_statements(". {\n  $a = 1\n  b $a\n"), Some(3));
    }
}
