// spec: gate-sdk/SPEC.md §The hook contract — the binding table's spellings every hook member reads
// and its three envelopes, in one crate site a unit test holds to the table's member rows.

pub const EVENT_BEFORE_TOOL_CALL: &str = "PreToolUse";

pub const TOOL_BASH: &str = "Bash";
pub const TOOL_POWERSHELL: &str = "PowerShell";

pub const EVENT_NAME: &str = "hook_event_name";
pub const SESSION_ID: &str = "session_id";
pub const CHILD_ID: &str = "agent_id";
pub const CHILD_TYPE: &str = "agent_type";

pub const TOOL_NAME: &str = "tool_name";
pub const TOOL_INPUT: &str = "tool_input";
pub const COMMAND: &str = "command";
pub const BACKGROUND: &str = "run_in_background";
pub const WRITTEN_PATH: &str = "file_path";
pub const MESSAGE_TARGET: &str = "to";
pub const MESSAGE_BODY: &str = "message";
pub const DISPATCH_TYPE: &str = "subagent_type";
pub const DISPATCH_ISOLATION: &str = "isolation";
pub const DISPATCH_MODEL: &str = "model";

pub const LOOP_GUARD: &str = "stop_hook_active";
pub const LIVE_TASKS: &str = "background_tasks";
pub const TASK_ID: &str = "id";
pub const TASK_KIND: &str = "type";
pub const TASK_KIND_CHILD: &str = "subagent";
pub const TASK_KIND_SHELL: &str = "shell";
pub const TASK_STATUS: &str = "status";
pub const TASK_RUNNING: &str = "running";
pub const CHILD_TRANSCRIPT: &str = "agent_transcript_path";

pub const HOOK_SPECIFIC_OUTPUT: &str = "hookSpecificOutput";
pub const ENVELOPE_EVENT: &str = "hookEventName";
pub const ADDITIONAL_CONTEXT: &str = "additionalContext";
pub const PERMISSION_DECISION: &str = "permissionDecision";
pub const DECISION_ALLOW: &str = "allow";
pub const PERMISSION_DECISION_REASON: &str = "permissionDecisionReason";
pub const UPDATED_INPUT: &str = "updatedInput";
pub const SYSTEM_MESSAGE: &str = "systemMessage";

// spec: gate-sdk/SPEC.md §The hook contract — the member rows' spellings no member reads, held here
// so the test can hold those rows whole; read by the test alone
#[cfg_attr(not(test), allow(dead_code))]
pub const UNREAD: &[&str] = &[
    "SessionStart",
    "SubagentStop",
    "Stop",
    "Agent",
    "SendMessage",
    "ScheduleWakeup",
    "CronCreate",
    "Write",
    "Edit",
    "Glob",
    "Grep",
    "transcript_path",
    "cwd",
    "permission_mode",
    "prompt_id",
    "effort",
    "tool_use_id",
    "description",
    "timeout",
    "prompt",
    "last_assistant_message",
    "session_crons",
    "decision",
    "block",
];

// spec: gate-sdk/SPEC.md §The hook contract — the context envelope; its braces stay a format so the
// key order is the one every substrate printed
pub fn context_envelope(text: &str) -> String {
    format!(
        r#"{{"{}":{{"{}":"{}","{}":{}}}}}"#,
        HOOK_SPECIFIC_OUTPUT,
        ENVELOPE_EVENT,
        EVENT_BEFORE_TOOL_CALL,
        ADDITIONAL_CONTEXT,
        super::quote(text)
    )
}

