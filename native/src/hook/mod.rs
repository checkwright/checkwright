// spec: gate-sdk/SPEC.md §The non-gate arm — the harness-integration arms: bridged non-gate arms
// whose named caller is the coding harness, so their channels are the harness's rather than the
// gate output contract's. `--hook <member>` is the one dispatching arm of that sub-class.
use serde_json::Value;

pub mod budget;
pub mod dispatch;
pub mod escalation;
pub mod poll;
pub mod statusline;
pub mod stop_liveness;
pub mod usage;
pub mod wakeup;
pub mod workflow_state;

// spec: gate-sdk/SPEC.md §The non-gate arm — the payload is `Option` rather than a `Value` that
// might be null, because an absent, empty or unparseable payload is one condition with one
// consequence: the member's own degraded path, the path each shell member had for a missing jq.
pub type HookFn = fn(Option<&Value>) -> i32;

// spec: gate-sdk/SPEC.md §The non-gate arm — the member table: the single roster the arm dispatches
// on, `--knobs --hook` reads and the unknown-member refusal prints. Each row's knob slice is
// exactly what that member's shell original read, less the knobs a compiled member cannot want.
pub const HOOKS: &[(&str, HookFn, &[&str])] = &[
    (
        "agent-budget-guard",
        budget::run,
        &["DELEGATION_KIT_VERDICT_BIN"],
    ),
    // spec: delegation-kit/SPEC.md §Layout and configuration — `DELEGATION_KIT_CONFIG_FILE` is not
    // declared here: it selects which file the bridge sources to resolve the roster below, so it is
    // resolved a process earlier and a declared copy would arrive too late to redirect anything.
    (
        "agent-dispatch-guard",
        dispatch::run,
        &["DELEGATION_KIT_READONLY_TYPES"],
    ),
    (
        "subagent-stop-liveness",
        stop_liveness::run,
        &[
            "DELEGATION_KIT_STOP_LOG",
            "DELEGATION_KIT_LIVENESS_CMD",
            "GATE_SDK_TMP_DIR",
        ],
    ),
    ("escalation-guard", escalation::run, &[]),
    ("wakeup-guard", wakeup::run, &["GUARD_KIT_WAKEUP_LOG"]),
    (
        "workflow-state-guard",
        workflow_state::run,
        &["GATE_SDK_WORKFLOW_DIR"],
    ),
];

// spec: gate-sdk/SPEC.md §The non-gate arm — the sentinel `--hook`'s own declared roster carries:
// it resolves to one member's knobs where the arm's argv names a member and to the union over the
// table where it does not, which is what keeps the bridge resolving one guard's configuration.
pub const EVERY_HOOK_KNOB: &str = "@every-hook-knob";

