// spec: gate-sdk/SPEC.md §The non-gate arm — the harness-integration arms: non-gate arms
// whose named caller is the coding harness, so their channels are the harness's rather than the
// gate output contract's. `--hook <member>` is the one dispatching arm of that sub-class.
use serde_json::Value;

pub mod budget;
pub mod dispatch;
pub mod escalation;
pub mod poll;
pub mod shell_guard;
pub mod statusline;
pub mod stop_liveness;
pub mod usage;
pub mod verdict;
pub mod wakeup;
pub mod workflow_state;

// spec: gate-sdk/SPEC.md §The non-gate arm — the parse is `Option` rather than a `Value` that
// might be null, because an absent, empty or unparseable payload is one condition with one
// consequence: the member's own degraded path, the path each shell member had for a missing jq.
pub struct Payload {
    bytes: Vec<u8>,
    value: Option<Value>,
}

impl Payload {
    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub type HookFn = fn(&Payload) -> i32;

// spec: gate-sdk/SPEC.md §The harness-integration arm — the member table: the single roster the
// arm dispatches on and the unknown-member refusal prints. Each row's knob slice is exactly what
// that member's shell original read, less the knobs a compiled member cannot want; its last column
// is the owning kit, held to the member module's leading `spec:` binding by a unit test below.
pub const HOOKS: &[(&str, HookFn, &[&str], &str)] = &[
    // spec: delegation-kit/SPEC.md §usage-verdict — the rule runs inside the hook process, so the row
    // declares the rule's own reads rather than a path to it, one roster for both callers
    (
        "agent-budget-guard",
        |p| budget::run(p.value()),
        verdict::KNOBS,
        "delegation-kit",
    ),
    (
        "agent-dispatch-guard",
        |p| dispatch::run(p.value()),
        &[
            "DELEGATION_KIT_READONLY_TYPES",
            "DELEGATION_KIT_MUTATING_TYPES",
            "DELEGATION_KIT_REQUIRE_TIER",
            "DELEGATION_KIT_AGENT_DIR",
        ],
        "delegation-kit",
    ),
    (
        "subagent-stop-liveness",
        |p| stop_liveness::run(p.value()),
        &[
            "DELEGATION_KIT_STOP_LOG",
            "DELEGATION_KIT_LIVENESS_CMD",
            "GATE_SDK_TMP_DIR",
        ],
        "delegation-kit",
    ),
    ("escalation-guard", |p| escalation::run(p.value()), &[], "guard-kit"),
    ("shell-guard", shell_guard::run, crate::guard::host::KNOBS, "guard-kit"),
    (
        "wakeup-guard",
        |p| wakeup::run(p.value()),
        &["GUARD_KIT_WAKEUP_LOG"],
        "guard-kit",
    ),
    (
        "workflow-state-guard",
        |p| workflow_state::run(p.value()),
        &["LIFECYCLE_KIT_STAGE_SESSION_TYPES", "LIFECYCLE_KIT_STATE_FILE"],
        "lifecycle-kit",
    ),
];

// spec: gate-sdk/SPEC.md §The non-gate arm — the sentinel `--hook`'s own declared roster carries,
// standing for one member's knobs where the arm's argv names a member and for the union over the
// table where it does not
pub const EVERY_HOOK_KNOB: &str = "@every-hook-knob";

pub fn members() -> Vec<&'static str> {
    HOOKS.iter().map(|(n, _, _, _)| *n).collect()
}

pub fn owner(member: &str) -> Option<&'static str> {
    HOOKS.iter().find(|(n, _, _, _)| *n == member).map(|(_, _, _, o)| *o)
}

// spec: context-kit/SPEC.md §check-settings-paths — a hook's tokens: `command` split on ASCII
// whitespace, or, in the exec form, `command` followed by each `args` element verbatim
pub fn command_tokens(hook: &Value) -> Vec<&str> {
    let cmd = hook.get("command").and_then(Value::as_str).unwrap_or("");
    match hook.get("args").and_then(Value::as_array) {
        Some(args) => std::iter::once(cmd)
            .chain(args.iter().filter_map(Value::as_str))
            .collect(),
        None => cmd.split_ascii_whitespace().collect(),
    }
}

