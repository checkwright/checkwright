// spec: drift-kit/SPEC.md §The stage-economics meter — lifecycle spend by stage × model ×
// iteration: WORKFLOW-STATE stamps ⋈ transcripts ⋈ a consumer price table, advisory by
// construction so exit is always 0 and a missing input is a notice rather than a failure.
// spec: gate-sdk/SPEC.md §The non-gate arm — a bridged-arm table member on the forced-family test
// (it resolves seven consumer knobs), and `Arm::Emit` because exit is always 0, so no `1` is
// load-bearing and the `{0, 2}` collapse costs nothing.
use std::collections::HashMap;

pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_METRIC_DIR",
    "DRIFT_KIT_STAGE_ECONOMICS_LOG",
    "DRIFT_KIT_PRICE_TABLE",
    "DRIFT_KIT_STATE_FILE",
    "DRIFT_KIT_SESSIONS_DIR",
    "DRIFT_KIT_SUPERVISION_LABEL",
    "DRIFT_KIT_FANOUT_SUFFIX",
];

// spec: drift-kit/SPEC.md §The stage-economics meter — the four token categories, carried as one
// value everywhere they travel so a pass cannot fold three of them and drop the fourth.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Tokens {
    pub input: u64,
    pub output: u64,
    pub cache_read: u64,
    pub cache_write: u64,
}

impl Tokens {
    fn add(&mut self, o: Tokens) {
        self.input += o.input;
        self.output += o.output;
        self.cache_read += o.cache_read;
        self.cache_write += o.cache_write;
    }
}

// spec: drift-kit/SPEC.md §The stage-economics meter — insertion order is load-bearing in four
// places the shell held with explicit `*_ORDER` vectors, because bash maps are unordered: which
// anchor is recorded first and which iteration takes an apportionment remainder both read it.
struct Ordered<V> {
    order: Vec<String>,
    map: HashMap<String, V>,
}

impl<V> Default for Ordered<V> {
    fn default() -> Self {
        Ordered {
            order: Vec::new(),
            map: HashMap::new(),
        }
    }
}

impl<V> Ordered<V> {
    fn get(&self, k: &str) -> Option<&V> {
        self.map.get(k)
    }
    fn contains(&self, k: &str) -> bool {
        self.map.contains_key(k)
    }
    fn insert_new(&mut self, k: &str, v: V) -> bool {
        if self.map.contains_key(k) {
            return false;
        }
        self.order.push(k.to_string());
        self.map.insert(k.to_string(), v);
        true
    }
    fn set(&mut self, k: &str, v: V) {
        if !self.map.contains_key(k) {
            self.order.push(k.to_string());
        }
        self.map.insert(k.to_string(), v);
    }
    fn entry_mut(&mut self, k: &str, seed: V) -> &mut V {
        if !self.map.contains_key(k) {
            self.order.push(k.to_string());
            self.map.insert(k.to_string(), seed);
        }
        self.map.get_mut(k).expect("just inserted")
    }
    fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.order.iter().map(|k| (k, &self.map[k]))
    }
}

