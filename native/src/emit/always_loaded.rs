// spec: context-kit/SPEC.md §The always-loaded meter — the standing per-session surface against
// the committed baseline. Two levels: `measure` produces the figures and `emit` renders the three
// modes over them, which is what lets `kpi-always-loaded` read them as data (§bin/footprint).
use crate::proc;
use crate::walk;

// spec: context-kit/SPEC.md §The always-loaded meter — the four knobs `lib/context.sh` defines and
// the bridge resolves by sourcing it; the two `GATE_SDK_*` names it already rides into these are
// deliberately absent, since declaring either would resolve one fact twice.
pub const KNOBS: &[&str] = &[
    "CONTEXT_KIT_SURFACES",
    "CONTEXT_KIT_HOOK_CMD",
    "CONTEXT_KIT_BASELINE_FILE",
    "CONTEXT_KIT_GROWTH_PATHS",
];

// spec: context-kit/SPEC.md §The always-loaded meter — the usage an unrecognized mode operand
// prints, the half of the bin/-tool contract that does not retire to the front-end.
const USAGE: &str = "usage: --emit always-loaded [--growth | --update-baseline]\n  bare: the total, its per-part split and the delta since the baseline; --growth: per-file net growth since the baseline commit; --update-baseline: rewrite the baseline row";

const BASELINE_HEADER: &str = "# contract: context-kit/SPEC.md §The always-loaded meter";

// spec: context-kit/SPEC.md §The always-loaded meter — the measurement: every field is one the
// rendered line already carried, read by the arm and by `kpi-always-loaded`. The baseline row's
// fourth field is not one — its only reader is the update mode's rewrite, which reads the row.
pub struct Measurement {
    pub surface: u64,
    pub hook: u64,
    pub total: u64,
    pub base_total: Option<u64>,
    pub base_commit: String,
}

// spec: context-kit/SPEC.md §The always-loaded meter — `[[:space:]]` as the holder's `grep -E`
// reads it inside one line, so a carriage return or a form feed leads a blank line here too.
fn trim_space(s: &str) -> &str {
    s.trim_matches([' ', '\t', '\u{b}', '\u{c}', '\r'])
}

// spec: context-kit/SPEC.md §The always-loaded meter — `read -r total _ commit extra` over the
// baseline row: three whitespace-delimited fields and then the remainder verbatim, which is what
// preserves a consumer's fourth field through the update mode's rewrite.
fn split_row(line: &str) -> (String, String, String) {
    let mut rest = line.trim_start_matches([' ', '\t', '\u{b}', '\u{c}', '\r']);
    let mut fields: Vec<String> = Vec::new();
    for _ in 0..3 {
        let end = rest
            .find([' ', '\t', '\u{b}', '\u{c}', '\r'])
            .unwrap_or(rest.len());
        fields.push(rest[..end].to_string());
        rest = rest[end..].trim_start_matches([' ', '\t', '\u{b}', '\u{c}', '\r']);
    }
    (
        fields[0].clone(),
        fields[2].clone(),
        trim_space(rest).to_string(),
    )
}

// spec: context-kit/SPEC.md §The always-loaded meter — the baseline row is the file's first line
// that is neither blank nor a comment; an absent file has no row, which is the no-delta reading.
fn baseline_row(path: &str) -> Option<(String, String, String)> {
    let text = std::fs::read(path).ok()?;
    let text = String::from_utf8_lossy(&text).into_owned();
    let line = text
        .lines()
        .find(|l| {
            let t = trim_space(l);
            !t.is_empty() && !t.starts_with('#')
        })?
        .to_string();
    Some(split_row(&line))
}

// spec: context-kit/SPEC.md §The always-loaded meter — the hook body is measured by running
// whatever command the knob names, through `bash -c`, and counting the lines it wrote: the knob is
// a consumer command seam, so the spawn is the contract rather than an implementation detail.
fn hook_lines(cmd: &str) -> u64 {
    let out = match proc::run_streamed("bash", &["-c", cmd], b"", proc::Stderr::Discard) {
        Ok(o) => o,
        // spec: context-kit/SPEC.md §The always-loaded meter — a hook command that could not run
        // contributes nothing, the holder's `|| true`: its stdout is read whatever its status, so
        // a failing command that still printed is counted on what it printed.
        Err(_) => return 0,
    };
    let text = String::from_utf8_lossy(out.stdout()).into_owned();
    let body = text.trim_end_matches('\n');
    if body.is_empty() {
        return 0;
    }
    body.matches('\n').count() as u64 + 1
}

