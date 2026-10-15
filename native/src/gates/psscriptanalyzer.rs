// spec: gate-sdk/SPEC.md §check-psscriptanalyzer — PSScriptAnalyzer lint of the PowerShell family
// at Warning and above (the self-lint contract), the wrapper class `check-shellcheck` is: the
// analyser is a declared requirement this starts through the consumer's host and refuses without
use crate::{proc, registry, substrate, walk};

const NAME: &str = "check-psscriptanalyzer";
const EXCLUDE_KNOB: &str = "GATE_SDK_PSSCRIPTANALYZER_EXCLUDE";

// spec: gate-sdk/SPEC.md §check-psscriptanalyzer — §check-shellcheck's per-kit directories
const KIT_SUBDIRS: &[&str] = &["lib", "bin", "checks", "templates", "smoke"];

// spec: gate-sdk/SPEC.md §check-psscriptanalyzer — the two values handed to the script, one per
// line, so no path or rule name is composed into the command text
const TARGETS_VAR: &str = "CHECKWRIGHT_PSSA_TARGETS";
const EXCLUDE_VAR: &str = "CHECKWRIGHT_PSSA_EXCLUDE";

// spec: gate-sdk/SPEC.md §check-psscriptanalyzer — the module's constant script: one line, single
// quotes only, so a Windows command line carries it unescaped; exit 3 is the absent module
const SCRIPT: &str = "$ErrorActionPreference = 'Stop'; \
if (-not (Get-Module -ListAvailable -Name PSScriptAnalyzer)) { exit 3 }; \
Import-Module PSScriptAnalyzer; \
$nl = [string][char]10; \
$targets = @($env:CHECKWRIGHT_PSSA_TARGETS -split $nl | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne '' }); \
$exclude = @($env:CHECKWRIGHT_PSSA_EXCLUDE -split $nl | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne '' }); \
$found = 0; \
foreach ($t in $targets) { \
$p = @{ Path = $t; Severity = @('Warning', 'Error', 'ParseError') }; \
if ($exclude.Count -gt 0) { $p.ExcludeRule = $exclude }; \
foreach ($r in @(Invoke-ScriptAnalyzer @p)) { $found++; [Console]::Out.WriteLine(('{0}:{1}: {2} {3}' -f $t, $r.Line, $r.RuleName, ($r.Message -replace '\\s+', ' '))) } \
}; \
if ($found -gt 0) { exit 1 }; exit 0";

// spec: gate-sdk/SPEC.md §check-psscriptanalyzer — the derived corpus: the gates dir, each kit
// root's directories, the extra-dirs knob, then each registered exec member's PowerShell target;
// positional dirs replace the whole set, the hermetic fixture affordance
fn targets(args: &[String]) -> Result<(Vec<String>, Vec<String>), String> {
    let (dirs, exec) = if !args.is_empty() {
        (args.to_vec(), Vec::new())
    } else {
        let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
        let kit_roots = walk::kit_roots_abs()?;
        let mut dirs = vec![gates_dir.clone()];
        for kit in &kit_roots {
            for sub in KIT_SUBDIRS {
                dirs.push(format!("{}/{}", kit, sub));
            }
        }
        dirs.extend(walk::knob_words("GATE_SDK_LINT_EXTRA_DIRS")?);
        (dirs, exec_targets(&gates_dir, &kit_roots)?)
    };
    let mut out: Vec<String> = Vec::new();
    for d in &dirs {
        for f in walk::glob_entries(&format!("{}/*", d)) {
            if substrate::is_powershell_body(&f) && std::path::Path::new(&f).is_file() && !out.contains(&f) {
                out.push(f);
            }
        }
    }
    for t in exec {
        if !out.contains(&t) {
            out.push(t);
        }
    }
    Ok((dirs, out))
}

fn exec_targets(gates_dir: &str, kit_roots: &[String]) -> Result<Vec<String>, String> {
    let list = registry::list_path(gates_dir);
    let Ok(text) = std::fs::read(&list) else {
        return Ok(Vec::new());
    };
    let dirs = registry::resolve_dirs(gates_dir, kit_roots);
    let mut out: Vec<String> = Vec::new();
    for m in registry::members(&String::from_utf8_lossy(&text)) {
        let Some(decl) = registry::resolve(m.trim(), &dirs) else {
            continue;
        };
        if let registry::Substrate::Exec(v) = registry::substrate(&decl)? {
            let t = registry::exec_target(&decl, &v);
            if substrate::is_powershell_body(&t) {
                out.push(t);
            }
        }
    }
    Ok(out)
}

fn refuse(lines: &[String]) -> i32 {
    for l in lines {
        eprintln!("{}", l);
    }
    2
}

