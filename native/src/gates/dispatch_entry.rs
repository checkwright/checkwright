// spec: lifecycle-kit/SPEC.md §check-dispatch-entry — while the dispatch marker holds a line, a
// commit adds a stamp
use crate::stages;
use crate::{proc, programs, walk};
use std::path::Path;

fn read_file(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {} ({})", path, e))
}

fn staged_paths() -> Result<Vec<String>, String> {
    let c = proc::run(&programs::GIT, &["diff", "--cached", "--name-only", "-z"])?;
    let out = c
        .stdout()
        .ok_or_else(|| "git diff --cached failed — the check could not run; treating as failure (not clean)".to_string())?;
    Ok(String::from_utf8_lossy(out)
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_string)
        .collect())
}

struct Inputs {
    marker: Option<String>,
    staged: Option<String>,
    priors: Vec<String>,
    paths: Vec<String>,
}

fn live() -> Result<Inputs, String> {
    let tmp = walk::knob_scalar("GATE_SDK_TMP_DIR")?;
    let name = walk::knob_scalar("LIFECYCLE_KIT_DISPATCH_MARKER_FILE")?;
    let path = format!("{}/{}", tmp.trim_end_matches('/'), name);
    let marker = if Path::new(&path).exists() {
        Some(read_file(&path)?)
    } else {
        None
    };
    if marker.as_deref().map_or(true, |m| stages::marker_lines(m).is_empty()) {
        return Ok(Inputs { marker, staged: None, priors: Vec::new(), paths: Vec::new() });
    }
    let state = walk::knob_scalar("LIFECYCLE_KIT_STATE_FILE")?;
    let (staged, priors) = match super::stamp_subject::live_state_sides(&state) {
        Some((s, p)) => (Some(s), p),
        None => (None, Vec::new()),
    };
    Ok(Inputs { marker, staged, priors, paths: staged_paths()? })
}

// spec: lifecycle-kit/SPEC.md §check-dispatch-entry — the fixture form: the marker, the staged and
// `HEAD` state-file blobs and a staged-path list, one path per line
fn fixture(args: &[String]) -> Result<Inputs, String> {
    let marker = read_file(&args[0])?;
    let staged = read_file(&args[1])?;
    let head = read_file(&args[2])?;
    let paths = read_file(&args[3])?
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect();
    Ok(Inputs { marker: Some(marker), staged: Some(staged), priors: vec![head], paths })
}

pub fn run(args: &[String]) -> i32 {
    let Some(msg) = args.first() else {
        println!("DISPATCH-ENTRY: clean (no message file argument — the commit-msg hook surface is not a whole-tree target; skipped)");
        return 0;
    };
    if args.len() != 1 && args.len() != 5 {
        eprintln!("check-dispatch-entry: usage: check-dispatch-entry <message-file> [<marker> <staged-state> <head-state> <staged-paths>]");
        return 2;
    }
    if !Path::new(msg).is_file() {
        eprintln!("check-dispatch-entry: message file not found: {}", msg);
        return 2;
    }
    let got = if args.len() == 5 { fixture(&args[1..]) } else { live() };
    let inputs = match got {
        Ok(i) => i,
        Err(e) => {
            eprintln!("check-dispatch-entry: {} — the check could not run; treating as failure (not clean)", e);
            return 2;
        }
    };
    let outstanding: Vec<&str> = inputs.marker.as_deref().map(stages::marker_lines).unwrap_or_default();
    if outstanding.is_empty() {
        println!("DISPATCH-ENTRY: clean (no dispatch declaration stands)");
        return 0;
    }
    let inbox = match walk::knob_scalar("LIFECYCLE_KIT_GAP_INBOX_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-dispatch-entry: {}", e);
            return 2;
        }
    };
    // spec: lifecycle-kit/SPEC.md §check-dispatch-entry — the lead's one sanctioned commit
    if inputs.paths.len() == 1 && inputs.paths[0] == inbox {
        println!("DISPATCH-ENTRY: clean (the commit stages the gap inbox alone — the lead's sanctioned commit)");
        return 0;
    }
    let roster = match stages::stages() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-dispatch-entry: {}", e);
            return 2;
        }
    };
    if let Some(staged) = &inputs.staged {
        let prior: Vec<&str> = inputs.priors.iter().map(String::as_str).collect();
        if let Some(line) = stages::last_added_stamp_over(staged, &prior, &roster) {
            println!("DISPATCH-ENTRY: clean (the commit adds the stamp '{}')", line);
            return 0;
        }
    }
    println!("check-dispatch-entry: a dispatched stage session has not entered, and this commit adds no stamp:");
    for s in &outstanding {
        println!("  outstanding dispatch: {}", s);
    }
    println!("  help: run --enter-stage <stage> and commit the stamp on its own first; a lead whose");
    println!("        dispatched session ended without entering runs --enter-stage --dispatch-withdraw <stage>.");
    1
}
