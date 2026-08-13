// spec: gate-sdk/SPEC.md §lib/gate.sh — the multi-call binary gate_command dispatches
// a .gate-declared registry member to; `--list` reports the subcommand roster
// check-gate-substrate-parity assertion B compares against the descriptors on disk
mod ere;
mod gates;
mod proc;
mod queue;
mod spec;
mod walk;

use std::process::exit;

// spec: gate-sdk/SPEC.md §Fail-closed contract — an unknown subcommand is a harness error,
// never a pass; `--reads` refuses through the same help so a descriptor naming a subcommand
// the binary does not carry cannot read as "reads nothing".
fn no_such_gate(name: &str) -> ! {
    eprintln!(
        "checkwright-gates: no such gate subcommand: {} — the check could not run; treating as failure (not clean)",
        name
    );
    let carried: Vec<&str> = gates::names_with_owners().iter().map(|(n, _)| *n).collect();
    eprintln!("  help: this binary carries: {}", carried.join(", "));
    exit(2);
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    let first = match argv.first() {
        Some(a) => a.as_str(),
        None => {
            eprintln!("checkwright-gates: no subcommand given");
            eprintln!("  usage: checkwright-gates --list | --reads <gate-name> | --knobs <gate-name> | --source-stamp | <gate-name> [args...]");
            exit(2);
        }
    };

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the baked stamp's only reader.
    // A top-level flag, never a registry member: check-gate-substrate-parity assertion B
    // equates the descriptor set with the `--list` roster, so a stamp arm inside that
    // roster would read as a subcommand nothing declares.
    if first == "--source-stamp" {
        println!("{}", env!("CHECKWRIGHT_SOURCE_STAMP"));
        exit(0);
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — two tab-separated columns,
    // `<subcommand>\t<owning-kit>`; a column rather than a fifth flag, because a column an
    // older binary does not print degrades where an unknown flag exits 2.
    if first == "--list" {
        for (n, owner) in gates::names_with_owners() {
            println!("{}\t{}", n, owner);
        }
        exit(0);
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — one line per walk root and nothing
    // else: a repo-relative directory, or `?` where the gate cannot bound one statically.
    // No count line, because the count is derivable from the lines.
    if first == "--reads" {
        let name = match argv.get(1) {
            Some(n) => n.as_str(),
            None => {
                eprintln!("checkwright-gates: --reads needs a gate name — the read set could not be reported; treating as failure (not clean)");
                eprintln!("  usage: checkwright-gates --reads <gate-name>");
                exit(2);
            }
        };
        match gates::roots(name) {
            Some(roots) => {
                for r in roots {
                    println!("{}", r);
                }
                exit(0);
            }
            None => no_such_gate(name),
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — one line per declared knob name and nothing else;
    // gate_command reads this to decide which GATE_SDK_KNOB_* elements to resolve into the
    // emitted argv. A top-level flag, outside `--list`'s roster for the reason `--reads` and
    // `--source-stamp` are: check-gate-substrate-parity assertion B equates the descriptor set
    // with that roster, so a flag inside it would read as a subcommand nothing declares.
    if first == "--knobs" {
        let name = match argv.get(1) {
            Some(n) => n.as_str(),
            None => {
                eprintln!("checkwright-gates: --knobs needs a gate name — the config bridge could not report; treating as failure (not clean)");
                eprintln!("  usage: checkwright-gates --knobs <gate-name>");
                exit(2);
            }
        };
        match gates::knobs(name) {
            Some(knobs) => {
                for k in knobs {
                    println!("{}", k);
                }
                exit(0);
            }
            None => no_such_gate(name),
        }
    }

    match gates::lookup(first) {
        Some(f) => exit(f(&argv[1..])),
        None => no_such_gate(first),
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the executed cross-substrate
    // coupling for the stamp, the shape check-knob-default-coupling's disposition sets:
    // the baked constant against the shell library's computation of the same thing
    #[test]
    fn source_stamp_agrees_with_the_shell_library() {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let top = Command::new("git")
            .args(["-C", crate_dir, "rev-parse", "--show-toplevel"])
            .output()
            .expect("cannot run git rev-parse --show-toplevel");
        assert!(top.status.success(), "git rev-parse --show-toplevel failed");
        let root = String::from_utf8_lossy(&top.stdout).trim().to_string();

        let out = Command::new("bash")
            .arg("-c")
            .arg("source gate-sdk/lib/gate.sh; gate_native_source_stamp")
            .current_dir(&root)
            .env("GATE_SDK_NATIVE_CRATE", crate_dir)
            .output()
            .expect("cannot run the shell stamp computation");
        assert!(
            out.status.success(),
            "gate_native_source_stamp failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
        let shell = String::from_utf8_lossy(&out.stdout).trim().to_string();
        assert!(!shell.is_empty(), "gate_native_source_stamp emitted nothing");
        assert_eq!(
            env!("CHECKWRIGHT_SOURCE_STAMP"),
            shell,
            "the baked source stamp and gate-sdk/lib/gate.sh's computation of it have \
             diverged — one side's git invocation is no longer the other's, which is \
             exactly the canonicalization drift git-as-hasher exists to prevent"
        );
    }
}
