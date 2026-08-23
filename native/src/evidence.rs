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
