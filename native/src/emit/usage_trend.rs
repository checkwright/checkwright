// spec: delegation-kit/SPEC.md §Trend reporter — footprint evolution from the usage-history log:
// one axis record per sample, segmented by the axis's reset epoch, smoothed against the window's
// one physical constraint, and reported per account.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Emit` because the declared contract is 0
// report emitted / 2 fail-closed and never 1, which is that family verbatim; the *absence* of a 1
// settles it here where its sibling `--usage-verdict`'s presence of one settled the other way.
use crate::walk;

// spec: delegation-kit/SPEC.md §Layout and configuration — the two names this arm resolves, both
// defined and defaulted in the shell library the bridge sources: a hardcoded default would work in
// this tree and break silently for a consumer that overrides either.
pub const KNOBS: &[&str] = &[
    "DELEGATION_KIT_USAGE_HISTORY",
    "DELEGATION_KIT_PAUSE_PCT_7D",
];

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the usage a shape refusal prints, the
// `-h`/`--help` half having retired to the front-end. It names the `--` escape because the refusal
// it accompanies is the only reason a caller would reach for one.
const USAGE: &str = "usage: run-gates.sh --emit usage-trend [--] [history-file]
  [history-file] overrides DELEGATION_KIT_USAGE_HISTORY, for test injection.
  \"--\" ends option processing, so a path beginning with \"-\" is still reachable.";

const HISTORY_KNOB: &str = "DELEGATION_KIT_USAGE_HISTORY";
const PAUSE_KNOB: &str = "DELEGATION_KIT_PAUSE_PCT_7D";

// spec: delegation-kit/SPEC.md §The usage.txt contract — the reader's half of the producer's
// omit-don't-empty rule: an absent optional key reads as its default rather than as a parse
// failure, because the producer is entitled to write a line without it.
const ABSENT: &str = "-";
const ABSENT_LOGIN: &str = "0";

// spec: delegation-kit/SPEC.md §Trend reporter — one axis record per sample, every key held as the
// producer spelled it: the segment tuple and the printed epochs are text upstream, and only the
// sort's three numeric keys and the arithmetic read a number out of one.
struct Rec {
    axis: &'static str,
    acct: String,
    tier: String,
    login: String,
    reset: String,
    updated: String,
    pct: f64,
    verdict: String,
    tin: String,
    tout: String,
    line: String,
}

// spec: delegation-kit/SPEC.md §Trend reporter — GNU `sort -n` reads a leading numeric prefix and
// yields zero for a value carrying none, so a malformed field orders as zero rather than aborting
// the run; the parse stage upstream already defaults `login_at`, so this is the residual case.
fn num(s: &str) -> f64 {
    let t = s.trim();
    let mut end = 0;
    for (i, c) in t.char_indices() {
        let head = i == 0;
        if c.is_ascii_digit() || (head && (c == '-' || c == '+')) || (!head && c == '.') {
            end = i + c.len_utf8();
        } else {
            break;
        }
    }
    t[..end].parse::<f64>().unwrap_or(0.0)
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — the wire line is space-separated
// `key=value` tokens split on the *first* `=`, so a value carrying one survives; a token with no
// `=` at all is not a key and is dropped, which is what lets a comment line yield no record.
fn tokens(line: &str) -> Vec<(&str, &str)> {
    line.split_whitespace()
        .filter_map(|t| t.find('=').map(|i| (&t[..i], &t[i + 1..])))
        .collect()
}

fn get<'a>(kv: &[(&'a str, &'a str)], key: &str) -> Option<&'a str> {
    kv.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

fn or<'a>(kv: &[(&'a str, &'a str)], key: &str, absent: &'a str) -> &'a str {
    get(kv, key).unwrap_or(absent)
}

