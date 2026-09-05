// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-overhead: governance + gate-output share over the overhead meter's log
use super::{date_epoch, na, now_epoch, read, Ctx};

const LABEL_GOV: &str = "overhead (gov share)";
// spec: drift-kit/SPEC.md §Bundled KPIs — sessions averaged: recent enough to track the trend,
// wide enough to damp per-session noise.
const WINDOW: usize = 10;

pub struct Window {
    pub sessions: usize,
    pub avg_pct: i64,
    pub gate_pct: i64,
    pub last_date: String,
}

// spec: drift-kit/SPEC.md §Bundled KPIs — awk's field loop assigns on every match, so the
// rightmost spelling on a line is the one that survives; the value is read to the first `=` the
// same way `split($i, a, "=")` takes a[2].
fn tagged(line: &str, tag: &str) -> Option<f64> {
    line.split_whitespace()
        .filter_map(|t| t.strip_prefix(tag))
        .next_back()
        .and_then(|v| v.split('=').next().unwrap_or(v).parse::<f64>().ok())
}

// spec: drift-kit/SPEC.md §Bundled KPIs — awk's `int(x + 0.5)` over the trailing window: the
// governance share is the per-session mean, the gate share a ratio of the summed volumes, and an
// absent field on a line reads as zero exactly as awk's uninitialised variable did.
pub fn summarize(text: &str) -> Window {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(WINDOW);
    let (mut sp, mut st, mut sg) = (0.0f64, 0.0f64, 0.0f64);
    let mut n = 0usize;
    let mut last = String::new();
    for l in &lines[start..] {
        sp += tagged(l, "pct=").unwrap_or(0.0);
        st += tagged(l, "total=").unwrap_or(0.0);
        sg += tagged(l, "gate=").unwrap_or(0.0);
        n += 1;
        last = l.split_whitespace().next().unwrap_or("").to_string();
    }
    Window {
        sessions: n,
        avg_pct: if n > 0 { (sp / n as f64 + 0.5) as i64 } else { 0 },
        gate_pct: if st != 0.0 {
            (sg * 100.0 / st + 0.5) as i64
        } else {
            0
        },
        last_date: last,
    }
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.overhead_log) {
        Some(t) if !t.is_empty() => t,
        _ => {
            return na(
                "lead",
                LABEL_GOV,
                "no measurement yet — run --emit overhead-meter",
                trend,
            )
        }
    };
    let w = summarize(&text);
    if trend {
        return Some(format!("ovh {}%\n", w.avg_pct));
    }

    let now = now_epoch();
    let last = date_epoch(&w.last_date).unwrap_or(now);
    let days = ((now - last) / 86400).max(0);
    Some(format!(
        "lead\t{}\t{}% over {} session(s), as of {} ({}d; byte-proxy)\nlead\toverhead (gate share)\t{}% of volume is gate output (byte-proxy)\n",
        LABEL_GOV, w.avg_pct, w.sessions, w.last_date, days, w.gate_pct
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the window is the trailing ten, so an eleventh
    // older session does not weigh on the mean
    #[test]
    fn only_the_trailing_window_is_averaged() {
        let mut t = String::new();
        t.push_str("2026-01-01 pct=100 total=100 gate=100\n");
        for _ in 0..WINDOW {
            t.push_str("2026-08-01 pct=20 total=100 gate=10\n");
        }
        let w = summarize(&t);
        assert_eq!(w.sessions, WINDOW);
        assert_eq!(w.avg_pct, 20);
        assert_eq!(w.gate_pct, 10);
        assert_eq!(w.last_date, "2026-08-01");
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — the gate share divides summed volumes rather than
    // averaging per-session ratios, so a large session weighs what it actually contributed
    #[test]
    fn the_gate_share_is_a_ratio_of_sums_and_a_zero_volume_is_not_a_division() {
        let w = summarize("d pct=0 total=900 gate=300\nd2 pct=0 total=100 gate=0\n");
        assert_eq!(w.gate_pct, 30);
        assert_eq!(summarize("d pct=5\n").gate_pct, 0);
        assert_eq!(summarize("d pct=5\n").avg_pct, 5);
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — awk's int(x + 0.5) is round-half-up, not truncation
    #[test]
    fn the_rounding_is_half_up_rather_than_truncating() {
        assert_eq!(summarize("d pct=20.5 total=1 gate=0\n").avg_pct, 21);
        assert_eq!(summarize("d pct=20.4 total=1 gate=0\n").avg_pct, 20);
    }
}
