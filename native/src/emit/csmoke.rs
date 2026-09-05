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
