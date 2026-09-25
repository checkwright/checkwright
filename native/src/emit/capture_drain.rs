// spec: gate-sdk/SPEC.md §The workflow directory — the rotation drain: a capture log a close procedure
// reads is renamed aside before the read, so a line a concurrent writer appends lands in a fresh live
// log rather than being erased by a truncate. The log is an operand, so the arm declares no knob.
use std::io::Write;

pub const KNOBS: &[&str] = &[];

const USAGE: &str = "usage: --emit capture-drain [--done] [--] <log>\n  moves every unread line of <log> into <log>.drain, leaves <log> present and empty, and prints the drain's path when it holds a line; --done removes <log>.drain";

pub fn emit(args: &[String]) -> Result<String, String> {
    let (done, log) = parse(args)?;
    let drain = format!("{}.drain", log);
    if done {
        return remove_if_present(&drain).map(|_| String::new());
    }
    let part = format!("{}.drain.part", log);
    // spec: gate-sdk/SPEC.md §The workflow directory — a part file found at the start is a crash
    // inside an earlier run, so it is folded in first: a crash can repeat lines and never drops one
    if exists(&part) {
        fold(&part, &drain)?;
    }
    if exists(&log) {
        if exists(&drain) {
            rename(&log, &part)?;
            fold(&part, &drain)?;
        } else {
            rename(&log, &drain)?;
        }
    }
    // spec: gate-sdk/SPEC.md §The workflow directory — an append-open never truncates, so a line a
    // writer landed after the rename survives in the live log
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log)
        .map_err(|e| format!("cannot reopen the live log {}: {}", log, e))?;
    if holds_a_line(&drain) {
        return Ok(format!("{}\n", drain));
    }
    remove_if_present(&drain)?;
    Ok(String::new())
}

fn parse(args: &[String]) -> Result<(bool, String), String> {
    let mut done = false;
    let mut operands: Vec<&String> = Vec::new();
    let mut options = true;
    for a in args {
        match a.as_str() {
            "--" if options => options = false,
            "--done" if options => done = true,
            s if options && s.starts_with('-') => {
                return Err(format!("unknown option: {}\n{}", s, USAGE));
            }
            _ => operands.push(a),
        }
    }
    match operands.as_slice() {
        [log] if !log.is_empty() => Ok((done, log.to_string())),
        _ => Err(USAGE.to_string()),
    }
}

fn exists(p: &str) -> bool {
    std::path::Path::new(p).exists()
}

fn holds_a_line(p: &str) -> bool {
    std::fs::metadata(p).map(|m| m.len() > 0).unwrap_or(false)
}

fn rename(from: &str, to: &str) -> Result<(), String> {
    std::fs::rename(from, to).map_err(|e| format!("cannot rename {} to {}: {}", from, to, e))
}

// spec: gate-sdk/SPEC.md §The workflow directory — append a part file's bytes to the drain, then
// remove it; the removal comes last, so a crash between the two repeats the part on the next run
fn fold(part: &str, drain: &str) -> Result<(), String> {
    let body = std::fs::read(part).map_err(|e| format!("cannot read {}: {}", part, e))?;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(drain)
        .map_err(|e| format!("cannot open {}: {}", drain, e))?;
    f.write_all(&body)
        .map_err(|e| format!("cannot append to {}: {}", drain, e))?;
    std::fs::remove_file(part).map_err(|e| format!("cannot remove {}: {}", part, e))
}

