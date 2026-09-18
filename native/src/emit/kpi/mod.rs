// spec: drift-kit/SPEC.md §The KPI plugin contract — the built-in members: the third resolution
// tier, reached only after the consumer's own dirs and the vendored kits' `kpis/` miss. Each
// returns the bytes its shell original printed, so the collator parses one grammar either way.
pub mod always_loaded;
pub mod amendment_age;
pub mod deferred_age;
pub mod gate_backlog;
pub mod gate_runtime;
pub mod incident_recurrence;
pub mod knowledge_friction;
pub mod overhead;
pub mod price_table_age;
pub mod prompt_friction;
pub mod queue_net_delta;
pub mod settings_local;
pub mod stage_economics_lag;
pub mod task_split;

#[cfg(not(unix))]
use crate::proc;

// spec: drift-kit/SPEC.md §The KPI plugin contract — the resolved knob set plus the two driver
// handoffs, which is what a built-in reads where a consumer plugin reads the exported environment;
// the pair is the same value at the same transition, which is the extension point's promise.
pub struct Ctx {
    pub queue_file: String,
    pub knowledge_log: String,
    pub timings_file: String,
    pub overhead_log: String,
    pub price_table: String,
    pub gates_dir: String,
    pub guard_log: String,
    pub settings: String,
    pub settings_local: String,
    pub done_section: String,
    pub deferred_section: String,
    pub icebox_section: String,
    pub kit_roots: Vec<String>,
    pub iteration_start: String,
    pub stage_economics_log: String,
    pub state_file: String,
    pub stages: Vec<String>,
}

// spec: drift-kit/SPEC.md §The KPI plugin contract — a member's whole result is the bytes it would
// have printed; `None` is the shell's non-zero exit, which the collator degrades to its
// fail-visible row rather than to silence.
pub type Member = fn(&Ctx, bool) -> Option<String>;

// spec: drift-kit/SPEC.md §Bundled KPIs — the built-in roster, keyed by the registry name a
// consumer file of the same name shadows. It is also what holds the shipped registry template in
// population once the sibling directory is gone (gate-sdk/SPEC.md §check-template-registry-parity).
pub const BUILTINS: &[(&str, Member)] = &[
    ("kpi-always-loaded", always_loaded::run),
    ("kpi-amendment-age", amendment_age::run),
    ("kpi-deferred-age", deferred_age::run),
    ("kpi-gate-backlog", gate_backlog::run),
    ("kpi-gate-runtime", gate_runtime::run),
    ("kpi-incident-recurrence", incident_recurrence::run),
    ("kpi-knowledge-friction", knowledge_friction::run),
    ("kpi-overhead", overhead::run),
    ("kpi-price-table-age", price_table_age::run),
    ("kpi-prompt-friction", prompt_friction::run),
    ("kpi-queue-net-delta", queue_net_delta::run),
    ("kpi-settings-local", settings_local::run),
    ("kpi-stage-economics-lag", stage_economics_lag::run),
    ("kpi-task-split", task_split::run),
];

// spec: drift-kit/SPEC.md §The extensibility contract — the kit and sibling directory whose
// registry these members answer, declared once: the parity gate's population predicate and the
// enforcement map's attribution are two compiled readers of that one fact.
pub const REGISTRY_KIT: &str = "drift-kit";
pub const REGISTRY_DIR: &str = "kpis";

pub fn lookup(name: &str) -> Option<Member> {
    BUILTINS.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

pub fn names() -> Vec<&'static str> {
    BUILTINS.iter().map(|(n, _)| *n).collect()
}

// spec: drift-kit/SPEC.md §The KPI plugin contract — a sibling kit's tool is found through the
// driver's `DRIFT_KIT_KIT_ROOTS` handoff, which a built-in reads as the resolved root set and a
// consumer plugin reads as exported environment; neither re-derives the roster.
pub fn sibling_tool(roots: &[String], rel: &str) -> Option<String> {
    roots
        .iter()
        .map(|k| format!("{}/{}", k, rel))
        .find(|p| std::path::Path::new(p).is_file())
}

