// spec: gate-sdk/SPEC.md §The hook contract — the Antigravity binding table's spellings, in one crate
// site a unit test holds to every row, and the adapter's two translations into and out of the
// shipped binding.
use crate::hook::binding;
use crate::{proc, programs};
use serde_json::{Map, Value};

pub const NAME: &str = "antigravity";

pub const EVENT_BEFORE_TOOL_CALL: &str = "PreToolUse";

pub const TOOL_SHELL: &str = "run_command";
pub const TOOL_WRITE_FILE: &str = "write_file";
pub const TOOL_EDIT_FILE: &str = "edit_file";
pub const TOOL_WRITE_TO_FILE: &str = "write_to_file";
pub const TOOL_REPLACE_FILE_CONTENT: &str = "replace_file_content";
pub const TOOL_MULTI_REPLACE_FILE_CONTENT: &str = "multi_replace_file_content";
pub const FILE_WRITE_TOOLS: [&str; 5] = [
    TOOL_WRITE_FILE,
    TOOL_EDIT_FILE,
    TOOL_WRITE_TO_FILE,
    TOOL_REPLACE_FILE_CONTENT,
    TOOL_MULTI_REPLACE_FILE_CONTENT,
];

pub const WORKSPACE_PATHS: &str = "workspacePaths";
pub const TOOL_CALL: &str = "toolCall";
pub const TOOL_CALL_NAME: &str = "name";
pub const TOOL_CALL_ARGS: &str = "args";
pub const COMMAND_LINE: &str = "CommandLine";
pub const WRITTEN_PATH: &str = "Path";
pub const TARGET_FILE: &str = "TargetFile";

pub const DECISION: &str = "decision";
pub const DECISION_DENY: &str = "deny";
pub const REASON: &str = "reason";

pub const HOOK_FILE: &str = "hooks.json";
pub const WIRING_COMMAND: &str = "command";
// spec: plugin/SPEC.md §The guards — read by the Windows posture test alone
#[cfg_attr(not(all(test, windows)), allow(dead_code))]
pub const WINDOWS_SHELL: &str = "cmd";
#[cfg_attr(not(all(test, windows)), allow(dead_code))]
pub const WINDOWS_SHELL_RUN: &str = "/c";

// spec: gate-sdk/SPEC.md §The hook contract — the rows' spellings no reader reads, held here so the
// test can hold those rows whole; read by the test alone
#[cfg_attr(not(test), allow(dead_code))]
pub const UNREAD: &[&str] = &[
    "conversationId",
    "transcriptPath",
    "artifactDirectoryPath",
    "modelName",
    "stepIdx",
    "sh",
    "timeout",
];

// spec: gate-sdk/SPEC.md §The hook contract — the shipped binding's tools an Antigravity tool stands
// for, so a rendering maps a template's matcher onto this binding's spellings
pub fn spellings(shipped_tool: &str) -> &'static [&'static str] {
    match shipped_tool {
        binding::TOOL_BASH => &[TOOL_SHELL],
        binding::TOOL_WRITE | binding::TOOL_EDIT => &FILE_WRITE_TOOLS,
        _ => &[],
    }
}

