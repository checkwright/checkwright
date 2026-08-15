// spec: RELEASING.md §The publish spec — every `npm publish` positional spec in a workflow is
// unambiguously a path by its own literal text: a leading `.` or `/`, or an expansion of a
// proven-absolute root
use crate::fresh;
use crate::walk;
use std::path::Path;

const DEFAULT_WFDIR: &str = ".github/workflows";
// spec: RELEASING.md §The publish spec — the roster is proven-absolute by written contract,
// never by runner observation
const ABS_ROOTS: &[&str] = &["PWD", "GITHUB_WORKSPACE", "RUNNER_TEMP"];
// spec: RELEASING.md §The publish spec — the token after one of these is a flag value, not the
// positional spec
const VALUE_FLAGS: &[&str] = &["--access", "--tag", "--otp", "--registry", "--workspace"];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}", e);
            2
        }
    }
}

// spec: RELEASING.md §The publish spec — quote-aware split, so a quoted spec carrying spaces
// stays one token instead of parsing as several
fn split_tokens(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut tok = String::new();
    let mut quote: Option<char> = None;
    for c in s.chars() {
        if let Some(q) = quote {
            tok.push(c);
            if c == q {
                quote = None;
            }
            continue;
        }
        match c {
            '"' | '\'' => {
                quote = Some(c);
                tok.push(c);
            }
            ' ' | '\t' => {
                if !tok.is_empty() {
                    out.push(std::mem::take(&mut tok));
                }
            }
            _ => tok.push(c),
        }
    }
    if !tok.is_empty() {
        out.push(tok);
    }
    out
}

// spec: RELEASING.md §The publish spec — strip one layer of shell quoting before deciding,
// since every real spec on this surface is quoted
fn strip_quotes(t: &str) -> &str {
    let b = t.as_bytes();
    if b.len() >= 2 && ((b[0] == b'"' && b[b.len() - 1] == b'"') || (b[0] == b'\'' && b[b.len() - 1] == b'\'')) {
        return &t[1..t.len() - 1];
    }
    t
}

// spec: RELEASING.md §The publish spec — npm's own path rule, then the proven-absolute-root arm
fn spec_unambiguous(s: &str) -> bool {
    if s.starts_with('.') || s.starts_with('/') {
        return true;
    }
    ABS_ROOTS.iter().any(|r| {
        s.starts_with(&format!("${}/", r)) || s.starts_with(&format!("${{{}}}/", r))
    })
}

