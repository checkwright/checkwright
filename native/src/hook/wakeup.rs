// spec: guard-kit/SPEC.md §wakeup-guard — the PreToolUse(ScheduleWakeup|CronCreate) member: it
// denies fail-closed whatever the payload said, and logs the attempt for close's triage. It emits
// no envelope and speaks the protocol through its exit status alone.
use crate::hook;
use crate::walk;
use serde_json::Value;

const NAME: &str = "wakeup-guard";
const LOG_KNOB: &str = "GUARD_KIT_WAKEUP_LOG";

pub fn run(payload: Option<&Value>) -> i32 {
    // spec: guard-kit/SPEC.md §wakeup-guard — the deny stands even where the record is lost: the
    // block is unconditional, so a knob fault or a failed append costs the record and not the rule,
    // and the denial names what cost it
    hook::block(NAME, &record_and_deny(payload))
}

fn record_and_deny(payload: Option<&Value>) -> String {
    let logged = record_target().and_then(|log| append_attempt(&log, payload));
    denial(logged.err().as_deref())
}

// spec: guard-kit/SPEC.md §wakeup-guard — the log the record goes to: the carried `--knobs` fault
// first, then the knob, absolute under the tree-free mark
pub(crate) fn record_target() -> Result<String, String> {
    if let Some(fault) = hook::knob_fault() {
        return Err(fault);
    }
    let log = walk::knob_scalar(LOG_KNOB)?;
    hook::tree_free_path(LOG_KNOB, &log)?;
    Ok(walk::capture_path(&log))
}

// spec: guard-kit/SPEC.md §wakeup-guard — with no repository the denial names neither a repository
// nor close's triage, and a lost record's fault replaces the logged sentence, so it is never silent
fn denial(unlogged: Option<&str>) -> String {
    let tree_free = hook::tree_free();
    let place = if tree_free { "" } else { " in this repo" };
    let tail = match unlogged {
        Some(fault) => format!("This attempt went unlogged: {}.", fault),
        None if tree_free => "Attempt logged.".to_string(),
        None => "Attempt logged (triaged at close alongside the friction log).".to_string(),
    };
    format!("ScheduleWakeup/CronCreate is blocked{} — {} {}", place, WHY, tail)
}

// spec: guard-kit/SPEC.md §wakeup-guard — the attempt record: a local-zone stamp, then the three
// payload fields that identify the attempt; a log it cannot open or write is the fault returned
fn append_attempt(log: &str, payload: Option<&Value>) -> Result<(), String> {
    use std::io::Write;
    let line = format!("{} {}\n", local_stamp(), summary(payload));
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log)
        .and_then(|mut f| f.write_all(line.as_bytes()))
        .map_err(|e| format!("cannot write {}: {}", log, e))
}

// spec: guard-kit/SPEC.md §wakeup-guard — GNU `date -Is`'s shape, built in-process from the offset
// the system reports; an unreadable clock or zone costs the stamp alone, exactly as the shell
// form's discarded stderr did.
fn local_stamp() -> String {
    let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) else {
        return String::new();
    };
    let epoch = now.as_secs() as i64;
    zone_offset(epoch).map_or_else(String::new, |offset| stamp_at(epoch, offset))
}

#[cfg(unix)]
fn zone_offset(epoch: i64) -> Option<i64> {
    let t = epoch as _;
    // spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — sound because
    // `tm` is plain integers, zeroed is a valid value, and `localtime_r` writes only the one we own
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    if unsafe { libc::localtime_r(&t, &mut tm) }.is_null() {
        return None;
    }
    Some(tm.tm_gmtoff as i64)
}

