// spec: gate-sdk/SPEC.md §lib/gate.sh — the multi-call binary gate_command dispatches
// a .gate-declared registry member to; `--list` reports the subcommand roster
// check-gate-substrate-parity assertion B compares against the descriptors on disk
mod declaration;
mod diff;
mod emit;
mod ere;
mod fresh;
mod gates;
mod json;
mod proc;
mod queue;
mod spec;
mod stages;
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

// spec: gate-sdk/SPEC.md §lib/declaration.sh — the arm reports *classification*, never an
// internal representation: the two holders share no data shape, and a comparison of derived
// literals would fail on a difference that is not a disagreement
fn declaration_parity(args: &[String]) -> i32 {
    let usage = "  usage: checkwright-gates --declaration-parity section <file> <section> | --declaration-parity record <file>";
    let read = |file: &str| -> Result<String, String> {
        std::fs::read_to_string(file).map_err(|e| {
            format!(
                "cannot read {}: {} — the classification could not be reported; treating as failure (not clean)",
                file, e
            )
        })
    };
    let mode = args.first().map(String::as_str);
    let text = match (mode, args.get(1)) {
        (Some("section"), Some(f)) | (Some("record"), Some(f)) => match read(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("checkwright-gates: {}", e);
                return 2;
            }
        },
        _ => {
            eprintln!("checkwright-gates: --declaration-parity needs a mode and a file — the classification could not be reported; treating as failure (not clean)");
            eprintln!("{}", usage);
            return 2;
        }
    };
    match mode {
        Some("section") => {
            let section = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("checkwright-gates: --declaration-parity section needs a section name — the classification could not be reported; treating as failure (not clean)");
                    eprintln!("{}", usage);
                    return 2;
                }
            };
            match declaration::section_bullets(&text, section) {
                None => println!("bullets\tabsent"),
                Some(b) => println!("bullets\t{}", b.len()),
            }
            match declaration::section_tokens(&text, section) {
                declaration::SectionVerdict::Absent => println!("verdict\tabsent"),
                declaration::SectionVerdict::ExplicitNone => println!("verdict\tnone"),
                declaration::SectionVerdict::Tokens(t) => {
                    println!("verdict\ttokens");
                    for tok in t {
                        println!("token\t{}", tok);
                    }
                }
                declaration::SectionVerdict::Unparsed(b) => {
                    println!("verdict\tunparsed");
                    for line in b {
                        println!("unparsed\t{}", line);
                    }
                }
            }
            0
        }
        _ => {
            match declaration::record_tokens(&text) {
                Ok(t) => {
                    println!("record\tok");
                    for tok in t {
                        println!("token\t{}", tok);
                    }
                }
                Err(b) => {
                    println!("record\tmalformed");
                    for line in b {
                        println!("malformed\t{}", line);
                    }
                }
            }
            0
        }
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    let first = match argv.first() {
        Some(a) => a.as_str(),
        None => {
            eprintln!("checkwright-gates: no subcommand given");
            eprintln!("  usage: checkwright-gates --list | --reads <gate-name> | --knobs <gate-name> | --source-stamp | --queue-parity <queue-file> | --declaration-parity section <file> <section> | --declaration-parity record <file> | --emit-<projection> | <gate-name> [args...]");
            eprintln!("  projections: {}", emit::projections().join(", "));
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

    // spec: queue-kit/SPEC.md §lib/queue.sh — the introspection arm the shell/compiled parity
    // harness reads: this module's classification of one queue file, one record per line. A
    // top-level flag rather than a subcommand, for the reason the three around it are:
    // check-gate-substrate-parity assertion B reds a subcommand no descriptor dispatches to,
    // and no gate dispatches here (gate-sdk/SPEC.md §check-gate-substrate-parity).
    if first == "--queue-parity" {
        let file = match argv.get(1) {
            Some(f) => f.as_str(),
            None => {
                eprintln!("checkwright-gates: --queue-parity needs a queue file — the classification could not be reported; treating as failure (not clean)");
                eprintln!("  usage: checkwright-gates --queue-parity <queue-file>");
                exit(2);
            }
        };
        let text = match std::fs::read_to_string(file) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("checkwright-gates: cannot read {}: {} — the classification could not be reported; treating as failure (not clean)", file, e);
                exit(2);
            }
        };
        let sec = match queue::Sections::active_and_deferred() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("checkwright-gates: {} — treating as failure (not clean)", e);
                exit(2);
            }
        };
        for rec in queue::parity_report(&text, &sec) {
            println!("{}", rec);
        }
        exit(0);
    }

    // spec: gate-sdk/SPEC.md §lib/declaration.sh — the standing oracle the dual disposition owes:
    // this module's classification of one input, one record per line, for the harness that holds
    // it against the shell library. A top-level flag rather than a subcommand, for the reason the
    // arms around it are: check-gate-substrate-parity assertion B reds a subcommand no descriptor
    // dispatches to, and no gate dispatches here.
    if first == "--declaration-parity" {
        exit(declaration_parity(&argv[1..]));
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
        // spec: gate-sdk/SPEC.md §The non-gate arm — the knob roster is published through this one
        // arm rather than a second flag, so a front-end asks one question whatever it is about to
        // invoke: a gate, or an arm whose caller must resolve its reads.
        match gates::knobs(name).or_else(|| emit::knobs(name)) {
            Some(knobs) => {
                for k in knobs {
                    println!("{}", k);
                }
                exit(0);
            }
            None => no_such_gate(name),
        }
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the ported emitters, resolved before the registry
    // lookup and absent from `--list`. A thin wrapper by construction: the emission is a library
    // function, so the comparator and the rollup join call it in-process rather than through this.
    if let Some(f) = emit::lookup(first) {
        match f() {
            Ok(doc) => {
                print!("{}", doc);
                exit(0);
            }
            Err(e) => {
                eprintln!("checkwright-gates: {}: {}", first, e);
                exit(2);
            }
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