fn rule(args: &[String]) -> Result<i32, String> {
    let wfdir = fresh::positional(args, 0, DEFAULT_WFDIR);
    if !fresh::is_dir(wfdir) {
        if !args.is_empty() {
            return Err(format!(
                "check-npm-publish-spec: workflows dir not found: {}",
                wfdir
            ));
        }
        println!(
            "NPM-PUBLISH-SPEC: clean (no {} in this tree — 0 npm publish invocation(s) to judge)",
            wfdir
        );
        return Ok(0);
    }

    // spec: RELEASING.md §The publish spec — the shell form's two globs in their own order,
    // every `*.yml` then every `*.yaml`
    let files: Vec<String> = walk::glob_files(
        Path::new(wfdir),
        &["*.yml".to_string(), "*.yaml".to_string()],
    )
    .map_err(|e| format!("check-npm-publish-spec: {}", e))?
    .into_iter()
    .map(|p| p.display().to_string())
    .collect();
    if files.is_empty() {
        println!(
            "NPM-PUBLISH-SPEC: clean (no YAML under {} — 0 npm publish invocation(s) to judge)",
            wfdir
        );
        return Ok(0);
    }

    let mut invocations = 0usize;
    let mut findings: Vec<String> = Vec::new();

    for f in &files {
        let text = fresh::read_captured(f).map_err(|e| format!("check-npm-publish-spec: {}", e))?;
        for (n, line) in fresh::file_lines(&text).iter().enumerate() {
            let lineno = n + 1;
            if !line.contains("npm publish") {
                continue;
            }
            if line.trim_start().starts_with('#') {
                continue;
            }
            invocations += 1;

            if line.ends_with('\\') {
                eprintln!("check-npm-publish-spec: an 'npm publish' line ends in a backslash continuation, so its");
                eprintln!("positional spec is not line-local and this gate would judge a partial invocation:");
                eprintln!("  {}:{}", f, lineno);
                eprintln!("  help: join the invocation onto one line, or move the spec into a variable assigned on");
                eprintln!("        its own line and published through a './', '/', or $PWD/-prefixed expansion.");
                return Ok(2);
            }

            let tail = line.split_once("npm publish").map(|(_, r)| r).unwrap_or("");
            let toks = split_tokens(tail);
            let mut cands: Vec<String> = Vec::new();
            let mut prev = String::new();
            for t in &toks {
                let skip = VALUE_FLAGS.iter().any(|vf| prev == *vf);
                prev = t.clone();
                if skip || t.starts_with('-') {
                    continue;
                }
                cands.push(t.clone());
            }

            if cands.len() > 1 {
                eprintln!("check-npm-publish-spec: more than one positional candidate parsed out of an 'npm publish'");
                eprintln!("invocation, which accepts at most one — the parse is wrong, so the gate refuses to guess:");
                eprintln!("  {}:{}: {}", f, lineno, cands.join(" "));
                eprintln!("  help: put the invocation on a line of its own carrying only flags and the one spec, or");
                eprintln!("        teach the gate's value-taking-flag roster the flag whose value it mistook for a spec.");
                return Ok(2);
            }
            if cands.len() != 1 {
                continue;
            }
            if !spec_unambiguous(strip_quotes(&cands[0])) {
                findings.push(format!("{}:{}: {}", f, lineno, cands[0]));
            }
        }
    }

    if !findings.is_empty() {
        println!("check-npm-publish-spec: an npm publish spec whose resolution depends on runtime state rather");
        println!("than on its own literal text — npm reads a positional spec as a path when it begins with '.'");
        println!("or '/', and as the GitHub 'owner/repo' shorthand otherwise:");
        for f in &findings {
            println!("  {}", f);
        }
        let roots_help: Vec<String> = ABS_ROOTS.iter().map(|r| format!("${}/", r)).collect();
        println!("  help: make the literal decide it — prefix the spec with './' or '/', or with a bare or braced");
        println!(
            "        expansion of a proven-absolute root ({}).",
            roots_help.join(", ")
        );
        println!("        A command substitution or a glob is assigned to a variable first, then published");
        println!("        through such a prefix.");
        return Ok(1);
    }

    println!(
        "NPM-PUBLISH-SPEC: clean ({} npm publish invocation(s) across {} YAML file(s) under {}, every positional spec unambiguous by its literal)",
        invocations,
        files.len(),
        wfdir
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: RELEASING.md §The publish spec — a quoted spec carrying spaces is one token
    #[test]
    fn the_split_is_quote_aware() {
        assert_eq!(split_tokens(" --access public './p'"), vec!["--access", "public", "'./p'"]);
        assert_eq!(split_tokens(" \"a b\" c"), vec!["\"a b\"", "c"]);
    }

    // spec: RELEASING.md §The publish spec — the literal decides, or the gate reds
    #[test]
    fn only_a_literal_path_or_a_proven_absolute_root_is_unambiguous() {
        for ok in ["./p", "/p", "$PWD/p", "${GITHUB_WORKSPACE}/p", "$RUNNER_TEMP/p", "../p"] {
            assert!(spec_unambiguous(ok), "{} should be unambiguous", ok);
        }
        for bad in ["p", "owner/repo", "$(ls)/p", "$HOME/p", "*.tgz"] {
            assert!(!spec_unambiguous(bad), "{} should be ambiguous", bad);
        }
    }

    #[test]
    fn one_layer_of_quoting_is_stripped() {
        assert_eq!(strip_quotes("'./p'"), "./p");
        assert_eq!(strip_quotes("\"./p\""), "./p");
        assert_eq!(strip_quotes("./p"), "./p");
    }
}
