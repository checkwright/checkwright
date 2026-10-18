// spec: drift-kit/SPEC.md §The statistics view — the per-iteration figures the other arms report as
// tables, charted into one self-contained HTML document under the metric dir. Advisory: exit is
// 0 bar a usage error or a failed write, and a missing input degrades its panel to its reason.
use super::queue_flow::{self, Window};
use super::stage_economics::{self, LogRow};
use super::trajectory::{self, Closed};
use std::fmt::Write as _;

pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_STATISTICS_FILE",
    "DRIFT_KIT_METRIC_DIR",
    "DRIFT_KIT_STAGE_ECONOMICS_LOG",
    "DRIFT_KIT_SUPERVISION_LABEL",
    "DRIFT_KIT_FANOUT_SUFFIX",
    "DRIFT_KIT_TRAJECTORY_SURFACES",
    "DRIFT_KIT_GATES_FILE",
    "DRIFT_KIT_STAGES",
    "GATE_SDK_WORKFLOW_DIR",
    "DRIFT_KIT_STATE_FILE",
    "DRIFT_KIT_QUEUE_FILE",
    "DRIFT_KIT_DEFERRED_SECTION",
    "DRIFT_KIT_ICEBOX_SECTION",
];

pub const USAGE: &str = "usage: --emit statistics [<n>]   (<n> a positive integer, the trailing \
                         closed iterations charted; default every closed iteration)";

fn count_arg(args: &[String]) -> Result<Option<usize>, String> {
    match args {
        [] => Ok(None),
        [n] => match n.parse::<usize>() {
            Ok(v) if v > 0 && n.bytes().all(|b| b.is_ascii_digit()) => Ok(Some(v)),
            _ => Err(USAGE.to_string()),
        },
        _ => Err(USAGE.to_string()),
    }
}

// spec: drift-kit/SPEC.md §The statistics view — everything the document is a function of, so a
// render reads no clock and no environment
pub struct Inputs {
    pub closed: Vec<Closed>,
    pub closed_names: Vec<String>,
    pub no_closed: Option<String>,
    pub log: Option<Vec<LogRow>>,
    pub windows: Result<Vec<Window>, String>,
    pub supervision: String,
    pub fanout_suffix: String,
}

