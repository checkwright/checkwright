// spec: gate-sdk/SPEC.md §check-action-run-shell — every GitHub Actions `run:` literal block
// scalar in an Actions-shaped YAML file is ShellCheck-clean at -S warning under the dialect the
// step actually runs, as the wrapper criterion 7's worked example ports to
use crate::actions_run::{actions_shaped, dialect_of, extract, print_refusal, records, Item, Runner};
use crate::{proc, programs};
use crate::walk;
use std::path::{Path, PathBuf};

const NAME: &str = "check-action-run-shell";

// spec: gate-sdk/SPEC.md §Fail-closed contract — the wrapper's own refusal text at the shell
// form's own point in the order: after the scan-root check and before the walk, so a tree with
// no YAML and no linter reports the linter rather than exiting clean on a zero count
fn refuse_absent_program() -> i32 {
    eprintln!("{}: {} not found on PATH — the gate cannot run.", NAME, programs::SHELLCHECK);
    eprintln!("  A gate that cannot run is not clean (fail-closed).");
    eprintln!("  help: install ShellCheck (e.g. 'apt install shellcheck' / 'brew install shellcheck').");
    2
}

struct Tally {
    walked: usize,
    subject: usize,
    skipped_files: usize,
    linted: usize,
    plain: usize,
    skipped_dialect: usize,
    findings: Vec<String>,
    unresolved: Vec<String>,
}

fn lint_block(
    frag: &Path,
    dialect: &str,
    file: &Path,
    blockstart: usize,
    tally: &mut Tally,
) -> Result<(), i32> {
    let frag_s = frag.display().to_string();
    let merged = match proc::run_merged(&programs::SHELLCHECK, &["-f", "gcc", "-S", "warning", "-s", dialect, &frag_s])
    {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return Err(2);
        }
    };
    let rc = merged.code().unwrap_or(-1);
    if !(0..=1).contains(&rc) {
        eprintln!(
            "{}: shellcheck exited {} on {} (block at line {})",
            NAME,
            rc,
            file.display(),
            blockstart
        );
        return Err(2);
    }
    if rc == 0 {
        return Ok(());
    }
    let out = String::from_utf8_lossy(merged.output()).into_owned();
    let prefix = format!("{}:", frag_s);
    for hit in records(out.trim_end_matches('\n')) {
        if hit.is_empty() {
            continue;
        }
        let rest = hit.strip_prefix(prefix.as_str()).unwrap_or(hit);
        let head = rest.split(':').next().unwrap_or("");
        let numeric = !head.is_empty() && head.bytes().all(|b| b.is_ascii_digit());
        match head.parse::<usize>() {
            Ok(fline) if numeric => {
                let tail = rest.split_once(':').map(|(_, t)| t).unwrap_or(rest);
                tally
                    .findings
                    .push(format!("{}:{}:{}", file.display(), blockstart + fline, tail));
            }
            _ => tally.findings.push(format!(
                "{} (run: block at line {}): {}",
                file.display(),
                blockstart,
                hit
            )),
        }
    }
    Ok(())
}