fn record(axis: &'static str, kv: &[(&str, &str)], pct: &str, reset: &str, updated: &str) -> Rec {
    let acct = or(kv, "account", ABSENT).to_string();
    let tier = or(kv, "tier", ABSENT).to_string();
    let login = or(kv, "login_at", ABSENT_LOGIN).to_string();
    let verdict = or(kv, "verdict", ABSENT).to_string();
    let tin = or(kv, "tokens_in", ABSENT).to_string();
    let tout = or(kv, "tokens_out", ABSENT).to_string();
    let line = [
        axis, &acct, &tier, &login, reset, updated, pct, &verdict, &tin, &tout,
    ]
    .join("\t");
    Rec {
        axis,
        acct,
        tier,
        login,
        reset: reset.to_string(),
        updated: updated.to_string(),
        pct: num(pct),
        verdict,
        tin,
        tout,
        line,
    }
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — the two-axis emission: a weekly record
// only when *both* weekly keys ride, tested on the key's presence rather than its value, because
// the producer omits an optional key rather than writing it empty.
fn parse(body: &str) -> Vec<Rec> {
    let mut out = Vec::new();
    for line in body.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let kv = tokens(line);
        let updated = match get(&kv, "updated_at") {
            Some(u) => u,
            None => continue,
        };
        if let (Some(pct), Some(reset)) = (get(&kv, "pct"), get(&kv, "resets_at")) {
            out.push(record("5h", &kv, pct, reset, updated));
        }
        if let (Some(pct), Some(reset)) = (get(&kv, "pct_7d"), get(&kv, "resets_7d")) {
            out.push(record("7d", &kv, pct, reset, updated));
        }
    }
    out
}

// spec: delegation-kit/SPEC.md §Trend reporter — the segment order is a contract, so the last
// resort is the whole record: that is GNU `sort`'s own tie rule absent `-s`, and a merely stable
// sort would keep input order and pick different segment endpoints.
fn sort(recs: &mut [Rec]) {
    recs.sort_by(|a, b| {
        a.acct
            .cmp(&b.acct)
            .then_with(|| a.axis.cmp(b.axis))
            .then_with(|| a.tier.cmp(&b.tier))
            .then_with(|| num(&a.login).total_cmp(&num(&b.login)))
            .then_with(|| num(&a.reset).total_cmp(&num(&b.reset)))
            .then_with(|| num(&a.updated).total_cmp(&num(&b.updated)))
            .then_with(|| a.line.cmp(&b.line))
    });
}

fn med3(a: f64, b: f64, c: f64) -> f64 {
    let (mut x, mut y, mut z) = (a, b, c);
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    if y > z {
        std::mem::swap(&mut y, &mut z);
    }
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    y
}

// spec: delegation-kit/SPEC.md §Trend reporter — median-of-3 smoothing resolves single-sample
// spikes and segment endpoints keep their own value, so no 2-window average is taken at either end
// and a one-sample segment is its own reading.
fn smoothed(rows: &[&Rec]) -> Vec<f64> {
    let n = rows.len();
    (0..n)
        .map(|i| {
            if n == 1 || i == 0 || i == n - 1 {
                rows[i].pct
            } else {
                med3(rows[i - 1].pct, rows[i].pct, rows[i + 1].pct)
            }
        })
        .collect()
}

// spec: delegation-kit/SPEC.md §Trend reporter — a downward correction indicts *both* sides: the
// low sample and every earlier one above it, so the elevated readings a correction reveals are
// excluded from rate math rather than averaged in.
fn suspect(rows: &[&Rec]) -> Vec<bool> {
    let n = rows.len();
    let mut susp = vec![false; n];
    let mut runmax = rows[0].pct;
    for i in 1..n {
        let p = rows[i].pct;
        if p < runmax {
            susp[i] = true;
            for j in 0..i {
                if rows[j].pct > p {
                    susp[j] = true;
                }
            }
        }
        if p > runmax {
            runmax = p;
        }
    }
    susp
}

