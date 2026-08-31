// spec: delegation-kit/SPEC.md §The delegation model — the PreToolUse(Agent) dispatch-shape guard:
// D1 the fork ban, D2 the read-only isolation claim, D3 the nested-dispatch advisory.
use crate::hook;
use crate::walk;
use serde_json::Value;

const NAME: &str = "agent-dispatch-guard";

pub fn run(payload: Option<&Value>) -> i32 {
    // spec: delegation-kit/SPEC.md §The delegation model — an unresolvable roster leaves D2 with an
    // empty set and earns a note, never a skip of D1: the shell member read this one knob outside
    // the validating loader precisely so a config fault could not wedge a dispatch.
    let (roster, roster_note) = match walk::knob_array("DELEGATION_KIT_READONLY_TYPES") {
        Ok(v) => (v, String::new()),
        Err(e) => (
            Vec::new(),
            format!(
                "D2 (the read-only isolation claim) went unenforced on this dispatch: {}. ",
                e
            ),
        ),
    };
    let Some(doc) = payload.filter(|d| d.get("tool_input").is_some_and(Value::is_object)) else {
        return degraded("the hook payload did not parse, or carried no tool_input object");
    };

    let subagent_type = hook::field(Some(doc), &["tool_input", "subagent_type"]);
    let isolation = hook::field(Some(doc), &["tool_input", "isolation"]);
    let nested = doc.get("agent_id").is_some_and(|v| !v.is_null());

    match route(&subagent_type, &isolation, nested, &roster) {
        "block" if subagent_type == "fork" => return hook::block(NAME, FORK_BAN),
        "block" => return hook::block(NAME, &read_only_claim(&subagent_type)),
        _ => {}
    }

    let notes = if nested {
        format!("{}{}", roster_note, NESTED)
    } else {
        roster_note
    };
    if notes.is_empty() {
        return 0;
    }
    advise(&notes)
}

fn advise(note: &str) -> i32 {
    hook::advise(&format!("{}: {}", NAME, note))
}

// spec: delegation-kit/SPEC.md §The delegation model — fail-open-but-loud: the guard allows the
// dispatch and names the rules it could not enforce, so the reviewer knows what to check by hand.
fn degraded(reason: &str) -> i32 {
    advise(&format!(
        "allowed this dispatch WITHOUT enforcing the fork ban (D1) or the read-only isolation claim (D2) — {}. Check the dispatch by hand: no fork, and a child claimed read-only takes isolation: worktree (delegation-kit/SPEC.md §The delegation model).",
        reason
    ))
}

fn read_only_claim(subagent_type: &str) -> String {
    format!(
        "'{}' is declared a read-only dispatch type (DELEGATION_KIT_READONLY_TYPES), but this dispatch's shape grants write reach — a subagent inherits its toolset from its type whatever the prompt says, and a type carrying no Edit or Write still reaches git through its shell. Make the claim with the shape: add isolation: worktree, whose commits and index are the child's own, and which the harness best-effort auto-cleans afterwards. If this type is not in fact dispatched read-only, drop it from the roster rather than working around the rule here (delegation-kit/SPEC.md §The delegation model).",
        subagent_type
    )
}

// spec: delegation-kit/SPEC.md §The delegation model — the routing the kit's decision table
// asserts over, kept apart from the messages so the table drives the decision rather than a
// process. D1 precedes D2: the fork ban is unconditional and its message is the more specific one.
fn route(subagent_type: &str, isolation: &str, nested: bool, roster: &[String]) -> &'static str {
    if subagent_type == "fork" {
        return "block";
    }
    if !subagent_type.is_empty() && isolation != "worktree" && roster.iter().any(|t| t == subagent_type) {
        return "block";
    }
    if nested {
        return "advise";
    }
    "fallthrough"
}

