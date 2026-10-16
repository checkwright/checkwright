// spec: gate-sdk/SPEC.md §lib/gate.sh — `--emit gate-argv`, `gate_command`'s answer for a `.gate`
// declaration: the invocation the battery starts the member with, one element per line, so no
// shell caller holds a second copy of the `# exec:` grammar or the host's arguments
use crate::registry;

pub const KNOBS: &[&str] = &["GATE_SDK_POWERSHELL"];

pub const USAGE: &str = "usage: --emit gate-argv <declaration>
  <declaration> is a resolved <dir>/<name>.sh or <dir>/<name>.gate; prints its invocation argv,
  one element per line, and refuses at exit 2 where the member cannot start.";

pub fn emit(args: &[String]) -> Result<String, String> {
    let [decl] = args else {
        return Err(USAGE.to_string());
    };
    if decl.starts_with('-') {
        return Err(format!("unrecognized option: {}\n{}", decl, USAGE));
    }
    let base = decl.rsplit('/').next().unwrap_or(decl);
    let Some(name) = base.strip_suffix(".gate").or_else(|| base.strip_suffix(".sh")) else {
        return Err(format!("{} is neither a .sh nor a .gate declaration\n{}", decl, USAGE));
    };
    let exe = crate::proc::self_image()
        .map_err(|e| format!("cannot resolve this binary's own path: {}", e))?
        .display()
        .to_string();
    argv(name, decl, &exe)
}

fn argv(name: &str, decl: &str, exe: &str) -> Result<String, String> {
    let inv = registry::invocation(name, decl, exe)?;
    let mut out = format!("{}\n", inv.program.invocation());
    for a in inv.args {
        out.push_str(&a);
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("cw-gate-argv-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        d
    }

    // spec: gate-sdk/SPEC.md §The gate-authoring surface — the arm's answer per start and its refusals
    #[test]
    fn each_substrate_prints_its_argv_and_an_unstartable_one_refuses() {
        let env = crate::knobenv::lock();
        let d = scratch("answers");
        let dir = crate::walk::cross_arg(&d.display().to_string());
        let host = format!("{}/pwsh", dir);
        std::fs::write(d.join("pwsh"), "").expect("host");
        std::fs::write(d.join("check-sh.sh"), "#!/bin/bash\n").expect("sh");
        std::fs::write(d.join("check-shellcheck.gate"), "# graph: couples=x\n").expect("compiled");
        std::fs::write(d.join("check-ps.gate"), "# graph: couples=x\n# exec: check-ps.ps1\n").expect("exec");
        std::fs::write(d.join("check-ps.ps1"), "exit 0\n").expect("target");
        std::fs::write(d.join("check-txt.gate"), "# exec: check-ps.txt\n").expect("txt");
        std::fs::write(d.join("check-ps.txt"), "\n").expect("txt target");
        std::fs::write(d.join("check-gone.gate"), "# exec: gone.ps1\n").expect("gone");
        std::fs::write(d.join("check-rs.gate"), "# exec: bin/check-rs\n").expect("executable");
        std::fs::create_dir_all(d.join("bin")).expect("bin");
        let program = d.join(crate::substrate::executable_path("bin/check-rs"));
        std::fs::write(&program, "").expect("program");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).expect("chmod");
        }
        std::fs::write(d.join("check-unbuilt.gate"), "# exec: bin/check-unbuilt\n").expect("unbuilt");
        env.set("GATE_SDK_POWERSHELL", &host);
        crate::knobs::reset(&env);
        let rs = argv("check-rs", &format!("{}/check-rs.gate", dir), "/bin/x");
        let unbuilt = argv("check-unbuilt", &format!("{}/check-unbuilt.gate", dir), "/bin/x");
        let sh = argv("check-sh", &format!("{}/check-sh.sh", dir), "/bin/x");
        let compiled = argv("check-shellcheck", &format!("{}/check-shellcheck.gate", dir), "/bin/x");
        let exec = argv("check-ps", &format!("{}/check-ps.gate", dir), "/bin/x");
        let txt = argv("check-txt", &format!("{}/check-txt.gate", dir), "/bin/x");
        let gone = argv("check-gone", &format!("{}/check-gone.gate", dir), "/bin/x");
        let undeclared = argv("check-nope", &format!("{}/check-shellcheck.gate", dir), "/bin/x");
        env.set("GATE_SDK_POWERSHELL", "");
        crate::knobs::reset(&env);
        let off = argv("check-ps", &format!("{}/check-ps.gate", dir), "/bin/x");
        env.remove("GATE_SDK_POWERSHELL");
        crate::knobs::reset(&env);
        let _ = std::fs::remove_dir_all(&d);
        let decl = format!("{}/check-sh.sh", dir);
        // spec: gate-sdk/SPEC.md §The gate-authoring surface — native Windows starts a shell member
        // through bash, every other host by its own path
        if cfg!(windows) {
            let sh = sh.unwrap();
            let lines: Vec<&str> = sh.lines().collect();
            assert_eq!(crate::programs::name_of(lines[0]), "bash", "{}", sh);
            assert_eq!(&lines[1..], [decl.as_str()], "{}", sh);
        } else {
            assert_eq!(sh.unwrap(), format!("{}\n", decl));
        }
        assert_eq!(rs.unwrap(), format!("{}/{}\n", dir, crate::substrate::executable_path("bin/check-rs")));
        let unbuilt = unbuilt.unwrap_err();
        assert!(unbuilt.contains("is not an executable file") && unbuilt.contains("your own build"), "{}", unbuilt);
        assert_eq!(compiled.unwrap(), "/bin/x\ncheck-shellcheck\n");
        assert_eq!(
            exec.unwrap(),
            format!("{}\n-NoLogo\n-NoProfile\n-NonInteractive\n-ExecutionPolicy\nBypass\n-File\n{}/check-ps.ps1\n", host, dir)
        );
        assert!(txt.unwrap_err().contains("no admitted target class"));
        assert!(gone.unwrap_err().contains("is not a file"));
        assert!(undeclared.unwrap_err().contains("no such gate subcommand: check-nope"));
        let off = off.unwrap_err();
        assert!(off.contains("GATE_SDK_POWERSHELL is empty") && off.contains("pwsh or powershell"), "{}", off);
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the operand is one declaration path, never a flag
    #[test]
    fn the_operand_is_one_declaration() {
        assert!(emit(&[]).unwrap_err().starts_with("usage:"));
        assert!(emit(&["--x".to_string()]).unwrap_err().contains("unrecognized option"));
        assert!(emit(&["a/b.txt".to_string()]).unwrap_err().contains("neither a .sh nor a .gate"));
    }
}
