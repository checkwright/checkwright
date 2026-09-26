// spec: gate-sdk/SPEC.md §run-gates — `GATE_SDK_SARIF_FILE`: the battery's verdict rendered as a
// SARIF 2.1.0 log from the runner's in-memory outcomes, one rule per red member
use serde_json::{json, Value};
use std::path::Path;

pub const KNOB: &str = "GATE_SDK_SARIF_FILE";

const TOOL_NAME: &str = "checkwright-gates";

// spec: gate-sdk/SPEC.md §run-gates — what one member contributes: its name, the registry line
// naming it, its verdict tail and captured output, and its declared invariant where it carries one
pub struct Member<'a> {
    pub name: &'a str,
    pub registry_line: Option<usize>,
    pub failed: bool,
    pub tail: &'a str,
    pub output: &'a [u8],
    pub invariant: Option<Invariant>,
}

pub struct Invariant {
    pub text: String,
    pub help_uri: Option<String>,
}

// spec: gate-sdk/SPEC.md §run-gates — a finding line opening `<path>:<line>:` or
// `<path>:<line>:<col>:`, the path repo-relative and naming a regular file
fn location(line: &str, is_file: &dyn Fn(&str) -> bool) -> Option<(String, u64, Option<u64>)> {
    let mut parts = line.splitn(4, ':');
    let path = parts.next()?;
    let row: u64 = digits(parts.next()?)?;
    let rest = parts.next()?;
    let col = match parts.next() {
        Some(_) => digits(rest),
        None => None,
    };
    if path.is_empty() || row == 0 || path.trim() != path {
        return None;
    }
    if crate::walk::path_root(path).is_some() || path.split(['/', '\\']).any(|s| s == "..") {
        return None;
    }
    if !is_file(path) {
        return None;
    }
    Some((path.replace('\\', "/"), row, col.filter(|c| *c > 0)))
}

fn digits(s: &str) -> Option<u64> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

fn at(uri: &str, row: Option<u64>, col: Option<u64>) -> Value {
    let mut physical = json!({ "artifactLocation": { "uri": uri } });
    if let Some(r) = row {
        let mut region = json!({ "startLine": r });
        if let Some(c) = col {
            region["startColumn"] = json!(c);
        }
        physical["region"] = region;
    }
    json!([{ "physicalLocation": physical }])
}

fn result(rule: usize, name: &str, message: &str, locations: Value) -> Value {
    json!({
        "ruleId": name,
        "ruleIndex": rule,
        "level": "error",
        "message": { "text": message },
        "locations": locations,
    })
}

pub fn render(registry: &str, members: &[Member], is_file: &dyn Fn(&str) -> bool) -> String {
    let registry = registry.replace('\\', "/");
    let mut rules: Vec<Value> = Vec::new();
    let mut results: Vec<Value> = Vec::new();
    for m in members.iter().filter(|m| m.failed) {
        let idx = rules.len();
        let mut rule = json!({ "id": m.name });
        if let Some(inv) = &m.invariant {
            rule["shortDescription"] = json!({ "text": inv.text });
            if let Some(u) = &inv.help_uri {
                rule["helpUri"] = json!(u);
            }
        }
        rules.push(rule);
        let text = String::from_utf8_lossy(m.output);
        let mut found = false;
        for line in text.lines() {
            if let Some((path, row, col)) = location(line, is_file) {
                results.push(result(idx, m.name, line, at(&path, Some(row), col)));
                found = true;
            }
        }
        if !found {
            let block = text.trim_end_matches('\n');
            let message = if block.trim().is_empty() { m.tail.trim() } else { block };
            let row = m.registry_line.map(|n| n as u64);
            results.push(result(idx, m.name, message, at(&registry, row, None)));
        }
    }
    let log = json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": { "driver": {
                "name": TOOL_NAME,
                "rules": rules,
            }},
            "results": results,
        }],
    });
    let mut out = serde_json::to_string_pretty(&log).unwrap_or_default();
    out.push('\n');
    out
}

