// spec: gate-sdk/SPEC.md §check-gate-fixture-coverage — every gates.list member has a fixture
// pair or a no-fixture opt-out
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

fn fixture_dir_for(member: &str, tests_dirs: &[String]) -> Option<String> {
    for t in tests_dirs {
        let p = format!("{}/{}", t, member);
        if fresh::is_dir(&p) {
            return Some(p);
        }
    }
    None
}

pub fn run(_args: &[String]) -> i32 {
    let dir = match walk::knob_scalar("GATE_SDK_GATES_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-gate-fixture-coverage: {}", e);
            return 2;
        }
    };
    let kit_roots = match walk::kit_roots_abs() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-gate-fixture-coverage: {}", e);
            return 2;
        }
    };
    let tests_dir = match walk::knob_scalar("GATE_SDK_TESTS_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-gate-fixture-coverage: {}", e);
            return 2;
        }
    };
    let mut tests_dirs = vec![tests_dir];
    tests_dirs.extend(kit_roots.iter().map(|k| format!("{}/gate-tests", k)));
    let mut resolve_dirs = vec![dir.clone()];
    resolve_dirs.extend(kit_roots.iter().map(|k| format!("{}/checks", k)));

    let list = format!("{}/gates.list", dir);
    if !Path::new(&list).is_file() {
        eprintln!("check-gate-fixture-coverage: no registry at {}", list);
        return 2;
    }
    let listing = match std::fs::read(&list) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => {
            eprintln!("check-gate-fixture-coverage: no members parsed from {}", list);
            return 2;
        }
    };
    let members = members(&listing);
    if members.is_empty() {
        eprintln!("check-gate-fixture-coverage: no members parsed from {}", list);
        return 2;
    }

    let mut neither: Vec<String> = Vec::new();
    let mut halfpair: Vec<String> = Vec::new();
    let (mut fixtured, mut optout, mut total) = (0usize, 0usize, 0usize);
    for m in &members {
        total += 1;
        if let Some(gd) = fixture_dir_for(m, &tests_dirs) {
            let good = fresh::is_dir(&format!("{}/good", gd));
            let bad = fresh::is_dir(&format!("{}/bad", gd));
            if good && bad {
                fixtured += 1;
            } else if good {
                halfpair.push(format!("{} ({}/ has good/ but no bad/)", m, gd));
            } else {
                halfpair.push(format!("{} ({}/ has bad/ but no good/)", m, gd));
            }
            continue;
        }
        let Some(src) = resolve(m, &resolve_dirs) else {
            neither.push(format!(
                "{} (no fixture pair, and source resolves in none of: {})",
                m,
                resolve_dirs.join(" ")
            ));
            continue;
        };
        let declared = std::fs::read(&src)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        if fresh::file_lines(&declared)
            .iter()
            .any(|l| l.starts_with("# no-fixture:"))
        {
            optout += 1;
        } else {
            neither.push(format!(
                "{} (no fixture pair under: {}; no '# no-fixture:' opt-out)",
                m,
                tests_dirs.join(" ")
            ));
        }
    }

    if !neither.is_empty() || !halfpair.is_empty() {
        if !neither.is_empty() {
            println!("check-gate-fixture-coverage: gates.list member(s) with neither a fixture pair");
            println!("nor a '# no-fixture:' opt-out (gate-sdk/SPEC.md §Fixture-pair discipline — a");
            println!("gate ships a good/bad fixture pair on write):");
            println!();
            for m in &neither {
                println!("  {}", m);
            }
            println!();
            println!("  help: add <tests-dir>/<gate>/{{good,bad}}/ (run by run-gate-tests.sh), OR");
            println!("        a '# no-fixture: <why>' header line for a whole-tree scanner with");
            println!("        no synthetic-dir mode (stopgap on a fixture-capable gate -> file");
            println!("        a fixture-backfill debt task and list it there).");
        }
        if !halfpair.is_empty() {
            if !neither.is_empty() {
                println!();
            }
            println!("check-gate-fixture-coverage: gates.list member(s) with a half-built fixture dir");
            println!("(a pair needs both good/ and bad/):");
            println!();
            for m in &halfpair {
                println!("  {}", m);
            }
            println!();
            println!("  help: add the missing half under <tests-dir>/<gate>/, or remove");
            println!("        the partial dir and add a '# no-fixture:' opt-out instead.");
        }
        return 1;
    }

    println!(
        "GATE-FIXTURE-COVERAGE: clean ({} members: {} fixtured, {} opted-out)",
        total, fixtured, optout
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-gate-fixture-coverage — the search is first-hit across the
    // tests dirs in order, so a consumer's own pair shadows a vendored kit's for one member
    #[test]
    fn the_pair_search_takes_the_first_tests_dir_that_holds_the_member() {
        let root = std::env::temp_dir().join(format!("cwgfc-{}", std::process::id()));
        let a = root.join("a");
        let b = root.join("b");
        std::fs::create_dir_all(a.join("check-x")).expect("mkdir");
        std::fs::create_dir_all(b.join("check-x")).expect("mkdir");
        let dirs = vec![a.display().to_string(), b.display().to_string()];
        assert_eq!(
            fixture_dir_for("check-x", &dirs).as_deref(),
            Some(format!("{}/check-x", a.display()).as_str())
        );
        assert_eq!(fixture_dir_for("check-absent", &dirs), None);
        std::fs::remove_dir_all(&root).expect("cleanup");
    }
}
