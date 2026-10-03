// spec: installer/SPEC.md §The front door's verbs — the pinned release and the pending window, read
// once for every gate holding a front-door claim to them, so no two can disagree about either
use super::install_pin::{is_triple, pin_of};
use super::release_bump::disposition_file;
use crate::{fresh, proc, programs, queue, stages};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub(crate) enum Disposition {
    Absent,
    Release,
    Withheld(String),
}

// spec: installer/SPEC.md §The front door's verbs — only the line keyed by the queue's iteration,
// and only its field, in one of three forms
fn disposition(path: &str, text: &str, iteration: &str) -> Result<Disposition, String> {
    let mut field: Option<&str> = None;
    for line in text.lines() {
        let mut f = line.split_whitespace();
        if f.next() == Some(iteration) && f.next() == Some("release") {
            field = Some(f.next().unwrap_or(""));
        }
    }
    let Some(field) = field else {
        return Ok(Disposition::Absent);
    };
    if field == "none" || field.strip_prefix("deferred:v").is_some_and(is_triple) {
        return Ok(Disposition::Withheld(field.to_string()));
    }
    if field.strip_prefix('v').is_some_and(is_triple) {
        return Ok(Disposition::Release);
    }
    Err(format!(
        "{}: iteration {}'s disposition field '{}' is none of vX.Y.Z, none or deferred:vX.Y.Z",
        path, iteration, field
    ))
}

fn git_ok(args: &[&str]) -> Result<Option<String>, String> {
    let c = proc::run(&programs::GIT, args)?;
    if c.code() != Some(0) {
        return Ok(None);
    }
    Ok(c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
}

pub(crate) fn read(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

// spec: installer/SPEC.md §The front door's verbs — the tag the hosted pin names, or none where it
// does not resolve here, as in a shallow checkout
pub(crate) fn pinned_tag() -> Result<Option<String>, String> {
    let install_sh = crate::walk::knob_scalar("GATE_LOCAL_INSTALL_SH")?;
    let pin = pin_of(&install_sh, &fresh::read_captured(&install_sh)?, "pin")?;
    let tag = format!("v{}", pin);
    Ok(git_ok(&["rev-parse", "--verify", "--quiet", &format!("refs/tags/{}", tag)])?.map(|_| tag))
}

// spec: installer/SPEC.md §The front door's verbs — a resolved tag that carries no such file is a
// refusal, never a dormant arm
pub(crate) fn show_at(tag: &str, path: &str) -> Result<String, String> {
    git_ok(&["show", &format!("{}:{}", tag, path)])?
        .ok_or_else(|| format!("the tag {} exists and carries no {}", tag, path))
}

// spec: plugin/SPEC.md §check-plugin-parity — whether a resolved tag carries a path, where the
// absence is the answer rather than a refusal
pub(crate) fn carries(tag: &str, path: &str) -> Result<bool, String> {
    Ok(git_ok(&["cat-file", "-e", &format!("{}:{}", tag, path)])?.is_some())
}

// spec: installer/SPEC.md §The front door's verbs — the live disposition file and queue file
pub(crate) fn live_paths() -> Result<(String, String), String> {
    Ok((disposition_file()?, queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?))
}

// spec: installer/SPEC.md §The front door's verbs — the iteration the queue header names, and its
// disposition line read; an absent file is an absent line
pub(crate) fn iteration_disposition(
    disposition_path: &str,
    queue_path: &str,
) -> Result<(String, Disposition), String> {
    let queue_text = read(queue_path)?;
    let iteration = stages::header(&queue_text)
        .map(stages::header_iter)
        .ok_or_else(|| format!("{} carries no '## Iteration:' header", queue_path))?;
    let disp = if Path::new(disposition_path).is_file() {
        disposition(disposition_path, &read(disposition_path)?, &iteration)?
    } else {
        Disposition::Absent
    };
    Ok((iteration, disp))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The front door's verbs — the three disposition forms, the absent
    // line, and a field of none of them
    #[test]
    fn the_disposition_reads_three_forms_and_the_absent_line() {
        let d = |t: &str| disposition("d", t, "it");
        assert_eq!(d("other release none — b\n"), Ok(Disposition::Absent));
        assert_eq!(d("it release v1.2.3 — b\n"), Ok(Disposition::Release));
        assert_eq!(d("it release none — b\n"), Ok(Disposition::Withheld("none".to_string())));
        assert_eq!(
            d("it release deferred:v1.2.3 — b\n"),
            Ok(Disposition::Withheld("deferred:v1.2.3".to_string()))
        );
        assert!(d("it release soon — b\n").is_err());
        assert!(d("it release deferred:1.2 — b\n").is_err());
    }
}
