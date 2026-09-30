// spec: delegation-kit/SPEC.md §The delegation model — the PreToolUse(Agent) dispatch-shape guard:
// D1 the fork ban, D2 the read-only isolation claim, D4 default isolation, D5 the chosen tier, D6
// the bound tier, D3 the nested-dispatch advisory.
use crate::gates::agent_tier_explicit;
use crate::hook;
use crate::walk;
use serde_json::Value;
use std::path::Path;

const NAME: &str = "agent-dispatch-guard";

// spec: delegation-kit/SPEC.md §The delegation model — the configuration the routing reads: D2's
// roster, D4's roster, D5's switch, D5's definition lookup, a function so the decision table
// can fix it without a directory, and D6's tier binding
struct Config<'a> {
    readonly: &'a [String],
    mutating: &'a [String],
    require_tier: bool,
    tier_defined: &'a dyn Fn(&str) -> bool,
    binding: &'a [String],
}

// spec: delegation-kit/SPEC.md §The delegation model — one knob read: its value, or empty with a
// note naming the rule it leaves unenforced, so a config fault costs that rule and never the dispatch
fn read_array(knob: &str, rule: &str, notes: &mut String) -> Vec<String> {
    match walk::knob_array(knob) {
        Ok(v) => v,
        Err(e) => {
            notes.push_str(&format!("{} went unenforced on this dispatch: {}. ", rule, e));
            Vec::new()
        }
    }
}

pub fn run(payload: Option<&Value>) -> i32 {
    // spec: delegation-kit/SPEC.md §The delegation model — an unresolvable roster leaves its rule
    // with an empty set and earns a note, never a skip of D1, so a config fault cannot wedge a dispatch.
    let mut notes = String::new();
    let readonly = read_array("DELEGATION_KIT_READONLY_TYPES", "D2 (the read-only isolation claim)", &mut notes);
    let mutating = read_array("DELEGATION_KIT_MUTATING_TYPES", "D4 (default isolation)", &mut notes);
    let tier = walk::knob_scalar("DELEGATION_KIT_REQUIRE_TIER")
        .and_then(|t| walk::knob_scalar("DELEGATION_KIT_AGENT_DIR").map(|d| (t, d)));
    let (require_tier, agent_dir) = match tier {
        Ok((t, d)) => (t == "on", d),
        Err(e) => {
            notes.push_str(&format!("D5 (the chosen tier) went unenforced on this dispatch: {}. ", e));
            (false, String::new())
        }
    };
    let binding = read_array("DELEGATION_KIT_TIER_MODEL", "D6 (the bound tier)", &mut notes);
    let Some(doc) = payload.filter(|d| d.get("tool_input").is_some_and(Value::is_object)) else {
        return degraded("the hook payload did not parse, or carried no tool_input object");
    };

    let subagent_type = hook::field(Some(doc), &["tool_input", "subagent_type"]);
    let isolation = hook::field(Some(doc), &["tool_input", "isolation"]);
    let model = hook::field(Some(doc), &["tool_input", "model"]);
    let nested = doc.get("agent_id").is_some_and(|v| !v.is_null());
    let lookup = |ty: &str| tier_defined(&agent_dir, ty);
    let cfg = Config {
        readonly: &readonly,
        mutating: &mutating,
        require_tier,
        tier_defined: &lookup,
        binding: &binding,
    };

    match route(&subagent_type, &isolation, nested, &model, &cfg) {
        "fork" => return hook::block(NAME, FORK_BAN),
        "read-only" => return hook::block(NAME, &read_only_claim(&subagent_type)),
        "isolation" => return hook::block(NAME, &default_isolation(&subagent_type)),
        "tier" => return hook::block(NAME, &chosen_tier(&subagent_type, &agent_dir)),
        "bound" => return hook::block(NAME, &bound_tier(&model, &binding)),
        _ => {}
    }

    if nested {
        notes.push_str(NESTED);
    }
    if notes.is_empty() {
        return 0;
    }
    advise(&notes)
}

