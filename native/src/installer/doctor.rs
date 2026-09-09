// spec: installer/README.md §doctor — renders the toolchain floor as an exit status so `init` and a
// CI step can gate on the verdict without parsing a report. The roster is the crate's own
// (context-kit/SPEC.md §bin/env-probe), because at init time nothing is vendored in the tree yet.
use super::{lock, GATES_DIR};
use crate::toolfloor::{self, Verdict as Floor};
use crate::{proc, sha256};
use std::fmt::Write as _;

const USAGE: &[&str] = &[
    "usage: checkwright doctor",
    "",
    "Reports whether this machine meets the toolchain contract it needs as a",
    "consumer, and — when run inside a repository that has been vendored",
    "into — what is installed there.",
    "Exit status is the verdict: 0 meets the contract, 1 below it.",
];

// spec: installer/README.md §doctor — the report is built whole so `init` can read the same
// rendering it prints, on the same three channels: what a caller shows, what it reports as a
// refusal, and the verdict itself.
pub struct Report {
    pub out: String,
    pub err: String,
    pub code: i32,
}

// spec: context-kit/SPEC.md §bin/env-probe — both version probes read from a closed stdin, and `-V`
// is only the fallback: a tool rejecting `--version` would otherwise reach a `-V` that reads
// inherited stdin and hangs.
fn probe_banner(tool: &str) -> String {
    if !proc::on_path(tool) {
        return String::new();
    }
    let resolved = proc::resolve_floor_tool(tool);
    for flag in ["--version", "-V"] {
        if let Ok(c) = proc::run_with_stdin(&resolved, &[flag], b"") {
            let raw = c
                .stdout()
                .map(|o| String::from_utf8_lossy(o).into_owned())
                .unwrap_or_default();
            if !raw.is_empty() {
                return raw;
            }
        }
    }
    "present".to_string()
}

// spec: installer/README.md §doctor — doctor defines no floor of its own: it renders whatever
// verdict the roster's own predicate returns, so the contract has one owner and this is a display.
fn render_member(out: &mut String, element: &str, banner: &str) -> bool {
    let e = toolfloor::parse(element);
    let version = toolfloor::version(banner);
    let shown = |fallback: &str| {
        if version.is_empty() {
            fallback.to_string()
        } else {
            version.clone()
        }
    };
    match toolfloor::check(element, banner) {
        Floor::Ok => {
            let _ = writeln!(out, "  {:<12} {}", e.name, shown("present"));
            false
        }
        Floor::Absent => {
            let _ = writeln!(out, "  {:<12} NOT FOUND", e.name);
            true
        }
        Floor::Below { found, floor } => {
            let _ = writeln!(
                out,
                "  {:<12} {} (below the floor of {})",
                e.name, found, floor
            );
            true
        }
        Floor::WrongImpl { found } => {
            let _ = writeln!(
                out,
                "  {:<12} {} (not the {} implementation the contract requires)",
                e.name,
                shown(&found),
                e.imp
            );
            true
        }
        Floor::Uncomparable => {
            let _ = writeln!(
                out,
                "  {:<12} could not be compared against the floor of {}",
                e.name, e.min
            );
            true
        }
    }
}

// spec: installer/README.md §The gate binary — the omitted-member record's reader. The two
// install-time reason tokens retired; the reason-agnostic class did not, so this reports whatever
// reason it finds and invents no remedy for it.
fn omitted_block(out: &mut String, list_text: &str) {
    let mut reasons: Vec<&str> = list_text
        .lines()
        .filter_map(|l| {
            let mut f = l.split_whitespace();
            match (f.next(), f.next(), f.next(), f.next()) {
                (Some("#"), Some("omitted:"), Some(_), Some(r)) => Some(r),
                _ => None,
            }
        })
        .collect();
    reasons.sort_unstable();
    let mut i = 0;
    while i < reasons.len() {
        let r = reasons[i];
        let n = reasons[i..].iter().take_while(|x| **x == r).count();
        let _ = writeln!(out, "  {:<12} {} gate(s), {}", "omitted", n, r);
        i += n;
    }
    // spec: installer/README.md §The gate binary — the all-omitted registry's own line, said only
    // when no live member survives: the per-reason counts read identically at 24-of-26 and at
    // 26-of-26, and the second is the one where an adopter's battery cannot run at all.
    let live = list_text
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !t.is_empty() && !t.starts_with('#')
        })
        .count();
    if live == 0 {
        let _ = writeln!(
            out,
            "  {:<12} no gate survives here, so the battery cannot run at all",
            "battery"
        );
    }
}

