// spec: guard-kit/SPEC.md §The shell guard — the crate's guard: the reader seam and its bash
// reader, the allow-match model, and the `--guard-json` reads and renders.
pub mod bash;
pub mod engine;
pub mod host;
pub mod reader;
mod rules;
pub mod text;

// spec: drift-kit/SPEC.md §Bundled KPIs — the kit-relative file whose presence *is* guard-kit being
// vendored, which is how `kpi-prompt-friction` witnesses the kit.
pub const WITNESS: &str = "templates/guard-config.knobs";

// spec: guard-kit/SPEC.md §check-guard-registration — the rule table as the binary carries it,
// the gate's in-process subject; the rules module stays private.
pub fn rule_table() -> &'static [engine::Rule] {
    rules::TABLE
}

// spec: guard-kit/SPEC.md §The shell guard — the settings-allow match core and its one compiled
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

// spec: guard-kit/SPEC.md §The shell guard — `--guard-json`'s `field` and
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

// spec: guard-kit/SPEC.md §The shell guard — the reader a payload's `tool_name` selects, with the
// shell the rule table filters on; `None` is a tool no reader serves.
pub fn reader_for(tool: &str) -> Option<(&'static dyn reader::Reader, engine::Shell)> {
    match tool {
        "Bash" => Some((&bash::Bash, engine::Shell::Bash)),
        _ => None,
    }
}

// spec: guard-kit/SPEC.md §The shell guard — the command the rules read: `tool_input.command`
// with its trailing newlines stripped, as the shell guard's command substitution stripped them; an
// absent or empty command is none.
pub fn command_of(payload: &serde_json::Value) -> Option<&str> {
    let cmd = payload.pointer("/tool_input/command")?.as_str()?.trim_end_matches('\n');
    (!cmd.is_empty()).then_some(cmd)
}

// spec: guard-kit/SPEC.md §The shell guard — `guard_allow`'s envelope; the braces stay a
// literal so the key order is the one the library always printed.
pub fn allow_envelope(reason: &str) -> String {
    allow_envelope_with(reason, None)
}

// spec: guard-kit/SPEC.md §Consumer rules — an envelope carrying a consumer fault adds it as
// `additionalContext`, after every key the library printed.
pub fn allow_envelope_with(reason: &str, context: Option<&str>) -> String {
    format!(
        r#"{{"hookSpecificOutput":{{"hookEventName":"PreToolUse","permissionDecision":"allow","permissionDecisionReason":{}{}}}}}"#,
        crate::hook::quote(reason),
        context_key(context)
    )
}

// spec: guard-kit/SPEC.md §The shell guard — the rewrite envelope, key order as above
pub fn rewrite_envelope(cmd: &str, reason: &str) -> String {
    rewrite_envelope_with(cmd, reason, None)
}

pub fn rewrite_envelope_with(cmd: &str, reason: &str, context: Option<&str>) -> String {
    format!(
        r#"{{"hookSpecificOutput":{{"hookEventName":"PreToolUse","permissionDecision":"allow","permissionDecisionReason":{},"updatedInput":{{"command":{}}}{}}}}}"#,
        crate::hook::quote(reason),
        crate::hook::quote(cmd),
        context_key(context)
    )
}

