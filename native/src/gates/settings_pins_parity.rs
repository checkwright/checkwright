// spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — the cohort's
// acceptance oracle: a differential run of the pin-path layer against the shell gate's own
// verdict, comparing verdicts rather than jq's rendered stdout
#![cfg(test)]

use crate::json::{values_equal, Path};
use crate::proc;
use serde_json::Value;

// spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — the shell
// gate's verdict classes, which is the comparison surface one layer out from jq's bytes
#[derive(Debug, PartialEq)]
enum Verdict {
    Malformed,
    Value(Value),
}

fn jq_available() -> bool {
    proc::run("jq", &["--version"])
        .map(|c| c.stdout().is_some())
        .unwrap_or(false)
}

// spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — the shell
// gate's verdict for one case, not jq's raw one: the leading-`.` pre-check, then a non-zero jq
// status as the malformed branch and the printed value as what it compares
fn shell_verdict(doc_src: &str, path: &str, file: &std::path::Path) -> Verdict {
    if !path.starts_with('.') {
        return Verdict::Malformed;
    }
    std::fs::write(file, doc_src).expect("cannot write the differential case document");
    let file_arg = file.display().to_string();
    let completed = proc::run("jq", &["-c", path, &file_arg]).expect("cannot run jq");
    let Some(out) = completed.stdout() else {
        return Verdict::Malformed;
    };
    let text = String::from_utf8_lossy(out).trim().to_string();
    match serde_json::from_str::<Value>(&text) {
        Ok(v) => Verdict::Value(v),
        Err(_) => Verdict::Malformed,
    }
}

fn crate_verdict(doc_src: &str, path: &str) -> Verdict {
    let doc: Value = serde_json::from_str(doc_src).expect("the generated document must parse");
    match Path::compile(path) {
        Err(_) => Verdict::Malformed,
        Ok(p) => match p.eval(&doc) {
            Err(_) => Verdict::Malformed,
            Ok(v) => Verdict::Value(v),
        },
    }
}

// spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — the generator
// covers path shapes rather than document shapes
const DOCUMENTS: &[&str] = &[
    r#"{"a":{"b":1},"n":null,"s":"x","arr":[10,20,30],"t":true,"f":1.0,"deep":{"k":{"j":[{"z":5}]}},"dotted":{"a.b":7},"empty":{},"nums":[1,2.0]}"#,
    r#"{"a":null}"#,
    r#"{}"#,
];

const PATHS: &[&str] = &[
    ".",
    ".a",
    ".a.b",
    ".missing",
    ".missing.deeper",
    ".n",
    ".n.k",
    ".s",
    ".s.k",
    ".arr",
    ".arr[0]",
    ".arr[2]",
    ".arr[-1]",
    ".arr[-9]",
    ".arr[9]",
    ".arr.k",
    ".a[0]",
    ".t",
    ".f",
    ".deep.k.j[0].z",
    ".dotted.\"a.b\"",
    ".dotted[\"a.b\"]",
    ".empty",
    ".nums",
    // spec: context-kit/SPEC.md §check-settings-pins — the leading-`[` case, kept because it is
    // what this arm caught: jq reads it as an array literal, the shell refuses it before jq, and
    // the compiled side must agree with the shell rather than with the tool
    "[\"a\"]",
    "[0]",
];

#[test]
fn the_pin_path_layer_agrees_with_jq_on_every_generated_case() {
    if !jq_available() {
        // spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — the
        // arm needs jq at contributor time, and says so rather than passing silently.
        panic!(
            "jq is not on PATH — this differential arm is the cohort's acceptance oracle and \
             cannot run without it; install jq, or read this arm as unverified rather than green"
        );
    }
    let dir = std::env::temp_dir().join(format!("checkwright-pin-parity-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("cannot create the differential scratch dir");
    let file = dir.join("settings.json");

    let mut compared = 0usize;
    let mut divergences: Vec<String> = Vec::new();
    for doc in DOCUMENTS {
        for path in PATHS {
            let theirs = shell_verdict(doc, path, &file);
            let ours = crate_verdict(doc, path);
            let agree = match (&theirs, &ours) {
                (Verdict::Malformed, Verdict::Malformed) => true,
                (Verdict::Value(a), Verdict::Value(b)) => values_equal(a, b),
                _ => false,
            };
            if !agree {
                divergences.push(format!(
                    "doc {} path {}: the shell gate said {:?}, the crate said {:?}",
                    doc, path, theirs, ours
                ));
            }
            compared += 1;
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        compared > 0,
        "no case was compared — an empty differential arm proves nothing"
    );
    assert!(
        divergences.is_empty(),
        "the pin-path layer diverges from jq on {} of {} cases: {:#?}",
        divergences.len(),
        compared,
        divergences
    );
}

// spec: context-kit/SPEC.md §check-settings-pins — the narrowing's own arm: a construct outside
// the path grammar is refused even where jq accepts it, the one place the two are designed to
// disagree
#[test]
fn the_grammar_refuses_filters_jq_would_accept_and_that_divergence_is_deliberate() {
    for src in [".a | .b", ".a?", ".[]", ".a[1:2]", "map(.x)"] {
        assert!(
            Path::compile(src).is_err(),
            "{} is a jq filter the pins grammar deliberately refuses",
            src
        );
    }
}
