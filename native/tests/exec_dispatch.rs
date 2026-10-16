// spec: gate-sdk/SPEC.md §The gate-authoring surface — the live dispatch of each consumer start: a
// scratch consumer registers a passing and a failing member of one start, and the battery built for
// this test run starts them, or refuses the member that cannot start

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_checkwright-gates");

const PASS: &str = "Write-Output 'PSPASS: clean (live)'\nexit 0\n";
const FAIL: &str = "Write-Output ('BIN=' + $env:GATE_SDK_NATIVE_BIN)\nWrite-Output 'psfail fired'\nWrite-Output 'help: fix the fixture'\nexit 1\n";

fn scratch(case: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("checkwright-exec-dispatch-{}-{}", std::process::id(), case));
    let _ = std::fs::remove_dir_all(&dir);
    let gates = dir.join("gates");
    std::fs::create_dir_all(&gates).expect("cannot create the scratch gates dir");
    std::fs::write(gates.join("gates.list"), "check-pspass\ncheck-psfail\n").expect("registry");
    let head = "# graph: couples=docs/*.md dir=one valve=none tier=precommit\n";
    std::fs::write(gates.join("check-pspass.gate"), format!("{}# exec: check-pspass.ps1\n", head)).expect("pass descriptor");
    std::fs::write(
        gates.join("check-psfail.gate"),
        format!("{}# spec: docs/gates.md §psfail — the live case's invariant\n# exec: check-psfail.ps1\n", head),
    )
    .expect("fail descriptor");
    std::fs::write(gates.join("check-pspass.ps1"), PASS).expect("pass target");
    std::fs::write(gates.join("check-psfail.ps1"), FAIL).expect("fail target");
    dir
}

// spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the case reads no knob of the invoking environment
fn run(dir: &Path, host: &str) -> String {
    let mut c = Command::new(BIN);
    c.arg("--run").current_dir(dir).stdin(Stdio::null());
    for (name, _) in std::env::vars_os() {
        let n = name.to_string_lossy();
        if n.starts_with("GATE_") || n.contains("_KIT_") || n.starts_with("GIT_") {
            c.env_remove(&name);
        }
    }
    let sdk = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repository root").join("gate-sdk");
    c.env("GATE_SDK_GATES_DIR", dir.join("gates"))
        .env("GATE_SDK_KIT_DIRS", sdk)
        .env("GATE_SDK_TMP_DIR", dir.join(".tmp"))
        .env("GATE_SDK_VERBOSE", "1")
        .env("GATE_SDK_POWERSHELL", host)
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir());
    let out = c.output().expect("cannot run the binary under test");
    format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

fn resolves(name: &str) -> bool {
    let exts: Vec<String> = if cfg!(windows) { vec![".exe".to_string(), String::new()] } else { vec![String::new()] };
    std::env::var_os("PATH").is_some_and(|p| {
        std::env::split_paths(&p).any(|d| exts.iter().any(|e| d.join(format!("{}{}", name, e)).is_file()))
    })
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — Windows PowerShell is on every Windows
// host, so a Windows build binds that arm; `pwsh` runs wherever it resolves
fn hosts() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    if cfg!(windows) || resolves("powershell") {
        out.push("powershell");
    }
    if resolves("pwsh") {
        out.push("pwsh");
    }
    out
}