// spec: delegation-kit/SPEC.md §Trend reporter — the per-segment report. `last_acct` carries
// across segments because the account heading prints on change alone, which is what groups a
// rotating operator's weekly trajectory instead of interleaving it.
fn flush(rows: &[&Rec], pause7: &str, last_acct: &mut Option<String>, out: &mut String) {
    let n = rows.len();
    if n == 0 {
        return;
    }
    let sp = smoothed(rows);
    let susp = suspect(rows);
    let nsusp = susp.iter().filter(|s| **s).count();
    let live: Vec<usize> = (0..n).filter(|i| !susp[*i]).collect();
    // spec: delegation-kit/SPEC.md §Trend reporter — every sample suspect falls back to the raw
    // span so the segment still reports rather than vanishing; the suspect count beside it is what
    // tells a reader the numbers came from an unreliable producer.
    let (fi, li) = match (live.first(), live.last()) {
        (Some(f), Some(l)) => (*f, *l),
        _ => (0, n - 1),
    };
    let (first, last) = (sp[fi], sp[li]);
    let hours = (num(&rows[li].updated) - num(&rows[fi].updated)) / 3600.0;
    let (rate, ratefmt) = if hours > 0.0 {
        let r = (last - first) / hours;
        (r, format!("{:+.2}%/h", r))
    } else {
        (0.0, "n/a (single reading)".to_string())
    };

    let acct = &rows[0].acct;
    if last_acct.as_deref() != Some(acct.as_str()) {
        if acct == ABSENT {
            out.push_str("\naccount: (unstamped)\n");
        } else {
            out.push_str(&format!("\naccount {}\n", acct));
        }
        *last_acct = Some(acct.clone());
    }
    out.push_str(&format!(
        "  [{}] reset@{} tier={}: {:.1}%->{:.1}% over {:.2}h, {}, {} sample(s), {} suspect\n",
        rows[0].axis, rows[0].reset, rows[0].tier, first, last, hours, ratefmt, n, nsusp
    ));
    // spec: delegation-kit/SPEC.md §The usage.txt contract — the token axis is read only when both
    // endpoints carry it, the omit-don't-empty rule meaning a mixed segment has no delta to report.
    if rows[li].tin != ABSENT && rows[fi].tin != ABSENT {
        out.push_str(&format!(
            "      tokens: +{} in / +{} out over the segment\n",
            (num(&rows[li].tin) - num(&rows[fi].tin)) as i64,
            (num(&rows[li].tout) - num(&rows[fi].tout)) as i64
        ));
    }
    // spec: delegation-kit/SPEC.md §Trend reporter — the weekly planning number, printed against
    // the configured ceiling verbatim so a consumer reads back the threshold it set.
    if rows[0].axis == "7d" {
        let hdr = num(pause7) - last;
        if rate > 0.0 {
            out.push_str(&format!(
                "      weekly headroom: {:.1}% to the {}% ceiling (~{:.1}h at current rate)\n",
                hdr,
                pause7,
                hdr / rate
            ));
        } else {
            out.push_str(&format!(
                "      weekly headroom: {:.1}% to the {}% ceiling (rate flat/negative — no depletion trend)\n",
                hdr, pause7
            ));
        }
    }
    if let Some(r) = rows.iter().find(|r| r.verdict == "PAUSE") {
        out.push_str(&format!("      first PAUSE onset at epoch {}\n", r.updated));
    }
}

// spec: delegation-kit/SPEC.md §Trend reporter — the segment tuple, compared as the upstream
// sort's own keys are spelled: as text. Two epochs that differ only in spelling are two segments
// there, so reading a number out of them here would silently merge a boundary.
fn segment_key(r: &Rec) -> (&str, &str, &str, &str, &str) {
    (r.axis, &r.acct, &r.tier, &r.login, &r.reset)
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the shape half, an *addition* this port makes
// rather than a behaviour it preserves; the scan is borrowed from the class's first free-text
// member rather than re-spelled, so the escape has one implementation.
fn positionals(args: &[String]) -> Result<&[String], String> {
    super::file_survey::positionals(args, "history path").map_err(|e| format!("{}\n{}", e, USAGE))
}

// spec: delegation-kit/SPEC.md §Trend reporter — the positional overrides the bridged knob and is
// read first, `${1:-…}`'s own laziness; the knob's default being empty, an absent bridge variable
// and a configured-empty one are one reading and take the tool's own diagnostic.
fn history(args: &[String]) -> Result<String, String> {
    let rest = positionals(args)?;
    match rest.first() {
        Some(p) if !p.is_empty() => Ok(p.clone()),
        _ => Ok(walk::knob_scalar(HISTORY_KNOB).unwrap_or_default()),
    }
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let history = history(args)?;
    if history.is_empty() {
        return Err(format!(
            "{} unset — no history to report (enable sampling first)",
            HISTORY_KNOB
        ));
    }
    let body =
        std::fs::read_to_string(&history).map_err(|_| format!("cannot read history {}", history))?;
    let pause7 = walk::knob_scalar(PAUSE_KNOB)?;

    let mut recs = parse(&body);
    if recs.is_empty() {
        return Ok(format!(
            "usage-trend: no parseable samples in {} (0 segments)\n",
            history
        ));
    }
    sort(&mut recs);

    let mut report = String::new();
    let mut last_acct: Option<String> = None;
    let mut seg: Vec<&Rec> = Vec::new();
    for r in &recs {
        if !seg.is_empty() && segment_key(seg[0]) != segment_key(r) {
            flush(&seg, &pause7, &mut last_acct, &mut report);
            seg.clear();
        }
        seg.push(r);
    }
    flush(&seg, &pause7, &mut last_acct, &mut report);

    Ok(format!(
        "usage-trend: {} axis-record(s) across the 5h/weekly segments\n{}\n",
        recs.len(),
        report.trim_end_matches('\n')
    ))
}
