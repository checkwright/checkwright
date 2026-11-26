// spec: gate-sdk/SPEC.md §run-gates — `GATE_SDK_SARIF_FILE`: the battery's verdict rendered as a
// SARIF 2.1.0 log from the runner's in-memory outcomes, the versioned verdict contract
use crate::output;
use crate::provenance::{Provenance, TOOL_NAME};
use serde_json::{json, Map, Value};
use std::path::Path;

pub const KNOB: &str = "GATE_SDK_SARIF_FILE";
pub const RESULTS_KNOB: &str = "GATE_SDK_SARIF_RESULTS";

// spec: gate-sdk/SPEC.md §run-gates — the verdict contract's version, raised by its migration rule
pub const CONTRACT: &str = "1.2.0";

// spec: gate-sdk/SPEC.md §run-gates — the fail result's fingerprint key, versioned with its recipe
pub const FINGERPRINT: &str = "checkwright/v1";

// spec: gate-sdk/SPEC.md §run-gates — which results the log carries: every member's, or the fail
// results and the rules they cite
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Selection {
    All,
    Failures,
}

impl Selection {
    // spec: gate-sdk/SPEC.md §run-gates — unset is `all`; any value but the two is refused
    pub fn parse(v: &str) -> Result<Selection, String> {
        match v {
            "" | "all" => Ok(Selection::All),
            "failures" => Ok(Selection::Failures),
            _ => Err(format!(
                "{}={} is not a selection — it takes `all` (the default) or `failures`",
                RESULTS_KNOB, v
            )),
        }
    }

    fn word(self) -> &'static str {
        match self {
            Selection::All => "all",
            Selection::Failures => "failures",
        }
    }
}

// spec: gate-sdk/SPEC.md §run-gates — what the run covers: the registry, an `--only` set or the
// members `--for` coupled to its paths
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scope {
    Registry,
    Only,
    For,
}

impl Scope {
    pub fn word(self) -> &'static str {
        match self {
            Scope::Registry => "registry",
            Scope::Only => "only",
            Scope::For => "for",
        }
    }
}

// spec: gate-sdk/SPEC.md §run-gates — what one member contributes: its name, the registry line
// naming it, its verdict tail and captured output, its declared invariant and the kit it resolved from
pub struct Member<'a> {
    pub name: &'a str,
    pub registry_line: Option<usize>,
    pub failed: bool,
    pub tail: &'a str,
    pub output: &'a [u8],
    pub invariant: Option<Invariant>,
    pub kit: Option<String>,
}

pub struct Invariant {
    pub text: String,
    pub help_uri: Option<String>,
}

// spec: gate-sdk/SPEC.md §run-gates — a declared omission: its registry line, its name and its
// reason token
pub struct Omission<'a> {
    pub line: usize,
    pub name: &'a str,
    pub reason: &'a str,
}

// spec: gate-sdk/SPEC.md §run-gates — the run-wide inputs, handed in so a test can fix them
pub struct Run<'a> {
    pub registry: &'a str,
    pub provenance: &'a Provenance,
    pub scope: Scope,
    pub selection: Selection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Artifact {
    File,
    Directory,
}

#[derive(Debug, PartialEq, Eq)]
enum Region {
    Whole,
    Line(u64, Option<u64>),
    Offset(u64),
}

#[derive(Debug, PartialEq, Eq)]
struct Placed {
    path: String,
    region: Region,
    text: String,
}

fn relative(path: &str) -> bool {
    !path.is_empty()
        && path.trim() == path
        && crate::walk::path_root(path).is_none()
        && !path.split(['/', '\\']).any(|s| s == "..")
}