// spec: drift-kit/SPEC.md §The stage-economics meter — a streaming transcript repeats a message id
// across lines with input and cache constant and output growing, so the **last** record per id is
// kept and raw lines are never summed; an unreadable line is skipped rather than fatal.
pub fn usage_by_model(body: &str) -> Vec<(String, Tokens)> {
    let mut ids: Ordered<(String, Tokens)> = Ordered::default();
    for line in body.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if v.get("type").and_then(|t| t.as_str()) != Some("assistant") {
            continue;
        }
        let Some(msg) = v.get("message") else { continue };
        let Some(usage) = msg.get("usage").filter(|u| !u.is_null()) else {
            continue;
        };
        let id = msg.get("id").and_then(|x| x.as_str()).unwrap_or("?");
        let model = msg
            .get("model")
            .and_then(|x| x.as_str())
            .unwrap_or("?")
            .to_string();
        let n = |k: &str| usage.get(k).and_then(|x| x.as_u64()).unwrap_or(0);
        let t = Tokens {
            input: n("input_tokens"),
            output: n("output_tokens"),
            cache_read: n("cache_read_input_tokens"),
            cache_write: n("cache_creation_input_tokens"),
        };
        ids.set(id, (model, t));
    }
    let mut by_model: Ordered<Tokens> = Ordered::default();
    for (_, (model, t)) in ids.iter() {
        by_model.entry_mut(model, Tokens::default()).add(*t);
    }
    by_model.iter().map(|(m, t)| (m.clone(), *t)).collect()
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the apportionment key: an integer split in
// proportion to the given counts, remainder to the **first**, so the parts re-sum to the whole
// exactly. The caller's order is the split's order and the first slot is where the remainder lands.
pub fn split_tokens(total: u64, counts: &[u64]) -> Vec<u64> {
    let sum: u64 = counts.iter().sum();
    if sum == 0 {
        return vec![total];
    }
    let mut parts: Vec<u64> = counts.iter().map(|n| total * n / sum).collect();
    let acc: u64 = parts.iter().sum();
    parts[0] += total - acc;
    parts
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the stamp grammar the committed-history arm
// filters added lines by: `<iteration> <stage> <session8> <YYYY-MM-DD>` with exactly one space
// between fields, which is what keeps a diff header and a comment line out of the union.
pub fn history_stamp(added: &str) -> Option<&str> {
    let line = added.strip_prefix('+')?;
    let f: Vec<&str> = line.splitn(5, ' ').collect();
    if f.len() < 4 {
        return None;
    }
    let kebab = |s: &str, head: fn(u8) -> bool| {
        !s.is_empty()
            && head(s.as_bytes()[0])
            && s.bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    };
    if !kebab(f[0], |b| b.is_ascii_lowercase() || b.is_ascii_digit())
        || !kebab(f[1], |b| b.is_ascii_lowercase())
        || f[2].is_empty()
        || !f[2].bytes().all(|b| b.is_ascii_alphanumeric())
    {
        return None;
    }
    let d = f[3].as_bytes();
    if d.len() < 10 || d[4] != b'-' || d[7] != b'-' {
        return None;
    }
    if ![0, 1, 2, 3, 5, 6, 8, 9]
        .iter()
        .all(|i| d[*i].is_ascii_digit())
    {
        return None;
    }
    Some(line)
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the price table is consumer config (the
// provenance seam). Blank rows, `#` comments and the `model` header are skipped; a model with no
// row degrades that cell rather than failing, so an absent table degrades and never errors.
#[derive(Default)]
pub struct Prices {
    pub present: bool,
    rows: HashMap<String, [f64; 4]>,
}

impl Prices {
    pub fn parse(text: &str) -> Prices {
        let mut rows: HashMap<String, [f64; 4]> = HashMap::new();
        for line in text.lines() {
            let f: Vec<&str> = line.split('\t').collect();
            let m = f[0];
            if m.is_empty() || m.starts_with('#') || m == "model" {
                continue;
            }
            let at = |i: usize| f.get(i).map_or(0.0, |v| v.trim().parse::<f64>().unwrap_or(0.0));
            rows.insert(m.to_string(), [at(1), at(2), at(3), at(4)]);
        }
        Prices {
            present: true,
            rows,
        }
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — floating-point addition is not
    // associative, so the four terms sum in the shell's own left-to-right order (in, out,
    // cache-read, cache-creation) or a cost's last digit can differ from the series already logged.
    pub fn cell(&self, model: &str, t: Tokens) -> Option<String> {
        let p = self.rows.get(model)?;
        let cost = t.input as f64 * p[0]
            + t.output as f64 * p[1]
            + t.cache_read as f64 * p[2]
            + t.cache_write as f64 * p[3];
        Some(format!("{:.4}", cost))
    }
}

// spec: drift-kit/SPEC.md §The stage-economics meter — an anchor is a transcript that already holds
// a row, carrying the apportionment its own row was split by, so a lead's fan-out and its
// supervision row can never disagree about which iteration the lead belonged to.
struct Anchor {
    label: String,
    iters: Vec<String>,
    counts: Vec<u64>,
    who: String,
}

struct Run {
    today: String,
    log: String,
    supervision: String,
    fanout_suffix: String,
    prices: Prices,
    inputs: crate::sessions::Inputs,
    out: String,
    rows: usize,
    incomplete: bool,
    kept: Vec<String>,
}

impl Run {
    fn say(&mut self, line: &str) {
        self.out.push_str(line);
        self.out.push('\n');
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — the log's dedup key is the
    // `<iteration> <stage> <model>` triple, filtered on append exactly as `session8` is in the
    // sibling meter.
    // comment-tier-exempt: the buffering is this implementation's alone and no section asserts a
    // per-row rewrite — the run reads the log once and writes it once, which is observationally
    // the shell's read-filter-rewrite per row and cheaper by the number of rows
    fn emit_row(&mut self, iter: &str, stage: &str, who: &str, model: &str, t: Tokens) {
        let cost = self
            .prices
            .cell(model, t)
            .unwrap_or_else(|| "n/a".to_string());
        if cost == "n/a" {
            self.incomplete = true;
        }
        self.say(&format!(
            "  {} {} {} [{}]: in={} out={} cr={} cw={} cost={}",
            iter, stage, who, model, t.input, t.output, t.cache_read, t.cache_write, cost
        ));
        let marker = format!(" {} {} {} in=", iter, stage, model);
        self.kept.retain(|l| !l.contains(&marker));
        self.kept.push(format!(
            "{} {} {} {} in={} out={} cr={} cw={} cost={}",
            self.today, iter, stage, model, t.input, t.output, t.cache_read, t.cache_write, cost
        ));
        self.rows += 1;
    }

    fn usage(&self, transcript: &str) -> Vec<(String, Tokens)> {
        match std::fs::read(transcript) {
            Ok(b) => usage_by_model(&String::from_utf8_lossy(&b)),
            Err(_) => Vec::new(),
        }
    }
}

fn knob(name: &str) -> Result<String, String> {
    crate::walk::knob_scalar(name)
}

// spec: drift-kit/SPEC.md §The stage-economics meter — history ∪ live, so the boundary truncation
// of the live file destroys no economics and a stamped-but-uncommitted stage stays visible; the
// 0-exit *nothing to read* notice fires only when **both** sources yield no stamps.
fn collect_stamps(state_file: &str) -> Vec<String> {
    let top = crate::walk::toplevel()
        .or_else(|_| crate::walk::cwd())
        .unwrap_or_else(|_| ".".to_string());
    let git = crate::history::Git { top };
    let mut out: Vec<String> = crate::history::added_lines(&git, state_file)
        .into_iter()
        .filter_map(|(_, l)| history_stamp(&l).map(str::to_string))
        .collect();
    if let Ok(b) = std::fs::read(state_file) {
        out.extend(String::from_utf8_lossy(&b).lines().map(str::to_string));
    }
    out
}

pub fn emit(args: &[String]) -> Result<String, String> {
    if !args.is_empty() {
        return Err("usage: --emit stage-economics   (it takes no argument)".to_string());
    }
    let state_file = knob("DRIFT_KIT_STATE_FILE")?;
    let price_table = knob("DRIFT_KIT_PRICE_TABLE")?;
    let var = |n: &str| std::env::var(n).unwrap_or_default();
    let pwd = var("PWD");
    let here = if pwd.is_empty() {
        crate::walk::cwd()?
    } else {
        pwd
    };
    let mut r = Run {
        today: super::kpi::today_iso(),
        log: knob("DRIFT_KIT_STAGE_ECONOMICS_LOG")?,
        supervision: knob("DRIFT_KIT_SUPERVISION_LABEL")?,
        fanout_suffix: knob("DRIFT_KIT_FANOUT_SUFFIX")?,
        prices: match std::fs::read_to_string(&price_table) {
            Ok(t) => Prices::parse(&t),
            Err(_) => Prices::default(),
        },
        inputs: crate::sessions::Inputs {
            session_id: String::new(),
            harness_id: String::new(),
            child: String::new(),
            sessions_dir: knob("DRIFT_KIT_SESSIONS_DIR")?,
            config_home: var("CLAUDE_CONFIG_DIR"),
            home: var("HOME"),
            here,
        },
        out: String::new(),
        rows: 0,
        incomplete: false,
        kept: Vec::new(),
    };
    if let Ok(b) = std::fs::read(&r.log) {
        r.kept = String::from_utf8_lossy(&b)
            .lines()
            .map(str::to_string)
            .collect();
    }
    let mut head = String::new();
    if !std::path::Path::new(&state_file).is_file() {
        head.push_str(&format!(
            "stage-economics: no live state file ({}) — reading committed history alone\n",
            state_file
        ));
    }
    head.push_str(&format!("stage-economics: {}\n", r.today));
    if !r.prices.present {
        head.push_str(&format!(
            "  no price table ({}) — token-only, cost=n/a (degraded, not failed)\n",
            price_table
        ));
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — the attribution invariant: one
    // transcript, one (iteration, stage). The stamp pass keys on the **session**, not the stamp, so
    // a session bearing two stamps resolves to its last and the yielded stamps take no row.
    let mut stamp_seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut sessions: Ordered<(String, String)> = Ordered::default();
    let mut yielded: HashMap<String, String> = HashMap::new();
    let (mut label_collision, mut suffix_collision, mut stamps) = (false, false, 0usize);
    for line in collect_stamps(&state_file) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.is_empty() || f[0].starts_with('#') || f[0] == "---" || f.len() < 3 {
            continue;
        }
        let (iter, stage, session8) = (f[0], f[1], f[2]);
        if !stamp_seen.insert(format!("{}/{}/{}", iter, stage, session8)) {
            continue;
        }
        stamps += 1;
        if stage == r.supervision {
            label_collision = true;
        }
        if !r.fanout_suffix.is_empty() && stage.ends_with(&r.fanout_suffix) {
            suffix_collision = true;
        }
        let prior = sessions.get(session8).map(|(i, s)| format!("{} {}", i, s));
        if let Some(prior) = prior {
            yielded
                .entry(session8.to_string())
                .or_default()
                .push_str(&format!("{}; ", prior));
        }
        sessions.set(session8, (iter.to_string(), stage.to_string()));
    }
    if stamps == 0 {
        head.push_str(&format!(
            "stage-economics: no stamps in either source (committed history or {}) — nothing to \
             read\n  help: set DRIFT_KIT_STATE_FILE to the WORKFLOW-STATE path carrying the stage \
             stamps.\n",
            state_file
        ));
        return Ok(head);
    }

    let mut attributed: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut anchors: Ordered<Anchor> = Ordered::default();
    let mut dispatch: HashMap<String, u64> = HashMap::new();
    let mut leads: Vec<String> = Vec::new();
    let (mut unmatched, mut unstamped) = (0usize, 0usize);

    let session_order: Vec<String> = sessions.order.clone();
    for session8 in &session_order {
        let (iter, stage) = sessions.map[session8].clone();
        let Some(transcript) = crate::sessions::find(&r.inputs, session8) else {
            // spec: drift-kit/SPEC.md §The stage-economics meter — unbounded history makes a
            // per-stamp skip notice unbounded output, so unmatched stamps are counted, not listed.
            unmatched += 1;
            continue;
        };
        attributed.insert(transcript.clone());
        // spec: drift-kit/SPEC.md §The stage-economics meter — a stage anchors on the stamp
        // resolving, not on a row being emitted: a stamped stage whose transcript carries no usage
        // is still a real, placeable (iteration, stage) for its subtree.
        anchors.insert_new(
            &transcript,
            Anchor {
                label: stage.clone(),
                iters: vec![iter.clone()],
                counts: vec![1],
                who: session8.clone(),
            },
        );
        // spec: drift-kit/SPEC.md §The stage-economics meter — a nested-tier transcript names its
        // supervising lead in its own path, which is what makes the supervision row derivable with
        // no stamp and no lifecycle change.
        if let Some((head_path, _)) = transcript.rsplit_once("/subagents/") {
            let lead = head_path.rsplit('/').next().unwrap_or(head_path).to_string();
            *dispatch.entry(format!("{} {}", lead, iter)).or_insert(0) += 1;
            if !leads.contains(&lead) {
                leads.push(lead);
            }
        }
        let usage = r.usage(&transcript);
        if usage.is_empty() {
            r.say(&format!(
                "  {} {} {}: no assistant-turn usage found (skipped)",
                iter, stage, session8
            ));
            continue;
        }
        for (model, t) in usage {
            r.emit_row(&iter, &stage, session8, &model, t);
        }
    }

    for session8 in &session_order {
        if let Some(y) = yielded.get(session8) {
            let (iter, stage) = &sessions.map[session8];
            r.say(&format!(
                "  {}: one session, several stamps — attributed to \"{} {}\"; yielded (no row): {}",
                session8,
                iter,
                stage,
                y.trim_end_matches("; ")
            ));
        }
    }

    supervision_pass(
        &mut r,
        &leads,
        &dispatch,
        label_collision,
        &mut attributed,
        &mut anchors,
        &mut unmatched,
    );
    let (fanout_rows, fanout_unresolved, fanout_no_meta) =
        fanout_pass(&mut r, suffix_collision, &mut attributed, &anchors);

    if fanout_no_meta {
        r.say(
            "  no dispatch-attribution records beside the transcripts — no fan-out rows this run; \
             every other row is unchanged (degraded, not failed)",
        );
    }
    if fanout_unresolved > 0 {
        r.say(&format!(
            "  {} dispatched transcript(s) resolved no anchor (absent or dangling attribution \
             record, or an over-long chain) — each stays in the unstamped bound below, never \
             guessed",
            fanout_unresolved
        ));
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — the under-count bound: the unmatched
    // counter reports stamps with no transcript and is structurally blind to the inverse, so the
    // inverse is counted too. A bound, never an attribution.
    for f in crate::sessions::every_transcript(&r.inputs) {
        if !attributed.contains(&f) {
            unstamped += 1;
        }
    }

    if unmatched > 0 {
        r.say(&format!(
            "  {} stamp(s) had no matching transcript (skipped — session transcripts age out of \
             the sessions dir)",
            unmatched
        ));
    }
    if unstamped > 0 {
        r.say(&format!(
            "  {} transcript(s) in the sessions dir match no stamp and resolved no anchor, so they \
             bill to no row — an upper bound on the unstamped-continuation under-count, not an \
             attribution",
            unstamped
        ));
    }
    if r.incomplete {
        r.say(
            "  total pricing incomplete — one or more model cost cells degraded to n/a (unpriced \
             model or absent table)",
        );
    }
    r.say(
        "  (cr=cache-read is the headline burn lever; one transcript bills to exactly one row key \
         — an (iteration, stage-or-role) pair or that pair's fan-out value — so a session bearing \
         several stamps is attributed to its last and the yielded stamps are named above, never \
         billed twice)",
    );
    if fanout_rows > 0 {
        let suffix = r.fanout_suffix.clone();
        r.say(&format!(
            "  a per-stage figure above excludes its own fan-out: the subtree is the adjacent \
             \"{}\" row, an aggregate over every dispatch shape (a fork and a typed dispatch fold \
             into one total)",
            suffix
        ));
    }
    let (log, rows) = (r.log.clone(), r.rows);
    r.say(&format!("  logged: {} ({} row(s))", log, rows));

    // spec: drift-kit/SPEC.md §The stage-economics meter — the log is touched only where a row was
    // emitted, so a run that priced nothing neither creates nor rewrites it.
    if rows > 0 {
        if let Some(dir) = std::path::Path::new(&log).parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let mut body = r.kept.join("\n");
        if !body.is_empty() {
            body.push('\n');
        }
        std::fs::write(&log, body).map_err(|e| format!("cannot write {}: {}", log, e))?;
    }
    head.push_str(&r.out);
    Ok(head)
}

// spec: drift-kit/SPEC.md §The stage-economics meter — supervision is its own row, never an
// apportionment across stages: the lead's burn carries no stamp, and folding it into stage rows
// would need an allocation key grounded in nothing measured.
#[allow(clippy::too_many_arguments)]
fn supervision_pass(
    r: &mut Run,
    leads: &[String],
    dispatch: &HashMap<String, u64>,
    label_collision: bool,
    attributed: &mut std::collections::HashSet<String>,
    anchors: &mut Ordered<Anchor>,
    unmatched: &mut usize,
) {
    if leads.is_empty() {
        return;
    }
    if label_collision {
        let label = r.supervision.clone();
        r.say(&format!(
            "  a stamp names the stage \"{}\", colliding with DRIFT_KIT_SUPERVISION_LABEL — no \
             supervision row emitted this run",
            label
        ));
        return;
    }
    let dir = crate::sessions::sessions_dir(&r.inputs);
    for lead in leads {
        let lead_path = format!("{}/{}.jsonl", dir, lead);
        if attributed.contains(&lead_path) {
            continue;
        }
        if !std::path::Path::new(&lead_path).is_file() {
            *unmatched += 1;
            continue;
        }
        // spec: drift-kit/SPEC.md §The stage-economics meter — the tie-break is explicit and never
        // a default sort: numeric descending on the dispatch count, then lexicographic ascending on
        // the iteration name, so the remainder lands on a determined iteration.
        let mut split: Vec<(u64, String)> = dispatch
            .iter()
            .filter_map(|(k, n)| {
                k.strip_prefix(&format!("{} ", lead))
                    .map(|iter| (*n, iter.to_string()))
            })
            .collect();
        split.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        let counts: Vec<u64> = split.iter().map(|(n, _)| *n).collect();
        let iters: Vec<String> = split.iter().map(|(_, i)| i.clone()).collect();
        let usage = r.usage(&lead_path);
        if usage.is_empty() {
            continue;
        }
        if iters.len() > 1 {
            let shown: Vec<String> = counts.iter().map(u64::to_string).collect();
            r.say(&format!(
                "  {} supervised {} iterations — apportioned by dispatch count ({}), remainder to \
                 {}",
                crate::sessions::normalize(lead),
                iters.len(),
                shown.join(" "),
                iters[0]
            ));
        }
        let label = r.supervision.clone();
        let who = crate::sessions::normalize(lead);
        for (model, t) in usage {
            let parts = apportion(t, &counts);
            for (i, iter) in iters.iter().enumerate() {
                r.emit_row(iter, &label, &who, &model, parts[i]);
            }
        }
        attributed.insert(lead_path.clone());
        anchors.insert_new(
            &lead_path,
            Anchor {
                label,
                iters,
                counts,
                who,
            },
        );
    }
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the four categories split by one key, and
// `split_tokens`' re-sum-to-the-whole property is why it exists rather than a per-field rounding.
fn apportion(t: Tokens, counts: &[u64]) -> Vec<Tokens> {
    let (i, o, cr, cw) = (
        split_tokens(t.input, counts),
        split_tokens(t.output, counts),
        split_tokens(t.cache_read, counts),
        split_tokens(t.cache_write, counts),
    );
    let at = |v: &Vec<u64>, n: usize| v.get(n).copied().unwrap_or(0);
    (0..counts.len().max(1))
        .map(|n| Tokens {
            input: at(&i, n),
            output: at(&o, n),
            cache_read: at(&cr, n),
            cache_write: at(&cw, n),
        })
        .collect()
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the fan-out row: the path cannot carry the
// parent edge, so it comes from the harness's sibling meta record and the walk stops at the
// **nearest** anchor, bounded by a visited set and by the transcript population.
fn fanout_pass(
    r: &mut Run,
    suffix_collision: bool,
    attributed: &mut std::collections::HashSet<String>,
    anchors: &Ordered<Anchor>,
) -> (usize, usize, bool) {
    if anchors.order.is_empty() {
        return (0, 0, false);
    }
    if suffix_collision {
        let suffix = r.fanout_suffix.clone();
        r.say(&format!(
            "  a stamp names a stage ending in \"{}\", colliding with DRIFT_KIT_FANOUT_SUFFIX — no \
             fan-out rows emitted this run",
            suffix
        ));
        return (0, 0, false);
    }
    let dir = crate::sessions::sessions_dir(&r.inputs);
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut have: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut nested = 0usize;
    for sub in crate::walk::glob_entries(&format!("{}/*/subagents", dir)) {
        let metas = crate::walk::glob_entries(&format!("{}/*.meta.json", sub));
        nested += crate::walk::glob_entries(&format!("{}/*.jsonl", sub)).len();
        if metas.is_empty() {
            continue;
        }
        for meta in metas {
            let Some(stem) = meta.strip_suffix(".meta.json") else {
                continue;
            };
            let transcript = format!("{}.jsonl", stem);
            have.insert(transcript.clone());
            let pid = std::fs::read_to_string(&meta)
                .ok()
                .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
                .and_then(|v| {
                    v.get("parentAgentId")
                        .and_then(|x| x.as_str())
                        .map(str::to_string)
                })
                .unwrap_or_default();
            // spec: drift-kit/SPEC.md §The stage-economics meter — an absent `parentAgentId` means
            // a direct child of the root session; a named agent with no transcript is counted,
            // never guessed, so the edge is left empty and the walk fails at it.
            let resolved = if pid.is_empty() {
                format!("{}.jsonl", sub.trim_end_matches("/subagents"))
            } else if std::path::Path::new(&format!("{}/agent-{}.jsonl", sub, pid)).is_file() {
                format!("{}/agent-{}.jsonl", sub, pid)
            } else if std::path::Path::new(&format!("{}/{}.jsonl", sub, pid)).is_file() {
                format!("{}/{}.jsonl", sub, pid)
            } else {
                String::new()
            };
            parent.insert(transcript, resolved);
        }
    }
    if nested > 0 && have.is_empty() {
        return (0, 0, true);
    }

    let walk_anchor = |from: &str, anchors: &Ordered<Anchor>| -> Option<String> {
        let mut cur = from.to_string();
        let mut hops = 0usize;
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        loop {
            if anchors.contains(&cur) {
                return Some(cur);
            }
            hops += 1;
            if hops > nested || !seen.insert(cur.clone()) || !have.contains(&cur) {
                return None;
            }
            match parent.get(&cur) {
                Some(p) if !p.is_empty() => cur = p.clone(),
                _ => return None,
            }
        }
    };

    let mut unresolved = 0usize;
    let mut fanout: Ordered<Tokens> = Ordered::default();
    let mut fanout_key: Vec<(String, String)> = Vec::new();
    for f in crate::walk::glob_entries(&format!("{}/*/subagents/*.jsonl", dir)) {
        if anchors.contains(&f) || attributed.contains(&f) {
            continue;
        }
        let Some(anchor) = walk_anchor(&f, anchors) else {
            unresolved += 1;
            continue;
        };
        attributed.insert(f.clone());
        for (model, t) in r.usage(&f) {
            let key = format!("{}\u{1}{}", anchor, model);
            if fanout.insert_new(&key, t) {
                fanout_key.push((anchor.clone(), model.clone()));
            } else {
                fanout.entry_mut(&key, Tokens::default()).add(t);
            }
        }
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — several anchors, one row: apportion
    // first and fold second, since the split is the anchor's property and the row is not.
    let mut rows: Ordered<(Tokens, String, usize)> = Ordered::default();
    let mut row_key: Vec<(String, String, String)> = Vec::new();
    for (anchor, model) in &fanout_key {
        let t = *fanout
            .get(&format!("{}\u{1}{}", anchor, model))
            .expect("the fan-out key was just recorded");
        let a = anchors.get(anchor).expect("an anchor key names an anchor");
        let (label, iters, who) = (a.label.clone(), a.iters.clone(), a.who.clone());
        let parts = apportion(t, &a.counts);
        for (i, iter) in iters.iter().enumerate() {
            let rk = format!("{}\u{1}{}\u{1}{}", iter, label, model);
            if rows.insert_new(&rk, (parts[i], who.clone(), 1)) {
                row_key.push((iter.clone(), label.clone(), model.clone()));
            } else {
                let e = rows.entry_mut(&rk, (Tokens::default(), who.clone(), 0));
                e.0.add(parts[i]);
                e.2 += 1;
            }
        }
    }

    let mut emitted = 0usize;
    let suffix = r.fanout_suffix.clone();
    for (iter, label, model) in &row_key {
        let rk = format!("{}\u{1}{}\u{1}{}", iter, label, model);
        let (t, who, count) = rows.get(&rk).expect("the row key was just recorded").clone();
        let who = if count == 1 {
            who
        } else {
            format!("{} anchors", count)
        };
        r.emit_row(iter, &format!("{}{}", label, suffix), &who, model, t);
        emitted += 1;
    }
    (emitted, unresolved, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §The stage-economics meter — the message-id dedup is not an
    // optimisation: a streaming transcript repeats an id with input and cache constant and output
    // growing, so summing raw lines multi-counts. The last record per id is the one kept.
    #[test]
    fn a_repeated_message_id_keeps_its_last_record_rather_than_summing() {
        let body = "{\"type\":\"assistant\",\"message\":{\"id\":\"m1\",\"model\":\"x\",\"usage\":\
                    {\"input_tokens\":10,\"output_tokens\":5,\"cache_read_input_tokens\":100,\
                    \"cache_creation_input_tokens\":20}}}\n\
                    {\"type\":\"assistant\",\"message\":{\"id\":\"m1\",\"model\":\"x\",\"usage\":\
                    {\"input_tokens\":10,\"output_tokens\":8,\"cache_read_input_tokens\":100,\
                    \"cache_creation_input_tokens\":20}}}\n\
                    {\"type\":\"assistant\",\"message\":{\"id\":\"m2\",\"model\":\"x\",\"usage\":\
                    {\"input_tokens\":4,\"output_tokens\":3,\"cache_read_input_tokens\":50,\
                    \"cache_creation_input_tokens\":10}}}\n";
        assert_eq!(
            usage_by_model(body),
            vec![(
                "x".to_string(),
                Tokens {
                    input: 14,
                    output: 11,
                    cache_read: 150,
                    cache_write: 30
                }
            )]
        );
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — a line the parser cannot read is skipped
    // rather than fatal, and a non-assistant turn or a null usage contributes nothing.
    #[test]
    fn unreadable_and_usage_less_lines_are_skipped_rather_than_fatal() {
        let body = "not json at all\n\
                    {\"type\":\"user\",\"message\":{\"content\":\"hi\"}}\n\
                    {\"type\":\"assistant\",\"message\":{\"id\":\"a\",\"usage\":null}}\n\
                    {\"type\":\"assistant\",\"message\":{\"id\":\"b\",\"model\":\"m\",\"usage\":\
                    {\"input_tokens\":1}}}\n";
        assert_eq!(
            usage_by_model(body),
            vec![(
                "m".to_string(),
                Tokens {
                    input: 1,
                    ..Tokens::default()
                }
            )]
        );
        assert!(usage_by_model("").is_empty());
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — per-model row order is deterministic
    // where awk's array iteration was unspecified; the log's dedup key is the triple, so no row's
    // identity depends on the order, but the emission's does and it is pinned here.
    #[test]
    fn per_model_rows_come_out_in_first_appearance_order() {
        let line = |id: &str, model: &str| {
            format!(
                "{{\"type\":\"assistant\",\"message\":{{\"id\":\"{}\",\"model\":\"{}\",\
                 \"usage\":{{\"input_tokens\":1}}}}}}\n",
                id, model
            )
        };
        let body = format!("{}{}{}", line("1", "beta"), line("2", "alpha"), line("3", "beta"));
        let models: Vec<String> = usage_by_model(&body).into_iter().map(|(m, _)| m).collect();
        assert_eq!(models, vec!["beta".to_string(), "alpha".to_string()]);
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — the apportionment re-sums to the whole
    // exactly, remainder to the first slot, which is the property the split exists for.
    #[test]
    fn the_split_re_sums_to_the_whole_with_the_remainder_first() {
        assert_eq!(split_tokens(10, &[1, 1, 1]), vec![4, 3, 3]);
        assert_eq!(split_tokens(10, &[1, 1, 1]).iter().sum::<u64>(), 10);
        assert_eq!(split_tokens(7, &[3, 1]), vec![6, 1]);
        assert_eq!(split_tokens(5, &[1]), vec![5]);
        assert_eq!(split_tokens(5, &[0, 0]), vec![5], "a zero split cannot divide");
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — the stamp grammar the history arm
    // filters by: exactly one space between fields, a kebab iteration, a lowercase-led stage, an
    // alphanumeric session8 and an ISO date, which is what keeps a diff header out of the union.
    #[test]
    fn the_history_grammar_admits_a_stamp_and_refuses_a_header() {
        assert_eq!(
            history_stamp("+alpha build s2 2025-01-01 abc123"),
            Some("alpha build s2 2025-01-01 abc123")
        );
        assert_eq!(history_stamp("+++ b/.workflow/WORKFLOW-STATE.txt"), None);
        assert_eq!(history_stamp("+# a comment line"), None);
        assert_eq!(history_stamp("+---"), None);
        assert_eq!(history_stamp("+alpha  build s2 2025-01-01"), None, "two spaces");
        assert_eq!(history_stamp("+Alpha build s2 2025-01-01"), None, "uppercase iteration");
        assert_eq!(history_stamp("+alpha build s2 2025-1-1"), None, "short date");
        assert_eq!(history_stamp("alpha build s2 2025-01-01"), None, "no + prefix");
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — the table's skipped rows, and the
    // term order the cost must sum in: in, out, cache-read, cache-creation.
    #[test]
    fn the_price_table_skips_its_non_rows_and_prices_in_the_shells_term_order() {
        let p = Prices::parse("# comment\nmodel\tinput\toutput\tcache_read\tcache_creation\n\n\
                               test-model\t1\t2\t3\t4\n");
        assert!(p.present);
        let t = Tokens {
            input: 14,
            output: 11,
            cache_read: 150,
            cache_write: 30,
        };
        assert_eq!(p.cell("test-model", t).as_deref(), Some("606.0000"));
        assert_eq!(p.cell("model", t), None, "the header row is not a priced model");
        assert_eq!(p.cell("unpriced", t), None);
        assert_eq!(Prices::default().cell("test-model", t), None);
    }
}
