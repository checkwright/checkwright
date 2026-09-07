// spec: gate-sdk/SPEC.md §check-portability-floor — no file on the configured install-path
// corpus uses a banned non-portable construct without a declared valve
use crate::ere::Ere;
use crate::fresh;
use crate::gates::commit_msg::is_pattern;
use crate::proc;
use crate::walk;

// spec: gate-sdk/SPEC.md §check-portability-floor — the self-exemption is a *prefix* glob over
// the basename, on check-tree-terms' precedent: a roster of banned constructs spells them by
// construction, and the shipped template and a `.local` sibling are exempt beside the file itself
const SELF_EXEMPT_PREFIX: &str = "portability-patterns";

// spec: gate-sdk/SPEC.md §check-portability-floor — the valve, on the matching line or the one
// above, in the window `update-target-exempt` and `comment-tier-exempt` already use
const VALVE: &str = "portability-declared:";

fn self_exempt(path: &str) -> bool {
    path.rsplit('/')
        .next()
        .unwrap_or(path)
        .starts_with(SELF_EXEMPT_PREFIX)
}

// spec: gate-sdk/SPEC.md §check-portability-floor — the reason is mandatory, so a line carrying
// the token with nothing after it is a violation rather than a pass: `None` is *no valve here*
// and `Some("")` is a valve whose only reader was given nothing to read
fn valve_reason(line: &str) -> Option<&str> {
    line.split_once(VALVE).map(|(_, r)| r.trim())
}

// spec: gate-sdk/SPEC.md §check-portability-floor — a NUL byte makes a member binary, and a
// construct is a spelling a shell runs: a match inside a compiled artifact names no line to edit
// and no command an adopter's machine executes, so the member is skipped and counted
fn binary(bytes: &[u8]) -> bool {
    bytes.contains(&0)
}

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}", msg);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §check-portability-floor — an *absent* pattern file disables the
// assertion where an unreadable one fails it closed: absence is how a consumer declines the
// gate, unreadability is a machine that cannot answer
fn resolve_patterns(files: &[String]) -> Result<(Vec<String>, usize), String> {
    let mut patterns: Vec<String> = Vec::new();
    let mut present = 0usize;
    for f in files {
        if !std::path::Path::new(f).exists() {
            continue;
        }
        present += 1;
        let Ok(bytes) = std::fs::read(f) else {
            return Err(format!(
                "check-portability-floor: pattern file not readable: {}\nPORTABILITY-FLOOR: {}",
                f,
                fresh::fail_closed("portability-patterns", Some(2))
            ));
        };
        let text = String::from_utf8_lossy(&bytes).into_owned();
        patterns.extend(
            fresh::file_lines(&text)
                .into_iter()
                .filter(|l| is_pattern(l))
                .map(String::from),
        );
    }
    Ok((patterns, present))
}