// spec: gate-sdk/SPEC.md §The hook contract — the deny envelope; its braces stay a literal so the
// key order is fixed
pub fn deny_envelope(reason: &str) -> String {
    format!(r#"{{"{}":"{}","{}":{}}}"#, DECISION, DECISION_DENY, REASON, super::quote(reason))
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the payload into the shipped binding's
// shape, `None` where the payload carries no tool call
pub fn translate_payload(payload: &Value) -> Option<Value> {
    let call = payload.get(TOOL_CALL)?.as_object()?;
    let name = call.get(TOOL_CALL_NAME).and_then(Value::as_str).unwrap_or("");
    let args = call.get(TOOL_CALL_ARGS);
    let arg = |k: &str| args.and_then(|a| a.get(k)).and_then(Value::as_str);
    let mut input = Map::new();
    let tool = if name == TOOL_SHELL {
        if let Some(c) = arg(COMMAND_LINE) {
            input.insert(binding::COMMAND.to_string(), Value::String(c.to_string()));
        }
        binding::TOOL_BASH
    } else if FILE_WRITE_TOOLS.contains(&name) {
        if let Some(p) = arg(WRITTEN_PATH).or_else(|| arg(TARGET_FILE)) {
            input.insert(binding::WRITTEN_PATH.to_string(), Value::String(p.to_string()));
        }
        binding::TOOL_WRITE
    } else {
        name
    };
    let mut out = Map::new();
    out.insert(binding::EVENT_NAME.to_string(), Value::String(binding::EVENT_BEFORE_TOOL_CALL.to_string()));
    if let Some(w) = payload.get(WORKSPACE_PATHS).and_then(|w| w.get(0)).and_then(Value::as_str) {
        out.insert(binding::WORKING_DIR.to_string(), Value::String(w.to_string()));
    }
    out.insert(binding::TOOL_NAME.to_string(), Value::String(tool.to_string()));
    out.insert(binding::TOOL_INPUT.to_string(), Value::Object(input));
    Some(Value::Object(out))
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a child's block is a deny carrying its
// stderr, and every other outcome is no decision, a grant included
pub fn translate_verdict(status: Option<i32>, stderr: &str) -> String {
    if status == Some(2) {
        deny_envelope(stderr.trim_end())
    } else {
        String::new()
    }
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the adapter: the member runs as a child of
// this binary on the translated payload, and the adapter exits 0 whatever the child did
pub fn run(member: &str, payload: &[u8]) -> i32 {
    let parsed = if payload.iter().all(u8::is_ascii_whitespace) {
        None
    } else {
        serde_json::from_slice::<Value>(payload).ok()
    };
    let input = parsed
        .as_ref()
        .and_then(translate_payload)
        .map(|v| v.to_string().into_bytes())
        .unwrap_or_default();
    let verdict = proc::self_image()
        .map_err(|e| e.to_string())
        .and_then(|exe| {
            proc::run_with_stdin(
                &programs::CHECKWRIGHT_GATES.at(exe.display().to_string()),
                &["--hook", member],
                &input,
            )
        })
        .map(|done| translate_verdict(done.code(), &String::from_utf8_lossy(done.streams().1)))
        .unwrap_or_default();
    if !verdict.is_empty() {
        println!("{}", verdict);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeSet;

    // spec: gate-sdk/SPEC.md §The hook contract — every row of the Antigravity table
    const ROWS: &[&str] = &[
        "- Events:",
        "- Tools:",
        "- Common payload:",
        "- Tool call:",
        "- Envelopes:",
        "- Variables:",
        "- Wiring:",
    ];

    fn atoms(s: &str) -> impl Iterator<Item = &str> {
        s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).filter(|a| !a.is_empty())
    }

    fn spec() -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../gate-sdk/SPEC.md");
        std::fs::read_to_string(path).expect("gate-sdk/SPEC.md must read")
    }

    fn table_atoms() -> BTreeSet<String> {
        let text = spec();
        let table = text
            .split("**The binding table, Antigravity.**")
            .nth(1)
            .and_then(|t| t.split("**Its measured record.**").next())
            .expect("the Antigravity table must sit between its heading and its measured record");
        assert_eq!(
            table.lines().filter(|l| l.starts_with("- ")).count(),
            ROWS.len(),
            "every row of the table must be named in ROWS"
        );
        let mut seen = BTreeSet::new();
        let mut rows = 0;
        for line in table.lines().filter(|l| ROWS.iter().any(|r| l.starts_with(r))) {
            rows += 1;
            let line = line.split("<!--").next().unwrap_or(line);
            for span in line.split('`').skip(1).step_by(2) {
                seen.extend(atoms(span).map(String::from));
            }
        }
        assert_eq!(rows, ROWS.len(), "each row must be found once in the table");
        seen
    }

    fn module_spellings() -> BTreeSet<String> {
        let src = include_str!("antigravity.rs");
        let body = src.split("#[cfg(test)]\nmod tests").next().expect("the module body");
        let mut held: BTreeSet<String> = UNREAD.iter().flat_map(|s| atoms(s)).map(String::from).collect();
        for line in body.lines().filter(|l| l.starts_with("pub const ") && l.contains(": &str = \"")) {
            held.extend(line.split('"').nth(1).into_iter().flat_map(atoms).map(String::from));
        }
        held.remove(NAME);
        held
    }

    // spec: gate-sdk/SPEC.md §The hook contract — the table's rows and this module name the same
    // spellings, so a renamed key moves this module and reds here until the table moves too
    #[test]
    fn the_table_rows_and_the_module_name_the_same_spellings() {
        let table = table_atoms();
        let module = module_spellings();
        assert!(module.contains("toolCall") && module.contains("stepIdx"), "the source read must reach both lists");
        let unheld: Vec<_> = table.difference(&module).collect();
        let unlisted: Vec<_> = module.difference(&table).collect();
        assert!(unheld.is_empty(), "spelled in the table, held by no constant here: {:?}", unheld);
        assert!(unlisted.is_empty(), "held here, spelled by no row of the table: {:?}", unlisted);
    }

    // spec: gate-sdk/SPEC.md §The hook contract — the shipped table's split stops before this one, so
    // `binding.rs`'s test still reads exactly its own rows
    #[test]
    fn the_shipped_tables_split_stops_before_this_table() {
        let text = spec();
        let shipped = text
            .split("**The binding table, Claude Code.**")
            .nth(1)
            .and_then(|t| t.split("**The measured record.**").next())
            .expect("the shipped table");
        assert!(!shipped.contains("**The binding table, Antigravity.**"));
        let after = text.split("**The measured record.**").nth(1).expect("the shipped record");
        assert!(after.contains("**The binding table, Antigravity.**"), "this table sits after the shipped record");
    }

    fn call(name: &str, args: Value) -> Value {
        json!({
            "conversationId": "c", "modelName": "m", "stepIdx": 2,
            "workspacePaths": ["/w/repo", "/w/other"],
            "toolCall": {"name": name, "args": args}
        })
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — each recorded tool call translates into
    // the shipped shape, the first workspace path as the working directory
    #[test]
    fn each_recorded_tool_call_translates_into_the_shipped_shape() {
        let shell = translate_payload(&call("run_command", json!({"CommandLine": "ls -la", "WaitMsBeforeAsync": 0}))).unwrap();
        assert_eq!(
            shell,
            json!({"hook_event_name": "PreToolUse", "cwd": "/w/repo", "tool_name": "Bash", "tool_input": {"command": "ls -la"}})
        );
        let write = translate_payload(&call("write_file", json!({"Path": "/w/repo/a", "Content": "x"}))).unwrap();
        assert_eq!(write["tool_name"], "Write");
        assert_eq!(write["tool_input"], json!({"file_path": "/w/repo/a"}));
        let edit = translate_payload(&call("edit_file", json!({"Path": "/w/repo/b", "Target": "t", "Replacement": "r"}))).unwrap();
        assert_eq!(edit["tool_input"], json!({"file_path": "/w/repo/b"}));
        assert_eq!(translate_payload(&json!({"conversationId": "c", "workspacePaths": ["/w"]})), None);
    }

    // spec: gate-sdk/SPEC.md §The hook contract — `TargetFile` carries the written path where
    // `Path` is absent, `multi_replace_file_content` included
    #[test]
    fn a_target_file_carries_the_written_path() {
        for tool in ["write_to_file", "replace_file_content", "multi_replace_file_content"] {
            let t = translate_payload(&call(tool, json!({"TargetFile": "/w/repo/c"}))).unwrap();
            assert_eq!(t["tool_name"], "Write", "{}", tool);
            assert_eq!(t["tool_input"], json!({"file_path": "/w/repo/c"}), "{}", tool);
        }
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — any other tool is handed on under its
    // own name with no command and no path
    #[test]
    fn an_unknown_tool_is_handed_on_with_no_command_and_no_path() {
        let t = translate_payload(&call("read_file", json!({"Path": "/w/repo/a", "CommandLine": "x"}))).unwrap();
        assert_eq!(t["tool_name"], "read_file");
        assert_eq!(t["tool_input"], json!({}));
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a block is a deny carrying the child's
    // stderr, and every other outcome, a grant envelope included, is no decision
    #[test]
    fn only_a_block_becomes_a_deny() {
        assert_eq!(
            translate_verdict(Some(2), "workflow-state-guard: no \"hand\" stamps\n"),
            r#"{"decision":"deny","reason":"workflow-state-guard: no \"hand\" stamps"}"#
        );
        assert_eq!(translate_verdict(Some(0), ""), "");
        assert_eq!(translate_verdict(Some(1), "boom"), "");
        assert_eq!(translate_verdict(None, "killed"), "");
        for shipped in [
            binding::allow_envelope("r", None, None),
            binding::allow_envelope("r", Some("c"), None),
            binding::context_envelope("advice"),
            binding::notice_envelope("notice"),
        ] {
            assert!(!shipped.is_empty());
            assert_eq!(translate_verdict(Some(0), &shipped), "", "an exit-0 envelope is no decision");
        }
    }

    // spec: gate-sdk/SPEC.md §The hook contract — the shipped tools map onto this binding's, and a
    // tool with no Antigravity spelling maps onto none
    #[test]
    fn the_shipped_tools_map_onto_this_bindings_spellings() {
        assert_eq!(spellings("Bash"), &["run_command"]);
        assert_eq!(spellings("Write"), &FILE_WRITE_TOOLS);
        assert_eq!(spellings("Edit"), &FILE_WRITE_TOOLS);
        assert!(spellings("PowerShell").is_empty() && spellings("ScheduleWakeup").is_empty());
    }
}
