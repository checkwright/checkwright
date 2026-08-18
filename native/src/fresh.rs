// spec: gate-sdk/SPEC.md §The diff renderer — the freshness family's shared shape: a projection
// read with command-substitution semantics, and the one site the cap is applied at
use crate::diff;
use crate::proc;
use std::path::Path;

// spec: gate-sdk/SPEC.md §Fail-closed contract — `fail_closed`'s wording, reproduced verbatim
// because a compiled member's refusal is read by the same person reading a shell member's
pub fn fail_closed(what: &str, code: Option<i32>) -> String {
    format!(
        "{} exited {} — the check could not run; treating as failure (not clean)",
        what,
        code.unwrap_or(-1)
    )
}

// spec: gate-sdk/SPEC.md §The consumer remainder cohort — the emitter anchor. The shell forms
// anchor at their own script's directory, which a compiled subcommand cannot recover; each of
// these members is consumer-declared and sat one level under the toplevel, so the two agree.
pub fn toplevel() -> Result<String, String> {
    let c = proc::run("git", &["rev-parse", "--show-toplevel"])?;
    let out = c
        .stdout()
        .ok_or_else(|| "not a git repository — the emitter anchor cannot be resolved".to_string())?;
    let s = String::from_utf8_lossy(out).trim().to_string();
    if s.is_empty() {
        return Err("git resolved no toplevel — the emitter anchor cannot be resolved".to_string());
    }
    Ok(s)
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the shell form's capture of a file, with the
// read failure the crate cannot drop: a substitution compares an unreadable file as the empty
// string, and a `Result` makes that reading unrepresentable.
pub fn read_captured(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|_| fail_closed("cat", Some(1)))
}

// spec: gate-sdk/SPEC.md §The diff renderer — `diff`'s own line model: a file's final newline
// terminates its last line rather than opening an empty one, and an empty file has no lines
pub fn file_lines(s: &str) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    s.strip_suffix('\n').unwrap_or(s).split('\n').collect()
}

// spec: gate-sdk/SPEC.md §The diff renderer — the family's `diff … | head -20`, the one place
// the cap is applied. The renderer returns every hunk; this truncates the rendered report.
pub fn print_capped_diff(left: &str, right: &str) {
    let (a, b) = (file_lines(left), file_lines(right));
    for line in diff::normal_diff(&a, &b)
        .iter()
        .take(diff::STALE_REPORT_CAP)
    {
        println!("{}", line);
    }
}

// spec: gate-sdk/SPEC.md §The consumer remainder cohort — bash's `${1:-default}`:
// an unset *and* an empty positional both fall back, which is not `args.get(n)` alone
pub fn positional<'a>(args: &'a [String], n: usize, default: &'a str) -> &'a str {
    args.get(n)
        .filter(|a| !a.is_empty())
        .map(String::as_str)
        .unwrap_or(default)
}

// spec: gate-sdk/SPEC.md §The consumer remainder cohort — bash's `${ROOT%/}`
pub fn strip_trailing_slash(s: &str) -> &str {
    s.strip_suffix('/').unwrap_or(s)
}

pub fn is_dir(p: &str) -> bool {
    Path::new(p).is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The diff renderer — the line model the cap is applied over, held
    // to `diff`'s own rather than to `str::lines`', which disagree on a lone newline
    #[test]
    fn a_files_final_newline_terminates_its_last_line() {
        assert_eq!(file_lines("a\nb\n"), vec!["a", "b"]);
        assert_eq!(file_lines("a\nb"), vec!["a", "b"]);
        assert_eq!(file_lines("\n"), vec![""]);
        assert!(file_lines("").is_empty());
    }

    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — `${1:-default}` falls
    // back on an empty positional too, the arm `args.get(n)` alone would get wrong
    #[test]
    fn an_empty_positional_falls_back_exactly_as_an_absent_one_does() {
        let empty: Vec<String> = vec![String::new()];
        let given: Vec<String> = vec!["x".into()];
        assert_eq!(positional(&[], 0, "d"), "d");
        assert_eq!(positional(&empty, 0, "d"), "d");
        assert_eq!(positional(&given, 0, "d"), "x");
    }
}
