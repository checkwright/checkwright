// spec: drift-kit/SPEC.md §The statistics view — the per-iteration figures the other arms report as
// tables, charted into one self-contained HTML document under the metric dir. Advisory: exit is
// 0 bar a usage error or a failed write, and a missing input degrades its panel to its reason.
use super::always_loaded;
use super::kpi::{self, deferred_age, overhead};
use super::queue_flow::{self, Window};
use super::stage_economics::{self, LogRow};
use super::trajectory::{self, days_from_civil, Closed};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;

pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_STATISTICS_FILE",
    "DRIFT_KIT_METRIC_DIR",
    "DRIFT_KIT_STAGE_ECONOMICS_LOG",
    "DRIFT_KIT_OVERHEAD_LOG",
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
    "CONTEXT_KIT_SURFACES",
    "GATE_SDK_KIT_DIRS",
    "GATE_SDK_GATES_DIR",
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

// spec: drift-kit/SPEC.md §The statistics view — one overhead-log line, the four fields the
// governance share reads
#[derive(Clone, Debug, PartialEq)]
pub struct Ovh {
    pub session8: String,
    pub total: f64,
    pub gov: f64,
    pub gate: f64,
}

// spec: drift-kit/SPEC.md §The statistics view — a line off the meter's grammar parses to nothing
fn ovh_line(line: &str) -> Option<Ovh> {
    let session8 = line.split_whitespace().nth(1)?;
    if session8.contains('=') {
        return None;
    }
    Some(Ovh {
        session8: session8.to_string(),
        total: overhead::tagged(line, "total=")?,
        gov: overhead::tagged(line, "gov=")?,
        gate: overhead::tagged(line, "gate=")?,
    })
}

// spec: drift-kit/SPEC.md §The statistics view — the stamps as `(iteration, session8)` in the
// collector's order, a stamp seen in both arms once
fn stamp_pairs(lines: &[String]) -> Vec<(String, String)> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for line in lines {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 3 || f[0].starts_with('#') || f[0] == "---" {
            continue;
        }
        if seen.insert(format!("{}/{}/{}", f[0], f[1], f[2])) {
            out.push((f[0].to_string(), f[2].to_string()));
        }
    }
    out
}

// spec: drift-kit/SPEC.md §The statistics view — everything the document is a function of, so a
// render reads no clock and no environment. `always` and `deferred` run beside `closed`, one value
// per charted iteration; `closed_names` is every closed iteration, the model ranking's range.
pub struct Inputs {
    pub closed: Vec<Closed>,
    pub closed_names: Vec<String>,
    pub no_closed: Option<String>,
    pub log: Option<Vec<LogRow>>,
    pub overhead: Option<Vec<Ovh>>,
    pub stamps: Vec<(String, String)>,
    pub always: Result<Vec<u64>, String>,
    pub deferred: Vec<Option<i64>>,
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

// spec: drift-kit/SPEC.md §The statistics view — the categorical slots a by-model panel assigns in
// rank order, and the band every later model folds into
const SLOTS: [&str; 7] = ["s1", "s2", "s3", "s4", "s5", "s6", "s7"];
const OTHER: &str = "other";
const OTHER_LABEL: &str = "other models";

#[derive(Clone, Default, Debug, PartialEq)]
struct Spend {
    bands: [f64; 3],
    cr_bands: [u64; 3],
    models: Vec<f64>,
    cr_models: Vec<u64>,
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

// spec: drift-kit/SPEC.md §The statistics view — the model bands: ranked by summed priced cost over
// every closed iteration, ties by summed cache-read, then by id; a model carrying no token draws no
// band. The ranking never sees `<n>`, so a trimmed view repaints nothing.
fn rank_models(rows: &[LogRow], closed: &[String]) -> Vec<String> {
    let mut sums: BTreeMap<&str, (f64, u64, bool)> = BTreeMap::new();
    for r in rows.iter().filter(|r| closed.contains(&r.iteration)) {
        let e = sums.entry(r.model.as_str()).or_default();
        e.0 += r.cost.parse::<f64>().unwrap_or(0.0);
        e.1 += r.tokens.cache_read;
        let t = r.tokens;
        e.2 |= t.input + t.output + t.cache_read + t.cache_write > 0;
    }
    let mut ranked: Vec<(&str, f64, u64)> = sums
        .into_iter()
        .filter(|(_, v)| v.2)
        .map(|(m, v)| (m, v.0, v.1))
        .collect();
    ranked.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(b.2.cmp(&a.2))
            .then(a.0.cmp(b.0))
    });
    ranked.into_iter().map(|(m, _, _)| m.to_string()).collect()
}

// spec: drift-kit/SPEC.md §The statistics view — a ranked model's band index, the eighth and later
// sharing the fold band
fn slot_of(ranked: &[String], model: &str) -> Option<usize> {
    ranked
        .iter()
        .position(|m| m == model)
        .map(|i| i.min(SLOTS.len()))
}

fn band_count(ranked: &[String]) -> usize {
    ranked.len().min(SLOTS.len() + 1)
}