pub fn run(args: &[String]) -> i32 {
    let host = match walk::knob_scalar(substrate::KNOB) {
        Ok(h) => h,
        Err(e) => return refuse(&[format!("{}: {}", NAME, e)]),
    };
    if host.is_empty() {
        println!("PSSCRIPTANALYZER: clean (the PowerShell host is off — {} is empty, so no PowerShell is linted)", substrate::KNOB);
        return 0;
    }
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the host is probed before the targets are
    // globbed, so a tree with nothing to lint and no host reports the host
    let program = match substrate::resolve_host("GATE_SDK_POWERSHELL", &host) {
        Ok(p) => p,
        Err(e) => {
            return refuse(&[
                format!("{}: {} — the gate cannot run.", NAME, e),
                "  A gate that cannot run is not clean (fail-closed).".to_string(),
                format!("  help: install PowerShell, or set {} to a host that resolves, or empty to switch the lint off.", substrate::KNOB),
            ])
        }
    };
    let (dirs, files) = match targets(args) {
        Ok(t) => t,
        Err(e) => return refuse(&[format!("{}: {}", NAME, e)]),
    };
    if files.is_empty() {
        return refuse(&[format!("{}: no *.ps1 found under: {} — nothing to lint.", NAME, dirs.join(" "))]);
    }
    let exclude = match walk::knob_words(EXCLUDE_KNOB) {
        Ok(e) => e,
        Err(e) => return refuse(&[format!("{}: {}", NAME, e)]),
    };
    let env = vec![(TARGETS_VAR.to_string(), files.join("\n")), (EXCLUDE_VAR.to_string(), exclude.join("\n"))];
    let argv = ["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", SCRIPT];
    let done = match proc::run_merged_in(&program, &argv, &env, None) {
        Ok(d) => d,
        Err(e) => return refuse(&[format!("{}: {}", NAME, e)]),
    };
    let report = String::from_utf8_lossy(done.output()).trim_end_matches(['\n', '\r']).to_string();
    match done.code() {
        Some(0) => {
            println!("PSSCRIPTANALYZER: clean ({} script(s) at Warning and above, {} rule(s) excluded)", files.len(), exclude.len());
            0
        }
        Some(1) => {
            println!("{}", report);
            println!();
            println!("help: PSScriptAnalyzer flagged the script(s) above (Warning and above). Fix each finding,");
            println!("      or silence a genuine false positive at its site with");
            println!("      [Diagnostics.CodeAnalysis.SuppressMessageAttribute('<rule>', '', Justification = '<why>')];");
            println!("      a rule your tree contradicts on every script goes in {}.", EXCLUDE_KNOB);
            1
        }
        Some(3) => refuse(&[
            format!("{}: the PSScriptAnalyzer module is not installed for {} — the gate cannot run.", NAME, program.invocation()),
            "  A gate that cannot run is not clean (fail-closed).".to_string(),
            "  help: install it for that host: Install-Module PSScriptAnalyzer -Scope CurrentUser".to_string(),
        ]),
        other => refuse(&[
            format!("{}: {} exited {:?} — the check could not run; treating as failure (not clean)", NAME, program.invocation(), other),
            report,
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-psscriptanalyzer — no path or rule name reaches the command
    // text, which carries no double quote a Windows command line would have to escape
    #[test]
    fn the_script_is_a_constant_reading_its_inputs_from_the_environment() {
        assert!(!SCRIPT.contains('"') && !SCRIPT.contains('\n'));
        assert!(SCRIPT.contains(&format!("$env:{}", TARGETS_VAR)) && SCRIPT.contains(&format!("$env:{}", EXCLUDE_VAR)));
        assert!(SCRIPT.contains("'Warning', 'Error', 'ParseError'") && SCRIPT.contains("exit 3"));
    }

    // spec: gate-sdk/SPEC.md §check-psscriptanalyzer — the exit mapping over a stub host: off is
    // clean, the analyser's 0 and 1 pass through, its absent module and any other status are 2
    #[cfg(unix)]
    #[test]
    fn the_exit_mapping_over_a_stub_host() {
        use std::os::unix::fs::PermissionsExt;
        let env = crate::knobenv::lock();
        let d = std::env::temp_dir().join(format!("cw-pssa-exit-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        std::fs::write(d.join("x.ps1"), "").expect("target");
        let host = d.join("pwsh");
        let dir = walk::cross_arg(&d.display().to_string());
        let mut got: Vec<i32> = Vec::new();
        env.set(substrate::KNOB, "");
        crate::knobs::reset(&env);
        got.push(run(std::slice::from_ref(&dir)));
        env.set(substrate::KNOB, &host.display().to_string());
        crate::knobs::reset(&env);
        for code in [0, 1, 3, 7] {
            std::fs::write(&host, format!("#!/bin/sh\nexit {}\n", code)).expect("stub");
            std::fs::set_permissions(&host, std::fs::Permissions::from_mode(0o755)).expect("chmod");
            got.push(run(std::slice::from_ref(&dir)));
        }
        std::fs::remove_file(&host).expect("drop the host");
        got.push(run(std::slice::from_ref(&dir)));
        env.remove(substrate::KNOB);
        crate::knobs::reset(&env);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(got, vec![0, 0, 1, 2, 2, 2]);
    }

    // spec: gate-sdk/SPEC.md §check-psscriptanalyzer — positional dirs replace the derived set, and
    // the extension is matched without regard to case
    #[test]
    fn positionals_replace_the_corpus_and_the_extension_ignores_case() {
        let d = std::env::temp_dir().join(format!("cw-pssa-corpus-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        for f in ["a.ps1", "B.PS1", "c.sh", "d.ps12"] {
            std::fs::write(d.join(f), "").expect("file");
        }
        let dir = walk::cross_arg(&d.display().to_string());
        let (dirs, files) = targets(std::slice::from_ref(&dir)).expect("positional corpus");
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(dirs, vec![dir.clone()]);
        assert_eq!(files, vec![format!("{}/B.PS1", dir), format!("{}/a.ps1", dir)]);
    }
}