#[test]
fn an_exec_member_starts_under_each_resolving_host() {
    for host in hosts() {
        let dir = scratch(host);
        let out = run(&dir, host);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(out.contains("  PASS: check-pspass"), "{}: {}", host, out);
        assert!(out.contains("  FAIL: check-psfail (exit 1)"), "{}: {}", host, out);
        assert!(out.contains("    spec: docs/gates.md §psfail — the live case's invariant"), "{}: {}", host, out);
        let seen = out.lines().find_map(|l| l.trim().strip_prefix("BIN=")).unwrap_or_default().to_string();
        assert!(Path::new(&seen).is_absolute(), "{}: the member saw GATE_SDK_NATIVE_BIN={:?}", host, seen);
        assert_eq!(
            std::fs::canonicalize(&seen).ok(),
            std::fs::canonicalize(BIN).ok(),
            "{}: the member was not handed the running binary",
            host
        );
    }
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — an empty host refuses the member at
// dispatch, a registered gate that cannot start being no clean one
#[test]
fn an_exec_member_with_no_host_is_a_dispatch_refusal() {
    let dir = scratch("off");
    let out = run(&dir, "");
    let _ = std::fs::remove_dir_all(&dir);
    assert!(out.contains("  FAIL: check-pspass (dispatch harness error, exit 2)"), "{}", out);
    assert!(out.contains("GATE_SDK_POWERSHELL is empty"), "{}", out);
}

fn assert_live(out: &str, pass: &str, fail: &str) {
    assert!(out.contains(&format!("  PASS: {}", pass)), "{}", out);
    assert!(out.contains(&format!("  FAIL: {} (exit 1)", fail)), "{}", out);
    assert!(out.contains("    spec: docs/gates.md §live — the live case's invariant"), "{}", out);
    let seen = out.lines().find_map(|l| l.trim().strip_prefix("BIN=")).unwrap_or_default().to_string();
    assert!(Path::new(&seen).is_absolute(), "the member saw GATE_SDK_NATIVE_BIN={:?}: {}", seen, out);
    assert_eq!(
        std::fs::canonicalize(&seen).ok(),
        std::fs::canonicalize(BIN).ok(),
        "the member was not handed the running binary: {}",
        out
    );
}

const HEAD: &str = "# graph: couples=docs/*.md dir=one valve=none tier=precommit\n";
const LIVE_SPEC: &str = "# spec: docs/gates.md §live — the live case's invariant\n";

fn consumer(case: &str, members: &[&str]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("checkwright-exec-dispatch-{}-{}", std::process::id(), case));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("gates")).expect("cannot create the scratch gates dir");
    std::fs::write(dir.join("gates/gates.list"), members.join("\n") + "\n").expect("registry");
    dir
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — the program the test's own toolchain
// compiles, beside the `cargo` running it
fn rustc() -> PathBuf {
    let beside = std::env::var_os("CARGO")
        .map(PathBuf::from)
        .map(|c| c.with_file_name(format!("rustc{}", std::env::consts::EXE_SUFFIX)));
    beside.filter(|p| p.is_file()).unwrap_or_else(|| PathBuf::from("rustc"))
}

fn build(src: &str, out: &Path) {
    let file = out.with_extension("rs");
    std::fs::write(&file, src).expect("program source");
    let done = Command::new(rustc())
        .arg("--edition=2021")
        .arg("-o")
        .arg(out)
        .arg(&file)
        .stdin(Stdio::null())
        .output()
        .expect("cannot run rustc");
    assert!(done.status.success(), "rustc failed: {}", String::from_utf8_lossy(&done.stderr));
}

const RS_PASS: &str = "fn main() { println!(\"RSPASS: clean (live)\"); }\n";
const RS_FAIL: &str = "fn main() {\n    println!(\"BIN={}\", std::env::var(\"GATE_SDK_NATIVE_BIN\").unwrap_or_default());\n    println!(\"rsfail fired\");\n    println!(\"help: fix the fixture\");\n    std::process::exit(1);\n}\n";

