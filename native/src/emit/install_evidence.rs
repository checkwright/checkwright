// spec: drift-kit/SPEC.md §The install-evidence projection — the public half of the
// install-observation channel: an aggregate-only table set, a pure function of the record and
// byte-stable over an unchanged one, so a consumer's byte-comparing freshness gate can hold it.
use super::file_install::{BEHAVIOURS, DAYS, DISPOSITIONS, NO_GREEN, RETAINED, VERDICTS};
use std::collections::{BTreeMap, BTreeSet};

pub const KNOBS: &[&str] = &["DRIFT_KIT_INSTALL_RECORD", "DRIFT_KIT_GATES_FILE"];

// spec: drift-kit/SPEC.md §The install-evidence projection — the aggregate row a red naming a gate
// outside the publisher's roster falls into: the name is the *adopter's* vocabulary, so it is
// counted rather than published, and counted rather than dropped.
const UNROSTERED: &str = "unrostered";

// spec: drift-kit/SPEC.md §The install-evidence projection — an empty cell in a degraded block's
// row, so the table's column count never moves with its content.
const EMPTY_CELL: &str = "·";

// spec: drift-kit/SPEC.md §The install-observation record — the record's path resolves against the
// repo root exactly as the capture arm resolves it, so writer and reader cannot diverge on it.
pub fn record_path() -> Result<(String, String), String> {
    super::file_survey::anchored("DRIFT_KIT_INSTALL_RECORD")
}

// spec: drift-kit/SPEC.md §The install-evidence projection — one parsed observation; a line the
// parser cannot read is skipped rather than fatal, the reading every sibling channel takes.
struct Event<'a> {
    date: &'a str,
    kind: &'a str,
    fields: Vec<&'a str>,
}

fn parse(body: &str) -> Vec<Event<'_>> {
    let mut out: Vec<Event<'_>> = Vec::new();
    for l in body.lines() {
        let f: Vec<&str> = l.split_whitespace().collect();
        if f.len() < 3 {
            continue;
        }
        out.push(Event {
            date: f[0],
            kind: f[1],
            fields: f[2..].to_vec(),
        });
    }
    out
}

fn of_kind<'a, 'b>(events: &'b [Event<'a>], kind: &str, arity: usize) -> Vec<&'b Event<'a>> {
    events
        .iter()
        .filter(|e| e.kind == kind && e.fields.len() == arity)
        .collect()
}

// spec: drift-kit/SPEC.md §The install-evidence projection — the lower of the two middle values at
// an even count, because a median of an even population needs a stated rule and an average would
// publish a figure no observation took.
fn median(sorted: &[i64]) -> Option<i64> {
    sorted.get((sorted.len().saturating_sub(1)) / 2).copied()
}

fn row(cells: &[String]) -> String {
    format!("| {} |\n", cells.join(" | "))
}

fn header(cols: &[&str]) -> String {
    let mut out = row(&cols.iter().map(|c| c.to_string()).collect::<Vec<_>>());
    out.push_str(&row(&vec!["---".to_string(); cols.len()]));
    out
}

// spec: drift-kit/SPEC.md §The install-evidence projection — a block degrades to one `n/a (<reason>)`
// cell and the run exits 0, this kit's fail-visible discipline.
fn degraded(cols: &[&str], reason: &str) -> String {
    let mut cells = vec![format!("n/a ({})", reason)];
    cells.extend(std::iter::repeat(EMPTY_CELL.to_string()).take(cols.len() - 1));
    row(&cells)
}

fn block(label: &str, cols: &[&str], body: String) -> String {
    format!("**{}**\n\n{}{}", label, header(cols), body)
}

fn count<T>(items: &[T], f: impl Fn(&T) -> bool) -> usize {
    items.iter().filter(|i| f(i)).count()
}

