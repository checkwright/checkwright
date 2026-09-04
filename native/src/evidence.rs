// spec: evidence-kit/SPEC.md §lib/evidence.sh — the binary side of evidence-kit's own library:
// the readers its gates share, kept here rather than reached for in `crate::stages` so the
// compiled form inherits the shell library's deliberate independence from lifecycle-kit

// spec: evidence-kit/SPEC.md §Evidence manifest — the versioned wire format the header
// declares. The spec owns the value; the shell library and this const are its two
// implementations, and the unit test below is what holds them equal.
pub const MANIFEST_CONTRACT: &str = "evidence-manifest v1";

// spec: evidence-kit/SPEC.md §check-evidence-manifest — `ek_data_lines`: everything but a
// comment line and a blank one. Distinct from `crate::stages::data_lines` — same name,
// different primitive, and that section owns why binding to the other one is silent.
pub fn data_lines(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|l| {
            let t = l.trim_start_matches([' ', '\t']);
            !t.is_empty() && !t.starts_with('#')
        })
        .collect()
}

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_queue_iteration`: the first `## Iteration:`
// line with its lead and any residual `[stage:` field stripped. `None` is the helper's non-zero
// return — an absent file or no header at all — where an empty string is a header with no name.
pub fn queue_iteration(text: &str) -> Option<String> {
    let hdr = text.lines().find(|l| l.starts_with("## Iteration:"))?;
    let mut s = hdr.strip_prefix("## Iteration:").unwrap_or(hdr);
    s = s.trim_start_matches([' ', '\t']);
    Some(match s.find("[stage:") {
        Some(i) => s[..i].trim_end_matches([' ', '\t']).to_string(),
        None => s.to_string(),
    })
}

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_state_stage`'s corpus: every non-blank line
// below the `---` separator, which is also the set assertion C's validate-stamp scan reads
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

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_state_stage`: the last data line's second
// field. `None` on all three non-zero shapes — absent file, no data line, no second field.
pub fn state_stage(text: &str) -> Option<String> {
    let last = state_lines(text).last().copied()?;
    last.split_whitespace().nth(1).map(String::from)
}

// spec: evidence-kit/SPEC.md §lib/evidence.sh — the never-named iteration the queue header carries
// before a run is named; `ek_run_key` reads it as *no key* rather than as one, so the placeholder
// never reaches a manifest line.
const UNNAMED: &str = "—";

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_run_key`: the queue header's iteration where it
// names one, else the configured run id. `None` is the helper's non-zero return, which the spine
// turns into the guards' exit 2 — never into a verdict.
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

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_suite_cmd`: the suite's configured run command,
// resolved out of the `EVIDENCE_KIT_RUN_*` family rather than by composing the variable name, so a
// suite the family does not carry answers empty and the caller's own guard reports it.
pub fn suite_cmd(run_family: &[(String, String)], suite: &str) -> String {
    crate::walk::knob_in_family(run_family, suite).unwrap_or_default()
}

