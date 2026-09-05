// spec: gate-sdk/SPEC.md §run-gate-tests — the golden-fixture runner, bridged as an `Arm::Run`
// member: its contract is a three-valued exit — 0 clean, 1 a logic failure in a gate or a unit
// test, 2 a harness or fixture error — which `Arm::Emit` collapses to 0-or-2.
// spec: gate-sdk/SPEC.md §The non-gate arm — a bridged-arm table member rather than a hardcoded
// top-level flag, because the member is configured: a top-level flag would resolve platform
// defaults and silently ignore every consumer override of the four knobs below.
use crate::proc;
use crate::walk;
use std::path::Path;

pub const KNOBS: &[&str] = &[
    "GATE_SDK_TESTS_DIR",
    "GATE_SDK_TMP_DIR",
    "GATE_SDK_NATIVE_BIN",
    "GATE_KIT_ROOTS_HERE",
];

const NAME: &str = "run-gate-tests";

// spec: gate-sdk/SPEC.md §run-gate-tests — the invoker-root resolutions, taken once before the
// pair loop: the gate-declaration dirs and the dispatch binary resolve here, while a case's knob
// *values* resolve inside the case dir.
struct Harness {
    sdk: String,
    bin: String,
    gate_dirs: Vec<String>,
    // spec: gate-sdk/SPEC.md §run-gate-tests — the absolutized binary is the shell form's
    // process-wide export: every child the runner spawns inherits it, so it rides both the case
    // invocation and the bespoke `*.test.sh` runs.
    exported: Vec<(String, String)>,
    case_tmp: String,
}

pub fn run(args: &[String]) -> i32 {
    let harness = match setup(args) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    let tests_dir = match tests_dir(args) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    if !Path::new(&tests_dir).is_dir() {
        eprintln!("{}: no fixture tree at {}", NAME, tests_dir);
        return 2;
    }
    execute(&harness, &tests_dir)
}

fn tests_dir(args: &[String]) -> Result<String, String> {
    match args.first() {
        Some(a) => Ok(a.clone()),
        None => walk::knob_scalar("GATE_SDK_TESTS_DIR"),
    }
}

fn setup(args: &[String]) -> Result<Harness, String> {
    let here = walk::cwd()?;
    let sdk = sdk_root()?;
    let requested: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        check_dirs(&sdk)?
    };
    // spec: gate-sdk/SPEC.md §run-gate-tests — the second position replaces the resolved default
    // and drops a non-existent member silently; that fail-open is carried rather than fixed, the
    // entry named there owning it.
    let gate_dirs: Vec<String> = requested
        .iter()
        .filter(|d| Path::new(d.as_str()).is_dir())
        .map(|d| walk::abs_against(&here, d))
        .collect();

    // spec: gate-sdk/SPEC.md §run-gate-tests — the binary is absolutized at the invoker's root,
    // because a case runs after a `cd` into its own dir and the knob's default is deliberately
    // repo-relative (§lib/gate.sh); the gate dirs above are absolutized for the same reason.
    let mut bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN")?;
    let mut exported: Vec<(String, String)> = Vec::new();
    let absolutized = walk::abs_against(&here, &bin);
    if absolutized != bin && Path::new(&bin).exists() {
        bin = absolutized;
        exported.push(("GATE_SDK_NATIVE_BIN".to_string(), bin.clone()));
    }

    // spec: gate-sdk/SPEC.md §run-gate-tests — the scratch dir absolutized for the mirror of that
    // reason, and pinned on the case invocation alone: the pin's scope is the pair loop, never
    // this process.
    let case_tmp = walk::abs_against(&here, &walk::knob_scalar("GATE_SDK_TMP_DIR")?);

    Ok(Harness {
        sdk,
        bin,
        gate_dirs,
        exported,
        case_tmp,
    })
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the arm reaches gate-sdk's own library through the
// transported kit roots rather than through a path relative to itself: a compiled member has no
// `BASH_SOURCE` anchor, and a binary the installer copied elsewhere cannot recover one.
fn sdk_root() -> Result<String, String> {
    walk::kit_roots_abs()?
        .into_iter()
        .find(|r| r.rsplit('/').next() == Some("gate-sdk"))
        .ok_or_else(|| {
            format!(
                "{}: GATE_KIT_ROOTS_HERE names no gate-sdk root, so the shell library this runner \
                 resolves each case's dispatch through cannot be found",
                NAME
            )
        })
}

