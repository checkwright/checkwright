// spec: lifecycle-kit/SPEC.md §The close-surface roster — the derivation: every close-surface:
// declaration across the resolved declaration surfaces, unioned with the workflow directory's
// capture tier so an undeclared capture surface is reported rather than missing
use crate::{proc, programs};
use crate::stages;
use crate::walk;
use std::path::Path;

// spec: lifecycle-kit/SPEC.md §The close-surface roster — the effective base, computed and never
// entered: `set_current_dir` is process-global, so the arm anchors every path it globs, ignores
// and prints against this instead of `cd`-ing into it.
pub fn base(args: &[String]) -> Result<String, String> {
    match args.first().filter(|a| !a.is_empty()) {
        Some(a) => {
            let root = if a.len() > 1 { a.trim_end_matches('/') } else { a.as_str() };
            let probe = proc::run(&programs::GIT, &["-C", root, "rev-parse", "--git-dir"])?;
            if probe.stdout().is_none() {
                return Err(not_a_repo());
            }
            Ok(root.to_string())
        }
        None => {
            crate::walk::toplevel_opt()?.ok_or_else(not_a_repo)
        }
    }
}

fn not_a_repo() -> String {
    "not a git repository — the capture tier is underivable".to_string()
}

fn joined(base: &str, rel: &str) -> String {
    format!("{}/{}", base, rel)
}

// spec: lifecycle-kit/SPEC.md §The close-surface roster — the shell form's `${f#./}`, generalised
// to the computed base: a surface is named relative to the world the scan was rooted at, whichever
// of the two ways that root was resolved.
fn relativize(base: &str, p: &Path) -> String {
    let s = p.display().to_string();
    match walk::rel_under(base, &s) {
        Some(r) => r.to_string(),
        None => s.trim_start_matches("./").to_string(),
    }
}

fn add_surface(surfaces: &mut Vec<String>, f: String) {
    if !surfaces.contains(&f) {
        surfaces.push(f);
    }
}

const WS: [char; 5] = [' ', '\t', '\x0b', '\x0c', '\r'];

fn trim_ws(s: &str) -> &str {
    s.trim_matches(WS)
}

// spec: lifecycle-kit/SPEC.md §The close-surface roster — fenced blocks are skipped, so the
// directive's own grammar is quotable where it is specified (check-spec-pointer's carve-out, same
// reason); the lead token is matched full-line, which needs no regex
fn declaration_lines(text: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    let mut fence = false;
    for line in text.lines() {
        let t = line.trim_start_matches(WS);
        if t.starts_with("```") {
            fence = !fence;
            continue;
        }
        if fence {
            continue;
        }
        if let Some(rest) = t.strip_prefix("close-surface:") {
            if rest.starts_with(WS) {
                out.push(line);
            }
        }
    }
    out
}

// spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — the mode is echoed verbatim and a malformed
// one is passed through for check-close-surfaces to rule on, so the split never repairs what the
// gate exists to catch: `reclaim=` runs to end of line, and the mode is whatever follows the path.
fn split_declaration(line: &str) -> (String, String, String) {
    let after = match line.find("close-surface:") {
        Some(i) => &line[i + "close-surface:".len()..],
        None => line,
    };
    let mut body = trim_ws(after);
    let mut reclaim = "-".to_string();
    if let Some(i) = body.find(" reclaim=") {
        reclaim = body[i + " reclaim=".len()..].to_string();
        body = &body[..i];
    }
    let path_end = body.find(WS).unwrap_or(body.len());
    let path = &body[..path_end];
    let mode = trim_ws(&body[path_end..]);
    (path.to_string(), mode.to_string(), reclaim)
}

// spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — the shell form's terminal
// `LC_ALL=C sort -t'\t' -k1,1`, whose missing `-s` makes the whole row the last-resort tie-break.
// Ties are reachable: one path declared on two surfaces is deliberately not collapsed.
fn sort_rows(rows: &mut [String]) {
    rows.sort_by(|a, b| {
        let ka = a.split('\t').next().unwrap_or(a).as_bytes();
        let kb = b.split('\t').next().unwrap_or(b).as_bytes();
        ka.cmp(kb).then_with(|| a.as_bytes().cmp(b.as_bytes()))
    });
}

// spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — the row's on-disk state as three named
// states rather than a size: a section row reads `-` unless its file is absent, a section's
// emptiness being its owner's read
fn row_state(base: &str, locator: &str) -> &'static str {
    let (file, section) = match locator.find('#') {
        Some(i) => (&locator[..i], true),
        None => (locator, false),
    };
    let p = joined(base, file);
    let own = Path::new(&p).exists();
    if section {
        return if own { "-" } else { "absent" };
    }
    // spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — a drain an unfinished triage left
    // behind shows on its log's row, so the next close walks it
    if DRAIN_SUFFIXES.iter().any(|s| holds_content(&format!("{}{}", p, s))) {
        return "non-empty";
    }
    if !own {
        return "absent";
    }
    if holds_content(&p) {
        "non-empty"
    } else {
        "empty"
    }
}

// spec: gate-sdk/SPEC.md §The workflow directory — a capture log's two drain companions
const DRAIN_SUFFIXES: [&str; 2] = [".drain", ".drain.part"];

fn holds_content(p: &str) -> bool {
    match std::fs::read(p) {
        Ok(b) => !stages::header_only(&String::from_utf8_lossy(&b)),
        Err(_) => Path::new(p).exists(),
    }
}

// spec: lifecycle-kit/SPEC.md §The close-surface roster — a drain companion of a declared log is
// part of that log's row, never a surface of its own
fn drain_of_declared(rel: &str, declared: &[String]) -> bool {
    DRAIN_SUFFIXES
        .iter()
        .any(|s| rel.strip_suffix(s).is_some_and(|stem| declared.iter().any(|d| d == stem)))
}

fn row_file(locator: &str) -> &str {
    locator.split('#').next().unwrap_or(locator)
}

fn nul_list(out: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(out)
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_string)
        .collect()
}

// spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — the `<tracking>` value of each file,
// from one `git ls-files` and one `git check-ignore --stdin` over them all
fn tracking(base: &str, files: &[&str]) -> Result<Vec<&'static str>, String> {
    if files.is_empty() {
        return Ok(Vec::new());
    }
    let mut ls = vec!["--literal-pathspecs", "-C", base, "ls-files", "-z", "--"];
    ls.extend(files.iter().copied());
    let listed = proc::run(&programs::GIT, &ls)?;
    let tracked = nul_list(
        listed
            .stdout()
            .ok_or_else(|| format!("git ls-files failed under {}", base))?,
    );
    let input: Vec<u8> = files.iter().flat_map(|f| f.bytes().chain(std::iter::once(0))).collect();
    let ci = proc::run_with_stdin(&programs::GIT, &["-C", base, "check-ignore", "--stdin", "-z"], &input)?;
    let ignored = match ci.code() {
        Some(0) => nul_list(ci.streams().0),
        Some(1) => Vec::new(),
        other => {
            return Err(format!(
                "git check-ignore exited {} under {}",
                other.unwrap_or(-1),
                base
            ))
        }
    };
    Ok(files
        .iter()
        .map(|f| {
            if tracked.iter().any(|t| walk::at_or_under(f.trim_end_matches('/'), t)) {
                "tracked"
            } else if ignored.iter().any(|i| i.trim_end_matches('/') == f.trim_end_matches('/')) {
                "ignored"
            } else {
                "untracked"
            }
        })
        .collect())
}

pub struct Roster {
    pub rows: Vec<String>,
}

