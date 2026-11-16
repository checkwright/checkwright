// spec: delegation-kit/SPEC.md §The usage.txt contract — the timer-driven poll producer: one poll
// cycle per invocation, an atomic snapshot rewrite, fail-soft. Its caller is a refresh command or a
// session rather than a gate on a tool call, so it keeps exit 2 where the guard arms fail open.
use crate::hook::usage::{self, Snapshot};
use crate::{proc, programs};
use crate::walk;
use serde_json::Value;

// spec: delegation-kit/SPEC.md §The usage.txt contract — the arm's declared reads. It reaches an
// external endpoint with an OAuth bearer token, which is the capability the operator's 2026-08-31
// ruling admitted into the installed binary with the tradeoff stated.
pub const KNOBS: &[&str] = &[
    "DELEGATION_KIT_USAGE_FILE",
    "DELEGATION_KIT_CRED_FILE",
    "DELEGATION_KIT_ACCOUNT_CONFIG",
    "DELEGATION_KIT_USAGE_ENDPOINT",
];

// spec: delegation-kit/SPEC.md §The usage.txt contract — a fail-soft cycle: the snapshot is left
// untouched and the reason is named, so `usage-verdict` reads the staleness as STALE rather than
// meeting a silent green. Exit 1 is that outcome; exit 0 wrote a snapshot.
fn fail(problem: &str, help: &str) -> i32 {
    eprintln!("usage-poller: {}", problem);
    eprintln!("  help: {}", help);
    1
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — this arm's callers (a timer's log, the refresh
// spawn, a session) all read its status, so a surplus argument, `--help` included, refuses at
// exit 2 before a file is read or curl is spawned; the spelling follows `run_validate::dispatch`.
pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        eprintln!(
            "usage-poller: takes no argument (got: {}) — usage: --usage-poll   (it takes no argument)",
            a
        );
        return 2;
    }
    let knob = |name: &str| walk::knob_scalar(name).unwrap_or_default();
    let usage::Paths { usage_file, cred_file, account_config } = usage::paths().unwrap_or_default();
    let endpoint = knob("DELEGATION_KIT_USAGE_ENDPOINT");

    // spec: delegation-kit/SPEC.md §The usage.txt contract — curl stays external and stays spawned:
    // no HTTP client enters the crate and no dependency is added. `jq` leaves the path entirely,
    // its two jobs — reading the credential file and mapping the payload — now being the crate's.
    if !proc::on_path(&programs::CURL) {
        return fail(
            "curl not found",
            "install curl; the poller fetches the usage source over HTTPS (file:// for a test stub).",
        );
    }

    if !std::path::Path::new(&cred_file).is_file() {
        return fail(
            &format!("credentials file unreadable: {}", cred_file),
            "point DELEGATION_KIT_CRED_FILE at the harness credentials file; the snapshot is untouched.",
        );
    }
    let token = usage::json_field(&cred_file, &["claudeAiOauth", "accessToken"]);
    if token.is_empty() {
        return fail(
            &format!("no OAuth token in {}", cred_file),
            "log the harness in to refresh the credentials file; the snapshot is untouched.",
        );
    }

    let moved = "the snapshot is untouched; the source shape moved — adjust the copy or DELEGATION_KIT_USAGE_ENDPOINT.";
    let payload = match fetch(&token, &endpoint) {
        Some(p) => p,
        None => return fail(
            &format!("fetch failed: {}", endpoint),
            "the snapshot is untouched — usage-verdict reads its staleness as STALE, never a silent green; if the source moved, set DELEGATION_KIT_USAGE_ENDPOINT.",
        ),
    };

    let [five_pct, five_resets, seven_pct, seven_resets] = match axes(&payload) {
        Ok(a) => a,
        Err(why) => return fail(&format!("unparseable payload: {} at {}", why, endpoint), moved),
    };

    let snapshot = Snapshot {
        five_hour_used_pct: five_pct,
        five_hour_resets_at: five_resets,
        seven_day_used_pct: seven_pct,
        seven_day_resets_at: seven_resets,
        account: usage::json_field(&account_config, &["oauthAccount", "accountUuid"]),
        tier: usage::json_field(&cred_file, &["claudeAiOauth", "subscriptionType"]),
    };
    if !snapshot.write(&usage_file) {
        return fail(
            &format!("cannot write the snapshot at {}", usage_file),
            "check the directory's permissions; the previous snapshot is untouched.",
        );
    }
    0
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — the payload mapped onto the snapshot's two
// axes; no five-hour window writes that pair empty beside an armed weekly pair, and neither axis,
// or half of either, is unparseable.
fn axes(payload: &Value) -> Result<[String; 4], &'static str> {
    let five = axis(payload, "five_hour");
    let seven = axis(payload, "seven_day");
    let [seven_pct, seven_resets] = if seven.0.is_empty() && seven.1.is_empty() {
        [String::new(), String::new()]
    } else if !usage::is_utilization(&seven.0) {
        return Err("no weekly utilization");
    } else {
        let resets = usage::epoch_of(&seven.1);
        if resets.is_empty() {
            return Err("no weekly reset epoch");
        }
        [seven.0, resets]
    };
    if five.0.is_empty() && five.1.is_empty() {
        if seven_pct.is_empty() {
            return Err("no five-hour and no weekly window");
        }
        return Ok([String::new(), String::new(), seven_pct, seven_resets]);
    }
    if !usage::is_utilization(&five.0) {
        return Err("no five-hour utilization");
    }
    let five_resets = usage::epoch_of(&five.1);
    if five_resets.is_empty() {
        return Err("no five-hour reset epoch");
    }
    Ok([five.0, five_resets, seven_pct, seven_resets])
}