pub fn write(path: &str, body: &str) -> Result<(), String> {
    std::fs::write(Path::new(path), body).map_err(|e| format!("cannot write the SARIF log {}: {}", path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(p: &str) -> bool {
        matches!(p, "a/b.md" | "c.rs")
    }

    fn red<'a>(name: &'a str, line: Option<usize>, out: &'a [u8]) -> Member<'a> {
        Member {
            name,
            registry_line: line,
            failed: true,
            tail: "  FAIL: x (exit 1)",
            output: out,
            invariant: None,
        }
    }

    // spec: gate-sdk/SPEC.md §run-gates — both finding shapes place a result at their region, and a
    // path naming no regular file, an absolute path and a path with no line place none
    #[test]
    fn the_location_reader_takes_both_shapes_and_only_a_repo_relative_file() {
        assert_eq!(location("a/b.md:12: bad", &files), Some(("a/b.md".into(), 12, None)));
        assert_eq!(location("c.rs:3:7: bad", &files), Some(("c.rs".into(), 3, Some(7))));
        assert_eq!(location("c.rs:3:", &files), Some(("c.rs".into(), 3, None)));
        assert_eq!(location("nope.md:3: bad", &files), None);
        assert_eq!(location("/c.rs:3: bad", &files), None);
        assert_eq!(location("../c.rs:3: bad", &files), None);
        assert_eq!(location("c.rs: bad", &files), None);
        assert_eq!(location("c.rs:x: bad", &files), None);
        assert_eq!(location("c.rs:0: bad", &files), None);
        assert_eq!(location("  c.rs:3: bad", &files), None);
        assert_eq!(location("help: c.rs", &files), None);
    }

    fn parse(s: &str) -> Value {
        serde_json::from_str(s).unwrap()
    }

    // spec: gate-sdk/SPEC.md §run-gates — a red member with finding lines gets one result per line,
    // and one with none gets one result at its registry line carrying the captured block
    #[test]
    fn a_red_member_places_its_findings_or_falls_back_to_the_registry_line() {
        let out1 = b"a/b.md:4: wrong\nc.rs:9:2: also\nhelp: fix it\n";
        let out2 = b"something broke\nhelp: look\n";
        let mut first = red("check-one", Some(3), out1);
        first.invariant = Some(Invariant {
            text: "x/SPEC.md \u{a7}Y \u{2014} holds".into(),
            help_uri: Some("https://example.test/x/SPEC#y".into()),
        });
        let green = Member {
            name: "check-green",
            registry_line: Some(4),
            failed: false,
            tail: "  PASS: check-green",
            output: b"a/b.md:1: not a finding on green",
            invariant: None,
        };
        let log = parse(&render("scripts/gates.list", &[first, green, red("check-two", Some(7), out2)], &files));
        let run = &log["runs"][0];
        assert_eq!(log["version"], "2.1.0");
        assert_eq!(run["tool"]["driver"]["name"], TOOL_NAME);
        let rules = run["tool"]["driver"]["rules"].as_array().unwrap();
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0]["id"], "check-one");
        assert_eq!(rules[0]["helpUri"], "https://example.test/x/SPEC#y");
        assert!(rules[1].get("shortDescription").is_none());
        let results = run["results"].as_array().unwrap();
        assert_eq!(results.len(), 3);
        let loc = |i: usize| &results[i]["locations"][0]["physicalLocation"];
        assert_eq!(loc(0)["artifactLocation"]["uri"], "a/b.md");
        assert_eq!(loc(0)["region"]["startLine"], 4);
        assert_eq!(loc(1)["region"]["startColumn"], 2);
        assert_eq!(results[2]["ruleId"], "check-two");
        assert_eq!(results[2]["ruleIndex"], 1);
        assert_eq!(loc(2)["artifactLocation"]["uri"], "scripts/gates.list");
        assert_eq!(loc(2)["region"]["startLine"], 7);
        assert_eq!(results[2]["message"]["text"], "something broke\nhelp: look");
    }

    // spec: gate-sdk/SPEC.md §run-gates — a member with no registry line (a sole `--only` name) is
    // placed at the registry file with no region, and an empty capture carries the verdict tail
    #[test]
    fn an_unlisted_silent_member_lands_at_the_registry_with_its_tail() {
        let log = parse(&render("scripts/gates.list", &[red("check-solo", None, b"")], &files));
        let r = &log["runs"][0]["results"][0];
        assert!(r["locations"][0]["physicalLocation"].get("region").is_none());
        assert_eq!(r["message"]["text"], "FAIL: x (exit 1)");
    }

    // spec: gate-sdk/SPEC.md §run-gates — a green run writes the log with an empty result set
    #[test]
    fn a_green_run_is_an_empty_result_set() {
        let g = Member {
            name: "check-green",
            registry_line: Some(1),
            failed: false,
            tail: "",
            output: b"",
            invariant: None,
        };
        let log = parse(&render("scripts/gates.list", &[g], &files));
        assert_eq!(log["runs"].as_array().unwrap().len(), 1);
        assert!(log["runs"][0]["results"].as_array().unwrap().is_empty());
        assert!(log["runs"][0]["tool"]["driver"]["rules"].as_array().unwrap().is_empty());
    }

    // spec: gate-sdk/SPEC.md §run-gates — an unwritable path is refused naming it
    #[test]
    fn an_unwritable_path_is_refused_naming_it() {
        let dir = std::env::temp_dir().join(format!("sarif-unwritable.{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.display().to_string();
        let e = write(&p, "{}").unwrap_err();
        assert!(e.contains(&p), "{}", e);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