// spec: guard-kit/SPEC.md §wakeup-guard — the bias in force, in minutes west of UTC: the zone's
// base plus its daylight or standard delta, whichever the system reports current
#[cfg(windows)]
fn zone_offset(_epoch: i64) -> Option<i64> {
    use crate::proc::kernel32;
    // spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — sound because
    // the struct is plain integers, zeroed is a valid value, and the call writes only the one we own
    let mut tz: kernel32::TimeZoneInformation = unsafe { std::mem::zeroed() };
    let delta = match unsafe { kernel32::GetTimeZoneInformation(&mut tz) } {
        kernel32::TIME_ZONE_ID_INVALID => return None,
        kernel32::TIME_ZONE_ID_DAYLIGHT => tz.daylight_bias,
        kernel32::TIME_ZONE_ID_STANDARD => tz.standard_bias,
        _ => 0,
    };
    Some(-60 * (i64::from(tz.bias) + i64::from(delta)))
}

// spec: guard-kit/SPEC.md §wakeup-guard — `YYYY-MM-DDTHH:MM:SS±HH:MM`, byte-compatible with GNU
// `date -Is`, a zero offset included as `+00:00`
fn stamp_at(epoch: i64, offset: i64) -> String {
    let local = epoch + offset;
    let (y, m, d) = hook::civil_from_days(local.div_euclid(86_400));
    let secs = local.rem_euclid(86_400);
    let sign = if offset < 0 { '-' } else { '+' };
    let off = offset.abs();
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}{:02}:{:02}",
        y,
        m,
        d,
        secs / 3600,
        secs % 3600 / 60,
        secs % 60,
        sign,
        off / 3600,
        off % 3600 / 60
    )
}

// spec: guard-kit/SPEC.md §wakeup-guard — `jq -c '{session_id, tool_name, tool_input}'`, whose
// object literal fixes the key order and emits `null` for a field the payload does not carry; a
// payload that did not parse is logged whole, which here is the empty body the reader received.
fn summary(payload: Option<&Value>) -> String {
    let Some(doc) = payload else {
        return String::new();
    };
    use crate::hook::binding::{SESSION_ID, TOOL_INPUT, TOOL_NAME};
    let at = |k: &str| format!("{}:{}", hook::quote(k), doc.get(k).cloned().unwrap_or(Value::Null));
    format!("{{{},{},{}}}", at(SESSION_ID), at(TOOL_NAME), at(TOOL_INPUT))
}

