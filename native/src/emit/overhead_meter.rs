// spec: drift-kit/SPEC.md §The overhead meter — the byte-proxy governance/task classifier over a
// session transcript: a fixed marker table, first match wins, whole-line classification, and one
// keyed line appended to the trend log per measured session.
// spec: gate-sdk/SPEC.md §The non-gate arm — a bridged-arm table member and not a hardcoded flag,
// because the meter resolves consumer knobs and a hardcoded flag receives no override at all; and
// `Arm::Emit` rather than `Arm::Run`, because exit is always 0 so no `1` is load-bearing.

pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_SESSIONS_DIR",
    "DRIFT_KIT_METRIC_DIR",
    "DRIFT_KIT_OVERHEAD_LOG",
];

const USAGE: &str =
    "usage: --emit overhead-meter [transcript.jsonl]\n  bare: the transcript this session is \
     running in, resolved under DRIFT_KIT_SESSIONS_DIR";

// spec: drift-kit/SPEC.md §The overhead meter — the fixed marker table: kit-name and gate-output
// shapes only (mechanism, never a private vocabulary; the seam holds). Each row is an alternation
// of literal substrings, so the shell's awk patterns port as `contains` rather than as regexes.
const MARKERS: [(&str, &[&str]); 4] = [
    (
        "gate",
        &[
            "PASS: check-",
            "FAIL: check-",
            "===== check",
            ": clean (",
            "run-gate",
        ],
    ),
    (
        "hook",
        &[
            "<system-reminder>",
            "PreToolUse",
            "PostToolUse",
            "SessionStart",
            "bash-guard",
            "hook error",
        ],
    ),
    (
        "stage",
        &[
            "lifecycle-kit/templates/stages",
            "enter-stage",
            "WORKFLOW-STATE",
            "Execute the template at",
        ],
    ),
    (
        "govdoc",
        &["SPEC.md", "SPEC-", "CLAUDE.md", "DOCTRINE.md", "BRIEF.local"],
    ),
];

#[derive(Default)]
pub struct Counts {
    pub total: u64,
    pub gate: u64,
    pub hook: u64,
    pub stage: u64,
    pub govdoc: u64,
}

impl Counts {
    pub fn gov(&self) -> u64 {
        self.gate + self.hook + self.stage + self.govdoc
    }
    pub fn task(&self) -> u64 {
        self.total - self.gov()
    }
}

// spec: drift-kit/SPEC.md §The overhead meter — the byte-proxy contract. `LC_ALL=C` was what made
// awk's `length()` count bytes; the raw slice's length needs no locale, so the shell's comment
// about it retires here.
// comment-tier-exempt: the lossy decode is a local hazard nothing above this line can state — it
// feeds *matching* only, and every marker is ASCII, which a lossy decode neither creates nor
// destroys, so an invalid byte can neither move a line between categories nor enter the count
pub fn classify(body: &[u8]) -> Counts {
    let mut c = Counts::default();
    for raw in body.split(|b| *b == b'\n') {
        let bytes = raw.len() as u64;
        c.total += bytes;
        let line = String::from_utf8_lossy(raw);
        for (name, pats) in MARKERS {
            if pats.iter().any(|p| line.contains(p)) {
                match name {
                    "gate" => c.gate += bytes,
                    "hook" => c.hook += bytes,
                    "stage" => c.stage += bytes,
                    _ => c.govdoc += bytes,
                }
                break;
            }
        }
    }
    c
}

// spec: drift-kit/SPEC.md §The overhead meter — the percentage is awk's integer half-up, ported as
// the same integer expression rather than through a float: the shell's arithmetic is exact in
// integers, and a float round-trip is the only way to disagree with the series already logged.
pub fn pct(part: u64, total: u64) -> u64 {
    if total == 0 {
        return 0;
    }
    (part * 100 + total / 2) / total
}

// spec: drift-kit/SPEC.md §The overhead meter — `session8` is the dedup key the meter reads on
// append: re-measuring a session replaces its line rather than double-counting it. The owner doc
// rules this read-filter-rewrite; the crate's append-only doctrine governs *capture* logs instead.
// comment-tier-exempt: the concurrency limit is this implementation's own and no section asserts
// otherwise — two meters finishing at once can lose one row, which is what the shell did too, so
// the port narrows nothing and closing it is not this member's work
fn rewrite_keyed(path: &std::path::Path, session8: &str, line: &str) -> std::io::Result<()> {
    use std::io::Write;
    let marker = format!(" {} total=", session8);
    let mut kept = String::new();
    if let Ok(existing) = std::fs::read(path) {
        for l in String::from_utf8_lossy(&existing).lines() {
            if !l.contains(&marker) {
                kept.push_str(l);
                kept.push('\n');
            }
        }
    }
    kept.push_str(line);
    kept.push('\n');
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let mut f = std::fs::File::create(path)?;
    f.write_all(kept.as_bytes())
}

