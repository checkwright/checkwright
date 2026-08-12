// spec: gate-sdk/SPEC.md §check-kit-registration — every gate_kit_roots kit is registered in
// the human-facing docs: a registry-doc row linking into each root, and a fixture-runner line
// for each root that ships gate-tests
use crate::proc;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-kit-registration — the docs resolve relative to the git
// toplevel, so the toplevel is resolved before either knob is read and a non-repo cwd is the
// misconfiguration exit rather than a doc-not-found one
fn toplevel() -> Option<String> {
    let c = proc::run("git", &["rev-parse", "--show-toplevel"]).ok()?;
    let out = c.stdout()?;
    let s = String::from_utf8_lossy(out).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn read_doc(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {} ({})", path, e))
}

pub fn run(args: &[String]) -> i32 {
    let repo_root = match toplevel() {
        Some(r) => r,
        None => {
            eprintln!(
                "check-kit-registration: not a git repository — cannot test tracked kit files"
            );
            return 2;
        }
    };

    let mut docs: Vec<String> = Vec::new();
    for (n, knob) in ["GATE_SDK_REGISTRY_DOC", "GATE_SDK_RUNNER_DOC"]
        .iter()
        .enumerate()
    {
        // spec: gate-sdk/SPEC.md §check-kit-registration — the positional argument overrides the
        // knob, and an empty positional is the shell's `${1:-…}`: unset and null both fall back
        let given = args.get(n).filter(|a| !a.is_empty()).cloned();
        let mut d = match given {
            Some(a) => a,
            None => match walk::knob_scalar(knob) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-kit-registration: {}", e);
                    return 2;
                }
            },
        };
        if !d.starts_with('/') {
            d = format!("{}/{}", repo_root, d);
        }
        docs.push(d);
    }
    let (registry_doc, runner_doc) = (docs[0].clone(), docs[1].clone());

    if !Path::new(&registry_doc).is_file() {
        eprintln!(
            "check-kit-registration: registry doc not found: {}",
            registry_doc
        );
        return 2;
    }
    if !Path::new(&runner_doc).is_file() {
        eprintln!(
            "check-kit-registration: runner doc not found: {}",
            runner_doc
        );
        return 2;
    }

    let kit_roots = match walk::kit_roots_rel() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-kit-registration: {}", e);
            return 2;
        }
    };
    if kit_roots.is_empty() {
        eprintln!("check-kit-registration: no kit roots enumerated");
        return 2;
    }

    let registry_text = match read_doc(&registry_doc) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "check-kit-registration: {} — the check could not run; treating as failure (not clean)",
                e
            );
            return 2;
        }
    };
    let runner_text = match read_doc(&runner_doc) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "check-kit-registration: {} — the check could not run; treating as failure (not clean)",
                e
            );
            return 2;
        }
    };

    let mut missing_row: Vec<String> = Vec::new();
    let mut missing_runner: Vec<String> = Vec::new();
    let mut runner_owed = 0usize;

    for root in &kit_roots {
        let r = root.trim_end_matches('/');
        // assertion A: registry row — a '](<kit>/…' link into the root in the registry doc
        if !registry_text.contains(&format!("]({}/", r)) {
            missing_row.push(r.to_string());
        }
        // assertion B: fixture-runner line — a '<kit>/gate-tests' line for each root shipping
        // gate-tests; the enumeration is git metadata, so an untracked fixture owes nothing
        let pathspec = format!("{}/gate-tests/", r);
        let completed = match proc::run("git", &["-C", &repo_root, "ls-files", "--", &pathspec]) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("check-kit-registration: {}", e);
                return 2;
            }
        };
        let listing = match completed.stdout() {
            Some(o) => o,
            None => {
                // spec: gate-sdk/SPEC.md §Fail-closed contract — the fail_closed wording the
                // shell form emitted, for the same condition and the same reason
                eprintln!(
                    "check-kit-registration: git ls-files exited {} — the check could not run; treating as failure (not clean)",
                    completed.code().unwrap_or(-1)
                );
                return 2;
            }
        };
        if !listing.is_empty() {
            runner_owed += 1;
            if !runner_text.contains(&format!("{}/gate-tests", r)) {
                missing_runner.push(r.to_string());
            }
        }
    }

    if !missing_row.is_empty() || !missing_runner.is_empty() {
        if !missing_row.is_empty() {
            println!("check-kit-registration: kit root(s) not registered in the registry doc");
            println!("({} has no '](<kit>/' link row):", registry_doc);
            for r in &missing_row {
                println!("  {}", r);
            }
        }
        if !missing_runner.is_empty() {
            if !missing_row.is_empty() {
                println!();
            }
            println!("check-kit-registration: kit root(s) shipping gate-tests but absent from the");
            println!(
                "fixture-runner battery ({} names no '<kit>/gate-tests' line):",
                runner_doc
            );
            for r in &missing_runner {
                println!("  {}", r);
            }
        }
        println!("  help: add the kit's registry row to {} (a '](<kit>/' link — bare", registry_doc);
        println!("        dir or a page under it, e.g. '](<kit>/index.md)') and,");
        println!("        for a kit that ships gate-tests, its 'run-gate-tests.sh <kit>/gate-tests'");
        println!("        line to {}, so a landed kit cannot fall out of the docs.", runner_doc);
        return 1;
    }

    println!(
        "KIT-REGISTRATION: clean ({} kit root(s) each carry a registry row; {} shipping gate-tests each name a fixture-runner line)",
        kit_roots.len(),
        runner_owed
    );
    0
}