// spec: drift-kit/SPEC.md §The install-evidence projection — the publisher's own gate roster, read
// as a *classifier* and never as a validator: an adopter's red may name a gate this roster does not
// carry, and refusing it would discard the observation over a naming disagreement.
fn roster() -> Option<BTreeSet<String>> {
    let path = crate::walk::knob_scalar("DRIFT_KIT_GATES_FILE").ok()?;
    if path.is_empty() {
        return None;
    }
    let text = std::fs::read(&path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .ok()?;
    Some(
        text.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(String::from)
            .collect(),
    )
}

// spec: drift-kit/SPEC.md §The install-evidence projection — Installs: observed, how many reached a
// first green, and the min, median and max time-to-first-green over those that did.
fn installs(events: &[Event<'_>]) -> String {
    let cols = &[
        "observed",
        "reached a first green",
        "time-to-first-green in minutes (min · median · max)",
    ];
    let rows = of_kind(events, "install", 4);
    let mut greens: Vec<i64> = rows
        .iter()
        .filter(|e| e.fields[3] != NO_GREEN)
        .filter_map(|e| e.fields[3].parse::<i64>().ok())
        .collect();
    greens.sort_unstable();
    let ttfg = match (greens.first(), median(&greens), greens.last()) {
        (Some(lo), Some(mid), Some(hi)) => format!("{} · {} · {}", lo, mid, hi),
        _ => "n/a (no install reached a first green)".to_string(),
    };
    block(
        "Installs",
        cols,
        row(&[rows.len().to_string(), greens.len().to_string(), ttfg]),
    )
}

// spec: drift-kit/SPEC.md §The install-evidence projection — Reds: the total and its verdict split,
// the total being the denominator every figure below it is read against.
fn reds(events: &[Event<'_>]) -> String {
    let rows = of_kind(events, "red", 5);
    let mut cols = vec!["reds"];
    cols.extend(VERDICTS);
    let mut cells = vec![rows.len().to_string()];
    for v in VERDICTS {
        cells.push(count(&rows, |e| e.fields[2] == *v).to_string());
    }
    block("Reds hit by a non-author", &cols, row(&cells))
}

// spec: drift-kit/SPEC.md §The install-evidence projection — Per gate: one row per *rostered* gate a
// red names, sorted; every red naming a gate outside the roster falls into the `unrostered` row,
// which is what keeps an adopter's contribution counted without publishing that adopter's roster.
fn per_gate(events: &[Event<'_>]) -> String {
    let cols = &["gate", "reds", "false positives", "changed behaviour"];
    let rows = of_kind(events, "red", 5);
    if rows.is_empty() {
        return block("Per gate", cols, degraded(cols, "no red observed"));
    }
    let known = match roster() {
        Some(r) => r,
        None => {
            return block("Per gate", cols, degraded(cols, "no gate roster to classify against"))
        }
    };
    block("Per gate", cols, gate_rows(&rows, &known))
}

// spec: drift-kit/SPEC.md §The install-evidence projection — the tally, taking the roster as an
// argument so it is provable without one on disk: names sorted, the aggregate row last whatever the
// roster holds, so its position is the block's rule rather than its spelling's.
fn gate_rows(rows: &[&Event<'_>], known: &BTreeSet<String>) -> String {
    let mut groups: BTreeMap<String, Vec<&&Event<'_>>> = BTreeMap::new();
    for e in rows {
        let name = if known.contains(e.fields[1]) {
            e.fields[1].to_string()
        } else {
            UNROSTERED.to_string()
        };
        groups.entry(name).or_default().push(e);
    }
    let unrostered = groups.remove(UNROSTERED);
    let mut body = String::new();
    for (name, hits) in groups
        .into_iter()
        .chain(unrostered.map(|h| (UNROSTERED.to_string(), h)))
    {
        body.push_str(&row(&[
            name,
            hits.len().to_string(),
            count(&hits, |e| e.fields[2] == VERDICTS[1]).to_string(),
            count(&hits, |e| e.fields[4] == BEHAVIOURS[0]).to_string(),
        ]));
    }
    body
}

// spec: drift-kit/SPEC.md §The install-evidence projection — Dispositions: what the person hit by a
// red did, over the same denominator the Reds block publishes.
fn dispositions(events: &[Event<'_>]) -> String {
    let rows = of_kind(events, "red", 5);
    let mut cols = vec!["reds"];
    cols.extend(DISPOSITIONS);
    let mut cells = vec![rows.len().to_string()];
    for d in DISPOSITIONS {
        cells.push(count(&rows, |e| e.fields[3] == *d).to_string());
    }
    block("Dispositions", &cols, row(&cells))
}

// spec: drift-kit/SPEC.md §The install-evidence projection — Retention: per `<day>` and per `<kit>`,
// the horizons in ascending roster order and the kit names sorted, so two emissions over one record
// cannot differ by iteration order alone.
fn retention(events: &[Event<'_>]) -> String {
    let cols = &["horizon (days)", "kit", "check-ins", "retained"];
    let rows = of_kind(events, "checkin", 4);
    if rows.is_empty() {
        return block("Retention", cols, degraded(cols, "no check-in observed"));
    }
    let mut body = String::new();
    for d in DAYS {
        let kits: BTreeSet<&str> = rows
            .iter()
            .filter(|e| e.fields[1] == *d)
            .map(|e| e.fields[2])
            .collect();
        for k in kits {
            let hits: Vec<&&Event<'_>> = rows
                .iter()
                .filter(|e| e.fields[1] == *d && e.fields[2] == k)
                .collect();
            body.push_str(&row(&[
                d.to_string(),
                k.to_string(),
                hits.len().to_string(),
                count(&hits, |e| e.fields[3] == RETAINED[0]).to_string(),
            ]));
        }
    }
    block("Retention", cols, body)
}

// spec: drift-kit/SPEC.md §The install-evidence projection — First useful red: derived rather than
// stored, so the record and this figure cannot disagree. The population is the install lines, so an
// id carrying a red and no install line contributes no span.
fn first_useful_red(events: &[Event<'_>]) -> String {
    let cols = &[
        "installs",
        "reaching a first useful red",
        "median days from the install line",
    ];
    let seen = of_kind(events, "install", 4);
    let mut spans: Vec<i64> = Vec::new();
    let mut reached = 0usize;
    for i in &seen {
        let first = of_kind(events, "red", 5)
            .into_iter()
            .find(|r| r.fields[0] == i.fields[0] && r.fields[2] == VERDICTS[0]);
        let first = match first {
            Some(f) => f,
            None => continue,
        };
        reached += 1;
        if let (Some(a), Some(b)) = (
            super::trajectory::days_from_civil(i.date),
            super::trajectory::days_from_civil(first.date),
        ) {
            spans.push(b - a);
        }
    }
    spans.sort_unstable();
    let mid = match median(&spans) {
        Some(m) => m.to_string(),
        None => "n/a (no first useful red observed)".to_string(),
    };
    block(
        "First useful red",
        cols,
        row(&[seen.len().to_string(), reached.to_string(), mid]),
    )
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let human = args.iter().any(|a| a == "--human");
    let (record, _) = record_path()?;
    // spec: drift-kit/SPEC.md §The install-evidence projection — an absent record and a record of
    // zero observations are one reading, the counted zero, so the table set renders either way.
    let body = std::fs::read(&record)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    let events = parse(&body);

    let mut out = String::new();
    if human {
        out.push_str(
            "=== External install observations (advisory — installs observed outside this tree) ===\n",
        );
        out.push_str(
            "Aggregate only; no row is keyed to a single install (drift-kit/SPEC.md).\n\n",
        );
    }
    out.push_str(&installs(&events));
    out.push('\n');
    out.push_str(&reds(&events));
    out.push('\n');
    out.push_str(&per_gate(&events));
    out.push('\n');
    out.push_str(&dispositions(&events));
    out.push('\n');
    out.push_str(&retention(&events));
    out.push('\n');
    out.push_str(&first_useful_red(&events));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECORD: &str = "2026-01-01 install i1 linux bash5 12\n\
                          2026-01-01 install i2 macos bash3 -\n\
                          2026-01-03 red i1 check-alpha true-positive fixed changed\n\
                          2026-01-05 red i1 check-alpha false-positive bypassed unchanged\n\
                          2026-01-06 red i2 check-zeta unclear abandoned unchanged\n\
                          2026-01-09 checkin i1 7 gate-sdk yes\n\
                          2026-01-09 checkin i2 7 gate-sdk no\n";

    // spec: drift-kit/SPEC.md §The install-evidence projection — a line the parser cannot read is
    // skipped rather than fatal, and a short line is one of those.
    #[test]
    fn a_short_line_is_skipped_rather_than_read_as_an_event() {
        let events = parse("2026-01-01 install\n\n2026-01-01 install i1 linux bash5 12\n");
        assert_eq!(events.len(), 1);
        assert_eq!(of_kind(&events, "install", 4).len(), 1);
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — the min · median · max over the
    // installs that reached a green, and a `-` ttfg is excluded from that population rather than
    // counted as a zero.
    #[test]
    fn the_installs_block_counts_its_denominator_and_excludes_the_no_green_sentinel() {
        let events = parse(RECORD);
        let out = installs(&events);
        assert!(out.contains("| 2 | 1 | 12 · 12 · 12 |"), "{}", out);
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — the lower of the two middle values
    // at an even count, stated as the block's rule rather than left per reader.
    #[test]
    fn the_median_is_the_lower_of_two_middles() {
        assert_eq!(median(&[]), None);
        assert_eq!(median(&[5]), Some(5));
        assert_eq!(median(&[2, 4]), Some(2));
        assert_eq!(median(&[1, 2, 3]), Some(2));
        assert_eq!(median(&[1, 2, 3, 4]), Some(2));
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — every published figure carries its
    // denominator, so the red-derived blocks lead with the total they split.
    #[test]
    fn every_red_derived_block_leads_with_its_denominator() {
        let events = parse(RECORD);
        assert!(reds(&events).contains("| 3 | 1 | 1 | 1 |"), "{}", reds(&events));
        assert!(
            dispositions(&events).contains("| 3 | 1 | 0 | 1 | 1 |"),
            "{}",
            dispositions(&events)
        );
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — three fields never reach the
    // projection: `<id>`, `<profile>` and `<floor>`. No row is keyed to a single install.
    #[test]
    fn no_id_profile_or_floor_reaches_the_emission() {
        let events = parse(RECORD);
        let mut out = String::new();
        out.push_str(&installs(&events));
        out.push_str(&reds(&events));
        out.push_str(&dispositions(&events));
        out.push_str(&retention(&events));
        out.push_str(&first_useful_red(&events));
        for leaked in ["i1", "i2", "linux", "macos", "bash5", "bash3"] {
            assert!(
                !out.contains(leaked),
                "the projection published \"{}\":\n{}",
                leaked,
                out
            );
        }
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — Retention renders per `<day>` and
    // per `<kit>`, the horizons ascending and the kit names sorted.
    #[test]
    fn retention_renders_per_horizon_and_kit_with_its_denominator() {
        let events = parse(RECORD);
        let out = retention(&events);
        assert!(out.contains("| 7 | gate-sdk | 2 | 1 |"), "{}", out);
        assert_eq!(DAYS, &["7", "30"], "the horizons are rendered in roster order");
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — first useful red is the first
    // `red` line for an install whose verdict is `true-positive`, derived and never stored.
    #[test]
    fn the_first_useful_red_is_the_first_true_positive_and_its_span_is_dated() {
        let events = parse(RECORD);
        let out = first_useful_red(&events);
        assert!(out.contains("| 2 | 1 | 2 |"), "{}", out);
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — an empty record degrades to counted
    // zeros and `n/a (<reason>)` where a statistic has no population, never to a missing block.
    #[test]
    fn an_empty_record_renders_every_block_with_a_counted_zero() {
        let events = parse("");
        assert!(installs(&events).contains("| 0 | 0 | n/a (no install reached a first green) |"));
        assert!(reds(&events).contains("| 0 | 0 | 0 | 0 |"));
        assert!(per_gate(&events).contains("n/a (no red observed)"));
        assert!(retention(&events).contains("n/a (no check-in observed)"));
        assert!(first_useful_red(&events).contains("n/a (no first useful red observed)"));
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — Per gate reads the *behaviour*
    // field for its changed-behaviour column and the *verdict* field for its false-positive one,
    // and counts a red outside the roster into the aggregate row rather than dropping it.
    #[test]
    fn per_gate_tallies_each_column_off_its_own_field_and_counts_the_unrostered_last() {
        let events = parse(RECORD);
        let rows = of_kind(&events, "red", 5);
        let known: BTreeSet<String> = ["check-alpha".to_string()].into_iter().collect();
        assert_eq!(
            gate_rows(&rows, &known),
            "| check-alpha | 2 | 1 | 1 |\n| unrostered | 1 | 0 | 0 |\n"
        );
        assert_eq!(
            gate_rows(&rows, &BTreeSet::new()),
            "| unrostered | 3 | 1 | 1 |\n",
            "an empty roster classifies every red rather than dropping it"
        );
    }

    // spec: drift-kit/SPEC.md §The install-evidence projection — a degraded row keeps the block's
    // column count, so a table's shape never moves with its content.
    #[test]
    fn a_degraded_row_keeps_its_blocks_column_count() {
        let cols = &["a", "b", "c"];
        assert_eq!(degraded(cols, "why"), "| n/a (why) | · | · |\n");
    }
}