fn scan(files: &[PathBuf], work: &Path) -> Result<Tally, i32> {
    let mut tally = Tally {
        walked: 0,
        subject: 0,
        skipped_files: 0,
        linted: 0,
        plain: 0,
        skipped_dialect: 0,
        findings: Vec::new(),
        unresolved: Vec::new(),
    };
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
            Err(r) => return Err(print_refusal(NAME, "linting", f, &r)),
        };
        let fdir = work.join(format!("f{}", tally.subject));
        if let Err(e) = std::fs::create_dir_all(&fdir) {
            eprintln!("{}: could not create a scratch dir ({})", NAME, e);
            return Err(2);
        }
        for item in &ex.items {
            match item {
                Item::Single { .. } => tally.plain += 1,
                Item::Block {
                    n,
                    line,
                    shell,
                    runner,
                    ..
                } => {
                    // spec: gate-sdk/SPEC.md §check-action-run-shell — a step's dialect must be
                    // knowable, and where the gate cannot state it the step says it
                    let dialect = match shell {
                        Some(raw) => dialect_of(raw),
                        None => match runner {
                            Runner::NonWindows => "bash",
                            Runner::Windows => {
                                tally.unresolved.push(format!(
                                    "{}:{}: the enclosing job runs on a Windows runner, whose default run: shell is pwsh, and the step names no shell:",
                                    f.display(), line));
                                continue;
                            }
                            Runner::Unreadable => {
                                tally.unresolved.push(format!(
                                    "{}:{}: the enclosing job's runs-on cannot be read, so the dialect cannot be stated, and the step names no shell:",
                                    f.display(), line));
                                continue;
                            }
                            Runner::NoJob => {
                                tally.unresolved.push(format!(
                                    "{}:{}: the step has no enclosing job, so the dialect cannot be stated, and the step names no shell:",
                                    f.display(), line));
                                continue;
                            }
                        },
                    };
                    if dialect.is_empty() {
                        tally.skipped_dialect += 1;
                        continue;
                    }
                    tally.linted += 1;
                    let frag = fdir.join(format!("block-{}.sh", n));
                    if let Err(e) = std::fs::write(&frag, ex.bodies[n - 1].as_bytes()) {
                        eprintln!("{}: could not write {} ({})", NAME, frag.display(), e);
                        return Err(2);
                    }
                    lint_block(&frag, dialect, f, *line, &mut tally)?;
                }
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

    if !proc::on_path(&programs::SHELLCHECK) {
        return refuse_absent_program();
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
            "ACTION-RUN-SHELL: clean (no YAML under {} — 0 run: block(s) to lint)",
            scanroot
        );
        return 0;
    }

    let work = std::env::temp_dir().join(format!("checkwright-runshell.{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    if let Err(e) = std::fs::create_dir_all(&work) {
        eprintln!("{}: could not create a scratch dir ({})", NAME, e);
        return 2;
    }
    let outcome = scan(&files, &work);
    let _ = std::fs::remove_dir_all(&work);

    let tally = match outcome {
        Ok(t) => t,
        Err(code) => return code,
    };

    if !tally.findings.is_empty() || !tally.unresolved.is_empty() {
        if !tally.findings.is_empty() {
            println!("{}: ShellCheck finding(s) in a workflow run: block — nothing else", NAME);
            println!("in the battery reaches this shell, and it executes only on a tag or a push:");
            for s in &tally.findings {
                println!("  {}", s);
            }
            println!("  help: fix each finding in the workflow's run: body (the line numbers are the");
            println!("        workflow's own), or silence a genuine false positive with an inline");
            println!("        '# shellcheck disable=SCxxxx' plus a justifying comment.");
        }
        // spec: gate-sdk/SPEC.md §Output contract — a gate with more than one failure class gives
        // each its own help: line, and this class's remedy is one key rather than an exemption
        if !tally.unresolved.is_empty() {
            println!("{}: run: block(s) whose shell dialect nothing states — the gate", NAME);
            println!("does not assume a dialect it cannot derive, so these are not linted:");
            for s in &tally.unresolved {
                println!("  {}", s);
            }
            println!("  help: name the step's dialect with a 'shell:' key — 'shell: bash' selects");
            println!("        Git-for-Windows bash on a Windows runner, and 'shell: pwsh' is");
            println!("        skipped and counted as a non-shell dialect.");
        }
        return 1;
    }

    println!(
        "ACTION-RUN-SHELL: clean ({} run: block(s) linted at -S warning across {} Actions-shaped file(s) of {} walked; {} file(s) skipped by the Actions-shape predicate, {} plain-scalar run: value(s) skipped, {} block(s) skipped on a non-shell dialect)",
        tally.linted, tally.subject, tally.walked, tally.skipped_files, tally.plain, tally.skipped_dialect
    );
    0
}
