// spec: gate-sdk/SPEC.md §The gate-authoring surface — the live dispatch of an exec member: a
// scratch consumer registers a passing and a failing PowerShell gate, and the battery built for
// this test run starts each under every host that resolves here, or refuses where none is set

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
