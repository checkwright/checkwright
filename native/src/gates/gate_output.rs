// spec: gate-sdk/SPEC.md §check-gate-output — every gates.list member emits a machine-keyable
// success line and a help: remedy
use crate::ere::Ere;
use crate::fresh;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gates_list_members`: every line that is neither blank
// nor a comment, in file order
fn members(text: &str) -> Vec<String> {
    fresh::file_lines(text)
        .iter()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .map(|l| (*l).to_string())
        .collect()
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_resolve`: dirs consumer-first, `.sh` before `.gate`
// within a dir
fn resolve(name: &str, dirs: &[String]) -> Option<String> {
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

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_native_module`: the implementation module a
// .gate-dispatched member's rule lives in, derived from the gate name by the crate's own
// convention rather than held in a second registry that could drift from it
fn native_module(crate_dir: &str, gate: &str) -> String {
    let stem = gate.strip_prefix("check-").unwrap_or(gate).replace('-', "_");
    format!("{}/src/gates/{}.rs", crate_dir, stem)
}

fn line_matches(text: &str, re: &Ere) -> bool {
    fresh::file_lines(text).iter().any(|l| re.is_match(l))
}

pub fn run(args: &[String]) -> i32 {
    let dir = match args.first() {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("GATE_SDK_GATES_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-gate-output: {}", e);
                return 2;
            }
        },
    };
    let list = format!("{}/gates.list", dir);
    if !Path::new(&list).is_file() {
        eprintln!("check-gate-output: no registry at {}", list);
        return 2;
    }
    let listing = match std::fs::read(&list) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => {
            eprintln!("check-gate-output: no members parsed from {}", list);
            return 2;
        }
    };
    let members = members(&listing);
    if members.is_empty() {
        eprintln!("check-gate-output: no members parsed from {}", list);
        return 2;
    }

    let mut resolve_dirs = vec![dir.clone()];
    match walk::kit_roots_abs() {
        Ok(v) => resolve_dirs.extend(v.into_iter().map(|k| format!("{}/checks", k))),
        Err(e) => {
            eprintln!("check-gate-output: {}", e);
            return 2;
        }
    }

    let crate_dir = match walk::knob_scalar("GATE_SDK_NATIVE_CRATE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-gate-output: {}", e);
            return 2;
        }
    };
    // spec: gate-sdk/SPEC.md §check-gate-output — crate presence is the manifest, never the
    // directory: a build artifact under the crate path creates the directory, so a consumer
    // holding only the binary would otherwise read as carrying the crate
    let crate_manifest = format!("{}/Cargo.toml", crate_dir);

    // spec: gate-sdk/SPEC.md §check-gate-output — corpus and emitter alternation both follow the
    // declaration's substrate: a descriptor cannot hold the strings, and the shell alternation
    // matches nothing in a Rust module, so a fixed one passes vacuously
    let shell_emit = "(echo|printf)";
    let rust_emit = "(println!|eprintln!|print!|eprint!|writeln!|write!)";
    let compile = |pat: &str| -> Result<Ere, String> {
        Ere::compile(pat).map_err(|e| format!("cannot compile {}: {}", pat, e))
    };
    let res: Result<Vec<Ere>, String> = [
        format!("{}.*: clean", shell_emit),
        format!("{}.*help:", shell_emit),
        format!("{}.*: clean", rust_emit),
        format!("{}.*help:", rust_emit),
    ]
    .iter()
    .map(|p| compile(p))
    .collect();
    let res = match res {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-gate-output: {}", e);
            return 2;
        }
    };

    let mut missing: Vec<String> = Vec::new();
    let mut no_help: Vec<String> = Vec::new();
    let mut out_of_reach: Vec<String> = Vec::new();
    let mut total = 0usize;
    let mut runtime = 0usize;
    for m in &members {
        total += 1;
        let Some(src) = resolve(m, &resolve_dirs) else {
            missing.push(format!(
                "{} (source resolves in none of: {})",
                m,
                resolve_dirs.join(" ")
            ));
            continue;
        };
        let declared = std::fs::read(&src)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        // spec: gate-sdk/SPEC.md §Output contract — the source-grep is the oracle only for a
        // member no fixture case can reach; every fixtured member is asserted on its real output
        // by the runner (§run-gate-tests), which no substrate escapes
        if !fresh::file_lines(&declared)
            .iter()
            .any(|l| l.starts_with("# no-fixture:"))
        {
            runtime += 1;
            continue;
        }
        let (corpus, clean_re, help_re) = if src.ends_with(".gate") {
            let module = native_module(&crate_dir, m);
            if !Path::new(&crate_manifest).is_file() {
                out_of_reach.push(m.clone());
                continue;
            }
            if !Path::new(&module).is_file() {
                missing.push(format!(
                    "{} (dispatches to a subcommand with no module at {})",
                    m, module
                ));
                continue;
            }
            (module, &res[2], &res[3])
        } else {
            (src, &res[0], &res[1])
        };
        let text = std::fs::read(&corpus)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        if !line_matches(&text, clean_re) {
            missing.push(format!("{} (no '<NAME>: clean (…)' success line)", m));
        }
        if !line_matches(&text, help_re) {
            no_help.push(format!("{} (no 'help:' remedy line on the failure path)", m));
        }
    }

    if !missing.is_empty() || !no_help.is_empty() {
        if !missing.is_empty() {
            println!("check-gate-output: gates.list member(s) with no machine-keyable success line");
            println!("(gate-sdk/SPEC.md §Output contract — success is '^<NAME>: clean (<what>)'):");
            println!();
            for m in &missing {
                println!("  {}", m);
            }
            println!();
            println!("  help: emit exactly one success line on the exit-0 path —");
            println!("        echo \"<NAME>: clean (<what was checked>)\"");
            println!("        where <NAME> is the gate's upper-token id (e.g. KIT-README).");
        }
        if !no_help.is_empty() {
            if !missing.is_empty() {
                println!();
            }
            println!("check-gate-output: gates.list member(s) with no 'help:' remedy line");
            println!("(gate-sdk/SPEC.md §Output contract — every failure path names the fix):");
            println!();
            for m in &no_help {
                println!("  {}", m);
            }
            println!();
            println!("  help: add a remedy line on the failure path naming the concrete");
            println!("        action — echo \"  help: <do this to fix it>\" (one per failure class).");
        }
        return 1;
    }

    // spec: gate-sdk/SPEC.md §check-gate-output — an out-of-reach member is named in the success
    // line, never silently dropped from the accounting: a count that quietly shrank is
    // indistinguishable from a member that stopped being checked
    let declared = if out_of_reach.is_empty() {
        String::new()
    } else {
        format!(
            ", {} declared out of reach with no crate at {} — {}",
            out_of_reach.len(),
            crate_manifest,
            out_of_reach.join(" ")
        )
    };
    println!(
        "GATE-OUTPUT: clean ({} gates.list member(s): {} source-grepped as no-fixture members, {} asserted on real output by run-gate-tests{})",
        total,
        total - runtime - out_of_reach.len(),
        runtime,
        declared
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the module path is derived from the gate name, so a
    // renamed gate cannot leave a stale mapping behind
    #[test]
    fn the_module_path_is_derived_from_the_gate_name() {
        assert_eq!(
            native_module("native", "check-gate-output"),
            "native/src/gates/gate_output.rs"
        );
        assert_eq!(native_module("n", "kpi-x-y"), "n/src/gates/kpi_x_y.rs");
    }

    // spec: gate-sdk/SPEC.md §check-gate-output — the alternation is substrate-specific, and a
    // shell pattern read against a Rust module is what a vacuous pass looks like
    #[test]
    fn the_emitter_alternation_is_matched_per_line_and_per_substrate() {
        let shell = Ere::compile("(echo|printf).*: clean").expect("pattern");
        let rust = Ere::compile("(println!|eprintln!|print!|eprint!|writeln!|write!).*: clean")
            .expect("pattern");
        assert!(line_matches("echo \"X: clean (all)\"\n", &shell));
        assert!(!line_matches("println!(\"X: clean\");\n", &shell));
        assert!(line_matches("    println!(\"X: clean ({})\", n);\n", &rust));
        // spec: gate-sdk/SPEC.md §check-gate-output — grep is line-based, so an emitter on one
        // line and a success string on the next is not a match
        assert!(!line_matches("echo one\nX: clean (two)\n", &shell));
    }
}
