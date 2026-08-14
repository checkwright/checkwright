// spec: context-kit/SPEC.md §check-settings-pins — every pin in the pins file holds against the
// tracked harness settings file, compared as parsed JSON values rather than as rendered strings
use crate::gates::template_registry_parity::list_members;
use crate::json::{values_equal, Path};
use crate::walk;
use serde_json::Value;

fn trim(s: &str) -> &str {
    s.trim_matches([' ', '\t', '\r'])
}

// spec: context-kit/SPEC.md §check-settings-pins — the line grammar, `<path> = <expected JSON>`,
// split at the first `=` exactly as the shell's `${line%%=*}` / `${line#*=}` pair does
fn split_pin(line: &str) -> Option<(&str, &str)> {
    let at = line.find('=')?;
    let path = trim(&line[..at]);
    let expected = trim(&line[at + 1..]);
    if path.is_empty() || expected.is_empty() {
        return None;
    }
    Some((path, expected))
}

pub fn run(args: &[String]) -> i32 {
    let mut fixture: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--fixture" => {
                fixture = args.get(i + 1).cloned();
                if fixture.is_none() {
                    eprintln!("check-settings-pins: --fixture needs a directory");
                    return 2;
                }
                i += 2;
            }
            a => {
                eprintln!("check-settings-pins: unexpected argument: {}", a);
                return 2;
            }
        }
    }

    let (settings_file, pins_file) = match &fixture {
        Some(d) => {
            if !std::path::Path::new(d).is_dir() {
                eprintln!("check-settings-pins: fixture dir not found: {}", d);
                return 2;
            }
            (format!("{}/settings.json", d), format!("{}/settings-pins.conf", d))
        }
        None => {
            let sf = match walk::knob_scalar("CONTEXT_KIT_SETTINGS_FILE") {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-settings-pins: {}", e);
                    return 2;
                }
            };
            let pf = match walk::knob_scalar("CONTEXT_KIT_SETTINGS_PINS") {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-settings-pins: {}", e);
                    return 2;
                }
            };
            (sf, pf)
        }
    };

    // spec: context-kit/SPEC.md §check-settings-pins — absent pins file is the opt-in-off state,
    // not a failure (the identity.conf precedent)
    if !std::path::Path::new(&pins_file).exists() {
        println!(
            "SETTINGS-PINS: clean (no pins file at {} — optional consumer config absent)",
            pins_file
        );
        return 0;
    }
    let pins_text = match std::fs::read_to_string(&pins_file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-settings-pins: pins file not readable: {}", pins_file);
            return 2;
        }
    };
    let settings_text = match std::fs::read_to_string(&settings_file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!(
                "check-settings-pins: settings file not readable: {}",
                settings_file
            );
            return 2;
        }
    };
    let doc: Value = match serde_json::from_str(&settings_text) {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "check-settings-pins: {} is not valid JSON",
                settings_file
            );
            return 2;
        }
    };

    let mut malformed: Vec<String> = Vec::new();
    let mut absent: Vec<String> = Vec::new();
    let mut mismatches: Vec<String> = Vec::new();
    let mut checked = 0usize;

    for line in list_members(&pins_text) {
        let Some((path_src, expected_src)) = split_pin(&line) else {
            malformed.push(line.clone());
            continue;
        };
        // spec: context-kit/SPEC.md §check-settings-pins — a pin outside the path grammar is a
        // fail-closed refusal naming pin, knob and construct, never a silent clean verdict
        let path = match Path::compile(path_src) {
            Ok(p) => p,
            Err(e) => {
                malformed.push(format!(
                    "{}  (path '{}' is outside the pins path grammar: {})",
                    line, path_src, e.construct
                ));
                continue;
            }
        };
        let expected: Value = match serde_json::from_str(expected_src) {
            Ok(v) => v,
            Err(_) => {
                malformed.push(format!(
                    "{}  (expected side '{}' is not a JSON value)",
                    line, expected_src
                ));
                continue;
            }
        };
        let actual = match path.eval(&doc) {
            Ok(v) => v,
            Err(e) => {
                malformed.push(format!("{}  ({})", line, e.message));
                continue;
            }
        };
        checked += 1;
        if values_equal(&actual, &expected) {
            continue;
        }
        // spec: context-kit/SPEC.md §check-settings-pins — a path evaluating to null is the
        // absent branch whether the key is absent or explicitly null: the shell cannot tell the
        // two apart and reproducing that conflation is the faithful port
        if actual.is_null() {
            absent.push(format!(
                "{} — pin expects {}, but {} has no such key",
                path_src, expected_src, settings_file
            ));
        } else {
            mismatches.push(format!(
                "{} — pin expects {}, settings has {}",
                path_src, expected_src, actual
            ));
        }
    }

    if !malformed.is_empty() {
        eprintln!(
            "check-settings-pins: malformed pin(s) in {} (expected '<path> = <expected JSON>', the path a CONTEXT_KIT_SETTINGS_PINS path expression):",
            pins_file
        );
        for m in &malformed {
            eprintln!("  {}", m);
        }
        return 2;
    }
    if !absent.is_empty() {
        eprintln!(
            "check-settings-pins: pinned key absent from {} — the manifest and settings have desynced:",
            settings_file
        );
        for a in &absent {
            eprintln!("  {}", a);
        }
        eprintln!(
            "  help: add the key to {}, or drop the pin from {} if the key was retired",
            settings_file, pins_file
        );
        return 2;
    }
    if !mismatches.is_empty() {
        println!(
            "check-settings-pins: {} does not match {}:",
            settings_file, pins_file
        );
        for m in &mismatches {
            println!("  {}", m);
        }
        println!("  help: restore each key in {} to its pinned value, or — if the", settings_file);
        println!("        expectation itself moved — update the matching line in {}.", pins_file);
        return 1;
    }

    println!(
        "SETTINGS-PINS: clean ({} pin(s) hold against {})",
        checked, settings_file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pin_line_splits_at_its_first_equals_and_trims_both_sides() {
        assert_eq!(split_pin("  .a.b = {\"x\":1}  "), Some((".a.b", "{\"x\":1}")));
        assert_eq!(split_pin(".a = \"x=y\""), Some((".a", "\"x=y\"")));
        assert_eq!(split_pin("no-equals"), None);
        assert_eq!(split_pin(".a ="), None);
        assert_eq!(split_pin(" = 1"), None);
    }
}