// spec: gate-sdk/SPEC.md §run-gate-tests — the default gate-declaration dir set is `gate_check_dirs`'
// own answer, read out of a bash that sources the unchanged library: the duplication-absent road
// §The non-gate arm records, so the crate learns the set and nothing about how it was derived.
fn check_dirs(sdk: &str) -> Result<Vec<String>, String> {
    let script = r#"source "$1/lib/gate.sh"; gate_check_dirs"#;
    let done = proc::run_streamed("bash", &["-c", script, "bash", sdk], b"", proc::Stderr::Inherit)?;
    if done.code() != 0 {
        return Err(format!(
            "{}: the shell library could not resolve the gate-declaration dirs",
            NAME
        ));
    }
    Ok(String::from_utf8_lossy(done.stdout())
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect())
}

// spec: gate-sdk/SPEC.md §run-gate-tests — every subdirectory of a tests dir is read as a case
// pair and `<tests-dir>/*.test.sh` as a unit test; the runner exits 2 only when it holds
// **neither**, so a gateless kit's unit-test-only dir is a lawful tests dir printing `0 pairs`.
fn execute(h: &Harness, tests_dir: &str) -> i32 {
    let (gate_dirs, unit_tests) = discover(tests_dir);
    if gate_dirs.is_empty() && unit_tests.is_empty() {
        eprintln!(
            "{}: no gate fixture dirs and no *.test.sh under {}",
            NAME, tests_dir
        );
        return 2;
    }

    let mut pairs = 0;
    let mut logic_fail = 0;
    let mut harness_fail = 0;

    for gate in &gate_dirs {
        let dir = format!("{}/{}", tests_dir, gate);
        let good = format!("{}/good", dir);
        let bad = format!("{}/bad", dir);
        if !Path::new(&good).is_dir() || !Path::new(&bad).is_dir() {
            println!("  HARNESS: {} is missing a good/ or bad/ case dir", gate);
            harness_fail += 1;
            continue;
        }
        let bad_expect_path = format!("{}/expect.txt", bad);
        if !Path::new(&bad_expect_path).is_file() {
            println!(
                "  HARNESS: {} bad/ has no expect.txt (a rejection substring is required)",
                gate
            );
            harness_fail += 1;
            continue;
        }
        let good_expect = read_expect(&format!("{}/expect.txt", good));
        let bad_expect = read_expect(&bad_expect_path);

        pairs += 1;

        let gc = run_case(h, gate, &good, 0, &good_expect);
        let bc = run_case(h, gate, &bad, 1, &bad_expect);
        if gc == 1 || bc == 1 {
            logic_fail += 1;
        }
        if gc == 2 || bc == 2 {
            harness_fail += 1;
        }
    }

    let mut unit = 0;
    let mut unit_fail = 0;
    for t in &unit_tests {
        unit += 1;
        let path = format!("{}/{}", tests_dir, t);
        match proc::run_merged_in("bash", &[path.as_str()], &h.exported, None) {
            Ok(m) if m.succeeded() => {}
            Ok(m) => {
                println!("  FAIL: {}", t);
                report(&text(m.output()));
                unit_fail += 1;
            }
            Err(e) => {
                println!("  FAIL: {}", t);
                report(&e);
                unit_fail += 1;
            }
        }
    }

    println!();
    if harness_fail > 0 {
        println!(
            "GATE-TESTS: {} harness/fixture error(s) (malformed fixtures — could not test)",
            harness_fail
        );
        return 2;
    }
    if logic_fail > 0 || unit_fail > 0 {
        println!(
            "GATE-TESTS: {} of {} gate(s) + {} of {} unit test(s) misbehaved",
            logic_fail, pairs, unit_fail, unit
        );
        return 1;
    }
    println!("GATE-TESTS: clean ({} pairs, {} unit tests)", pairs, unit);
    0
}

