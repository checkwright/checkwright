// spec: gate-sdk/SPEC.md §check-hook-exec-bit — every tracked file in the hooks dir carries
// index mode 100755, or a fresh clone silently skips a non-executable hook
use crate::fresh;
use crate::proc;
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    let dir = match args.first() {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("GATE_SDK_HOOKS_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-hook-exec-bit: {}", e);
                return 2;
            }
        },
    };

    let probe = match proc::run("git", &["rev-parse", "--git-dir"]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("check-hook-exec-bit: {}", e);
            return 2;
        }
    };
    if probe.stdout().is_none() {
        eprintln!("check-hook-exec-bit: not a git repository — cannot read index modes");
        return 2;
    }

    if !Path::new(&dir).is_dir() {
        println!(
            "HOOK-EXEC-BIT: clean (no hooks dir at {} — nothing committed to skip)",
            dir
        );
        return 0;
    }

    let listing = match proc::run("git", &["ls-files", "-s", "--", &dir]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("check-hook-exec-bit: {}", e);
            return 2;
        }
    };
    let text = match listing.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            eprintln!(
                "check-hook-exec-bit: {}",
                fresh::fail_closed("git-ls-files", listing.code())
            );
            return 2;
        }
    };

    let mut bad: Vec<String> = Vec::new();
    let mut count = 0usize;
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        count += 1;
        let mode = line.split(' ').next().unwrap_or("");
        let path = line.split_once('\t').map(|(_, p)| p).unwrap_or("");
        if mode != "100755" {
            bad.push(format!("{} (index mode {})", path, mode));
        }
    }

    if !bad.is_empty() {
        println!("check-hook-exec-bit: hook file(s) not committed executable (mode 100755) — git");
        println!("silently skips a non-executable hook, disabling the gate battery for a fresh clone:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: git update-index --chmod=+x <path> (and chmod +x locally), then recommit.");
        return 1;
    }

    println!(
        "HOOK-EXEC-BIT: clean ({} tracked hook file(s) in {} at index mode 100755)",
        count, dir
    );
    0
}