pub fn diagnose() -> Report {
    let mut out = String::new();
    let mut err = String::new();
    let mut failed = false;
    let mut artifact_finding = String::new();

    out.push_str("toolchain\n");
    for element in toolfloor::PROBE_SET {
        // spec: installer/README.md §doctor — a contributor-audience member is skipped outright
        // rather than rendered as informational: doctor is the adopter's verb, and showing an
        // adopter a tool the install path never reaches is an invitation to install it.
        if toolfloor::parse(element).audience == "contributor" {
            continue;
        }
        let banner = probe_banner(&toolfloor::parse(element).name);
        failed |= render_member(&mut out, element, &banner);
    }

    // spec: gate-sdk/SPEC.md §The crate's crosser — both producers go through the crosser, so a
    // report run outside a work tree names the same dialect one run inside it does.
    let root = super::repo_root()
        .or_else(|| crate::walk::cwd().ok().map(std::path::PathBuf::from))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let lock_path = lock::path(&root);

    if !lock_path.is_file() {
        let _ = writeln!(
            out,
            "\nNo {} here — nothing has been vendored into this directory.",
            lock::FILE
        );
    } else {
        let manifest = lock::Manifest::read(&lock_path).filter(lock::Manifest::schema_ok);
        let Some(manifest) = manifest else {
            let _ = writeln!(
                err,
                "checkwright doctor: {} carries a schema this build does not know.",
                lock_path.display()
            );
            err.push_str("  help: this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for.\n");
            return Report {
                out,
                err,
                code: 2,
            };
        };
        let version = manifest.field("version");
        // spec: installer/README.md §doctor — a manifest carrying `files` and no `version` is a
        // residue rather than an install, `version` being the field an install always has and a
        // residue never does.
        // spec: installer/README.md §doctor — it guards the whole installed block rather than
        // sitting beside it: every line below is a per-install reading, and printing them past a
        // residue is the mixed verdict doctor's exit-status carve-out already refuses.
        if version.is_empty() {
            let _ = writeln!(
                out,
                "\nno install here — {} file(s) remain that a previous install wrote and you have since edited. They are yours, and a future init will still protect them.",
                manifest.file_count()
            );
        } else {
            out.push_str("\ninstalled\n");
            let _ = writeln!(out, "  {:<12} {}", "version", version);
            let _ = writeln!(out, "  {:<12} {}", "commit", manifest.field("commit"));
            let _ = writeln!(out, "  {:<12} {}", "profile", manifest.field("profile"));
            let _ = writeln!(out, "  {:<12} {}", "kits", manifest.field("kits"));

            // spec: installer/README.md §doctor — the registry this tree's battery runs from is
            // named rather than left implicit: it is the one install fact the identity fields do
            // not carry, and a report resolving the wrong file would say nothing about which.
            let list = manifest.own_file(&format!("{}/gates.list", GATES_DIR));
            if list.is_empty() {
                let _ = writeln!(out, "  {:<12} none recorded — re-run init", "registry");
            } else {
                let tail = if root.join(&list).is_file() {
                    ""
                } else {
                    " — recorded, but not on disk; re-run init"
                };
                let _ = writeln!(out, "  {:<12} {}{}", "registry", list, tail);
            }

            // spec: installer/README.md §The gate binary — the recorded digest's second reader:
            // re-verifying in place is all that stands between a consumer and a binary swapped
            // after install, and the path comes from the knob that owns it, not a stored copy.
            let (target, digest) = manifest.artifact();
            if !target.is_empty() {
                let seam = manifest.own_file(&format!("{}/gate-sdk-config.sh", GATES_DIR));
                let bin = seam_binary(&root, &seam);
                // spec: installer/README.md §doctor — an artifact finding reports without setting
                // the verdict, deliberately: the status is the toolchain contract init gates on, so
                // failing it here would block the re-run that is this finding's own remedy.
                match bin {
                    None => {
                        let _ = writeln!(out, "  {:<12} {} — recorded, but nothing at the path GATE_SDK_NATIVE_BIN names; re-run init", "artifact", target);
                        artifact_finding = "the recorded gate binary is not on disk".to_string();
                    }
                    Some(rel) if sha256::file_hex(&root.join(&rel)).ok().as_deref() == Some(digest.as_str()) => {
                        let _ = writeln!(out, "  {:<12} {} (verified in place)", "artifact", target);
                    }
                    Some(rel) => {
                        let _ = writeln!(out, "  {:<12} {} — DIGEST MISMATCH, {} differs from what init wrote; re-run init", "artifact", target, rel);
                        artifact_finding =
                            "the installed gate binary does not match its recorded digest".to_string();
                    }
                }
            }

            if !list.is_empty() {
                if let Ok(text) = std::fs::read_to_string(root.join(&list)) {
                    omitted_block(&mut out, &text);
                }
            }
        }

        // spec: installer/README.md §doctor — the report stops at the toolchain and identity
        // fields; per-file divergence is a separate verb's question on both paths, so `DOCTOR:
        // clean` is never read as a claim about the tree's contents.
        out.push_str("\nrun checkwright diff to see which files, if any, have changed since.\n");
    }

    if failed {
        out.push_str("\nDOCTOR: below contract\n");
        out.push_str("  help: install or upgrade each tool reported above; the floors are the ones the gate battery needs to run, not preferences.\n");
        return Report { out, err, code: 1 };
    }
    // spec: installer/README.md §doctor — the verdict line names an artifact finding rather than
    // swallowing it: the exit status stays the toolchain contract, but a run that reported a digest
    // mismatch must not sign off as plainly clean.
    if !artifact_finding.is_empty() {
        let _ = writeln!(
            out,
            "\nDOCTOR: toolchain clean, 1 artifact finding — {}",
            artifact_finding
        );
        out.push_str("  help: re-run init; it re-verifies the published digest and rewrites the binary.\n");
        return Report { out, err, code: 0 };
    }
    out.push_str("\nDOCTOR: clean\n");
    Report { out, err, code: 0 }
}

