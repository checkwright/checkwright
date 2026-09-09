// spec: context-kit/SPEC.md §Testing — the expected-output runner for the advisory index-first
// tools and the always-loaded meter, bridged as an `Arm::Run` member: the three-valued exit is the
// contract and `Arm::Emit` collapses it.
// spec: context-kit/SPEC.md §Testing — the checks reach their arms through the `--emit` front-end
// rather than through the binary, which is the property the port must not lose.
use crate::proc;
use crate::walk;
use std::path::{Path, PathBuf};

// spec: context-kit/SPEC.md §Testing — one declared knob, the transported kit roots; a compiled
// member has no `BASH_SOURCE` anchor. No `CONTEXT_KIT_` knob is declared, that section's rule for
// the sibling smoke.
pub const KNOBS: &[&str] = &["GATE_KIT_ROOTS_HERE"];

const NAME: &str = "run-index-tests";
const VERDICT: &str = "INDEX-TESTS";

// spec: context-kit/SPEC.md §Testing — the shell file's `usage:` line, moved to the arm's own
// refusal text: the deleted script's header is no longer a place a reader can find it.
const USAGE: &str =
    "usage: --run-index-tests [--update]   (bare: diff vs golden; --update: rewrite goldens)";

// spec: context-kit/SPEC.md §Testing — the scratch lifecycle is the arm's own control flow rather
// than a trap, and one root removed on every exit path is what repairs the shell form's leak
// window rather than reproducing it.
struct Scratch {
    root: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

pub fn run(args: &[String]) -> i32 {
    let mut update = false;
    for a in args {
        if a == "--update" {
            update = true;
        } else {
            eprintln!("{}: unknown option: {}; {}", NAME, a, USAGE);
            return 2;
        }
    }
    match execute(update) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

fn execute(update: bool) -> Result<i32, String> {
    let kit = kit_root("context-kit")?;
    let sdk = kit_root("gate-sdk")?;
    let host = parent_of(&sdk)?;
    let front_end = format!("{}/bin/run-gates.sh", sdk);
    let corpus = format!("{}/index-tests/corpus", kit);
    let expected = format!("{}/index-tests/expected", kit);
    std::fs::create_dir_all(&expected)
        .map_err(|e| format!("cannot create {}: {}", expected, e))?;

    let scratch = make_scratch()?;
    let shadow_cfg = write_shadow(&scratch)?;
    let cfg = write_meter_config(&scratch, &corpus)?;

    let mut r = Runner {
        update,
        expected,
        front_end,
        host: PathBuf::from(&host),
        pass: 0,
        fail: 0,
        harness: 0,
    };

    // spec: context-kit/SPEC.md §Testing — the golden-held checks in the shell driver's order.
    let sample_md = format!("{}/sample.md", corpus);
    let sample_rs = format!("{}/sample.rs", corpus);
    let sample_ts = format!("{}/sample.ts", corpus);
    r.check("md-index", "md-index.txt", &["--emit", "md-index", &sample_md], &[])?;
    r.check(
        "md-section",
        "md-section.txt",
        &["--emit", "md-section", &sample_md, "Code First"],
        &[],
    )?;
    r.check("pub-index", "pub-index.txt", &["--emit", "pub-index", &sample_rs], &[])?;
    r.check(
        "pub-index-ts",
        "pub-index-ts.txt",
        &["--emit", "pub-index", &sample_ts],
        &[],
    )?;
    // spec: context-kit/SPEC.md §Testing — the extractor seam's end-to-end proof: the scratch
    // `rust.sh` reaches disk and `CONTEXT_KIT_CONFIG_FILE` is passed into the *child*, never
    // resolved here.
    r.check(
        "pub-index-shadow",
        "pub-index-shadow.txt",
        &["--emit", "pub-index", &sample_rs],
        &[("CONTEXT_KIT_CONFIG_FILE".to_string(), shadow_cfg.clone())],
    )?;
    r.check(
        "always-loaded",
        "always-loaded.txt",
        &["--emit", "always-loaded"],
        &[("CONTEXT_KIT_CONFIG_FILE".to_string(), cfg.clone())],
    )?;

    if !r.update {
        r.refusal_case(&cfg)?;
    }

    println!();
    if r.update {
        println!("{}: goldens rewritten", VERDICT);
        return Ok(0);
    }
    if r.harness > 0 {
        println!("{}: {} harness error(s)", VERDICT, r.harness);
        return Ok(2);
    }
    if r.fail > 0 {
        println!(
            "{}: {} of {} check(s) failed",
            VERDICT,
            r.fail,
            r.pass + r.fail
        );
        return Ok(1);
    }
    println!(
        "{}: clean ({} checks: the goldens match and the meter refuses an unrecognized mode)",
        VERDICT, r.pass
    );
    Ok(0)
}

struct Runner {
    update: bool,
    expected: String,
    front_end: String,
    host: PathBuf,
    pass: usize,
    fail: usize,
    harness: usize,
}

impl Runner {
    // spec: context-kit/SPEC.md §Testing — a check through the `--emit` front-end, spawned by
    // absolute path with the host checkout as the child's working directory: the front-end refuses
    // outside a git repository, and the shell form met that only by being run from the root.
    fn check(
        &mut self,
        name: &str,
        golden: &str,
        emit_args: &[&str],
        env: &[(String, String)],
    ) -> Result<(), String> {
        let front_end = self.front_end.clone();
        self.spawn_check(name, golden, &front_end, emit_args, env)
    }

    fn spawn_check(
        &mut self,
        name: &str,
        golden: &str,
        script: &str,
        rest: &[&str],
        env: &[(String, String)],
    ) -> Result<(), String> {
        let mut argv: Vec<&str> = vec![script];
        argv.extend_from_slice(rest);
        let done = proc::run_with_env_in("bash", &argv, env, Some(&self.host))?;
        // spec: context-kit/SPEC.md §Testing — the shell form read the tool's own status out of
        // `PIPESTATUS[0]` and called any non-zero a *harness* error rather than a finding; the
        // fail-closed accessor withholds stdout on exactly that status, so the branch is the same.
        let Some(out) = done.stdout() else {
            println!(
                "  HARNESS: {} — tool exited {}",
                name,
                done.reported_code()
            );
            self.harness += 1;
            return Ok(());
        };
        let payload = payload(&String::from_utf8_lossy(out));
        let path = format!("{}/{}", self.expected, golden);
        let shown = format!("index-tests/expected/{}", golden);
        if self.update {
            std::fs::write(&path, payload.as_bytes())
                .map_err(|e| format!("cannot write {}: {}", path, e))?;
            println!("  UPDATED: {} -> {}", name, shown);
            return Ok(());
        }
        if !Path::new(&path).is_file() {
            println!("  HARNESS: {} — no golden at {} (run --update)", name, shown);
            self.harness += 1;
            return Ok(());
        }
        let golden_text = std::fs::read(&path).map_err(|e| format!("cannot read {}: {}", path, e))?;
        if golden_text == payload.as_bytes() {
            self.pass += 1;
            return Ok(());
        }
        println!("  FAIL: {} — output differs from {}:", name, shown);
        // spec: context-kit/SPEC.md §Testing — the mismatch report is the runner's own diagnostic
        // and no golden holds it, so it renders through the crate's diff rather than a `diff -u`
        // spawn: that is what keeps this arm's spawned set `bash` alone.
        let held = String::from_utf8_lossy(&golden_text).into_owned();
        let a: Vec<&str> = held.trim_end_matches('\n').split('\n').collect();
        let b: Vec<&str> = payload.trim_end_matches('\n').split('\n').collect();
        for line in crate::diff::normal_diff(&a, &b) {
            println!("    {}", line);
        }
        self.fail += 1;
        Ok(())
    }

    // spec: context-kit/SPEC.md §The always-loaded meter — a mode outside the closed set is
    // refused. No golden holds this: the assertion is the exit status and the *stream* the usage
    // went to.
    fn refusal_case(&mut self, cfg: &str) -> Result<(), String> {
        let env = [("CONTEXT_KIT_CONFIG_FILE".to_string(), cfg.to_string())];
        let done = proc::run_with_env_in(
            "bash",
            &[&self.front_end, "--emit", "always-loaded", "--growht"],
            &env,
            Some(&self.host),
        )?;
        // spec: gate-sdk/SPEC.md §Fail-closed contract — the failed child's whole account of
        // itself, which is what makes one spawn answer what the shell form needed two for: the
        // report renders an empty stdout as `stdout: <empty>`, so `usage:` and `--growht` appearing
        // anywhere in it can only have arrived on stderr once that clause holds.
        let report = done.failure_report().unwrap_or_default();
        let rc = done.reported_code();
        let empty_stdout = report.contains("stdout: <empty>");
        if rc == 2 && empty_stdout && report.contains("usage:") && report.contains("--growht") {
            self.pass += 1;
            return Ok(());
        }
        println!(
            "  FAIL: always-loaded-refusal — expected exit 2 with the usage block on stderr and \
             nothing on stdout; got {}",
            report_or_clean(&report, rc)
        );
        self.fail += 1;
        Ok(())
    }
}

fn report_or_clean(report: &str, rc: i32) -> String {
    if report.is_empty() {
        format!("exit {} and no refusal at all", rc)
    } else {
        report.to_string()
    }
}

// spec: context-kit/SPEC.md §Testing — the golden's byte shape, the shell form's `$( … )` capture
// followed by one `printf '%s\n'`.
fn payload(raw: &str) -> String {
    format!("{}\n", norm(raw).trim_end_matches('\n'))
}

// spec: context-kit/SPEC.md §Testing — `norm()`'s path rewrite, `sed 's#^[^ ]*/corpus/#corpus/#'`.
// Losing it makes every golden hold an absolute path and the suite passes only on the machine that
// last ran `--update`.
fn norm(text: &str) -> String {
    text.lines().map(norm_line).collect::<Vec<String>>().join("\n")
}

// spec: context-kit/SPEC.md §Testing — the regex is anchored and `[^ ]*` is greedy, so the rewrite
// takes the *last* `/corpus/` inside the line's leading run of non-space characters and nothing on
// a line whose first field carries none.
fn norm_line(line: &str) -> String {
    const NEEDLE: &str = "/corpus/";
    let head = &line[..line.find(' ').unwrap_or(line.len())];
    match head.rfind(NEEDLE) {
        Some(at) => format!("corpus/{}", &line[at + NEEDLE.len()..]),
        None => line.to_string(),
    }
}

// spec: context-kit/SPEC.md §Testing — the arm reaches both kits through the transported roots
// rather than a path relative to itself.
fn kit_root(name: &str) -> Result<String, String> {
    walk::kit_roots_abs()?
        .into_iter()
        .find(|r| r.rsplit('/').next() == Some(name))
        .ok_or_else(|| {
            format!(
                "GATE_KIT_ROOTS_HERE names no {} root, so neither the fixture corpus nor the \
                 front-end the checks reach their arms through can be found",
                name
            )
        })
}

fn parent_of(root: &str) -> Result<String, String> {
    let trimmed = root.trim_end_matches('/');
    match trimmed.rsplit_once('/') {
        Some((host, _)) if !host.is_empty() => Ok(host.to_string()),
        _ => Err(format!("{} has no host checkout above it", root)),
    }
}

fn make_scratch() -> Result<Scratch, String> {
    let root = std::env::temp_dir().join(format!("checkwright-index-tests.{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("pub-lang"))
        .map_err(|e| format!("cannot create the scratch dir {}: {}", root.display(), e))?;
    Ok(Scratch { root })
}

// spec: context-kit/SPEC.md §Testing — the consumer extractor the shadow case points
// `CONTEXT_KIT_PUB_LANG_DIR` at: its `rust.sh` emits a marker row, so the built-in grammar's
// absence is visible in the golden rather than inferred.
fn write_shadow(scratch: &Scratch) -> Result<String, String> {
    let dir = scratch.root.join("pub-lang");
    write(
        &dir.join("rust.sh"),
        "# shellcheck shell=bash disable=SC2034\n\
         PUB_LANG_GLOBS=(\"*.rs\")\n\
         pub_lang_extract() { printf 'shadow ok 1\\n'; }\n",
    )?;
    let cfg = scratch.root.join("shadow.conf");
    write(
        &cfg,
        &format!(
            "CONTEXT_KIT_PUB_LANG_DIR=\"{}\"\nCONTEXT_KIT_PUB_LANGS=(\"rust\")\n",
            dir.display()
        ),
    )?;
    Ok(cfg.display().to_string())
}

// spec: context-kit/SPEC.md §The always-loaded meter — the meter's three inputs pinned at the
// fixture corpus, so the golden holds a measurement of the corpus rather than of this repo.
fn write_meter_config(scratch: &Scratch, corpus: &str) -> Result<String, String> {
    let cfg = scratch.root.join("meter.conf");
    write(
        &cfg,
        &format!(
            "CONTEXT_KIT_SURFACES=(\"{0}/surface.md\")\n\
             CONTEXT_KIT_HOOK_CMD=\"cat {0}/hook-sample.txt\"\n\
             CONTEXT_KIT_BASELINE_FILE=\"{0}/baseline.txt\"\n",
            corpus
        ),
    )?;
    Ok(cfg.display().to_string())
}

fn write(path: &Path, body: &str) -> Result<(), String> {
    std::fs::write(path, body).map_err(|e| format!("cannot write {}: {}", path.display(), e))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §Testing — `--update` is an argument the rule itself consumes, so
    // anything else is a refusal rather than the silently ignored word the shell form's `$1` read
    // left it.
    #[test]
    fn the_member_takes_only_update() {
        assert_eq!(run(&["--nope".to_string()]), 2);
    }

    // spec: context-kit/SPEC.md §Testing — `norm()`'s rewrite is a sub-behaviour no golden holds,
    // so it is asserted directly across the anchor, the greedy arm, and the untouched line.
    #[test]
    fn the_path_rewrite_makes_a_golden_machine_independent() {
        assert_eq!(
            norm_line("/elsewhere/checkout/context-kit/index-tests/corpus/sample.md  (23L)"),
            "corpus/sample.md  (23L)"
        );
        assert_eq!(
            norm_line("/a/corpus/b/corpus/sample.rs  (1)"),
            "corpus/sample.rs  (1)"
        );
        assert_eq!(norm_line("corpus/sample.md  (23L)"), "corpus/sample.md  (23L)");
        assert_eq!(
            norm_line("  ## Overview:5  — /tmp/corpus/x"),
            "  ## Overview:5  — /tmp/corpus/x",
            "a /corpus/ past the first space was rewritten, which the anchored regex cannot reach"
        );
    }

    // spec: context-kit/SPEC.md §Testing — the golden's byte shape, held because it is what makes a
    // `--update` run idempotent against a golden the shell form wrote.
    #[test]
    fn a_golden_carries_exactly_one_trailing_newline() {
        assert_eq!(payload("a\nb\n\n\n"), "a\nb\n");
        assert_eq!(payload(""), "\n");
        assert_eq!(payload("a"), "a\n");
    }
}