fn fetch(token: &str, endpoint: &str) -> Option<Value> {
    let out = proc::run(
        &programs::CURL,
        &[
            "-fsS",
            "--max-time",
            "30",
            "-H",
            &format!("Authorization: Bearer {}", token),
            "-H",
            "anthropic-beta: oauth-2025-04-20",
            endpoint,
        ],
    )
    .ok()?;
    serde_json::from_slice(out.stdout()?).ok()
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — one axis's utilization and reset stamp,
// read as strings whatever the source spelled them as, because the snapshot's own contract is text
// and a number re-rendered through a float would change the reading it carries.
fn axis(payload: &Value, name: &str) -> (String, String) {
    let at = |parent: Option<&Value>, key: &str| -> String {
        match parent.and_then(|p| p.get(key)) {
            Some(Value::String(s)) => s.clone(),
            Some(Value::Number(n)) => n.to_string(),
            _ => String::new(),
        }
    };
    let flat = payload.get(name);
    let nested = payload.get("rate_limits").and_then(|r| r.get(name));
    let pick = |flat_key: &str, nested_key: &str| -> String {
        let v = at(flat, flat_key);
        if v.is_empty() {
            at(nested, nested_key)
        } else {
            v
        }
    };
    (
        pick("utilization", "used_percentage"),
        pick("resets_at", "resets_at"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(src: &str) -> Value {
        serde_json::from_str(src).expect("the fixture must parse")
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — the flat shape leads and the nested
    // one is the fallback, so a source that moved between them still reads
    #[test]
    fn either_source_shape_reads_the_same_axis() {
        let flat = doc(r#"{"five_hour":{"utilization":91,"resets_at":"1756656000"}}"#);
        assert_eq!(axis(&flat, "five_hour"), ("91".to_string(), "1756656000".to_string()));
        let nested = doc(
            r#"{"rate_limits":{"five_hour":{"used_percentage":12.5,"resets_at":1756656000}}}"#,
        );
        assert_eq!(
            axis(&nested, "five_hour"),
            ("12.5".to_string(), "1756656000".to_string())
        );
        assert_eq!(axis(&flat, "seven_day"), (String::new(), String::new()));
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — the flat shape wins where both are
    // present, which is the `//` chain's own precedence and not an arbitrary one
    #[test]
    fn the_flat_shape_outranks_the_nested_one() {
        let both = doc(
            r#"{"five_hour":{"utilization":"1"},"rate_limits":{"five_hour":{"used_percentage":"2"}}}"#,
        );
        assert_eq!(axis(&both, "five_hour").0, "1");
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — no five-hour window maps to the empty
    // pair beside an armed weekly one, and only a payload with neither axis, or a half axis, refuses
    #[test]
    fn a_missing_five_hour_window_maps_to_the_empty_pair() {
        let weekly = doc(r#"{"five_hour":null,"seven_day":{"utilization":56,"resets_at":"1756656000"}}"#);
        assert_eq!(
            axes(&weekly),
            Ok([String::new(), String::new(), "56".to_string(), "1756656000".to_string()])
        );
        let nested = doc(r#"{"rate_limits":{"seven_day":{"used_percentage":3,"resets_at":1756656000}}}"#);
        assert_eq!(axes(&nested).map(|a| a[2].clone()), Ok("3".to_string()));
        assert!(axes(&doc(r#"{"five_hour":null,"seven_day":null}"#)).is_err());
        assert!(axes(&doc(r#"{"seven_day":{"utilization":56}}"#)).is_err());
        assert_eq!(
            axes(&doc(r#"{"five_hour":{"utilization":4},"seven_day":{"utilization":5,"resets_at":"1"}}"#)),
            Err("no five-hour reset epoch")
        );
        assert_eq!(
            axes(&doc(r#"{"five_hour":{"resets_at":"1"}}"#)),
            Err("no five-hour utilization")
        );
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — half a weekly pair is unparseable beside
    // a complete five-hour pair, where it wrote the weekly axis unarmed; a weekly window absent or
    // null still writes the three mandatory lines alone
    #[test]
    fn half_a_weekly_pair_refuses_beside_a_complete_five_hour_pair() {
        let five = r#""five_hour":{"utilization":4,"resets_at":"1756656000"}"#;
        let with = |seven: &str| axes(&doc(&format!("{{{},\"seven_day\":{}}}", five, seven)));
        assert_eq!(with(r#"{"utilization":99}"#), Err("no weekly reset epoch"));
        assert_eq!(with(r#"{"utilization":99,"resets_at":null}"#), Err("no weekly reset epoch"));
        assert_eq!(with(r#"{"resets_at":"1756656000"}"#), Err("no weekly utilization"));
        assert_eq!(with(r#"{"utilization":"n/a","resets_at":"1756656000"}"#), Err("no weekly utilization"));
        let nested = doc(&format!(
            "{{{},\"rate_limits\":{{\"seven_day\":{{\"used_percentage\":7}}}}}}",
            five
        ));
        assert_eq!(axes(&nested), Err("no weekly reset epoch"));
        let three = [
            "4".to_string(),
            "1756656000".to_string(),
            String::new(),
            String::new(),
        ];
        assert_eq!(with("null"), Ok(three.clone()));
        assert_eq!(axes(&doc(&format!("{{{}}}", five))), Ok(three));
        assert_eq!(
            with(r#"{"utilization":99,"resets_at":"1756656000"}"#).map(|a| a[2..].to_vec()),
            Ok(vec!["99".to_string(), "1756656000".to_string()])
        );
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a surplus argument, `--help` included, refuses
    // at exit 2 before any file is read or curl is spawned
    #[test]
    fn a_surplus_argument_refuses_at_exit_2() {
        assert_eq!(run(&["--bogus".to_string()]), 2);
        assert_eq!(run(&["--help".to_string()]), 2);
    }
}
