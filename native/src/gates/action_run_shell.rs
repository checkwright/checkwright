// spec: gate-sdk/SPEC.md §check-action-run-shell — every GitHub Actions `run:` literal block
// scalar in an Actions-shaped YAML file is ShellCheck-clean at -S warning under the dialect the
// step actually runs, as the wrapper criterion 7's worked example ports to
use crate::proc;
use crate::walk;
use std::path::{Path, PathBuf};

const NAME: &str = "check-action-run-shell";
const PROGRAM: &str = "shellcheck";

// spec: gate-sdk/SPEC.md §Fail-closed contract — the wrapper's own refusal text at the shell
// form's own point in the order: after the scan-root check and before the walk, so a tree with
// no YAML and no linter reports the linter rather than exiting clean on a zero count
fn refuse_absent_program() -> i32 {
    eprintln!("{}: {} not found on PATH — the gate cannot run.", NAME, PROGRAM);
    eprintln!("  A gate that cannot run is not clean (fail-closed).");
    eprintln!("  help: install ShellCheck (e.g. 'apt install shellcheck' / 'brew install shellcheck').");
    2
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — the extractor stays inline: a lib/ helper
// earns its place at a second consumer and there is none. §check-action-gh-repo records why it
// is not that consumer, so the rule follows the code into this module rather than moving.
struct Refusal {
    line: usize,
    what: String,
}

enum Item {
    Plain,
    Block { n: usize, line: usize, shell: String },
}

fn ind(s: &str) -> i64 {
    match s.chars().position(|c| c != ' ') {
        Some(n) => n as i64,
        None => -1,
    }
}

fn is_blank(s: &str) -> bool {
    s.chars().all(|c| c == ' ' || c == '\t')
}

fn is_comment(s: &str) -> bool {
    s.trim_start_matches(' ').starts_with('#')
}

fn dash_prefix_len(s: &str) -> Option<usize> {
    let mut it = s.chars().peekable();
    let mut n = 0usize;
    while it.peek() == Some(&' ') {
        it.next();
        n += 1;
    }
    if it.next() != Some('-') {
        return None;
    }
    n += 1;
    let mut spaces = 0usize;
    while it.peek() == Some(&' ') {
        it.next();
        spaces += 1;
    }
    if spaces == 0 {
        None
    } else {
        Some(n + spaces)
    }
}

fn substr_from(s: &str, n: i64) -> String {
    if n <= 0 {
        return s.to_string();
    }
    s.chars().skip(n as usize).collect()
}

fn trim_htab(s: &str) -> &str {
    s.trim_start_matches([' ', '\t']).trim_end_matches([' ', '\t'])
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — `${{ … }}` is replaced per line by
// `${GHEXPR}`, a braced parameter expansion presenting as the opaque runtime value the
// expression is; `None` is the unbalanced case, which refuses rather than linting a mangled line
fn ghexpr(s: &str) -> Option<String> {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("${{") {
        out.push_str(&rest[..i]);
        out.push_str("${GHEXPR}");
        rest = &rest[i + 3..];
        let j = rest.find("}}")?;
        rest = &rest[j + 2..];
    }
    out.push_str(rest);
    Some(out)
}

#[derive(Default)]
struct Extractor {
    inblock: bool,
    keycol: i64,
    bodyindent: i64,
    buf: Vec<String>,
    blockstart: usize,
    nblk: usize,
    pend: Vec<(usize, usize)>,
    stepcol: i64,
    stepshell: String,
    items: Vec<Item>,
    bodies: Vec<String>,
    fnr: usize,
}

impl Extractor {
    fn new() -> Self {
        Extractor {
            stepcol: -1,
            bodyindent: -1,
            ..Default::default()
        }
    }

    fn startblock(&mut self, col: i64, ln: usize) {
        self.inblock = true;
        self.keycol = col;
        self.bodyindent = -1;
        self.buf.clear();
        self.blockstart = ln;
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the body is dedented by the first body
    // line's indentation with its trailing blank lines dropped, and a body that is entirely
    // blank contributes no block at all
    fn endblock(&mut self) -> Result<(), Refusal> {
        self.inblock = false;
        let mut last = self.buf.len();
        while last > 0 && self.buf[last - 1].is_empty() {
            last -= 1;
        }
        if last == 0 {
            return Ok(());
        }
        self.nblk += 1;
        let mut body = String::new();
        for i in 0..last {
            match ghexpr(&self.buf[i]) {
                Some(sline) => {
                    body.push_str(&sline);
                    body.push('\n');
                }
                None => {
                    return Err(Refusal {
                        line: self.blockstart + 1 + i,
                        what: "an unbalanced GitHub expression — ${{ with no closing }} on a run: body line".to_string(),
                    })
                }
            }
        }
        self.bodies.push(body);
        let n = self.nblk;
        let start = self.blockstart;
        self.pend.push((n, start));
        Ok(())
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the step's dialect is resolved at the
    // step boundary, not at the block, because the `shell:` sibling key may sit either side of
    // the `run:` block it governs
    fn flushstep(&mut self) {
        let shell = self.stepshell.clone();
        for (n, line) in std::mem::take(&mut self.pend) {
            self.items.push(Item::Block {
                n,
                line,
                shell: shell.clone(),
            });
        }
    }

    fn processkey(&mut self, rest: &str, col: i64) -> Result<(), Refusal> {
        if let Some(after) = rest.strip_prefix("shell:") {
            if after.is_empty() || after.starts_with(' ') || after.starts_with('\t') {
                self.stepshell = trim_htab(after).to_string();
                return Ok(());
            }
        }
        let Some(after) = rest.strip_prefix("run:") else {
            return Ok(());
        };
        let v = trim_htab(after).to_string();
        let refuse = |what: String| Err(Refusal { line: self.fnr, what });
        if v.starts_with('>') {
            return refuse(format!("a folded block scalar (run: {})", v));
        }
        if v.starts_with('|') && v[1..].starts_with(|c: char| c.is_ascii_digit()) {
            return refuse(format!(
                "an explicit block-scalar indentation indicator (run: {})",
                v
            ));
        }
        if v.starts_with('*') {
            return refuse(format!("a YAML alias as the run: value (run: {})", v));
        }
        if v.starts_with('&') {
            return refuse(format!("a YAML anchor on the run: value (run: {})", v));
        }
        if v == "|" || v == "|-" || v == "|+" {
            self.startblock(col, self.fnr);
            return Ok(());
        }
        if v.is_empty() {
            return Ok(());
        }
        self.items.push(Item::Plain);
        Ok(())
    }

    fn feed(&mut self, line: &str) -> Result<(), Refusal> {
        self.fnr += 1;
        // spec: gate-sdk/SPEC.md §check-action-run-shell — no block header is recognised while
        // inside a block, so a body containing a heredoc whose text is literally `run: |` stays
        // shell instead of being double-extracted
        if self.inblock {
            if is_blank(line) {
                self.buf.push(String::new());
                return Ok(());
            }
            let bcol = ind(line);
            if bcol > self.keycol {
                if self.bodyindent < 0 {
                    self.bodyindent = bcol;
                }
                let dedented = substr_from(line, self.bodyindent);
                self.buf.push(dedented);
                return Ok(());
            }
            self.endblock()?;
        }
        if is_blank(line) {
            return Ok(());
        }
        // spec: gate-sdk/SPEC.md §check-action-run-shell — a comment line is never a header
        if is_comment(line) {
            return Ok(());
        }
        // spec: gate-sdk/SPEC.md §check-action-run-shell — the key column is the column of the
        // key token, never the list dash: taking the dash's column swallows every sibling key of
        // the step into the shell body, which is a false-positive engine rather than a miss
        if let Some(rl) = dash_prefix_len(line) {
            self.flushstep();
            self.stepcol = rl as i64;
            self.stepshell.clear();
            let rest = substr_from(line, rl as i64);
            return self.processkey(&rest, rl as i64);
        }
        let kcol = ind(line);
        if self.stepcol >= 0 && kcol < self.stepcol {
            self.flushstep();
            self.stepcol = -1;
            self.stepshell.clear();
        }
        if self.stepcol >= 0 && kcol == self.stepcol {
            let rest = substr_from(line, kcol);
            return self.processkey(&rest, kcol);
        }
        if self.stepcol < 0 {
            let tail = substr_from(line, kcol);
            if tail.starts_with("run:") {
                self.stepcol = kcol;
                self.stepshell.clear();
                return self.processkey(&tail, kcol);
            }
        }
        Ok(())
    }

    fn finish(&mut self) -> Result<(), Refusal> {
        if self.inblock {
            self.endblock()?;
        }
        self.flushstep();
        Ok(())
    }
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — awk's record split, which `str::lines()` is
// not: it strips a carriage return awk keeps in `$0`, and a file's final newline terminates its
// last record rather than opening an empty one
fn records(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    let body = text.strip_suffix('\n').unwrap_or(text);
    body.split('\n').collect()
}

fn extract(text: &str) -> Result<Extractor, Refusal> {
    let mut ex = Extractor::new();
    for line in records(text) {
        ex.feed(line)?;
    }
    ex.finish()?;
    Ok(ex)
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — absent resolves to bash on GitHub's
// documented runner default; a dialect ShellCheck has no theory of is skipped and counted, never
// linted as shell, so the empty string here is the skip rather than a fallback
fn dialect_of(raw: &str) -> &str {
    let first = raw.split([' ', '\t', '\n', '\r']).next().unwrap_or("");
    let first = first.strip_prefix('"').unwrap_or(first);
    let first = first.strip_suffix('"').unwrap_or(first);
    let first = first.strip_prefix('\'').unwrap_or(first);
    let first = first.strip_suffix('\'').unwrap_or(first);
    match first {
        "" | "bash" => "bash",
        "sh" | "dash" | "ksh" => first,
        _ => "",
    }
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — the Actions-shape predicate governs
// extraction and refusal alike: a file it skips is neither linted nor refused, because `run:` is
// an ordinary word serving as a key in more than one CI schema
fn actions_shaped(text: &str) -> bool {
    records(text)
        .iter()
        .any(|l| l.starts_with("jobs:") || l.starts_with("runs:"))
}

fn print_refusal(file: &Path, r: &Refusal) -> i32 {
    eprintln!("{}: the extractor met a construct it does not handle, so it", NAME);
    eprintln!("refuses rather than linting a mangled fragment (fail-closed):");
    eprintln!("  {}:{}: {}", file.display(), r.line, r.what);
    eprintln!("  help: a multi-line run: body in an Actions-shaped file must be a literal block");
    eprintln!("        scalar written 'run: |' (or '|-' / '|+'), with no explicit indentation");
    eprintln!("        indicator and no YAML anchor or alias, and every ${{{{ }}}} on a body line");
    eprintln!("        balanced.");
    2
}

struct Tally {
    walked: usize,
    subject: usize,
    skipped_files: usize,
    linted: usize,
    plain: usize,
    skipped_dialect: usize,
    findings: Vec<String>,
}

fn lint_block(
    frag: &Path,
    dialect: &str,
    file: &Path,
    blockstart: usize,
    tally: &mut Tally,
) -> Result<(), i32> {
    let frag_s = frag.display().to_string();
    let merged = match proc::run_merged(PROGRAM, &["-f", "gcc", "-S", "warning", "-s", dialect, &frag_s])
    {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return Err(2);
        }
    };
    let rc = merged.code().unwrap_or(-1);
    if !(0..=1).contains(&rc) {
        eprintln!(
            "{}: shellcheck exited {} on {} (block at line {})",
            NAME,
            rc,
            file.display(),
            blockstart
        );
        return Err(2);
    }
    if rc == 0 {
        return Ok(());
    }
    let out = String::from_utf8_lossy(merged.output()).into_owned();
    let prefix = format!("{}:", frag_s);
    for hit in records(out.trim_end_matches('\n')) {
        if hit.is_empty() {
            continue;
        }
        let rest = hit.strip_prefix(prefix.as_str()).unwrap_or(hit);
        let head = rest.split(':').next().unwrap_or("");
        let numeric = !head.is_empty() && head.bytes().all(|b| b.is_ascii_digit());
        match head.parse::<usize>() {
            Ok(fline) if numeric => {
                let tail = rest.split_once(':').map(|(_, t)| t).unwrap_or(rest);
                tally
                    .findings
                    .push(format!("{}:{}:{}", file.display(), blockstart + fline, tail));
            }
            _ => tally.findings.push(format!(
                "{} (run: block at line {}): {}",
                file.display(),
                blockstart,
                hit
            )),
        }
    }
    Ok(())
}

fn scan(files: &[PathBuf], work: &Path) -> Result<Tally, i32> {
    let mut tally = Tally {
        walked: 0,
        subject: 0,
        skipped_files: 0,
        linted: 0,
        plain: 0,
        skipped_dialect: 0,
        findings: Vec::new(),
    };
    for f in files {
        tally.walked += 1;
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "{}: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                    NAME,
                    f.display(),
                    e
                );
                return Err(2);
            }
        };
        if !actions_shaped(&text) {
            tally.skipped_files += 1;
            continue;
        }
        tally.subject += 1;
        let ex = match extract(&text) {
            Ok(ex) => ex,
            Err(r) => return Err(print_refusal(f, &r)),
        };
        let fdir = work.join(format!("f{}", tally.subject));
        if let Err(e) = std::fs::create_dir_all(&fdir) {
            eprintln!("{}: could not create a scratch dir ({})", NAME, e);
            return Err(2);
        }
        for item in &ex.items {
            match item {
                Item::Plain => tally.plain += 1,
                Item::Block { n, line, shell } => {
                    let dialect = dialect_of(shell);
                    if dialect.is_empty() {
                        tally.skipped_dialect += 1;
                        continue;
                    }
                    tally.linted += 1;
                    let frag = fdir.join(format!("block-{}.sh", n));
                    if let Err(e) = std::fs::write(&frag, ex.bodies[n - 1].as_bytes()) {
                        eprintln!("{}: could not write {} ({})", NAME, frag.display(), e);
                        return Err(2);
                    }
                    lint_block(&frag, dialect, f, *line, &mut tally)?;
                }
            }
        }
    }
    Ok(tally)
}

pub fn run(args: &[String]) -> i32 {
    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(scanroot).is_dir() {
        eprintln!("{}: scan root not found: {}", NAME, scanroot);
        return 2;
    }

    if !proc::on_path(PROGRAM) {
        return refuse_absent_program();
    }

    let files = match walk::find_files(Path::new(scanroot), &["yml", "yaml"]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{}: {} — the check could not run; treating as failure (not clean)",
                NAME, e
            );
            return 2;
        }
    };

    if files.is_empty() {
        println!(
            "ACTION-RUN-SHELL: clean (no YAML under {} — 0 run: block(s) to lint)",
            scanroot
        );
        return 0;
    }

    let work = std::env::temp_dir().join(format!("checkwright-runshell.{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    if let Err(e) = std::fs::create_dir_all(&work) {
        eprintln!("{}: could not create a scratch dir ({})", NAME, e);
        return 2;
    }
    let outcome = scan(&files, &work);
    let _ = std::fs::remove_dir_all(&work);

    let tally = match outcome {
        Ok(t) => t,
        Err(code) => return code,
    };

    if !tally.findings.is_empty() {
        println!("{}: ShellCheck finding(s) in a workflow run: block — nothing else", NAME);
        println!("in the battery reaches this shell, and it executes only on a tag or a push:");
        for s in &tally.findings {
            println!("  {}", s);
        }
        println!("  help: fix each finding in the workflow's run: body (the line numbers are the");
        println!("        workflow's own), or silence a genuine false positive with an inline");
        println!("        '# shellcheck disable=SCxxxx' plus a justifying comment.");
        return 1;
    }

    println!(
        "ACTION-RUN-SHELL: clean ({} run: block(s) linted at -S warning across {} Actions-shaped file(s) of {} walked; {} file(s) skipped by the Actions-shape predicate, {} plain-scalar run: value(s) skipped, {} block(s) skipped on a non-shell dialect)",
        tally.linted, tally.subject, tally.walked, tally.skipped_files, tally.plain, tally.skipped_dialect
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blocks(text: &str) -> Vec<(String, String)> {
        let Ok(ex) = extract(text) else {
            panic!("the extractor refused a case that should extract")
        };
        ex.items
            .iter()
            .filter_map(|i| match i {
                Item::Block { n, shell, .. } => {
                    Some((ex.bodies[n - 1].clone(), dialect_of(shell).to_string()))
                }
                Item::Plain => None,
            })
            .collect()
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the key column is the column of the key
    // token, never the list dash: the pair proves the swallow does not happen by staying clean,
    // and this proves what was extracted rather than that nothing complained
    #[test]
    fn a_dash_inline_block_does_not_swallow_its_step_s_sibling_keys() {
        let yaml = "jobs:\n  j:\n    steps:\n      - run: |\n          echo body\n        env:\n          FOO: bar\n        name: the step's own name\n";
        let got = blocks(yaml);
        assert_eq!(got.len(), 1, "expected exactly one block: {:?}", got);
        assert_eq!(
            got[0].0, "echo body\n",
            "the sibling keys of the step were swallowed into its shell body"
        );
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the `shell:` sibling key may sit either
    // side of the `run:` block it governs, which is why the dialect resolves at the step boundary
    #[test]
    fn the_shell_key_resolves_from_either_side_of_its_own_block() {
        let before = "jobs:\n  j:\n    steps:\n      - shell: sh\n        run: |\n          echo hi\n";
        let after = "jobs:\n  j:\n    steps:\n      - run: |\n          echo hi\n        shell: sh\n";
        assert_eq!(blocks(before)[0].1, "sh", "a leading shell: key was lost");
        assert_eq!(blocks(after)[0].1, "sh", "a trailing shell: key was lost");
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the dialect table, including the skip
    // arm: an unknown dialect yields the empty string, which the caller counts rather than lints
    #[test]
    fn the_dialect_table_maps_every_arm_including_the_skip() {
        assert_eq!(dialect_of(""), "bash", "an absent shell: key must be bash");
        assert_eq!(dialect_of("bash --noprofile {0}"), "bash");
        assert_eq!(dialect_of("\"sh\""), "sh", "a quoted dialect was not unquoted");
        assert_eq!(dialect_of("'dash'"), "dash");
        assert_eq!(dialect_of("ksh"), "ksh");
        assert_eq!(dialect_of("pwsh"), "", "a non-shell dialect must skip, not lint");
        assert_eq!(dialect_of("python"), "");
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — `${{ … }}` becomes a braced parameter
    // expansion, and an unbalanced one is the refusal rather than a mangled line
    #[test]
    fn a_github_expression_becomes_a_braced_expansion_and_an_unbalanced_one_refuses() {
        assert_eq!(
            ghexpr("echo \"${{ github.ref_name }}\"").as_deref(),
            Some("echo \"${GHEXPR}\"")
        );
        assert_eq!(ghexpr("echo \"${{ github.ref").as_deref(), None);
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — awk's record split rather than
    // `str::lines()`: a carriage return stays in the record, and a trailing newline terminates
    #[test]
    fn a_record_keeps_its_carriage_return_and_a_final_newline_opens_no_record() {
        assert_eq!(records("a\r\nb\n"), vec!["a\r", "b"]);
        assert_eq!(records(""), Vec::<&str>::new());
        assert_eq!(records("\n"), vec![""]);
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the Actions-shape predicate reads a
    // top-level key, so an indented `jobs:` in a foreign schema leaves the file out of subject
    #[test]
    fn the_actions_shape_predicate_reads_a_top_level_key_only() {
        assert!(actions_shaped("name: x\njobs:\n  j: {}\n"));
        assert!(actions_shaped("runs:\n  using: composite\n"));
        assert!(
            !actions_shaped("version: 2\nworkflows:\n  jobs:\n    - a\n"),
            "an indented jobs: key pulled a foreign schema into the subject"
        );
    }
}
