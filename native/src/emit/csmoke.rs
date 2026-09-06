// spec: gate-sdk/SPEC.md §Consumer smoke — the spawn-side seam onto `lib/consumer-smoke.sh`: its
// helpers are called in the library that owns them, so both arms that take that road share one
// transport and one prologue rather than a copy each.
use crate::proc::{self, Stderr};

// spec: gate-sdk/SPEC.md §Consumer smoke — the script prologue every spawn-side caller opens with;
// `$1` is the gate-sdk root, which is why a caller's own positionals start at `$2`.
pub const SOURCE: &str = r#"source "$1/lib/gate.sh"; source "$1/lib/consumer-smoke.sh";"#;

// spec: gate-sdk/SPEC.md §Consumer smoke — `bash -c <script> bash <args…>`: the `$0` word is
// supplied here so a caller writes only the arguments its own script reads.
pub fn spawn(script: &str, args: &[&str], stderr: Stderr) -> Result<proc::Streamed, String> {
    let mut argv: Vec<&str> = vec!["-c", script, "bash"];
    argv.extend_from_slice(args);
    proc::run_streamed("bash", &argv, b"", stderr)
}

// spec: gate-sdk/SPEC.md §Consumer smoke — `csmoke_place_binary` reads its caller's `SCRATCH`, so
// the shared seam runs in the input direction: the arm supplies the variable the helper's own
// contract names and the library is untouched.
// spec: gate-sdk/SPEC.md §Consumer smoke — it lives beside the prologue rather than in either arm,
// having gained its second caller; `Ok` is the helper's status, so each arm renders its own verdict.
pub fn place_binary(
    sdk: &str,
    consumer: &str,
    host: &str,
    roots: &[String],
) -> Result<i32, String> {
    let script = format!(
        "{} SCRATCH=\"$2\"; host=\"$3\"; shift 3; csmoke_place_binary \"$host\" \"$@\" 1>&2",
        SOURCE
    );
    let refs: Vec<&str> = vec![sdk, consumer, host]
        .into_iter()
        .chain(roots.iter().map(String::as_str))
        .collect();
    Ok(spawn(&script, &refs, Stderr::Inherit)?.code())
}