// spec: guard-kit/SPEC.md §wakeup-guard — why a self-scheduled wakeup is refused, carried off the
// shell member
const WHY: &str = "a self-scheduled wakeup re-fires its stored prompt in a later session as if the user typed it, long after its premises are stale, and the scheduling call is invisible at the moment it matters. Surface the intent to the user and let them re-invoke instead.";

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §wakeup-guard — the record's field order and its null-for-absent are
    // jq's object literal, so a reader of the existing log meets the shape it already parses
    #[test]
    fn the_attempt_record_keeps_jqs_field_order_and_nulls() {
        let doc: Value = serde_json::from_str(r#"{"tool_name":"ScheduleWakeup","extra":1}"#)
            .expect("the fixture must parse");
        assert_eq!(
            summary(Some(&doc)),
            r#"{"session_id":null,"tool_name":"ScheduleWakeup","tool_input":null}"#
        );
        assert_eq!(summary(None), "");
    }

    // spec: guard-kit/SPEC.md §wakeup-guard — with no repository a relative log, the default
    // included, and a carried `--knobs` fault each cost the record, never the denial, which names it
    #[test]
    fn with_no_repository_a_lost_record_still_denies_naming_its_fault() {
        let m = hook::Marked::new("wakeup", None);
        let relative = record_target().expect_err("the derived default is relative");
        assert!(relative.contains(LOG_KNOB) && relative.contains("absolute paths only"), "{}", relative);
        assert_eq!(run(None), 2);
        m.carry("--knobs names a relative path: rel.knobs, and takes an absolute file only");
        let carried = record_target().expect_err("the carried fault comes first");
        assert!(carried.starts_with("--knobs") && carried.contains("rel.knobs"), "{}", carried);
        assert_eq!(run(None), 2);
        let text = denial(Some(&carried));
        assert!(text.contains(WHY) && text.ends_with("takes an absolute file only."), "{}", text);
        for held in [denial(Some(&carried)), denial(None)] {
            assert!(!held.contains("in this repo") && !held.contains("triaged at close"), "{}", held);
        }
        drop(m);
    }

    // spec: guard-kit/SPEC.md §wakeup-guard — in a repository the denial keeps its repository and
    // triage wording
    #[test]
    fn in_a_repository_the_denial_names_the_repo_and_the_triage() {
        let text = denial(None);
        assert!(text.contains("blocked in this repo") && text.ends_with("alongside the friction log)."), "{}", text);
    }

    // spec: guard-kit/SPEC.md §wakeup-guard — an absolute log under the mark is the record's target
    #[test]
    fn with_no_repository_an_absolute_log_is_the_target() {
        let log = std::env::temp_dir().join("wakeup-attempts.log").display().to_string();
        let _m = hook::Marked::new("wakeup-abs", Some(&format!("{} = {}\n", LOG_KNOB, log)));
        assert_eq!(record_target(), Ok(log));
    }

    // spec: guard-kit/SPEC.md §wakeup-guard — a log the append cannot open costs the record, and the
    // denial names the file and the fault in place of the logged sentence
    #[test]
    fn an_unwritable_log_is_denied_as_unlogged() {
        let dir = std::env::temp_dir().join(format!("wakeup-unwritable-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let log = dir.join("absent").join("wakeup-attempts.log").display().to_string();
        let _m = hook::Marked::new("wakeup-unwritable", Some(&format!("{} = {}\n", LOG_KNOB, log)));
        let text = record_and_deny(None);
        assert!(text.contains("This attempt went unlogged: cannot write") && text.contains(&log), "{}", text);
        assert!(!text.contains("Attempt logged"), "{}", text);
        std::fs::create_dir_all(dir.join("absent")).expect("cannot make the log dir");
        assert!(record_and_deny(None).ends_with("Attempt logged."), "a written record was reported lost");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: guard-kit/SPEC.md §wakeup-guard — the stamp is GNU `date -Is`'s bytes at a fixed epoch
    // and offset, east, west and zero
    #[test]
    fn the_stamp_is_gnu_date_is_shaped() {
        assert_eq!(stamp_at(1_789_776_000, 7_200), "2026-09-19T02:00:00+02:00");
        assert_eq!(stamp_at(1_789_776_000, -16_200), "2026-09-18T19:30:00-04:30");
        assert_eq!(stamp_at(1_789_776_000, 0), "2026-09-19T00:00:00+00:00");
    }

    // spec: guard-kit/SPEC.md §wakeup-guard — the live stamp spawns nothing, so a host whose `PATH`
    // carries no userland still gets one, and it reads back as this instant; `crate-tests-windows`
    // holds the off-unix reader
    #[test]
    fn the_live_stamp_spawns_nothing_and_reads_back_as_now() {
        crate::proc::recorder::start();
        let stamp = local_stamp();
        let spawned = crate::proc::recorder::stop();
        assert!(spawned.is_empty(), "the stamp spawned {:?}", spawned);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("the clock must read")
            .as_secs() as i64;
        assert_eq!(stamp.len(), 25, "stamp {:?}", stamp);
        let num = |r: std::ops::Range<usize>| -> i64 {
            stamp[r].parse().unwrap_or_else(|_| panic!("stamp {:?}", stamp))
        };
        let sign = match &stamp[19..20] {
            "+" => 1,
            "-" => -1,
            _ => panic!("stamp {:?}", stamp),
        };
        let offset = sign * (num(20..22) * 3600 + num(23..25) * 60);
        assert!(offset.abs() <= 14 * 3600, "stamp {:?}", stamp);
        assert!(
            (now - 5..=now + 1).any(|e| stamp_at(e, offset) == stamp),
            "stamp {:?} does not read back as epoch {}",
            stamp,
            now
        );
    }
}
