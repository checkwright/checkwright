// spec: guard-kit/SPEC.md §The guard framework — the crate's guard: the reader seam and its bash
// reader, the allow-match model, and the `--guard-json` reads and renders.
pub mod bash;
pub mod engine;
pub mod host;
pub mod reader;
mod rules;
pub mod text;

// spec: guard-kit/SPEC.md §The guard framework — the kit-relative shell library, with two readers:
// the holder the reader is compared against, and the file whose presence *is* guard-kit being
// vendored, which is how `kpi-prompt-friction` witnesses the kit (drift-kit/SPEC.md §Bundled KPIs).
pub const LIB: &str = "lib/guard.sh";

// spec: guard-kit/SPEC.md §The guard framework — the settings-allow match core and its one compiled
// holder: a closing `:*`, or a closing ` *` that is the rule's only `*`, is the head alone or the
// head, a space, anything.
pub fn allow_match(s: &str, glob: &str) -> bool {
    let (body, tail) = match glob.strip_suffix(')').filter(|b| b.ends_with('*')) {
        Some(b) => (b, ")"),
        None => (glob, ""),
    };
    let head = if let Some(h) = body.strip_suffix(":*") {
        Some(h.replace(":*", "*"))
    } else {
        body.strip_suffix(" *")
            .filter(|h| !h.contains('*'))
            .map(String::from)
    };
    match head {
        Some(head) => {
            crate::walk::glob_match(&format!("{head}{tail}"), s)
                || crate::walk::glob_match(&format!("{head} *{tail}"), s)
        }
        None => crate::walk::glob_match(&glob.replace(":*", "*"), s),
    }
}

// spec: guard-kit/SPEC.md §The guard framework (`lib/guard.sh`) — `--guard-json`'s `field` and
// `field-or-empty` modes; `or_empty` is jq's `// empty`, which fires on false too.
pub fn json_field(payload: Option<&serde_json::Value>, path: &str, or_empty: bool) -> Option<String> {
    use serde_json::Value;
    let v = crate::json::Path::compile(path).ok()?.eval(payload?).ok()?;
    match v {
        Value::String(s) => Some(s),
        Value::Bool(false) if or_empty => None,
        Value::Bool(_) | Value::Number(_) => Some(v.to_string()),
        Value::Null | Value::Object(_) | Value::Array(_) => None,
    }
}

// spec: guard-kit/SPEC.md §The guard framework — `guard_allow`'s envelope; the braces stay a
// literal so the key order is the one the library always printed.
pub fn allow_envelope(reason: &str) -> String {
    format!(
        r#"{{"hookSpecificOutput":{{"hookEventName":"PreToolUse","permissionDecision":"allow","permissionDecisionReason":{}}}}}"#,
        crate::hook::quote(reason)
    )
}

// spec: guard-kit/SPEC.md §The guard framework — the rewrite envelope, key order as above
pub fn rewrite_envelope(cmd: &str, reason: &str) -> String {
    format!(
        r#"{{"hookSpecificOutput":{{"hookEventName":"PreToolUse","permissionDecision":"allow","permissionDecisionReason":{},"updatedInput":{{"command":{}}}}}}}"#,
        crate::hook::quote(reason),
        crate::hook::quote(cmd)
    )
}

