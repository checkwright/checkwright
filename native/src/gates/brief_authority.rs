// spec: delegation-kit/SPEC.md §check-brief-authority — a brief grants no authority the consumer
// forbids and carries every section it requires
use crate::ere::Ere;
use crate::fresh;
use crate::walk;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Directive {
    Forbid,
    Unless,
    Require,
}

struct Rule {
    directive: Directive,
    pattern: String,
    ere: Ere,
}

// spec: delegation-kit/SPEC.md §check-brief-authority — one directive and one POSIX ERE per live
// line; another directive, no pattern or a malformed pattern refuses naming the file and the line
fn parse_rules(file: &str, text: &str) -> Result<Vec<Rule>, String> {
    let mut rules: Vec<Rule> = Vec::new();
    for (i, raw) in fresh::file_lines(text).into_iter().enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if !fresh::live_line(line) {
            continue;
        }
        let at = format!("{}:{}", file, i + 1);
        let line = line.trim_start_matches([' ', '\t']);
        let (word, rest) = line.split_once([' ', '\t']).unwrap_or((line, ""));
        let directive = match word {
            "forbid" => Directive::Forbid,
            "unless" => Directive::Unless,
            "require" => Directive::Require,
            other => return Err(format!("{}: unknown directive '{}' (forbid, unless or require)", at, other)),
        };
        let pattern = rest.trim_start_matches([' ', '\t']);
        if pattern.is_empty() {
            return Err(format!("{}: '{}' carries no pattern", at, word));
        }
        let ere = Ere::compile(pattern).map_err(|e| format!("{}: malformed pattern '{}': {}", at, pattern, e))?;
        rules.push(Rule { directive, pattern: pattern.to_string(), ere });
    }
    Ok(rules)
}

fn load_rules() -> Result<Vec<Rule>, String> {
    let mut rules: Vec<Rule> = Vec::new();
    for f in walk::knob_words("DELEGATION_KIT_BRIEF_RULES")? {
        if !Path::new(&f).is_file() {
            return Err(format!("rule file not found: {} (DELEGATION_KIT_BRIEF_RULES)", f));
        }
        let bytes = std::fs::read(&f).map_err(|e| format!("cannot read rule file {}: {}", f, e))?;
        rules.extend(parse_rules(&f, &String::from_utf8_lossy(&bytes))?);
    }
    Ok(rules)
}