pub fn measure() -> Result<Measurement, String> {
    let mut surface = 0u64;
    for f in walk::knob_array("CONTEXT_KIT_SURFACES")? {
        if !std::path::Path::new(&f).is_file() {
            continue;
        }
        if let Ok(b) = std::fs::read(&f) {
            surface += b.iter().filter(|&&c| c == b'\n').count() as u64;
        }
    }
    let cmd = walk::knob_scalar("CONTEXT_KIT_HOOK_CMD")?;
    let hook = if cmd.is_empty() { 0 } else { hook_lines(&cmd) };
    let (row_total, base_commit) = match baseline_row(&walk::knob_scalar("CONTEXT_KIT_BASELINE_FILE")?) {
        Some((total, commit, _)) => (total, commit),
        None => (String::new(), String::new()),
    };
    // spec: context-kit/SPEC.md §The always-loaded meter — `^[0-9]+$`: a row whose total is not a
    // bare non-negative integer carries no delta, rather than one read off a partial parse.
    let base_total = if !row_total.is_empty() && row_total.bytes().all(|c| c.is_ascii_digit()) {
        row_total.parse::<u64>().ok()
    } else {
        None
    };
    Ok(Measurement {
        surface,
        hook,
        total: surface + hook,
        base_total,
        base_commit,
    })
}

// spec: context-kit/SPEC.md §The always-loaded meter — `${commit:0:8}`, the short form both the
// bare line's delta suffix and the update mode's confirmation carry.
fn short(commit: &str) -> String {
    commit.chars().take(8).collect()
}

// spec: context-kit/SPEC.md §The always-loaded meter — the per-part parenthetical, shared by the
// bare line and the update mode's confirmation so the two cannot render one figure two ways.
fn parts(m: &Measurement) -> String {
    format!(
        "{}l (surfaces {} \u{b7} hook {})",
        m.total, m.surface, m.hook
    )
}

// spec: context-kit/SPEC.md §The always-loaded meter — the default invocation: total, per-part
// breakdown, and the delta against the baseline when the row carries a numeric total.
pub fn line(m: &Measurement) -> String {
    match m.base_total {
        Some(base) => format!(
            "{}  {:+} since {}",
            parts(m),
            m.total as i64 - base as i64,
            short(&m.base_commit)
        ),
        None => parts(m),
    }
}

fn commit_resolves(commit: &str) -> bool {
    proc::run(
        "git",
        &["rev-parse", "-q", "--verify", &format!("{}^{{commit}}", commit)],
    )
    .map(|c| c.stdout().is_some())
    .unwrap_or(false)
}

// spec: context-kit/SPEC.md §The always-loaded meter — the growth worklist: every governed prose
// file whose net line growth since the baseline commit is positive, largest first, a tie broken by
// descending path bytes because that is what `sort -rn`'s last-resort comparison gives under `-r`.
fn growth(m: &Measurement, baseline_file: &str) -> Result<String, String> {
    if m.base_commit.is_empty() || !commit_resolves(&m.base_commit) {
        return Ok(format!(
            "growth: no resolvable baseline commit in {}\n",
            baseline_file
        ));
    }
    let paths = walk::knob_array("CONTEXT_KIT_GROWTH_PATHS")?;
    let mut args: Vec<&str> = vec!["diff", "--numstat", &m.base_commit, "--"];
    args.extend(paths.iter().map(String::as_str));
    let numstat = proc::run("git", &args)
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
        .unwrap_or_default();
    let mut rows: Vec<(i64, String)> = Vec::new();
    for l in numstat.lines() {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 3 || f[0] == "-" {
            continue;
        }
        let net = f[0].parse::<i64>().unwrap_or(0) - f[1].parse::<i64>().unwrap_or(0);
        if net > 0 {
            rows.push((net, f[2].to_string()));
        }
    }
    rows.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
    let net: i64 = rows.iter().map(|(d, _)| d).sum();
    let mut out = format!(
        "growth since {}: {} file(s) grew, +{} net line(s)\n",
        short(&m.base_commit),
        rows.len(),
        net
    );
    for (d, f) in &rows {
        out.push_str(&format!("  +{}\t{}\n", d, f));
    }
    Ok(out)
}

