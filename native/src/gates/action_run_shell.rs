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

// spec: gate-sdk/SPEC.md §check-action-run-shell — the runner classes the dialect turns on. One
// distinction and only one, because bash is GitHub's default everywhere that is not Windows;
// `NoJob` is the composite action's steps and anything else the job partition never reached
#[derive(Clone, Copy, Default)]
enum Runner {
    NonWindows,
    Windows,
    Unreadable,
    #[default]
    NoJob,
}

enum Item {
    Plain,
    Block {
        n: usize,
        line: usize,
        shell: Option<String>,
        runner: Runner,
    },
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

// spec: gate-sdk/SPEC.md §check-action-run-shell — the key token of a mapping line, which is a
// key only when its colon is followed by a space or by nothing; `a:b` is a plain scalar
fn key_of(s: &str) -> Option<&str> {
    let t = trim_htab(s);
    let i = t.find(':')?;
    let rest = &t[i + 1..];
    if rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t') {
        Some(&t[..i])
    } else {
        None
    }
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — a trailing YAML comment is stripped from the
// captured runner value, so a label carrying one is still matched by the Windows test
fn strip_comment(s: &str) -> &str {
    match s.find(" #") {
        Some(i) => trim_htab(&s[..i]),
        None => s,
    }
}

fn unquote(s: &str) -> String {
    let s = trim_htab(s);
    let s = s
        .strip_prefix('"')
        .and_then(|r| r.strip_suffix('"'))
        .or_else(|| s.strip_prefix('\'').and_then(|r| r.strip_suffix('\'')))
        .unwrap_or(s);
    trim_htab(s).to_lowercase()
}

fn flow_members(s: &str) -> Vec<String> {
    let inner = match (s.find('['), s.rfind(']')) {
        (Some(a), Some(b)) if b > a => &s[a + 1..b],
        _ => s,
    };
    inner
        .split(',')
        .map(unquote)
        .filter(|m| !m.is_empty())
        .collect()
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — a mapping value is a runner group: its
// `labels:` members are the labels, and a mapping carrying `group:` with no `labels:` yields none
fn mapping_labels(parts: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut collecting = false;
    for p in parts {
        let p = trim_htab(p.trim_matches(['{', '}']));
        if let Some(i) = p.find("labels:") {
            let rest = trim_htab(&p[i + "labels:".len()..]);
            collecting = rest.is_empty();
            if rest.starts_with('[') {
                out.extend(flow_members(rest));
            } else if !rest.is_empty() {
                out.push(unquote(rest));
            }
            continue;
        }
        if collecting {
            match p.strip_prefix('-') {
                Some(m) => out.push(unquote(m)),
                None => collecting = false,
            }
        }
    }
    out.retain(|m| !m.is_empty());
    out
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — a scalar value is one label, a sequence (flow
// or block) is its members, a mapping is a runner group
fn labels_of(parts: &[String]) -> Vec<String> {
    let Some(first) = parts.first().map(|p| trim_htab(p)) else {
        return Vec::new();
    };
    if first.starts_with('[') {
        return flow_members(first);
    }
    if first.starts_with('{') || key_of(first).is_some() {
        return mapping_labels(parts);
    }
    if first.starts_with('-') {
        return parts
            .iter()
            .filter_map(|p| trim_htab(p).strip_prefix('-').map(unquote))
            .filter(|m| !m.is_empty())
            .collect();
    }
    let one = unquote(first);
    if one.is_empty() {
        Vec::new()
    } else {
        vec![one]
    }
}

// spec: gate-sdk/SPEC.md §check-action-run-shell — `runs-on` is classified by a Windows test and
// never by a platform roster: enumerating the labels that resolve to bash would be a maintained
// roster of runner labels drifting against a provider's release notes
fn classify(parts: &[String]) -> Runner {
    if parts.iter().any(|p| p.contains("${{")) {
        return Runner::Unreadable;
    }
    let labels = labels_of(parts);
    if labels.is_empty() {
        return Runner::Unreadable;
    }
    if labels
        .iter()
        .any(|l| l == "windows" || l.starts_with("windows-"))
    {
        return Runner::Windows;
    }
    Runner::NonWindows
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
    injobs: bool,
    injob: bool,
    jobcol: i64,
    jobkeycol: i64,
    runson: Vec<String>,
    capturing: bool,
    jobitems: Vec<usize>,
    defcol: i64,
    defline: usize,
}

impl Extractor {
    fn new() -> Self {
        Extractor {
            stepcol: -1,
            bodyindent: -1,
            jobcol: -1,
            jobkeycol: -1,
            defcol: -1,
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

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the step's **explicit** dialect is
    // resolved at the step boundary, not at the block, because the `shell:` sibling key may sit
    // either side of the `run:` block it governs
    fn flushstep(&mut self) {
        let shell = if self.stepshell.is_empty() {
            None
        } else {
            Some(self.stepshell.clone())
        };
        for (n, line) in std::mem::take(&mut self.pend) {
            self.jobitems.push(self.items.len());
            self.items.push(Item::Block {
                n,
                line,
                shell: shell.clone(),
                runner: Runner::NoJob,
            });
        }
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the *inferred* half resolves at the job
    // boundary, not the step's: a job's `runs-on` has the same freedom against the whole `steps:`
    // list that `shell:` has against its own block, so it may not have arrived when a step ends
    fn flushjob(&mut self) {
        let class = if self.injob {
            classify(&self.runson)
        } else {
            Runner::NoJob
        };
        for i in std::mem::take(&mut self.jobitems) {
            if let Item::Block { shell, runner, .. } = &mut self.items[i] {
                if shell.is_none() {
                    *runner = class;
                }
            }
        }
        self.runson.clear();
        self.capturing = false;
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — a column-0 key other than `jobs` closes
    // the job section, and a line at the job column ends the job before it; both end the step
    fn closejob(&mut self) {
        self.flushstep();
        self.stepcol = -1;
        self.stepshell.clear();
        self.flushjob();
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

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the job partition: `jobs:` at column 0
    // opens the section, the first indent under it is the job column, and the first indent under
    // a job id is the job-key column where `runs-on:` is captured
    fn jobstructure(&mut self, line: &str) {
        let kcol = ind(line);
        let dash = dash_prefix_len(line).is_some();
        if kcol == 0 && !dash {
            self.closejob();
            self.injob = false;
            self.jobcol = -1;
            self.jobkeycol = -1;
            self.injobs = key_of(line) == Some("jobs");
            if key_of(line) == Some("defaults") {
                self.defcol = 0;
                self.defline = self.fnr;
            }
            return;
        }
        if !self.injobs {
            return;
        }
        if self.jobcol < 0 && kcol > 0 && !dash {
            self.jobcol = kcol;
        }
        if kcol == self.jobcol && !dash {
            self.closejob();
            self.injob = true;
            self.jobkeycol = -1;
            return;
        }
        if !self.injob || kcol <= self.jobcol {
            return;
        }
        if self.jobkeycol < 0 && !dash {
            self.jobkeycol = kcol;
        }
        if kcol != self.jobkeycol || dash {
            return;
        }
        let rest = substr_from(line, kcol);
        match key_of(&rest) {
            Some("runs-on") => {
                let v = strip_comment(trim_htab(&rest["runs-on:".len()..])).to_string();
                self.runson.clear();
                if v.is_empty() {
                    self.capturing = true;
                } else {
                    self.runson.push(v);
                }
            }
            Some("defaults") => {
                self.defcol = kcol;
                self.defline = self.fnr;
            }
            _ => {}
        }
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
        // spec: gate-sdk/SPEC.md §check-action-run-shell — a `defaults:` subtree carrying a
        // `run:` key is refused: `defaults.run.shell` overrides the runner default for every step
        // beneath it, so a resolver reading `runs-on` alone would state the wrong dialect
        if self.defcol >= 0 {
            if ind(line) > self.defcol {
                let rest = match dash_prefix_len(line) {
                    Some(rl) => substr_from(line, rl as i64),
                    None => substr_from(line, ind(line)),
                };
                if key_of(&rest) == Some("run") {
                    return Err(Refusal {
                        line: self.defline,
                        what: format!(
                            "a defaults: block carrying a run: key (its run: key is at line {})",
                            self.fnr
                        ),
                    });
                }
                return Ok(());
            }
            self.defcol = -1;
        }
        // spec: gate-sdk/SPEC.md §check-action-run-shell — a `runs-on:` value empty on its key
        // line takes the following more-indented lines, so a block sequence and a mapping are
        // captured whole rather than read as absent
        if self.capturing {
            if ind(line) > self.jobkeycol {
                self.runson
                    .push(strip_comment(trim_htab(line)).to_string());
                return Ok(());
            }
            self.capturing = false;
        }
        self.jobstructure(line);
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
        self.closejob();
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

// spec: gate-sdk/SPEC.md §check-action-run-shell — the **explicit** `shell:` value's dialect; an
// absent key resolves at the job boundary and never arrives here, and a dialect ShellCheck has no
// theory of is skipped and counted, never linted as shell, so the empty return is the skip
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
    eprintln!("        balanced. A 'defaults:' subtree carrying a 'run:' key is refused too —");
    eprintln!("        it overrides the runner default for every step beneath it, so name the");
    eprintln!("        dialect on each step instead.");
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
    unresolved: Vec<String>,
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
        unresolved: Vec::new(),
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
                Item::Block {
                    n,
                    line,
                    shell,
                    runner,
                } => {
                    // spec: gate-sdk/SPEC.md §check-action-run-shell — a step's dialect must be
                    // knowable, and where the gate cannot state it the step says it
                    let dialect = match shell {
                        Some(raw) => dialect_of(raw),
                        None => match runner {
                            Runner::NonWindows => "bash",
                            Runner::Windows => {
                                tally.unresolved.push(format!(
                                    "{}:{}: the enclosing job runs on a Windows runner, whose default run: shell is pwsh, and the step names no shell:",
                                    f.display(), line));
                                continue;
                            }
                            Runner::Unreadable => {
                                tally.unresolved.push(format!(
                                    "{}:{}: the enclosing job's runs-on cannot be read, so the dialect cannot be stated, and the step names no shell:",
                                    f.display(), line));
                                continue;
                            }
                            Runner::NoJob => {
                                tally.unresolved.push(format!(
                                    "{}:{}: the step has no enclosing job, so the dialect cannot be stated, and the step names no shell:",
                                    f.display(), line));
                                continue;
                            }
                        },
                    };
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

    if !tally.findings.is_empty() || !tally.unresolved.is_empty() {
        if !tally.findings.is_empty() {
            println!("{}: ShellCheck finding(s) in a workflow run: block — nothing else", NAME);
            println!("in the battery reaches this shell, and it executes only on a tag or a push:");
            for s in &tally.findings {
                println!("  {}", s);
            }
            println!("  help: fix each finding in the workflow's run: body (the line numbers are the");
            println!("        workflow's own), or silence a genuine false positive with an inline");
            println!("        '# shellcheck disable=SCxxxx' plus a justifying comment.");
        }
        // spec: gate-sdk/SPEC.md §Output contract — a gate with more than one failure class gives
        // each its own help: line, and this class's remedy is one key rather than an exemption
        if !tally.unresolved.is_empty() {
            println!("{}: run: block(s) whose shell dialect nothing states — the gate", NAME);
            println!("does not assume a dialect it cannot derive, so these are not linted:");
            for s in &tally.unresolved {
                println!("  {}", s);
            }
            println!("  help: name the step's dialect with a 'shell:' key — 'shell: bash' selects");
            println!("        Git-for-Windows bash on a Windows runner, and 'shell: pwsh' is");
            println!("        skipped and counted as a non-shell dialect.");
        }
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

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the dialect table's two axes read as one
    // value, the `!`-led spellings standing for the rows that are findings rather than dialects
    fn resolved(shell: &Option<String>, runner: &Runner) -> String {
        match shell {
            Some(raw) => dialect_of(raw).to_string(),
            None => match runner {
                Runner::NonWindows => "bash".to_string(),
                Runner::Windows => "!windows".to_string(),
                Runner::Unreadable => "!unreadable".to_string(),
                Runner::NoJob => "!nojob".to_string(),
            },
        }
    }

    fn blocks(text: &str) -> Vec<(String, String)> {
        let Ok(ex) = extract(text) else {
            panic!("the extractor refused a case that should extract")
        };
        ex.items
            .iter()
            .filter_map(|i| match i {
                Item::Block {
                    n, shell, runner, ..
                } => Some((ex.bodies[n - 1].clone(), resolved(shell, runner))),
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
        assert_eq!(
            dialect_of(""),
            "bash",
            "an explicitly empty shell: value is the runner default spelled out"
        );
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

    // spec: gate-sdk/SPEC.md §check-action-run-shell — an absent `shell:` resolves from the
    // enclosing job's `runs-on`, and the Windows job is the finding this member was minted for
    #[test]
    fn an_absent_shell_key_resolves_from_the_job_s_runs_on() {
        let linux = "jobs:\n  j:\n    runs-on: ubuntu-latest\n    steps:\n      - run: |\n          echo hi\n";
        let win = "jobs:\n  j:\n    runs-on: windows-latest\n    steps:\n      - run: |\n          echo hi\n";
        let expr = "jobs:\n  j:\n    runs-on: ${{ matrix.runner }}\n    steps:\n      - run: |\n          echo hi\n";
        let none = "jobs:\n  j:\n    steps:\n      - run: |\n          echo hi\n";
        let composite = "runs:\n  using: composite\n  steps:\n    - run: |\n        echo hi\n";
        assert_eq!(blocks(linux)[0].1, "bash");
        assert_eq!(blocks(win)[0].1, "!windows");
        assert_eq!(blocks(expr)[0].1, "!unreadable");
        assert_eq!(blocks(none)[0].1, "!unreadable");
        assert_eq!(blocks(composite)[0].1, "!nojob");
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — `runs-on` may arrive after the `steps:`
    // list it governs, which is why the inferred half resolves at the job boundary
    #[test]
    fn a_runs_on_key_below_its_own_steps_list_still_governs_them() {
        let below = "jobs:\n  j:\n    steps:\n      - run: |\n          echo hi\n    runs-on: windows-latest\n";
        assert_eq!(blocks(below)[0].1, "!windows", "a trailing runs-on was lost");
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — a line at the job column ends the job
    // before it, so one job's runner cannot leak into the next
    #[test]
    fn each_job_carries_its_own_runner_and_the_next_job_resets_it() {
        let two = "jobs:\n  a:\n    runs-on: windows-latest\n    steps:\n      - run: |\n          echo a\n  b:\n    runs-on: ubuntu-latest\n    steps:\n      - run: |\n          echo b\n";
        let got = blocks(two);
        assert_eq!(got.len(), 2, "expected one block per job: {:?}", got);
        assert_eq!(got[0].1, "!windows");
        assert_eq!(got[1].1, "bash", "the first job's runner leaked into the second");
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — the label forms, and the Windows test that
    // makes any matching label decide: a mixed selector may land on a Windows machine
    #[test]
    fn the_runner_classifier_reads_every_label_form() {
        let seq = "jobs:\n  j:\n    runs-on: [self-hosted, linux]\n    steps:\n      - run: |\n          echo hi\n";
        let blockseq = "jobs:\n  j:\n    runs-on:\n      - self-hosted\n      - Windows-2022\n    steps:\n      - run: |\n          echo hi\n";
        let group = "jobs:\n  j:\n    runs-on:\n      group: my-group\n    steps:\n      - run: |\n          echo hi\n";
        let labelled = "jobs:\n  j:\n    runs-on:\n      group: my-group\n      labels: [self-hosted, windows]\n    steps:\n      - run: |\n          echo hi\n";
        let commented = "jobs:\n  j:\n    runs-on: windows # the release leg\n    steps:\n      - run: |\n          echo hi\n";
        assert_eq!(blocks(seq)[0].1, "bash", "a flow sequence of non-Windows labels");
        assert_eq!(
            blocks(blockseq)[0].1,
            "!windows",
            "a block sequence's Windows member did not decide the job"
        );
        assert_eq!(
            blocks(group)[0].1,
            "!unreadable",
            "a group-only mapping yields no labels, so no dialect can be stated"
        );
        assert_eq!(blocks(labelled)[0].1, "!windows");
        assert_eq!(
            blocks(commented)[0].1,
            "!windows",
            "a trailing YAML comment hid the label"
        );
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — an explicit `shell:` is the step's own
    // answer whatever the job runs on, which is what makes the finding about the step
    #[test]
    fn an_explicit_shell_key_outranks_the_job_s_runner() {
        let win = "jobs:\n  j:\n    runs-on: windows-latest\n    steps:\n      - shell: bash\n        run: |\n          echo hi\n";
        let expr = "jobs:\n  j:\n    runs-on: ${{ matrix.runner }}\n    steps:\n      - shell: pwsh\n        run: |\n          Write-Output hi\n";
        assert_eq!(blocks(win)[0].1, "bash");
        assert_eq!(blocks(expr)[0].1, "", "an explicit pwsh must skip, not become a finding");
    }

    // spec: gate-sdk/SPEC.md §check-action-run-shell — a `defaults:` subtree carrying `run:` is
    // refused at either level, because modelling its third inheritance layer is what the gate
    // declines rather than guesses at
    #[test]
    fn a_defaults_subtree_carrying_run_refuses_at_either_level() {
        let workflow = "defaults:\n  run:\n    shell: pwsh\njobs:\n  j:\n    runs-on: ubuntu-latest\n    steps:\n      - run: |\n          echo hi\n";
        let job = "jobs:\n  j:\n    runs-on: ubuntu-latest\n    defaults:\n      run:\n        shell: pwsh\n    steps:\n      - run: |\n          echo hi\n";
        let unrelated = "defaults:\n  shell: pwsh\njobs:\n  j:\n    runs-on: ubuntu-latest\n    steps:\n      - run: |\n          echo hi\n";
        assert!(extract(workflow).is_err(), "a workflow-level defaults.run was not refused");
        assert!(extract(job).is_err(), "a job-level defaults.run was not refused");
        assert!(
            extract(unrelated).is_ok(),
            "a defaults: block with no run: key is not the refused construct"
        );
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
