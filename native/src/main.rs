// spec: gate-sdk/SPEC.md §lib/gate.sh — the multi-call binary gate_command dispatches
// a .gate-declared registry member to; `--list` reports the subcommand roster
// check-gate-substrate-parity assertion B compares against the descriptors on disk
mod actions;
mod bashscan;
mod declaration;
mod diff;
mod emit;
mod ere;
mod evidence;
mod fresh;
mod gates;
mod guard;
mod hook;
mod install;
mod json;
#[cfg(test)]
mod knobenv;
mod marker;
mod proc;
mod queue;
mod registry;
mod runner;
mod section;
mod sessions;
mod sha256;
mod spec;
mod stages;
mod toolfloor;
#[cfg(test)]
mod usage_tests;
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

// spec: evidence-kit/SPEC.md §lib/evidence.sh — the arm reports *classification* and never an
// internal representation, `--queue-parity`'s own rule: the two holders share no data shape, so a
// comparison of derived literals would fail on a difference that is not a disagreement spec
fn stages_lib_parity(args: &[String]) -> i32 {
    let usage = "  usage: checkwright-gates --stages-lib-parity iter <file>... | cursor <file>... \
| known <name>... | journal <stage>... | written <file>... | mark | open <file>";
    let sub = args.first().map(String::as_str);
    let rest = if args.is_empty() { &args[0..0] } else { &args[1..] };
    match sub {
        Some("iter") => {
            for f in rest {
                let text = std::fs::read_to_string(f).unwrap_or_default();
                let hdr = stages::header(&text).unwrap_or("");
                println!("iter\t{}\t{}", f, stages::header_iter(hdr));
            }
            0
        }
        Some("cursor") => {
            for f in rest {
                let text = std::fs::read_to_string(f).unwrap_or_default();
                println!("cursor\t{}\t{}", f, stages::current_stage(&text));
            }
            0
        }
        Some("known") => {
            let set = match stages::stages() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("checkwright-gates: {}", e);
                    return 2;
                }
            };
            for n in rest {
                println!("known\t{}\t{}", n, stages::stage_known(&set, n));
            }
            0
        }
        Some("journal") => {
            let pattern = match walk::knob_scalar("LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN") {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("checkwright-gates: {}", e);
                    return 2;
                }
            };
            for s in rest {
                println!("journal\t{}\t{}", s, emit::enter_stage::journal_path(&pattern, s));
            }
            0
        }
        Some("written") => {
            for f in rest {
                println!("written\t{}\t{}", f, emit::enter_stage::journal_written(f));
            }
            0
        }
        Some("mark") => {
            println!("mark\t{}", emit::enter_stage::JOURNAL_MARK);
            0
        }
        // spec: lifecycle-kit/SPEC.md §lib/stages.sh — the compiled writer, so the harness holds
        // the shell predicate against the opener's own bytes rather than a lookalike of them
        Some("open") => {
            let Some(path) = rest.first() else {
                eprintln!("checkwright-gates: --stages-lib-parity open needs a path");
                return 2;
            };
            match emit::enter_stage::journal_open(path, "build", "it", "aaaa", "2026-06-01", "none") {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("checkwright-gates: {}", e);
                    2
                }
            }
        }
        _ => {
            eprintln!("checkwright-gates: --stages-lib-parity needs a subcommand — the comparison could not run; treating as failure (not clean)");
            eprintln!("{}", usage);
            2
        }
    }
}

// spec: context-kit/SPEC.md §bin/env-probe — the floor predicate's second holder reporting its
// *classification* over a canned corpus, `--queue-parity`'s own rule: the parse's four
// fields and the verdict's own words, never a rendered profile line.
fn toolfloor_parity(args: &[String]) -> i32 {
    let usage = "  usage: checkwright-gates --toolfloor-parity parse <element>... | --toolfloor-parity check <element> <banner>...";
    match args.first().map(String::as_str) {
        Some("parse") => {
            for e in &args[1..] {
                let p = toolfloor::parse(e);
                println!("parse\t{}\t{}\t{}\t{}\t{}", e, p.name, p.min, p.imp, p.audience);
            }
            0
        }
        // spec: context-kit/SPEC.md §Testing — the corpus is `(element, banner)` *pairs*, so an odd
        // tail is a malformed corpus rather than a banner-less element: refusing it is what keeps a
        // silently-shortened comparison from reading as agreement.
        Some("check") => {
            let rest = &args[1..];
            if rest.len() % 2 != 0 {
                eprintln!("checkwright-gates: --toolfloor-parity check takes (element, banner) pairs and got an odd count — the classification could not be reported; treating as failure (not clean)");
                eprintln!("{}", usage);
                return 2;
            }
            for pair in rest.chunks(2) {
                println!(
                    "check\t{}\t{}\t{}",
                    pair[0],
                    pair[1],
                    toolfloor::check(&pair[0], &pair[1]).rendered()
                );
            }
            0
        }
        _ => {
            eprintln!("checkwright-gates: --toolfloor-parity needs a mode — the classification could not be reported; treating as failure (not clean)");
            eprintln!("{}", usage);
            2
        }
    }
}

