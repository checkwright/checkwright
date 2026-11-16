// spec: evidence-kit/SPEC.md §The evidence adapters — evidence-kit's adapters and the readers its
// gates share, kept here rather than reached for in `crate::stages` so the kit stays independent of
// lifecycle-kit
use crate::programs;

// spec: evidence-kit/SPEC.md §Evidence manifest — the major the writer writes, and the one legacy
// major the gate still reads and the fold rewrites
pub const MANIFEST_CONTRACT: &str = "evidence-manifest v2";
pub const MANIFEST_CONTRACT_V1: &str = "evidence-manifest v1";

#[derive(Debug, PartialEq)]
pub enum Header {
    V2,
    V1,
    Unknown(String),
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the header names the grammar; a first line naming
// neither major is no grammar this reader may guess
pub fn manifest_header(text: &str) -> Header {
    let first = text.lines().next().unwrap_or("");
    match first.strip_prefix("# contract: ") {
        Some(MANIFEST_CONTRACT) => Header::V2,
        Some(MANIFEST_CONTRACT_V1) => Header::V1,
        _ => Header::Unknown(first.to_string()),
    }
}

pub fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

pub fn is_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the date's digit shape, never a calendar parse
pub fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 10 {
        return false;
    }
    b.iter().enumerate().all(|(i, c)| {
        if i == 4 || i == 7 {
            *c == b'-'
        } else {
            c.is_ascii_digit()
        }
    })
}

// spec: evidence-kit/SPEC.md §check-evidence-manifest — everything but a comment line and a blank
// one. Distinct from `crate::stages::data_lines` — same name,
// different primitive, and that section owns why binding to the other one is silent.
pub fn data_lines(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|l| {
            let t = l.trim_start_matches([' ', '\t']);
            !t.is_empty() && !t.starts_with('#')
        })
        .collect()
}

// spec: lifecycle-kit/SPEC.md §The state machine — the queue header's iteration, a residual
// trailing `[stage:` field stripped
pub fn queue_iteration(text: &str) -> Option<String> {
    let hdr = text.lines().find(|l| l.starts_with("## Iteration:"))?;
    let mut s = hdr.strip_prefix("## Iteration:").unwrap_or(hdr);
    s = s.trim_start_matches([' ', '\t']);
    Some(match s.find("[stage:") {
        Some(i) => s[..i].trim_end_matches([' ', '\t']).to_string(),
        None => s.to_string(),
    })
}

// spec: lifecycle-kit/SPEC.md §The state machine — the evidence file's data lines, below its `---`
// separator
pub fn state_lines(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut seen = false;
    for line in text.lines() {
        if !seen {
            if line.starts_with("---") && line[3..].chars().all(|c| c == ' ' || c == '\t') {
                seen = true;
            }
            continue;
        }
        if line.split_whitespace().next().is_some() {
            out.push(line);
        }
    }
    out
}

// spec: evidence-kit/SPEC.md §The evidence adapters — the cursor: the last data line's stage, `None`
// wherever the reader answers no stage
pub fn state_stage(text: &str) -> Option<String> {
    let last = state_lines(text).last().copied()?;
    last.split_whitespace().nth(1).map(String::from)
}

// spec: evidence-kit/SPEC.md §check-evidence-manifest — (A)'s date window: the earliest validate
// stamp's date for the iteration, `None` where it has none
pub fn earliest_validate(state_text: &str, iter: &str) -> Option<String> {
    let mut earliest: Option<String> = None;
    for line in state_lines(state_text) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.first().copied().unwrap_or("") != iter || f.get(1).copied().unwrap_or("") != "validate" {
            continue;
        }
        let d = f.get(3).copied().unwrap_or("");
        if earliest.as_deref().map_or(true, |e| e.is_empty() || d < e) {
            earliest = Some(d.to_string());
        }
    }
    earliest
}

// spec: evidence-kit/SPEC.md §The evidence adapters — the unnamed-iteration placeholder, which names
// no run key
const UNNAMED: &str = "—";

// spec: evidence-kit/SPEC.md §The evidence adapters — the run key; `None` is the no-key case the
// spine refuses at the guards' exit 2
pub fn run_key(queue_text: Option<&str>, run_id: &str) -> Option<String> {
    if let Some(iter) = queue_text.and_then(queue_iteration) {
        if !iter.is_empty() && iter != UNNAMED {
            return Some(iter);
        }
    }
    if run_id.is_empty() {
        return None;
    }
    Some(run_id.to_string())
}

// spec: evidence-kit/SPEC.md §The evidence adapters — the suite's configured run command, looked up
// in the `EVIDENCE_KIT_RUN_` family by suite
pub fn suite_cmd(run_family: &[(String, String)], suite: &str) -> String {
    crate::walk::knob_in_family(run_family, suite).unwrap_or_default()
}

// spec: evidence-kit/SPEC.md §Layout and configuration — the per-suite override ahead of the global
// knob, and an override resolving *empty* falls through to the global exactly as an unset one does.
pub fn parser_for(parser_family: &[(String, String)], suite: &str, global: &str) -> String {
    match crate::walk::knob_in_family(parser_family, suite) {
        Some(v) if !v.is_empty() => v,
        _ => global.to_string(),
    }
}

// spec: evidence-kit/SPEC.md §The evidence adapters — the parser's three arms: the two bundled
// adapters and, for any other value, the consumer command word-split and spawned with the log
// appended last, its exit status unread
pub fn parse(
    suite: &str,
    log: &std::path::Path,
    status: i32,
    parser: &str,
) -> Result<Vec<String>, String> {
    match parser {
        "exit-code" => Ok(vec![format!(
            "{} {}",
            suite,
            if status == 0 { "pass" } else { "fail" }
        )]),
        "libtest" => {
            let text = std::fs::read(log)
                .map(|b| String::from_utf8_lossy(&b).into_owned())
                .unwrap_or_default();
            Ok(libtest_lines(&text))
        }
        _ => {
            let mut words: Vec<&str> = parser.split_whitespace().collect();
            if words.is_empty() {
                return Ok(Vec::new());
            }
            let program = programs::Program::consumer("EVIDENCE_KIT_PARSER", words.remove(0));
            let display = log.display().to_string();
            words.push(&display);
            let out = crate::proc::run_streamed(&program, &words, b"", crate::proc::Stderr::Inherit)?;
            Ok(String::from_utf8_lossy(out.stdout())
                .lines()
                .map(String::from)
                .collect())
        }
    }
}

// spec: evidence-kit/SPEC.md §The evidence adapters — the `libtest` adapter: one scenario per
// per-test result line
fn libtest_lines(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        if !line.starts_with("test ") || !line.contains(" ... ") {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        let (Some(name), Some(res)) = (fields.get(1), fields.last()) else {
            continue;
        };
        match *res {
            "ok" => out.push(format!("{} pass", name)),
            "FAILED" => out.push(format!("{} fail", name)),
            "ignored" => out.push(format!("{} ignore", name)),
            _ => {}
        }
    }
    out
}

// spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — the shared per-scenario diff's findings and its
// verdict, non-zero the moment a new failure fires
pub struct Diff {
    pub findings: Vec<String>,
    pub new_failure: bool,
}

// spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — a baseline `pass` scenario red or
// absent is a new failure, a `fail`/`ignore` one observed green an unpromoted recovery, an observed
// `fail` with no baseline row a new failure; the skip demotion runs before the pass/fail branch.
pub fn diff(baseline_text: &str, suite: &str, observed_text: &str, skip_text: &str) -> Diff {
    let mut obs: Vec<(String, String)> = Vec::new();
    for line in observed_text.lines() {
        let mut f = line.split_whitespace();
        let Some(sc) = f.next() else { continue };
        let st = f.next().unwrap_or("");
        match obs.iter_mut().find(|(k, _)| k == sc) {
            Some(slot) => slot.1 = st.to_string(),
            None => obs.push((sc.to_string(), st.to_string())),
        }
    }
    let mut skip: Vec<&str> = Vec::new();
    for line in skip_text.lines() {
        let mut f = line.split_whitespace();
        if let (Some(f1), Some(f2)) = (f.next(), f.next()) {
            if f1 == suite {
                skip.push(f2);
            }
        }
    }

    let mut out = Diff {
        findings: Vec::new(),
        new_failure: false,
    };
    let mut seen: Vec<&str> = Vec::new();
    for line in data_lines(baseline_text) {
        let f: Vec<&str> = line.split_whitespace().collect();
        let (Some(bsuite), Some(bscen)) = (f.first().copied(), f.get(1).copied()) else {
            continue;
        };
        let bstat = f.get(2).copied().unwrap_or("");
        if bsuite != suite {
            continue;
        }
        seen.push(bscen);
        let mut cur = obs
            .iter()
            .find(|(k, _)| k == bscen)
            .map(|(_, v)| v.as_str())
            .unwrap_or("absent");
        if skip.contains(&bscen) && cur == "pass" {
            cur = "skip";
        }
        if bstat == "pass" {
            if cur != "pass" {
                out.findings.push(format!("new-failure {} {}", suite, bscen));
                out.new_failure = true;
            }
        } else if cur == "pass" {
            out.findings.push(format!("recovery {} {}", suite, bscen));
        }
    }

    // spec: evidence-kit/SPEC.md §Baseline manifest — an observed failure absent from the baseline
    // is a new failure; the rule is `fail`, never non-pass: an absent `pass` is the stated
    // classification cost and an absent `ignore` is a non-verdict, neither a red.
    for (sc, st) in &obs {
        if seen.contains(&sc.as_str()) {
            continue;
        }
        seen.push(sc.as_str());
        if st == "fail" {
            out.findings.push(format!("new-failure {} {}", suite, sc));
            out.new_failure = true;
        }
    }
    out
}

pub const STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";
pub const PREDICATE_TYPE: &str = "https://checkwright.dev/gate-run/v1";
pub const PREDICATE_CONTRACT: &str = "1.0.0";
pub const DSSE_PAYLOAD_TYPE: &str = "application/vnd.in-toto+json";

fn words(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

// spec: evidence-kit/SPEC.md §The gate-run predicate — the command digest: the spawned argv of the
// run command and the effective parser, whitespace-normalized, the pre-hook left out
pub fn command_digest(run: &str, parser: &str) -> String {
    crate::sha256::hex(format!("run {}\nparser {}\n", words(run), words(parser)).as_bytes())
}

// spec: evidence-kit/SPEC.md §The gate-run predicate — a suite's command digest as this tree
// resolves it; `None` for a suite with no run command
pub fn resolved_command_digest(
    run_family: &[(String, String)],
    parser_family: &[(String, String)],
    global_parser: &str,
    suite: &str,
) -> Option<String> {
    let run = suite_cmd(run_family, suite);
    if run.trim().is_empty() {
        return None;
    }
    Some(command_digest(&run, &parser_for(parser_family, suite, global_parser)))
}

// spec: evidence-kit/SPEC.md §The gate-run predicate — the binding digest over the newline-terminated
// lines that join the command, the source commit and the counts
pub fn binding_digest(
    suite: &str,
    cmd: &str,
    commit: &str,
    counts: (u64, u64, u64),
    verdict: &str,
    log: &str,
) -> String {
    let text = format!(
        "gate-run-binding v1\n{}\n{}\n{}\n{} {} {}\n{}\n{}\n",
        suite, cmd, commit, counts.0, counts.1, counts.2, verdict, log
    );
    crate::sha256::hex(text.as_bytes())
}

// spec: evidence-kit/SPEC.md §Evidence manifest — one v2 suite line's fields
#[derive(Clone, Debug, PartialEq)]
pub struct SuiteLine {
    pub key: String,
    pub suite: String,
    pub log: String,
    pub counts: (u64, u64, u64),
    pub verdict: String,
    pub date: String,
    pub commit: String,
    pub tree: String,
    pub cmd: String,
    pub origin: String,
}

impl SuiteLine {
    pub fn render(&self) -> String {
        format!(
            "{} {} sha256={} pass={} fail={} ignore={} verdict={} {} commit={} tree={} cmd={} origin={}",
            self.key,
            self.suite,
            self.log,
            self.counts.0,
            self.counts.1,
            self.counts.2,
            self.verdict,
            self.date,
            self.commit,
            self.tree,
            self.cmd,
            self.origin
        )
    }

    pub fn binding(&self) -> String {
        binding_digest(&self.suite, &self.cmd, &self.commit, self.counts, &self.verdict, &self.log)
    }
}

// spec: evidence-kit/SPEC.md §Test controls — one declared control
#[derive(Clone, Debug, PartialEq)]
pub struct Control {
    pub name: String,
    pub suite: String,
    pub scenario: String,
    pub env: Vec<(String, String)>,
}

// spec: evidence-kit/SPEC.md §Test controls — the assignments digest the spine writes and (C) recomputes
pub fn assignments_digest(control: &Control) -> String {
    let text: String = control.env.iter().map(|(k, v)| format!("{}={}\n", k, v)).collect();
    crate::sha256::hex(text.as_bytes())
}

pub const CONTROL_PREFIX: &str = "EVIDENCE_KIT_CONTROL_";
const CONTROL_SINCE: &str = "SINCE";

fn is_identifier(s: &str) -> bool {
    let mut c = s.chars();
    matches!(c.next(), Some(h) if h.is_ascii_alphabetic() || h == '_')
        && c.all(|t| t.is_ascii_alphanumeric() || t == '_')
}

// spec: evidence-kit/SPEC.md §Test controls — the one declaration parser `--run-validate`'s guards
// and `check-test-control` (A) share: the well-formed controls, and a finding per malformed one
pub fn controls(
    names: &[String],
    family: &[(String, String)],
    suites: &[String],
) -> (Vec<Control>, Vec<String>) {
    let mut out = Vec::new();
    let mut errors = Vec::new();
    for name in names {
        if name == CONTROL_SINCE {
            errors.push(format!(
                "control '{}' collides with {}{}, the since-commit knob, so it can have no member — rename the control",
                name, CONTROL_PREFIX, CONTROL_SINCE
            ));
            continue;
        }
        let Some(value) = crate::walk::knob_in_family(family, name) else {
            errors.push(format!("control '{}' has no {}{} declaration", name, CONTROL_PREFIX, name));
            continue;
        };
        let words: Vec<&str> = value.split_whitespace().collect();
        let (Some(suite), Some(scenario)) = (words.first(), words.get(1)) else {
            errors.push(format!(
                "control '{}' is '{}' — want <suite> <scenario> <VAR>=<value> [<VAR>=<value> ...]",
                name, value
            ));
            continue;
        };
        let mut bad = false;
        if !suites.iter().any(|s| s == suite) {
            errors.push(format!("control '{}' names suite '{}', which is not in EVIDENCE_KIT_SUITES", name, suite));
            bad = true;
        }
        if words.len() < 3 {
            errors.push(format!("control '{}' carries no <VAR>=<value> assignment", name));
            bad = true;
        }
        let mut env = Vec::new();
        for w in &words[2.min(words.len())..] {
            match w.split_once('=') {
                Some((k, v)) if is_identifier(k) => env.push((k.to_string(), v.to_string())),
                _ => {
                    errors.push(format!(
                        "control '{}' carries '{}', which is not a <identifier>=<value> assignment",
                        name, w
                    ));
                    bad = true;
                }
            }
        }
        if !bad {
            out.push(Control { name: name.clone(), suite: suite.to_string(), scenario: scenario.to_string(), env });
        }
    }
    (out, errors)
}

// spec: evidence-kit/SPEC.md §Test controls — the scenario's last observed status as the witness
pub fn witness(parsed: &[String], scenario: &str) -> &'static str {
    let status = parsed.iter().rev().find_map(|l| {
        let mut f = l.split_whitespace();
        (f.next() == Some(scenario)).then(|| f.next().unwrap_or(""))
    });
    match status {
        Some("fail") => "red",
        Some("pass") => "green",
        _ => "absent",
    }
}