// spec: delegation-kit/SPEC.md §check-brief-authority — a forbid match on a line no unless rule
// matches is a grant; a require rule no line matches is a missing section
fn judge(brief: &str, text: &str, rules: &[Rule]) -> (Vec<String>, Vec<String>) {
    let of = |d: Directive| rules.iter().filter(move |r| r.directive == d);
    let lines: Vec<&str> = fresh::file_lines(text).into_iter().map(|l| l.strip_suffix('\r').unwrap_or(l)).collect();
    let mut grants: Vec<String> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if of(Directive::Unless).any(|r| r.ere.is_match(line)) {
            continue;
        }
        if let Some(r) = of(Directive::Forbid).find(|r| r.ere.is_match(line)) {
            grants.push(format!("{}:{}: grants forbidden authority (forbid {})", brief, i + 1, r.pattern));
        }
    }
    let missing: Vec<String> = of(Directive::Require)
        .filter(|r| !lines.iter().any(|l| r.ere.is_match(l)))
        .map(|r| format!("{}: lacks a required section (require {})", brief, r.pattern))
        .collect();
    (grants, missing)
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-brief-authority: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    // spec: delegation-kit/SPEC.md §check-brief-authority — no brief is a tracked surface, so the
    // battery's no-operand run reads nothing
    if args.is_empty() {
        println!("BRIEF-AUTHORITY: clean (not applicable: no brief operand — a brief is read where it is dispatched, never as a tracked surface; skipped)");
        return Ok(0);
    }
    let rules = load_rules()?;
    let mut briefs: Vec<(String, String)> = Vec::new();
    for b in args {
        if !Path::new(b).is_file() {
            return Err(format!("brief not found: {}", b));
        }
        let bytes = std::fs::read(b).map_err(|e| format!("cannot read brief {}: {}", b, e))?;
        briefs.push((b.clone(), String::from_utf8_lossy(&bytes).into_owned()));
    }

    let (mut grants, mut missing) = (Vec::new(), Vec::new());
    for (b, text) in &briefs {
        let (g, m) = judge(b, text, &rules);
        grants.extend(g);
        missing.extend(m);
    }
    for g in &grants {
        println!("{}", g);
    }
    if !grants.is_empty() {
        println!("  help: drop the grant from the brief, or phrase the line as a prohibition an unless rule names; keep one instruction per line");
    }
    for m in &missing {
        println!("{}", m);
    }
    if !missing.is_empty() {
        println!("  help: add the section the require rule names to the brief before dispatching it");
    }
    if !grants.is_empty() || !missing.is_empty() {
        return Ok(1);
    }
    let count = |d: Directive| rules.iter().filter(|r| r.directive == d).count();
    println!(
        "BRIEF-AUTHORITY: clean ({} brief(s); {} forbid, {} unless, {} require rule(s))",
        briefs.len(),
        count(Directive::Forbid),
        count(Directive::Unless),
        count(Directive::Require)
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grammar_refuses_an_unknown_directive_an_empty_pattern_and_a_malformed_one() {
        let err = |t: &str| parse_rules("r.list", t).err().expect("refused");
        assert!(err("# c\n\nallow push\n").contains("r.list:3: unknown directive 'allow'"));
        assert!(err("forbid   \n").contains("r.list:1: 'forbid' carries no pattern"));
        assert!(err("require\n").contains("'require' carries no pattern"));
        assert!(err("forbid (push\n").contains("r.list:1: malformed pattern '(push'"));
    }

    #[test]
    fn comments_and_blanks_are_skipped_and_each_directive_keeps_its_whole_pattern() {
        let rules = parse_rules("r.list", "# intent\n\n  forbid \tgit push( |$)\nunless never\r\nrequire ^> \n").expect("parses");
        let got: Vec<(Directive, &str)> = rules.iter().map(|r| (r.directive, r.pattern.as_str())).collect();
        assert_eq!(
            got,
            [(Directive::Forbid, "git push( |$)"), (Directive::Unless, "never"), (Directive::Require, "^> ")]
        );
    }

    #[test]
    fn an_unless_match_excuses_its_whole_line_and_a_require_reads_every_line() {
        let rules = parse_rules("r", "forbid push\nforbid install\nunless never\nrequire ^> ").expect("parses");
        let (g, m) = judge("b.md", "> the request\nnever push the branch\npush the branch\r\nnever push; install it\n", &rules);
        assert_eq!(g, ["b.md:3: grants forbidden authority (forbid push)"]);
        assert!(m.is_empty());
        let (_, m) = judge("b.md", "push nothing here\n", &parse_rules("r", "require ^> ").expect("parses"));
        assert_eq!(m, ["b.md: lacks a required section (require ^> )"]);
    }

    #[test]
    fn a_no_operand_run_is_a_clean_skip_and_a_missing_brief_or_rule_file_refuses() {
        let knobs = crate::knobenv::lock();
        assert_eq!(rule(&[]).expect("skips"), 0);
        let scratch = std::env::temp_dir().join(format!("brief-authority-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&scratch);
        std::fs::create_dir_all(&scratch).expect("mkdir");
        let brief = scratch.join("brief.md");
        std::fs::write(&brief, "do the work\n").expect("write");
        let absent = scratch.join("absent.list").display().to_string();
        let brief = brief.display().to_string();
        let missing_brief = rule(&[scratch.join("nope.md").display().to_string()]);
        knobs.set("DELEGATION_KIT_BRIEF_RULES", &absent);
        let missing_rules = rule(std::slice::from_ref(&brief));
        knobs.remove("DELEGATION_KIT_BRIEF_RULES");
        let _ = std::fs::remove_dir_all(&scratch);
        assert!(missing_brief.expect_err("refused").contains("brief not found"));
        assert!(missing_rules.expect_err("refused").contains("rule file not found"));
    }
}