pub fn read(path: &str) -> Option<String> {
    std::fs::read(path)
        .ok()
        .map(|b| String::from_utf8_lossy(&b).into_owned())
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the `n/a (<reason>)` degrade in the member's own value:
// a missing surface is a readable row, and `--trend` volunteers nothing rather than a null fragment.
pub fn na(section: &str, label: &str, reason: &str, trend: bool) -> Option<String> {
    if trend {
        return Some(String::new());
    }
    Some(format!("{}\t{}\tn/a ({})\n", section, label, reason))
}

pub fn now_epoch() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

// spec: drift-kit/SPEC.md §Bundled KPIs — a civil day or an ISO datetime to an epoch in-process: a
// stated zone applies as written, an unstated one is the operator's (`local_epoch`), and any other
// shape is no reading.
pub fn date_epoch(s: &str) -> Option<i64> {
    let days = civil_days(s.get(..10)?)?;
    let rest = &s[10..];
    if rest.is_empty() {
        return local_epoch(days, 0);
    }
    let (secs, zone) = clock(rest.strip_prefix(['T', 't', ' '])?)?;
    match zone {
        Some(offset) => Some(days * 86_400 + secs - offset),
        None => local_epoch(days, secs),
    }
}

fn civil_days(day: &str) -> Option<i64> {
    if !is_iso_day(day) {
        return None;
    }
    let days = super::trajectory::days_from_civil(day)?;
    (iso_day(days) == day).then_some(days)
}

pub fn iso_day(days: i64) -> String {
    let (y, m, d) = crate::hook::civil_from_days(days);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn two_digits(b: &[u8], at: usize) -> Option<i64> {
    let d = b.get(at..at + 2)?;
    d.iter()
        .all(u8::is_ascii_digit)
        .then(|| i64::from(d[0] - b'0') * 10 + i64::from(d[1] - b'0'))
}

// spec: drift-kit/SPEC.md §Bundled KPIs — `HH:MM[:SS[.frac]]` then an optional `Z` or `±HH[[:]MM]`;
// the seconds past midnight and the zone's offset east of UTC, `None` where no zone is written.
fn clock(t: &str) -> Option<(i64, Option<i64>)> {
    let b = t.as_bytes();
    let (h, m) = (two_digits(b, 0)?, two_digits(b, 3)?);
    if b[2] != b':' {
        return None;
    }
    let (mut s, mut i) = (0, 5);
    if b.get(i) == Some(&b':') {
        s = two_digits(b, i + 1)?;
        i += 3;
        if matches!(b.get(i), Some(b'.' | b',')) {
            let start = i + 1;
            i = start + b[start..].iter().take_while(|c| c.is_ascii_digit()).count();
            if i == start {
                return None;
            }
        }
    }
    if h > 23 || m > 59 || s > 60 {
        return None;
    }
    let zone = match &b[i..] {
        [] => None,
        [b'Z' | b'z'] => Some(0),
        [sign @ (b'+' | b'-'), tail @ ..] => {
            let oh = two_digits(tail, 0)?;
            let om = match tail.len() {
                2 => 0,
                4 => two_digits(tail, 2)?,
                5 if tail[2] == b':' => two_digits(tail, 3)?,
                _ => return None,
            };
            if oh > 23 || om > 59 {
                return None;
            }
            let off = oh * 3600 + om * 60;
            Some(if *sign == b'+' { off } else { -off })
        }
        _ => return None,
    };
    Some((h * 3600 + m * 60 + s, zone))
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the operator's zone, from `mktime(3)` on unix, which
// resolves the offset in force on that day rather than today's
#[cfg(unix)]
fn local_epoch(days: i64, secs: i64) -> Option<i64> {
    let (y, m, d) = crate::hook::civil_from_days(days);
    // spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — sound because
    // `tm` is plain integers, zeroed is a valid value, and `mktime` writes only the one we own
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    tm.tm_year = libc::c_int::try_from(y - 1900).ok()?;
    tm.tm_mon = m as libc::c_int - 1;
    tm.tm_mday = d as libc::c_int;
    tm.tm_hour = (secs / 3600) as libc::c_int;
    tm.tm_min = (secs % 3600 / 60) as libc::c_int;
    tm.tm_sec = (secs % 60) as libc::c_int;
    tm.tm_isdst = -1;
    let t = unsafe { libc::mktime(&mut tm) };
    (t != -1).then_some(t as i64)
}

// spec: drift-kit/SPEC.md §Bundled KPIs — a non-unix build has no zone reader in `std`, and its
// MSYS userland's `date` is GNU, so the local reading stays a subprocess there
#[cfg(not(unix))]
fn local_epoch(days: i64, secs: i64) -> Option<i64> {
    let stamp = format!(
        "{} {:02}:{:02}:{:02}",
        iso_day(days),
        secs / 3600,
        secs % 3600 / 60,
        secs % 60
    );
    let c = proc::run("date", &["-d", &stamp, "+%s"]).ok()?;
    String::from_utf8_lossy(c.stdout()?).trim().parse::<i64>().ok()
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the operator's civil today, the anchor an expiry counts
// from: the one day whose local midnights bracket now
#[cfg(unix)]
pub fn today_iso() -> String {
    let now = now_epoch();
    let utc = now.div_euclid(86_400);
    (utc - 1..=utc + 1)
        .find(|c| {
            matches!((local_epoch(*c, 0), local_epoch(c + 1, 0)),
                (Some(a), Some(b)) if a <= now && now < b)
        })
        .map(iso_day)
        .unwrap_or_default()
}

#[cfg(not(unix))]
pub fn today_iso() -> String {
    proc::run("date", &["+%F"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
        .unwrap_or_default()
}

// spec: queue-kit/SPEC.md §The queue-index arm — `days` civil days before the operator's today
pub fn days_ago(days: i64) -> Option<String> {
    Some(iso_day(civil_days(&today_iso())? - days))
}

pub fn is_iso_day(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && [0, 1, 2, 3, 5, 6, 8, 9]
            .iter()
            .all(|i| b[*i].is_ascii_digit())
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the awk section walk every queue-reading member shares:
// lines under `## <name>`, reset by the next `## ` heading whatever it names, so a section placed
// after the scanned one drops out of the input by construction.
pub fn section_lines<'a>(text: &'a str, name: &str) -> Vec<&'a str> {
    let head = format!("## {}", name);
    let mut inx = false;
    let mut out: Vec<&str> = Vec::new();
    for line in text.lines() {
        if line.starts_with("## ") {
            inx = line.trim_end() == head;
            continue;
        }
        if inx {
            out.push(line);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the roster is the registry's other half, so a
    // built-in nobody registers and a registered name nothing answers are both defects
    #[test]
    fn every_built_in_resolves_under_its_registry_name_and_nothing_else_does() {
        for n in names() {
            assert!(lookup(n).is_some(), "{} is in the roster but resolves to nothing", n);
        }
        assert!(lookup("kpi-always-loaded.sh").is_none());
        assert!(lookup("always-loaded").is_none());
        assert_eq!(BUILTINS.len(), 14, "the bundled set moved without its roster");
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — the section walk resets on any heading, which is what
    // keeps a later section's lines out; the control is a second section carrying a lookalike line
    #[test]
    fn the_section_walk_stops_at_the_next_heading_whatever_it_names() {
        let t = "## Deferred\n- alpha\n## Icebox\n- beta\n";
        assert_eq!(section_lines(t, "Deferred"), vec!["- alpha"]);
        assert_eq!(section_lines(t, "Icebox"), vec!["- beta"]);
        assert!(section_lines(t, "Done").is_empty());
    }

    #[test]
    fn an_iso_day_is_ten_characters_with_two_separators_and_eight_digits() {
        assert!(is_iso_day("2026-08-29"));
        assert!(!is_iso_day("2026-8-29"));
        assert!(!is_iso_day("2026-08-291"));
        assert!(!is_iso_day("not-a-date"));
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — a written zone is applied as written, whatever the
    // host's zone, and every shape outside the grammar is no reading
    #[test]
    fn a_datetime_with_a_zone_reads_the_same_instant_on_every_host() {
        for s in [
            "1970-01-02T00:00:00Z",
            "1970-01-02t00:00:00z",
            "1970-01-02T00:00:00.123456Z",
            "1970-01-02 01:30:00+01:30",
            "1970-01-02T01:30:00+0130",
            "1970-01-01T22:00:00-02",
            "1970-01-02T00:00Z",
        ] {
            assert_eq!(date_epoch(s), Some(86_400), "{}", s);
        }
        for s in [
            "",
            "not a time at all",
            "2026-02-31",
            "2026-13-01",
            "2026-09-18x",
            "2026-09-18T25:00:00Z",
            "2026-09-18T12:00:00.Z",
            "2026-09-18T12:00:00+1",
            "2026-09-18T12:00:00 UTC",
        ] {
            assert_eq!(date_epoch(s), None, "{:?}", s);
        }
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — an unwritten zone is the operator's: a bare day is its
    // local midnight, and today is the day whose local midnights bracket now
    #[test]
    fn an_unzoned_reading_is_the_local_one_and_today_brackets_now() {
        assert_eq!(date_epoch("2026-09-18"), date_epoch("2026-09-18T00:00:00"));
        let today = today_iso();
        let start = date_epoch(&today).expect("today has a local midnight");
        let now = now_epoch();
        assert!(start <= now && now - start < 90_000, "{} does not bracket now", today);
        assert_eq!(days_ago(0).as_deref(), Some(today.as_str()));
        let yesterday = days_ago(1).expect("yesterday");
        assert!(yesterday < today, "{} is not before {}", yesterday, today);
    }
}