// spec: guard-kit/SPEC.md §The guard framework — the holder's four classes, so both sides take one
// spelling; `hd`/`hdq` carry nothing here because the branch reading them is unreachable, and a
// token outside the four is a malformed corpus rather than a silent no-class.
fn parse_wants(spec: &str) -> Result<guard::Wants, String> {
    let mut w = guard::Wants::default();
    if spec == "-" {
        return Ok(w);
    }
    for t in spec.split(',') {
        match t {
            "sq" => w.sq = true,
            "dq" => w.dq = true,
            "hd" | "hdq" => {}
            _ => return Err(t.to_string()),
        }
    }
    Ok(w)
}

// spec: guard-kit/SPEC.md §The guard framework — the standing oracle criterion 6's *unless* clause
// owes for the three twinned primitives: this module's classification of one canned corpus,
// reported as classification and never as an internal representation, `--queue-parity`'s own rule.
fn guard_lib_parity(args: &[String]) -> i32 {
    let usage = "  usage: checkwright-gates --guard-lib-parity split <cmd>... | --guard-lib-parity skeleton <wants> <cmd>... | --guard-lib-parity redirect <cmd>...";
    match args.first().map(String::as_str) {
        Some("split") => {
            for c in &args[1..] {
                for (i, seg) in guard::split_compound(c).iter().enumerate() {
                    println!("split\t{}\t{}\t{}", c, i, seg);
                }
            }
            0
        }
        Some("skeleton") => {
            let spec = match args.get(1) {
                Some(s) => s,
                None => {
                    eprintln!("checkwright-gates: --guard-lib-parity skeleton needs an inert-class list ('-' for none) — the classification could not be reported; treating as failure (not clean)");
                    eprintln!("{}", usage);
                    return 2;
                }
            };
            let w = match parse_wants(spec) {
                Ok(w) => w,
                Err(t) => {
                    eprintln!("checkwright-gates: --guard-lib-parity skeleton got '{}', which is not one of sq, dq, hd, hdq — the classification could not be reported; treating as failure (not clean)", t);
                    eprintln!("{}", usage);
                    return 2;
                }
            };
            for c in &args[2..] {
                match guard::skeleton(c, w) {
                    Ok(s) => println!("skeleton\t{}\t{}\t{}", spec, c, s),
                    // spec: guard-kit/SPEC.md §The guard framework — the twin implements the
                    // newline-free contract, so a newline-bearing corpus is out of contract rather
                    // than something to normalize with a branch this holder does not carry.
                    Err(guard::NewlineInInput) => {
                        eprintln!("checkwright-gates: --guard-lib-parity skeleton was handed a newline-bearing command, which is outside the twin's contract ({} flattens every logged line) — the classification could not be reported; treating as failure (not clean)", guard::LIB);
                        return 2;
                    }
                }
            }
            0
        }
        Some("redirect") => {
            for c in &args[1..] {
                match guard::redirect_pairs(c) {
                    Ok(pairs) => {
                        for (i, p) in pairs.iter().enumerate() {
                            println!("redirect\t{}\t{}\t{}", c, i, p);
                        }
                    }
                    Err(e) => {
                        eprintln!("checkwright-gates: redirect pattern failed to compile: {} — the classification could not be reported; treating as failure (not clean)", e);
                        return 2;
                    }
                }
            }
            0
        }
        _ => {
            eprintln!("checkwright-gates: --guard-lib-parity needs a mode — the classification could not be reported; treating as failure (not clean)");
            eprintln!("{}", usage);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §run-gates — the arms that live in `main` rather than in either lookup
// table, rostered because the normalization below must know an arm name when it sees one.
const TOP_LEVEL_FLAGS: &[&str] = &[
    "--source-stamp",
    "--list",
    "--queue-parity",
    "--toolfloor-parity",
    "--stages-lib-parity",
    "--guard-lib-parity",
    "--install",
    "--reads",
    "--needs",
    "--knobs",
];

fn known_arm(name: &str) -> bool {
    TOP_LEVEL_FLAGS.contains(&name) || emit::lookup(name).is_some() || gates::lookup(name).is_some()
}

// spec: gate-sdk/SPEC.md §run-gates — the front-end's spelling normalized to the arm table's, above
// both doors and recursing through `--knobs` so they cannot disagree. A dashless leading token is
// left alone: only the front-end can tell a gates-dir from a gate name.
fn normalize(argv: Vec<String>) -> Vec<String> {
    let Some(first) = argv.first().cloned() else {
        return argv;
    };
    if first == "--knobs" && argv.len() > 1 {
        let mut out = vec![first];
        out.extend(normalize(argv[1..].to_vec()));
        return out;
    }
    if first == "--emit" {
        if let Some(name) = argv.get(1) {
            let mut out = vec![format!("--emit-{}", name)];
            out.extend_from_slice(&argv[2..]);
            return out;
        }
    }
    if known_arm(&first) {
        return argv;
    }
    if first.starts_with('-') {
        let mut out = vec!["--run".to_string()];
        out.extend(argv);
        return out;
    }
    argv
}

fn main() {
    let argv: Vec<String> = normalize(std::env::args().skip(1).collect());

    let first = match argv.first() {
        Some(a) => a.as_str(),
        None => {
            eprintln!("checkwright-gates: no subcommand given");
            eprintln!("  usage: checkwright-gates --list | --reads <gate-name> | --needs <gate-name> | --knobs <gate-name> | --source-stamp | --queue-parity <queue-file> | --toolfloor-parity <mode> <arg>... | --guard-lib-parity <mode> <arg>... | --install <op> [--<key> <value>]... | --run [--gates-dir <dir>] [--only <name>... | --for <path>...] | --hook <member> | --emit-<arm> | <gate-name> [args...]");
            eprintln!("  bridged arms: {}", emit::arms().join(", "));
            exit(2);
        }
    };

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the baked stamp's only reader. A top-
    // level flag, never a registry member: check-gate-substrate-parity assertion B equates the
    // descriptor set with the `--list` roster, so a stamp arm inside that roster would read as a
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
    // harness reads: this module's classification of one queue file, one record per line.
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

    // spec: context-kit/SPEC.md §bin/env-probe — the standing oracle criterion 6's *unless* clause
    // owes the floor predicate's second holder: this module's classification of one canned corpus,
    // for the harness holding it against `lib/toolfloor.sh`. A top-level flag, like its sibling.
    if first == "--toolfloor-parity" {
        exit(toolfloor_parity(&argv[1..]));
    }

    // spec: lifecycle-kit/SPEC.md §lib/stages.sh — the standing oracle criterion 6's *unless*
    // clause owes a comparator wherever a library stays shell while its readers compile.
    if first == "--stages-lib-parity" {
        exit(stages_lib_parity(&argv[1..]));
    }

    // spec: guard-kit/SPEC.md §scan-prompts — a hardcoded top-level flag, measured rather than
    // assumed: table membership turns on whether the arm resolves a `GUARD_KIT_*` knob, and all
    // three modes resolve none. A top-level flag, like the arms around it.
    if first == "--guard-lib-parity" {
        exit(guard_lib_parity(&argv[1..]));
    }

    // spec: installer/README.md §The install boundary — the install seam both bootstraps call,
    // resolved here before the registry lookup and absent from `--list` like the arms around it.
    if first == "--install" {
        exit(install::run(&argv[1..]));
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — one line per walk root and nothing else: a
    // repo-relative directory or `?`, optionally followed by a tab and the name of the knob
    // whose value filters it. No count line, because the count is derivable from the lines.
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
                for (r, filter) in roots {
                    if filter.is_empty() {
                        println!("{}", r);
                    } else {
                        println!("{}\t{}", r, filter);
                    }
                }
                exit(0);
            }
            None => no_such_gate(name),
        }
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — one line per requirement and nothing
    // else: a program name, or `?` and a tab and the knob whose resolved value's command word is
    // the requirement, or `?` alone. No count line, because the count is derivable from the lines.
    if first == "--needs" {
        let name = match argv.get(1) {
            Some(n) => n.as_str(),
            None => {
                eprintln!("checkwright-gates: --needs needs a gate name — the requirement set could not be reported; treating as failure (not clean)");
                eprintln!("  usage: checkwright-gates --needs <gate-name>");
                exit(2);
            }
        };
        match gates::needs(name) {
            Some(reqs) => {
                for (program, knob) in reqs {
                    if knob.is_empty() {
                        println!("{}", program);
                    } else {
                        println!("{}\t{}", program, knob);
                    }
                }
                exit(0);
            }
            None => no_such_gate(name),
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — one line per declared knob name and nothing else;
    // gate_command reads this to decide which GATE_SDK_KNOB_* elements to resolve into the
    // emitted argv.
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
        match gates::knobs(name)
            .map(<[&str]>::to_vec)
            .or_else(|| emit::knobs(name, &argv[2..]))
        {
            Some(knobs) => {
                for k in knobs {
                    println!("{}", k);
                }
                exit(0);
            }
            None => no_such_gate(name),
        }
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the bridged arms, resolved before the registry
    // lookup and absent from `--list`. A thin wrapper by construction: the emission is a library
    // function, so the comparator and the rollup join call it in-process rather than through this.
    if let Some(arm) = emit::lookup(first) {
        match arm {
            emit::Arm::Emit(f) => match f(&argv[1..]) {
                Ok(doc) => {
                    print!("{}", doc);
                    exit(0);
                }
                Err(e) => {
                    eprintln!("checkwright-gates: {}: {}", first, e);
                    exit(2);
                }
            },
            emit::Arm::Run(f) => exit(f(&argv[1..])),
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
        // spec: gate-sdk/SPEC.md §The path-dialect contract — recorded verdict: cargo's build-time
        // constant reaches only directory-consuming APIs here, `walk::toplevel_in`'s `-C` and
        // `Command::current_dir`, and is never composed, so it is owed no crossing
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let root = crate::walk::toplevel_in(crate_dir).expect("cannot resolve the repo toplevel");

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