fn trailing<T>(mut v: Vec<T>, n: Option<usize>) -> Vec<T> {
    if let Some(n) = n {
        if v.len() > n {
            v.drain(..v.len() - n);
        }
    }
    v
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct Spend {
    bands: [f64; 3],
    cache_read: u64,
    unpriced: usize,
    rows: usize,
}

// spec: drift-kit/SPEC.md §The statistics view — the band a row's stage-or-role value falls in:
// a fan-out row by its suffix first, so `supervision+fanout` is fan-out spend
fn band(stage: &str, supervision: &str, suffix: &str) -> usize {
    if !suffix.is_empty() && stage.ends_with(suffix) {
        2
    } else if stage == supervision {
        1
    } else {
        0
    }
}

fn spend_of(rows: &[LogRow], iteration: &str, supervision: &str, suffix: &str) -> Spend {
    let mut s = Spend::default();
    for r in rows.iter().filter(|r| r.iteration == iteration) {
        s.rows += 1;
        s.cache_read += r.tokens.cache_read;
        match r.cost.parse::<f64>() {
            Ok(c) => s.bands[band(&r.stage, supervision, suffix)] += c,
            Err(_) => s.unpriced += 1,
        }
    }
    s
}

const FIGURES: [&str; 7] = [
    "spend",
    "cache-read",
    "commit shape",
    "amendments",
    "validate",
    "gate roster",
    "queue flow",
];

pub struct Rendered {
    pub document: String,
    pub report: String,
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let n = count_arg(args)?;
    let knob = crate::walk::knob_scalar;
    let file = knob("DRIFT_KIT_STATISTICS_FILE")?;
    let log_path = knob("DRIFT_KIT_STAGE_ECONOMICS_LOG")?;
    let h = trajectory::harvest()?;
    let closed_names: Vec<String> = h.closed.iter().map(|c| c.name.clone()).collect();
    let no_closed = if !h.state_present {
        Some(format!("no committed state file at {}", h.state_file))
    } else if h.closed.is_empty() {
        Some("no closed iteration".to_string())
    } else {
        None
    };
    let log = std::fs::read(crate::walk::capture_path(&log_path)).ok().map(|b| {
        String::from_utf8_lossy(&b)
            .lines()
            .filter_map(stage_economics::parse_row)
            .collect()
    });
    let inputs = Inputs {
        closed: trailing(h.closed, n),
        closed_names,
        no_closed,
        log,
        windows: queue_flow::windows(n)?,
        supervision: knob("DRIFT_KIT_SUPERVISION_LABEL")?,
        fanout_suffix: knob("DRIFT_KIT_FANOUT_SUFFIX")?,
    };
    let r = render(&inputs);
    let at = crate::walk::capture_path(&file);
    if let Some(dir) = std::path::Path::new(&at).parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    std::fs::write(&at, &r.document).map_err(|e| format!("cannot write {}: {}", file, e))?;
    Ok(format!(
        "statistics: wrote {} — {} closed iteration(s)\n{}",
        file,
        inputs.closed.len(),
        r.report
    ))
}

// spec: drift-kit/SPEC.md §The statistics view — the document and the per-figure report from one
// pass, so the stdout line for a panel cannot disagree with what the panel shows
pub fn render(inp: &Inputs) -> Rendered {
    let names: Vec<&str> = inp.closed.iter().map(|c| c.name.as_str()).collect();
    let spend: Vec<Spend> = match &inp.log {
        Some(rows) => names
            .iter()
            .map(|n| spend_of(rows, n, &inp.supervision, &inp.fanout_suffix))
            .collect(),
        None => Vec::new(),
    };
    let flow: Vec<Option<&Window>> = match &inp.windows {
        Ok(ws) => names
            .iter()
            .map(|n| ws.iter().find(|w| w.label == *n))
            .collect(),
        Err(_) => Vec::new(),
    };

    let tracked = |extra: Option<String>| -> Result<(), String> {
        match (&inp.no_closed, extra) {
            (Some(r), _) => Err(r.clone()),
            (None, Some(r)) => Err(r),
            (None, None) => Ok(()),
        }
    };
    let logged = match &inp.log {
        None => Err("no stage-economics log".to_string()),
        Some(_) => tracked(
            (!spend.iter().any(|s| s.rows > 0))
                .then(|| "no trend-log row names a charted iteration".to_string()),
        ),
    };
    let flowed = match &inp.windows {
        Err(r) => tracked(None).and(Err(r.clone())),
        Ok(_) => tracked(
            (!flow.iter().any(Option::is_some))
                .then(|| "no queue-flow window names a charted iteration".to_string()),
        ),
    };
    let status: [Result<(), String>; 7] = [
        logged.clone(),
        logged,
        tracked(None),
        tracked(None),
        tracked(None),
        tracked(None),
        flowed,
    ];

    let mut report = String::new();
    for (name, st) in FIGURES.iter().zip(status.iter()) {
        match st {
            Ok(()) => {
                let _ = writeln!(report, "  {}: charted", name);
            }
            Err(r) => {
                let _ = writeln!(report, "  {}: n/a ({})", name, r);
            }
        }
    }
    let closed = |n: &str| inp.closed_names.iter().any(|c| c == n);
    let mut stray: Vec<&str> = Vec::new();
    for r in inp.log.iter().flatten() {
        if !closed(&r.iteration) && !stray.contains(&r.iteration.as_str()) {
            stray.push(&r.iteration);
        }
    }
    let unpriced = spend.iter().filter(|s| s.unpriced > 0).count();
    let unjoined = match &inp.windows {
        Ok(ws) => ws.iter().filter(|w| !closed(&w.label)).count(),
        Err(_) => 0,
    };
    for (k, what) in [
        (stray.len(), "trend-log iteration(s) name no closed iteration"),
        (unpriced, "iteration(s) with incomplete pricing"),
        (unjoined, "queue-flow window(s) name no closed iteration"),
    ] {
        if k > 0 {
            let _ = writeln!(report, "  {} {}", k, what);
        }
    }

    let fr = Frame::new(names.len());
    let mut doc = String::new();
    doc.push_str(HEAD);
    for para in trajectory::LIMITS.split("\n\n") {
        let para = para.trim();
        if !para.is_empty() {
            let _ = writeln!(doc, "<p class=\"limit\">{}</p>", esc(para));
        }
    }
    let span = match (names.first(), names.last()) {
        (Some(a), Some(b)) => format!(
            "{} closed iteration(s), oldest left: {} to {}. Hover a mark for its iteration and value.",
            names.len(),
            esc(a),
            esc(b)
        ),
        _ => "No closed iteration is charted.".to_string(),
    };
    let _ = writeln!(doc, "<p class=\"meta\">{}</p>", span);

    let panels: [(&str, &str, &str); 7] = [
        (
            "Spend",
            "USD per iteration, the <code>cost</code> field of the stage-economics trend log \
             (<code>DRIFT_KIT_STAGE_ECONOMICS_LOG</code>), stage rows, the supervision row and \
             fan-out rows stacked. A caret marks a bar holding an <code>n/a</code> cost row; a \
             baseline tick marks an iteration with no trend-log row.",
            "spend",
        ),
        (
            "Cache-read",
            "Cache-read tokens per iteration, the <code>cr</code> field of the same trend log, \
             every row summed.",
            "cache-read",
        ),
        (
            "Commit shape",
            "Feature and debt commits in each iteration's close range, from committed history \
             (the trajectory extractor's commit-shape harvest).",
            "commit-shape",
        ),
        (
            "Amendments",
            "Amendments merged (upper strip) and the longest add-to-merge lag in days (lower \
             strip, drawn where one merged), from committed history (the trajectory extractor's \
             amendment-latency harvest).",
            "amendments",
        ),
        (
            "Validate",
            "The iteration's validate attestations, from the committed evidence manifest's \
             history (the trajectory extractor's validate harvest).",
            "validate",
        ),
        (
            "Gate roster",
            "The <code>gates.list</code> member count at each iteration's close commit, from \
             committed history.",
            "gate-roster",
        ),
        (
            "Queue flow",
            "Design-pending entries filed (above the axis) and drained (below) per iteration \
             window, from the queue file at consecutive iteration-start commits (the queue-flow \
             arm), joined on each window's iteration label.",
            "queue-flow",
        ),
    ];
    for (i, (title, caption, id)) in panels.iter().enumerate() {
        let _ = writeln!(doc, "<section class=\"panel\" id=\"{}\">", id);
        let _ = writeln!(doc, "<h2>{}</h2>", title);
        let _ = writeln!(doc, "<p class=\"caption\">{}</p>", caption);
        match &status[i] {
            Err(r) => {
                let _ = writeln!(doc, "<p class=\"na\">n/a ({})</p>", esc(r));
            }
            Ok(()) => {
                let body = match i {
                    0 => spend_panel(&fr, &names, &spend),
                    1 => cache_panel(&fr, &names, &spend),
                    2 => commit_panel(&fr, &inp.closed),
                    3 => amendment_panel(&fr, &inp.closed),
                    4 => validate_panel(&fr, &inp.closed),
                    5 => gate_panel(&fr, &inp.closed),
                    _ => flow_panel(&fr, &names, &flow),
                };
                doc.push_str(&body);
            }
        }
        doc.push_str("</section>\n");
    }
    doc.push_str("</main>\n</body>\n</html>\n");
    Rendered {
        document: doc,
        report,
    }
}

const HEAD: &str = "<!DOCTYPE html>
<html lang=\"en\">
<head>
<meta charset=\"utf-8\">
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
<title>Iteration statistics</title>
<style>
.viz-root {
  color-scheme: light;
  --page: #f9f9f7; --surface: #fcfcfb; --ink: #0b0b0b; --ink-2: #52514e; --muted: #898781;
  --grid: #e1e0d9; --axis: #c3c2b7; --ring: rgba(11,11,11,0.10);
  --s1: #2a78d6; --s2: #eb6834; --s3: #1baf7a; --good: #0ca30c; --critical: #d03b3b;
}
@media (prefers-color-scheme: dark) {
  .viz-root {
    color-scheme: dark;
    --page: #0d0d0d; --surface: #1a1a19; --ink: #f0efec; --ink-2: #c3c2b7; --muted: #898781;
    --grid: #2c2c2a; --axis: #383835; --ring: rgba(255,255,255,0.10);
    --s1: #3987e5; --s2: #d95926; --s3: #199e70;
  }
}
html, body { margin: 0; }
.viz-root { background: var(--page); color: var(--ink); min-height: 100vh; padding: 24px;
  box-sizing: border-box; font: 14px/1.45 system-ui, -apple-system, \"Segoe UI\", sans-serif; }
.viz-root h1 { font-size: 20px; margin: 0 0 8px; }
.viz-root h2 { font-size: 15px; margin: 0 0 2px; }
.limit, .meta, .caption { color: var(--ink-2); margin: 0 0 6px; max-width: 72em; }
.caption { font-size: 12.5px; }
.na { color: var(--ink-2); font-style: italic; margin: 12px 0; }
.panel { background: var(--surface); border: 1px solid var(--ring); border-radius: 8px;
  padding: 14px 16px 10px; margin: 14px 0; max-width: 1040px; }
.legend { display: flex; flex-wrap: wrap; gap: 4px 16px; font-size: 12.5px; color: var(--ink-2);
  margin: 4px 0; }
.key { display: inline-block; width: 10px; height: 10px; border-radius: 2px;
  margin-right: 6px; vertical-align: -1px; }
svg { display: block; width: 100%; height: auto; }
svg text { font-size: 11px; fill: var(--muted); font-variant-numeric: tabular-nums; }
.grid { stroke: var(--grid); stroke-width: 1; }
.base { stroke: var(--axis); stroke-width: 1; }
.hit { fill: transparent; }
.hit:hover { fill: var(--grid); fill-opacity: 0.5; }
.s1 { fill: var(--s1); } .s2 { fill: var(--s2); } .s3 { fill: var(--s3); }
.good { fill: var(--good); } .critical { fill: var(--critical); } .none { fill: var(--axis); }
.tick { fill: var(--axis); }
.caret { fill: var(--ink-2); }
.line { fill: none; stroke: var(--s2); stroke-width: 2; stroke-linejoin: round; }
.line1 { fill: none; stroke: var(--s1); stroke-width: 2; stroke-linejoin: round; }
.dot { fill: var(--s2); stroke: var(--surface); stroke-width: 1; }
.dot1 { fill: var(--s1); stroke: var(--surface); stroke-width: 1; }
</style>
</head>
<body>
<main class=\"viz-root\">
<h1>Iteration statistics</h1>
";

const W: f64 = 960.0;
const LEFT: f64 = 56.0;
const RIGHT: f64 = 12.0;
const LABELS: f64 = 22.0;

struct Frame {
    n: usize,
    step: f64,
    bar: f64,
}

impl Frame {
    fn new(n: usize) -> Frame {
        let step = (W - LEFT - RIGHT) / n.max(1) as f64;
        let bar = if step >= 4.0 {
            (step - 2.0).min(28.0)
        } else {
            step
        };
        Frame { n, step, bar }
    }
    fn center(&self, i: usize) -> f64 {
        LEFT + self.step * (i as f64 + 0.5)
    }
    fn left(&self, i: usize) -> f64 {
        self.center(i) - self.bar / 2.0
    }
}

// spec: drift-kit/SPEC.md §The statistics view — Determinism: every coordinate prints at one
// fixed precision, so two renders over the same inputs are byte-identical
fn c(v: f64) -> String {
    let s = format!("{:.1}", v);
    match s.strip_suffix(".0") {
        Some(t) if t != "-0" => t.to_string(),
        Some(_) => "0".to_string(),
        None => s,
    }
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

fn usd(v: f64) -> String {
    format!("${:.2}", v)
}

fn usd_axis(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("${}", v as i64)
    } else {
        usd(v)
    }
}

fn tokens(v: f64) -> String {
    let a = v.abs();
    let (div, unit) = if a >= 1e9 {
        (1e9, "G")
    } else if a >= 1e6 {
        (1e6, "M")
    } else if a >= 1e3 {
        (1e3, "k")
    } else {
        return format!("{}", v as i64);
    };
    format!("{}{}", c(v / div), unit)
}

fn int(v: f64) -> String {
    format!("{}", v.round() as i64)
}

// spec: drift-kit/SPEC.md §The statistics view — a y-axis of about four steps, each one, two,
// two-and-a-half or five times a power of ten; a count axis steps by whole numbers
fn scale(max: f64, whole: bool) -> (f64, f64) {
    let raw = if max > 0.0 { max / 4.0 } else { 1.0 };
    let pow = 10f64.powf(raw.log10().floor());
    let mults: &[f64] = if whole {
        &[1.0, 2.0, 5.0, 10.0]
    } else {
        &[1.0, 2.0, 2.5, 5.0, 10.0]
    };
    let mut step = mults
        .iter()
        .map(|m| m * pow)
        .find(|s| *s >= raw * (1.0 - 1e-9))
        .unwrap_or(10.0 * pow);
    if whole {
        step = step.ceil().max(1.0);
    }
    let top = ((max / step).ceil() * step).max(step);
    (top, step)
}

fn svg_open(out: &mut String, h: f64) {
    let _ = writeln!(
        out,
        "<svg viewBox=\"0 0 {} {}\" role=\"img\">",
        c(W),
        c(h)
    );
}

fn y_axis(out: &mut String, y0: f64, h: f64, top: f64, step: f64, fmt: fn(f64) -> String) {
    let mut v = 0.0;
    while v <= top * (1.0 + 1e-9) {
        let y = y0 + h - v / top * h;
        let _ = writeln!(
            out,
            "<line class=\"{}\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/><text x=\"{}\" y=\"{}\" \
             text-anchor=\"end\">{}</text>",
            if v == 0.0 { "base" } else { "grid" },
            c(LEFT),
            c(W - RIGHT),
            c(y),
            c(y),
            c(LEFT - 6.0),
            c(y + 4.0),
            esc(&fmt(v))
        );
        v += step;
    }
}

fn x_labels(out: &mut String, fr: &Frame, names: &[&str], y: f64) {
    if let Some(first) = names.first() {
        let _ = writeln!(
            out,
            "<text x=\"{}\" y=\"{}\">{}</text>",
            c(LEFT),
            c(y),
            esc(first)
        );
    }
    if fr.n > 1 {
        let _ = writeln!(
            out,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{}</text>",
            c(W - RIGHT),
            c(y),
            esc(names[fr.n - 1])
        );
    }
}

fn hit(out: &mut String, fr: &Frame, i: usize, y: f64, h: f64, title: &str) {
    let _ = writeln!(
        out,
        "<rect class=\"hit\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"><title>{}</title></rect>",
        c(LEFT + fr.step * i as f64),
        c(y),
        c(fr.step),
        c(h),
        esc(title)
    );
}

fn rect(out: &mut String, class: &str, x: f64, y: f64, w: f64, h: f64, title: &str) {
    let _ = writeln!(
        out,
        "<rect class=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\"><title>{}</title></rect>",
        class,
        c(x),
        c(y),
        c(w),
        c(h.max(0.0)),
        c((w / 4.0).min(2.0)),
        esc(title)
    );
}

fn legend(out: &mut String, keys: &[(&str, &str)]) {
    out.push_str("<div class=\"legend\">");
    for (class, label) in keys {
        let _ = write!(
            out,
            "<span><span class=\"key\" style=\"background: var(--{})\"></span>{}</span>",
            class, label
        );
    }
    out.push_str("</div>\n");
}

const PLOT_H: f64 = 140.0;
const TOP: f64 = 8.0;
const GAP: f64 = 1.5;

// spec: drift-kit/SPEC.md §The statistics view — a stacked bar, bottom band first, a surface gap
// between bands; the caller's titles name each band's iteration and value
fn stack(out: &mut String, fr: &Frame, i: usize, base: f64, top: f64, parts: &[(&str, f64, String)]) {
    let total: f64 = parts.iter().map(|p| p.1).sum();
    let mut y = base;
    let mut drawn = 0.0;
    for (k, (class, v, title)) in parts.iter().enumerate() {
        if *v <= 0.0 {
            continue;
        }
        let h = v / top * PLOT_H;
        drawn += v;
        let more = parts[k + 1..].iter().any(|p| p.1 > 0.0) && drawn < total;
        let g = if more && h > 2.0 * GAP { GAP } else { 0.0 };
        rect(out, class, fr.left(i), y - h + g, fr.bar, h - g, title);
        y -= h;
    }
}

fn spend_panel(fr: &Frame, names: &[&str], spend: &[Spend]) -> String {
    let mut out = String::new();
    legend(
        &mut out,
        &[("s1", "stage rows"), ("s2", "supervision"), ("s3", "fan-out")],
    );
    let max = spend.iter().map(|s| s.bands.iter().sum::<f64>()).fold(0.0, f64::max);
    let (top, step) = scale(max, false);
    let h = TOP + PLOT_H + LABELS;
    svg_open(&mut out, h);
    y_axis(&mut out, TOP, PLOT_H, top, step, usd_axis);
    let base = TOP + PLOT_H;
    for (i, s) in spend.iter().enumerate() {
        let total: f64 = s.bands.iter().sum();
        let name = names[i];
        let summary = if s.rows == 0 {
            format!("{}: no trend-log row", name)
        } else {
            format!(
                "{}: {} total (stage {}, supervision {}, fan-out {}){}",
                name,
                usd(total),
                usd(s.bands[0]),
                usd(s.bands[1]),
                usd(s.bands[2]),
                if s.unpriced > 0 {
                    format!("; incomplete pricing, {} row(s) cost n/a", s.unpriced)
                } else {
                    String::new()
                }
            )
        };
        hit(&mut out, fr, i, TOP, PLOT_H, &summary);
        if s.rows == 0 {
            rect(&mut out, "tick", fr.left(i), base - 2.0, fr.bar, 2.0, &summary);
            continue;
        }
        let parts = [
            ("s1", s.bands[0], format!("{}: stage rows {}", name, usd(s.bands[0]))),
            ("s2", s.bands[1], format!("{}: supervision {}", name, usd(s.bands[1]))),
            ("s3", s.bands[2], format!("{}: fan-out {}", name, usd(s.bands[2]))),
        ];
        stack(&mut out, fr, i, base, top, &parts);
        if s.unpriced > 0 {
            let x = fr.center(i);
            let y = base - total / top * PLOT_H - 3.0;
            let r = (fr.bar / 2.0).clamp(2.0, 5.0);
            let _ = writeln!(
                out,
                "<path class=\"caret\" d=\"M{} {}L{} {}L{} {}Z\"><title>{}</title></path>",
                c(x - r),
                c(y - r),
                c(x + r),
                c(y - r),
                c(x),
                c(y),
                esc(&summary)
            );
        }
    }
    x_labels(&mut out, fr, names, h - 6.0);
    out.push_str("</svg>\n");
    out
}

fn cache_panel(fr: &Frame, names: &[&str], spend: &[Spend]) -> String {
    let mut out = String::new();
    let max = spend.iter().map(|s| s.cache_read as f64).fold(0.0, f64::max);
    let (top, step) = scale(max, max < 1e3);
    let h = TOP + PLOT_H + LABELS;
    svg_open(&mut out, h);
    y_axis(&mut out, TOP, PLOT_H, top, step, tokens);
    let base = TOP + PLOT_H;
    for (i, s) in spend.iter().enumerate() {
        let title = if s.rows == 0 {
            format!("{}: no trend-log row", names[i])
        } else {
            format!("{}: {} cache-read tokens", names[i], s.cache_read)
        };
        hit(&mut out, fr, i, TOP, PLOT_H, &title);
        if s.rows == 0 {
            rect(&mut out, "tick", fr.left(i), base - 2.0, fr.bar, 2.0, &title);
            continue;
        }
        let v = s.cache_read as f64 / top * PLOT_H;
        rect(&mut out, "s1", fr.left(i), base - v, fr.bar, v, &title);
    }
    x_labels(&mut out, fr, names, h - 6.0);
    out.push_str("</svg>\n");
    out
}

fn names_of(closed: &[Closed]) -> Vec<&str> {
    closed.iter().map(|c| c.name.as_str()).collect()
}

fn commit_panel(fr: &Frame, closed: &[Closed]) -> String {
    let mut out = String::new();
    legend(&mut out, &[("s1", "feature commits"), ("s2", "debt commits")]);
    let max = closed.iter().map(|e| (e.feat + e.debt) as f64).fold(0.0, f64::max);
    let (top, step) = scale(max, true);
    let h = TOP + PLOT_H + LABELS;
    svg_open(&mut out, h);
    y_axis(&mut out, TOP, PLOT_H, top, step, int);
    for (i, e) in closed.iter().enumerate() {
        hit(
            &mut out,
            fr,
            i,
            TOP,
            PLOT_H,
            &format!("{}: {} feature, {} debt", e.name, e.feat, e.debt),
        );
        let parts = [
            ("s1", e.feat as f64, format!("{}: {} feature commit(s)", e.name, e.feat)),
            ("s2", e.debt as f64, format!("{}: {} debt commit(s)", e.name, e.debt)),
        ];
        stack(&mut out, fr, i, TOP + PLOT_H, top, &parts);
    }
    x_labels(&mut out, fr, &names_of(closed), h - 6.0);
    out.push_str("</svg>\n");
    out
}

// spec: drift-kit/SPEC.md §The statistics view — the lag line's own axis is its own strip under
// the count bars, sharing the x-axis, so no plot carries two y-scales
fn amendment_panel(fr: &Frame, closed: &[Closed]) -> String {
    const LAG_TOP: f64 = TOP + PLOT_H + 16.0;
    const LAG_H: f64 = 70.0;
    let mut out = String::new();
    legend(
        &mut out,
        &[("s1", "amendments merged (upper)"), ("s2", "longest lag, days (lower)")],
    );
    let max = closed.iter().map(|e| e.amendments as f64).fold(0.0, f64::max);
    let (top, step) = scale(max, true);
    let lag_max = closed
        .iter()
        .filter(|e| e.amendments > 0)
        .map(|e| e.max_lag as f64)
        .fold(0.0, f64::max);
    let (lag_top, lag_step) = scale(lag_max, true);
    let (lag_step, h) = (lag_step.max(lag_top / 2.0), LAG_TOP + LAG_H + LABELS);
    svg_open(&mut out, h);
    y_axis(&mut out, TOP, PLOT_H, top, step, int);
    y_axis(&mut out, LAG_TOP, LAG_H, lag_top, lag_step, int);
    let lag_y = |e: &Closed| LAG_TOP + LAG_H - e.max_lag as f64 / lag_top * LAG_H;
    for (i, e) in closed.iter().enumerate() {
        let summary = if e.amendments == 0 {
            format!("{}: no amendment merged", e.name)
        } else {
            format!(
                "{}: {} amendment(s) merged, longest lag {} day(s)",
                e.name, e.amendments, e.max_lag
            )
        };
        hit(&mut out, fr, i, TOP, LAG_TOP + LAG_H - TOP, &summary);
        let v = e.amendments as f64 / top * PLOT_H;
        if e.amendments > 0 {
            rect(&mut out, "s1", fr.left(i), TOP + PLOT_H - v, fr.bar, v, &summary);
        }
    }
    let mut d = String::new();
    for (i, w) in closed.windows(2).enumerate() {
        if w[0].amendments > 0 && w[1].amendments > 0 {
            let _ = write!(
                d,
                "M{} {}L{} {}",
                c(fr.center(i)),
                c(lag_y(&w[0])),
                c(fr.center(i + 1)),
                c(lag_y(&w[1]))
            );
        }
    }
    if !d.is_empty() {
        let _ = writeln!(out, "<path class=\"line\" d=\"{}\"/>", d);
    }
    for (i, e) in closed.iter().enumerate().filter(|(_, e)| e.amendments > 0) {
        let _ = writeln!(
            out,
            "<circle class=\"dot\" cx=\"{}\" cy=\"{}\" r=\"{}\"><title>{}: longest lag {} day(s)</title></circle>",
            c(fr.center(i)),
            c(lag_y(e)),
            c((fr.bar / 2.0).clamp(1.5, 4.0)),
            esc(&e.name),
            e.max_lag
        );
    }
    x_labels(&mut out, fr, &names_of(closed), h - 6.0);
    out.push_str("</svg>\n");
    out
}

fn validate_panel(fr: &Frame, closed: &[Closed]) -> String {
    const CELL: f64 = 22.0;
    let mut out = String::new();
    legend(
        &mut out,
        &[("good", "clean"), ("critical", "non-clean"), ("axis", "no attestation")],
    );
    let h = TOP + CELL + LABELS;
    svg_open(&mut out, h);
    for (i, e) in closed.iter().enumerate() {
        let (class, title) = if !e.val_present {
            ("none", format!("{}: no attestation", e.name))
        } else if e.val_fail == 0 {
            ("good", format!("{}: {} suite(s) clean", e.name, e.val_suites))
        } else {
            (
                "critical",
                format!("{}: {} suite(s), {} failure(s)", e.name, e.val_suites, e.val_fail),
            )
        };
        rect(&mut out, class, fr.left(i), TOP, fr.bar, CELL, &title);
    }
    x_labels(&mut out, fr, &names_of(closed), h - 6.0);
    out.push_str("</svg>\n");
    out
}

fn gate_panel(fr: &Frame, closed: &[Closed]) -> String {
    let mut out = String::new();
    let max = closed.iter().filter_map(|e| e.gates).max().unwrap_or(0) as f64;
    let (top, step) = scale(max, true);
    let h = TOP + PLOT_H + LABELS;
    svg_open(&mut out, h);
    y_axis(&mut out, TOP, PLOT_H, top, step, int);
    let y = |g: usize| TOP + PLOT_H - g as f64 / top * PLOT_H;
    for (i, e) in closed.iter().enumerate() {
        let title = match e.gates {
            Some(g) => format!("{}: {} gate(s) at close", e.name, g),
            None => format!("{}: no gates.list at close", e.name),
        };
        hit(&mut out, fr, i, TOP, PLOT_H, &title);
    }
    let mut d = String::new();
    for (i, w) in closed.windows(2).enumerate() {
        if let (Some(a), Some(b)) = (w[0].gates, w[1].gates) {
            let _ = write!(
                d,
                "M{} {}L{} {}",
                c(fr.center(i)),
                c(y(a)),
                c(fr.center(i + 1)),
                c(y(b))
            );
        }
    }
    if !d.is_empty() {
        let _ = writeln!(out, "<path class=\"line1\" d=\"{}\"/>", d);
    }
    for (i, e) in closed.iter().enumerate() {
        if let Some(g) = e.gates {
            let _ = writeln!(
                out,
                "<circle class=\"dot1\" cx=\"{}\" cy=\"{}\" r=\"{}\"><title>{}: {} gate(s) at close</title></circle>",
                c(fr.center(i)),
                c(y(g)),
                c((fr.bar / 2.0).clamp(1.5, 4.0)),
                esc(&e.name),
                g
            );
        }
    }
    x_labels(&mut out, fr, &names_of(closed), h - 6.0);
    out.push_str("</svg>\n");
    out
}

fn flow_panel(fr: &Frame, names: &[&str], flow: &[Option<&Window>]) -> String {
    const HALF: f64 = PLOT_H / 2.0;
    let mut out = String::new();
    legend(&mut out, &[("s1", "filed (up)"), ("s2", "drained (down)")]);
    let max = flow
        .iter()
        .flatten()
        .map(|w| w.filed.max(w.drained) as f64)
        .fold(0.0, f64::max);
    let (top, step) = scale(max, true);
    let h = TOP + PLOT_H + LABELS;
    let zero = TOP + HALF;
    svg_open(&mut out, h);
    let mut v = step;
    while v <= top * (1.0 + 1e-9) {
        for (sign, label) in [(-1.0, int(v)), (1.0, format!("-{}", int(v)))] {
            let y = zero + sign * v / top * HALF;
            let _ = writeln!(
                out,
                "<line class=\"grid\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/><text x=\"{}\" y=\"{}\" \
                 text-anchor=\"end\">{}</text>",
                c(LEFT),
                c(W - RIGHT),
                c(y),
                c(y),
                c(LEFT - 6.0),
                c(y + 4.0),
                label
            );
        }
        v += step;
    }
    let _ = writeln!(
        out,
        "<line class=\"base\" x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\"/><text x=\"{}\" y=\"{}\" \
         text-anchor=\"end\">0</text>",
        c(LEFT),
        c(W - RIGHT),
        c(zero),
        c(zero),
        c(LEFT - 6.0),
        c(zero + 4.0)
    );
    for (i, w) in flow.iter().enumerate() {
        let name = names[i];
        match w {
            None => {
                let title = format!("{}: no queue-flow window", name);
                hit(&mut out, fr, i, TOP, PLOT_H, &title);
                rect(&mut out, "tick", fr.left(i), zero - 1.0, fr.bar, 2.0, &title);
            }
            Some(w) => {
                let title = format!("{}: {} filed, {} drained", name, w.filed, w.drained);
                hit(&mut out, fr, i, TOP, PLOT_H, &title);
                let up = w.filed as f64 / top * HALF;
                let down = w.drained as f64 / top * HALF;
                if w.filed > 0 {
                    rect(
                        &mut out,
                        "s1",
                        fr.left(i),
                        zero - up,
                        fr.bar,
                        up - GAP / 2.0,
                        &format!("{}: {} filed", name, w.filed),
                    );
                }
                if w.drained > 0 {
                    rect(
                        &mut out,
                        "s2",
                        fr.left(i),
                        zero + GAP / 2.0,
                        fr.bar,
                        down - GAP / 2.0,
                        &format!("{}: {} drained", name, w.drained),
                    );
                }
            }
        }
    }
    x_labels(&mut out, fr, names, h - 6.0);
    out.push_str("</svg>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn closed(name: &str, feat: usize, gates: Option<usize>) -> Closed {
        Closed {
            name: name.to_string(),
            stages: Vec::new(),
            feat,
            debt: 1,
            amendments: feat,
            max_lag: 2,
            val_present: feat > 0,
            val_suites: 3,
            val_fail: 0,
            gates,
        }
    }

    fn row(iteration: &str, stage: &str, cost: &str) -> LogRow {
        LogRow {
            date: "2026-01-01".to_string(),
            iteration: iteration.to_string(),
            stage: stage.to_string(),
            model: "m".to_string(),
            tokens: stage_economics::Tokens {
                input: 1,
                output: 1,
                cache_read: 100,
                cache_write: 1,
            },
            cost: cost.to_string(),
        }
    }

    fn inputs() -> Inputs {
        Inputs {
            closed: vec![closed("alpha", 2, Some(3)), closed("beta", 0, None)],
            closed_names: vec!["alpha".to_string(), "beta".to_string()],
            no_closed: None,
            log: Some(vec![
                row("alpha", "build", "1.5"),
                row("alpha", "supervision", "0.25"),
                row("alpha", "build+fanout", "0.5"),
                row("alpha", "supervision+fanout", "0.25"),
                row("beta", "close", "n/a"),
                row("in-flight", "scope", "1.0"),
            ]),
            windows: Ok(vec![
                Window {
                    label: "alpha".to_string(),
                    filed: 2,
                    drained: 1,
                },
                Window {
                    label: "renamed".to_string(),
                    filed: 0,
                    drained: 0,
                },
            ]),
            supervision: "supervision".to_string(),
            fanout_suffix: "+fanout".to_string(),
        }
    }

    // spec: drift-kit/SPEC.md §The statistics view — `<n>` is a positive integer and absent means
    // every closed iteration; anything else is a usage error
    #[test]
    fn the_count_is_a_positive_integer_defaulting_to_every_iteration() {
        assert_eq!(count_arg(&[]), Ok(None));
        assert_eq!(count_arg(&["3".to_string()]), Ok(Some(3)));
        for bad in ["0", "+3", "x", "-1", ""] {
            assert!(count_arg(&[bad.to_string()]).is_err(), "{}", bad);
        }
        assert!(count_arg(&["1".to_string(), "2".to_string()]).is_err());
    }

    // spec: drift-kit/SPEC.md §The statistics view — `<n>` keeps the trailing iterations
    #[test]
    fn the_count_keeps_the_trailing_iterations() {
        assert_eq!(trailing(vec![1, 2, 3], Some(2)), vec![2, 3]);
        assert_eq!(trailing(vec![1, 2, 3], Some(9)), vec![1, 2, 3]);
        assert_eq!(trailing(vec![1, 2, 3], None), vec![1, 2, 3]);
    }

    // spec: drift-kit/SPEC.md §The statistics view — the three bands, told apart by the
    // supervision label and the fan-out suffix, and an `n/a` row adding nothing but its mark
    #[test]
    fn spend_splits_into_three_bands_and_an_unpriced_row_marks_its_iteration() {
        let inp = inputs();
        let rows = inp.log.as_ref().unwrap();
        let a = spend_of(rows, "alpha", "supervision", "+fanout");
        assert_eq!(a.bands, [1.5, 0.25, 0.75]);
        assert_eq!((a.unpriced, a.rows, a.cache_read), (0, 4, 400));
        let b = spend_of(rows, "beta", "supervision", "+fanout");
        assert_eq!(b.bands, [0.0, 0.0, 0.0]);
        assert_eq!((b.unpriced, b.rows), (1, 1));
        let r = render(&inp);
        assert!(r.report.contains("  1 iteration(s) with incomplete pricing\n"), "{}", r.report);
        assert!(r.report.contains("  1 trend-log iteration(s) name no closed iteration\n"));
        assert!(r.report.contains("  1 queue-flow window(s) name no closed iteration\n"));
        assert!(r.document.contains("class=\"caret\""));
        assert!(r.document.contains("beta: no queue-flow window"));
        assert!(!r.document.contains("in-flight"));
    }

    // spec: drift-kit/SPEC.md §The statistics view — every panel present, the document free of
    // script and of any reference outside itself
    #[test]
    fn the_document_holds_seven_panels_and_references_nothing_outside_itself() {
        let r = render(&inputs());
        for f in FIGURES {
            assert!(r.report.contains(&format!("  {}: charted\n", f)), "{}", r.report);
        }
        assert_eq!(r.document.matches("<section class=\"panel\"").count(), 7);
        for banned in ["<script", "src=", "url(", "href", "@import"] {
            assert!(!r.document.contains(banned), "{}", banned);
        }
        assert!(r.document.contains("No controlled ungoverned baseline exists"));
    }

    // spec: drift-kit/SPEC.md §The statistics view — Degrade: the log's absence takes figures 1
    // and 2, an absent state file takes all seven, and a queue-flow reason takes figure 7 alone
    #[test]
    fn a_missing_input_degrades_only_the_panels_reading_it() {
        let mut inp = inputs();
        inp.log = None;
        inp.windows = Err("fewer than two iteration-start commits in reach".to_string());
        let r = render(&inp);
        assert!(r.report.contains("  spend: n/a (no stage-economics log)\n"), "{}", r.report);
        assert!(r.report.contains("  cache-read: n/a (no stage-economics log)\n"));
        assert!(r.report.contains("  commit shape: charted\n"));
        assert!(r.report.contains("  gate roster: charted\n"));
        assert!(r
            .report
            .contains("  queue flow: n/a (fewer than two iteration-start commits in reach)\n"));
        assert_eq!(r.document.matches("<section class=\"panel\"").count(), 7);

        let mut none = inputs();
        none.closed.clear();
        none.closed_names.clear();
        none.no_closed = Some("no committed state file at s".to_string());
        let r = render(&none);
        assert_eq!(r.report.matches("n/a (no committed state file at s)").count(), 7, "{}", r.report);
    }

    // spec: drift-kit/SPEC.md §The statistics view — Determinism: two renders over unchanged
    // inputs are byte-identical
    #[test]
    fn two_renders_over_the_same_inputs_are_byte_identical() {
        assert_eq!(render(&inputs()).document, render(&inputs()).document);
    }

    // spec: drift-kit/SPEC.md §The statistics view — a y-axis steps by one, two, two-and-a-half or
    // five times a power of ten, by whole numbers on a count axis, and tops at or above the max
    #[test]
    fn an_axis_tops_at_or_above_its_maximum_in_a_round_step() {
        assert_eq!(scale(10.0, true), (10.0, 5.0));
        assert_eq!(scale(3.0, true), (3.0, 1.0));
        assert_eq!(scale(0.0, true), (1.0, 1.0));
        assert_eq!(scale(37.0, false), (40.0, 10.0));
        assert_eq!(c(-0.04), "0");
        assert_eq!(c(12.0), "12");
    }
}
