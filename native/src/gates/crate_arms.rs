// spec: gate-sdk/SPEC.md §check-crate-arms — the crate's lint and test arms at commit time, as the
// wrapper a member whose rule *is* an invocation of cargo ports to: the programs stay declared
// dependencies this spawns, and the source-stamp cache is relocated rather than redesigned
use crate::fresh;
use crate::proc;
use crate::programs::{self, Program, CARGO, RUSTC};
use crate::walk;

const NAME: &str = "check-crate-arms";

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
fn version_of(program: &Program) -> String {
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
    let id = proc::run_with_stdin(&programs::GIT, &["hash-object", "--stdin"], crate_dir.as_bytes())
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
    let m = proc::run_merged(&CARGO, argv)?;
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

// spec: gate-sdk/SPEC.md §check-crate-arms — the fixture arm: every derived suite through the
// runner under an environment stripped of git's repository locators, a failing suite's report
// printed whole with its re-run command, and a suite holding the working directory skipped
fn fixture_arm(runner: &Program, suites: &[(String, String, String)], here: &str) -> Result<(bool, usize), String> {
    let (mut ok, mut ran) = (true, 0usize);
    for (suite, tests, checks) in suites {
        if walk::at_or_under(&walk::abs_against(here, tests), here) {
            continue;
        }
        ran += 1;
        let mut argv = vec!["--run-gate-tests", tests.as_str()];
        if !checks.is_empty() {
            argv.push(checks.as_str());
        }
        let m = proc::run_merged_without(runner, &argv, proc::GIT_REPO_LOCATORS)?;
        if m.succeeded() {
            continue;
        }
        ok = false;
        println!(
            "{}: fixture suite {} failed (exit {}):",
            NAME,
            suite,
            m.reported_code()
        );
        println!("{}", String::from_utf8_lossy(m.output()).trim_end_matches('\n'));
        println!("  re-run: {} {}", runner.invocation(), argv.join(" "));
    }
    Ok((ok, ran))
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

    if !proc::on_path(&CARGO) {
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
            &programs::GIT,
            &["-C", &crate_dir, "ls-files", "--others", "--exclude-standard", "--", "."],
        )
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
        .unwrap_or_default();
        if untracked.trim_end_matches('\n').is_empty() {
            key = format!("{} {} {}", stamp, version_of(&RUSTC), version_of(&CARGO));
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

    // spec: gate-sdk/SPEC.md §check-crate-arms — the third arm runs under the same predicate and
    // whatever the first two said, through the binary every suite already runs
    let suites = crate::registry::fixture_suites()
        .and_then(|s| Ok((s, walk::knob_scalar("GATE_SDK_NATIVE_BIN")?, walk::cwd()?)))
        .and_then(|(s, bin, here)| fixture_arm(&programs::CHECKWRIGHT_GATES.at(bin), &s, &here));
    let suites = match suites {
        Ok((ok, n)) => {
            fail |= !ok;
            n
        }
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };

    if fail {
        println!("  help: fix the finding above. These are the arms CI runs, and this gate is now their");
        println!("        only spelling — the battery plus bash gate-sdk/bin/build-native.sh is the whole");
        println!("        commit-time obligation, and neither discharges the other. A fixture suite runs");
        println!("        the binary GATE_SDK_NATIVE_BIN names, so rebuild it before reading a suite's red.");
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
        "CRATE-ARMS: clean (cargo clippy --all-targets at -D warnings and cargo test, both --release over {}, build scratch {}; {} fixture suite(s) green)",
        crate_dir, target_dir, suites
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

    // spec: gate-sdk/SPEC.md §check-crate-arms — the stub is exec'd only once no process can hold
    // it open for writing: a sibling test thread forking while it was being written keeps a copy
    // of the write descriptor until that child execs, and a spawn meanwhile fails ETXTBSY
    #[cfg(unix)]
    fn stub_runner(dir: &std::path::Path, body: &str) -> Program {
        use std::os::unix::fs::PermissionsExt;
        let stub = dir.join("runner");
        std::fs::write(&stub, body).expect("stub runner");
        std::fs::set_permissions(&stub, std::fs::Permissions::from_mode(0o755)).expect("exec bit");
        const ETXTBSY: i32 = 26;
        for _ in 0..200 {
            match std::process::Command::new(&stub).arg("--probe").output() {
                Err(e) if e.raw_os_error() == Some(ETXTBSY) => {
                    std::thread::sleep(std::time::Duration::from_millis(10))
                }
                Err(e) => panic!("cannot run the stub runner: {}", e),
                Ok(_) => return programs::CHECKWRIGHT_GATES.at(stub.display().to_string()),
            }
        }
        panic!("the stub runner stayed busy for two seconds");
    }

    // spec: gate-sdk/SPEC.md §check-crate-arms — a failing suite reds the fixture arm whatever its
    // siblings said, and a suite holding the working directory is skipped; the stub stands in for
    // the binary, whose case verdicts §run-gate-tests' own pair holds
    #[cfg(unix)]
    #[test]
    fn a_failing_fixture_suite_reds_the_arm_and_a_suite_never_reenters_itself() {
        let dir = std::env::temp_dir().join(format!("crate-arms-fixture-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for d in ["passing/gate-tests", "failing/gate-tests", "own/gate-tests/check-x/bad"] {
            std::fs::create_dir_all(dir.join(d)).expect("synthetic suite dir");
        }
        let runner = stub_runner(
            &dir,
            "#!/bin/sh\ncase \"$2\" in *failing*) echo '  FAIL: check-x bad expected exit 1, got 0'; exit 1;; esac\necho 'GATE-TESTS: clean (1 pairs, 0 unit tests)'\n",
        );
        let root = dir.display().to_string();
        let row = |s: &str| (s.to_string(), format!("{}/{}/gate-tests", root, s), String::new());
        let here = format!("{}/own/gate-tests/check-x/bad", root);

        let green = fixture_arm(&runner, &[row("passing")], &here).expect("arm ran");
        assert_eq!(green, (true, 1));
        let red = fixture_arm(&runner, &[row("passing"), row("failing"), row("own")], &here).expect("arm ran");
        assert_eq!(red, (false, 2), "the failing suite must red the arm, and the suite holding the cwd must not run");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: gate-sdk/SPEC.md §check-crate-arms — a suite never inherits the hook's repository
    // locators. `GIT_PREFIX` stands in for the set because nothing in the tree, git included,
    // reads it, so writing it process-wide cannot perturb a sibling test's git.
    #[cfg(unix)]
    #[test]
    fn a_fixture_suite_never_inherits_the_hooks_repository_locators() {
        let dir = std::env::temp_dir().join(format!("crate-arms-locators-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("suite/gate-tests")).expect("synthetic suite dir");
        let runner = stub_runner(
            &dir,
            "#!/bin/sh\n[ -n \"${GIT_PREFIX+set}\" ] && { echo 'inherited GIT_PREFIX'; exit 1; }\nexit 0\n",
        );
        let row = (
            "suite".to_string(),
            format!("{}/suite/gate-tests", dir.display()),
            String::new(),
        );
        let verdict = {
            let knobs = crate::knobenv::lock();
            knobs.set("GIT_PREFIX", "leaked/");
            let v = fixture_arm(&runner, &[row], &dir.display().to_string());
            knobs.remove("GIT_PREFIX");
            v
        };
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(verdict.expect("arm ran"), (true, 1), "the suite saw a repository locator the hook exported");
    }

    // spec: gate-sdk/SPEC.md §check-crate-arms — an absent program contributes an empty version
    // field rather than a refusal, which is the arm the fixture pair cannot carry: no committed
    // case can take a program off PATH, and `rustc` is read at exactly this one site.
    #[test]
    fn an_absent_program_yields_an_empty_version_field_not_a_refusal() {
        assert_eq!(
            version_of(&Program::consumer("test", "checkwright-no-such-program-exists")),
            ""
        );
        assert!(
            !version_of(&CARGO).is_empty(),
            "cargo printed no version, so every cache key would collapse to one field"
        );
    }
}
