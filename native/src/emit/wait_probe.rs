// spec: delegation-kit/SPEC.md §bin/wait-probe — the wait-primitive instrument, an `Arm::Run`
// rather than an `Arm::Emit` member because `report` returns 1 on an empty evidence file and the
// emitting variant collapses every outcome to {0, 2}
// spec: gate-sdk/SPEC.md §The non-gate arm — the member whose subject, not whose implementation,
// fixes its spawned-program set: the wait body, the local arming and the producer stay shell
use crate::proc;
use crate::walk;
use std::io::Write;

// spec: delegation-kit/SPEC.md §bin/wait-probe — the two knobs the shell form read inline; the
// instrument mints none of its own, so scratch and the evidence file both resolve through gate-sdk
pub const KNOBS: &[&str] = &["GATE_SDK_TMP_DIR", "GATE_SDK_WORKFLOW_DIR"];

// spec: delegation-kit/SPEC.md §bin/wait-probe — one wait body for every form, so no form's result
// rests on a differently-shaped trial: the loop, its beat and its four exit traps are one constant
// the arm execs into, never a compiled re-expression of the form under measurement
const WAIT_BODY: &str = r#"
set -uo pipefail
t0="$1"; ppid_rec="$2"; hb="$3"; marker="$4"; pred="$5"; st="$6"; key="$7"; form="$8"
beat() {
    alive=0
    kill -0 "$ppid_rec" 2>/dev/null && alive=1
    now="$(date +%s%3N)"
    case "$now" in *[!0-9]*) now="$(date +%s)000" ;; esac
    printf '%s %s\n' "$(( now - t0 ))" "$alive" > "$hb"
}
: > "$hb"
rm -f "$st"
trap 'printf "%s\n" "$?" > "$st"' EXIT
trap 'exit 143' TERM
trap 'exit 130' INT
trap 'exit 129' HUP
if [ "$pred" = liveness ]; then
    until kill -0 "$ppid_rec" 2>/dev/null; do
        beat
        sleep 1
    done
else
    until [ -f "$marker" ]; do
        beat
        sleep 1
    done
fi
beat
printf 'wait-probe: condition true for %s (%s/%s)\n' "$key" "$form" "$pred"
"#;

// spec: delegation-kit/SPEC.md §bin/wait-probe — the producer stays a backgrounded *shell* child so
// it is reaped when it exits; a compiled producer whose parent outlives it leaves a zombie, and
// `kill -0` on a zombie succeeds, which would invert `producer_alive_at_exit` on every trial
const PRODUCE_SH: &str = r#"nohup bash -c 'sleep "$(( $2 / 1000 ))"; date +%s%3N > "$1"' _ "$1" "$2" >/dev/null 2>&1 &
printf 'pid=%s\n' "$!"
"#;

// spec: delegation-kit/SPEC.md §bin/wait-probe — `arm-local` arms the waiter as a detached shell
// child, which is the harness-uninvolved control's whole definition; a crate-armed child would make
// the form's name false on the axis the instrument measures
const ARM_LOCAL_SH: &str = r#"nohup "$1" --wait-probe waiter "$2" local "$3" > "$4" 2>&1 &
printf 'pid=%s\n' "$!"
"#;

// spec: delegation-kit/SPEC.md §bin/wait-probe — the sweep's durations are the instrument's
// calibration rather than configuration: a consumer varying them varies the experiment
const SWEEP_MS: &[u64] = &[10_000, 100_000, 200_000];

const ROSTER: &str = "usage: run-gates.sh --wait-probe <subcommand> [args]

  produce <key> <duration_ms>   stand a producer up: sleep <duration_ms>, then write the marker.
                                Records its pid at launch in <scratch>/<key>.run.
  waiter <key> <form> [pred]    the wait body itself: until <marker>; do sleep 1; done.
                                Identical across every form; whoever arms it names the form.
  arm-local <key> [pred]        arm `waiter` as a detached shell child (the harness-uninvolved control).
  record <key> <form> <ms>      append one trial line for an armed-and-finished waiter.
  report                        classify the recorded trials and print the verdict.
  sweep                         the self-contained local run: produce + arm-local + record over the
                                declared duration sweep. This is the reproducer a second machine runs.

Scratch resolves through GATE_SDK_TMP_DIR; the evidence file through GATE_SDK_WORKFLOW_DIR.";