// spec: gate-sdk/SPEC.md §Output contract — the class grammar `[A-Z][A-Z0-9_-]*`, which a deny
// list's `# rule:` id is held to as well
pub(crate) fn is_class(token: &str) -> bool {
    let mut bytes = token.bytes();
    let head = bytes.next().is_some_and(|b| b.is_ascii_uppercase());
    head && bytes.all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

// spec: gate-sdk/SPEC.md §Output contract — a finding text opening `[<CLASS>]`, followed by a space
// or the text's end
fn class(text: &str) -> Option<&str> {
    let (token, after) = text.strip_prefix('[')?.split_once(']')?;
    (is_class(token) && (after.is_empty() || after.starts_with(' '))).then_some(token)
}

// spec: gate-sdk/SPEC.md §run-gates — a finding line in one of the three shapes, its path
// repo-relative, with the text after the location prefix
fn location(line: &str, artifact: &dyn Fn(&str) -> Option<Artifact>) -> Option<Placed> {
    let trim = |t: &str| t.trim_start_matches(|c: char| c.is_ascii_whitespace()).to_string();
    let (head, rest) = line.split_once(':')?;
    if !relative(head) {
        return None;
    }
    if let Some((row, tail)) = rest.split_once(':').and_then(|(r, t)| Some((digits(r)?, t))) {
        if row == 0 || artifact(head) != Some(Artifact::File) {
            return None;
        }
        let (col, text) = match tail.split_once(':').and_then(|(c, t)| Some((digits(c)?, t))) {
            Some((c, t)) => (Some(c).filter(|c| *c > 0), t),
            None => (None, tail),
        };
        return Some(Placed { path: head.replace('\\', "/"), region: Region::Line(row, col), text: trim(text) });
    }
    let text = trim(rest);
    class(&text)?;
    if let Some((file, offset)) = head.rsplit_once('@').and_then(|(f, n)| Some((f, digits(n)?))) {
        if relative(file) && artifact(file) == Some(Artifact::File) {
            return Some(Placed { path: file.replace('\\', "/"), region: Region::Offset(offset), text });
        }
    }
    artifact(head)?;
    Some(Placed { path: head.replace('\\', "/"), region: Region::Whole, text })
}

type Seen = std::collections::HashMap<(String, String, String), usize>;

// spec: gate-sdk/SPEC.md §run-gates — a fail result's identity: its newline-terminated lines,
// the ordinal counting the run's earlier results with the same rule, `uri` and text
fn fingerprint(seen: &mut Seen, rule: &str, uri: &str, text: &str) -> Value {
    let n = seen.entry((rule.to_string(), uri.to_string(), text.to_string())).or_insert(0);
    let ordinal = *n;
    *n += 1;
    let body = format!("checkwright-result v1\n{}\n{}\n{}\n{}\n", rule, uri, text, ordinal);
    json!({ FINGERPRINT: crate::sha256::hex(body.as_bytes()) })
}

fn digits(s: &str) -> Option<u64> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

fn at(uri: &str, region: &Region) -> Value {
    let mut physical = json!({ "artifactLocation": { "uri": uri } });
    match *region {
        Region::Whole => {}
        Region::Line(r, col) => {
            let mut lines = json!({ "startLine": r });
            if let Some(c) = col {
                lines["startColumn"] = json!(c);
            }
            physical["region"] = lines;
        }
        Region::Offset(o) => physical["region"] = json!({ "byteOffset": o }),
    }
    json!([{ "physicalLocation": physical }])
}

fn registry_at(uri: &str, row: Option<u64>) -> Value {
    at(uri, &row.map_or(Region::Whole, |r| Region::Line(r, None)))
}

// spec: gate-sdk/SPEC.md §run-gates — SARIF holds `level` at `none` for every kind but `fail`
fn result(rule: usize, name: &str, kind: &str, message: &str, locations: Value) -> Value {
    json!({
        "ruleId": name,
        "ruleIndex": rule,
        "kind": kind,
        "level": if kind == "fail" { "error" } else { "none" },
        "message": { "text": message },
        "locations": locations,
    })
}

fn rule(name: &str, invariant: Option<&Invariant>, kit: Option<&str>) -> Value {
    let mut rule = json!({ "id": name });
    if let Some(inv) = invariant {
        rule["shortDescription"] = json!({ "text": inv.text });
        if let Some(u) = &inv.help_uri {
            rule["helpUri"] = json!(u);
        }
    }
    if let Some(k) = kit {
        rule["properties"] = json!({ "kit": k });
    }
    rule
}

// spec: gate-sdk/SPEC.md §run-gates — a green member's kind and message: its last clean line, a
// `not applicable: ` parenthetical making it `notApplicable`; no clean line is a `pass` carrying
// its verdict line, since the exit status is the verdict
pub(crate) fn green(output: &[u8], tail: &str) -> (&'static str, String) {
    let text = String::from_utf8_lossy(output);
    match text.lines().rev().find(|l| output::clean_line(l)) {
        Some(l) if output::not_applicable(l) => ("notApplicable", l.to_string()),
        Some(l) => ("pass", l.to_string()),
        None => ("pass", tail.trim().to_string()),
    }
}

fn driver(p: &Provenance, rules: Vec<Value>) -> Value {
    let mut d = Map::new();
    d.insert("name".into(), json!(TOOL_NAME));
    if let Some(v) = &p.tool_version {
        d.insert("version".into(), json!(v));
    }
    if let Some(v) = p.semantic_version() {
        d.insert("semanticVersion".into(), json!(v));
    }
    d.insert("rules".into(), json!(rules));
    Value::Object(d)
}

fn extensions(p: &Provenance) -> Value {
    let kits: Vec<Value> = p
        .kits
        .iter()
        .map(|k| match &k.version {
            Some(v) => json!({ "name": k.name, "version": v }),
            None => json!({ "name": k.name }),
        })
        .collect();
    json!(kits)
}

pub fn render(run: &Run, members: &[Member], omissions: &[Omission], artifact: &dyn Fn(&str) -> Option<Artifact>) -> String {
    let registry = run.registry.replace('\\', "/");
    let all = run.selection == Selection::All;
    let mut rules: Vec<Value> = Vec::new();
    let mut results: Vec<Value> = Vec::new();
    let mut pending = omissions.iter().peekable();
    let mut seen = Seen::new();
    let omit = |o: &Omission, rules: &mut Vec<Value>, results: &mut Vec<Value>| {
        if all {
            let idx = rules.len();
            rules.push(rule(o.name, None, None));
            let loc = registry_at(&registry, Some(o.line as u64));
            results.push(result(idx, o.name, "notApplicable", o.reason, loc));
        }
    };
    for m in members {
        // spec: gate-sdk/SPEC.md §run-gates — an omission sits in registry order among the members
        while let Some(o) = pending.next_if(|o| m.registry_line.is_some_and(|l| o.line < l)) {
            omit(o, &mut rules, &mut results);
        }
        if !m.failed && !all {
            continue;
        }
        let idx = rules.len();
        rules.push(rule(m.name, m.invariant.as_ref(), m.kit.as_deref()));
        let row = m.registry_line.map(|n| n as u64);
        if !m.failed {
            let (kind, message) = green(m.output, m.tail);
            results.push(result(idx, m.name, kind, &message, registry_at(&registry, row)));
            continue;
        }
        let text = String::from_utf8_lossy(m.output);
        let mut found = false;
        for line in text.lines() {
            if let Some(p) = location(line, artifact) {
                let mut fail = result(idx, m.name, "fail", line, at(&p.path, &p.region));
                fail["fingerprints"] = fingerprint(&mut seen, m.name, &p.path, &p.text);
                if let Some(c) = class(&p.text) {
                    fail["properties"] = json!({ "ruleClass": c });
                }
                results.push(fail);
                found = true;
            }
        }
        if !found {
            let block = text.trim_end_matches('\n');
            let message = if block.trim().is_empty() { m.tail.trim() } else { block };
            let mut fail = result(idx, m.name, "fail", message, registry_at(&registry, row));
            fail["fingerprints"] = fingerprint(&mut seen, m.name, &registry, "");
            results.push(fail);
        }
    }
    for o in pending {
        omit(o, &mut rules, &mut results);
    }
    let p = run.provenance;
    let mut tool = Map::new();
    tool.insert("driver".into(), driver(p, rules));
    if !p.kits.is_empty() {
        tool.insert("extensions".into(), extensions(p));
    }
    let log = json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "properties": { "verdictContract": CONTRACT },
        "runs": [{
            "tool": Value::Object(tool),
            "properties": {
                "provenance": p.block(),
                "scope": run.scope.word(),
                "results": run.selection.word(),
            },
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
pub(crate) use tests::fixed;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::Kit;

    fn files(p: &str) -> Option<Artifact> {
        match p {
            "a/b.md" | "c.rs" | "bin/tool" => Some(Artifact::File),
            "a" => Some(Artifact::Directory),
            _ => None,
        }
    }

    fn placed(path: &str, region: Region, text: &str) -> Option<Placed> {
        Some(Placed { path: path.into(), region, text: text.into() })
    }

    fn member<'a>(name: &'a str, line: Option<usize>, failed: bool, out: &'a [u8]) -> Member<'a> {
        Member {
            name,
            registry_line: line,
            failed,
            tail: if failed { "  FAIL: x (exit 1)" } else { "  PASS: x" },
            output: out,
            invariant: None,
            kit: None,
        }
    }

    fn red<'a>(name: &'a str, line: Option<usize>, out: &'a [u8]) -> Member<'a> {
        member(name, line, true, out)
    }

    fn pass<'a>(name: &'a str, line: usize, out: &'a [u8]) -> Member<'a> {
        member(name, Some(line), false, out)
    }

    fn bare() -> Provenance {
        Provenance {
            tool_version: None,
            exact: false,
            source_stamp: None,
            target: None,
            kits: Vec::new(),
            commit: None,
            clean: false,
            tree_id: None,
            gate_set: None,
        }
    }

    // spec: gate-sdk/SPEC.md §run-gates — the fixed provenance the goldens render, so they carry no
    // live stamp
    pub(crate) fn fixed() -> Provenance {
        Provenance {
            tool_version: Some("1.0.0".into()),
            exact: true,
            source_stamp: Some("0123456789abcdef0123456789abcdef01234567".into()),
            target: Some("x86_64-unknown-linux-gnu".into()),
            kits: vec![
                Kit {
                    name: "gate-sdk".into(),
                    root: "gate-sdk".into(),
                    version: Some("1.0.0".into()),
                },
                Kit {
                    name: "canon-kit".into(),
                    root: "canon-kit".into(),
                    version: Some("1.0.0".into()),
                },
            ],
            commit: Some("89abcdef0123456789abcdef0123456789abcdef".into()),
            clean: true,
            tree_id: Some("fedcba9876543210fedcba9876543210fedcba98".into()),
            gate_set: Some("0f1e2d3c4b5a69780f1e2d3c4b5a69780f1e2d3c4b5a69780f1e2d3c4b5a6978".into()),
        }
    }

    fn rendered(p: &Provenance, scope: Scope, selection: Selection, members: &[Member], omissions: &[Omission]) -> String {
        let run = Run {
            registry: "scripts/gates.list",
            provenance: p,
            scope,
            selection,
        };
        render(&run, members, omissions, &files)
    }

    fn parse(s: &str) -> Value {
        serde_json::from_str(s).unwrap()
    }

    fn all(members: &[Member]) -> Value {
        parse(&rendered(&bare(), Scope::Registry, Selection::All, members, &[]))
    }

    // spec: gate-sdk/SPEC.md §run-gates — the three finding shapes place a result at their region,
    // the two classed shapes only with their token; a path naming no artifact, an absolute path and
    // a classless path with no line place none
    #[test]
    fn the_location_reader_takes_the_three_shapes_and_only_a_repo_relative_artifact() {
        assert_eq!(location("a/b.md:12: bad", &files), placed("a/b.md", Region::Line(12, None), "bad"));
        assert_eq!(location("c.rs:3:7: bad", &files), placed("c.rs", Region::Line(3, Some(7)), "bad"));
        assert_eq!(location("c.rs:3:", &files), placed("c.rs", Region::Line(3, None), ""));
        assert_eq!(location("c.rs:3: a: b", &files), placed("c.rs", Region::Line(3, None), "a: b"));
        assert_eq!(location("c.rs:3:\t x:", &files), placed("c.rs", Region::Line(3, None), "x:"));
        assert_eq!(location("c.rs: [A] bad", &files), placed("c.rs", Region::Whole, "[A] bad"));
        assert_eq!(location("a: [NAME-2]", &files), placed("a", Region::Whole, "[NAME-2]"));
        assert_eq!(location("bin/tool@4096: [B_1] term 3", &files), placed("bin/tool", Region::Offset(4096), "[B_1] term 3"));
        assert_eq!(location("a@12: [B] x", &files), None, "an offset names a regular file");
        assert_eq!(location("nope.md:3: bad", &files), None);
        assert_eq!(location("a:3: bad", &files), None, "a line names a regular file");
        assert_eq!(location("/c.rs:3: bad", &files), None);
        assert_eq!(location("../c.rs:3: bad", &files), None);
        assert_eq!(location("c.rs: bad", &files), None);
        assert_eq!(location("c.rs: [a] bad", &files), None);
        assert_eq!(location("c.rs: [blocked-by: x]", &files), None);
        assert_eq!(location("c.rs: [A]bad", &files), None);
        assert_eq!(location("c.rs:x: bad", &files), None);
        assert_eq!(location("c.rs:0: bad", &files), None);
        assert_eq!(location("  c.rs:3: bad", &files), None);
        assert_eq!(location("help: c.rs", &files), None);
        assert_eq!(location("error: [A] c.rs", &files), None);
    }

    // spec: gate-sdk/SPEC.md §run-gates — a placed result opening a class token carries it as
    // `properties.ruleClass`; a classless or registry-line result carries no property
    #[test]
    fn a_placed_result_carries_its_rule_class() {
        let log = all(&[red("check-a", Some(1), b"a/b.md:4: [TELL-B] hedged\nc.rs:2: plain\na: [N] denied\n"), red("check-b", Some(2), b"[X] broke\n")]);
        let results = log["runs"][0]["results"].as_array().unwrap();
        let classes: Vec<Option<&str>> = results.iter().map(|r| r.pointer("/properties/ruleClass").and_then(|c| c.as_str())).collect();
        assert_eq!(classes, [Some("TELL-B"), None, Some("N"), None]);
        assert!(results[2]["locations"][0]["physicalLocation"].get("region").is_none());
        assert_eq!(results[2]["locations"][0]["physicalLocation"]["artifactLocation"]["uri"], "a");
    }

    // spec: gate-sdk/SPEC.md §run-gates — a byte-offset finding's identity excludes its offset, so the
    // same record shifted by a rebuild keeps its fingerprint
    #[test]
    fn a_shifted_byte_offset_keeps_its_fingerprint() {
        let before = all(&[red("check-strings", Some(1), b"bin/tool@1024: [TERM] list entry 2\n")]);
        let after = all(&[red("check-strings", Some(1), b"bin/tool@2311: [TERM] list entry 2\n")]);
        let region = |log: &Value| log["runs"][0]["results"][0]["locations"][0]["physicalLocation"]["region"].clone();
        assert_eq!((region(&before), region(&after)), (json!({ "byteOffset": 1024 }), json!({ "byteOffset": 2311 })));
        assert!(prints(&before)[0].is_some());
        assert_eq!(prints(&before), prints(&after));
    }

    // spec: gate-sdk/SPEC.md §run-gates — a red member with finding lines gets one `fail` result per
    // line, one with none gets one at its registry line carrying the captured block, and a green
    // member between them keeps its rule and its own result
    #[test]
    fn a_red_member_places_its_findings_or_falls_back_to_the_registry_line() {
        let out1 = b"a/b.md:4: wrong\nc.rs:9:2: also\nhelp: fix it\n";
        let out2 = b"something broke\nhelp: look\n";
        let mut first = red("check-one", Some(3), out1);
        first.invariant = Some(Invariant {
            text: "x/SPEC.md \u{a7}Y \u{2014} holds".into(),
            help_uri: Some("https://example.test/x/SPEC#y".into()),
        });
        first.kit = Some("x-kit".into());
        let green = pass("check-green", 4, b"a/b.md:1: not a finding on green");
        let log = all(&[first, green, red("check-two", Some(7), out2)]);
        let run = &log["runs"][0];
        assert_eq!(log["version"], "2.1.0");
        assert_eq!(run["tool"]["driver"]["name"], TOOL_NAME);
        let rules = run["tool"]["driver"]["rules"].as_array().unwrap();
        assert_eq!(rules.len(), 3);
        assert_eq!(rules[0]["id"], "check-one");
        assert_eq!(rules[0]["helpUri"], "https://example.test/x/SPEC#y");
        assert_eq!(rules[0]["properties"]["kit"], "x-kit");
        assert!(rules[1].get("properties").is_none(), "a consumer gates-dir member names no kit");
        assert!(rules[2].get("shortDescription").is_none());
        let results = run["results"].as_array().unwrap();
        assert_eq!(results.len(), 4);
        let loc = |i: usize| &results[i]["locations"][0]["physicalLocation"];
        assert_eq!(loc(0)["artifactLocation"]["uri"], "a/b.md");
        assert_eq!(loc(0)["region"]["startLine"], 4);
        assert_eq!((results[0]["kind"].as_str(), results[0]["level"].as_str()), (Some("fail"), Some("error")));
        assert_eq!(loc(1)["region"]["startColumn"], 2);
        assert_eq!(results[2]["ruleId"], "check-green");
        assert_eq!(results[3]["ruleId"], "check-two");
        assert_eq!(results[3]["ruleIndex"], 2);
        assert_eq!(loc(3)["artifactLocation"]["uri"], "scripts/gates.list");
        assert_eq!(loc(3)["region"]["startLine"], 7);
        assert_eq!(results[3]["message"]["text"], "something broke\nhelp: look");
    }

    // spec: gate-sdk/SPEC.md §run-gates — a member with no registry line (a sole `--only` name) is
    // placed at the registry file with no region, and an empty capture carries the verdict tail
    #[test]
    fn an_unlisted_silent_member_lands_at_the_registry_with_its_tail() {
        let log = all(&[red("check-solo", None, b"")]);
        let r = &log["runs"][0]["results"][0];
        assert!(r["locations"][0]["physicalLocation"].get("region").is_none());
        assert_eq!(r["message"]["text"], "FAIL: x (exit 1)");
    }

    fn prints(log: &Value) -> Vec<Option<String>> {
        log["runs"][0]["results"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r.pointer("/fingerprints/checkwright~1v1").and_then(|f| f.as_str()).map(String::from))
            .collect()
    }

    // spec: gate-sdk/SPEC.md §run-gates — a fail result's fingerprint is its recipe's digest, kept
    // across a line move and a registry-line capture change, an identical finding counted by
    // ordinal, and a green result carries none
    #[test]
    fn a_fail_result_carries_a_fingerprint_stable_across_reruns() {
        let want = |rule: &str, uri: &str, text: &str, n: usize| {
            Some(crate::sha256::hex(format!("checkwright-result v1\n{}\n{}\n{}\n{}\n", rule, uri, text, n).as_bytes()))
        };
        let first = all(&[
            red("check-a", Some(1), b"a/b.md:4:  bad thing\nc.rs:9:2: bad thing\na/b.md:7: bad thing\n"),
            pass("check-g", 2, b""),
            red("check-b", Some(3), b"scratch /tmp/x1 took 3ms\n"),
        ]);
        assert_eq!(
            prints(&first),
            [
                want("check-a", "a/b.md", "bad thing", 0),
                want("check-a", "c.rs", "bad thing", 0),
                want("check-a", "a/b.md", "bad thing", 1),
                None,
                want("check-b", "scripts/gates.list", "", 0),
            ]
        );
        let moved = all(&[
            red("check-a", Some(1), b"a/b.md:5: bad thing\nc.rs:30:1: bad thing\na/b.md:9:\tbad thing\n"),
            pass("check-g", 2, b""),
            red("check-b", Some(3), b"scratch /tmp/x2 took 4ms\n"),
        ]);
        assert_eq!(prints(&first), prints(&moved), "a rerun's surviving failures keep their identity");
        let failures = parse(&rendered(&bare(), Scope::Registry, Selection::Failures, &[red("check-a", Some(1), b"a/b.md:4: bad thing\n")], &[]));
        assert_eq!(prints(&failures)[0], want("check-a", "a/b.md", "bad thing", 0));
    }

    // spec: gate-sdk/SPEC.md §run-gates — a green member is a `pass` or a `notApplicable` at its
    // registry line, its last clean line the message, at level `none`; one that printed no clean
    // line is a `pass` carrying its verdict line
    #[test]
    fn a_green_member_is_a_pass_or_not_applicable_by_its_clean_line() {
        let log = all(&[
            pass("check-a", 1, b"note\nA: clean (3 files)\n"),
            pass("check-b", 2, b"B: clean (not applicable: B_GLOBS is unset)\n"),
            pass("check-c", 3, b""),
        ]);
        let results = log["runs"][0]["results"].as_array().unwrap();
        let kinds: Vec<&str> = results.iter().map(|r| r["kind"].as_str().unwrap()).collect();
        assert_eq!(kinds, ["pass", "notApplicable", "pass"]);
        assert!(results.iter().all(|r| r["level"] == "none"));
        assert_eq!(results[0]["message"]["text"], "A: clean (3 files)");
        assert_eq!(results[1]["locations"][0]["physicalLocation"]["region"]["startLine"], 2);
        assert_eq!(results[2]["message"]["text"], "PASS: x");
    }

    // spec: gate-sdk/SPEC.md §run-gates — a declared omission is a `notApplicable` result at its own
    // registry line, in registry order, with its reason token and a rule carrying no invariant
    #[test]
    fn an_omission_is_a_not_applicable_result_at_its_own_line() {
        let omitted = [Omission {
            line: 2,
            name: "check-off",
            reason: "toolchain",
        }];
        let members = [pass("check-a", 1, b""), pass("check-b", 3, b"")];
        let log = parse(&rendered(&bare(), Scope::Registry, Selection::All, &members, &omitted));
        let results = log["runs"][0]["results"].as_array().unwrap();
        let ids: Vec<&str> = results.iter().map(|r| r["ruleId"].as_str().unwrap()).collect();
        assert_eq!(ids, ["check-a", "check-off", "check-b"]);
        assert_eq!(results[1]["kind"], "notApplicable");
        assert_eq!(results[1]["message"]["text"], "toolchain");
        assert_eq!(results[1]["locations"][0]["physicalLocation"]["region"]["startLine"], 2);
        let rules = log["runs"][0]["tool"]["driver"]["rules"].as_array().unwrap();
        assert_eq!(rules[1], json!({ "id": "check-off" }));
        assert_eq!(log["runs"][0]["properties"]["scope"], "registry");
    }

    // spec: gate-sdk/SPEC.md §run-gates — an `--only` run reports only what it selected, no omission
    #[test]
    fn an_only_run_carries_no_omission() {
        let log = parse(&rendered(&bare(), Scope::Only, Selection::All, &[pass("check-a", 1, b"")], &[]));
        assert_eq!(log["runs"][0]["results"].as_array().unwrap().len(), 1);
        assert_eq!(log["runs"][0]["properties"]["scope"], "only");
    }

    // spec: gate-sdk/SPEC.md §run-gates — `failures` keeps the fail results and the rules they cite,
    // so a green run's failures log is an empty result set
    #[test]
    fn the_failures_selection_keeps_fail_results_and_their_rules_only() {
        let omitted = [Omission {
            line: 2,
            name: "check-off",
            reason: "toolchain",
        }];
        let members = [pass("check-a", 1, b"A: clean (x)\n"), red("check-b", Some(3), b"broke\n")];
        let log = parse(&rendered(&bare(), Scope::Registry, Selection::Failures, &members, &omitted));
        let run = &log["runs"][0];
        let rules = run["tool"]["driver"]["rules"].as_array().unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0]["id"], "check-b");
        assert_eq!(run["results"][0]["ruleIndex"], 0);
        assert_eq!(run["results"].as_array().unwrap().len(), 1);
        assert_eq!(run["properties"]["results"], "failures");
        let green = parse(&rendered(&bare(), Scope::Registry, Selection::Failures, &[pass("check-a", 1, b"")], &[]));
        assert!(green["runs"][0]["results"].as_array().unwrap().is_empty());
        assert_eq!(green["properties"]["verdictContract"], CONTRACT);
    }

    // spec: gate-sdk/SPEC.md §run-gates — the selection knob takes `all` or `failures`, and refuses
    // any other value naming itself and both values
    #[test]
    fn a_selection_outside_the_two_values_is_refused() {
        assert_eq!(Selection::parse(""), Ok(Selection::All));
        assert_eq!(Selection::parse("all"), Ok(Selection::All));
        assert_eq!(Selection::parse("failures"), Ok(Selection::Failures));
        let e = Selection::parse("red").unwrap_err();
        assert!(e.contains(RESULTS_KNOB) && e.contains("`all`") && e.contains("`failures`"), "{}", e);
    }

    // spec: gate-sdk/SPEC.md §run-gates — an unfilled provenance leaves the native mirrors absent too
    #[test]
    fn an_unfilled_provenance_mirrors_nothing() {
        let log = all(&[pass("check-a", 1, b"")]);
        let tool = &log["runs"][0]["tool"];
        assert!(tool["driver"].get("version").is_none());
        assert!(tool["driver"].get("semanticVersion").is_none());
        assert!(tool.get("extensions").is_none());
        assert_eq!(log["runs"][0]["properties"]["provenance"]["tool"], json!({ "name": TOOL_NAME }));
    }

    // spec: gate-sdk/SPEC.md §run-gates — the native mirrors: `version` always, `semanticVersion`
    // only for an exact stamp, one extension per kit
    #[test]
    fn the_native_fields_mirror_the_block() {
        let mut p = fixed();
        p.tool_version = Some("0.36.0-6-gabc".into());
        p.exact = false;
        p.kits[1].version = None;
        let log = parse(&rendered(&p, Scope::For, Selection::All, &[pass("check-a", 1, b"")], &[]));
        let tool = &log["runs"][0]["tool"];
        assert_eq!(tool["driver"]["version"], "0.36.0-6-gabc");
        assert!(tool["driver"].get("semanticVersion").is_none());
        assert_eq!(tool["extensions"], json!([{ "name": "gate-sdk", "version": "1.0.0" }, { "name": "canon-kit" }]));
        p.tool_version = Some("0.36.0".into());
        p.exact = true;
        let log = parse(&rendered(&p, Scope::For, Selection::All, &[pass("check-a", 1, b"")], &[]));
        assert_eq!(log["runs"][0]["tool"]["driver"]["semanticVersion"], "0.36.0");
        assert_eq!(log["runs"][0]["properties"]["scope"], "for");
    }

    fn golden_red() -> [Member<'static>; 3] {
        let mut a = red(
            "check-findings",
            Some(1),
            b"a/b.md:4: [A] the rule is broken here\na: [B] a denied name, list entry 1\nbin/tool@1024: [C] a denied string, list entry 2\nhelp: fix it\n",
        );
        a.invariant = Some(Invariant {
            text: "gate-sdk/SPEC.md \u{a7}Example \u{2014} findings name their line".into(),
            help_uri: None,
        });
        a.kit = Some("gate-sdk".into());
        let mut b = red("check-silent", Some(2), b"the corpus could not be read\n");
        b.kit = Some("canon-kit".into());
        let mut c = pass("check-green", 3, b"CHECK-GREEN: clean (4 files)\n");
        c.kit = Some("gate-sdk".into());
        [a, b, c]
    }

    // spec: gate-sdk/SPEC.md §run-gates — the four published goldens, byte for byte: a renderer
    // change moving one is a visible contract diff, landed with the golden edited
    #[test]
    fn the_goldens_hold_byte_for_byte() {
        let p = fixed();
        let mut a = pass("check-alpha", 1, b"CHECK-ALPHA: clean (12 files)\n");
        a.kit = Some("gate-sdk".into());
        a.invariant = Some(Invariant {
            text: "gate-sdk/SPEC.md \u{a7}Example \u{2014} every file holds the rule".into(),
            help_uri: Some("https://example.test/gate-sdk/SPEC#example".into()),
        });
        let mut b = pass("check-beta", 2, b"CHECK-BETA: clean (3 sections)\n");
        b.kit = Some("canon-kit".into());
        let mut c = pass("check-gamma", 3, b"CHECK-GAMMA: clean (not applicable: CHECK_GAMMA_GLOBS is unset)\n");
        c.kit = Some("canon-kit".into());
        let green = rendered(&p, Scope::Registry, Selection::All, &[a, b, c], &[]);
        let red = rendered(&p, Scope::Registry, Selection::All, &golden_red(), &[]);
        let failures = rendered(&p, Scope::Registry, Selection::Failures, &golden_red(), &[]);
        let mut a = pass("check-alpha", 1, b"CHECK-ALPHA: clean (12 files)\n");
        a.kit = Some("gate-sdk".into());
        let omitted = [Omission {
            line: 2,
            name: "check-shellcheck",
            reason: "toolchain",
        }];
        let omitted = rendered(&p, Scope::Registry, Selection::All, &[a], &omitted);
        let goldens = [
            (green, include_str!("../../docs/contracts/verdict/v1/green.sarif")),
            (red, include_str!("../../docs/contracts/verdict/v1/red.sarif")),
            (omitted, include_str!("../../docs/contracts/verdict/v1/omitted.sarif")),
            (failures, include_str!("../../docs/contracts/verdict/v1/failures.sarif")),
        ];
        for (i, (rendered, golden)) in goldens.iter().enumerate() {
            assert_eq!(rendered, golden, "golden {} moved", i);
        }
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