fn inner(args: &[String]) -> Result<i32, String> {
    // spec: gate-sdk/SPEC.md §check-portability-floor — positional pattern files win, the
    // override check-commit-msg's pair already takes; the corpus stays a knob, no positional
    // being able to carry an adopter's own install-path roster across the battery
    let files: Vec<String> = if !args.is_empty() {
        args.to_vec()
    } else {
        walk::knob_array("GATE_PORTABILITY_PATTERNS")?
    };
    let paths = walk::knob_array("GATE_PORTABILITY_PATHS")?;

    let (patterns, files_present) = resolve_patterns(&files)?;

    // spec: gate-sdk/SPEC.md §check-portability-floor — a clean verdict that says *nothing is
    // configured* is a different sentence from one that says *nothing was found*, and telling
    // them apart in the detail line is the whole bound on the degradation
    if paths.is_empty() {
        println!(
            "PORTABILITY-FLOOR: clean (no install-path corpus configured; the assertion is \
             disabled and nothing was scanned)"
        );
        return Ok(0);
    }
    if patterns.is_empty() {
        println!(
            "PORTABILITY-FLOOR: clean (0 banned construct(s) configured across {} pattern \
             file(s) present; the corpus is unchecked)",
            files_present
        );
        return Ok(0);
    }

    // spec: gate-sdk/SPEC.md §check-portability-floor — a GNU escape in a portability blocklist
    // is refused by name at compile, check-tree-terms' own exit-2 on the same engine
    let mut compiled: Vec<(String, Ere)> = Vec::new();
    for p in &patterns {
        match Ere::compile(p) {
            Ok(e) => compiled.push((p.clone(), e)),
            Err(e) => {
                eprintln!("check-portability-floor: {}: {}", p, e);
                return Err(format!(
                    "PORTABILITY-FLOOR: {}",
                    fresh::fail_closed("portability-pattern-compile", Some(2))
                ));
            }
        }
    }

    // spec: gate-sdk/SPEC.md §check-portability-floor — the corpus is the *tracked* set under the
    // configured pathspecs, never a directory walk: what an adopter executes is what the payload
    // carries, and the payload is assembled from tracked files
    let mut argv: Vec<&str> = vec!["ls-files", "--"];
    for p in &paths {
        argv.push(p.as_str());
    }
    let ls = proc::run("git", &argv).map_err(|e| format!("check-portability-floor: {}", e))?;
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            return Err(format!(
                "check-portability-floor: {}",
                fresh::fail_closed("git-ls-files", ls.code())
            ))
        }
    };

    let mut hits: Vec<String> = Vec::new();
    let mut scanned = 0usize;
    let mut skipped_binary = 0usize;
    for path in listing.lines() {
        if path.is_empty() || self_exempt(path) {
            continue;
        }
        let Ok(bytes) = std::fs::read(path) else {
            return Err(format!(
                "check-portability-floor: corpus member not readable: {}\nPORTABILITY-FLOOR: {}",
                path,
                fresh::fail_closed("corpus-member", Some(2))
            ));
        };
        if binary(&bytes) {
            skipped_binary += 1;
            continue;
        }
        scanned += 1;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let lines = fresh::file_lines(&text);
        for (i, line) in lines.iter().enumerate() {
            let Some((pat, _)) = compiled.iter().find(|(_, re)| re.is_match(line)) else {
                continue;
            };
            // spec: gate-sdk/SPEC.md §check-portability-floor — the window is the matching line
            // or the one above, and an empty reason clears nothing
            let above = if i > 0 { lines[i - 1] } else { "" };
            if let Some(r) = valve_reason(line).or_else(|| valve_reason(above)) {
                if !r.is_empty() {
                    continue;
                }
            }
            hits.push(format!("{}:{}:{}\n    pattern: {}", path, i + 1, line, pat));
        }
    }

    if !hits.is_empty() {
        println!(
            "check-portability-floor: install-path file(s) use a banned non-portable construct \
             with no declaration:"
        );
        for h in &hits {
            println!("{}", h);
        }
        println!("  help: rewrite the site in the portable spelling, or — where the dependency is");
        println!("        deliberate — declare it to your adopters and mark the site");
        println!("        '# portability-declared: <reason>' on that line or the one above,");
        println!("        naming the surface the declaration lives on. An empty reason is a");
        println!("        violation: the field's only reader is whoever reviews the diff.");
        return Ok(1);
    }

    println!(
        "PORTABILITY-FLOOR: clean ({} install-path file(s) scanned under {} configured \
         pathspec(s), {} binary member(s) skipped; none uses one of the {} banned construct(s) \
         undeclared)",
        scanned,
        paths.len(),
        skipped_binary,
        patterns.len()
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-portability-floor — the reason is mandatory, so the three
    // states a line can be in are distinct: no valve, a valve with nothing after it, a valve
    // with a reason
    #[test]
    fn a_valve_with_no_reason_is_distinguishable_from_no_valve_at_all() {
        assert!(valve_reason("    older=\"$(sort -V)\"").is_none());
        assert_eq!(valve_reason("# portability-declared:"), Some(""));
        assert_eq!(valve_reason("# portability-declared:   "), Some(""));
        assert_eq!(
            valve_reason("# portability-declared: docs/install.md §Requirements"),
            Some("docs/install.md §Requirements")
        );
    }

    // spec: gate-sdk/SPEC.md §check-portability-floor — the self-exemption is a basename prefix,
    // so the shipped template and a `.local` sibling are exempt beside the roster itself
    #[test]
    fn the_self_exemption_is_a_basename_prefix_and_not_an_exact_name() {
        assert!(self_exempt("scripts/portability-patterns.list"));
        assert!(self_exempt("scripts/portability-patterns.local.list"));
        assert!(self_exempt("gate-sdk/templates/portability-patterns.list"));
        assert!(!self_exempt("scripts/patterns.list"));
        assert!(!self_exempt("docs/notes-portability-patterns.list"));
    }

    // spec: gate-sdk/SPEC.md §check-portability-floor — the shipped roster's own shapes compile
    // through the crate's POSIX ERE engine and select the constructs they name, so a GNU escape
    // creeping into the template is a red here rather than an exit 2 on an adopter's first commit
    #[test]
    fn the_shipped_construct_shapes_compile_and_select_the_right_lines() {
        let pats = [
            "find [^|;]*-printf",
            "sort -V",
            "realpath [^|;]*--",
            "(^|[^[:alnum:]_.-])tac([^[:alnum:]_.-]|$)",
        ];
        let res: Vec<Ere> = pats
            .iter()
            .map(|p| Ere::compile(p).expect("a shipped construct pattern failed to compile"))
            .collect();
        let hit = |s: &str| res.iter().any(|r| r.is_match(s));
        assert!(hit("    find \"$d\" -type f -printf '%P\\n'"));
        assert!(hit("printf '%s\\n' \"$a\" \"$b\" | sort -V | head -n1"));
        assert!(hit("realpath --relative-to=\"$PWD\" \"$root\""));
        assert!(hit("tac \"$f\" > \"$out\""));
        assert!(!hit("find \"$d\" -type f -print"));
        assert!(!hit("sort -u \"$f\""));
        assert!(!hit("contact the maintainer"));
        assert!(!hit("the syntactic sugar is fine"));
    }

    #[test]
    fn a_nul_bearing_member_is_binary_and_a_script_is_not() {
        assert!(binary(b"\x7fELF\x00\x01"));
        assert!(!binary(b"#!/usr/bin/env bash\nsort -V\n"));
    }
}
