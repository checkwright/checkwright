// spec: delegation-kit/SPEC.md §The usage.txt contract — the snapshot both non-hook
// harness-integration members write: one `key=value` record per line, rewritten atomically through
// a sibling `.tmp` so a reader never meets a half-written file.
use crate::emit::kpi;
use serde_json::Value;

// spec: delegation-kit/SPEC.md §The usage.txt contract — the two required fields lead and
// `updated_at` follows them, because a reader grades the record's staleness by it; the four
// optional fields are omitted rather than written empty.
pub struct Snapshot {
    pub five_hour_used_pct: String,
    pub five_hour_resets_at: String,
    pub seven_day_used_pct: String,
    pub seven_day_resets_at: String,
    pub account: String,
    pub tier: String,
}

impl Snapshot {
    pub fn render(&self) -> String {
        let mut out = format!(
            "five_hour_used_pct={}\nfive_hour_resets_at={}\nupdated_at={}\n",
            self.five_hour_used_pct,
            self.five_hour_resets_at,
            kpi::now_epoch()
        );
        for (key, value) in [
            ("seven_day_used_pct", &self.seven_day_used_pct),
            ("seven_day_resets_at", &self.seven_day_resets_at),
            ("account", &self.account),
            ("tier", &self.tier),
        ] {
            if !value.is_empty() {
                out.push_str(&format!("{}={}\n", key, value));
            }
        }
        out
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — write-then-rename: the snapshot is
    // read by a second process, so a partial write must never be observable. A failed write leaves
    // the previous snapshot untouched, which the reader grades as STALE rather than as green.
    pub fn write(&self, usage_file: &str) -> bool {
        crate::install::write_atomically(std::path::Path::new(usage_file), self.render().as_bytes()).is_ok()
    }
}

// spec: delegation-kit/SPEC.md §Layout and configuration — the three harness paths, each empty knob
// derived here: the usage file under the harness's config home, the credentials file beside it, and
// the account config under the home directory
#[derive(Default)]
pub struct Paths {
    pub usage_file: String,
    pub cred_file: String,
    pub account_config: String,
}

pub fn paths() -> Result<Paths, String> {
    let k = crate::walk::knob_scalar;
    let var = |n: &str| std::env::var(n).unwrap_or_default();
    let home = var(crate::sessions::HOME_VAR);
    let mut p = Paths {
        usage_file: k("DELEGATION_KIT_USAGE_FILE")?,
        cred_file: k("DELEGATION_KIT_CRED_FILE")?,
        account_config: k("DELEGATION_KIT_ACCOUNT_CONFIG")?,
    };
    if p.usage_file.is_empty() {
        p.usage_file = format!("{}/usage.txt", crate::sessions::config_home(&var(crate::hook::binding::CONFIG_DIR_VAR), &home));
    }
    if p.cred_file.is_empty() {
        p.cred_file = cred_beside(&p.usage_file);
    }
    if p.account_config.is_empty() {
        p.account_config = format!("{}/.claude.json", home);
    }
    Ok(p)
}

// spec: delegation-kit/SPEC.md §Layout and configuration — "in that file's directory" is split on the
// host's separators, so a backslash-only Windows usage path keeps its directory
fn cred_beside(usage_file: &str) -> String {
    let dir = usage_file.rsplit_once(std::path::is_separator).map_or(usage_file, |(d, _)| d);
    format!("{}/.credentials.json", dir)
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — one string field of a JSON file by object
// path, empty where the file is unreadable, unparseable or carries no such field; `jq -r '<path>
// // empty' <file> 2>/dev/null`, which is how both members read the credential and account files.
pub fn json_field(file: &str, path: &[&str]) -> String {
    let Ok(text) = std::fs::read_to_string(file) else {
        return String::new();
    };
    let Ok(doc) = serde_json::from_str::<Value>(&text) else {
        return String::new();
    };
    let mut cur = &doc;
    for step in path {
        match cur.get(step) {
            Some(next) => cur = next,
            None => return String::new(),
        }
    }
    cur.as_str().unwrap_or("").to_string()
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — the reset stamp normalised to an epoch: an
// all-digit value is already one, anything else is drift-kit's ISO datetime reading, a bare calendar
// day, which that reading takes at local midnight, being no datetime
pub fn epoch_of(v: &str) -> String {
    if v.is_empty() {
        return String::new();
    }
    if v.bytes().all(|b| b.is_ascii_digit()) {
        return v.to_string();
    }
    if v.get(10..).map_or(true, str::is_empty) {
        return String::new();
    }
    kpi::date_epoch(v).map(|e| e.to_string()).unwrap_or_default()
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — a utilization reading is digits with at
// most one dot between digits; anything else is no reading
pub fn is_utilization(v: &str) -> bool {
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    match v.split_once('.') {
        Some((whole, frac)) => digits(whole) && digits(frac),
        None => digits(v),
    }
}

#[derive(Debug, PartialEq)]
pub enum Half {
    Utilization,
    Reset,
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — one window's pair held to its shape: both
// empty is no window, a utilization beside a reset epoch is the pair with the reset normalised,
// and anything else is the half or malformed pair a producer writes nothing for
pub fn window(pct: &str, resets: &str) -> Result<Option<(String, String)>, Half> {
    if pct.is_empty() && resets.is_empty() {
        return Ok(None);
    }
    if !is_utilization(pct) {
        return Err(Half::Utilization);
    }
    match epoch_of(resets) {
        e if e.is_empty() => Err(Half::Reset),
        e => Ok(Some((pct.to_string(), e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: delegation-kit/SPEC.md §Layout and configuration — the credentials file lands in the
    // usage file's directory; `crate-tests-windows` holds the backslash-only spelling
    #[test]
    fn the_credentials_file_sits_in_the_usage_files_directory() {
        assert_eq!(cred_beside("/h/.claude/usage.txt"), "/h/.claude/.credentials.json");
        #[cfg(windows)]
        assert_eq!(cred_beside(r"C:\h\.claude\usage.txt"), r"C:\h\.claude/.credentials.json");
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — the two required fields always appear
    // and an absent optional one is omitted rather than written empty, which is what lets a reader
    // tell "no seven-day axis" from "a seven-day axis reading zero"
    #[test]
    fn an_absent_optional_field_is_omitted_not_emptied() {
        let s = Snapshot {
            five_hour_used_pct: "12".to_string(),
            five_hour_resets_at: "1756656000".to_string(),
            seven_day_used_pct: String::new(),
            seven_day_resets_at: String::new(),
            account: String::new(),
            tier: "max".to_string(),
        };
        let text = s.render();
        assert!(text.starts_with("five_hour_used_pct=12\nfive_hour_resets_at=1756656000\n"));
        assert!(text.contains("\nupdated_at="));
        assert!(text.trim_end().ends_with("\ntier=max"));
        assert!(!text.contains("seven_day"), "an absent axis must not be written");
        assert!(!text.contains("account="), "an absent account must not be written");
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — the poll and the statusline producer
    // write the snapshot unserialized, so each writer's own temporary keeps every write whole
    #[test]
    fn two_snapshot_writers_each_publish_a_whole_snapshot() {
        let dir = std::env::temp_dir().join(format!("cw-usage-race-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        let usage = dir.join("usage.txt").to_string_lossy().into_owned();
        let snap = |pct: &str| Snapshot {
            five_hour_used_pct: pct.to_string(),
            five_hour_resets_at: "1756656000".to_string(),
            seven_day_used_pct: String::new(),
            seven_day_resets_at: String::new(),
            account: String::new(),
            tier: String::new(),
        };
        let (a, b) = (snap("12"), snap("87"));
        let whole = [a.render(), b.render()];
        std::thread::scope(|s| {
            let writers: Vec<_> = [&a, &b].into_iter().map(|w| s.spawn(|| (0..200).all(|_| w.write(&usage)))).collect();
            for w in writers {
                assert!(w.join().expect("a writer panicked"), "a snapshot write failed");
            }
        });
        let text = std::fs::read_to_string(&usage).expect("no snapshot");
        assert!(whole.contains(&text), "a torn snapshot: {:?}", text);
        let left = crate::walk::list_dir(&dir).expect("scratch");
        assert_eq!(left.len(), 1, "a temporary stayed: {:?}", left);
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — an epoch passes through and a
    // non-numeric stamp is converted, so a source that switched representation is still read
    #[test]
    fn an_epoch_passes_through_and_a_stamp_converts() {
        assert_eq!(epoch_of("1756656000"), "1756656000");
        assert_eq!(epoch_of(""), "");
        assert_eq!(epoch_of("1970-01-02T00:00:00Z"), "86400");
        assert_eq!(epoch_of("not a time at all"), "");
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — a reset needs a time of day, so a bare
    // calendar day is no epoch rather than a guess at local midnight
    #[test]
    fn a_bare_calendar_day_is_no_reset() {
        assert_eq!(epoch_of("1970-01-02"), "");
        assert_eq!(window("56", "2026-11-28"), Err(Half::Reset));
        assert_eq!(epoch_of("1970-01-02T00:00Z"), "86400");
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — a utilization reading is a decimal, so
    // a null, an empty field, a word and a stray or doubled dot are all no reading
    #[test]
    fn only_a_decimal_is_a_reading() {
        assert!(is_utilization("91"));
        assert!(is_utilization("12.5"));
        assert!(!is_utilization(""));
        assert!(!is_utilization("null"));
        assert!(!is_utilization("91%"));
        for malformed in ["1..2", ".", "1.", ".5", "1.2.3"] {
            assert!(!is_utilization(malformed), "{:?}", malformed);
        }
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — a pair is both halves in shape or
    // neither; a non-scalar reset, which a JSON reader renders `{}`, is no epoch
    #[test]
    fn a_window_is_both_halves_in_shape_or_neither() {
        assert_eq!(window("", ""), Ok(None));
        assert_eq!(window("56", "1756656000"), Ok(Some(("56".to_string(), "1756656000".to_string()))));
        assert_eq!(window("56", "1970-01-02T00:00:00Z"), Ok(Some(("56".to_string(), "86400".to_string()))));
        assert_eq!(window("1..2", "1756656000"), Err(Half::Utilization));
        assert_eq!(window("", "1756656000"), Err(Half::Utilization));
        assert_eq!(window("56", "{}"), Err(Half::Reset));
        assert_eq!(window("56", ""), Err(Half::Reset));
    }
}