const FORK_BAN: &str = "a fork inherits the dispatcher's whole context, toolset and model tier and disclaims nothing, so any narrowing this prompt states exists only as a sentence. Two lawful alternatives: dispatch a TYPED agent whose definition carries the narrower authority, brief and tier, so the narrowing is structural rather than requested; or, where the child does the same job at the same authority and you only want parallelism or its own index, dispatch that typed agent with isolation: worktree. There is no per-dispatch override — a knob here would restore the honour system this rule replaced, so the valve is unregistering the hook. The full protocol is /agent-execution (delegation-kit/SPEC.md §The delegation model).";

const NESTED: &str = "you are yourself a dispatched agent, so this call creates a grandchild with no upward channel to you: it cannot message you mid-run, and neither level knows its own address or its parent's. Give it return-value-only work, or grant it a durable path in the main checkout, named absolutely in its prompt, and read that path yourself (delegation-kit/SPEC.md §Operative residency).";

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(src: &str) -> Value {
        serde_json::from_str(src).expect("the fixture must parse")
    }

    // spec: delegation-kit/SPEC.md §The delegation model — D1 is unconditional and outranks D2, and
    // a dispatch violating neither passes silently
    #[test]
    fn the_fork_ban_outranks_the_read_only_claim() {
        let fork = payload(r#"{"tool_input":{"subagent_type":"fork","isolation":"worktree"}}"#);
        assert_eq!(
            hook::field(Some(&fork), &["tool_input", "subagent_type"]),
            "fork"
        );
        let plain = payload(r#"{"tool_input":{"subagent_type":"audit-sweep"}}"#);
        assert_eq!(
            hook::field(Some(&plain), &["tool_input", "isolation"]),
            "",
            "an absent isolation must read empty, never 'null'"
        );
        assert!(
            plain.get("agent_id").is_none(),
            "a top-level agent_id is what makes a dispatch nested"
        );
    }

    // spec: delegation-kit/SPEC.md §The delegation model — the kit's own decision table, read
    // from disk rather than transcribed into Rust literals: the table is reviewable test data and
    // a copy here would trade that review for a recompile. This test replaces its shell driver.
    #[test]
    fn the_kits_decision_table_routes_every_case() {
        let table = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../delegation-kit/usage-tests/dispatch-guard-cases.tsv");
        let text = std::fs::read_to_string(&table).expect("the kit's decision table must be read");
        // spec: delegation-kit/SPEC.md §Testing — the driver fixed one roster for the whole table
        let roster = vec!["ro-type".to_string()];
        let mut ran = 0usize;
        for line in text.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            assert!(cols.len() >= 4, "malformed case row: {}", line);
            let (want, ty, iso, nested) = (cols[0], cols[1], cols[2], cols[3]);
            let desc = cols.get(4).copied().unwrap_or("");
            fn dash(v: &str) -> &str { if v == "-" { "" } else { v } }
            // spec: delegation-kit/SPEC.md §Testing — the table's two sentinels in the type
            // column: `UNPARSEABLE` is the degraded path, which always advises without reaching
            // the routing, and a `noroster:` prefix runs the same type against an empty roster
            let got = if ty == "UNPARSEABLE" {
                "advise"
            } else if let Some(bare) = ty.strip_prefix("noroster:") {
                route(bare, dash(iso), dash(nested) == "yes", &[])
            } else {
                route(dash(ty), dash(iso), dash(nested) == "yes", &roster)
            };
            assert_eq!(got, want, "case [{}]: {}", desc, line);
            ran += 1;
        }
        assert!(ran >= 9, "only {} cases parsed — the table did not load", ran);
    }

    // spec: delegation-kit/SPEC.md §The delegation model — the degraded advisory is the same
    // envelope the enforcing paths write, so a payload that cannot be read never blocks
    #[test]
    fn an_unreadable_payload_advises_rather_than_blocks() {
        assert_eq!(degraded("the hook payload did not parse, or carried no tool_input object"), 0);
        let no_object = payload(r#"{"tool_input":"not-an-object"}"#);
        assert!(!no_object.get("tool_input").is_some_and(Value::is_object));
    }
}
