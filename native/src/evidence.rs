// spec: evidence-kit/SPEC.md §The evidence adapters — evidence-kit's adapters and the readers its
// gates share, kept here rather than reached for in `crate::stages` so the kit stays independent of
// lifecycle-kit
use crate::programs;

// spec: evidence-kit/SPEC.md §Evidence manifest — the versioned wire format the header declares
pub const MANIFEST_CONTRACT: &str = "evidence-manifest v1";

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

// spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — newline-terminated lines only: a final line
// with no newline is not read
fn bash_lines(text: &str) -> Vec<&str> {
    match text.rfind('\n') {
        Some(i) => text[..=i].lines().collect(),
        None => Vec::new(),
    }
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — three outcomes, never two: an
// unparseable lock is corruption and fails closed rather than reading free
pub enum LockRead {
    Absent,
    Corrupt,
    Held { pid: String, run_key: String },
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
        Some((pid, run_key)) => LockRead::Held { pid, run_key },
        None => LockRead::Corrupt,
    }
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the record grammar, matched whole
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

    // spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — a skip record demotes its pass, and a final
    // record with no newline is not read
    #[test]
    fn a_skip_record_is_read_only_when_newline_terminated() {
        let base = "s a pass\n";
        assert!(diff(base, "s", "a pass\n", "").findings.is_empty());
        assert!(diff(base, "s", "a pass\n", "s a\n").new_failure);
        assert!(!diff(base, "s", "a pass\n", "s a").new_failure);
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