// spec: delegation-kit/SPEC.md §The delegation model — D5's lookup: a definition under the agent
// dir whose declared type is this one and whose frontmatter states `model:`
fn tier_defined(agent_dir: &str, subagent_type: &str) -> bool {
    let dir = agent_dir.trim_end_matches('/');
    if subagent_type.is_empty() || !Path::new(dir).is_dir() {
        return false;
    }
    let Ok(files) = walk::find_files(Path::new(dir), &["md"]) else {
        return false;
    };
    files.iter().any(|f| {
        let Ok(text) = std::fs::read_to_string(f) else {
            return false;
        };
        let stem = f.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        agent_tier_explicit::defined_type(&text, stem) == subagent_type
            && agent_tier_explicit::has_explicit_model(&text)
    })
}

fn advise(note: &str) -> i32 {
    hook::advise(&format!("{}: {}", NAME, note))
}

// spec: delegation-kit/SPEC.md §The delegation model — fail-open-but-loud: the guard allows the
// dispatch and names the rules it could not enforce, so the reviewer knows what to check by hand.
fn degraded(reason: &str) -> i32 {
    advise(&format!(
        "allowed this dispatch WITHOUT enforcing the fork ban (D1), the read-only isolation claim (D2), default isolation (D4), the chosen tier (D5) or the bound tier (D6) — {}. Check the dispatch by hand: no fork, a child claimed read-only or of an undeclared type takes isolation: worktree, the dispatch names its model unless its type's definition states one, and a named model is one of the consumer's bound tiers (delegation-kit/SPEC.md §The delegation model).",
        reason
    ))
}

fn type_label(subagent_type: &str) -> String {
    if subagent_type.is_empty() {
        "the harness's default type".to_string()
    } else {
        format!("'{}'", subagent_type)
    }
}

fn read_only_claim(subagent_type: &str) -> String {
    format!(
        "'{}' is declared a read-only dispatch type (DELEGATION_KIT_READONLY_TYPES), but this dispatch's shape grants write reach — a subagent inherits its toolset from its type whatever the prompt says, and a type carrying no Edit or Write still reaches git through its shell. Make the claim with the shape: add isolation: worktree, whose commits and index are the child's own, and which the harness best-effort auto-cleans afterwards. If this type is not in fact dispatched read-only, drop it from the roster rather than working around the rule here (delegation-kit/SPEC.md §The delegation model).",
        subagent_type
    )
}

fn default_isolation(subagent_type: &str) -> String {
    format!(
        "{} is not a declared mutating dispatch type (DELEGATION_KIT_MUTATING_TYPES), and this dispatch is not isolated: a type nobody declared mutating is confined to a worktree by default, so reaching past isolation takes a named choice. Two lawful shapes: add isolation: worktree, where the child reads the main checkout by absolute path and appends its journal by shell under the main checkout's scratch dir; or dispatch a declared mutating type (delegation-kit/SPEC.md §The delegation model).",
        type_label(subagent_type)
    )
}

fn chosen_tier(subagent_type: &str, agent_dir: &str) -> String {
    format!(
        "this dispatch names no model, and {} has no definition under {} stating model:, so the child would inherit the dispatcher's tier by default rather than by choice. Name a model for this dispatch, or dispatch a type whose definition states one: selection is affirmative (delegation-kit/templates/agent-execution.md, Match the dispatched model and effort to the unit's shape).",
        type_label(subagent_type),
        agent_dir
    )
}

// spec: delegation-kit/SPEC.md §The delegation model — D6's message names the bound values, which
// are the consumer's config rather than kit literals
fn bound_tier(model: &str, binding: &[String]) -> String {
    format!(
        "this dispatch names model '{}', which matches none of the consumer's bound tiers ({}), so it would choose around the tier binding rather than among its tiers. Name one of the bound values (bash gate-sdk/bin/run-gates.sh --emit knob-values DELEGATION_KIT_TIER_MODEL), or, where the class you need is pinned to an exact model id, dispatch a type whose definition declares that class: a per-dispatch model is an alias only (delegation-kit/SPEC.md §The tier binding).",
        model,
        binding.join(", ")
    )
}

