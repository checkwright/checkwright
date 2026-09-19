// spec: gate-sdk/SPEC.md §check-front-end-fail-open — each front-end stub's fail-open declaration
// line names exactly the crate's FAIL_OPEN_ARMS
use crate::emit::FAIL_OPEN_ARMS;
use crate::fresh;
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

const NAME: &str = "check-front-end-fail-open";

// spec: gate-sdk/SPEC.md §check-front-end-fail-open — one stub: its path under the gate-sdk root,
// the name its declaration line assigns, and that line's parser
struct Stub {
    rel: &'static str,
    var: &'static str,
    parse: fn(&str) -> Option<Vec<String>>,
}

const STUBS: &[Stub] = &[
    Stub {
        rel: "bin/run-gates.sh",
        var: "FAIL_OPEN_ARMS",
        parse: parse_sh,
    },
    Stub {
        rel: "bin/run-gates.ps1",
        var: "$FailOpenArms",
        parse: parse_ps1,
    },
];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let sdk_default = walk::sdk_root();
    let sdk = fresh::strip_trailing_slash(fresh::positional(args, 0, &sdk_default)).to_string();
    let declared: BTreeSet<&str> = FAIL_OPEN_ARMS.iter().copied().collect();
    let mut held: Vec<String> = Vec::new();
    let mut absent: Vec<String> = Vec::new();
    let mut findings: Vec<String> = Vec::new();
    for stub in STUBS {
        let path = format!("{}/{}", sdk, stub.rel);
        if !Path::new(&path).is_file() {
            absent.push(path);
            continue;
        }
        let text = fresh::read_captured(&path)?;
        let names = declaration(&text, stub, &path)?;
        let named: BTreeSet<&str> = names.iter().map(String::as_str).collect();
        for arm in named.difference(&declared) {
            findings.push(format!(
                "{}: names '{}', which the crate's FAIL_OPEN_ARMS does not declare",
                path, arm
            ));
        }
        for arm in declared.difference(&named) {
            findings.push(format!(
                "{}: omits '{}', which the crate's FAIL_OPEN_ARMS declares",
                path, arm
            ));
        }
        held.push(path);
    }

    let set = FAIL_OPEN_ARMS.join(" ");
    if !findings.is_empty() {
        println!(
            "{}: a front-end stub's fail-open set differs from the crate's declaration ({}):",
            NAME, set
        );
        println!();
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: a stub exits 0 without the binary only for the arms it names, so a missing");
        println!("        name wedges a binary-less session and an extra one silences a verdict. Edit");
        println!("        the stub's declaration line to the crate's FAIL_OPEN_ARMS, or change that");
        println!("        declaration in native/src/emit/mod.rs first.");
        return Ok(1);
    }

    let mut line = match held.len() {
        0 => format!("FRONT-END-FAIL-OPEN: clean (no front-end stub to hold to the fail-open set {}", set),
        1 => format!("FRONT-END-FAIL-OPEN: clean ({} names the fail-open set {}", held[0], set),
        _ => format!("FRONT-END-FAIL-OPEN: clean ({} name the fail-open set {}", held.join(" and "), set),
    };
    for a in &absent {
        line.push_str(&format!("; {} absent, no copy to hold", a));
    }
    line.push(')');
    println!("{}", line);
    Ok(0)
}

// spec: gate-sdk/SPEC.md §check-front-end-fail-open — exactly one declaration line, and it parses;
// a stub the gate cannot read is not a clean one
fn declaration(text: &str, stub: &Stub, path: &str) -> Result<Vec<String>, String> {
    let lines: Vec<&str> = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| {
            l.strip_prefix(stub.var)
                .is_some_and(|rest| rest.trim_start().starts_with('='))
        })
        .collect();
    let unreadable = |why: String| {
        format!(
            "{} {} — the check could not run; treating as failure (not clean)",
            path, why
        )
    };
    match lines.as_slice() {
        [one] => (stub.parse)(one)
            .ok_or_else(|| unreadable(format!("carries a {} line that does not parse: {}", stub.var, one))),
        [] => Err(unreadable(format!("carries no {} declaration line", stub.var))),
        many => Err(unreadable(format!(
            "carries {} {} declaration lines where it declares exactly one",
            many.len(),
            stub.var
        ))),
    }
}

// spec: gate-sdk/SPEC.md §check-front-end-fail-open — `FAIL_OPEN_ARMS='<arm> <arm>'`: one
// single-quoted value, split on blanks
fn parse_sh(line: &str) -> Option<Vec<String>> {
    let value = line.strip_prefix("FAIL_OPEN_ARMS=")?;
    let inner = value.strip_prefix('\'')?.strip_suffix('\'')?;
    if inner.contains('\'') {
        return None;
    }
    Some(inner.split_whitespace().map(String::from).collect())
}

// spec: gate-sdk/SPEC.md §check-front-end-fail-open — `$FailOpenArms = @('<arm>', '<arm>')`: an
// array literal of single-quoted strings
fn parse_ps1(line: &str) -> Option<Vec<String>> {
    let value = line.strip_prefix("$FailOpenArms")?.trim_start().strip_prefix('=')?.trim();
    let inner = value.strip_prefix("@(")?.strip_suffix(')')?.trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }
    inner
        .split(',')
        .map(|item| {
            let s = item.trim().strip_prefix('\'')?.strip_suffix('\'')?;
            (!s.is_empty() && !s.contains('\'')).then(|| s.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-front-end-fail-open — each dialect's declaration reads back as
    // its names, and a line in the other dialect's shape or a stray quote does not parse
    #[test]
    fn each_dialect_parses_its_own_declaration_shape_alone() {
        assert_eq!(
            parse_sh("FAIL_OPEN_ARMS='--hook --statusline'"),
            Some(vec!["--hook".to_string(), "--statusline".to_string()])
        );
        assert_eq!(
            parse_ps1("$FailOpenArms = @('--hook', '--statusline')"),
            Some(vec!["--hook".to_string(), "--statusline".to_string()])
        );
        assert_eq!(parse_sh("FAIL_OPEN_ARMS=--hook"), None);
        assert_eq!(parse_sh("FAIL_OPEN_ARMS='--hook' '--x'"), None);
        assert_eq!(parse_ps1("$FailOpenArms = '--hook'"), None);
        assert_eq!(parse_ps1("$FailOpenArms = @('--hook', --statusline)"), None);
    }

    // spec: gate-sdk/SPEC.md §check-front-end-fail-open — a line reading the name is not a line
    // assigning it, so the membership test beside the declaration is never a second declaration
    #[test]
    fn only_an_assignment_counts_as_a_declaration_line() {
        let stub = &STUBS[1];
        let text = "$FailOpenArms = @('--hook')\nif ($FailOpenArms -ccontains $lead) { }\n";
        assert_eq!(declaration(text, stub, "x").unwrap(), vec!["--hook".to_string()]);
        let twice = "$FailOpenArms = @('--hook')\n$FailOpenArms=@('--hook')\n";
        assert!(declaration(twice, stub, "x").is_err());
    }
}
