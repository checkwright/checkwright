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
        let tmp = format!("{}.tmp", usage_file);
        if std::fs::write(&tmp, self.render()).is_err() {
            return false;
        }
        if std::fs::rename(&tmp, usage_file).is_err() {
            let _ = std::fs::remove_file(&tmp);
            return false;
        }
        true
    }
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
// all-digit value is already one, anything else goes through `date -d`, whose zone reading is the
// operator's and is why drift-kit keeps that call a subprocess rather than an in-crate conversion.
pub fn epoch_of(v: &str) -> String {
    if v.is_empty() {
        return String::new();
    }
    if v.bytes().all(|b| b.is_ascii_digit()) {
        return v.to_string();
    }
    kpi::date_epoch(v).map(|e| e.to_string()).unwrap_or_default()
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — a utilization reading is digits and dots
// and nothing else, the shell form's `[[ "$v" =~ ^[0-9.]+$ ]]`; anything else is no reading.
pub fn is_utilization(v: &str) -> bool {
    !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit() || b == b'.')
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // spec: delegation-kit/SPEC.md §The usage.txt contract — an epoch passes through and a
    // non-numeric stamp is converted, so a source that switched representation is still read
    #[test]
    fn an_epoch_passes_through_and_a_stamp_converts() {
        assert_eq!(epoch_of("1756656000"), "1756656000");
        assert_eq!(epoch_of(""), "");
        assert_eq!(epoch_of("1970-01-02T00:00:00Z"), "86400");
        assert_eq!(epoch_of("not a time at all"), "");
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — a utilization reading is digits and
    // dots, so a null, an empty field and a word are all no reading
    #[test]
    fn only_digits_and_dots_are_a_reading() {
        assert!(is_utilization("91"));
        assert!(is_utilization("12.5"));
        assert!(!is_utilization(""));
        assert!(!is_utilization("null"));
        assert!(!is_utilization("91%"));
    }
}
