// spec: delegation-kit/SPEC.md §The statusline arm — the harness's `statusLine` integration
// point: it writes the snapshot `usage-verdict` reads and renders one ANSI-coloured line on stdout.
// The harness ignores the exit status, so this member has none to speak.
use crate::emit::kpi;
use crate::hook::{self, usage};
use crate::hook::usage::Snapshot;
use crate::proc;
use crate::walk;

// spec: delegation-kit/SPEC.md §The statusline arm — the arm's declared reads. NOT
// `DELEGATION_KIT_USAGE_HISTORY`: the shipped producer calls `usage-verdict` nowhere, so the knob
// has no reader here and a declared one would be a field with no named reader.
pub const KNOBS: &[&str] = &[
    "DELEGATION_KIT_USAGE_FILE",
    "DELEGATION_KIT_CRED_FILE",
    "DELEGATION_KIT_ACCOUNT_CONFIG",
];

// spec: delegation-kit/SPEC.md §The statusline arm — the gauge's geometry and its three
// thresholds, the one place either is spelled
const GAUGE_WIDTH: usize = 10;
const BG_HOT: u16 = 124;
const BG_WARM: u16 = 136;
const BG_COOL: u16 = 28;
const BG_EMPTY: u16 = 238;

pub fn run(_args: &[String]) -> i32 {
    let payload = hook::read_payload();
    let knob = |name: &str| walk::knob_scalar(name).unwrap_or_default();
    let usage_file = knob("DELEGATION_KIT_USAGE_FILE");
    let cred_file = knob("DELEGATION_KIT_CRED_FILE");
    let account_config = knob("DELEGATION_KIT_ACCOUNT_CONFIG");

    let at = |path: &[&str]| hook::field(payload.as_ref(), path);
    let five = at(&["rate_limits", "five_hour", "used_percentage"]);
    let five_resets = at(&["rate_limits", "five_hour", "resets_at"]);
    let seven = at(&["rate_limits", "seven_day", "used_percentage"]);
    let seven_resets = at(&["rate_limits", "seven_day", "resets_at"]);

    Snapshot {
        five_hour_used_pct: five.clone(),
        five_hour_resets_at: five_resets.clone(),
        seven_day_used_pct: seven.clone(),
        seven_day_resets_at: seven_resets.clone(),
        account: usage::json_field(&account_config, &["oauthAccount", "accountUuid"]),
        tier: usage::json_field(&cred_file, &["claudeAiOauth", "subscriptionType"]),
    }
    .write(&usage_file);

    let model = slug(&at(&["model", "display_name"]));
    let effort = at(&["effort", "level"]);
    let ctx = at(&["context_window", "used_percentage"]);

    let mut bar = format!(
        "[{}{}]·ctx {}",
        if model.is_empty() { "?" } else { &model },
        if effort.is_empty() {
            String::new()
        } else {
            format!("-{}", effort)
        },
        gauge(&ctx)
    );
    bar.push_str(&format!("·5h {}", gauge(&five)));
    if !five_resets.is_empty() {
        bar.push_str(&format!(" {}", remaining(&five_resets)));
    }
    if !seven.is_empty() {
        bar.push_str(&format!("·7d {}", gauge(&seven)));
        if !seven_resets.is_empty() {
            bar.push_str(&format!(" {}", remaining(&seven_resets)));
        }
    }
    let (iteration, stage, counts) = project();
    if !iteration.is_empty() {
        bar.push_str(&format!(
            "·⟳ {}{}",
            iteration,
            if stage.is_empty() {
                String::new()
            } else {
                format!("@{}", stage)
            }
        ));
    }
    let group = counters(&counts);
    if !group.is_empty() {
        bar.push_str(&format!("·{}", group));
    }
    println!("{}", bar);
    0
}