fn seam_binary(root: &std::path::Path, seam: &str) -> Option<String> {
    if seam.is_empty() {
        return None;
    }
    let text = std::fs::read_to_string(root.join(seam)).ok()?;
    let rel = text
        .lines()
        .find_map(|l| l.strip_prefix("GATE_SDK_NATIVE_BIN="))?;
    if rel.is_empty() || !root.join(rel).is_file() {
        return None;
    }
    Some(rel.to_string())
}

pub fn run(args: &[String]) -> i32 {
    if let Some(outcome) = super::help_only(args, USAGE) {
        return super::finish("doctor", outcome);
    }
    let r = diagnose();
    print!("{}", r.out);
    if !r.err.is_empty() {
        eprint!("{}", r.err);
    }
    r.code
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/README.md §doctor — every arm of the closed verdict set renders, and only the
    // clean arm leaves the verdict alone.
    #[test]
    fn each_verdict_renders_and_only_the_clean_one_passes() {
        let mut out = String::new();
        assert!(!render_member(&mut out, "git", "git version 2.44.0"));
        assert!(render_member(&mut out, "git", ""));
        assert!(render_member(&mut out, "bash:9.9", "GNU bash, version 5.2.37"));
        assert!(render_member(&mut out, "awk::GNU", "mawk 1.3.4"));
        assert!(render_member(&mut out, "bash:4.3", "GNU bash, no version"));
        assert!(out.contains("git          2.44.0"));
        assert!(out.contains("git          NOT FOUND"));
        assert!(out.contains("bash         5.2.37 (below the floor of 9.9)"));
        assert!(out.contains("awk          1.3.4 (not the GNU implementation the contract requires)"));
        assert!(out.contains("bash         could not be compared against the floor of 4.3"));
    }

    // spec: installer/README.md §The gate binary — the omitted block reports whatever reason it
    // finds, invents no remedy for a retired one, and says the all-omitted line only when no live
    // member survives.
    #[test]
    fn the_omitted_block_is_reason_agnostic_and_names_an_empty_battery() {
        let mut out = String::new();
        omitted_block(
            &mut out,
            "# gate-sdk\ncheck-a\n# omitted: check-b local-policy\n# omitted: check-c local-policy\n",
        );
        assert!(out.contains("omitted      2 gate(s), local-policy"));
        assert!(!out.contains("battery"), "a live member reported an empty battery");

        let mut empty = String::new();
        omitted_block(&mut empty, "# gate-sdk\n# omitted: check-b local-policy\n");
        assert!(empty.contains("battery      no gate survives here"));
    }

    // spec: installer/README.md §The verbs — `--help` answers on its own and an unknown argument is
    // a usage refusal rather than an ignored token.
    #[test]
    fn help_answers_and_an_unknown_argument_refuses() {
        assert_eq!(run(&["--help".to_string()]), 0);
        assert_eq!(run(&["--nope".to_string()]), 2);
    }
}