fn remove_if_present(p: &str) -> Result<(), String> {
    match std::fs::remove_file(p) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("cannot remove {}: {}", p, e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Sandbox(std::path::PathBuf);

    impl Sandbox {
        fn new(name: &str) -> Sandbox {
            let d = std::env::temp_dir().join(format!("checkwright-drain-{}-{}", name, std::process::id()));
            let _ = std::fs::remove_dir_all(&d);
            std::fs::create_dir_all(&d).expect("the sandbox must be creatable");
            Sandbox(d)
        }
        fn at(&self, n: &str) -> String {
            self.0.join(n).display().to_string()
        }
        fn read(&self, n: &str) -> Option<String> {
            std::fs::read_to_string(self.0.join(n)).ok()
        }
        fn write(&self, n: &str, body: &str) {
            std::fs::write(self.0.join(n), body).expect("the sandbox must be writable");
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn drain(args: &[&str]) -> Result<String, String> {
        emit(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    // spec: gate-sdk/SPEC.md §The workflow directory — the first drain renames the log aside, leaves
    // the live log present and empty, and prints the drain's path
    #[test]
    fn a_first_drain_renames_the_log_and_leaves_it_present_and_empty() {
        let s = Sandbox::new("first");
        s.write("f.log", "one\ntwo\n");
        assert_eq!(drain(&[&s.at("f.log")]), Ok(format!("{}\n", s.at("f.log.drain"))));
        assert_eq!(s.read("f.log.drain").as_deref(), Some("one\ntwo\n"));
        assert_eq!(s.read("f.log").as_deref(), Some(""));
    }

    // spec: gate-sdk/SPEC.md §The workflow directory — a line appended after the rename lands in the
    // live log, and a second drain appends it behind the unfinished drain's lines
    #[test]
    fn a_line_after_the_rename_survives_and_a_leftover_drain_is_appended_to() {
        let s = Sandbox::new("leftover");
        s.write("f.log", "one\n");
        drain(&[&s.at("f.log")]).expect("the first drain");
        let mut f = std::fs::OpenOptions::new().append(true).open(s.at("f.log")).expect("live log");
        f.write_all(b"late\n").expect("the late line");
        assert_eq!(s.read("f.log").as_deref(), Some("late\n"));
        drain(&[&s.at("f.log")]).expect("the second drain");
        assert_eq!(s.read("f.log.drain").as_deref(), Some("one\nlate\n"));
        assert_eq!(s.read("f.log").as_deref(), Some(""));
        assert_eq!(s.read("f.log.drain.part"), None);
    }

    // spec: gate-sdk/SPEC.md §The workflow directory — a writer holding the log open across the
    // rename does not block it, and its next line lands in the drain: the honest limit's window
    #[test]
    fn a_log_held_open_by_a_writer_still_rotates() {
        let s = Sandbox::new("held");
        s.write("f.log", "one\n");
        let mut held = std::fs::OpenOptions::new().append(true).open(s.at("f.log")).expect("held");
        drain(&[&s.at("f.log")]).expect("the drain under an open handle");
        held.write_all(b"in flight\n").expect("the in-flight line");
        assert_eq!(s.read("f.log.drain").as_deref(), Some("one\nin flight\n"));
        assert_eq!(s.read("f.log").as_deref(), Some(""));
    }

    // spec: gate-sdk/SPEC.md §The workflow directory — a part file left by a crash is folded in
    // first, ahead of the log's own lines
    #[test]
    fn a_leftover_part_is_recovered_before_the_log() {
        let s = Sandbox::new("part");
        s.write("f.log.drain", "old\n");
        s.write("f.log.drain.part", "crashed\n");
        s.write("f.log", "new\n");
        drain(&[&s.at("f.log")]).expect("the drain");
        assert_eq!(s.read("f.log.drain").as_deref(), Some("old\ncrashed\nnew\n"));
        assert_eq!(s.read("f.log.drain.part"), None);
    }

    // spec: gate-sdk/SPEC.md §The workflow directory — an absent or empty log prints nothing and
    // leaves no drain, and the live log is present afterwards
    #[test]
    fn nothing_to_drain_prints_nothing_and_leaves_no_drain() {
        let s = Sandbox::new("empty");
        assert_eq!(drain(&[&s.at("f.log")]), Ok(String::new()));
        assert_eq!(s.read("f.log").as_deref(), Some(""));
        assert_eq!(drain(&["--", &s.at("f.log")]), Ok(String::new()));
        assert_eq!(s.read("f.log.drain"), None);
    }

    // spec: gate-sdk/SPEC.md §The workflow directory — `--done` removes a present drain and is no
    // error on an absent one, and it leaves the live log alone
    #[test]
    fn done_removes_the_drain_and_tolerates_its_absence() {
        let s = Sandbox::new("done");
        s.write("f.log", "one\n");
        drain(&[&s.at("f.log")]).expect("the drain");
        s.write("f.log", "late\n");
        assert_eq!(drain(&["--done", &s.at("f.log")]), Ok(String::new()));
        assert_eq!(s.read("f.log.drain"), None);
        assert_eq!(s.read("f.log").as_deref(), Some("late\n"));
        assert_eq!(drain(&["--done", &s.at("f.log")]), Ok(String::new()));
    }

    // spec: gate-sdk/SPEC.md §The workflow directory — a missing operand, a second operand and an
    // unknown option each refuse with the usage line; a failed rename names the path
    #[test]
    fn the_refusals() {
        for bad in [&[][..], &["a", "b"][..], &["--nope", "a"][..], &[""][..], &["--done"][..]] {
            let err = drain(bad).expect_err("a malformed argv was accepted");
            assert!(err.contains("usage: --emit capture-drain"), "{:?}: {}", bad, err);
        }
        let s = Sandbox::new("refuse");
        s.write("f.log", "one\n");
        std::fs::create_dir_all(s.0.join("f.log.drain").join("occupied")).expect("the blocker");
        s.write("f.log.drain.part", "x\n");
        let err = drain(&[&s.at("f.log")]).expect_err("a drain that cannot be appended to passed");
        assert!(err.contains(&s.at("f.log.drain")), "the refusal names no path: {}", err);
    }
}
