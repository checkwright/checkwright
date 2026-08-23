// spec: gate-sdk/SPEC.md §lib/gate.sh — the crate-side registry layer: the `gates.list` member
// grammar, `gate_resolve`'s declaration path, the `# graph:` field read and the couples/trigger
// expansion, in one module rather than a private copy per reader
// spec: gate-sdk/SPEC.md §The non-gate arm — a universal layer, so no `.gate` descriptor couples
// it: an edit here can change every member's verdict, which is what §check-crate-arms and the
// binary's source stamp hold, exactly as they hold `walk.rs` and `proc.rs`
use crate::fresh;
use std::path::Path;

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gates_list_members`: every line that is neither blank
// nor a comment, in file order
pub fn members(text: &str) -> Vec<String> {
    fresh::file_lines(text)
        .iter()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .map(|l| (*l).to_string())
        .collect()
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_resolve`: dirs consumer-first, `.sh` beating `.gate`
// *within* a dir, so a consumer shadowing a ported member with its own shell script still wins
pub fn resolve(name: &str, dirs: &[String]) -> Option<String> {
    for d in dirs {
        for ext in ["sh", "gate"] {
            let p = format!("{}/{}.{}", d, name, ext);
            if Path::new(&p).is_file() {
                return Some(p);
            }
        }
    }
    None
}

// spec: gate-sdk/SPEC.md §run-gates — the resolve-dir order a battery reads a member's declaration
// through: the gates dir first, then each kit's `checks/`
pub fn resolve_dirs(gates_dir: &str, kit_roots: &[String]) -> Vec<String> {
    let mut dirs = vec![gates_dir.to_string()];
    for k in kit_roots {
        dirs.push(format!("{}/checks", k.trim_end_matches('/')));
    }
    dirs
}

pub fn list_path(gates_dir: &str) -> String {
    format!("{}/gates.list", gates_dir)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the first `# graph: ` line's whitespace-split
// `<key>=<value>` tokens; an unknown token is the caller's to report, so this keeps them all
pub fn manifest_line(text: &str) -> Option<&str> {
    fresh::file_lines(text)
        .into_iter()
        .find(|l| l.starts_with("# graph: "))
}

pub fn manifest_fields(man: &str) -> Vec<(String, String)> {
    man.trim_start_matches("# graph: ")
        .split_whitespace()
        .map(|kv| match kv.split_once('=') {
            Some((k, v)) => (k.to_string(), v.to_string()),
            None => (kv.to_string(), String::new()),
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — `gate_manifest_field`: the value, empty when
// the field is absent; never an error on a missing field
pub fn field(fields: &[(String, String)], key: &str) -> String {
    fields
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .unwrap_or_default()
}

// spec: gate-sdk/SPEC.md §check-graph — gate_expand_couples_var: each `kit:<glob>` token becomes
// `<kit-root>/<glob>` for every repo-relative kit root, in root order; a non-kit token passes
// through verbatim
pub fn expand_couples(field: &str, kit_roots_rel: &[String]) -> String {
    let mut out: Vec<String> = Vec::new();
    for tok in field.split(',') {
        match tok.strip_prefix("kit:") {
            Some(glob) => {
                for r in kit_roots_rel {
                    out.push(format!("{}/{}", r.trim_end_matches('/'), glob));
                }
            }
            None => out.push(tok.to_string()),
        }
    }
    out.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_member_line_is_neither_blank_nor_a_comment() {
        let got = members("check-a\n\n  # a comment\ncheck-b\n");
        assert_eq!(got, vec!["check-a".to_string(), "check-b".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §check-graph — a `kit:` token expands once per repo-relative kit
    // root and a plain token passes through, which is the expansion both substrates must agree on
    #[test]
    fn kit_tokens_expand_and_plain_tokens_pass_through() {
        let roots = vec!["gate-sdk".to_string(), "queue-kit/".to_string()];
        assert_eq!(
            expand_couples("scripts/gates.list,kit:*.sh", &roots),
            "scripts/gates.list,gate-sdk/*.sh,queue-kit/*.sh"
        );
    }
}
