// spec: gate-sdk/SPEC.md §The hook contract — a write call's targets have one reader: the written
// path, else each path a patch carried as the command names in its file headers.
use crate::hook;
use crate::hook::binding::{COMMAND, TOOL_INPUT, WORKING_DIR, WRITTEN_PATH};
use serde_json::Value;
use std::path::Path;

const BEGIN: &str = "*** Begin Patch";
const ADD: &str = "*** Add File: ";
const UPDATE: &str = "*** Update File: ";
const DELETE: &str = "*** Delete File: ";
const MOVE: &str = "*** Move to: ";
const TARGET_HEADERS: [&str; 4] = [ADD, UPDATE, DELETE, MOVE];

pub fn written_paths(payload: Option<&Value>) -> Vec<String> {
    let cwd = hook::field(payload, &[WORKING_DIR]);
    let path = hook::field(payload, &[TOOL_INPUT, WRITTEN_PATH]);
    let targets = if path.is_empty() {
        patch_targets(&hook::field(payload, &[TOOL_INPUT, COMMAND]))
    } else {
        vec![path]
    };
    targets.into_iter().map(|t| join(&cwd, t)).collect()
}

// spec: gate-sdk/SPEC.md §The hook contract — a header line is read trimmed, as the patch tool's own
// parser reads it, so blanks around a header never hide the path it writes
fn patch_targets(command: &str) -> Vec<String> {
    let mut lines = command.lines().map(str::trim).skip_while(|l| l.is_empty());
    if lines.next() != Some(BEGIN) {
        return Vec::new();
    }
    lines
        .filter_map(|l| TARGET_HEADERS.iter().find_map(|h| l.strip_prefix(h)))
        .filter(|t| !t.is_empty())
        .map(String::from)
        .collect()
}

fn join(cwd: &str, target: String) -> String {
    if cwd.is_empty() || crate::walk::path_root(&target).is_some() {
        return target;
    }
    Path::new(cwd).join(&target).display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn patch(cwd: Option<&str>, command: &str) -> Value {
        let mut doc = json!({"tool_input": {"command": command}});
        if let Some(c) = cwd {
            doc["cwd"] = json!(c);
        }
        doc
    }

    // spec: gate-sdk/SPEC.md §The hook contract — the written path is the call's one target, and
    // wins over a patch beside it
    #[test]
    fn a_written_path_is_the_one_target_and_wins_over_a_patch() {
        let alone = json!({"tool_input": {"file_path": "/r/a.txt"}});
        assert_eq!(written_paths(Some(&alone)), vec!["/r/a.txt"]);
        let both = json!({"tool_input": {"file_path": "/r/a.txt", "command": "*** Begin Patch\n*** Add File: /r/b.txt\n+x\n*** End Patch\n"}});
        assert_eq!(written_paths(Some(&both)), vec!["/r/a.txt"]);
        assert!(written_paths(None).is_empty());
    }

    // spec: gate-sdk/SPEC.md §The hook contract — every target header names a path, a move both of
    // its own, and an environment line none
    #[test]
    fn each_target_header_names_a_path_and_the_environment_line_none() {
        let p = patch(
            None,
            "*** Begin Patch\n*** Environment ID: env\n*** Add File: /r/a\n+x\n*** Delete File: /r/d\n*** Update File: /r/u\n*** Move to: /r/m\n@@\n-o\n+n\n*** End Patch\n",
        );
        assert_eq!(written_paths(Some(&p)), vec!["/r/a", "/r/d", "/r/u", "/r/m"]);
    }

    // spec: gate-sdk/SPEC.md §The hook contract — a relative target joins the payload's working
    // directory, and stays relative to the entered tree where the payload carries none
    #[test]
    fn a_relative_target_joins_the_working_directory_where_there_is_one() {
        let cmd = "*** Begin Patch\n*** Add File: sub/a.txt\n+x\n*** End Patch\n";
        let joined = Path::new("/w/tree").join("sub/a.txt").display().to_string();
        assert_eq!(written_paths(Some(&patch(Some("/w/tree"), cmd))), vec![joined]);
        assert_eq!(written_paths(Some(&patch(None, cmd))), vec!["sub/a.txt"]);
        let abs = Path::new(env!("CARGO_MANIFEST_DIR")).display().to_string();
        let written = json!({"cwd": "/w/tree", "tool_input": {"file_path": abs}});
        assert_eq!(written_paths(Some(&written)), vec![abs]);
    }

    // spec: gate-sdk/SPEC.md §The hook contract — a command not opening with the begin line is no
    // patch, and blank lines before the begin line do not hide it
    #[test]
    fn only_a_command_opening_with_the_begin_line_is_a_patch() {
        let late = patch(None, "echo hi\n*** Begin Patch\n*** Add File: /r/a\n+x\n*** End Patch\n");
        assert!(written_paths(Some(&late)).is_empty());
        let blank = patch(None, "\n  \n*** Begin Patch\n*** Add File: /r/a\n+x\n*** End Patch\n");
        assert_eq!(written_paths(Some(&blank)), vec!["/r/a"]);
    }

    // spec: gate-sdk/SPEC.md §The hook contract — a CRLF patch and blanks around a header read as
    // the patch tool's parser reads them: the line trimmed, the path past the header verbatim
    #[test]
    fn crlf_ends_and_blanks_around_a_header_never_hide_its_path() {
        let crlf = patch(None, "*** Begin Patch\r\n*** Update File: /r/u\r\n@@\r\n-o\r\n+n\r\n*** End Patch\r\n");
        assert_eq!(written_paths(Some(&crlf)), vec!["/r/u"]);
        let padded = patch(None, "  *** Begin Patch\n  *** Add File: /r/a  \n+x\n*** Add File:  /r/b\n+y\n*** End Patch\n");
        assert_eq!(written_paths(Some(&padded)), vec!["/r/a", " /r/b"]);
    }
}