// spec: delegation-kit/SPEC.md §The statusline arm — the model's first word, lowercased, with
// every non-alphanumeric character dropped: a name the harness may spell any way becomes a token
// narrow enough for a status bar.
fn slug(display_name: &str) -> String {
    display_name
        .split(' ')
        .next()
        .unwrap_or("")
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

// spec: delegation-kit/SPEC.md §The statusline arm — the gauge escapes are self-contained by
// that section's own rule and stay so: no asset leaves the binary. The reading is truncated to its
// integer part and clamped, so a source spelling a percentage any way still renders in ten cells.
fn gauge(reading: &str) -> String {
    let pct: usize = reading
        .split('.')
        .next()
        .unwrap_or("")
        .parse()
        .unwrap_or(0)
        .min(100);
    let filled = pct * GAUGE_WIDTH / 100;
    let bg = if pct >= 80 {
        BG_HOT
    } else if pct >= 50 {
        BG_WARM
    } else {
        BG_COOL
    };
    let label = format!("{}%", pct);
    let left = (GAUGE_WIDTH - label.len()) / 2;
    let cells: Vec<char> = format!(
        "{}{}{}",
        " ".repeat(left),
        label,
        " ".repeat(GAUGE_WIDTH - label.len() - left)
    )
    .chars()
    .collect();
    let mut out = "\u{1b}[1;38;5;231m".to_string();
    for (i, c) in cells.iter().enumerate() {
        let cell_bg = if i < filled { bg } else { BG_EMPTY };
        out.push_str(&format!("\u{1b}[48;5;{}m{}", cell_bg, c));
    }
    out.push_str("\u{1b}[0m");
    out
}

// spec: delegation-kit/SPEC.md §The statusline arm — the time left on an axis, coarsened to
// two units: a window that has already reset renders nothing rather than a negative span.
fn remaining(resets_at: &str) -> String {
    if resets_at.is_empty() || resets_at == "null" {
        return String::new();
    }
    let Ok(at) = resets_at.parse::<i64>() else {
        return String::new();
    };
    let left = at - kpi::now_epoch();
    if left <= 0 {
        return String::new();
    }
    let (days, hours, minutes) = (left / 86_400, (left % 86_400) / 3_600, (left % 3_600) / 60);
    if days > 0 {
        format!("{}d{}h", days, hours)
    } else {
        format!("{}h{}m", hours, minutes)
    }
}

// spec: delegation-kit/SPEC.md §The statusline arm — the section vocabulary is the counter
// tool's, never this member's: each label is the initial of a name the tool returned, widened to
// two characters for every counter as soon as two returned names share one.
fn counters(tsv: &str) -> String {
    let rows: Vec<(&str, &str)> = tsv
        .lines()
        .filter_map(|l| l.split_once('\t'))
        .filter(|(name, _)| !name.is_empty())
        .collect();
    if rows.is_empty() {
        return String::new();
    }
    let collides = rows.iter().enumerate().any(|(i, (a, _))| {
        rows[i + 1..]
            .iter()
            .any(|(b, _)| a.chars().next() == b.chars().next())
    });
    let width = if collides { 2 } else { 1 };
    rows.iter()
        .map(|(name, n)| format!("{}{}", name.chars().take(width).collect::<String>(), n))
        .collect::<Vec<_>>()
        .join(" ")
}

// spec: delegation-kit/SPEC.md §The statusline arm — the project trio, read at their literal
// tracked paths exactly as the shell member read them: an unresolvable root leaves all three empty
// and the render drops their sections rather than printing a partial parse.
fn project() -> (String, String, String) {
    let empty = (String::new(), String::new(), String::new());
    let Ok(root) = walk::cwd().and_then(|d| walk::toplevel_in(&d)) else {
        return empty;
    };
    let read = |rel: &str| std::fs::read_to_string(format!("{}/{}", root, rel));
    let iteration = read("TASK-QUEUE.md")
        .ok()
        .and_then(|t| {
            t.lines()
                .find(|l| l.starts_with("## Iteration:"))
                .map(str::to_string)
        })
        .map(|l| {
            let name = l.trim_start_matches("## Iteration:").trim_start();
            match name.split_once("[stage:") {
                Some((before, _)) => before.trim_end().to_string(),
                None => name.to_string(),
            }
        })
        .unwrap_or_default();
    let stage = read(".workflow/WORKFLOW-STATE.txt")
        .ok()
        .map(|t| last_stamp_stage(&t))
        .unwrap_or_default();
    // spec: delegation-kit/SPEC.md §The statusline arm — a subprocess and never an in-process
    // call: the counter's own library exits 2 on a malformed queue config, which here would take
    // the whole status bar down for a component worth four characters.
    let counter = format!("{}/queue-kit/bin/queue-counts.sh", root);
    let counts = if proc::is_executable(std::path::Path::new(&counter)) {
        proc::run("bash", &[&counter])
            .ok()
            .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
            .unwrap_or_default()
    } else {
        String::new()
    };
    (iteration, stage, counts)
}

// spec: lifecycle-kit/SPEC.md §The stamp protocol — the cursor is the LAST stamp's stage, read as
// field 2 of the last non-empty line after the `---` separator; a file with no data line leaves it
// empty and the render drops the `@stage` suffix.
fn last_stamp_stage(text: &str) -> String {
    let mut past_separator = false;
    let mut stage = String::new();
    for line in text.lines() {
        if line.trim() == "---" {
            past_separator = true;
            continue;
        }
        if !past_separator || line.split_whitespace().next().is_none() {
            continue;
        }
        if let Some(field) = line.split_whitespace().nth(1) {
            stage = field.to_string();
        }
    }
    stage
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: delegation-kit/SPEC.md §The statusline arm — the gauge is ten cells wide whatever
    // the reading, clamps out of range, and carries its own escapes with no asset outside the
    // binary
    #[test]
    fn the_gauge_is_ten_cells_whatever_the_reading() {
        for reading in ["0", "49.9", "50", "79", "80", "100", "9999", "", "null"] {
            let g = gauge(reading);
            assert_eq!(
                g.matches("\u{1b}[48;5;").count(),
                GAUGE_WIDTH,
                "reading {} did not render ten cells",
                reading
            );
            assert!(g.ends_with("\u{1b}[0m"));
        }
        assert!(gauge("91").contains(&format!("\u{1b}[48;5;{}m", BG_HOT)));
        assert!(gauge("60").contains(&format!("\u{1b}[48;5;{}m", BG_WARM)));
        assert!(gauge("10").contains(&format!("\u{1b}[48;5;{}m", BG_COOL)));
        assert!(gauge("0").contains(&format!("\u{1b}[48;5;{}m", BG_EMPTY)));
    }

    // spec: delegation-kit/SPEC.md §The statusline arm — the label widens to two characters
    // only when two returned names share an initial, so a colliding pair stays readable
    #[test]
    fn a_colliding_initial_widens_every_label() {
        assert_eq!(counters("New Features\t1\nIcebox\t55"), "N1 I55");
        assert_eq!(
            counters("New Features\t1\nTechnical Debt\t0\nDeferred\t281\nIcebox\t55"),
            "N1 T0 D281 I55"
        );
        assert_eq!(counters("Deferred\t2\nDone\t3"), "De2 Do3");
        assert_eq!(counters(""), "");
        assert_eq!(counters("no tab here\n"), "");
    }

    // spec: lifecycle-kit/SPEC.md §The stamp protocol — the cursor is the last stamp's stage, and a
    // file with no data line after the separator leaves it empty rather than half-parsed
    #[test]
    fn the_stage_is_the_last_stamps_second_field() {
        let text = "header\n---\n\niter scope aaa 2026-01-01 h1\niter build bbb 2026-01-02 h2\n";
        assert_eq!(last_stamp_stage(text), "build");
        assert_eq!(last_stamp_stage("header only\n"), "");
        assert_eq!(last_stamp_stage("header\n---\n"), "");
    }

    // spec: delegation-kit/SPEC.md §The statusline arm — the model name becomes one narrow
    // token: its first word, lowercased, with every non-alphanumeric character dropped
    #[test]
    fn the_model_name_narrows_to_one_token() {
        assert_eq!(slug("Opus 5 (1M context)"), "opus");
        assert_eq!(slug("claude-3.5"), "claude35");
        assert_eq!(slug(""), "");
    }
}
