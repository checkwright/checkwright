// spec: gate-sdk/SPEC.md §lib/gate.sh — the multi-call binary gate_command dispatches
// a .gate-declared registry member to; `--list` reports the subcommand roster
// check-gate-substrate-parity assertion B compares against the descriptors on disk
mod gates;
mod walk;

use std::process::exit;

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    let first = match argv.first() {
        Some(a) => a.as_str(),
        None => {
            eprintln!("checkwright-gates: no subcommand given");
            eprintln!("  usage: checkwright-gates --list | <gate-name> [args...]");
            exit(2);
        }
    };

    if first == "--list" {
        for n in gates::names() {
            println!("{}", n);
        }
        exit(0);
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — an unknown subcommand is a
    // harness error, never a pass
    match gates::lookup(first) {
        Some(f) => exit(f(&argv[1..])),
        None => {
            eprintln!(
                "checkwright-gates: no such gate subcommand: {} — the check could not run; treating as failure (not clean)",
                first
            );
            eprintln!("  help: this binary carries: {}", gates::names().join(", "));
            exit(2);
        }
    }
}
