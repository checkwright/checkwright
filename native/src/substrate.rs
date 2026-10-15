// spec: gate-sdk/SPEC.md §The gate-authoring surface — the PowerShell target class: its extension
// test, its host argument list and its host resolution, shared by a gate's `# exec:` dispatch, a
// PowerShell KPI plugin's start and the scratch runner
use crate::programs::Program;

pub const KNOB: &str = "GATE_SDK_POWERSHELL";

// spec: gate-sdk/SPEC.md §The gate-authoring surface — each flag carries its reason there
pub const HOST_FLAGS: &[&str] = &["-NoLogo", "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"];

// spec: gate-sdk/SPEC.md §The gate-authoring surface — `.ps1` matched without regard to ASCII case
pub fn is_powershell_body(target: &str) -> bool {
    let b = target.as_bytes();
    b.len() >= 4 && b[b.len() - 4..].eq_ignore_ascii_case(b".ps1")
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — a host knob is empty, or names `pwsh` or
// `powershell` as `PATH` names a program, so every host knob accepts one set
pub fn host_refusal(knob: &str, value: &str) -> Option<String> {
    if value.is_empty() || matches!(crate::programs::name_of(value).as_str(), "pwsh" | "powershell") {
        return None;
    }
    Some(format!("{} must be empty, or name pwsh or powershell (got '{}')", knob, value))
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — the host is resolved before the spawn, so a
// spawn that takes its program verbatim receives a path; `ground` is the knob that named it
pub fn resolve_host(ground: &'static str, host: &str) -> Result<Program, String> {
    let program = Program::consumer(ground, host);
    if host.chars().any(std::path::is_separator) {
        return if std::path::Path::new(host).is_file() {
            Ok(program)
        } else {
            Err(unresolved(ground, host))
        };
    }
    match crate::proc::which(host) {
        Some(path) => Ok(program.at(path)),
        None => Err(unresolved(ground, host)),
    }
}

fn unresolved(ground: &str, host: &str) -> String {
    format!("{} names '{}', which does not resolve to a PowerShell host here", ground, host)
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — the argv a PowerShell target starts with:
// the host's flags, the target, then the caller's own arguments
pub fn host_args(target: &str) -> Vec<String> {
    let mut out: Vec<String> = HOST_FLAGS.iter().map(|s| s.to_string()).collect();
    out.push(target.to_string());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The gate-authoring surface — the extension decides, without regard to
    // ASCII case
    #[test]
    fn the_extension_test_ignores_ascii_case() {
        assert!(is_powershell_body(".tmp/x.ps1") && is_powershell_body("X.PS1") && is_powershell_body("a.Ps1"));
        assert!(!is_powershell_body("x.sh") && !is_powershell_body("ps1") && !is_powershell_body("x.ps12"));
    }

    // spec: gate-sdk/SPEC.md §The gate-authoring surface — the validator's accept and refuse sets
    #[test]
    fn the_host_validator_accepts_off_and_the_two_hosts_only() {
        for ok in ["", "pwsh", "powershell", "/opt/microsoft/powershell/7/pwsh", "tools/powershell"] {
            assert_eq!(host_refusal("K", ok), None, "{}", ok);
        }
        for bad in ["bash", "pwsh7", "/usr/bin/python3", "off"] {
            let e = host_refusal("K", bad).unwrap_or_default();
            assert!(e.starts_with("K must be empty") && e.contains(bad), "{}: {}", bad, e);
        }
    }

    // spec: gate-sdk/SPEC.md §The gate-authoring surface — the argument list, and the scratch runner
    // spawning a body with the same one
    #[test]
    fn the_argument_list_ends_in_the_target_and_is_the_scratch_runners() {
        let a = host_args("g.ps1");
        assert_eq!(a.last().map(String::as_str), Some("g.ps1"));
        assert_eq!(&a[..a.len() - 1], HOST_FLAGS);
        let scratch = include_str!("emit/scratch_run.rs");
        assert!(
            scratch.contains("use crate::substrate::{is_powershell_body, HOST_FLAGS};") && !scratch.contains("\"-NoProfile\""),
            "the scratch runner spells its own host flags"
        );
    }

    // spec: gate-sdk/SPEC.md §The gate-authoring surface — a path host must be a file and a bare one
    // must resolve on `PATH`; either refusal names the knob
    #[test]
    fn an_unresolvable_host_is_refused_naming_its_knob() {
        let e = resolve_host("GATE_SDK_POWERSHELL", "/no/such/dir/pwsh").err().unwrap_or_default();
        assert!(e.contains("GATE_SDK_POWERSHELL") && e.contains("does not resolve"), "{}", e);
        let e = resolve_host("GATE_SDK_POWERSHELL", "checkwright-no-such-host").err().unwrap_or_default();
        assert!(e.contains("does not resolve"), "{}", e);
        let bash = crate::proc::which("bash").expect("bash");
        let p = resolve_host("GATE_SDK_POWERSHELL", &bash).expect("a file path resolves as given");
        assert_eq!(p.invocation(), bash);
    }
}
