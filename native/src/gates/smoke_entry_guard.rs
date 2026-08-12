// spec: gate-sdk/SPEC.md §check-smoke-entry-guard — every mutating smoke script (install.sh,
// violation.sh) carries the ${SMOKE_KIT_ROOT:?} entry-point guard so a bare run refuses instead
// of mutating the caller's tree
use crate::proc;
use crate::walk;
use std::path::Path;

const GUARD: &str = "${SMOKE_KIT_ROOT:?";

// spec: gate-sdk/SPEC.md §check-smoke-entry-guard — bare: sweep gate_kit_roots against the git
// toplevel; a positional root resolves relative kit roots against a fixture tree instead
pub fn scan_root(args: &[String], gate: &str) -> Option<String> {
    if let Some(r) = args.first().filter(|a| !a.is_empty()) {
        return Some(r.clone());
    }
    let c = proc::run("git", &["rev-parse", "--show-toplevel"]).ok()?;
    let out = c.stdout()?;
    let s = String::from_utf8_lossy(out).trim().to_string();
    if s.is_empty() {
        eprintln!("{}: not a git repository and no root argument", gate);
        return None;
    }
    Some(s)
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the shell form reads gate_kit_roots, whose live values
// are absolute; the bridge carries the relative spelling, so an absolute root passes through and
// a relative one resolves against the positional root exactly as the shell resolves it
pub fn kit_abs(root: &str, r: &str) -> String {
    if r.starts_with('/') {
        r.to_string()
    } else {
        format!("{}/{}", root, r)
    }
}

pub fn kit_name(r: &str) -> &str {
    match r.rfind('/') {
        Some(i) => &r[i + 1..],
        None => r,
    }
}

pub fn run(args: &[String]) -> i32 {
    let root = match scan_root(args, "check-smoke-entry-guard") {
        Some(r) => r,
        None => return 2,
    };
    if !Path::new(&root).is_dir() {
        eprintln!("check-smoke-entry-guard: root not found: {}", root);
        return 2;
    }

    let kit_roots = match walk::kit_roots() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-smoke-entry-guard: {}", e);
            return 2;
        }
    };
    if kit_roots.is_empty() {
        eprintln!("check-smoke-entry-guard: no kit roots enumerated");
        return 2;
    }

    let mut findings: Vec<String> = Vec::new();
    let mut swept = 0usize;
    for raw in &kit_roots {
        let r = raw.trim_end_matches('/');
        let abs = kit_abs(&root, r);
        let kit = kit_name(r);
        if !Path::new(&format!("{}/smoke", abs)).is_dir() {
            continue;
        }
        for name in ["install.sh", "violation.sh"] {
            let f = format!("{}/smoke/{}", abs, name);
            if !Path::new(&f).exists() {
                continue;
            }
            let text = match std::fs::read(&f) {
                Ok(b) => String::from_utf8_lossy(&b).into_owned(),
                Err(_) => {
                    eprintln!(
                        "check-smoke-entry-guard: unreadable smoke script: {}/smoke/{}",
                        kit, name
                    );
                    return 2;
                }
            };
            swept += 1;
            if !text.contains(GUARD) {
                findings.push(format!(
                    "{}/smoke/{}: no ${{SMOKE_KIT_ROOT:?…}} entry-point guard",
                    kit, name
                ));
            }
        }
    }

    if !findings.is_empty() {
        println!("check-smoke-entry-guard: mutating smoke script(s) missing the entry-point guard:");
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: add ': \"${{SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}}\"' right after");
        println!("        'set -euo pipefail' and before the first mutating command, so a bare run");
        println!("        refuses instead of writing into the caller's repo (gate-sdk/SPEC.md §Consumer smoke).");
        return 1;
    }

    println!(
        "SMOKE-ENTRY-GUARD: clean ({} mutating smoke script(s) carry the entry-point guard)",
        swept
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_relative_kit_root_resolves_against_the_positional_root_and_an_absolute_one_does_not() {
        assert_eq!(kit_abs(".", "alpha-kit"), "./alpha-kit");
        assert_eq!(kit_abs("/repo", "gate-sdk"), "/repo/gate-sdk");
        assert_eq!(kit_abs("/repo", "/elsewhere/kit"), "/elsewhere/kit");
    }

    #[test]
    fn the_kit_label_is_the_root_basename_in_either_spelling() {
        assert_eq!(kit_name("gate-sdk"), "gate-sdk");
        assert_eq!(kit_name("/repo/gate-sdk"), "gate-sdk");
    }
}