// spec: evidence-kit/SPEC.md §Evidence manifest — one v2 control line's fields
#[derive(Clone, Debug, PartialEq)]
pub struct ControlLine {
    pub key: String,
    pub suite: String,
    pub name: String,
    pub scenario: String,
    pub witnessed: String,
    pub date: String,
    pub commit: String,
    pub cmd: String,
    pub assign: String,
}

impl ControlLine {
    pub fn render(&self) -> String {
        format!(
            "{} {} control={} scenario={} witnessed={} {} commit={} cmd={} assign={}",
            self.key,
            self.suite,
            self.name,
            self.scenario,
            self.witnessed,
            self.date,
            self.commit,
            self.cmd,
            self.assign
        )
    }
}

// spec: evidence-kit/SPEC.md §check-evidence-baseline — the baseline as committed at `commit`, which
// `label` names in a refusal: empty where that tree holds no such file, and Err on a git failure
// listing or reading it, never an empty read
pub fn baseline_at(commit: &str, label: &str, baseline: &str) -> Result<String, String> {
    let closed = |what: String| format!("{} — the check could not run; treating as failure (not clean)", what);
    let listed = crate::listing::names(None, &["ls-tree", "-z", "--full-name", "--name-only", commit, "--", baseline])
        .map_err(|e| e.text_or(|_| closed(format!("cannot list {} at {} {}", baseline, label, commit))))?;
    let name = listed.into_iter().next().unwrap_or_default();
    if name.is_empty() {
        return Ok(String::new());
    }
    let shown = crate::proc::run(&programs::GIT, &["show", &format!("{}:{}", commit, name)])?;
    match shown.stdout() {
        Some(b) => Ok(String::from_utf8_lossy(b).into_owned()),
        None => Err(closed(format!("cannot read {} at {} {}", name, label, commit))),
    }
}

// spec: evidence-kit/SPEC.md §The gate-run predicate — the Statement, keys sorted by the map itself
// and no insignificant whitespace, so the same run gives the same bytes
pub fn statement(run: &str, subject: &str, provenance: serde_json::Value, suites: &[SuiteLine]) -> String {
    use serde_json::json;
    let suites: Vec<serde_json::Value> = suites
        .iter()
        .map(|s| {
            json!({
                "name": s.suite,
                "command": { "sha256": s.cmd },
                "log": { "sha256": s.log },
                "counts": { "pass": s.counts.0, "fail": s.counts.1, "ignore": s.counts.2 },
                "verdict": s.verdict,
                "date": s.date,
                "tree": s.tree,
                "binding": { "sha256": s.binding() },
            })
        })
        .collect();
    let v = json!({
        "_type": STATEMENT_TYPE,
        "predicateType": PREDICATE_TYPE,
        "subject": [{ "digest": { "gitCommit": subject } }],
        "predicate": {
            "contract": PREDICATE_CONTRACT,
            "run": run,
            "provenance": provenance,
            "suites": suites,
        },
    });
    v.to_string()
}

// spec: evidence-kit/SPEC.md §verify-gate-run — one suite as a Statement carries it
#[derive(Debug, PartialEq)]
pub struct StatementSuite {
    pub name: String,
    pub cmd: String,
    pub log: String,
    pub counts: (u64, u64, u64),
    pub verdict: String,
    pub date: String,
    pub tree: String,
    pub binding: String,
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub subject: String,
    pub run: String,
    pub provenance_commit: Option<String>,
    pub suites: Vec<StatementSuite>,
}

