// spec: queue-kit/SPEC.md §The lesson-sink arm — the outbound channel's router, an `Arm::Run`
// rather than an `Arm::Emit` member because the sink's exit status *is* this arm's
// spec: gate-sdk/SPEC.md §The non-gate arm — the member the flag-keyed table exists for
use crate::proc::{self, Stderr};
use crate::walk;
use std::io::Read;
use std::io::Write;

// spec: queue-kit/SPEC.md §The lesson-sink arm — the seam survives the port: the sink map stays
// the adopter's configuration and the workflow dir stays the fail-open default's home. Both
// cross the bridge, the first through the keyed-map arm (gate-sdk/SPEC.md §lib/gate.sh).
pub const KNOBS: &[&str] = &["QUEUE_KIT_LESSON_SINKS", "GATE_SDK_WORKFLOW_DIR"];

pub fn run(args: &[String]) -> i32 {
    match route(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("checkwright-gates: --lesson-sink: {}", e);
            2
        }
    }
}

fn route(args: &[String]) -> Result<i32, String> {
    let mut tag = String::new();
    for a in args {
        if a.starts_with('-') {
            return Err(format!("unknown option: {}", a));
        }
        tag = a.clone();
    }
    if tag.is_empty() {
        return Err("needs a <tag>".to_string());
    }

    // spec: queue-kit/SPEC.md §The lesson-sink arm — stdin is buffered rather than streamed, so
    // the bound on a body is the queue's own per-entry cap
    let mut body: Vec<u8> = Vec::new();
    std::io::stdin()
        .read_to_end(&mut body)
        .map_err(|e| format!("cannot read the lesson body on stdin: {}", e))?;

    let sinks = walk::knob_map("QUEUE_KIT_LESSON_SINKS")?;
    if let Some((_, command)) = sinks.iter().find(|(k, _)| *k == tag) {
        return spawn_sink(command, &body);
    }
    fallback(&tag, &body)
}

// spec: queue-kit/SPEC.md §The lesson-sink arm — spawn-and-report: `Streamed::code()` already
// wraps the signal-aware exit-code spelling, and `run_streamed` captures stdout rather than
// inheriting it, so the arm re-emits what the child wrote
fn spawn_sink(command: &str, body: &[u8]) -> Result<i32, String> {
    let done = proc::run_streamed("bash", &["-c", command], body, Stderr::Inherit)?;
    std::io::stdout()
        .write_all(done.stdout())
        .map_err(|e| format!("cannot re-emit the sink's output: {}", e))?;
    Ok(done.code())
}

// spec: queue-kit/SPEC.md §The lesson-sink arm — the unconfigured tag falls **open**, appending
// to `<workflow-dir>/<tag>-harvest.md`, which is what keeps a fresh clone closing cleanly and
// preserves the staging file's documented reclaim path.
fn fallback(tag: &str, body: &[u8]) -> Result<i32, String> {
    let dir = walk::knob_scalar("GATE_SDK_WORKFLOW_DIR")?;
    let path = format!("{}/{}-harvest.md", dir, tag);
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("cannot open the harvest file {}: {}", path, e))?;
    f.write_all(body)
        .map_err(|e| format!("cannot append to the harvest file {}: {}", path, e))?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: queue-kit/SPEC.md §The lesson-sink arm — the argv is one `<tag>` and nothing else, so
    // a missing tag and an option are both refusals rather than a silent no-op
    #[test]
    fn the_argv_is_one_tag_and_a_missing_one_refuses() {
        assert_eq!(run(&[]), 2);
        assert_eq!(run(&["--nope".to_string()]), 2);
    }

    // spec: queue-kit/SPEC.md §The lesson-sink arm — the fail-open default is a *bridged* read
    // now, so an adopter who deleted the knob from their config gets a refusal rather than a
    // silent write to a platform default
    #[test]
    fn the_fallback_appends_under_the_bridged_workflow_dir() {
        let knobs = crate::knobenv::lock();
        let dir = std::env::temp_dir().join(format!("checkwright-sink-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot create the sandbox");
        knobs.set(
            "GATE_SDK_KNOB_GATE_SDK_WORKFLOW_DIR",
            &dir.display().to_string(),
        );
        assert_eq!(fallback("lesson", b"one body\n"), Ok(0));
        assert_eq!(fallback("lesson", b"two body\n"), Ok(0));
        let got = std::fs::read_to_string(dir.join("lesson-harvest.md")).expect("no harvest file");
        assert_eq!(got, "one body\ntwo body\n", "the fallback is append, not truncate");
        std::fs::remove_dir_all(&dir).ok();
        knobs.remove("GATE_SDK_KNOB_GATE_SDK_WORKFLOW_DIR");
        assert!(fallback("lesson", b"x").is_err(), "an unset knob must refuse");
    }
}