// spec: delegation-kit/SPEC.md §The delegation model — the routing the kit's decision table
// asserts over, kept apart from the messages so the table drives the decision rather than a
// process; the rule order is that section's
fn route(subagent_type: &str, isolation: &str, nested: bool, model: &str, cfg: &Config) -> &'static str {
    let model_named = !model.is_empty();
    if subagent_type == "fork" {
        return "fork";
    }
    let isolated = isolation == "worktree";
    if !subagent_type.is_empty() && !isolated && cfg.readonly.iter().any(|t| t == subagent_type) {
        return "read-only";
    }
    if !isolated && !cfg.mutating.is_empty() && !cfg.mutating.iter().any(|t| t == subagent_type) {
        return "isolation";
    }
    if cfg.require_tier && !model_named && !(cfg.tier_defined)(subagent_type) {
        return "tier";
    }
    if !cfg.binding.is_empty()
        && model_named
        && !crate::tier::values(cfg.binding).iter().any(|v| crate::tier::matches(v, model))
    {
        return "bound";
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
        assert_eq!(
            hook::field(Some(&plain), &["tool_input", "model"]),
            "",
            "an absent model must read empty, so D5 reads it as unnamed"
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
        // spec: delegation-kit/SPEC.md §Testing — the driver fixes the configuration: D2's roster
        // everywhere, D4's roster, D5's switch and one tier-stating definition behind `armed:`, and
        // D6's binding behind `bound:`
        let readonly = vec!["ro-type".to_string()];
        let mutating = vec!["mut-type".to_string()];
        let binding = vec!["judgment=big-model".to_string(), "mechanical=small".to_string()];
        let tiered = |t: &str| t == "tiered-type";
        let none = |_: &str| false;
        let quiet = Config { readonly: &readonly, mutating: &[], require_tier: false, tier_defined: &none, binding: &[] };
        let bare = Config { readonly: &[], mutating: &[], require_tier: false, tier_defined: &none, binding: &[] };
        let armed =
            Config { readonly: &readonly, mutating: &mutating, require_tier: true, tier_defined: &tiered, binding: &[] };
        let bound =
            Config { readonly: &readonly, mutating: &[], require_tier: false, tier_defined: &none, binding: &binding };
        let mut ran = 0usize;
        for line in text.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            assert!(cols.len() >= 5, "malformed case row: {}", line);
            let (want, ty, iso, nested, model) = (cols[0], cols[1], cols[2], cols[3], cols[4]);
            let desc = cols.get(5).copied().unwrap_or("");
            fn dash(v: &str) -> &str { if v == "-" { "" } else { v } }
            // spec: delegation-kit/SPEC.md §Testing — the table's four type-column sentinels;
            // `UNPARSEABLE` is the degraded path, which advises without reaching the routing
            let got = if ty == "UNPARSEABLE" {
                "advise"
            } else {
                let (bare_ty, cfg) = if let Some(t) = ty.strip_prefix("noroster:") {
                    (t, &bare)
                } else if let Some(t) = ty.strip_prefix("armed:") {
                    (t, &armed)
                } else if let Some(t) = ty.strip_prefix("bound:") {
                    (t, &bound)
                } else {
                    (ty, &quiet)
                };
                match route(dash(bare_ty), dash(iso), dash(nested) == "yes", dash(model), cfg) {
                    "fork" | "read-only" | "isolation" | "tier" | "bound" => "block",
                    other => other,
                }
            };
            assert_eq!(got, want, "case [{}]: {}", desc, line);
            ran += 1;
        }
        assert!(ran >= 16, "only {} cases parsed — the table did not load", ran);
    }

    // spec: delegation-kit/SPEC.md §The delegation model — the degraded advisory is the same
    // envelope the enforcing paths write, so a payload that cannot be read never blocks
    #[test]
    fn an_unreadable_payload_advises_rather_than_blocks() {
        assert_eq!(degraded("the hook payload did not parse, or carried no tool_input object"), 0);
        let no_object = payload(r#"{"tool_input":"not-an-object"}"#);
        assert!(!no_object.get("tool_input").is_some_and(Value::is_object));
    }

    // spec: delegation-kit/SPEC.md §The delegation model — D5's lookup reads a definition's
    // declared name before its file stem, and a definition stating no model does not count
    #[test]
    fn a_type_counts_as_tiered_only_through_a_definition_stating_model() {
        assert_eq!(agent_tier_explicit::defined_type("---\nname: sweep\nmodel: x\n---\n", "file"), "sweep");
        assert_eq!(agent_tier_explicit::defined_type("---\nmodel: x\n---\n", "file"), "file");
        assert!(!agent_tier_explicit::has_explicit_model("---\nname: sweep\n---\n"));
        assert!(!tier_defined("/nonexistent-agent-dir-checkwright", "sweep"));
        assert!(!tier_defined(".claude/agents", ""));
    }
}
