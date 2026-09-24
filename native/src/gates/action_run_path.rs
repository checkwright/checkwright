// spec: gate-sdk/SPEC.md §check-action-run-path — every repo-relative `.sh` path in invocation
// position in a shell `run:` body of an Actions-shaped YAML file resolves under the scan root
use crate::actions_run::{actions_shaped, dialect_of, extract, print_refusal, Item, Runner};
use crate::gates::docs_cmd;
use crate::walk;
use std::path::{Path, PathBuf};

const NAME: &str = "check-action-run-path";

#[derive(Default)]
struct Tally {
    walked: usize,
    subject: usize,
    skipped_files: usize,
    bodies: usize,
    resolved: usize,
    non_shell: usize,
    unresolved_dialect: usize,
    escaped: usize,
    unclosed: usize,
    workdir: usize,
    absolute: usize,
    findings: Vec<String>,
}

// spec: gate-sdk/SPEC.md §check-action-run-path — a body is read when its resolved dialect is a
// shell; any other body is skipped and counted, the unresolved dialects being
// check-action-run-shell's findings already
fn is_shell(shell: &Option<String>, runner: &Runner, tally: &mut Tally) -> bool {
    match shell {
        Some(raw) if dialect_of(raw).is_empty() => {
            tally.non_shell += 1;
            false
        }
        Some(_) => true,
        None => match runner {
            Runner::NonWindows => true,
            _ => {
                tally.unresolved_dialect += 1;
                false
            }
        },
    }
}

// spec: gate-sdk/SPEC.md §check-action-run-path — resolution is against the scan root, or a
// step's literal `working-directory:` joined to it; an expression or an absolute directory is
// skipped and counted
fn base_of(root: &str, workdir: &Option<String>) -> Option<String> {
    match workdir {
        None => Some(root.to_string()),
        Some(wd) if wd.contains("${{") || walk::path_root(wd).is_some() || wd.is_empty() => None,
        Some(wd) => Some(format!("{}/{}", root.trim_end_matches('/'), wd.trim_start_matches("./"))),
    }
}

// spec: gate-sdk/SPEC.md §check-action-run-path — a shell comment line runs no command, so it
// invokes nothing
fn read_line(file: &Path, ln: usize, text: &str, base: &str, tally: &mut Tally) {
    if text.trim_start().starts_with('#') {
        return;
    }
    for tok in docs_cmd::invoked_tokens(text) {
        if walk::path_root(&tok).is_some() {
            tally.absolute += 1;
            continue;
        }
        let rel = tok.strip_prefix("./").unwrap_or(&tok);
        if Path::new(&format!("{}/{}", base.trim_end_matches('/'), rel)).is_file() {
            tally.resolved += 1;
        } else {
            tally.findings.push(format!(
                "{}:{}: run: invokes {}, which does not resolve under {}",
                file.display(),
                ln,
                tok,
                base
            ));
        }
    }
}

fn scan(files: &[PathBuf], root: &str) -> Result<Tally, i32> {
    let mut tally = Tally::default();
    for f in files {
        tally.walked += 1;
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "{}: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                    NAME,
                    f.display(),
                    e
                );
                return Err(2);
            }
        };
        if !actions_shaped(&text) {
            tally.skipped_files += 1;
            continue;
        }
        tally.subject += 1;
        let ex = match extract(&text) {
            Ok(ex) => ex,
            Err(r) => return Err(print_refusal(NAME, "reading", f, &r)),
        };
        for item in &ex.items {
            let (shell, runner, workdir) = match item {
                Item::Block {
                    shell,
                    runner,
                    workdir,
                    ..
                }
                | Item::Single {
                    shell,
                    runner,
                    workdir,
                    ..
                } => (shell, runner, workdir),
            };
            if !is_shell(shell, runner, &mut tally) {
                continue;
            }
            let Some(base) = base_of(root, workdir) else {
                tally.workdir += 1;
                continue;
            };
            match item {
                Item::Block { n, line, .. } => {
                    tally.bodies += 1;
                    for (k, l) in ex.bodies[n - 1].lines().enumerate() {
                        read_line(f, line + 1 + k, l, &base, &mut tally);
                    }
                }
                Item::Single { text, line, .. } => match text {
                    Ok(t) => {
                        tally.bodies += 1;
                        read_line(f, *line, t, &base, &mut tally);
                    }
                    Err("escaped double-quoted value") => tally.escaped += 1,
                    Err(_) => tally.unclosed += 1,
                },
            }
        }
    }
    Ok(tally)
}

pub fn run(args: &[String]) -> i32 {
    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(scanroot).is_dir() {
        eprintln!("{}: scan root not found: {}", NAME, scanroot);
        return 2;
    }

    let files = match walk::find_files(Path::new(scanroot), &["yml", "yaml"]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{}: {} — the check could not run; treating as failure (not clean)",
                NAME, e
            );
            return 2;
        }
    };

    if files.is_empty() {
        println!(
            "ACTION-RUN-PATH: clean (no YAML under {} — 0 run: bodies to read)",
            scanroot
        );
        return 0;
    }

    let tally = match scan(&files, scanroot) {
        Ok(t) => t,
        Err(code) => return code,
    };

    if !tally.findings.is_empty() {
        println!("{}: run: step(s) invoking a script that does not resolve — nothing else in", NAME);
        println!("the battery reads a workflow's commands as paths, so CI finds it on the push");
        println!("that runs the step:");
        for s in &tally.findings {
            println!("  {}", s);
        }
        println!("  help: repoint the step at the script that replaced it, or delete a step whose");
        println!("        script is spent.");
        return 1;
    }

    println!(
        "ACTION-RUN-PATH: clean ({} run: body(ies) read, {} invocation(s) resolved across {} Actions-shaped file(s) of {} walked; {} file(s) skipped by the Actions-shape predicate; skipped: {} on a non-shell dialect, {} on an unresolved dialect, {} escaped double-quoted value(s), {} quoted value(s) not closed on their line, {} on a working-directory it cannot resolve, {} absolute invocation path(s))",
        tally.bodies,
        tally.resolved,
        tally.subject,
        tally.walked,
        tally.skipped_files,
        tally.non_shell,
        tally.unresolved_dialect,
        tally.escaped,
        tally.unclosed,
        tally.workdir,
        tally.absolute
    );
    0
}