pub fn derive(args: &[String]) -> Result<Roster, String> {
    let base = base(args)?;

    let mut surfaces: Vec<String> = Vec::new();
    let roster_basename = walk::knob_scalar("LIFECYCLE_KIT_ROSTER_BASENAME")?;
    // spec: gate-sdk/SPEC.md §Layout and configuration — the root is joined onto a path statted
    // under the base and then carried as the surface's own path, so it takes `kit_roots`
    for r in walk::kit_roots()? {
        if r.is_empty() {
            continue;
        }
        let rel = format!("{}/{}", r.trim_end_matches('/'), roster_basename);
        if Path::new(&joined(&base, &rel)).is_file() {
            add_surface(&mut surfaces, rel);
        }
    }
    let globs = walk::knob_array("LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS")?;
    for p in walk::glob_corpus(Path::new(&base), &globs)? {
        add_surface(&mut surfaces, relativize(&base, &p));
    }

    let mut rows: Vec<String> = Vec::new();
    let mut declared: Vec<String> = Vec::new();
    for s in &surfaces {
        let text = std::fs::read_to_string(joined(&base, s))
            .map_err(|e| format!("declaration surface not readable: {}: {}", s, e))?;
        for line in declaration_lines(&text) {
            let (path, mode, reclaim) = split_declaration(line);
            let state = row_state(&base, &path);
            rows.push(format!("{}\t{}\t{}\t{}\t{}", path, mode, reclaim, s, state));
            declared.push(path);
        }
    }

    // spec: lifecycle-kit/SPEC.md §The close-surface roster — source 2, the closure that makes the
    // roster fail loudly: every gitignored member of the workflow directory is capture-tier by
    // definition, so one added with no declaration appears as `(undeclared)` rather than not at all
    let workflow_dir = walk::knob_scalar("GATE_SDK_WORKFLOW_DIR")?;
    let wf_path = joined(&base, &workflow_dir);
    if Path::new(&wf_path).is_dir() {
        for (name, _) in walk::list_dir(Path::new(&wf_path))? {
            let rel = format!("{}/{}", workflow_dir, name);
            if !Path::new(&joined(&base, &rel)).is_file() || drain_of_declared(&rel, &declared) {
                continue;
            }
            let ci = proc::run(&programs::GIT, &["-C", &base, "check-ignore", "-q", "--", &rel])?;
            match ci.code() {
                Some(0) => {}
                Some(1) => continue,
                other => {
                    return Err(format!(
                        "git check-ignore exited {} on {}",
                        other.unwrap_or(-1),
                        rel
                    ))
                }
            }
            if !declared.contains(&rel) {
                rows.push(format!("{}\t(undeclared)\t-\t-\t{}", rel, row_state(&base, &rel)));
            }
        }
    }

    let files: Vec<&str> = rows
        .iter()
        .map(|r| row_file(r.split('\t').next().unwrap_or("")))
        .collect();
    let values = tracking(&base, &files)?;
    let mut rows: Vec<String> = rows
        .iter()
        .zip(values)
        .map(|(r, t)| format!("{}\t{}", r, t))
        .collect();
    sort_rows(&mut rows);
    Ok(Roster { rows })
}