fn spend_of(
    rows: &[LogRow],
    iteration: &str,
    supervision: &str,
    suffix: &str,
    ranked: &[String],
) -> Spend {
    let mut s = Spend {
        models: vec![0.0; band_count(ranked)],
        cr_models: vec![0; band_count(ranked)],
        ..Spend::default()
    };
    for r in rows.iter().filter(|r| r.iteration == iteration) {
        s.rows += 1;
        let b = band(&r.stage, supervision, suffix);
        let m = slot_of(ranked, &r.model);
        s.cr_bands[b] += r.tokens.cache_read;
        if let Some(m) = m {
            s.cr_models[m] += r.tokens.cache_read;
        }
        match r.cost.parse::<f64>() {
            Ok(c) => {
                s.bands[b] += c;
                if let Some(m) = m {
                    s.models[m] += c;
                }
            }
            Err(_) => s.unpriced += 1,
        }
    }
    s
}

// spec: drift-kit/SPEC.md §The statistics view — an iteration's governance share, ratios of its
// lines' sums
#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct Gov {
    total: f64,
    gov: f64,
    gate: f64,
    sessions: usize,
}

impl Gov {
    fn shares(&self) -> Option<(f64, f64)> {
        (self.sessions > 0 && self.total > 0.0).then(|| {
            let gate = self.gate / self.total * 100.0;
            (gate, ((self.gov - self.gate) / self.total * 100.0).max(0.0))
        })
    }
}

// spec: drift-kit/SPEC.md §The statistics view — the governance join: a line belongs to the
// iteration its session's last stamp names. Returns each named iteration's sums and the sessions
// the stamps map to no closed iteration.
fn governance(lines: &[Ovh], stamps: &[(String, String)], closed: &[String]) -> (HashMap<String, Gov>, usize) {
    let mut owner: HashMap<&str, &str> = HashMap::new();
    for (iteration, session) in stamps {
        owner.insert(session, iteration);
    }
    let mut by: HashMap<String, Gov> = HashMap::new();
    let mut stray: HashSet<&str> = HashSet::new();
    for l in lines {
        match owner.get(l.session8.as_str()) {
            Some(it) if closed.iter().any(|c| c == it) => {
                let g = by.entry(it.to_string()).or_default();
                g.total += l.total;
                g.gov += l.gov;
                g.gate += l.gate;
                g.sessions += 1;
            }
            _ => {
                stray.insert(&l.session8);
            }
        }
    }
    (by, stray.len())
}

// spec: drift-kit/SPEC.md §The statistics view — the deferred age: whole days from the oldest defer
// date of the queue's deferred section to the close stamp's own date, never today
fn deferred_age_at(queue: &str, section: &str, close_date: &str) -> Option<i64> {
    let close = days_from_civil(close_date)?;
    let oldest = deferred_age::defer_dates(&kpi::section_lines(queue, section))
        .iter()
        .filter_map(|d| days_from_civil(d))
        .min()?;
    Some((close - oldest).max(0))
}

const FIGURES: [&str; 12] = [
    "spend",
    "spend by model",
    "cache-read",
    "cache-read by model",
    "governance share",
    "always-loaded",
    "commit shape",
    "amendments",
    "validate",
    "gate roster",
    "queue flow",
    "deferred age",
];

pub struct Rendered {
    pub document: String,
    pub report: String,
}

