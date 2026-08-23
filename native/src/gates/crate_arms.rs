// spec: gate-sdk/SPEC.md §check-crate-arms — the crate's lint and test arms at commit time, as the
// wrapper a member whose rule *is* an invocation of cargo ports to: the programs stay declared
// dependencies this spawns, and the source-stamp cache is relocated rather than redesigned
use crate::fresh;
use crate::proc;
use crate::walk;

const NAME: &str = "check-crate-arms";
const CARGO: &str = "cargo";
const RUSTC: &str = "rustc";

// spec: gate-sdk/SPEC.md §Fail-closed contract — this member's own refusal text at the shell
// form's own point in the order, which that section states is *after* the crate-presence branch
// and for `cargo` alone.
fn refuse_absent_cargo(crate_dir: &str) -> i32 {
    eprintln!(
        "{}: {} is not on PATH but a crate is present at {} — the check could not run; treating as failure (not clean)",
        NAME, CARGO, crate_dir
    );
    eprintln!("  help: cargo is the contributor-side toolchain floor for a tree carrying the crate");
    eprintln!("        (context-kit/SPEC.md §bin/env-probe). Install a Rust toolchain, then re-run.");
    2
}

// spec: gate-sdk/SPEC.md §check-crate-arms — `<prog> --version` captured with stderr discarded and
// its emptiness never tested: an absent program contributes an empty field, which is a cache
// *miss* against any key written while it was present rather than a refusal
fn version_of(program: &str) -> String {
    proc::run(program, &["--version"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
        .unwrap_or_default()
        .trim_end_matches('\n')
        .to_string()
}

// spec: gate-sdk/SPEC.md §check-crate-arms — the cache file is named for the crate it caches, by
// git's content identity of that path, so two crates under one scratch dir cannot share a record
fn cache_path(tmp_dir: &str, crate_dir: &str) -> String {
    let id = proc::run_with_stdin("git", &["hash-object", "--stdin"], crate_dir.as_bytes())
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
        .map(|s| s.trim_end_matches('\n').to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "crate".to_string());
    format!("{}/crate-arms-{}.green", tmp_dir, id)
}

// spec: gate-sdk/SPEC.md §check-crate-arms — one arm's spawn and its report: the merged capture is
// read whatever the status, because for these two the *failing* run is the one whose report has to
// print, and a command substitution's value keeps exactly one trailing newline when echoed back
fn arm(label: &str, crate_dir: &str, argv: &[&str]) -> Result<bool, String> {
    let m = proc::run_merged(CARGO, argv)?;
    if m.succeeded() {
        return Ok(true);
    }
    println!(
        "{}: cargo {} failed (exit {}) on {}:",
        NAME,
        label,
        m.reported_code(),
        crate_dir
    );
    println!(
        "{}",
        String::from_utf8_lossy(m.output()).trim_end_matches('\n')
    );
    Ok(false)
}

pub fn run(_args: &[String]) -> i32 {
    let crate_dir = match walk::knob_scalar("GATE_SDK_NATIVE_CRATE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };
    let target_dir = match walk::knob_scalar("GATE_SDK_CARGO_TARGET_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };
    let tmp_dir = match walk::knob_scalar("GATE_SDK_TMP_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };
    let manifest = format!("{}/Cargo.toml", crate_dir);

    // spec: gate-sdk/SPEC.md §check-crate-arms — the predicate is the crate's presence, never
    // cargo's: a consumer tree is missing the corpus, not the toolchain, and a gate with no
    // corpus reports clean rather than declaring a skip
    if !std::path::Path::new(&manifest).is_file() {
        println!(
            "CRATE-ARMS: clean (no crate at {} — {} is absent, so there is no corpus to lint or test)",
            crate_dir, manifest
        );
        return 0;
    }

    if !proc::on_path(CARGO) {
        return refuse_absent_cargo(&crate_dir);
    }

    // spec: gate-sdk/SPEC.md §check-crate-arms — the source-stamp cache, whose miss conditions
    // that section states: a moved stamp, a moved toolchain, an untracked file under the crate,
    // or a crate git cannot answer for
    let cache = cache_path(&tmp_dir, &crate_dir);
    let mut key = String::new();
    if let Some(stamp) = fresh::source_stamp(&crate_dir) {
        // spec: gate-sdk/SPEC.md §check-crate-arms — the untracked probe runs only behind a stamp
        // that succeeded, which is what proves git can answer for this crate root; an unreadable
        // listing here is the shell form's discarded-stderr capture and not a second verdict
        let untracked = proc::run(
            "git",
            &["-C", &crate_dir, "ls-files", "--others", "--exclude-standard", "--", "."],
        )
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
        .unwrap_or_default();
        if untracked.trim_end_matches('\n').is_empty() {
            key = format!("{} {} {}", stamp, version_of(RUSTC), version_of(CARGO));
            if let Ok(recorded) = std::fs::read_to_string(&cache) {
                if recorded.trim_end_matches('\n') == key {
                    println!(
                        "CRATE-ARMS: clean (cached — source stamp {} and toolchain unchanged since the last green run recorded at {}; cargo clippy --all-targets at -D warnings and cargo test, both --release over {})",
                        &stamp[..12.min(stamp.len())],
                        cache,
                        crate_dir
                    );
                    return 0;
                }
            }
        }
    }

    // spec: gate-sdk/SPEC.md §check-crate-arms — both arms run even when the first fails, so one
    // commit-time report carries what CI would have said in two. The second call is not guarded by
    // the first's verdict, which is the whole of that rule.
    let mut fail = false;
    for (label, argv) in [
        (
            "clippy",
            vec![
                "clippy",
                "--release",
                "--manifest-path",
                manifest.as_str(),
                "--target-dir",
                target_dir.as_str(),
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ),
        (
            "test",
            vec![
                "test",
                "--release",
                "--manifest-path",
                manifest.as_str(),
                "--target-dir",
                target_dir.as_str(),
            ],
        ),
    ] {
        match arm(label, &crate_dir, &argv) {
            Ok(ok) => fail |= !ok,
            Err(e) => {
                eprintln!("{}: {}", NAME, e);
                return 2;
            }
        }
    }

    if fail {
        println!("  help: fix the finding above. These are the arms CI runs, and this gate is now their");
        println!("        only spelling — the battery plus bash gate-sdk/bin/build-native.sh is the whole");
        println!("        commit-time obligation, and neither discharges the other.");
        return 1;
    }

    if !key.is_empty() {
        if let Some(parent) = std::path::Path::new(&cache).parent() {
            if std::fs::create_dir_all(parent).is_ok() {
                let _ = std::fs::write(&cache, key.as_bytes());
            }
        }
    }
    println!(
        "CRATE-ARMS: clean (cargo clippy --all-targets at -D warnings and cargo test, both --release over {}, build scratch {})",
        crate_dir, target_dir
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-crate-arms — the cache is keyed on the crate path's content
    // identity, the arm no fixture case can reach: each case points at its own scratch dir as
    // well as its own crate, so a collision would be invisible there.
    #[test]
    fn two_crates_under_one_scratch_dir_get_distinct_cache_records() {
        let a = cache_path(".tmp", "native");
        let b = cache_path(".tmp", "vendor/other-crate");
        assert_ne!(
            a, b,
            "two crate roots hashed to one cache file, so one crate's green run would answer \
             for the other's arms"
        );
        assert!(a.starts_with(".tmp/crate-arms-") && a.ends_with(".green"), "{}", a);
    }

    // spec: gate-sdk/SPEC.md §check-crate-arms — an absent program contributes an empty version
    // field rather than a refusal, which is the arm the fixture pair cannot carry: no committed
    // case can take a program off PATH, and `rustc` is read at exactly this one site.
    #[test]
    fn an_absent_program_yields_an_empty_version_field_not_a_refusal() {
        assert_eq!(version_of("checkwright-no-such-program-exists"), "");
        assert!(
            !version_of(CARGO).is_empty(),
            "cargo printed no version, so every cache key would collapse to one field"
        );
    }
}