// spec: guard-kit/SPEC.md §The guard framework — `--guard-json <mode> [<arg>…]`, the reads and
// renders `lib/guard.sh` spawns; every mode exits 0 and only a usage error exits 2.
pub fn json_arm(args: &[String]) -> i32 {
    let usage = "  usage: checkwright-gates --guard-json field <path> | --guard-json field-or-empty <path> | --guard-json allow-entries <settings-file> | --guard-json advise <msg> | --guard-json allow <reason> | --guard-json rewrite <cmd> <reason>";
    let arg = |i: usize| args.get(i).map(String::as_str);
    let out = match (arg(0), arg(1), arg(2)) {
        (Some(m @ ("field" | "field-or-empty")), Some(path), None) => {
            let payload = crate::hook::read_payload();
            json_field(payload.as_ref(), path, m == "field-or-empty").map(|v| vec![v])
        }
        (Some("allow-entries"), Some(file), None) => match crate::emit::compare_settings_allow::read_allow(file) {
            crate::emit::compare_settings_allow::AllowRead::Entries(e) => Some(e),
            _ => None,
        },
        (Some("advise"), Some(msg), None) => Some(vec![crate::hook::advise_envelope(msg)]),
        (Some("allow"), Some(reason), None) => Some(vec![allow_envelope(reason)]),
        (Some("rewrite"), Some(cmd), Some(reason)) if args.len() == 3 => {
            Some(vec![rewrite_envelope(cmd, reason)])
        }
        _ => {
            eprintln!("checkwright-gates: --guard-json needs a mode and exactly its operands — nothing could be read or rendered; treating as failure (not clean)");
            eprintln!("{}", usage);
            return 2;
        }
    };
    for line in out.unwrap_or_default() {
        println!("{}", line);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §The guard framework — the field read's rendering per JSON type,
    // and the one place `field-or-empty` differs from `field`
    #[test]
    fn a_field_renders_each_json_type_as_the_library_reads_it() {
        let doc: serde_json::Value = serde_json::from_str(
            r#"{"tool_input":{"command":"a\r\nb","n":4.5,"t":true,"f":false,"z":null,"o":{"k":1},"a":[1]},"arr":["x"]}"#,
        )
        .expect("the fixture must parse");
        let f = |p: &str, e: bool| json_field(Some(&doc), p, e);
        assert_eq!(f(".tool_input.command", false).as_deref(), Some("a\r\nb"), "a CR is read verbatim");
        assert_eq!(f(".tool_input.n", false).as_deref(), Some("4.5"));
        assert_eq!(f(".tool_input.t", true).as_deref(), Some("true"));
        assert_eq!(f(".tool_input.f", false).as_deref(), Some("false"));
        assert_eq!(f(".tool_input.f", true), None, "// empty fires on false");
        assert_eq!(f(".tool_input.z", false), None);
        assert_eq!(f(".tool_input.o", false), None);
        assert_eq!(f(".tool_input.a", false), None);
        assert_eq!(f(".tool_input.missing", false), None);
        assert_eq!(f(".arr[0]", false).as_deref(), Some("x"));
        assert_eq!(f(".tool_input[\"command\"]", true).as_deref(), Some("a\r\nb"));
        assert_eq!(f(".tool_input.command | length", false), None, "a filter reads as absent");
        assert_eq!(f(".tool_input.command.deeper", false), None, "a type error reads as absent");
        assert_eq!(json_field(None, ".tool_input.command", true), None);
    }

    // spec: guard-kit/SPEC.md §The guard framework — the envelopes parse, carry every
    // interpolated value back unchanged, and keep the key order the library printed
    #[test]
    fn every_envelope_serializes_its_values_and_keeps_its_key_order() {
        let hostile = "a \"quoted\" \\ back\nslash\ttab \u{1}";
        let allow = allow_envelope(hostile);
        let v: serde_json::Value = serde_json::from_str(&allow).expect("allow must parse");
        assert_eq!(v["hookSpecificOutput"]["permissionDecisionReason"], hostile);
        assert!(allow.starts_with(r#"{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","permissionDecisionReason":"#));
        let rw = rewrite_envelope(hostile, "why");
        let v: serde_json::Value = serde_json::from_str(&rw).expect("rewrite must parse");
        assert_eq!(v["hookSpecificOutput"]["updatedInput"]["command"], hostile);
        assert!(rw.ends_with(r#""permissionDecisionReason":"why","updatedInput":{"command":"a \"quoted\" \\ back\nslash\ttab \u0001"}}}"#));
        let adv = crate::hook::advise_envelope(hostile);
        let v: serde_json::Value = serde_json::from_str(&adv).expect("advise must parse");
        assert_eq!(v["hookSpecificOutput"]["additionalContext"], hostile);
        assert!(!allow.contains('\n') && !rw.contains('\n') && !adv.contains('\n'));
    }

    // spec: guard-kit/SPEC.md §The guard framework — only a usage error exits 2
    #[test]
    fn a_malformed_invocation_is_the_one_exit_2() {
        let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(json_arm(&s(&[])), 2);
        assert_eq!(json_arm(&s(&["nope", "x"])), 2);
        assert_eq!(json_arm(&s(&["rewrite", "only-one"])), 2);
        assert_eq!(json_arm(&s(&["allow", "a", "b"])), 2);
        assert_eq!(json_arm(&s(&["allow-entries", "/no/such/settings.json"])), 0);
    }

    // spec: guard-kit/SPEC.md §The guard framework — a closing `:*` grants the head alone or the
    // head and a space, never a glued continuation, in both the inner and the wrapped rule form
    #[test]
    fn a_closing_colon_star_is_bounded_by_a_space_or_the_end() {
        for s in ["touch f", "touch f g"] {
            assert!(allow_match(s, "touch f:*"), "{s}");
        }
        for s in ["touch fg", "touch f-g", "touch f.g"] {
            assert!(!allow_match(s, "touch f:*"), "{s}");
        }
        assert!(!allow_match("python3 -c x", "python3 -:*"));
        assert!(allow_match("Bash(git status)", "Bash(git status:*)"));
        assert!(allow_match("Bash(git status --short)", "Bash(git status:*)"));
        assert!(!allow_match("Bash(git statusx)", "Bash(git status:*)"));
        assert!(allow_match("touch fg", "touch f*"));
    }

    // spec: guard-kit/SPEC.md §The guard framework — a closing ` *` that is the rule's only `*`
    // grants the bare head too; beside another `*` it does not
    #[test]
    fn a_sole_trailing_space_star_also_grants_the_bare_head() {
        assert!(allow_match("git status", "git status *"));
        assert!(allow_match("Bash(git status)", "Bash(git status *)"));
        assert!(allow_match("touch f", "touch f *"));
        assert!(!allow_match("touch fg", "touch f *"));
        assert!(!allow_match("git -C . status", "git -C * status *"));
        assert!(allow_match("git -C . status --short", "git -C * status *"));
    }
}