// spec: gate-sdk/SPEC.md §run-gate-tests — the two globs the shell form takes, in the order bash
// expands them: dotfiles excluded, sorted by name, and a `*/` match follows a symlinked directory.
fn discover(tests_dir: &str) -> (Vec<String>, Vec<String>) {
    let mut dirs: Vec<String> = Vec::new();
    let mut tests: Vec<String> = Vec::new();
    let Ok(entries) = walk::list_dir(Path::new(tests_dir)) else {
        return (dirs, tests);
    };
    for (name, _) in entries {
        if name.starts_with('.') {
            continue;
        }
        // spec: gate-sdk/SPEC.md §run-gate-tests — the classification follows a symlink, because
        // bash's own `*/` does: the entry's own type would read a symlinked case dir as a file.
        if Path::new(&format!("{}/{}", tests_dir, name)).is_dir() {
            dirs.push(name);
        } else if name.ends_with(".test.sh") {
            tests.push(name);
        }
    }
    dirs.sort();
    tests.sort();
    (dirs, tests)
}

fn read_expect(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

// spec: gate-sdk/SPEC.md §run-gate-tests — a captured child's output is the shell form's `$( … )`,
// which strips every trailing newline before the caller prints it.
fn text(out: &[u8]) -> String {
    String::from_utf8_lossy(out)
        .trim_end_matches('\n')
        .to_string()
}

// spec: gate-sdk/SPEC.md §run-gate-tests — the shell form's `printf '    %s\n' "$out"`: one format
// application over the whole capture, so the first line carries the indent and the rest print as
// the child wrote them.
fn report(out: &str) {
    println!("    {}", out);
}

fn basename(p: &str) -> &str {
    p.rsplit('/').next().unwrap_or(p)
}

// spec: gate-sdk/SPEC.md §run-gate-tests — one case: resolve the dispatch inside the case dir,
// take the executable guard and the absolutization on the first element that is neither `env` nor
// an assignment, then run it from inside the case dir with the scratch pin.
fn run_case(h: &Harness, gate: &str, casedir: &str, want: i32, expect: &str) -> i32 {
    let Some(mut argv) = resolve_argv(h, gate, casedir) else {
        println!(
            "  HARNESS: {} resolves in none of: {}",
            gate,
            h.gate_dirs.join(" ")
        );
        return 2;
    };
    if argv.is_empty() {
        println!(
            "  HARNESS: {} resolves in none of: {}",
            gate,
            h.gate_dirs.join(" ")
        );
        return 2;
    }
    // spec: gate-sdk/SPEC.md §run-gate-tests — a bridged argv leads with `env` and its NAME=VALUE
    // elements, so the dispatch executable is the first element that is neither, and it is that
    // element the executable guard and the absolutization take.
    let x = dispatch_index(&argv);
    if x >= argv.len() || !proc::is_executable(Path::new(&argv[x])) {
        println!(
            "  HARNESS: {} is not executable",
            argv
                .get(x)
                .map(String::as_str)
                .unwrap_or("<no dispatch executable in argv>")
        );
        return 2;
    }
    let here = match walk::cwd() {
        Ok(c) => c,
        Err(e) => {
            println!("  HARNESS: {}", e);
            return 2;
        }
    };
    argv[x] = walk::abs_against(&here, &argv[x]);

    // spec: gate-sdk/SPEC.md §run-gate-tests — `#` lines are stripped and the surviving text is
    // word-split on whitespace into argv, never taken one argument per line; the deliberate
    // narrowing recorded there is that this splits without the shell form's pathname expansion.
    let mut args: Vec<String> = Vec::new();
    if let Ok(body) = std::fs::read_to_string(format!("{}/args", casedir)) {
        for line in body.lines() {
            if line.starts_with('#') {
                continue;
            }
            args.extend(line.split_whitespace().map(String::from));
        }
    }

    let mut env = h.exported.clone();
    env.push(("GATE_SDK_TMP_DIR".to_string(), h.case_tmp.clone()));
    let mut spawn: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    spawn.extend(args.iter().map(String::as_str));
    // spec: gate-sdk/SPEC.md §run-gate-tests — the working directory is set on the *child* rather
    // than entered by this process, so no code path can leave the harness in a case dir.
    let done = match proc::run_merged_in(&argv[0], &spawn, &env, Some(Path::new(casedir))) {
        Ok(m) => m,
        Err(e) => {
            println!("  HARNESS: {} {} exited 2 (gate could not run / malformed fixture):", gate, casedir);
            report(&e);
            return 2;
        }
    };
    let out = text(done.output());
    let rc = done.reported_code();

    if rc == 2 {
        println!(
            "  HARNESS: {} {} exited 2 (gate could not run / malformed fixture):",
            gate, casedir
        );
        report(&out);
        return 2;
    }
    if rc != want {
        println!(
            "  FAIL: {} {} expected exit {}, got {}",
            gate,
            basename(casedir),
            want,
            rc
        );
        report(&out);
        return 1;
    }
    // spec: gate-sdk/SPEC.md §run-gate-tests — an `expect.txt` is a conjunction: order-independent,
    // a blank line asserting nothing, and every missing line named rather than the first.
    let missing: Vec<&str> = expect
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter(|l| !contains_line(&out, l))
        .collect();
    if !missing.is_empty() {
        println!(
            "  FAIL: {} {} exit {} OK but output lacks expected line(s):",
            gate,
            basename(casedir),
            rc
        );
        for m in &missing {
            println!("        missing: {}", m);
        }
        report(&out);
        return 1;
    }

    // spec: gate-sdk/SPEC.md §Output contract — asserted at runtime, both cases: a `good/` case
    // must emit the canonical clean line and a `bad/` case a `help:` remedy line, on top of exit
    // code and `expect.txt`.
    if want == 0 {
        if !out.lines().any(clean_line) {
            println!(
                "  FAIL: {} {} exited 0 but emitted no '<NAME>: clean (…)' line",
                gate,
                basename(casedir)
            );
            report(&out);
            return 1;
        }
    } else if !out.lines().any(help_line) {
        println!(
            "  FAIL: {} {} reported a violation with no 'help:' remedy line",
            gate,
            basename(casedir)
        );
        report(&out);
        return 1;
    }
    0
}

// spec: gate-sdk/SPEC.md §run-gate-tests — the dispatch resolution is `gate_command`'s own answer,
// read out of a bash that sources the unchanged `lib/gate.sh` with the `cd` on the shell side,
// before `gate_command` runs.
fn resolve_argv(h: &Harness, gate: &str, casedir: &str) -> Option<Vec<String>> {
    // spec: gate-sdk/SPEC.md §run-gate-tests — the binary is exported into the resolving shell,
    // which is the shell form's process-wide export.
    let script = r#"export GATE_SDK_NATIVE_BIN="$1"; source "$2/lib/gate.sh"; shift 2; cd "$1" || exit 2; shift; gate_command "$@""#;
    let mut argv: Vec<&str> = vec!["-c", script, "bash", &h.bin, &h.sdk, casedir, gate];
    argv.extend(h.gate_dirs.iter().map(String::as_str));
    // spec: gate-sdk/SPEC.md §run-gate-tests — a refusal is a status, not a parse: `gate_command`
    // names the refusal on stderr, which is inherited, and its status becomes the caller's
    // `HARNESS:` line.
    let done = proc::run_streamed("bash", &argv, b"", proc::Stderr::Inherit).ok()?;
    if done.code() != 0 {
        return None;
    }
    Some(
        String::from_utf8_lossy(done.stdout())
            .lines()
            .map(String::from)
            .collect(),
    )
}

// spec: gate-sdk/SPEC.md §run-gate-tests — `env` is a PATH lookup with no directory to resolve, so
// guarding argv[0] would reject every bridged member; the guard takes the first element that is
// neither `env` nor a `NAME=VALUE` assignment.
pub fn dispatch_index(argv: &[String]) -> usize {
    if argv.first().map(String::as_str) != Some("env") {
        return 0;
    }
    let mut x = 1;
    while x < argv.len() && assignment(&argv[x]) {
        x += 1;
    }
    x
}

fn assignment(el: &str) -> bool {
    let mut cs = el.chars();
    match cs.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    el.contains('=')
}

// spec: gate-sdk/SPEC.md §run-gate-tests — `grep -qF` over the combined output: a pin has no
// newline in it, so line-containment is the whole of the match.
pub fn contains_line(out: &str, want: &str) -> bool {
    out.lines().any(|l| l.contains(want))
}

// spec: gate-sdk/SPEC.md §Output contract — the canonical clean line, `^[A-Z][A-Z0-9-]*: clean
// \(.*\)$`: the name admits no space or colon, so the first separator is the only one.
pub fn clean_line(l: &str) -> bool {
    let Some((name, rest)) = l.split_once(": clean (") else {
        return false;
    };
    if !rest.ends_with(')') {
        return false;
    }
    let mut cs = name.chars();
    match cs.next() {
        Some(c) if c.is_ascii_uppercase() => {}
        _ => return false,
    }
    cs.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-')
}

// spec: gate-sdk/SPEC.md §Output contract — the remedy line, `(^|[[:space:]])help:`: a `help:` that
// opens a word, so a `--help:` spelling inside a word does not satisfy it.
pub fn help_line(l: &str) -> bool {
    let mut from = 0;
    while let Some(p) = l[from..].find("help:") {
        let at = from + p;
        if at == 0 || l[..at].ends_with(char::is_whitespace) {
            return true;
        }
        from = at + 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §run-gate-tests — the executable guard's target across the two argv
    // shapes `gate_command` emits: a one-element shell path, and a bridged `env` prefix whose
    // assignments the guard must walk past.
    #[test]
    fn the_dispatch_element_is_found_past_any_env_prefix() {
        let shell = vec!["checks/check-foo.sh".to_string()];
        assert_eq!(dispatch_index(&shell), 0);
        let bridged: Vec<String> = ["env", "GATE_SDK_KNOB_A=1", "GATE_SDK_KNOB_B=x\ty", "bin", "foo"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(dispatch_index(&bridged), 3);
        let bare_env: Vec<String> = vec!["env".to_string()];
        assert_eq!(
            dispatch_index(&bare_env),
            1,
            "an argv that is nothing but `env` has no dispatch element, and the guard must report \
             that rather than index into it"
        );
    }

    // spec: gate-sdk/SPEC.md §Output contract — the clean line's grammar, which a fixture pair
    // cannot pin: the runner asserts it about *other* members and owes no pair of its own.
    #[test]
    fn the_clean_line_grammar_admits_only_the_canonical_shape() {
        assert!(clean_line("CHECK-FOO: clean (3 files)"));
        assert!(clean_line("A: clean ()"));
        assert!(!clean_line("check-foo: clean (3 files)"), "the name is upper-case");
        assert!(!clean_line("CHECK FOO: clean (3)"), "the name admits no space");
        assert!(!clean_line("CHECK-FOO: clean 3 files"), "the parenthetical is required");
        assert!(!clean_line("CHECK-FOO: clean (3) and more"), "the line ends at the paren");
    }

    // spec: gate-sdk/SPEC.md §Output contract — the remedy line opens a word, so an embedded
    // spelling does not satisfy it and an indented one does.
    #[test]
    fn the_remedy_line_must_open_a_word() {
        assert!(help_line("help: do the thing"));
        assert!(help_line("  help: do the thing"));
        assert!(!help_line("--help: do the thing"));
        assert!(!help_line("nohelp: do the thing"));
    }

    // spec: gate-sdk/SPEC.md §run-gate-tests — the expect conjunction's per-line containment, and
    // the blank line that asserts nothing.
    #[test]
    fn an_expect_pin_matches_within_one_line() {
        let out = "alpha fired\nbeta fired";
        assert!(contains_line(out, "alpha fired"));
        assert!(contains_line(out, "eta fir"));
        assert!(
            !contains_line(out, "fired\nbeta"),
            "a pin spanning two lines would make grep -F read the file as alternatives"
        );
    }
}