pub fn members() -> Vec<&'static str> {
    HOOKS.iter().map(|(n, _, _)| *n).collect()
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the per-member answer `--knobs --hook <member>` gives,
// reachable because `gate_knob_env "$arm" "$@"` forwards the arm's own argv to the bridge.
pub fn knobs(member: &str) -> Option<&'static [&'static str]> {
    HOOKS
        .iter()
        .find(|(n, _, _)| *n == member)
        .map(|(_, _, k)| *k)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — stdout is the hook-JSON envelope, serialized rather
// than composed by hand: this retires `agent-dispatch-guard.sh`'s degraded arm, which kept its
// advisory literals free of any character JSON must escape by convention alone.
pub fn advise(msg: &str) -> i32 {
    println!(
        r#"{{"hookSpecificOutput":{{"hookEventName":"PreToolUse","additionalContext":{}}}}}"#,
        quote(msg)
    );
    0
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the escaper the hand-written envelope never had. The
// envelope's own braces stay a literal so the key order is `guard_advise`'s, which a JSON object
// does not carry but a reader diffing two substrates' output does.
pub fn quote(s: &str) -> String {
    serde_json::Value::String(s.to_string()).to_string()
}

// spec: gate-sdk/SPEC.md §The non-gate arm — stderr is the member's block text, which the harness
// shows, and exit 2 is the harness's block signal; the `<name>: ` prefix is `guard_block`'s.
pub fn block(name: &str, msg: &str) -> i32 {
    eprintln!("{}: {}", name, msg);
    2
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the member's own face of the fail-open rule: a guard
// that cannot run declines loudly rather than wedging the session. The front-end holds the two
// causes it can see; this holds the one only the member can, an unresolvable declared knob.
pub fn decline(name: &str, reason: &str) -> i32 {
    eprintln!(
        "{}: {} — the rule could not be enforced on this call and the call was allowed.",
        name, reason
    );
    0
}

// spec: gate-sdk/SPEC.md §The non-gate arm — one field of the payload by object path, rendered as
// `jq -r '<path> // ""'` renders it: a string bare, a number by its own spelling, and null, false
// or absent as empty, `//` being an alternative operator that fires on false too.
pub fn field(payload: Option<&Value>, path: &[&str]) -> String {
    let mut cur = match payload {
        Some(v) => v,
        None => return String::new(),
    };
    for step in path {
        match cur.get(step) {
            Some(next) => cur = next,
            None => return String::new(),
        }
    }
    match cur {
        Value::String(s) => s.clone(),
        Value::Null | Value::Bool(false) => String::new(),
        other => other.to_string(),
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's dispatch: the member name is argv, never the
// hook event name, so the port is a one-for-one substitution of a `command` value and every
// `matcher` in the consumer's settings is left untouched.
pub fn run(args: &[String]) -> i32 {
    let Some(member) = args.first() else {
        eprintln!("checkwright-gates: --hook needs a member name — the hook could not run");
        eprintln!("  help: this binary carries: {}", members().join(", "));
        return 2;
    };
    let Some((_, f, _)) = HOOKS.iter().find(|(n, _, _)| n == member) else {
        eprintln!(
            "checkwright-gates: no such hook member: {} — the hook could not run",
            member
        );
        eprintln!("  help: this binary carries: {}", members().join(", "));
        return 2;
    };
    f(read_payload().as_ref())
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the payload read whole and parsed once; a read error,
// an empty body and a body that is not JSON collapse to `None`, the member's degraded input.
pub fn read_payload() -> Option<Value> {
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).ok()?;
    if buf.iter().all(u8::is_ascii_whitespace) {
        return None;
    }
    serde_json::from_slice(&buf).ok()
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the UTC stamp the log line carries,
// computed rather than spawned: `date -u` asks no question about the operator's zone, so the
// subprocess drift-kit's KPIs keep for `date +%F` has no ground here.
pub fn utc_stamp(epoch: i64) -> String {
    let days = epoch.div_euclid(86_400);
    let secs = epoch.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y,
        m,
        d,
        secs / 3600,
        (secs % 3600) / 60,
        secs % 60
    )
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — Howard Hinnant's civil-from-days, the
// inverse of `emit::trajectory`'s days-from-civil, which is the crate's existing half of this pair.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The non-gate arm — the table is the single roster, so a member
    // resolves only under its own name and the roster the refusal prints is that same table
    #[test]
    fn a_member_resolves_only_under_its_own_name() {
        assert!(knobs("agent-budget-guard").is_some());
        assert!(knobs("agent-budget-guards").is_none());
        assert!(knobs("PreToolUse").is_none());
        assert_eq!(members().len(), HOOKS.len());
        assert_eq!(knobs("escalation-guard"), Some(&[] as &[&str]));
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the computed stamp is the one
    // `date -u +%Y-%m-%dT%H:%M:%SZ` prints, checked at an epoch whose civil value is known
    #[test]
    fn the_utc_stamp_is_the_shell_form_of_the_same_instant() {
        assert_eq!(utc_stamp(0), "1970-01-01T00:00:00Z");
        assert_eq!(utc_stamp(1_756_656_000), "2025-08-31T16:00:00Z");
        assert_eq!(utc_stamp(1_709_164_800), "2024-02-29T00:00:00Z");
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — an absent payload and a payload missing the field
    // are one condition for a reader, which is what makes the degraded path a single branch
    #[test]
    fn a_missing_field_and_a_missing_payload_read_alike() {
        let doc: Value = serde_json::from_str(r#"{"tool_input":{"to":"main"},"n":1}"#)
            .expect("the fixture must parse");
        assert_eq!(field(Some(&doc), &["tool_input", "to"]), "main");
        assert_eq!(field(Some(&doc), &["tool_input", "message"]), "");
        assert_eq!(field(None, &["tool_input", "to"]), "");
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — a number reads as its own spelling, never as
    // empty: the harness sends `used_percentage` as a number, and a reader that took only strings
    // rendered every gauge at zero while a string-spelled fixture agreed with it
    #[test]
    fn a_number_field_reads_as_jq_renders_it() {
        let doc: Value = serde_json::from_str(
            r#"{"context_window":{"used_percentage":42.7},"n":1,"t":true,"f":false,"z":null}"#,
        )
        .expect("the fixture must parse");
        assert_eq!(field(Some(&doc), &["context_window", "used_percentage"]), "42.7");
        assert_eq!(field(Some(&doc), &["n"]), "1");
        assert_eq!(field(Some(&doc), &["t"]), "true");
        assert_eq!(field(Some(&doc), &["f"]), "", "jq's // fires on false too");
        assert_eq!(field(Some(&doc), &["z"]), "");
    }
}