// spec: evidence-kit/SPEC.md §verify-gate-run — a Statement or a DSSE envelope around one, judged for
// shape alone: every refusal here is one the verify arm reports at exit 2
pub fn read_statement(bytes: &[u8]) -> Result<Statement, String> {
    let v: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| format!("the input is not JSON: {}", e))?;
    let v = match v.get("payloadType") {
        None => v,
        Some(t) => {
            if t.as_str() != Some(DSSE_PAYLOAD_TYPE) {
                return Err(format!("a DSSE envelope whose payloadType is not {}", DSSE_PAYLOAD_TYPE));
            }
            let payload = v
                .get("payload")
                .and_then(|p| p.as_str())
                .ok_or("a DSSE envelope with no string payload")?;
            let raw = base64_decode(payload).ok_or("a DSSE payload that is not base64")?;
            serde_json::from_slice(&raw).map_err(|e| format!("a DSSE payload that is not JSON: {}", e))?
        }
    };
    let text = |v: &serde_json::Value, path: &str| -> Result<String, String> {
        v.pointer(path)
            .and_then(|s| s.as_str())
            .map(String::from)
            .ok_or_else(|| format!("{} is absent or not a string", path))
    };
    if text(&v, "/_type")? != STATEMENT_TYPE {
        return Err(format!("_type is not {}", STATEMENT_TYPE));
    }
    let ptype = text(&v, "/predicateType")?;
    if ptype != PREDICATE_TYPE {
        return Err(format!("predicateType '{}' is a type this reader does not know (want {})", ptype, PREDICATE_TYPE));
    }
    let contract = text(&v, "/predicate/contract")?;
    if !crate::provenance::semver(&contract) || contract.split('.').next() != Some("1") {
        return Err(format!("predicate contract '{}' is no major this reader knows (want 1.x.y)", contract));
    }
    let subjects = v.pointer("/subject").and_then(|s| s.as_array()).ok_or("/subject is absent or not an array")?;
    if subjects.len() != 1 {
        return Err(format!("/subject holds {} descriptors (want one, the commit)", subjects.len()));
    }
    let subject = text(&subjects[0], "/digest/gitCommit")?;
    if !crate::provenance::object_id(&subject) {
        return Err(format!("gitCommit '{}' is not 40 or 64 lowercase hex characters", subject));
    }
    let run = text(&v, "/predicate/run")?;
    let provenance_commit = provenance_shape(v.pointer("/predicate/provenance"))?;
    let listed = v
        .pointer("/predicate/suites")
        .and_then(|s| s.as_array())
        .ok_or("/predicate/suites is absent or not an array")?;
    if listed.is_empty() {
        return Err("/predicate/suites is empty: the Statement covers no suite".to_string());
    }
    let mut suites: Vec<StatementSuite> = Vec::new();
    for (i, s) in listed.iter().enumerate() {
        let at = |f: &str| format!("/predicate/suites/{}{}", i, f);
        let field = |f: &str| text(s, f).map_err(|_| format!("{} is absent or not a string", at(f)));
        let digest = |f: &str| {
            field(f).and_then(|d| if is_hex64(&d) { Ok(d) } else { Err(format!("{} is not 64 lowercase hex characters", at(f))) })
        };
        let count = |f: &str| {
            s.pointer(f).and_then(|n| n.as_u64()).ok_or_else(|| format!("{} is absent or not a non-negative integer", at(f)))
        };
        let name = field("/name")?;
        if name.is_empty() || name.contains(char::is_whitespace) {
            return Err(format!("{} '{}' is not a suite name", at("/name"), name));
        }
        if suites.iter().any(|x| x.name == name) {
            return Err(format!("suite '{}' appears twice", name));
        }
        let verdict = field("/verdict")?;
        if verdict != "clean" && verdict != "new-failures" {
            return Err(format!("{} '{}' is not clean or new-failures", at("/verdict"), verdict));
        }
        let date = field("/date")?;
        if !is_date(&date) {
            return Err(format!("{} '{}' is not YYYY-MM-DD", at("/date"), date));
        }
        let tree = field("/tree")?;
        if tree != "clean" && tree != "modified" {
            return Err(format!("{} '{}' is not clean or modified", at("/tree"), tree));
        }
        suites.push(StatementSuite {
            cmd: digest("/command/sha256")?,
            log: digest("/log/sha256")?,
            counts: (count("/counts/pass")?, count("/counts/fail")?, count("/counts/ignore")?),
            binding: digest("/binding/sha256")?,
            name,
            verdict,
            date,
            tree,
        });
    }
    Ok(Statement { subject, run, provenance_commit, suites })
}

// spec: evidence-kit/SPEC.md §verify-gate-run — check 1 types the embedded provenance block to
// gate-sdk/SPEC.md §run-gates' table; the commit it carries, if any, is check 2's to judge
fn provenance_shape(p: Option<&serde_json::Value>) -> Result<Option<String>, String> {
    const AT: &str = "/predicate/provenance";
    let p = p.filter(|p| p.is_object()).ok_or_else(|| format!("{} is absent or not an object", AT))?;
    let named = |v: &serde_json::Value, at: &str| -> Result<(), String> {
        match v.get("name").and_then(|n| n.as_str()) {
            Some(n) if !n.is_empty() => Ok(()),
            _ => Err(format!("{}/name is absent, empty or not a string", at)),
        }
    };
    let optional = |v: &serde_json::Value, at: &str, key: &str| -> Result<(), String> {
        match v.get(key) {
            Some(s) if s.as_str().map_or(true, str::is_empty) => Err(format!("{}/{} is empty or not a string", at, key)),
            _ => Ok(()),
        }
    };
    let tool_at = format!("{}/tool", AT);
    let tool = p.get("tool").filter(|t| t.is_object()).ok_or_else(|| format!("{} is absent or not an object", tool_at))?;
    named(tool, &tool_at)?;
    for key in ["version", "sourceStamp", "target"] {
        optional(tool, &tool_at, key)?;
    }
    let kits = p.get("kits").and_then(|k| k.as_array()).ok_or_else(|| format!("{}/kits is absent or not an array", AT))?;
    for (i, kit) in kits.iter().enumerate() {
        let at = format!("{}/kits/{}", AT, i);
        if !kit.is_object() {
            return Err(format!("{} is not an object", at));
        }
        named(kit, &at)?;
        match kit.get("root").and_then(|r| r.as_str()) {
            Some(r) if !r.is_empty() => {}
            _ => return Err(format!("{}/root is absent, empty or not a string", at)),
        }
        optional(kit, &at, "version")?;
    }
    let commit = match p.get("commit") {
        None => None,
        Some(c) => match c.as_str() {
            Some(c) if crate::provenance::object_id(c) => Some(c.to_string()),
            _ => return Err(format!("{}/commit is not 40 or 64 lowercase hex characters", AT)),
        },
    };
    match (commit.is_some(), p.get("worktree")) {
        (true, Some(w)) if w == "clean" || w == "modified" => {}
        (true, Some(_)) => return Err(format!("{}/worktree is not clean or modified", AT)),
        (true, None) => return Err(format!("{}/worktree is absent beside a commit", AT)),
        (false, Some(_)) => return Err(format!("{}/worktree is present with no commit", AT)),
        (false, None) => {}
    }
    Ok(commit)
}