// spec: gate-sdk/SPEC.md §The non-gate arm — an empty roster prints nothing and the caller reads
// it as zero surfaces: a resolved-empty derivation is an answer, never an error.
pub fn emit(args: &[String]) -> Result<String, String> {
    let r = derive(args)?;
    if r.rows.is_empty() {
        return Ok(String::new());
    }
    let mut out = r.rows.join("\n");
    out.push('\n');
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §The close-surface roster — the fence skip and the full-line
    // lead token, which together decide what counts as a declaration at all
    #[test]
    fn a_fenced_directive_is_quotation_and_a_bare_lead_token_is_not_a_declaration() {
        let text = "close-surface: a.md advisory\n```\nclose-surface: fenced.md advisory\n```\n\
                    close-surface:nospace.md\n  close-surface: b.md advisory\n";
        let got: Vec<&str> = declaration_lines(text);
        assert_eq!(
            got,
            vec!["close-surface: a.md advisory", "  close-surface: b.md advisory"]
        );
    }

    // spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — `reclaim=` runs to end of line, so it
    // splits at its first occurrence and the mode is whatever sits between path and reclaim
    #[test]
    fn reclaim_runs_to_end_of_line_and_the_mode_is_echoed_verbatim() {
        assert_eq!(
            split_declaration("close-surface: .workflow/x.log advisory reclaim=: > a reclaim=b"),
            (
                ".workflow/x.log".to_string(),
                "advisory".to_string(),
                ": > a reclaim=b".to_string()
            )
        );
        assert_eq!(
            split_declaration("close-surface: q.md#S forced=the entry refusal"),
            (
                "q.md#S".to_string(),
                "forced=the entry refusal".to_string(),
                "-".to_string()
            )
        );
        assert_eq!(
            split_declaration("close-surface: bare.md"),
            ("bare.md".to_string(), String::new(), "-".to_string())
        );
    }

    #[test]
    fn a_row_reads_absent_empty_or_non_empty_and_a_section_row_dash_unless_absent() {
        let dir = std::env::temp_dir().join(format!("close-surfaces-state-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mk");
        std::fs::write(dir.join("header.txt"), "# contract: x\n\n").expect("w");
        std::fs::write(dir.join("zero.log"), "").expect("w");
        std::fs::write(dir.join("data.md"), "# contract: x\n- a bullet\n").expect("w");
        let base = dir.display().to_string();
        assert_eq!(row_state(&base, "missing.log"), "absent");
        assert_eq!(row_state(&base, "zero.log"), "empty");
        assert_eq!(row_state(&base, "header.txt"), "empty");
        assert_eq!(row_state(&base, "data.md"), "non-empty");
        assert_eq!(row_state(&base, "data.md#Deferred"), "-");
        assert_eq!(row_state(&base, "missing.md#Deferred"), "absent");
        std::fs::write(dir.join("zero.log.drain"), "left behind\n").expect("w");
        assert_eq!(row_state(&base, "zero.log"), "non-empty", "a left drain shows on its row");
        std::fs::write(dir.join("gone.log.drain.part"), "crashed\n").expect("w");
        assert_eq!(row_state(&base, "gone.log"), "non-empty", "a part shows on an absent log's row");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: lifecycle-kit/SPEC.md §The close-surface roster — only a companion of a declared log folds
    #[test]
    fn a_drain_companion_folds_only_into_a_declared_row() {
        let declared = vec![".workflow/a.log".to_string()];
        assert!(drain_of_declared(".workflow/a.log.drain", &declared));
        assert!(drain_of_declared(".workflow/a.log.drain.part", &declared));
        assert!(!drain_of_declared(".workflow/b.log.drain", &declared));
        assert!(!drain_of_declared(".workflow/a.log", &declared));
    }

    // spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — tracked, ignored and untracked each
    // read as such, and a section row reads its file's value
    #[test]
    fn a_row_reads_tracked_ignored_or_untracked_and_a_section_row_its_files() {
        let dir = std::env::temp_dir().join(format!("close-surfaces-tracking-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mk");
        let base = dir.display().to_string();
        let git = |a: &[&str]| {
            let mut v = vec!["-C", base.as_str()];
            v.extend_from_slice(a);
            proc::run(&programs::GIT, &v).expect("git").stdout().is_some()
        };
        assert!(git(&["init", "-q"]));
        std::fs::write(dir.join(".gitignore"), "cap.log\n").expect("w");
        std::fs::write(dir.join("q.md"), "x\n").expect("w");
        std::fs::write(dir.join("cap.log"), "x\n").expect("w");
        std::fs::write(dir.join("loose.md"), "x\n").expect("w");
        assert!(git(&["add", "q.md", ".gitignore"]));
        let files = ["q.md", row_file("q.md#Deferred"), "cap.log", "loose.md", "missing.md"];
        assert_eq!(
            tracking(&base, &files).expect("tracking"),
            vec!["tracked", "tracked", "ignored", "untracked", "untracked"]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — the tie-break is the whole row and it is
    // observable: two surfaces declaring one path are not collapsed, so path-only ordering would
    // leave the gate's error order at the sorter's discretion
    #[test]
    fn a_tied_path_breaks_on_the_whole_row() {
        let mut rows = vec![
            "p\tadvisory\t-\tz.md".to_string(),
            "a\tadvisory\t-\tz.md".to_string(),
            "p\tadvisory\t-\ta.md".to_string(),
        ];
        sort_rows(&mut rows);
        assert_eq!(
            rows,
            vec![
                "a\tadvisory\t-\tz.md".to_string(),
                "p\tadvisory\t-\ta.md".to_string(),
                "p\tadvisory\t-\tz.md".to_string(),
            ]
        );
    }
}