fn read_lines<T>(path: &str, parse: fn(&str) -> Option<T>) -> Option<Vec<T>> {
    std::fs::read(crate::walk::capture_path(path))
        .ok()
        .map(|b| String::from_utf8_lossy(&b).lines().filter_map(parse).collect())
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let n = count_arg(args)?;
    let knob = crate::walk::knob_scalar;
    let file = knob("DRIFT_KIT_STATISTICS_FILE")?;
    let h = trajectory::harvest()?;
    let closed_names: Vec<String> = h.closed.iter().map(|c| c.name.clone()).collect();
    let no_closed = if !h.state_present {
        Some(format!("no committed state file at {}", h.state_file))
    } else if h.closed.is_empty() {
        Some("no closed iteration".to_string())
    } else {
        None
    };
    let overhead = read_lines(&knob("DRIFT_KIT_OVERHEAD_LOG")?, ovh_line);
    let stamps = match overhead {
        Some(_) => stamp_pairs(&crate::history::stamp_lines(&knob("DRIFT_KIT_STATE_FILE")?)),
        None => Vec::new(),
    };
    let closed = trailing(h.closed, n);

    // spec: drift-kit/SPEC.md §The statistics view — the two close-commit reads share one batched
    // blob reader, so a long history buys one `git cat-file` process
    let mut blobs = always_loaded::blobs();
    let always = if kpi::always_loaded::present(&crate::walk::kit_roots_abs()?) {
        let surfaces = crate::walk::knob_array("CONTEXT_KIT_SURFACES")?;
        Ok(closed
            .iter()
            .map(|c| match blobs.as_mut() {
                Some(b) => always_loaded::surface_at(b, &c.close, &surfaces),
                None => 0,
            })
            .collect())
    } else {
        Err("context-kit absent".to_string())
    };
    let queue = knob("DRIFT_KIT_QUEUE_FILE")?;
    let top = crate::walk::toplevel().or_else(|_| crate::walk::cwd())?;
    let queue = crate::walk::rel_under(&top, &queue).unwrap_or(&queue).to_string();
    let section = knob("DRIFT_KIT_DEFERRED_SECTION")?;
    let deferred = closed
        .iter()
        .map(|c| {
            let text = blobs.as_mut()?.at(&c.close, &queue).ok().flatten()?;
            deferred_age_at(&text, &section, &c.close_date)
        })
        .collect();

    let inputs = Inputs {
        closed,
        closed_names,
        no_closed,
        log: read_lines(&knob("DRIFT_KIT_STAGE_ECONOMICS_LOG")?, stage_economics::parse_row),
        overhead,
        stamps,
        always,
        deferred,
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

const SPEND_CAPTION: &str = "the <code>cost</code> field of the stage-economics trend log \
     (<code>DRIFT_KIT_STAGE_ECONOMICS_LOG</code>)";

// spec: drift-kit/SPEC.md §The statistics view — the document and the per-figure report from one
// pass, so the stdout line for a panel cannot disagree with what the panel shows
pub fn render(inp: &Inputs) -> Rendered {
    let names: Vec<&str> = inp.closed.iter().map(|c| c.name.as_str()).collect();
    let ranked = match &inp.log {
        Some(rows) => rank_models(rows, &inp.closed_names),
        None => Vec::new(),
    };
    let spend: Vec<Spend> = match &inp.log {
        Some(rows) => names
            .iter()
            .map(|n| spend_of(rows, n, &inp.supervision, &inp.fanout_suffix, &ranked))
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
    let (gov_by, gov_stray) = match &inp.overhead {
        Some(lines) => governance(lines, &inp.stamps, &inp.closed_names),
        None => (HashMap::new(), 0),
    };
    let gov: Vec<Gov> = names
        .iter()
        .map(|n| gov_by.get(*n).copied().unwrap_or_default())
        .collect();

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
    let governed = match &inp.overhead {
        None => Err("no overhead log".to_string()),
        Some(_) => tracked(
            (!gov.iter().any(|g| g.shares().is_some()))
                .then(|| "no overhead-log session names a charted iteration".to_string()),
        ),
    };
    let loaded = match &inp.always {
        Err(r) => tracked(None).and(Err(r.clone())),
        Ok(_) => tracked(None),
    };
    let flowed = match &inp.windows {
        Err(r) => tracked(None).and(Err(r.clone())),
        Ok(_) => tracked(
            (!flow.iter().any(Option::is_some))
                .then(|| "no queue-flow window names a charted iteration".to_string()),
        ),
    };
    let aged = tracked(
        (!inp.deferred.iter().any(Option::is_some))
            .then(|| "no charted close commit holds a defer date".to_string()),
    );
    let status: [Result<(), String>; 12] = [
        logged.clone(),
        logged.clone(),
        logged.clone(),
        logged,
        governed,
        loaded,
        tracked(None),
        tracked(None),
        tracked(None),
        tracked(None),
        flowed,
        aged,
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
        (gov_stray, "overhead-log session(s) name no closed iteration"),
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

    let spend_caption = format!(
        "USD per iteration, {}, stage rows, the supervision row and fan-out rows stacked. A caret \
         marks a bar holding an <code>n/a</code> cost row; a baseline tick marks an iteration with \
         no trend-log row.",
        SPEND_CAPTION
    );
    let model_caption = format!(
        "USD per iteration, {}, stacked by the row's model. Models are ranked by spend over every \
         closed iteration, so a model keeps its colour whatever the window; past the seventh they \
         fold into one band. A caret marks a bar holding an <code>n/a</code> cost row.",
        SPEND_CAPTION
    );
    let panels: [(&str, String, &str); 12] = [
        ("Spend", spend_caption, "spend"),
        ("Spend by model", model_caption, "spend-by-model"),
        (
            "Cache-read",
            "Cache-read tokens per iteration, the <code>cr</code> field of the same trend log, in \
             the same three bands."
                .to_string(),
            "cache-read",
        ),
        (
            "Cache-read by model",
            "Cache-read tokens per iteration, the <code>cr</code> field of the same trend log, \
             stacked by model in the bands and colours of spend by model."
                .to_string(),
            "cache-read-by-model",
        ),
        (
            "Governance share",
            "The share of each iteration's measured session volume that is governance, gate \
             output and the rest stacked, from the overhead meter's log \
             (<code>DRIFT_KIT_OVERHEAD_LOG</code>), each line joined to the iteration its \
             session last stamped. A ratio of summed bytes, so it reads proportion only: bytes \
             are not tokens (the meter's byte-proxy caveat). A baseline tick marks an iteration \
             with no measured session."
                .to_string(),
            "governance-share",
        ),
        (
            "Always-loaded",
            "Code points of the always-loaded surfaces (<code>CONTEXT_KIT_SURFACES</code>, \
             today's roster) at each iteration's close commit, from committed history through \
             the always-loaded meter's past-commit read. The hook body is not recoverable at a \
             past commit and is not charted."
                .to_string(),
            "always-loaded",
        ),
        (
            "Commit shape",
            "Feature and debt commits in each iteration's close range, from committed history \
             (the trajectory extractor's commit-shape harvest)."
                .to_string(),
            "commit-shape",
        ),
        (
            "Amendments",
            "Amendments merged (upper strip) and the longest add-to-merge lag in days (lower \
             strip, drawn where one merged), from committed history (the trajectory extractor's \
             amendment-latency harvest)."
                .to_string(),
            "amendments",
        ),
        (
            "Validate",
            "The iteration's validate attestations, from the committed evidence manifest's \
             history (the trajectory extractor's validate harvest)."
                .to_string(),
            "validate",
        ),
        (
            "Gate roster",
            "The <code>gates.list</code> member count at each iteration's close commit, from \
             committed history."
                .to_string(),
            "gate-roster",
        ),
        (
            "Queue flow",
            "Design-pending entries filed (above the axis) and drained (below) per iteration \
             window, from the queue file at consecutive iteration-start commits (the queue-flow \
             arm), joined on each window's iteration label."
                .to_string(),
            "queue-flow",
        ),
        (
            "Deferred age",
            "Age in days of the oldest defer date in the queue file's deferred section at each \
             iteration's close commit, counted to the close stamp's date (<code>kpi-deferred-age</code>'s \
             read, from committed history). A baseline tick marks a close commit where that read \
             finds no defer date: no queue file, or entries in a grammar the read does not parse."
                .to_string(),
            "deferred-age",
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
                    1 => model_panel(&fr, &names, &spend, &ranked, true),
                    2 => cache_panel(&fr, &names, &spend),
                    3 => model_panel(&fr, &names, &spend, &ranked, false),
                    4 => governance_panel(&fr, &names, &gov),
                    5 => always_panel(&fr, &names, inp.always.as_deref().unwrap_or(&[])),
                    6 => commit_panel(&fr, &inp.closed),
                    7 => amendment_panel(&fr, &inp.closed),
                    8 => validate_panel(&fr, &inp.closed),
                    9 => gate_panel(&fr, &inp.closed),
                    10 => flow_panel(&fr, &names, &flow),
                    _ => deferred_panel(&fr, &inp.closed, &inp.deferred),
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
  --s1: #2a78d6; --s2: #eb6834; --s3: #1baf7a; --s4: #eda100; --s5: #e87ba4; --s6: #008300;
  --s7: #6250d6; --other: #898781; --good: #0ca30c; --critical: #d03b3b;
}
@media (prefers-color-scheme: dark) {
  .viz-root {
    color-scheme: dark;
    --page: #0d0d0d; --surface: #1a1a19; --ink: #f0efec; --ink-2: #c3c2b7; --muted: #898781;
    --grid: #2c2c2a; --axis: #383835; --ring: rgba(255,255,255,0.10);
    --s1: #3987e5; --s2: #d95926; --s3: #199e70; --s4: #c98500; --s5: #d55181; --s6: #008300;
    --s7: #9085e9; --other: #898781;
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
.s1 { fill: var(--s1); } .s2 { fill: var(--s2); } .s3 { fill: var(--s3); } .s4 { fill: var(--s4); }
.s5 { fill: var(--s5); } .s6 { fill: var(--s6); } .s7 { fill: var(--s7); } .other { fill: var(--other); }
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
            class,
            esc(label)
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

// spec: drift-kit/SPEC.md §The statistics view — one stacked column: its bands in key order, the
// column's hover summary, a baseline tick in place of a bar where the input holds no value, and a
// caret over a bar whose total is incomplete
struct Column {
    parts: Vec<f64>,
    summary: String,
    empty: bool,
    caret: bool,
}

fn bands_panel(
    fr: &Frame,
    names: &[&str],
    keys: &[(&str, String)],
    cols: &[Column],
    (top, step): (f64, f64),
    axis: fn(f64) -> String,
    value: fn(f64) -> String,
) -> String {
    let mut out = String::new();
    let refs: Vec<(&str, &str)> = keys.iter().map(|(c, l)| (*c, l.as_str())).collect();
    legend(&mut out, &refs);
    let h = TOP + PLOT_H + LABELS;
    svg_open(&mut out, h);
    y_axis(&mut out, TOP, PLOT_H, top, step, axis);
    let base = TOP + PLOT_H;
    for (i, col) in cols.iter().enumerate() {
        hit(&mut out, fr, i, TOP, PLOT_H, &col.summary);
        if col.empty {
            rect(&mut out, "tick", fr.left(i), base - 2.0, fr.bar, 2.0, &col.summary);
            continue;
        }
        let parts: Vec<(&str, f64, String)> = keys
            .iter()
            .zip(&col.parts)
            .map(|((class, label), v)| (*class, *v, format!("{}: {} {}", names[i], label, value(*v))))
            .collect();
        stack(&mut out, fr, i, base, top, &parts);
        if col.caret {
            let total: f64 = col.parts.iter().sum();
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
                esc(&col.summary)
            );
        }
    }
    x_labels(&mut out, fr, names, h - 6.0);
    out.push_str("</svg>\n");
    out
}

fn col_max(cols: &[Column]) -> f64 {
    cols.iter()
        .filter(|c| !c.empty)
        .map(|c| c.parts.iter().sum::<f64>())
        .fold(0.0, f64::max)
}

fn incomplete(s: &Spend) -> String {
    if s.unpriced > 0 {
        format!("; incomplete pricing, {} row(s) cost n/a", s.unpriced)
    } else {
        String::new()
    }
}

const STAGE_BANDS: [(&str, &str); 3] = [("s1", "stage rows"), ("s2", "supervision"), ("s3", "fan-out")];

fn stage_keys() -> Vec<(&'static str, String)> {
    STAGE_BANDS.iter().map(|(c, l)| (*c, l.to_string())).collect()
}

// spec: drift-kit/SPEC.md §The statistics view — the model bands' keys: the ranked models in
// their slots, then the fold band where an eighth model exists
fn model_keys(ranked: &[String]) -> Vec<(&'static str, String)> {
    let mut keys: Vec<(&str, String)> = ranked
        .iter()
        .take(SLOTS.len())
        .enumerate()
        .map(|(i, m)| (SLOTS[i], m.clone()))
        .collect();
    if ranked.len() > SLOTS.len() {
        keys.push((OTHER, OTHER_LABEL.to_string()));
    }
    keys
}

fn no_row(name: &str) -> String {
    format!("{}: no trend-log row", name)
}

fn spend_panel(fr: &Frame, names: &[&str], spend: &[Spend]) -> String {
    let cols: Vec<Column> = spend
        .iter()
        .zip(names)
        .map(|(s, name)| Column {
            parts: s.bands.to_vec(),
            summary: if s.rows == 0 {
                no_row(name)
            } else {
                format!(
                    "{}: {} total (stage {}, supervision {}, fan-out {}){}",
                    name,
                    usd(s.bands.iter().sum()),
                    usd(s.bands[0]),
                    usd(s.bands[1]),
                    usd(s.bands[2]),
                    incomplete(s)
                )
            },
            empty: s.rows == 0,
            caret: s.unpriced > 0,
        })
        .collect();
    let range = scale(col_max(&cols), false);
    bands_panel(fr, names, &stage_keys(), &cols, range, usd_axis, usd)
}

fn cache_panel(fr: &Frame, names: &[&str], spend: &[Spend]) -> String {
    let cols: Vec<Column> = spend
        .iter()
        .zip(names)
        .map(|(s, name)| Column {
            parts: s.cr_bands.iter().map(|v| *v as f64).collect(),
            summary: if s.rows == 0 {
                no_row(name)
            } else {
                format!(
                    "{}: {} cache-read tokens (stage {}, supervision {}, fan-out {})",
                    name,
                    s.cr_bands.iter().sum::<u64>(),
                    s.cr_bands[0],
                    s.cr_bands[1],
                    s.cr_bands[2]
                )
            },
            empty: s.rows == 0,
            caret: false,
        })
        .collect();
    let max = col_max(&cols);
    bands_panel(fr, names, &stage_keys(), &cols, scale(max, max < 1e3), tokens, int)
}

// spec: drift-kit/SPEC.md §The statistics view — the by-model panels, spend or cache-read, sharing
// one slot assignment; each column's title lists every model's value, so the numbers read without
// telling two colours apart
fn model_panel(fr: &Frame, names: &[&str], spend: &[Spend], ranked: &[String], cost: bool) -> String {
    let keys = model_keys(ranked);
    let cols: Vec<Column> = spend
        .iter()
        .zip(names)
        .map(|(s, name)| {
            let parts: Vec<f64> = if cost {
                s.models.clone()
            } else {
                s.cr_models.iter().map(|v| *v as f64).collect()
            };
            let fmt = |v: f64| if cost { usd(v) } else { int(v) };
            let each: Vec<String> = keys
                .iter()
                .zip(&parts)
                .filter(|(_, v)| **v > 0.0)
                .map(|((_, l), v)| format!("{} {}", l, fmt(*v)))
                .collect();
            let total: f64 = parts.iter().sum();
            let summary = if s.rows == 0 {
                no_row(name)
            } else if cost {
                format!("{}: {} total ({}){}", name, usd(total), each.join(", "), incomplete(s))
            } else {
                format!("{}: {} cache-read tokens ({})", name, int(total), each.join(", "))
            };
            Column {
                parts,
                summary,
                empty: s.rows == 0,
                caret: cost && s.unpriced > 0,
            }
        })
        .collect();
    let max = col_max(&cols);
    if cost {
        bands_panel(fr, names, &keys, &cols, scale(max, false), usd_axis, usd)
    } else {
        bands_panel(fr, names, &keys, &cols, scale(max, max < 1e3), tokens, int)
    }
}

fn pct(v: f64) -> String {
    format!("{}%", c(v))
}

fn governance_panel(fr: &Frame, names: &[&str], gov: &[Gov]) -> String {
    let keys = vec![("s1", "gate output".to_string()), ("s2", "rest of governance".to_string())];
    let cols: Vec<Column> = gov
        .iter()
        .zip(names)
        .map(|(g, name)| match g.shares() {
            Some((gate, rest)) => Column {
                parts: vec![gate, rest],
                summary: format!(
                    "{}: governance {} of volume (gate output {}, rest {}) over {} measured session(s)",
                    name,
                    pct(gate + rest),
                    pct(gate),
                    pct(rest),
                    g.sessions
                ),
                empty: false,
                caret: false,
            },
            None => Column {
                parts: vec![0.0, 0.0],
                summary: format!("{}: no measured session", name),
                empty: true,
                caret: false,
            },
        })
        .collect();
    bands_panel(fr, names, &keys, &cols, (100.0, 25.0), pct, pct)
}

// spec: drift-kit/SPEC.md §The statistics view — a per-iteration line: segments join consecutive
// valued points, and a point with no value is a baseline tick
fn line_panel(
    fr: &Frame,
    pts: &[Option<f64>],
    titles: &[String],
    names: &[&str],
    axis: fn(f64) -> String,
) -> String {
    let mut out = String::new();
    let max = pts.iter().flatten().copied().fold(0.0, f64::max);
    let (top, step) = scale(max, max < 1e3);
    let h = TOP + PLOT_H + LABELS;
    let base = TOP + PLOT_H;
    svg_open(&mut out, h);
    y_axis(&mut out, TOP, PLOT_H, top, step, axis);
    let y = |v: f64| base - v / top * PLOT_H;
    for (i, p) in pts.iter().enumerate() {
        hit(&mut out, fr, i, TOP, PLOT_H, &titles[i]);
        if p.is_none() {
            rect(&mut out, "tick", fr.left(i), base - 2.0, fr.bar, 2.0, &titles[i]);
        }
    }
    let mut d = String::new();
    for (i, w) in pts.windows(2).enumerate() {
        if let (Some(a), Some(b)) = (w[0], w[1]) {
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
    for (i, p) in pts.iter().enumerate() {
        if let Some(v) = p {
            let _ = writeln!(
                out,
                "<circle class=\"dot1\" cx=\"{}\" cy=\"{}\" r=\"{}\"><title>{}</title></circle>",
                c(fr.center(i)),
                c(y(*v)),
                c((fr.bar / 2.0).clamp(1.5, 4.0)),
                esc(&titles[i])
            );
        }
    }
    x_labels(&mut out, fr, names, h - 6.0);
    out.push_str("</svg>\n");
    out
}

fn always_panel(fr: &Frame, names: &[&str], always: &[u64]) -> String {
    let pts: Vec<Option<f64>> = always.iter().map(|v| Some(*v as f64)).collect();
    let titles: Vec<String> = always
        .iter()
        .zip(names)
        .map(|(v, name)| format!("{}: {}cp of always-loaded surfaces at close", name, v))
        .collect();
    line_panel(fr, &pts, &titles, names, tokens)
}

fn deferred_panel(fr: &Frame, closed: &[Closed], deferred: &[Option<i64>]) -> String {
    let names = names_of(closed);
    let pts: Vec<Option<f64>> = deferred.iter().map(|d| d.map(|v| v as f64)).collect();
    let titles: Vec<String> = closed
        .iter()
        .zip(deferred)
        .map(|(e, d)| match d {
            Some(d) => format!(
                "{}: oldest deferred entry {} day(s) old at close ({})",
                e.name, d, e.close_date
            ),
            None => format!("{}: no defer date read at close", e.name),
        })
        .collect();
    line_panel(fr, &pts, &titles, &names, int)
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
            close: format!("{}-close", name),
            close_date: "2026-01-10".to_string(),
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

    fn modelled(iteration: &str, stage: &str, model: &str, cr: u64, cost: &str) -> LogRow {
        LogRow {
            date: "2026-01-01".to_string(),
            iteration: iteration.to_string(),
            stage: stage.to_string(),
            model: model.to_string(),
            tokens: stage_economics::Tokens {
                input: 1,
                output: 1,
                cache_read: cr,
                cache_write: 1,
            },
            cost: cost.to_string(),
        }
    }

    fn row(iteration: &str, stage: &str, cost: &str) -> LogRow {
        modelled(iteration, stage, "m", 100, cost)
    }

    fn ovh(session: &str, total: f64, gov: f64, gate: f64) -> Ovh {
        Ovh {
            session8: session.to_string(),
            total,
            gov,
            gate,
        }
    }

    fn pair(iteration: &str, session: &str) -> (String, String) {
        (iteration.to_string(), session.to_string())
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
                modelled("alpha", "build", "m2", 50, "0.5"),
                row("beta", "close", "n/a"),
                row("in-flight", "scope", "1.0"),
            ]),
            overhead: Some(vec![
                ovh("aaaa1111", 1000.0, 600.0, 100.0),
                ovh("ffff9999", 10.0, 1.0, 1.0),
            ]),
            stamps: vec![pair("alpha", "aaaa1111")],
            always: Ok(vec![1200, 1100]),
            deferred: vec![Some(9), None],
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
        let ranked = rank_models(rows, &inp.closed_names);
        let a = spend_of(rows, "alpha", "supervision", "+fanout", &ranked);
        assert_eq!(a.bands, [2.0, 0.25, 0.75]);
        assert_eq!(a.cr_bands, [150, 100, 200]);
        assert_eq!((a.unpriced, a.rows), (0, 5));
        let b = spend_of(rows, "beta", "supervision", "+fanout", &ranked);
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

    // spec: drift-kit/SPEC.md §The statistics view — the model bands: the ranking spans every closed
    // iteration so a smaller window repaints nothing, the eighth model folds into one band, and a
    // model carrying no token draws none
    #[test]
    fn the_model_ranking_ignores_the_window_and_folds_the_eighth_model() {
        let mut rows: Vec<LogRow> = (1..=8)
            .map(|k| modelled("alpha", "build", &format!("m{}", k), 10, &format!("{}", k)))
            .collect();
        rows.push(modelled("beta", "build", "m1", 5, "0.5"));
        rows.push(modelled("beta", "build", "m8", 5, "0.5"));
        rows.push(modelled("beta", "build", "tie-b", 9, "0"));
        rows.push(modelled("beta", "build", "tie-a", 9, "0"));
        let mut empty = modelled("beta", "build", "zero-model", 0, "n/a");
        empty.tokens = stage_economics::Tokens::default();
        rows.push(empty);
        rows.push(modelled("in-flight", "build", "m-late", 1, "99"));
        let names = vec!["alpha".to_string(), "beta".to_string()];
        let ranked = rank_models(&rows, &names);
        assert_eq!(
            ranked,
            ["m8", "m7", "m6", "m5", "m4", "m3", "m2", "m1", "tie-a", "tie-b"]
        );
        assert_eq!(slot_of(&ranked, "m2"), Some(6));
        assert_eq!(slot_of(&ranked, "m1"), Some(7));
        assert_eq!(slot_of(&ranked, "tie-b"), Some(7));
        assert_eq!(slot_of(&ranked, "zero-model"), None);
        let keys = model_keys(&ranked);
        assert_eq!(keys.len(), 8);
        assert_eq!(keys[7], (OTHER, OTHER_LABEL.to_string()));
        let b = spend_of(&rows, "beta", "", "", &ranked);
        assert_eq!(b.models[0], 0.5);
        assert_eq!(b.models[7], 0.5);
        assert_eq!(b.cr_models[7], 23);

        let mut full = inputs();
        full.log = Some(rows);
        let mut trimmed = inputs();
        trimmed.log = full.log.clone();
        trimmed.closed.remove(0);
        trimmed.always = Ok(vec![1100]);
        trimmed.deferred = vec![None];
        let m8_in_slot_one = "class=\"s1\" x=";
        for doc in [render(&full).document, render(&trimmed).document] {
            let at = doc.find("beta: m8 $0.50</title>").expect("m8's beta band");
            let open = doc[..at].rfind("<rect class=\"").expect("its rect");
            assert!(doc[open..].starts_with(&format!("<rect {}", m8_in_slot_one)), "{}", &doc[open..at]);
            assert!(doc.contains("other models"));
        }
    }

    // spec: drift-kit/SPEC.md §The statistics view — the governance join: a session stamped in two
    // iterations lands in its last stamp's, the share is a ratio of sums, and a session no stamp
    // names charts nowhere and is counted
    #[test]
    fn a_governance_line_joins_its_sessions_last_stamp_and_shares_are_ratios_of_sums() {
        let lines = vec![
            ovh("twice000", 1000.0, 500.0, 100.0),
            ovh("once0000", 3000.0, 1500.0, 0.0),
            ovh("nostamp0", 50.0, 50.0, 50.0),
            ovh("flight00", 50.0, 50.0, 50.0),
        ];
        let stamps = vec![
            pair("alpha", "twice000"),
            pair("beta", "twice000"),
            pair("beta", "once0000"),
            pair("in-flight", "flight00"),
        ];
        let names = vec!["alpha".to_string(), "beta".to_string()];
        let (by, stray) = governance(&lines, &stamps, &names);
        assert!(!by.contains_key("alpha"));
        let b = by["beta"];
        assert_eq!((b.sessions, b.total, b.gov, b.gate), (2, 4000.0, 2000.0, 100.0));
        assert_eq!(b.shares(), Some((2.5, 47.5)));
        assert_eq!(stray, 2);
        assert_eq!(
            stamp_pairs(&[
                "# contract".to_string(),
                "---".to_string(),
                "alpha build s2 2025-01-01 h".to_string(),
                "alpha build s2 2025-01-01 h".to_string(),
                "beta scope".to_string(),
            ]),
            vec![pair("alpha", "s2")]
        );
        assert_eq!(
            ovh_line("2026-07-11 472b902e total=111395 gov=71289 gate=6976 pct=64"),
            Some(ovh("472b902e", 111395.0, 71289.0, 6976.0))
        );
        assert_eq!(ovh_line("2026-07-11 total=1 gov=1 gate=1"), None);
        assert_eq!(ovh_line("2026-07-11 472b902e total=1 gate=1"), None);

        let r = render(&inputs());
        assert!(r.report.contains("  governance share: charted\n"), "{}", r.report);
        assert!(r.report.contains("  1 overhead-log session(s) name no closed iteration\n"));
        assert!(r
            .document
            .contains("alpha: governance 60% of volume (gate output 10%, rest 50%)"));
        assert!(r.document.contains("beta: no measured session"));
        assert!(!r.document.contains("aaaa1111") && !r.document.contains("ffff9999"));
    }

    // spec: drift-kit/SPEC.md §The statistics view — the deferred age counts whole days from the
    // oldest defer date in the deferred section to the close stamp's date, never today
    #[test]
    fn the_deferred_age_counts_to_the_close_stamps_date_off_the_deferred_section() {
        let queue = "# Queue\n## Done\n### old-done\nSurfaced 2020-01-01.\n## Deferred\n\
                     ### later\nFiled 2026-01-05 by close.\n### oldest\nSurfaced 2025-12-31 at \
                     build.\nFiled 2025-01-01.\n## Icebox\n### ice\nSurfaced 2019-01-01.\n";
        assert_eq!(deferred_age_at(queue, "Deferred", "2026-01-10"), Some(10));
        assert_eq!(deferred_age_at(queue, "Deferred", "2026-01-01"), Some(1));
        assert_eq!(deferred_age_at(queue, "Deferred", ""), None);
        assert_eq!(
            deferred_age_at("## Deferred\n### x\nno date\n", "Deferred", "2026-01-10"),
            None
        );
        let r = render(&inputs());
        assert!(r.report.contains("  deferred age: charted\n"), "{}", r.report);
        assert!(r
            .document
            .contains("alpha: oldest deferred entry 9 day(s) old at close (2026-01-10)"));
        assert!(r.document.contains("beta: no defer date read at close"));
        assert!(r
            .document
            .contains("alpha: 1200cp of always-loaded surfaces at close"));
    }

    // spec: drift-kit/SPEC.md §The statistics view — every panel present, the document free of
    // script and of any reference outside itself
    #[test]
    fn the_document_holds_twelve_panels_and_references_nothing_outside_itself() {
        let r = render(&inputs());
        for f in FIGURES {
            assert!(r.report.contains(&format!("  {}: charted\n", f)), "{}", r.report);
        }
        assert_eq!(r.document.matches("<section class=\"panel\"").count(), 12);
        for banned in ["<script", "src=", "url(", "href", "@import"] {
            assert!(!r.document.contains(banned), "{}", banned);
        }
        assert!(r.document.contains("No controlled ungoverned baseline exists"));
    }

    // spec: drift-kit/SPEC.md §The statistics view — Degrade: each log's absence takes its own
    // figures, context-kit's absence takes always-loaded, no defer date takes deferred age, an
    // absent state file takes every figure, and a queue-flow reason takes queue flow alone
    #[test]
    fn a_missing_input_degrades_only_the_panels_reading_it() {
        let mut inp = inputs();
        inp.log = None;
        inp.overhead = None;
        inp.always = Err("context-kit absent".to_string());
        inp.deferred = vec![None, None];
        inp.windows = Err("fewer than two iteration-start commits in reach".to_string());
        let r = render(&inp);
        for f in ["spend", "spend by model", "cache-read", "cache-read by model"] {
            assert!(
                r.report.contains(&format!("  {}: n/a (no stage-economics log)\n", f)),
                "{}",
                r.report
            );
        }
        assert!(r.report.contains("  governance share: n/a (no overhead log)\n"));
        assert!(r.report.contains("  always-loaded: n/a (context-kit absent)\n"));
        assert!(r
            .report
            .contains("  deferred age: n/a (no charted close commit holds a defer date)\n"));
        assert!(r.report.contains("  commit shape: charted\n"));
        assert!(r.report.contains("  gate roster: charted\n"));
        assert!(r
            .report
            .contains("  queue flow: n/a (fewer than two iteration-start commits in reach)\n"));
        assert_eq!(r.document.matches("<section class=\"panel\"").count(), 12);

        let mut unjoined = inputs();
        unjoined.stamps.clear();
        let r = render(&unjoined);
        assert!(r.report.contains(
            "  governance share: n/a (no overhead-log session names a charted iteration)\n"
        ));
        assert!(r.report.contains("  2 overhead-log session(s) name no closed iteration\n"));

        let mut none = inputs();
        none.closed.clear();
        none.closed_names.clear();
        none.always = Ok(Vec::new());
        none.deferred.clear();
        none.no_closed = Some("no committed state file at s".to_string());
        let r = render(&none);
        assert_eq!(
            r.report.matches("n/a (no committed state file at s)").count(),
            12,
            "{}",
            r.report
        );
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