// spec: gate-sdk/SPEC.md §The gate-authoring surface — an executable target, built from source and
// placed at its extensionless path plus the host's suffix, starts directly; an unbuilt one is a
// dispatch refusal
#[test]
fn an_executable_exec_member_starts_directly_and_an_unbuilt_one_refuses() {
    let dir = consumer("rs", &["check-rspass", "check-rsfail", "check-rsgone"]);
    let gates = dir.join("gates");
    std::fs::create_dir_all(gates.join("bin")).expect("bin");
    std::fs::write(gates.join("check-rspass.gate"), format!("{}# exec: bin/check-rspass\n", HEAD)).expect("pass descriptor");
    std::fs::write(gates.join("check-rsfail.gate"), format!("{}{}# exec: bin/check-rsfail\n", HEAD, LIVE_SPEC)).expect("fail descriptor");
    std::fs::write(gates.join("check-rsgone.gate"), format!("{}# exec: bin/check-rsgone\n", HEAD)).expect("gone descriptor");
    let suffix = std::env::consts::EXE_SUFFIX;
    build(RS_PASS, &gates.join(format!("bin/check-rspass{}", suffix)));
    build(RS_FAIL, &gates.join(format!("bin/check-rsfail{}", suffix)));
    let out = run(&dir, "");
    let _ = std::fs::remove_dir_all(&dir);
    assert_live(&out, "check-rspass", "check-rsfail");
    assert!(out.contains("  FAIL: check-rsgone (dispatch harness error, exit 2)"), "{}", out);
    assert!(out.contains("is not an executable file"), "{}", out);
}

const SH_PASS: &str = "#!/usr/bin/env bash\nif [ -f trip ]; then\n  echo 'shpass tripped'\n  echo 'help: remove the trip file'\n  exit 1\nfi\necho 'SHPASS: clean (live)'\n";
const SH_FAIL: &str = "#!/usr/bin/env bash\n# spec: docs/gates.md §live — the live case's invariant\necho \"BIN=$GATE_SDK_NATIVE_BIN\"\necho 'shfail fired'\necho 'help: fix the fixture'\nexit 1\n";

fn shell_member(path: &Path, body: &str) {
    std::fs::write(path, body).expect("shell member");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — a `.sh` member starts under `--run` and
// under `--run-gate-tests` alike: by its own path on a POSIX host, through bash on native Windows
#[test]
fn a_shell_member_starts_under_the_battery_and_the_fixture_runner() {
    let dir = consumer("sh", &["check-shpass", "check-shfail"]);
    let gates = dir.join("gates");
    shell_member(&gates.join("check-shpass.sh"), SH_PASS);
    shell_member(&gates.join("check-shfail.sh"), SH_FAIL);
    let out = run(&dir, "");
    assert_live(&out, "check-shpass", "check-shfail");

    let pairs = dir.join("tests");
    for (case, trip) in [("good", false), ("bad", true)] {
        let c = pairs.join("check-shpass").join(case);
        std::fs::create_dir_all(&c).expect("case dir");
        let want = if trip { "shpass tripped\n" } else { "SHPASS: clean (live)\n" };
        std::fs::write(c.join("expect.txt"), want).expect("expect");
        if trip {
            std::fs::write(c.join("trip"), "").expect("trip");
        }
    }
    let sdk = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repository root").join("gate-sdk");
    let mut c = Command::new(BIN);
    c.arg("--run-gate-tests").arg(&pairs).arg(&gates).current_dir(&dir).stdin(Stdio::null());
    for (name, _) in std::env::vars_os() {
        let n = name.to_string_lossy();
        if n.starts_with("GATE_") || n.contains("_KIT_") || n.starts_with("GIT_") {
            c.env_remove(&name);
        }
    }
    c.env("GATE_SDK_ROOT", &sdk)
        .env("GATE_SDK_KIT_DIRS", &sdk)
        .env("GATE_SDK_GATES_DIR", &gates)
        .env("GATE_SDK_TMP_DIR", dir.join(".tmp"))
        .env("GATE_SDK_NATIVE_BIN", BIN)
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir());
    let done = c.output().expect("cannot run the binary under test");
    let tests = format!("{}{}", String::from_utf8_lossy(&done.stdout), String::from_utf8_lossy(&done.stderr));
    let _ = std::fs::remove_dir_all(&dir);
    assert!(done.status.success(), "{}", tests);
    assert!(tests.contains("GATE-TESTS: clean (1 pairs, 0 unit tests)"), "{}", tests);
}