// spec: context-kit/SPEC.md §check-settings-paths — the command token is not always argv[0]: a
// command may lead with `env NAME=VALUE ...` before the interpreter, and a `bash`/`sh` interpreter
// word is then skipped too. One walk for the grant reader, the hook reader and the enforcement map.
pub fn command_index(tok: &[&str]) -> Option<usize> {
    let mut i = 0usize;
    if tok.first() == Some(&"env") {
        i = 1;
        while i < tok.len() && is_assignment(tok[i]) {
            i += 1;
        }
    }
    if matches!(tok.get(i), Some(&"bash") | Some(&"sh")) {
        i += 1;
    }
    (i < tok.len()).then_some(i)
}

fn is_assignment(t: &str) -> bool {
    let Some(at) = t.find('=') else { return false };
    let b = t.as_bytes();
    at > 0 && (b[0].is_ascii_alphabetic() || b[0] == b'_')
}

pub const FRONT_ENDS: [&str; 2] = ["run-gates.sh", "run-gates.ps1"];

#[derive(Debug, PartialEq)]
pub enum Registration<'a> {
    NotAMember,
    Member(&'a str),
    NoOperand,
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the registration grammar's one parser: a
// command token with the front end's file name, then `--hook`, then the member operand
pub fn registration<'a>(tok: &[&'a str]) -> Registration<'a> {
    let Some(i) = command_index(tok) else {
        return Registration::NotAMember;
    };
    let base = tok[i]
        .trim_matches(|c| c == '"' || c == '\'')
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("");
    if !FRONT_ENDS.contains(&base) || tok.get(i + 1) != Some(&"--hook") {
        return Registration::NotAMember;
    }
    match tok.get(i + 2) {
        Some(m) => Registration::Member(m),
        None => Registration::NoOperand,
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — stdout is the hook-JSON envelope, serialized rather
// than composed by hand: this retires `agent-dispatch-guard.sh`'s degraded arm, which kept its
// advisory literals free of any character JSON must escape by convention alone.
pub fn advise(msg: &str) -> i32 {
    println!("{}", advise_envelope(msg));
    0
}

pub fn advise_envelope(msg: &str) -> String {
    format!(
        r#"{{"hookSpecificOutput":{{"hookEventName":"PreToolUse","additionalContext":{}}}}}"#,
        quote(msg)
    )
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the escaper the hand-written envelope never had. The
// envelope's own braces stay a literal so the key order matches the shell guard's original advise
// envelope, which a JSON object does not carry but a reader diffing two substrates' output does.
pub fn quote(s: &str) -> String {
    serde_json::Value::String(s.to_string()).to_string()
}

// spec: gate-sdk/SPEC.md §The non-gate arm — stderr is the member's block text, which the harness
// shows, and exit 2 is the harness's block signal; the `<name>: ` prefix matches the shell guard's.
pub fn block(name: &str, msg: &str) -> i32 {
    eprintln!("{}: {}", name, msg);
    2
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the member's own face of the fail-open
// rule: a guard that cannot run declines rather than wedging the session. The front-end holds the
// cause it can see; this holds the one only the member can, an unresolvable declared knob.
pub fn decline(name: &str, reason: &str, payload: Option<&Value>) -> i32 {
    let text = format!(
        "{}: {} — the rule could not be enforced on this call and the call was allowed.",
        name, reason
    );
    println!("{}", decline_envelope(&text, payload));
    eprintln!("{}", text);
    0
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the firing event's envelope: the advise
// envelope reaches the model on `PreToolUse`; elsewhere the universal `systemMessage` reaches the
// operator, a turn-end event having no model channel that does not refuse the stop
pub fn decline_envelope(text: &str, payload: Option<&Value>) -> String {
    if field(payload, &["hook_event_name"]) == "PreToolUse" {
        advise_envelope(text)
    } else {
        format!(r#"{{"systemMessage":{}}}"#, quote(text))
    }
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
    let Some((_, f, _, _)) = HOOKS.iter().find(|(n, _, _, _)| n == member) else {
        eprintln!(
            "checkwright-gates: no such hook member: {} — the hook could not run",
            member
        );
        eprintln!("  help: this binary carries: {}", members().join(", "));
        return 2;
    };
    f(&read_input())
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the payload read whole and parsed once; a read error,
// an empty body and a body that is not JSON collapse to `None`, the member's degraded input.
pub fn read_input() -> Payload {
    use std::io::Read;
    let mut bytes = Vec::new();
    if std::io::stdin().read_to_end(&mut bytes).is_err() {
        return Payload { bytes: Vec::new(), value: None };
    }
    let value = if bytes.iter().all(u8::is_ascii_whitespace) {
        None
    } else {
        serde_json::from_slice(&bytes).ok()
    };
    Payload { bytes, value }
}

pub fn read_payload() -> Option<Value> {
    read_input().value
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the UTC stamp the log line carries,
// computed rather than spawned: `date -u` asks no question about the operator's zone.
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
pub(crate) fn civil_from_days(z: i64) -> (i64, u32, u32) {
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
        assert!(members().contains(&"agent-budget-guard"));
        assert!(!members().contains(&"agent-budget-guards"));
        assert!(!members().contains(&"PreToolUse"));
        assert_eq!(members().len(), HOOKS.len());
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a decline speaks through the firing
    // event's envelope and never through a verdict: exit 0, and the event read off the payload
    #[test]
    fn a_decline_writes_the_firing_events_envelope() {
        let doc = |ev: &str| -> Value {
            serde_json::from_str(&format!(r#"{{"hook_event_name":"{}"}}"#, ev)).unwrap()
        };
        let pre = doc("PreToolUse");
        let adv: Value =
            serde_json::from_str(&decline_envelope("m: \"k\" unset", Some(&pre))).unwrap();
        assert_eq!(adv["hookSpecificOutput"]["hookEventName"], "PreToolUse");
        assert_eq!(adv["hookSpecificOutput"]["additionalContext"], "m: \"k\" unset");
        assert!(adv.get("systemMessage").is_none());
        for p in [Some(&doc("SubagentStop")), None] {
            let sys: Value = serde_json::from_str(&decline_envelope("t", p)).unwrap();
            assert_eq!(sys, serde_json::json!({"systemMessage": "t"}));
        }
        assert_eq!(decline("m", "r", None), 0);
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — the owner column is checked against the
    // member module rather than transcribed: each row's owner is the kit its module's leading
    // `spec:` binding names, and a member missing from this module list fails rather than passes
    #[test]
    fn each_members_owner_is_the_kit_its_module_binds_to() {
        let modules: &[(&str, &str)] = &[
            ("agent-budget-guard", include_str!("budget.rs")),
            ("agent-dispatch-guard", include_str!("dispatch.rs")),
            ("subagent-stop-liveness", include_str!("stop_liveness.rs")),
            ("escalation-guard", include_str!("escalation.rs")),
            ("shell-guard", include_str!("shell_guard.rs")),
            ("wakeup-guard", include_str!("wakeup.rs")),
            ("workflow-state-guard", include_str!("workflow_state.rs")),
        ];
        for (name, _, _, owner) in HOOKS {
            let src = modules
                .iter()
                .find(|(m, _)| m == name)
                .unwrap_or_else(|| panic!("member {} names no module in this test", name))
                .1;
            let kit = src
                .lines()
                .next()
                .and_then(|l| l.strip_prefix("// spec: "))
                .and_then(|l| l.split('/').next())
                .unwrap_or_else(|| panic!("member {}'s module leads with no spec: binding", name));
            assert_eq!(kit, *owner, "member {}'s owner", name);
        }
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — the parser both readers of a
    // registration call: the front end by file name as the command token, then `--hook`
    #[test]
    fn a_registration_names_its_member_only_through_the_front_end() {
        let t = |s: &'static str| s.split_ascii_whitespace().collect::<Vec<_>>();
        assert_eq!(
            registration(&t("bash gate-sdk/bin/run-gates.sh --hook shell-guard")),
            Registration::Member("shell-guard")
        );
        assert_eq!(
            registration(&t("env A=1 bash \"${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh\" --hook x")),
            Registration::Member("x")
        );
        assert_eq!(
            registration(&t("bash gate-sdk/bin/run-gates.sh --hook")),
            Registration::NoOperand
        );
        assert_eq!(
            registration(&t("bash gate-sdk/bin/run-gates.sh --run")),
            Registration::NotAMember
        );
        assert_eq!(
            registration(&t("bash scripts/x.sh --hook shell-guard")),
            Registration::NotAMember
        );
        assert_eq!(registration(&t("")), Registration::NotAMember);
        let exec: Value = serde_json::from_str(
            r#"{"command":"bash","args":["gate-sdk/bin/run-gates.sh","--hook","a b"]}"#,
        )
        .unwrap();
        assert_eq!(registration(&command_tokens(&exec)), Registration::Member("a b"));
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