fn context_key(context: Option<&str>) -> String {
    context.map_or(String::new(), |c| format!(r#","additionalContext":{}"#, crate::hook::quote(c)))
}

// spec: guard-kit/SPEC.md §Consumer rules — `--guard-json view`: nothing for each case that section
// lists, or for a dequoted view the skeleton cannot be aligned with.
pub fn json_view(payload: Option<&serde_json::Value>, view: reader::View) -> Option<String> {
    let p = payload?;
    let (reader, _) = reader_for(p.get("tool_name")?.as_str()?)?;
    let cmd = command_of(p)?;
    if reader::control_byte(cmd).is_some() {
        return None;
    }
    reader.view(cmd, view).map(|v| reader::unmark(&v))
}

// spec: guard-kit/SPEC.md §The shell guard — `--guard-json <mode> [<arg>…]`, the reads and
// renders a consumer rule command calls; every mode exits 0 and only a usage error exits 2.
pub fn json_arm(args: &[String]) -> i32 {
    let usage = "  usage: checkwright-gates --guard-json field <path> | --guard-json field-or-empty <path> | --guard-json view <view> | --guard-json allow-entries <settings-file> | --guard-json advise <msg> | --guard-json allow <reason> | --guard-json rewrite <cmd> <reason>";
    let arg = |i: usize| args.get(i).map(String::as_str);
    // spec: guard-kit/SPEC.md §Consumer rules — a view is named by its declaration spelling, as one
    // operand or as its words; `body` names no view of a whole command, since it takes an index.
    let view = (arg(0) == Some("view"))
        .then(|| reader::View::from_spelling(&args[1..].join(" ")))
        .flatten()
        .filter(|v| *v != reader::View::Body);
    let out = match (arg(0), arg(1), arg(2)) {
        (Some("view"), Some(_), _) if view.is_some() => {
            let payload = crate::hook::read_payload();
            view.and_then(|v| json_view(payload.as_ref(), v)).map(|v| vec![v])
        }
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

    // spec: guard-kit/SPEC.md §The shell guard — the field read's rendering per JSON type,
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

    // spec: guard-kit/SPEC.md §The shell guard — the envelopes parse, carry every
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

    // spec: guard-kit/SPEC.md §Consumer rules — the view is the reader's, read off the command as the
    // member reads it, and nothing for a tool no reader serves or a command there is not
    #[test]
    fn a_view_is_read_through_the_tool_s_reader_or_not_at_all() {
        let doc = |s: &str| serde_json::from_str::<serde_json::Value>(s).expect("the fixture must parse");
        let bash = doc(r#"{"tool_name":"Bash","tool_input":{"command":"git commit -m \"x\"\n\n"}}"#);
        assert_eq!(json_view(Some(&bash), reader::View::SqDqHd).as_deref(), Some("git commit -m DQ"));
        assert_eq!(json_view(Some(&bash), reader::View::Raw).as_deref(), Some("git commit -m \"x\""));
        let read = doc(r#"{"tool_name":"Read","tool_input":{"command":"ls"}}"#);
        assert_eq!(json_view(Some(&read), reader::View::Raw), None);
        let empty = doc(r#"{"tool_name":"Bash","tool_input":{"command":"\n"}}"#);
        assert_eq!(json_view(Some(&empty), reader::View::Raw), None);
        assert_eq!(json_view(None, reader::View::Raw), None);
        let forged = doc(r#"{"tool_name":"Bash","tool_input":{"command":"echo \u0001 'x'"}}"#);
        assert_eq!(json_view(Some(&forged), reader::View::Raw), None);
        assert_eq!(json_view(Some(&forged), reader::View::SqDqHd), None);
    }

    // spec: guard-kit/SPEC.md §The shell guard — every C0 byte but tab, line feed and carriage return
    // is refused, and the refusal names the first one
    #[test]
    fn a_control_byte_is_found_and_the_whitespace_controls_are_not() {
        assert_eq!(reader::control_byte("a\tb\r\nc"), None);
        assert_eq!(reader::control_byte("a\u{0}SQ"), Some(0));
        assert_eq!(reader::control_byte("a\u{1f}b\u{1}"), Some(0x1f));
        assert_eq!(reader::control_byte("a\u{7f}"), None);
        assert_eq!(reader::unmark(&format!("x {} y", reader::DQ_MARK)), "x DQ y");
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — every view reads back from its own spelling
    #[test]
    fn a_view_spelling_round_trips() {
        for v in reader::View::ALL {
            assert_eq!(reader::View::from_spelling(v.spelling()), Some(v));
        }
        assert_eq!(reader::View::from_spelling("dq"), None);
        assert_eq!(reader::View::from_spelling("hd sq dq"), None);
    }

    // spec: guard-kit/SPEC.md §Consumer rules — a fault rides after every key the library printed
    #[test]
    fn a_fault_context_is_the_envelope_s_last_key() {
        let a = allow_envelope_with("why", Some("fault"));
        assert!(a.starts_with(&allow_envelope("why")[..allow_envelope("why").len() - 2]));
        assert!(a.ends_with(r#","additionalContext":"fault"}}"#));
        let r = rewrite_envelope_with("c", "why", Some("fault"));
        assert!(r.ends_with(r#""updatedInput":{"command":"c"},"additionalContext":"fault"}}"#));
        assert_eq!(allow_envelope_with("why", None), allow_envelope("why"));
    }

    // spec: guard-kit/SPEC.md §The shell guard — only a usage error exits 2
    #[test]
    fn a_malformed_invocation_is_the_one_exit_2() {
        let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(json_arm(&s(&[])), 2);
        assert_eq!(json_arm(&s(&["nope", "x"])), 2);
        assert_eq!(json_arm(&s(&["rewrite", "only-one"])), 2);
        assert_eq!(json_arm(&s(&["allow", "a", "b"])), 2);
        assert_eq!(json_arm(&s(&["view"])), 2);
        assert_eq!(json_arm(&s(&["view", "body"])), 2);
        assert_eq!(json_arm(&s(&["view", "dq"])), 2);
        assert_eq!(json_arm(&s(&["allow-entries", "/no/such/settings.json"])), 0);
    }

    // spec: guard-kit/SPEC.md §The shell guard — a closing `:*` grants the head alone or the
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

    // spec: guard-kit/SPEC.md §The shell guard — a closing ` *` that is the rule's only `*`
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