// spec: gate-sdk/SPEC.md §The hook contract — the allow envelope, a rewrite when it carries the
// changed command, and a context after every key the guard library printed
pub fn allow_envelope(reason: &str, rewrite: Option<&str>, context: Option<&str>) -> String {
    let rewrite = rewrite.map_or(String::new(), |c| {
        format!(r#","{}":{{"{}":{}}}"#, UPDATED_INPUT, COMMAND, super::quote(c))
    });
    let context = context.map_or(String::new(), |c| {
        format!(r#","{}":{}"#, ADDITIONAL_CONTEXT, super::quote(c))
    });
    format!(
        r#"{{"{}":{{"{}":"{}","{}":"{}","{}":{}{}{}}}}}"#,
        HOOK_SPECIFIC_OUTPUT,
        ENVELOPE_EVENT,
        EVENT_BEFORE_TOOL_CALL,
        PERMISSION_DECISION,
        DECISION_ALLOW,
        PERMISSION_DECISION_REASON,
        super::quote(reason),
        rewrite,
        context
    )
}

pub fn notice_envelope(text: &str) -> String {
    format!(r#"{{"{}":{}}}"#, SYSTEM_MESSAGE, super::quote(text))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    // spec: gate-sdk/SPEC.md §The hook contract — the table rows whose spellings a member reads
    const MEMBER_ROWS: &[&str] = &[
        "- Events:",
        "- Tools:",
        "- Common payload:",
        "- Tool call:",
        "- Child turn end:",
        "- Session start:",
        "- Envelopes:",
    ];

    fn atoms(s: &str) -> impl Iterator<Item = &str> {
        s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).filter(|a| !a.is_empty())
    }

    fn table_atoms() -> BTreeSet<String> {
        let spec = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../gate-sdk/SPEC.md");
        let text = std::fs::read_to_string(&spec).expect("gate-sdk/SPEC.md must read");
        let table = text
            .split("**The binding table, Claude Code.**")
            .nth(1)
            .and_then(|t| t.split("**The measured record.**").next())
            .expect("the binding table must sit between its heading and the measured record");
        let mut seen = BTreeSet::new();
        let mut rows = 0;
        for line in table.lines().filter(|l| MEMBER_ROWS.iter().any(|r| l.starts_with(r))) {
            rows += 1;
            let line = line.split("<!--").next().unwrap_or(line);
            for span in line.split('`').skip(1).step_by(2) {
                // spec: gate-sdk/SPEC.md §The hook contract — a JSON literal is a value, never a spelling
                seen.extend(atoms(span).filter(|a| !matches!(*a, "true" | "false" | "null")).map(String::from));
            }
        }
        assert_eq!(rows, MEMBER_ROWS.len(), "each member row must be found once in the table");
        seen
    }

    fn module_spellings() -> BTreeSet<String> {
        let src = include_str!("binding.rs");
        let body = src.split("#[cfg(test)]\nmod tests").next().expect("the module body");
        let mut held: BTreeSet<String> = UNREAD.iter().map(|s| s.to_string()).collect();
        for line in body.lines().filter(|l| l.starts_with("pub const ") && l.contains(": &str = \"")) {
            held.extend(line.split('"').nth(1).map(String::from));
        }
        held
    }

    // spec: gate-sdk/SPEC.md §The hook contract — the table's member rows and this module name the
    // same spellings, so a renamed key moves this module and reds here until the table moves too
    #[test]
    fn the_member_rows_and_the_module_name_the_same_spellings() {
        let table = table_atoms();
        let module = module_spellings();
        assert!(module.contains("tool_input") && module.contains("Grep"), "the source read must reach both lists");
        let unheld: Vec<_> = table.difference(&module).collect();
        let unlisted: Vec<_> = module.difference(&table).collect();
        assert!(unheld.is_empty(), "spelled in the table, held by no constant here: {:?}", unheld);
        assert!(unlisted.is_empty(), "held here, spelled by no member row of the table: {:?}", unlisted);
    }

    // spec: gate-sdk/SPEC.md §The hook contract — the envelopes keep the byte order every substrate
    // printed
    #[test]
    fn each_envelope_keeps_its_key_order() {
        assert_eq!(
            context_envelope("a\"b"),
            r#"{"hookSpecificOutput":{"hookEventName":"PreToolUse","additionalContext":"a\"b"}}"#
        );
        assert_eq!(
            allow_envelope("r", Some("c"), Some("f")),
            r#"{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","permissionDecisionReason":"r","updatedInput":{"command":"c"},"additionalContext":"f"}}"#
        );
        assert_eq!(notice_envelope("n"), r#"{"systemMessage":"n"}"#);
    }
}
