// spec: gate-sdk/SPEC.md §The hook contract — the binding table's spellings every reader reads, a
// hook member or not, and its three envelopes, in one crate site a unit test holds to every row.

use serde_json::Value;

pub const EVENT_BEFORE_TOOL_CALL: &str = "PreToolUse";
pub const EVENT_SESSION_START: &str = "SessionStart";

pub const TOOL_BASH: &str = "Bash";
pub const TOOL_POWERSHELL: &str = "PowerShell";
pub const TOOL_WRITE: &str = "Write";
pub const TOOL_EDIT: &str = "Edit";

pub const EVENT_NAME: &str = "hook_event_name";
pub const SESSION_ID: &str = "session_id";
pub const WORKING_DIR: &str = "cwd";
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

pub const PROJECT_DIR_VAR: &str = "CLAUDE_PROJECT_DIR";
pub const SESSION_ID_VAR: &str = "CLAUDE_CODE_SESSION_ID";
pub const CHILD_SESSION_VAR: &str = "CLAUDE_CODE_CHILD_SESSION";
pub const CONFIG_DIR_VAR: &str = "CLAUDE_CONFIG_DIR";
// consumer-value-exempt: the harness's config-home default, a binding spelling and no consumer path
pub const CONFIG_HOME_DEFAULT: &str = ".claude";

pub const WIRING_ARGS: &str = "args";

pub const PROJECTS_DIR: &str = "projects";
pub const SUBAGENTS_DIR: &str = "subagents";
pub const TRANSCRIPT_SUFFIX: &str = ".jsonl";
pub const META_SUFFIX: &str = ".meta.json";
pub const PARENT_AGENT_ID: &str = "parentAgentId";
pub const TOOL_USE_BLOCK: &str = "tool_use";

pub const PERMISSIONS: &str = "permissions";
pub const ALLOW_LIST: &str = "allow";
pub const PREFIX_IDIOM: &str = ":*";

pub const HOOK_SPECIFIC_OUTPUT: &str = "hookSpecificOutput";
pub const ENVELOPE_EVENT: &str = "hookEventName";
pub const ADDITIONAL_CONTEXT: &str = "additionalContext";
pub const PERMISSION_DECISION: &str = "permissionDecision";
pub const DECISION_ALLOW: &str = "allow";
pub const PERMISSION_DECISION_REASON: &str = "permissionDecisionReason";
pub const UPDATED_INPUT: &str = "updatedInput";
pub const SYSTEM_MESSAGE: &str = "systemMessage";

// spec: gate-sdk/SPEC.md §The hook contract — the rows' spellings no reader reads, held here so the
// test can hold those rows whole; read by the test alone
#[cfg_attr(not(test), allow(dead_code))]
pub const UNREAD: &[&str] = &[
    "SubagentStop",
    "Stop",
    "Agent",
    "SendMessage",
    "ScheduleWakeup",
    "CronCreate",
    "Glob",
    "Grep",
    "transcript_path",
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
    "CLAUDE_PLUGIN_ROOT",
    "onFailure",
    "spawnDepth",
    "isFork",
    "agentType",
];

// spec: gate-sdk/SPEC.md §The hook contract — the allow list a settings document carries, `None`
// where it is absent or not a list
pub fn allow_array(doc: &Value) -> Option<&Vec<Value>> {
    doc.get(PERMISSIONS)?.get(ALLOW_LIST)?.as_array()
}

// spec: gate-sdk/SPEC.md §The hook contract — a shell rule's pattern past the tool's name and its
// opening parenthesis, the closing one left on
pub fn shell_rule_open(entry: &str) -> Option<&str> {
    entry.strip_prefix(TOOL_BASH)?.strip_prefix('(')
}

// spec: gate-sdk/SPEC.md §The hook contract — a shell rule's pattern, `None` for any other entry
pub fn shell_rule_inner(entry: &str) -> Option<&str> {
    shell_rule_open(entry)?.strip_suffix(')')
}

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

    // spec: gate-sdk/SPEC.md §The hook contract — every row of the binding table
    const ROWS: &[&str] = &[
        "- Events:",
        "- Tools:",
        "- Common payload:",
        "- Tool call:",
        "- Child turn end:",
        "- Session start:",
        "- Envelopes:",
        "- Variables:",
        "- Wiring:",
        "- Transcript:",
        "- Permission rules:",
    ];

    fn atoms(s: &str) -> impl Iterator<Item = &str> {
        s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).filter(|a| !a.is_empty())
    }

    // spec: gate-sdk/SPEC.md §The hook contract — a `<placeholder>` names a role the prose fills in,
    // never a spelling
    fn without_placeholders(span: &str) -> String {
        let mut out = String::new();
        let mut open = false;
        for c in span.chars() {
            match c {
                '<' => open = true,
                '>' if open => open = false,
                _ if !open => out.push(c),
                _ => {}
            }
        }
        out
    }

    fn table_atoms() -> (BTreeSet<String>, BTreeSet<String>) {
        let spec = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../gate-sdk/SPEC.md");
        let text = std::fs::read_to_string(&spec).expect("gate-sdk/SPEC.md must read");
        let table = text
            .split("**The binding table, Claude Code.**")
            .nth(1)
            .and_then(|t| t.split("**The measured record.**").next())
            .expect("the binding table must sit between its heading and the measured record");
        let mut seen = BTreeSet::new();
        let mut spans = BTreeSet::new();
        let mut rows = 0;
        assert_eq!(
            table.lines().filter(|l| l.starts_with("- ")).count(),
            ROWS.len(),
            "every row of the table must be named in ROWS"
        );
        for line in table.lines().filter(|l| ROWS.iter().any(|r| l.starts_with(r))) {
            rows += 1;
            let line = line.split("<!--").next().unwrap_or(line);
            for span in line.split('`').skip(1).step_by(2) {
                spans.insert(span.to_string());
                let span = without_placeholders(span);
                // spec: gate-sdk/SPEC.md §The hook contract — a JSON literal is a value, never a spelling
                seen.extend(atoms(&span).filter(|a| !matches!(*a, "true" | "false" | "null")).map(String::from));
            }
        }
        assert_eq!(rows, ROWS.len(), "each row must be found once in the table");
        (seen, spans)
    }

    fn module_spellings() -> BTreeSet<String> {
        let src = include_str!("binding.rs");
        let body = src.split("#[cfg(test)]\nmod tests").next().expect("the module body");
        let mut held: BTreeSet<String> = UNREAD.iter().flat_map(|s| atoms(s)).map(String::from).collect();
        for line in body.lines().filter(|l| l.starts_with("pub const ") && l.contains(": &str = \"")) {
            held.extend(line.split('"').nth(1).into_iter().flat_map(atoms).map(String::from));
        }
        held
    }

    // spec: gate-sdk/SPEC.md §The hook contract — the table's rows and this module name the same
    // spellings, so a renamed key moves this module and reds here until the table moves too
    #[test]
    fn the_table_rows_and_the_module_name_the_same_spellings() {
        let (table, spans) = table_atoms();
        assert!(spans.contains(PREFIX_IDIOM), "the prefix idiom is no atom, so its span is held whole");
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