// spec: drift-kit/SPEC.md §The overhead meter — the meter measures the transcript the invoking
// session is itself running in, so a delegated session resolves its own subagent transcript rather
// than its lead's, and every other session takes the two-tier scan.
// comment-tier-exempt: which `Inputs` fields carry that is a local construction the section does
// not fix — the harness id rides only beside the child flag, because the pair is what narrows the
// scan while the id alone is source 2, which answers with an id where this tool needs the file
fn inputs() -> Result<crate::sessions::Inputs, String> {
    let var = |n: &str| std::env::var(n).unwrap_or_default();
    let pwd = var("PWD");
    let here = if pwd.is_empty() {
        crate::walk::cwd()?
    } else {
        pwd
    };
    let child = var("CLAUDE_CODE_CHILD_SESSION");
    let harness = if child.is_empty() {
        String::new()
    } else {
        var("CLAUDE_CODE_SESSION_ID")
    };
    Ok(crate::sessions::Inputs {
        session_id: String::new(),
        harness_id: harness,
        child,
        sessions_dir: crate::walk::knob_scalar("DRIFT_KIT_SESSIONS_DIR")?,
        config_home: var("CLAUDE_CONFIG_DIR"),
        home: var("HOME"),
        here,
    })
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let given = super::file_survey::positionals(args, "transcript")?;
    if given.len() > 1 {
        return Err(USAGE.to_string());
    }
    let transcript = match given.first() {
        Some(t) => t.clone(),
        None => crate::sessions::resolve(&inputs()?).unwrap_or_default(),
    };
    // spec: drift-kit/SPEC.md §The overhead meter — advisory by construction: a missing transcript
    // is a notice at exit 0, never a refusal, so it is returned as the arm's document.
    if transcript.is_empty() || !std::path::Path::new(&transcript).is_file() {
        return Ok(format!(
            "overhead-meter: no transcript to measure{}\n  help: pass a transcript path, or set \
             DRIFT_KIT_SESSIONS_DIR to the agent transcript dir.\n",
            if transcript.is_empty() {
                String::new()
            } else {
                format!(": {}", transcript)
            }
        ));
    }
    let body = std::fs::read(&transcript)
        .map_err(|e| format!("cannot read the transcript {}: {}", transcript, e))?;
    let c = classify(&body);
    let (gov, task) = (c.gov(), c.task());
    let (p, tp) = (pct(gov, c.total), pct(task, c.total));

    let session8 = crate::sessions::key(&transcript);
    let today = super::kpi::today_iso();
    let log = crate::walk::knob_scalar("DRIFT_KIT_OVERHEAD_LOG")?;
    let line = format!(
        "{} {} total={} gov={} gate={} pct={}",
        today, session8, c.total, gov, c.gate, p
    );
    rewrite_keyed(std::path::Path::new(&log), &session8, &line)
        .map_err(|e| format!("cannot write {}: {}", log, e))?;

    Ok(format!(
        "overhead-meter: {} {}\n  total={} bytes\n  governance={} ({}%)  [gate={} hook={} \
         stage={} govdoc={}]\n  task={} ({}%)\n  (byte-proxy at line granularity — a proportion \
         across same-shape sessions, not tokens; drift-kit/SPEC.md §The overhead meter)\n  \
         logged: {}\n",
        today, session8, c.total, gov, p, c.gate, c.hook, c.stage, c.govdoc, task, tp, log
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §The overhead meter — the marker table is first-match-wins over the
    // whole line, so a line carrying two categories' markers scores once, to the earlier row.
    #[test]
    fn the_first_matching_row_wins_and_an_unmatched_line_falls_to_task() {
        let body = b"PASS: check-x and SPEC.md\n<system-reminder>\nordinary work\n" as &[u8];
        let c = classify(body);
        assert_eq!(c.gate, 25, "the gate row must claim the line SPEC.md also matched");
        assert_eq!(c.govdoc, 0);
        assert_eq!(c.hook, 17);
        assert_eq!(c.total, 25 + 17 + 13);
        assert_eq!(c.task(), 13, "the unmatched line is task, and only it");
    }

    // spec: drift-kit/SPEC.md §The overhead meter — the byte-proxy contract counts *bytes*, so a
    // multi-byte character contributes its encoded width and not one character.
    #[test]
    fn the_count_is_bytes_rather_than_characters() {
        assert_eq!(classify("é\n".as_bytes()).total, 2);
        assert_eq!(classify(b"ab" as &[u8]).total, 2, "a final line with no newline is counted");
    }

    // spec: drift-kit/SPEC.md §The overhead meter — awk's integer half-up, which rounds .5 away
    // from zero where truncation would not.
    #[test]
    fn the_percentage_is_integer_half_up() {
        assert_eq!(pct(1, 8), 13, "12.5% rounds up where truncation would report 12");
        assert_eq!(pct(1, 3), 33);
        assert_eq!(pct(2, 3), 67);
        assert_eq!(pct(0, 0), 0, "an empty transcript reports zero rather than dividing");
    }

    // spec: drift-kit/SPEC.md §The overhead meter — the dedup key is `session8` and the filter is
    // over ` <session8> total=`, so a re-measure replaces exactly its own line and leaves every
    // other session's standing.
    #[test]
    fn a_re_measure_replaces_its_own_line_and_keeps_the_others() {
        let dir = std::env::temp_dir().join(format!("checkwright-ovh.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let log = dir.join("overhead-log.txt");
        rewrite_keyed(&log, "aaaabbbb", "2026-01-01 aaaabbbb total=1 gov=0 gate=0 pct=0")
            .expect("the first append must create the log");
        rewrite_keyed(&log, "ccccdddd", "2026-01-01 ccccdddd total=2 gov=0 gate=0 pct=0")
            .expect("a second session must append");
        rewrite_keyed(&log, "aaaabbbb", "2026-01-02 aaaabbbb total=9 gov=0 gate=0 pct=0")
            .expect("a re-measure must rewrite");
        let text = std::fs::read_to_string(&log).expect("the log must be readable");
        assert_eq!(text.lines().count(), 2, "the re-measure doubled a session: {}", text);
        assert!(text.contains("2026-01-02 aaaabbbb total=9"), "{}", text);
        assert!(text.contains("2026-01-01 ccccdddd total=2"), "{}", text);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
