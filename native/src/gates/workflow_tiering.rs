// spec: gate-sdk/SPEC.md §The workflow directory — every workflow-dir member is tracked or
// ignored, and every tracked non-directory member's first line is a ruled '# contract: ' header
use crate::ere::Ere;
use crate::proc;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §The workflow directory — the two ruled payload forms
const POINTER_RE_SRC: &str = "^[A-Za-z0-9._/-]+\\.md[[:space:]]+§[^[:space:]]";
const VERSION_RE_SRC: &str = "^[a-z0-9-]+ v[0-9]+$";

pub fn run(args: &[String]) -> i32 {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        eprintln!("check-workflow-tiering: not a directory: {}", root);
        return 2;
    }
    let probe = match proc::run("git", &["-C", root, "rev-parse", "--git-dir"]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("check-workflow-tiering: {}", e);
            return 2;
        }
    };
    if probe.stdout().is_none() {
        eprintln!(
            "check-workflow-tiering: {} is not a git repository — the tracked/ignored partition is unreadable",
            root
        );
        return 2;
    }

    let wf = match walk::knob_scalar("GATE_SDK_WORKFLOW_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-workflow-tiering: {}", e);
            return 2;
        }
    };
    let wf_path = format!("{}/{}", root, wf);
    if !Path::new(&wf_path).is_dir() {
        eprintln!("check-workflow-tiering: workflow directory not found: {}", wf_path);
        return 2;
    }

    let pointer_re = match Ere::compile(POINTER_RE_SRC) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("check-workflow-tiering: pointer pattern failed to compile: {}", e);
            return 2;
        }
    };
    let version_re = match Ere::compile(VERSION_RE_SRC) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("check-workflow-tiering: version pattern failed to compile: {}", e);
            return 2;
        }
    };

    let entries = match walk::list_dir(Path::new(&wf_path)) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-workflow-tiering: {}", e);
            return 2;
        }
    };

    let mut errors: Vec<String> = Vec::new();
    let mut tracked = 0usize;
    let mut ignored = 0usize;

    for (name, is_dir) in &entries {
        let is_dir = *is_dir;
        let rel = format!("{}/{}", wf, name);

        let is_tracked = if is_dir {
            let ls = match proc::run("git", &["-C", root, "ls-files", "--", &rel]) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("check-workflow-tiering: {}", e);
                    return 2;
                }
            };
            match ls.stdout() {
                Some(o) => !o.is_empty(),
                None => {
                    eprintln!(
                        "check-workflow-tiering: git ls-files ({}) exited {} — the check could not run; treating as failure (not clean)",
                        rel,
                        ls.code().unwrap_or(-1)
                    );
                    return 2;
                }
            }
        } else {
            match proc::run(
                "git",
                &["-C", root, "ls-files", "--error-unmatch", "--", &rel],
            ) {
                Ok(c) => c.code() == Some(0),
                Err(e) => {
                    eprintln!("check-workflow-tiering: {}", e);
                    return 2;
                }
            }
        };

        let is_ignored = {
            let ci = match proc::run("git", &["-C", root, "check-ignore", "-q", "--", &rel]) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("check-workflow-tiering: {}", e);
                    return 2;
                }
            };
            match ci.code() {
                Some(0) => true,
                Some(1) => false,
                other => {
                    eprintln!(
                        "check-workflow-tiering: git check-ignore exited {} on {}",
                        other.unwrap_or(-1),
                        rel
                    );
                    return 2;
                }
            }
        };

        // assertion A: partition totality
        if !is_tracked && !is_ignored {
            errors.push(format!("{}: neither tracked nor ignored — no tier holds it", rel));
            continue;
        }
        if !is_tracked {
            ignored += 1;
            continue;
        }
        tracked += 1;
        if is_dir {
            continue;
        }

        // assertion B: header presence and ruled payload form
        let path = format!("{}/{}", wf_path, name);
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("check-workflow-tiering: member not readable: {}: {}", rel, e);
                return 2;
            }
        };
        let first = text.lines().next().unwrap_or("");
        let Some(payload) = first.strip_prefix("# contract: ") else {
            errors.push(format!(
                "{}: tracked member's first line is not a '# contract: ' header: {}",
                rel,
                truncate(first, 60)
            ));
            continue;
        };
        let sig = signature(payload);
        if !pointer_re.is_match(sig) && !version_re.is_match(sig) {
            errors.push(format!(
                "{}: '# contract: ' payload is neither '<owner-path>.md §<section>' nor '<format-name> v<N>': {}",
                rel, sig
            ));
        }
    }

    if !errors.is_empty() {
        println!("check-workflow-tiering: {} workflow-surface violation(s):", errors.len());
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: every workflow-dir member belongs to one of two tiers — a tracked checked projection or a gitignored local capture; add the member to .gitignore or commit it. A tracked member's first line is '# contract: <owner-path>.md §<section>' (optionally ' — <grammar or gloss>'), or '# contract: <format-name> v<N>' where a gate parses the header as a wire-format version and its owning SPEC says so.");
        return 1;
    }
    println!(
        "WORKFLOW-TIERING: clean ({} tracked projection(s) with a ruled '# contract: ' header, {} capture member(s); every member holds a tier)",
        tracked, ignored
    );
    0
}

// spec: gate-sdk/SPEC.md §The workflow directory — the payload's signature: everything before
// the first ' — ' or ' -- ' grammar/gloss separator, trailing whitespace trimmed
fn signature(payload: &str) -> &str {
    let cut1 = payload.find(" — ").unwrap_or(payload.len());
    let sig = &payload[..cut1];
    let cut2 = sig.find(" -- ").unwrap_or(sig.len());
    sig[..cut2].trim_end_matches([' ', '\t', '\x0b', '\x0c', '\r'])
}

fn truncate(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_cuts_at_the_first_separator() {
        assert_eq!(
            signature("gate-sdk/SPEC.md §X — a grammar note"),
            "gate-sdk/SPEC.md §X"
        );
        assert_eq!(signature("fixture-format v3"), "fixture-format v3");
        assert_eq!(signature("a v1 -- gloss"), "a v1");
        assert_eq!(signature("no separator here"), "no separator here");
    }

    #[test]
    fn the_two_payload_forms_compile_and_match() {
        let pointer = Ere::compile(POINTER_RE_SRC).unwrap();
        let version = Ere::compile(VERSION_RE_SRC).unwrap();
        assert!(pointer.is_match("gate-sdk/SPEC.md §The workflow directory"));
        assert!(!pointer.is_match("not a pointer and not a version marker"));
        assert!(version.is_match("fixture-format v3"));
        assert!(!version.is_match("fixture-format v3 extra"));
    }
}