// spec: delegation-kit/SPEC.md §bin/wait-probe — the three-state exit contract, and the misuse arm
// is the only one this wrapper decides: 0 and 1 both arrive from the subcommand, so no `Err` path
// may collapse the honest empty reading into the typo code
pub fn run(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("checkwright-gates: --wait-probe: {}", e);
            eprintln!("{}", ROSTER);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the shape half crosses the port: an argument
// beginning with `-` that names no subcommand is a refusal with the roster on stderr, `--` ends
// option processing, and the `-h`/`--help` arm retires to the front-end
fn dispatch(args: &[String]) -> Result<i32, String> {
    let mut rest: Vec<&str> = Vec::new();
    let mut escaped = false;
    for a in args {
        if !escaped && a == "--" {
            escaped = true;
            continue;
        }
        rest.push(a.as_str());
    }
    let Some(sub) = rest.first().copied() else {
        return Err("needs a <subcommand>".to_string());
    };
    let operands = &rest[1..];
    if !escaped && sub.starts_with('-') {
        return Err(format!("unrecognized option: {}", sub));
    }
    match (sub, operands.len()) {
        ("produce", 2) => cmd_produce(operands[0], operands[1]),
        ("waiter", 2) => cmd_waiter(operands[0], operands[1], "marker"),
        ("waiter", 3) => cmd_waiter(operands[0], operands[1], operands[2]),
        ("arm-local", 1) => cmd_arm_local(operands[0], "marker"),
        ("arm-local", 2) => cmd_arm_local(operands[0], operands[1]),
        ("record", 3) => cmd_record(operands[0], operands[1], operands[2]),
        ("report", 0) => cmd_report(),
        ("sweep", 0) => cmd_sweep(),
        ("produce" | "waiter" | "arm-local" | "record" | "report" | "sweep", n) => Err(format!(
            "{} does not take {} argument(s)",
            sub, n
        )),
        _ => Err(format!("no such subcommand: {}", sub)),
    }
}

struct Paths {
    scratch: String,
    work: String,
    evidence: String,
}

fn paths() -> Result<Paths, String> {
    let scratch = walk::knob_scalar("GATE_SDK_TMP_DIR")?;
    let workflow = walk::knob_scalar("GATE_SDK_WORKFLOW_DIR")?;
    Ok(Paths {
        work: format!("{}/wait-probe", scratch),
        evidence: format!("{}/wait-primitive-evidence.txt", workflow),
        scratch,
    })
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — the `t0` stamp is the instrument's own bookkeeping
// and crosses the seam; the *marker* stays `date +%s%3N` inside the producer's shell, both being
// epoch milliseconds
fn now_ms() -> Result<u128, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .map_err(|e| format!("cannot read the clock: {}", e))
}

fn write_file(path: &str, body: &str) -> Result<(), String> {
    std::fs::write(path, body).map_err(|e| format!("cannot write {}: {}", path, e))
}

fn read_trimmed(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map(|s| s.trim_end_matches(['\n', '\r']).to_string())
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

fn non_empty(path: &str) -> bool {
    std::fs::metadata(path).map(|m| m.len() > 0).unwrap_or(false)
}

fn mkdir_p(path: &str) -> Result<(), String> {
    std::fs::create_dir_all(path).map_err(|e| format!("cannot create {}: {}", path, e))
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — the producer's own pid is recorded at launch and
// never logged in a trial line: the `<key>.run` record holds it for the lifetime the wait needs, and
// `check-producer-liveness` and guard rule 14 read the probe's own producers like any other
fn cmd_produce(key: &str, ms: &str) -> Result<i32, String> {
    let p = paths()?;
    mkdir_p(&p.work)?;
    let marker = format!("{}/{}.marker", p.work, key);
    let t0_path = format!("{}/{}.t0", p.work, key);
    let run_path = format!("{}/{}.run", p.scratch, key);
    for stale in [&marker, &t0_path, &run_path] {
        std::fs::remove_file(stale).ok();
    }
    write_file(&t0_path, &format!("{}\n", now_ms()?))?;
    let pid = spawned_pid(PRODUCE_SH, &["_", &marker, ms])?;
    write_file(&run_path, &format!("pid={} run={}\n", pid, key))?;
    println!(
        "wait-probe: producer {} pid={} duration_ms={} marker={}",
        key, pid, ms, marker
    );
    Ok(0)
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — the launched pid is read back out of the shell that
// backgrounded it, so the record names the process the harness would signal
fn spawned_pid(script: &str, argv: &[&str]) -> Result<u32, String> {
    let mut call: Vec<&str> = vec!["-c", script];
    call.extend_from_slice(argv);
    let done = proc::run("bash", &call)?;
    let out = done
        .stdout()
        .map(|b| String::from_utf8_lossy(b).to_string())
        .unwrap_or_default();
    out.lines()
        .find_map(|l| l.strip_prefix("pid="))
        .and_then(|n| n.trim().parse::<u32>().ok())
        .ok_or_else(|| format!("the arming shell reported no pid: {}", out.trim()))
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — this arm `exec`s into the wait body rather than
// spawning and waiting on it, so the process depth between the harness and the loop stays one
fn cmd_waiter(key: &str, form: &str, pred: &str) -> Result<i32, String> {
    let p = paths()?;
    let marker = format!("{}/{}.marker", p.work, key);
    let hb = format!("{}/{}.{}.hb", p.work, key, form);
    let st = format!("{}/{}.{}.st", p.work, key, form);
    let t0 = read_trimmed(&format!("{}/{}.t0", p.work, key))?;
    let run = read_trimmed(&format!("{}/{}.run", p.scratch, key))?;
    let ppid = recorded_pid(&run)
        .ok_or_else(|| format!("no pid= field in the launch record for {}", key))?;
    write_file(&format!("{}/{}.{}.pred", p.work, key, form), &format!("{}\n", pred))?;
    exec_wait_body(&[
        "_", &t0, &ppid, &hb, &marker, pred, &st, key, form,
    ])
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — the launch record's two-field line is the format
// `check-producer-liveness` and guard rule 14 read, so the arm parses and writes exactly it
fn recorded_pid(record: &str) -> Option<String> {
    record.split_whitespace().find_map(|f| {
        let n = f.strip_prefix("pid=")?;
        (!n.is_empty() && n.bytes().all(|b| b.is_ascii_digit())).then(|| n.to_string())
    })
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — the wait body is a POSIX shell loop and this arm
// replaces its own process image with it; a host with no `exec` has no such subject to measure, so
// the arm refuses rather than silently measuring a spawned-and-waited configuration
#[cfg(unix)]
fn exec_wait_body(argv: &[&str]) -> Result<i32, String> {
    use std::os::unix::process::CommandExt;
    let mut cmd = std::process::Command::new("bash");
    cmd.arg("-c").arg(WAIT_BODY).args(argv);
    Err(format!("cannot exec the wait body: {}", cmd.exec()))
}

#[cfg(not(unix))]
fn exec_wait_body(_argv: &[&str]) -> Result<i32, String> {
    Err("the wait body is a POSIX shell loop and this host cannot exec into one".to_string())
}

fn cmd_arm_local(key: &str, pred: &str) -> Result<i32, String> {
    let p = paths()?;
    let out = format!("{}/{}.local.out", p.work, key);
    let run_path = format!("{}/{}-local.run", p.scratch, key);
    let me = std::env::current_exe()
        .map_err(|e| format!("cannot resolve this binary's own path: {}", e))?
        .display()
        .to_string();
    let pid = spawned_pid(ARM_LOCAL_SH, &["_", &me, key, pred, &out])?;
    write_file(&run_path, &format!("pid={} run={}-local\n", pid, key))?;
    println!("wait-probe: local waiter for {} pid={}", key, pid);
    Ok(0)
}

fn cmd_record(key: &str, form: &str, ms: &str) -> Result<i32, String> {
    let p = paths()?;
    let marker = format!("{}/{}.marker", p.work, key);
    let hb = format!("{}/{}.{}.hb", p.work, key, form);
    let st = format!("{}/{}.{}.st", p.work, key, form);
    let pred_path = format!("{}/{}.{}.pred", p.work, key, form);

    let mut pred = "marker".to_string();
    if non_empty(&pred_path) {
        pred = read_trimmed(&pred_path)?;
    }
    let t0: i128 = read_trimmed(&format!("{}/{}.t0", p.work, key))?
        .parse()
        .map_err(|_| "the t0 stamp is not a number".to_string())?;

    let mut marker_at = "-".to_string();
    if std::path::Path::new(&marker).exists() {
        let at: i128 = read_trimmed(&marker)?
            .parse()
            .map_err(|_| "the marker is not a number".to_string())?;
        marker_at = (at - t0).to_string();
    }
    let mut waiter_at = "-".to_string();
    let mut alive = "-".to_string();
    if non_empty(&hb) {
        let beat = read_trimmed(&hb)?;
        let mut fields = beat.split(' ');
        waiter_at = fields.next().unwrap_or("-").to_string();
        alive = fields.next().unwrap_or("-").to_string();
    }
    let mut waiter_exit = "killed".to_string();
    if non_empty(&st) {
        waiter_exit = read_trimmed(&st)?;
    }
    let class = classify(&marker_at, &waiter_at, &waiter_exit, &alive);
    let line = format!(
        "form={} predicate={} producer_ms={} waiter_exit={} marker_at_ms={} waiter_at_ms={} producer_alive_at_exit={} class={}\n",
        form, pred, ms, waiter_exit, marker_at, waiter_at, alive, class
    );
    if let Some(dir) = std::path::Path::new(&p.evidence).parent() {
        mkdir_p(&dir.display().to_string())?;
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&p.evidence)
        .map_err(|e| format!("cannot open the evidence file {}: {}", p.evidence, e))?;
    f.write_all(line.as_bytes())
        .map_err(|e| format!("cannot append to {}: {}", p.evidence, e))?;
    print!("{}", line);
    Ok(0)
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — the closed cause list, 'unexplained' included, and
// (ii) is cross-trial so it is named by `report` and never by a line
fn classify(marker_at: &str, waiter_at: &str, waiter_exit: &str, alive: &str) -> &'static str {
    let ordered = match (marker_at.parse::<i128>(), waiter_at.parse::<i128>()) {
        (Ok(m), Ok(w)) => w >= m,
        _ => false,
    };
    if ordered {
        return "ok";
    }
    if waiter_exit == "0" && marker_at == "-" {
        return "predicate";
    }
    if alive == "1" && waiter_exit != "0" {
        return "reaped";
    }
    "unexplained"
}

#[derive(Default)]
struct Cell {
    n: u64,
    ok: u64,
    bad: u64,
    okmax: u64,
    badmin: u64,
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — cause (ii)'s tell is a threshold across the sweep,
// so the report is the only reader that can see it, and the ceiling line is the one verdict no
// trial line can carry
fn cmd_report() -> Result<i32, String> {
    let p = paths()?;
    if !non_empty(&p.evidence) {
        eprintln!("wait-probe: no trials recorded in {}", p.evidence);
        return Ok(1);
    }
    let body = std::fs::read_to_string(&p.evidence)
        .map_err(|e| format!("cannot read the evidence file {}: {}", p.evidence, e))?;
    println!("=== trials ===");
    print!("{}", body);
    println!("\n=== verdict ===");
    print!("{}", verdict(&body));
    Ok(0)
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — the verdict block: per-cell tallies, the ceiling
// tell, and the predicate-shaped conclusion. Cells are emitted in sorted order, awk's own
// association-array order having been unspecified and so never a contract to preserve.
fn verdict(body: &str) -> String {
    let mut cells: Vec<(String, Cell)> = Vec::new();
    let mut bad_preds: Vec<String> = Vec::new();
    let mut bad_forms: Vec<String> = Vec::new();
    let mut ok_forms: Vec<String> = Vec::new();

    for line in body.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut form = String::new();
        let mut pred = String::new();
        let mut ms: u64 = 0;
        let mut class = String::new();
        for field in line.split(' ') {
            let Some((k, v)) = field.split_once('=') else {
                continue;
            };
            match k {
                "form" => form = v.to_string(),
                "predicate" => pred = v.to_string(),
                "producer_ms" => ms = v.parse().unwrap_or(0),
                "class" => class = v.to_string(),
                _ => {}
            }
        }
        let name = format!("{}/{}", form, pred);
        let idx = match cells.iter().position(|(c, _)| *c == name) {
            Some(i) => i,
            None => {
                cells.push((name, Cell::default()));
                cells.len() - 1
            }
        };
        let cell = &mut cells[idx].1;
        cell.n += 1;
        if class == "ok" {
            cell.ok += 1;
            if ms > cell.okmax {
                cell.okmax = ms;
            }
            push_once(&mut ok_forms, &form);
        } else {
            cell.bad += 1;
            if cell.badmin == 0 || ms < cell.badmin {
                cell.badmin = ms;
            }
            push_once(&mut bad_forms, &form);
            push_once(&mut bad_preds, &pred);
        }
    }
    cells.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = String::new();
    for (name, c) in &cells {
        out.push_str(&format!(
            "{:<18} {} trial(s), {} ok, {} early; longest clean wait={}ms\n",
            format!("{}:", name),
            c.n,
            c.ok,
            c.bad,
            c.okmax
        ));
    }
    for (name, c) in &cells {
        if c.bad > 0 && c.ok > 0 && c.badmin > c.okmax {
            out.push_str(&format!(
                "  ceiling tell HOLDS for {}: every early exit outlasts every clean wait (threshold between {}ms and {}ms) -> cause (ii)\n",
                name, c.okmax, c.badmin
            ));
        }
    }
    let np = bad_preds.len();
    let nff = bad_forms.len();
    let nof = ok_forms.len();
    if np == 1 && nff > 1 {
        out.push_str(&format!(
            "\nEvery early exit carries predicate={}, across {} of the {} forms measured, and no form is early on any other predicate.\nThe early exits are predicate-shaped, not form-shaped: the wait form is exonerated and the condition is the cause (iii).\n",
            bad_preds[0], nff, nof
        ));
    } else if np == 0 {
        out.push_str("\nNo early exit recorded.\n");
    } else {
        out.push_str(&format!(
            "\nEarly exits span {} predicate(s) and {} form(s) -- read the class column; no single-cause reading is licensed.\n",
            np, nff
        ));
    }
    out
}

fn push_once(seen: &mut Vec<String>, v: &str) {
    if !seen.iter().any(|s| s == v) {
        seen.push(v.to_string());
    }
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — the sweep waits on the *recorded* pid's liveness
// with a `while` polarity, never a `until`: an `until` here would reproduce inside the instrument
// the very defect the instrument's own trials found
fn cmd_sweep() -> Result<i32, String> {
    let p = paths()?;
    mkdir_p(&p.work)?;
    for ms in SWEEP_MS {
        let key = format!("sweep{}", ms);
        cmd_produce(&key, &ms.to_string())?;
        cmd_arm_local(&key, "marker")?;
    }
    for ms in SWEEP_MS {
        let key = format!("sweep{}", ms);
        let waiter_run = format!("{}/{}-local.run", p.scratch, key);
        if let Some(pid) = recorded_pid(&read_trimmed(&waiter_run)?) {
            while pid_alive(&pid) {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        cmd_record(&key, "local", &ms.to_string())?;
        std::fs::remove_file(format!("{}/{}.run", p.scratch, key)).ok();
        std::fs::remove_file(&waiter_run).ok();
    }
    cmd_report()
}

// spec: delegation-kit/SPEC.md §bin/wait-probe — liveness is `kill -0` on the recorded pid and never
// a process-table pattern, which is the protocol's own rule
fn pid_alive(pid: &str) -> bool {
    proc::run("bash", &["-c", "kill -0 \"$1\" 2>/dev/null", "_", pid])
        .map(|d| d.code() == Some(0))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: delegation-kit/SPEC.md §bin/wait-probe — the argv-shape refusal crosses the port: a
    // missing subcommand, an unknown one and a wrong arity are all exit 2 with the roster
    #[test]
    fn the_argv_shape_refusals_are_exit_two() {
        assert_eq!(run(&[]), 2);
        assert_eq!(run(&["--help".to_string()]), 2);
        assert_eq!(run(&["nope".to_string()]), 2);
        assert_eq!(run(&["report".to_string(), "extra".to_string()]), 2);
        assert_eq!(run(&["produce".to_string()]), 2);
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — `--` ends option processing, so a subcommand
    // spelled with a leading dash is still unreachable as an option but reachable as an operand
    #[test]
    fn the_double_dash_escape_ends_option_processing() {
        let args = vec!["--".to_string(), "-weird".to_string()];
        assert!(dispatch(&args).is_err(), "no such subcommand, not an option refusal");
    }

    // spec: delegation-kit/SPEC.md §bin/wait-probe — the closed cause list, each arm reached on the
    // tells a single trial can see
    #[test]
    fn the_cause_list_is_closed_and_each_arm_is_reachable() {
        assert_eq!(classify("500", "600", "0", "0"), "ok");
        assert_eq!(classify("500", "500", "0", "0"), "ok");
        assert_eq!(classify("-", "400", "0", "1"), "predicate");
        assert_eq!(classify("500", "400", "143", "1"), "reaped");
        assert_eq!(classify("500", "400", "0", "0"), "unexplained");
    }

    // spec: delegation-kit/SPEC.md §bin/wait-probe — the launch record's `pid=` field is read out of
    // the two-field line and nothing else in it is a pid
    #[test]
    fn the_launch_record_yields_its_pid_and_nothing_else() {
        assert_eq!(recorded_pid("pid=4242 run=sweep10000"), Some("4242".to_string()));
        assert_eq!(recorded_pid("run=sweep10000"), None);
        assert_eq!(recorded_pid("pid=x run=k"), None);
    }

    // spec: delegation-kit/SPEC.md §bin/wait-probe — the ceiling tell is the one verdict no trial
    // line can carry, so it is the arm's own assertion rather than a re-read of `class`
    #[test]
    fn the_ceiling_tell_fires_on_a_threshold_across_the_sweep() {
        let body = "\
form=local predicate=marker producer_ms=10000 waiter_exit=0 marker_at_ms=10 waiter_at_ms=11 producer_alive_at_exit=0 class=ok
form=local predicate=marker producer_ms=100000 waiter_exit=143 marker_at_ms=- waiter_at_ms=9 producer_alive_at_exit=1 class=reaped
form=local predicate=marker producer_ms=200000 waiter_exit=143 marker_at_ms=- waiter_at_ms=9 producer_alive_at_exit=1 class=reaped
";
        let got = verdict(body);
        assert!(
            got.contains("ceiling tell HOLDS for local/marker"),
            "the threshold between 10000ms and 100000ms must be named: {}",
            got
        );
        assert!(got.contains("threshold between 10000ms and 100000ms"), "{}", got);
        assert!(got.starts_with("local/marker:      3 trial(s), 1 ok, 2 early"), "{}", got);
    }

    // spec: delegation-kit/SPEC.md §bin/wait-probe — the predicate-shaped conclusion, which needs
    // one bad predicate across more than one form
    #[test]
    fn the_predicate_shaped_conclusion_needs_one_predicate_and_two_forms() {
        let body = "\
form=local predicate=marker producer_ms=10000 waiter_exit=0 marker_at_ms=10 waiter_at_ms=11 producer_alive_at_exit=0 class=ok
form=local predicate=liveness producer_ms=10000 waiter_exit=0 marker_at_ms=- waiter_at_ms=1 producer_alive_at_exit=1 class=predicate
form=harness predicate=liveness producer_ms=10000 waiter_exit=0 marker_at_ms=- waiter_at_ms=1 producer_alive_at_exit=1 class=predicate
";
        let got = verdict(body);
        assert!(got.contains("Every early exit carries predicate=liveness"), "{}", got);
        assert!(got.contains("across 2 of the 1 forms measured"), "{}", got);
    }

    // spec: delegation-kit/SPEC.md §bin/wait-probe — a clean sweep names no cause
    #[test]
    fn a_clean_run_records_no_early_exit() {
        let body = "form=local predicate=marker producer_ms=10000 waiter_exit=0 marker_at_ms=10 waiter_at_ms=11 producer_alive_at_exit=0 class=ok\n";
        assert!(verdict(body).ends_with("\nNo early exit recorded.\n"));
    }

    // spec: delegation-kit/SPEC.md §bin/wait-probe — the wait body is one constant every form arms,
    // and its polarity is the property this tool's own trials corrected
    #[test]
    fn the_one_wait_body_keeps_the_measured_polarity() {
        assert!(WAIT_BODY.contains("until [ -f \"$marker\" ]"));
        assert!(WAIT_BODY.contains("until kill -0 \"$ppid_rec\""));
        assert_eq!(WAIT_BODY.matches("sleep 1").count(), 2);
        for sig in ["TERM", "INT", "HUP", "EXIT"] {
            assert!(WAIT_BODY.contains(&format!("' {}", sig)), "no {} trap", sig);
        }
    }

    // spec: delegation-kit/SPEC.md §bin/wait-probe — the producer and the local waiter are both
    // *shell* children: the first so it is reaped, the second so the form's name stays true
    #[test]
    fn the_producer_and_the_local_waiter_stay_shell_children() {
        assert!(PRODUCE_SH.starts_with("nohup bash -c"));
        assert!(PRODUCE_SH.contains("&\nprintf 'pid=%s\\n' \"$!\""));
        assert!(ARM_LOCAL_SH.starts_with("nohup \"$1\" --wait-probe waiter"));
        assert!(ARM_LOCAL_SH.contains("&\nprintf 'pid=%s\\n' \"$!\""));
    }
}