// spec: context-kit/SPEC.md §The always-loaded meter — the close-stage act: the header, the
// `<total> <surface> <commit>` row and a consumer's trailing extra fields carried verbatim; the
// write is checked, so no confirmation line can report a rewrite that did not happen.
fn update_baseline(m: &Measurement, baseline_file: &str) -> Result<String, String> {
    let commit = proc::run("git", &["rev-parse", "HEAD"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    let extra = baseline_row(baseline_file).map_or(String::new(), |(_, _, e)| e);
    let mut row = format!("{} {} {}", m.total, m.surface, commit);
    if !extra.is_empty() {
        row.push(' ');
        row.push_str(&extra);
    }
    std::fs::write(baseline_file, format!("{}\n{}\n", BASELINE_HEADER, row))
        .map_err(|e| format!("cannot write {}: {}", baseline_file, e))?;
    Ok(format!(
        "always-loaded baseline updated: {} @ {}\n",
        parts(m),
        short(&commit)
    ))
}

// spec: context-kit/SPEC.md §The always-loaded meter — the modes are operands drawn from a closed
// set, and one outside it is a refusal: a typo of `--update-baseline` would otherwise print an
// ordinary meter line at exit 0 while writing no baseline at all.
fn mode(args: &[String]) -> Result<Option<&str>, String> {
    let mut chosen: Option<&str> = None;
    for a in args {
        if a.is_empty() {
            continue;
        }
        if a != "--growth" && a != "--update-baseline" {
            return Err(format!("unrecognized mode: {}\n{}", a, USAGE));
        }
        if chosen.is_none() {
            chosen = Some(a.as_str());
        }
    }
    Ok(chosen)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let selected = mode(args)?;
    let m = measure()?;
    let baseline_file = walk::knob_scalar("CONTEXT_KIT_BASELINE_FILE")?;
    match selected {
        Some("--growth") => growth(&m, &baseline_file),
        Some("--update-baseline") => update_baseline(&m, &baseline_file),
        _ => Ok(format!("always-loaded: {}\n", line(&m))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: context-kit/SPEC.md §The always-loaded meter — the mode operand is drawn from a closed
    // set, so a misspelling is refused rather than absorbed into the bare reading; an empty operand
    // is the holder's own `${1:-}` reading and selects the bare mode
    #[test]
    fn an_unrecognized_mode_operand_is_refused_and_an_empty_one_is_the_bare_reading() {
        for bad in ["--growht", "--help", "-h", "--update-baselines", "growth"] {
            let err = mode(&argv(&[bad]))
                .expect_err("a mode operand outside the closed set fell through to the bare mode");
            assert!(err.contains(bad), "the refusal named no offender: {}", err);
            assert!(err.contains("usage:"), "the refusal printed no usage: {}", err);
        }
        assert_eq!(mode(&argv(&[])).expect("an empty argv was refused"), None);
        assert_eq!(mode(&argv(&[""])).expect("an empty operand was refused"), None);
        assert_eq!(
            mode(&argv(&["--growth"])).expect("a legitimate mode was refused"),
            Some("--growth")
        );
    }

    // spec: context-kit/SPEC.md §The always-loaded meter — the baseline row's fourth field is the
    // consumer's own and survives the split, and a row shorter than three fields reads as empty
    // rather than as a mis-assigned commit
    #[test]
    fn the_baseline_row_splits_three_fields_and_keeps_the_remainder_verbatim() {
        assert_eq!(
            split_row("213 203 de046195 41 extra"),
            (
                "213".to_string(),
                "de046195".to_string(),
                "41 extra".to_string()
            )
        );
        assert_eq!(
            split_row("  213   203   de046195  "),
            ("213".to_string(), "de046195".to_string(), String::new())
        );
        assert_eq!(
            split_row("213 203"),
            ("213".to_string(), String::new(), String::new())
        );
    }

    // spec: context-kit/SPEC.md §The always-loaded meter — the default invocation's line, and a
    // baseline whose total is not a number carries no delta rather than a nonsense one
    #[test]
    fn the_bare_line_carries_the_parts_and_only_a_numeric_baseline_adds_a_delta() {
        let m = Measurement {
            surface: 203,
            hook: 10,
            total: 213,
            base_total: Some(211),
            base_commit: "de0461952f".to_string(),
        };
        assert_eq!(line(&m), "213l (surfaces 203 \u{b7} hook 10)  +2 since de046195");
        let m = Measurement {
            base_total: None,
            ..m
        };
        assert_eq!(line(&m), "213l (surfaces 203 \u{b7} hook 10)");
        let m = Measurement {
            base_total: Some(227),
            ..m
        };
        assert_eq!(line(&m), "213l (surfaces 203 \u{b7} hook 10)  -14 since de046195");
    }
}