// spec: evidence-kit/SPEC.md §Layout and configuration — `ek_parser_for`: the per-suite override
// ahead of the global knob, `${!var:-$EVIDENCE_KIT_PARSER}` — the `:-` form, so an override
// resolving *empty* falls through to the global exactly as an unset one does.
pub fn parser_for(parser_family: &[(String, String)], suite: &str, global: &str) -> String {
    match crate::walk::knob_in_family(parser_family, suite) {
        Some(v) if !v.is_empty() => v,
        _ => global.to_string(),
    }
}

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_parse`'s three arms: the two bundled adapters
// and, for any other value, the consumer command word-split and spawned with the log appended last.
// The consumer arm keeps the child's stdout whatever its status, the shell capture's own rule.
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
            let program = words.remove(0);
            let display = log.display().to_string();
            words.push(&display);
            let out = crate::proc::run_streamed(program, &words, b"", crate::proc::Stderr::Inherit)?;
            Ok(String::from_utf8_lossy(out.stdout())
                .lines()
                .map(String::from)
                .collect())
        }
    }
}

// spec: evidence-kit/SPEC.md §lib/evidence.sh — the `libtest` adapter's awk program: a line whose
// first field is `test` and which carries a ` ... ` run of the result separator, keyed on the
// second field and graded by the last. A token that is none of the three ranks is no scenario.
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

// spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — `ek_diff`'s findings and its verdict, the one
// per-scenario diff both arms share: exactly two printed shapes and no third, and the status is 1
// the moment a new failure fires, so a recovery-only diff is clean.
pub struct Diff {
    pub findings: Vec<String>,
    pub new_failure: bool,
}

// spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — `ek_diff`: a baseline `pass` scenario red or
// absent is a new failure, a `fail`/`ignore` one observed green an unpromoted recovery, an observed
// `fail` with no baseline row a new failure; the skip demotion runs before the pass/fail branch.
pub fn diff(baseline_text: &str, suite: &str, observed_text: &str, skip_text: &str) -> Diff {
    let mut obs: Vec<(String, String)> = Vec::new();
    for line in bash_lines(observed_text) {
        let mut f = line.split_whitespace();
        let Some(sc) = f.next() else { continue };
        let st = f.next().unwrap_or("");
        match obs.iter_mut().find(|(k, _)| k == sc) {
            Some(slot) => slot.1 = st.to_string(),
            None => obs.push((sc.to_string(), st.to_string())),
        }
    }
    let mut skip: Vec<&str> = Vec::new();
    for line in bash_lines(skip_text) {
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

// spec: evidence-kit/SPEC.md §lib/evidence.sh — what bash's `while read` over a redirected file
// yields: a final line with no terminating newline is read into the variables but the loop body
// never runs on it, so the adapters see complete lines only and the compiled twin must too.
fn bash_lines(text: &str) -> Vec<&str> {
    match text.rfind('\n') {
        Some(i) => text[..=i].lines().collect(),
        None => Vec::new(),
    }
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — `ek_lock_read`'s three outcomes as
// three variants rather than a `Result<Option<_>>`: the helper's 1 and 2 mean opposite things to
// a caller, and a shape folding either into the other is the reading that section forbids.
pub enum LockRead {
    Absent,
    Corrupt,
    Held { pid: String, run_key: String },
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — `ek_lock_read`, reproducing
// `IFS= read -r line <"$lock" || return 2` at the byte level: bash's `read` returns non-zero at
// EOF with no delimiter, so a record with no trailing newline is corruption, and so is an empty file.
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
        Some((pid, run_key)) => LockRead::Held { pid, run_key },
        None => LockRead::Corrupt,
    }
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the record grammar
// `^pid=([1-9][0-9]*)[[:space:]]run=([^[:space:]]+)$`, anchored whole: exactly one whitespace
// byte between the fields, a run key carrying none, and no leading zero on the pid.
fn parse_lock_line(line: &[u8]) -> Option<(String, String)> {
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
    let key = rest[1..].strip_prefix(b"run=")?;
    if key.is_empty() || key.iter().any(|b| is_posix_space(*b)) {
        return None;
    }
    Some((
        String::from_utf8_lossy(pid).into_owned(),
        String::from_utf8_lossy(key).into_owned(),
    ))
}

fn is_posix_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the two ways `ek_pid_alive`'s compiled twin can
// fail to answer, told apart because only one of them is a wrapper refusal the member owns text
// for; the other is `proc::run`'s standing backstop.
#[derive(Debug)]
pub enum PidProbe {
    PsAbsent,
    Spawn(String),
}

// spec: evidence-kit/SPEC.md §check-producer-liveness — `ek_pid_alive`: the pid grammar, bash's
// `kill -0` builtin, then `ps -p` for the EPERM case. gate-sdk/SPEC.md §Fail-closed contract owns
// why the builtin is reached through `bash -c` rather than through a second off-floor program.
pub fn pid_alive(pid: &str) -> Result<bool, PidProbe> {
    if pid.is_empty() || pid.starts_with('0') || !pid.bytes().all(|b| b.is_ascii_digit()) {
        return Ok(false);
    }
    let signalled = crate::proc::run("bash", &["-c", "kill -0 \"$1\"", "bash", pid])
        .map_err(PidProbe::Spawn)?;
    if signalled.code() == Some(0) {
        return Ok(true);
    }
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the probe sits *here*, on the fallback leg,
    // because that is the only leg that reaches the program: a `kill -0` that answers never does.
    if !crate::proc::on_path("ps") {
        return Err(PidProbe::PsAbsent);
    }
    let listed = crate::proc::run("ps", &["-p", pid]).map_err(PidProbe::Spawn)?;
    Ok(listed.code() == Some(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: evidence-kit/SPEC.md §Evidence manifest — the wire-format version has two
    // implementations, so the crate's copy is held to the shell library's by executing it.
    // A static roster would be a third holder of the same value.
    #[test]
    fn the_wire_contract_matches_the_shell_library() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let completed = crate::proc::run(
            "bash",
            &[
                "-c",
                "cd \"$1\" || exit 2; . evidence-kit/lib/evidence.sh; \
                 printf '%s' \"$EVIDENCE_MANIFEST_CONTRACT\"",
                "bash",
                &repo.display().to_string(),
            ],
        )
        .expect("cannot run the shell library");
        let out = completed
            .stdout()
            .expect("evidence-kit/lib/evidence.sh could not report the wire contract");
        assert_eq!(String::from_utf8_lossy(out), MANIFEST_CONTRACT);
    }

    #[test]
    fn comments_and_blanks_are_not_data_lines() {
        assert_eq!(data_lines("# h\n\n  \nu a pass\n  # c\n"), vec!["u a pass"]);
    }

    // spec: evidence-kit/SPEC.md §lib/evidence.sh — the two readers part company on shape:
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
}
