// spec: gate-sdk/SPEC.md §lib/gate.sh — the multi-call binary gate_command dispatches
// a .gate-declared registry member to; `--list` reports the subcommand roster
// check-gate-substrate-parity assertion B compares against the descriptors on disk
mod gates;
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
    eprintln!("  help: this binary carries: {}", gates::names().join(", "));
    exit(2);
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    let first = match argv.first() {
        Some(a) => a.as_str(),
        None => {
            eprintln!("checkwright-gates: no subcommand given");
            eprintln!("  usage: checkwright-gates --list | --reads <gate-name> | <gate-name> [args...]");
            exit(2);
        }
    };

    if first == "--list" {
        for n in gates::names() {
            println!("{}", n);
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

    match gates::lookup(first) {
        Some(f) => exit(f(&argv[1..])),
        None => no_such_gate(first),
    }
}