// spec: evidence-kit/SPEC.md §verify-gate-run — DSSE's payload encoding, the standard and the
// URL-safe alphabets both read, padding optional
pub fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut acc: u32 = 0;
    let mut bits = 0;
    let body = s.trim_end_matches('=');
    if s.len() - body.len() > 2 {
        return None;
    }
    for c in body.bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    if bits >= 6 || acc != 0 {
        return None;
    }
    Some(out)
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — three outcomes, never two: an
// unparseable lock is corruption and fails closed rather than reading free
pub enum LockRead {
    Absent,
    Corrupt,
    Held {
        pid: String,
        run_key: String,
        handoff: Option<String>,
    },
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the record is one newline-terminated
// line: an unterminated one does not parse, nor does an empty file
pub fn lock_read(path: &std::path::Path) -> LockRead {
    if !path.is_file() {
        return LockRead::Absent;
    }
    let Ok(bytes) = std::fs::read(path) else {
        return LockRead::Corrupt;
    };
    let Some(nl) = bytes.iter().position(|b| *b == b'\n') else {
        return LockRead::Corrupt;
    };
    match parse_lock_line(&bytes[..nl]) {
        Some((pid, run_key, handoff)) => LockRead::Held {
            pid,
            run_key,
            handoff,
        },
        None => LockRead::Corrupt,
    }
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the record grammar, matched whole
fn parse_lock_line(line: &[u8]) -> Option<(String, String, Option<String>)> {
    let rest = line.strip_prefix(b"pid=")?;
    let digits = rest
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .count();
    if digits == 0 || rest[0] == b'0' {
        return None;
    }
    let (pid, rest) = rest.split_at(digits);
    let sep = *rest.first()?;
    if !is_posix_space(sep) {
        return None;
    }
    let rest = rest[1..].strip_prefix(b"run=")?;
    let n = rest.iter().take_while(|b| !is_posix_space(**b)).count();
    if n == 0 {
        return None;
    }
    let (key, rest) = rest.split_at(n);
    let handoff = match rest.split_first() {
        None => None,
        Some((_, field)) => Some(parse_handoff(field.strip_prefix(b"handoff=")?)?),
    };
    Some((
        String::from_utf8_lossy(pid).into_owned(),
        String::from_utf8_lossy(key).into_owned(),
        handoff,
    ))
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the optional third field's value
fn parse_handoff(path: &[u8]) -> Option<String> {
    if path.is_empty() || path.iter().any(|b| is_posix_space(*b)) {
        return None;
    }
    let path = std::str::from_utf8(path).ok()?;
    crate::walk::path_root(path).map(|_| path.to_string())
}

fn is_posix_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the two ways `ek_pid_alive`'s compiled twin can
// fail to answer, told apart because only one of them is a wrapper refusal the member owns text
// for; the other is `proc::run`'s standing backstop.
#[derive(Debug)]
pub enum PidProbe {
    #[cfg(not(unix))]
    PsAbsent,
    Unanswered(String),
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the pid grammar, then signal 0: on unix
// `kill(2)` reads EPERM as held and ESRCH as gone; on Windows the native leg asks first and the MSYS
// legs answer behind it; gate-sdk/SPEC.md §Fail-closed contract owns the per-platform route.
pub fn pid_alive(pid: &str) -> Result<bool, PidProbe> {
    pid_held_through(pid).map(|leg| leg.is_some())
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the same predicate, naming the leg that
// answered held for the one reader that prints it
pub fn pid_held_through(pid: &str) -> Result<Option<&'static str>, PidProbe> {
    pid_held_for_reader(pid, std::process::id())
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the reader's own pid is never a held
// reading; the reader is a parameter so a test can ask as a process it is not.
pub(crate) fn pid_held_for_reader(pid: &str, reader: u32) -> Result<Option<&'static str>, PidProbe> {
    if pid.is_empty() || pid.starts_with('0') || !pid.bytes().all(|b| b.is_ascii_digit()) {
        return Ok(None);
    }
    signal_zero(pid, reader)
}

#[cfg(unix)]
fn signal_zero(pid: &str, reader: u32) -> Result<Option<&'static str>, PidProbe> {
    let Ok(n) = pid.parse::<libc::pid_t>() else {
        return Ok(None);
    };
    if u32::try_from(n) == Ok(reader) {
        return Ok(None);
    }
    // spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — sound because
    // `kill` takes two integers and touches no memory this crate owns
    if unsafe { libc::kill(n, 0) } == 0 {
        return Ok(Some("kill(2)"));
    }
    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::EPERM) => Ok(Some("kill(2), EPERM")),
        Some(libc::ESRCH) => Ok(None),
        _ => Err(PidProbe::Unanswered(format!(
            "kill(2) could not answer for pid {pid}: {err}"
        ))),
    }
}

#[cfg(not(unix))]
#[cfg_attr(not(windows), allow(unused_variables))]
fn signal_zero(pid: &str, reader: u32) -> Result<Option<&'static str>, PidProbe> {
    #[cfg(windows)]
    if native_leg_held(pid, reader) {
        return Ok(Some("the native leg"));
    }
    let signalled = crate::proc::run(&programs::BASH, &["-c", "kill -0 \"$1\"", "bash", pid])
        .map_err(PidProbe::Unanswered)?;
    if signalled.code() == Some(0) {
        return Ok(Some("the kill -0 leg"));
    }
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the probe sits *here*, on the fallback leg,
    // because that is the only leg that reaches the program: a `kill -0` that answers never does.
    if !crate::proc::on_path(&programs::PS) {
        return Err(PidProbe::PsAbsent);
    }
    let listed = crate::proc::run(&programs::PS, &["-p", pid]).map_err(PidProbe::Unanswered)?;
    Ok((listed.code() == Some(0)).then_some("the ps -p leg"))
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the reader's own Windows pid skips this
// leg alone: a record may name an MSYS pid of the same number, which the legs behind still answer.
#[cfg(windows)]
fn native_leg_held(pid: &str, reader: u32) -> bool {
    pid.parse::<u32>().ok() != Some(reader) && windows_pid_held(pid)
}

#[cfg(windows)]
use crate::proc::kernel32;

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the native leg, run first: a process that
// opens and has not exited, or whose exit code cannot be read, is held, and so is one the open is
// denied; any other answer defers to the MSYS legs.
#[cfg(windows)]
fn windows_pid_held(pid: &str) -> bool {
    let Ok(n) = pid.parse::<u32>() else {
        return false;
    };
    // spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — sound because
    // the calls take integers, a stack-local out-parameter and a handle this function closes itself
    unsafe {
        let handle = kernel32::OpenProcess(kernel32::PROCESS_QUERY_LIMITED_INFORMATION, 0, n);
        if handle.is_null() {
            return std::io::Error::last_os_error().raw_os_error() == Some(kernel32::ERROR_ACCESS_DENIED);
        }
        let mut code = 0u32;
        let read = kernel32::GetExitCodeProcess(handle, &mut code);
        kernel32::CloseHandle(handle);
        read == 0 || code == kernel32::STILL_ACTIVE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declared(pairs: &[(&str, &str)]) -> Result<Vec<Control>, Vec<String>> {
        let family: Vec<(String, String)> = pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        let names: Vec<String> = pairs.iter().map(|(k, _)| k.to_string()).collect();
        match controls(&names, &family, &["unit".to_string()]) {
            (ok, e) if e.is_empty() => Ok(ok),
            (_, e) => Err(e),
        }
    }

    // spec: evidence-kit/SPEC.md §Test controls — the parser's refusals, each naming its control
    #[test]
    fn the_declaration_parser_refuses_each_malformed_shape() {
        let ok = declared(&[("nullguard", "unit t_null BREAK_NULL=1 MODE=x")]).unwrap();
        assert_eq!(
            ok[0].env,
            vec![("BREAK_NULL".to_string(), "1".to_string()), ("MODE".to_string(), "x".to_string())]
        );
        for (value, want) in [
            ("ghost t BREAK=1", "not in EVIDENCE_KIT_SUITES"),
            ("unit t", "carries no <VAR>=<value> assignment"),
            ("unit t 9X=1", "not a <identifier>=<value> assignment"),
            ("unit t NOEQUALS", "not a <identifier>=<value> assignment"),
            ("unit", "want <suite> <scenario>"),
        ] {
            let err = declared(&[("c", value)]).unwrap_err().join("; ");
            assert!(err.contains(want) && err.contains("'c'"), "{}: {}", value, err);
        }
        let since = declared(&[("SINCE", "unit t BREAK=1")]).unwrap_err().join("; ");
        assert!(since.contains("collides with EVIDENCE_KIT_CONTROL_SINCE"), "{}", since);
        let missing = controls(&["gone".to_string()], &[], &["unit".to_string()]).1.join("; ");
        assert!(missing.contains("has no EVIDENCE_KIT_CONTROL_gone declaration"), "{}", missing);
    }

    #[test]
    fn the_witness_maps_the_scenario_status_last_observation_winning() {
        let parsed = |s: &[&str]| s.iter().map(|l| l.to_string()).collect::<Vec<_>>();
        assert_eq!(witness(&parsed(&["t fail"]), "t"), "red");
        assert_eq!(witness(&parsed(&["t pass"]), "t"), "green");
        assert_eq!(witness(&parsed(&["t ignore"]), "t"), "absent");
        assert_eq!(witness(&parsed(&["u fail"]), "t"), "absent");
        assert_eq!(witness(&parsed(&["t pass", "t fail"]), "t"), "red");
    }

    // spec: evidence-kit/SPEC.md §Test controls — the assignments digest: declared order, each
    // assignment newline-terminated, the declaration's whitespace never reaching it
    #[test]
    fn the_assignments_digest_pins_the_declared_assignments() {
        let two = &declared(&[("c", "unit t BREAK_NULL=1 MODE=x")]).unwrap()[0];
        let d = assignments_digest(two);
        assert_eq!(d, crate::sha256::hex(b"BREAK_NULL=1\nMODE=x\n"));
        assert_eq!(d, assignments_digest(&declared(&[("c", "  unit\tt  BREAK_NULL=1   MODE=x ")]).unwrap()[0]));
        assert_ne!(d, assignments_digest(&declared(&[("c", "unit t MODE=x BREAK_NULL=1")]).unwrap()[0]));
        assert_ne!(d, assignments_digest(&declared(&[("c", "unit t BREAK_NULL=0 MODE=x")]).unwrap()[0]));
    }

    const GOLDEN_SUBJECT: &str = "0123456789abcdef0123456789abcdef01234567";

    fn golden_row(suite: &str, run: &str, counts: (u64, u64, u64), verdict: &str) -> SuiteLine {
        SuiteLine {
            key: "golden-run".into(),
            suite: suite.into(),
            log: crate::sha256::hex(format!("{} log\n", suite).as_bytes()),
            counts,
            verdict: verdict.into(),
            date: "2026-01-01".into(),
            commit: GOLDEN_SUBJECT.into(),
            tree: "clean".into(),
            cmd: command_digest(run, "libtest"),
            origin: "local".into(),
        }
    }

    fn golden_statement() -> String {
        let prov = crate::provenance::Provenance {
            tool_version: Some("1.0.0".into()),
            exact: true,
            source_stamp: None,
            target: Some("x86_64-unknown-linux-gnu".into()),
            kits: vec![crate::provenance::Kit {
                name: "evidence-kit".into(),
                root: "evidence-kit".into(),
                version: Some("1.0.0".into()),
            }],
            commit: Some(GOLDEN_SUBJECT.into()),
            clean: true,
        };
        let rows = [
            golden_row("unit", "cargo test", (12, 0, 1), "clean"),
            golden_row("fixtures", "checkwright-gates --run-gate-tests tests", (40, 2, 0), "new-failures"),
        ];
        format!("{}\n", statement("golden-run", GOLDEN_SUBJECT, prov.block(), &rows))
    }

    fn keys(v: &serde_json::Value) -> Vec<String> {
        let mut k: Vec<String> = v.as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default();
        k.sort();
        k
    }

    fn strings(v: Option<&serde_json::Value>) -> Vec<String> {
        let mut k: Vec<String> = v
            .and_then(|a| a.as_array())
            .map(|a| a.iter().filter_map(|s| s.as_str().map(String::from)).collect())
            .unwrap_or_default();
        k.sort();
        k
    }

    // spec: evidence-kit/SPEC.md §The gate-run predicate — the published golden is the renderer's
    // bytes, and the published schema names exactly the fields the renderer writes, required
    #[test]
    fn the_published_golden_and_schema_match_the_renderer() {
        let rendered = golden_statement();
        assert_eq!(
            rendered,
            include_str!("../../docs/contracts/gate-run/v1/statement.json"),
            "docs/contracts/gate-run/v1/statement.json is stale; the renderer now writes:\n{}",
            rendered
        );
        let st: serde_json::Value = serde_json::from_str(&rendered).expect("the golden is JSON");
        let schema: serde_json::Value =
            serde_json::from_str(include_str!("../../docs/contracts/gate-run/v1/schema.json")).expect("the schema is JSON");
        let pred = schema.pointer("/properties/predicate").expect("the schema describes the predicate");
        let want = keys(&st["predicate"]);
        assert_eq!(keys(&pred["properties"]), want);
        assert_eq!(strings(pred.get("required")), want);
        let item = pred.pointer("/properties/suites/items").expect("the schema describes a suite");
        let want = keys(&st["predicate"]["suites"][0]);
        assert_eq!(keys(&item["properties"]), want);
        assert_eq!(strings(item.get("required")), want);
        assert_eq!(strings(schema.get("required")), keys(&st));
        let prov = pred.pointer("/properties/provenance").expect("the schema describes the provenance block");
        let full = crate::provenance::Provenance {
            tool_version: Some("1.0.0".into()),
            exact: true,
            source_stamp: Some("s".into()),
            target: Some("t".into()),
            kits: vec![crate::provenance::Kit { name: "k".into(), root: "k".into(), version: Some("1.0.0".into()) }],
            commit: Some(GOLDEN_SUBJECT.into()),
            clean: true,
        }
        .block();
        assert_eq!(keys(&prov["properties"]), keys(&full));
        assert_eq!(keys(&prov["properties"]["tool"]["properties"]), keys(&full["tool"]));
        assert_eq!(keys(&prov["properties"]["kits"]["items"]["properties"]), keys(&full["kits"][0]));
        for fields in [&prov["properties"]["tool"]["properties"], &prov["properties"]["kits"]["items"]["properties"]] {
            for (key, field) in fields.as_object().expect("the schema types the block's fields") {
                if field["type"] == "string" {
                    assert_eq!(field["minLength"], 1, "the schema admits a blank provenance {}", key);
                }
            }
        }
        let read = read_statement(rendered.as_bytes()).expect("the golden reads back");
        assert_eq!(read.suites.len(), 2);
        assert_eq!(read.suites[1].binding, golden_row("fixtures", "checkwright-gates --run-gate-tests tests", (40, 2, 0), "new-failures").binding());
    }

    // spec: evidence-kit/SPEC.md §The gate-run predicate — the schema's `contract` pattern admits at
    // major 1 exactly what the reader's SemVer check admits
    #[test]
    fn the_schema_contract_pattern_agrees_with_the_readers_semver() {
        let schema: serde_json::Value =
            serde_json::from_str(include_str!("../../docs/contracts/gate-run/v1/schema.json")).expect("the schema is JSON");
        let pattern = schema
            .pointer("/properties/predicate/properties/contract/pattern")
            .and_then(|p| p.as_str())
            .expect("the schema patterns the contract");
        let re = crate::ere::Ere::compile(pattern).expect("the pattern is in the ERE subset");
        for v in [
            "1.0.0", "1.2.3", "1.10.0", "1.0.0-0", "1.0.0-alpha", "1.0.0-alpha.1", "1.0.0-0a", "1.0.0-a-b",
            "1.0.0--", "1.0.0-x.7.z.92", "1.0.0+build", "1.0.0+001", "1.0.0+a.b-c", "1.0.0-rc.1+build.5",
            "1.0.0-01", "1.0.0-a..b", "1.0.0-.a", "1.0.0-a.", "1.0.0-", "1.0.0+", "1.0.0+..", "1.0.0+a.",
            "1.0.0-a_b", "1.0.0+a_b", "1.01.0", "1.0.01", "1.0", "1.0.0.0", "01.0.0", "1.0.0-00",
        ] {
            assert_eq!(re.is_match(v), crate::provenance::semver(v), "{}", v);
        }
        assert!(!re.is_match("2.0.0") && crate::provenance::semver("2.0.0"), "the pattern pins major 1");
    }

    // spec: evidence-kit/SPEC.md §The gate-run predicate — the command digest hashes the spawned
    // argv, so whitespace that splits to the same words binds the same, and other words do not
    #[test]
    fn the_command_digest_normalizes_whitespace_alone() {
        let d = command_digest("cargo test", "libtest");
        assert_eq!(d, command_digest("  cargo\t test \n", " libtest "));
        assert_eq!(d, crate::sha256::hex(b"run cargo test\nparser libtest\n"));
        assert_ne!(d, command_digest("cargo test --release", "libtest"));
        assert_ne!(d, command_digest("cargo test", "exit-code"));
        assert_eq!(resolved_command_digest(&[], &[], "libtest", "unit"), None);
        let fam = [("unit".to_string(), "cargo   test".to_string())];
        let parsers = [("unit".to_string(), "exit-code".to_string())];
        assert_eq!(resolved_command_digest(&fam, &[], "libtest", "unit"), Some(d));
        assert_eq!(
            resolved_command_digest(&fam, &parsers, "libtest", "unit"),
            Some(command_digest("cargo test", "exit-code"))
        );
    }

    // spec: evidence-kit/SPEC.md §The gate-run predicate — the binding digest over a fixed line
    #[test]
    fn the_binding_digest_is_its_newline_terminated_lines() {
        let line = golden_row("unit", "cargo test", (12, 0, 1), "clean");
        let text = format!(
            "gate-run-binding v1\nunit\n{}\n{}\n12 0 1\nclean\n{}\n",
            line.cmd, GOLDEN_SUBJECT, line.log
        );
        assert_eq!(line.binding(), crate::sha256::hex(text.as_bytes()));
        let mut moved = line.clone();
        moved.counts = (11, 1, 1);
        assert_ne!(moved.binding(), line.binding());
        assert_eq!(line.render().split_whitespace().count(), 12);
    }

    fn base64_encode(b: &[u8]) -> String {
        const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in b.chunks(3) {
            let n = chunk.iter().enumerate().fold(0u32, |acc, (i, &c)| acc | (u32::from(c) << (16 - 8 * i)));
            for i in 0..=chunk.len() {
                out.push(A[((n >> (18 - 6 * i)) & 63) as usize] as char);
            }
            for _ in chunk.len()..3 {
                out.push('=');
            }
        }
        out
    }

    // spec: evidence-kit/SPEC.md §verify-gate-run — a DSSE envelope is unwrapped to the Statement
    // it carries; a payload of another type, or one that is not base64, is a shape refusal
    #[test]
    fn a_dsse_envelope_unwraps_to_its_statement() {
        let st = golden_statement();
        let env = serde_json::json!({
            "payloadType": DSSE_PAYLOAD_TYPE,
            "payload": base64_encode(st.as_bytes()),
            "signatures": [{ "sig": "c2ln" }],
        });
        let read = read_statement(env.to_string().as_bytes()).expect("the envelope unwraps");
        assert_eq!(read, read_statement(st.as_bytes()).unwrap());
        let other = serde_json::json!({ "payloadType": "text/plain", "payload": "" });
        assert!(read_statement(other.to_string().as_bytes()).unwrap_err().contains("payloadType"));
        let bad = serde_json::json!({ "payloadType": DSSE_PAYLOAD_TYPE, "payload": "!!" });
        assert!(read_statement(bad.to_string().as_bytes()).unwrap_err().contains("base64"));
    }

    #[test]
    fn base64_reads_both_alphabets_with_or_without_padding() {
        assert_eq!(base64_decode("aGk=").as_deref(), Some(&b"hi"[..]));
        assert_eq!(base64_decode("aGk").as_deref(), Some(&b"hi"[..]));
        assert_eq!(base64_decode("-_8=").as_deref(), Some(&[0xfb, 0xff][..]));
        assert_eq!(base64_decode("+/8=").as_deref(), Some(&[0xfb, 0xff][..]));
        assert_eq!(base64_decode(""), Some(Vec::new()));
        assert_eq!(base64_decode("a"), None);
        assert_eq!(base64_decode("aGl="), None);
        assert_eq!(base64_decode("aG*="), None);
        assert_eq!(base64_decode("aGk==="), None);
    }

    // spec: evidence-kit/SPEC.md §verify-gate-run — a reader refuses a type or a contract major it
    // does not know, a Statement whose subject is not one commit, and each provenance shape the
    // table refuses
    #[test]
    fn the_shape_refusals_name_their_field() {
        let st: serde_json::Value = serde_json::from_str(&golden_statement()).unwrap();
        let refuse = |edit: &dyn Fn(&mut serde_json::Value), want: &str| {
            let mut v = st.clone();
            edit(&mut v);
            let e = read_statement(v.to_string().as_bytes()).expect_err(want);
            assert!(e.contains(want), "want '{}' in: {}", want, e);
        };
        refuse(&|v| v["predicateType"] = "https://checkwright.dev/gate-run/v2".into(), "predicateType");
        refuse(&|v| v["predicate"]["contract"] = "2.0.0".into(), "contract");
        refuse(&|v| v["_type"] = "x".into(), "_type");
        refuse(&|v| v["subject"] = serde_json::json!([]), "descriptors");
        refuse(&|v| v["subject"][0]["digest"]["gitCommit"] = "ABC".into(), "gitCommit");
        refuse(&|v| v["predicate"]["suites"] = serde_json::json!([]), "empty");
        refuse(&|v| v["predicate"]["suites"][1]["name"] = "unit".into(), "twice");
        refuse(&|v| v["predicate"]["suites"][0]["tree"] = "dirty".into(), "tree");
        refuse(&|v| v["predicate"]["suites"][0]["counts"]["fail"] = (-1).into(), "counts/fail");
        let commit = st["predicate"]["provenance"]["commit"].clone();
        refuse(&|v| v["predicate"]["provenance"] = serde_json::json!({ "commit": commit }), "provenance/tool");
        refuse(&|v| v["predicate"]["provenance"] = "x".into(), "provenance is absent or not an object");
        refuse(&|v| v["predicate"]["provenance"]["tool"] = "x".into(), "provenance/tool");
        refuse(&|v| v["predicate"]["provenance"]["tool"]["name"] = "".into(), "provenance/tool/name");
        refuse(&|v| v["predicate"]["provenance"]["tool"]["version"] = 1.into(), "provenance/tool/version");
        refuse(&|v| v["predicate"]["provenance"]["tool"]["sourceStamp"] = 1.into(), "provenance/tool/sourceStamp");
        refuse(&|v| v["predicate"]["provenance"]["tool"]["target"] = 1.into(), "provenance/tool/target");
        refuse(&|v| { v["predicate"]["provenance"].as_object_mut().unwrap().remove("kits"); }, "provenance/kits");
        refuse(&|v| v["predicate"]["provenance"]["kits"] = serde_json::json!(["x"]), "provenance/kits/0 is not");
        refuse(&|v| { v["predicate"]["provenance"]["kits"][0].as_object_mut().unwrap().remove("name"); }, "kits/0/name");
        refuse(&|v| v["predicate"]["provenance"]["kits"][0]["root"] = 7.into(), "kits/0/root");
        refuse(&|v| v["predicate"]["provenance"]["kits"][0]["version"] = 7.into(), "kits/0/version");
        refuse(&|v| v["predicate"]["provenance"]["tool"]["version"] = "".into(), "provenance/tool/version is empty");
        refuse(&|v| v["predicate"]["provenance"]["tool"]["sourceStamp"] = "".into(), "provenance/tool/sourceStamp is empty");
        refuse(&|v| v["predicate"]["provenance"]["tool"]["target"] = "".into(), "provenance/tool/target is empty");
        refuse(&|v| v["predicate"]["provenance"]["kits"][0]["version"] = "".into(), "kits/0/version is empty");
        refuse(&|v| v["predicate"]["provenance"]["commit"] = "ABC".into(), "provenance/commit");
        refuse(&|v| v["predicate"]["provenance"]["worktree"] = "dirty".into(), "provenance/worktree is not");
        refuse(&|v| { v["predicate"]["provenance"].as_object_mut().unwrap().remove("worktree"); }, "beside a commit");
        refuse(&|v| { v["predicate"]["provenance"].as_object_mut().unwrap().remove("commit"); }, "with no commit");
        let mut unborn = st.clone();
        let block = unborn["predicate"]["provenance"].as_object_mut().unwrap();
        block.remove("commit");
        block.remove("worktree");
        block["tool"].as_object_mut().unwrap().retain(|k, _| k == "name");
        block["kits"] = serde_json::json!([]);
        assert_eq!(
            read_statement(unborn.to_string().as_bytes()).map(|s| s.provenance_commit),
            Ok(None),
            "a block whose fields the run could not fill reads, absent fields absent"
        );
        let mut extra = st.clone();
        extra["predicate"]["future"] = "ignored".into();
        assert!(read_statement(extra.to_string().as_bytes()).is_ok(), "an unknown field is ignored");
    }

    #[test]
    fn comments_and_blanks_are_not_data_lines() {
        assert_eq!(data_lines("# h\n\n  \nu a pass\n  # c\n"), vec!["u a pass"]);
    }

    // spec: evidence-kit/SPEC.md §The evidence adapters — the two readers part company on shape:
    // the iteration is a header field, the stage a positional on the last data line
    #[test]
    fn the_iteration_and_the_cursor_are_read_from_their_own_shapes() {
        assert_eq!(
            queue_iteration("# q\n## Iteration: alpha  [stage: build]\n").as_deref(),
            Some("alpha")
        );
        assert_eq!(queue_iteration("## Iteration:").as_deref(), Some(""));
        assert_eq!(queue_iteration("# q\n"), None);
        assert_eq!(
            state_stage("h\n---\nit scope s1 d\nit close s3 d\n").as_deref(),
            Some("close")
        );
        assert_eq!(state_stage("h\n---\n"), None);
        assert_eq!(state_stage("h\nit close s3 d\n"), None);
        assert_eq!(state_stage("h\n---\nlonely\n"), None);
    }

    // spec: evidence-kit/SPEC.md §The evidence adapters — the run key: the header's iteration, else
    // the run id, and neither the placeholder nor a nameless header is a key
    #[test]
    fn the_run_key_is_the_iteration_else_the_run_id_and_never_the_placeholder() {
        assert_eq!(run_key(Some("## Iteration: alpha\n"), "rid").as_deref(), Some("alpha"));
        assert_eq!(run_key(Some("## Iteration: —\n"), "rid").as_deref(), Some("rid"));
        assert_eq!(run_key(Some("## Iteration:\n"), "rid").as_deref(), Some("rid"));
        assert_eq!(run_key(None, "rid").as_deref(), Some("rid"));
        assert_eq!(run_key(Some("## Iteration: —\n"), ""), None);
        assert_eq!(run_key(None, ""), None);
    }

    // spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — a skip record demotes its pass, and an
    // observed `fail` with no baseline row is a new failure, each with or without a final newline
    #[test]
    fn a_final_line_is_read_without_its_newline() {
        let base = "s a pass\n";
        assert!(diff(base, "s", "a pass\n", "").findings.is_empty());
        assert!(diff(base, "s", "a pass", "").findings.is_empty());
        assert!(diff(base, "s", "a pass\n", "s a\n").new_failure);
        assert!(diff(base, "s", "a pass\n", "s a").new_failure);
        assert!(diff(base, "s", "a pass\nb fail", "").new_failure);
    }

    #[cfg(unix)]
    const ABS: &str = "/x/h";
    #[cfg(windows)]
    const ABS: &str = "C:\\x\\h";

    // spec: evidence-kit/SPEC.md §The producer-liveness lock — the record grammar, the optional
    // handoff field in its one place and form, and every other shape no record
    #[test]
    fn a_record_line_parses_only_in_its_one_grammar() {
        let parse = |s: &str| parse_lock_line(s.as_bytes());
        let held = |k: &str, h: Option<&str>| Some(("12".into(), k.into(), h.map(String::from)));
        assert_eq!(parse("pid=12 run=k"), held("k", None));
        assert_eq!(parse("pid=12\trun=k"), held("k", None));
        assert_eq!(parse(&format!("pid=12 run=k handoff={}", ABS)), held("k", Some(ABS)));
        assert_eq!(parse(&format!("pid=12 run=k\thandoff={}", ABS)), held("k", Some(ABS)));
        for bad in [
            "pid=012 run=k".to_string(),
            "pid=12  run=k".into(),
            "pid=12 run=k x".into(),
            "pid=12 run=k ".into(),
            "garbage".into(),
            "pid=12 run=k handoff=".into(),
            "pid=12 run=k handoff=rel/h".into(),
            format!("pid=12 run=k  handoff={}", ABS),
            format!("pid=12 run=k handoff={} ", ABS),
            format!("pid=12 run=k handoff={} handoff={}", ABS, ABS),
            format!("pid=12 handoff={} run=k", ABS),
            format!("pid=12 run=k run=k handoff={}", ABS),
            format!("pid=12 run=k owner={}", ABS),
        ] {
            assert_eq!(parse(&bad), None, "the record grammar admitted {:?}", bad);
        }
        let mut raw = b"pid=12 run=k handoff=".to_vec();
        raw.extend_from_slice(ABS.as_bytes());
        raw.push(0xff);
        assert_eq!(parse_lock_line(&raw), None, "a non-UTF-8 handoff path parsed");
    }

    // spec: evidence-kit/SPEC.md §The producer-liveness lock — the native leg answers the Windows
    // namespace: this process is held and an id no process carries is not
    #[cfg(windows)]
    #[test]
    fn the_native_leg_reads_a_windows_pid() {
        assert!(windows_pid_held(&std::process::id().to_string()));
        assert!(!windows_pid_held("2147483646"));
    }

    // spec: evidence-kit/SPEC.md §The producer-liveness lock — the reader's own pid is never held
    // through the native leg, and the same pid asked by any other reader is
    #[cfg(windows)]
    #[test]
    fn the_native_leg_skips_the_readers_own_pid() {
        let own = std::process::id();
        assert!(!native_leg_held(&own.to_string(), own));
        assert!(native_leg_held(&own.to_string(), own.wrapping_add(1)));
    }
}
