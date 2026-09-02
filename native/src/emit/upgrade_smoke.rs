// spec: gate-sdk/SPEC.md §upgrade-smoke — the two-phase upgrade proof, bridged as an `Arm::Run`
// member: its contract is the exit status (2 broken tag or environment, 1 upgrade finding, 0 clean
// with one verdict line on stdout), which `Arm::Emit` collapses to 0-or-2
use crate::declaration::{self, SectionVerdict};
use crate::ere::Ere;
use crate::proc::{self, Stderr};
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §upgrade-smoke — the five knobs the resolve step reads, plus the kit-roots
// transport: a compiled arm has no `BASH_SOURCE` anchor to find its own kit library from.
pub const KNOBS: &[&str] = &[
    "GATE_SDK_UPGRADE_REPO",
    "GATE_SDK_UPGRADE_FROM",
    "GATE_SDK_UPGRADE_TO",
    "GATE_SDK_TMP_DIR",
    "GATE_SDK_WORKFLOW_DIR",
    "GATE_KIT_ROOTS_HERE",
];

const NAME: &str = "upgrade-smoke";
const SECTION: &str = "Tightened gates";

// spec: gate-sdk/SPEC.md §upgrade-smoke — the exit-status contract as a type, so a finding cannot
// be raised on a broken environment's spelling or the reverse
#[derive(Debug)]
struct Fail {
    code: i32,
    report: Vec<String>,
}

fn broken(lines: Vec<String>) -> Fail {
    Fail {
        code: 2,
        report: lines,
    }
}

fn finding(lines: Vec<String>) -> Fail {
    Fail {
        code: 1,
        report: lines,
    }
}

fn one(line: String) -> Vec<String> {
    vec![line]
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — a knob the bridge could not resolve is a broken
// environment, never an upgrade finding
fn knob(name: &str) -> Result<String, Fail> {
    walk::knob_scalar(name).map_err(|e| broken(one(format!("{}: {}", NAME, e))))
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the per-ref worktrees, the extracted trees and the
// scratch consumer are trap-removed in the shell form; `Drop` is that trap, in the same order
struct Scratch {
    repo: String,
    worktrees: Vec<String>,
    work: String,
    consumer: String,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        for w in &self.worktrees {
            let _ = proc::run(
                "git",
                &["-C", &self.repo, "worktree", "remove", "--force", w],
            );
        }
        if !self.work.is_empty() {
            let _ = std::fs::remove_dir_all(&self.work);
        }
        if !self.consumer.is_empty() {
            let _ = std::fs::remove_dir_all(&self.consumer);
        }
    }
}

pub fn run(args: &[String]) -> i32 {
    // spec: gate-sdk/SPEC.md §upgrade-smoke — the member takes no positional and no flag, so an
    // argument is a refusal rather than a silently ignored word
    if let Some(a) = args.first() {
        eprintln!("{}: takes no arguments (got: {})", NAME, a);
        return 2;
    }
    match smoke() {
        Ok(line) => {
            println!("{}", line);
            0
        }
        Err(f) => {
            for l in &f.report {
                eprintln!("{}", l);
            }
            f.code
        }
    }
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the whole suite in the shell driver's order: resolve,
// archive both refs, baseline at FROM, swap to TO, judge determinism, regenerate, then contain the
// phase-B red set inside TO's declaration
fn smoke() -> Result<String, Fail> {
    let repo = resolve_repo()?;
    let from = resolve_from(&repo)?;
    let to = resolve_to(&repo)?;
    let sdk = sdk_root()?;

    let base = scratch_base()?;
    let work = mktemp_dir(&base)?;
    let mut env = Scratch {
        repo: repo.clone(),
        worktrees: Vec::new(),
        work: work.clone(),
        consumer: String::new(),
    };

    let from_tree = format!("{}/from", work);
    let to_tree = format!("{}/to", work);
    for t in [&from_tree, &to_tree] {
        std::fs::create_dir_all(t)
            .map_err(|e| broken(one(format!("{}: cannot create {}: {}", NAME, t, e))))?;
    }
    extract(&repo, &from, &from_tree)?;
    extract(&repo, &to, &to_tree)?;

    let fromroots = kit_dirs_in(&from_tree)?;
    let toroots = kit_dirs_in(&to_tree)?;
    if fromroots.is_empty() {
        return Err(broken(one(format!(
            "{}: no vendorable kits at FROM ({})",
            NAME, from
        ))));
    }
    if toroots.is_empty() {
        return Err(broken(one(format!(
            "{}: no vendorable kits at TO ({})",
            NAME, to
        ))));
    }

    // spec: gate-sdk/SPEC.md §upgrade-smoke — step 1: FROM's kits paired with FROM's own binary, so
    // phase 1's claim is about FROM alone
    let mut from_bin = String::new();
    if descriptors(&sdk, &fromroots)? > 0 {
        from_bin = ref_binary_tree(&mut env, &from, "from", &work)?;
    }
    let consumer = vendor_and_install(&mut env, &sdk, &base, &from_bin, &fromroots, &from)?;

    let (rc, out) = run_battery(&consumer)?;
    if rc != 0 || !green(&out)? {
        let mut r = one(format!(
            "{}: FAIL(env) — the FROM baseline ({}) is not green under zero config; the tag itself is broken, not an upgrade finding",
            NAME, from
        ));
        r.push(out.trim_end().to_string());
        return Err(broken(r));
    }

    // spec: gate-sdk/SPEC.md §upgrade-smoke — phase A, step 1 of 2: the vendored kit directories are
    // replaced wholesale at TO, the contract's consumer step
    let mut seen: Vec<String> = Vec::new();
    for r in &fromroots {
        let k = basename(r);
        let _ = std::fs::remove_dir_all(format!("{}/{}", consumer, k));
    }
    for r in &toroots {
        let k = basename(r);
        copy_tree(r, &format!("{}/{}", consumer, k))?;
        seen.push(k);
    }
    for r in &fromroots {
        let k = basename(r);
        if !seen.contains(&k) {
            seen.push(k);
        }
    }

    // spec: gate-sdk/SPEC.md §upgrade-smoke — the binary is re-placed in the motion that swaps the
    // kit directories, because that swap *is* the upgrade transition
    if descriptors(&sdk, &toroots)? > 0 {
        let tree = ref_binary_tree(&mut env, &to, "to", &work)?;
        place_binary(&sdk, &consumer, &tree, &toroots, &to)?;
    }

    determinism(&consumer, &seen)?;
    regenerate(&consumer, &to)?;
    commit_phase_a(&consumer, &to)?;

    let (decl_src, allowed) = declared_set(&repo, &to, &to_tree)?;
    let (rc, out) = run_battery(&consumer)?;
    let red = red_set(rc, &out)?;

    if !red.is_empty() && decl_src.is_empty() {
        return Err(no_declaration(&repo, &to, &red)?);
    }
    let undeclared: Vec<&String> = red.iter().filter(|g| !allowed.contains(g)).collect();
    if !undeclared.is_empty() {
        let mut r = one(format!(
            "{}: FAIL — gate(s) went red that TO's tightened-gates declaration does not name:",
            NAME
        ));
        for g in &undeclared {
            r.push(format!("  {}", g));
        }
        r.push(format!(
            "  each red must be named in {} — a bullet in the note's Tightened gates section, or a data line of the declaration surface — or the tree fixed (docs/install.md §The upgrade contract).",
            decl_src
        ));
        return Err(finding(r));
    }

    Ok(format!(
        "UPGRADE-SMOKE: clean ({} → {}; {}→{} kits vendored, phase A deterministic, red set {} ⊆ {} declared by {})",
        from,
        to,
        fromroots.len(),
        toroots.len(),
        red.len(),
        allowed.len(),
        if decl_src.is_empty() {
            "no declaration".to_string()
        } else {
            decl_src
        }
    ))
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — `GATE_SDK_UPGRADE_REPO` empty means *derive it*: the
// enclosing repo's toplevel, the value the deleted driver computed inline.
// spec: gate-sdk/SPEC.md §upgrade-smoke — `-d "$REPO/.git"` is carried across **verbatim**, refusal
// included: it refuses inside a linked worktree, where `.git` is a pointer file, and settling that
// live `upgrade-smoke-refuses-inside-a-worktree` fork is design work a port does not do.
fn resolve_repo() -> Result<String, Fail> {
    let mut repo = knob("GATE_SDK_UPGRADE_REPO")?;
    if repo.is_empty() {
        repo = walk::toplevel_opt().unwrap_or(None).unwrap_or_default();
    }
    if repo.is_empty() || !Path::new(&format!("{}/.git", repo)).is_dir() {
        let shown = if repo.is_empty() { "<unset>" } else { &repo };
        return Err(broken(one(format!(
            "{}: GATE_SDK_UPGRADE_REPO is not a git repository: {}",
            NAME, shown
        ))));
    }
    Ok(repo)
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — `GATE_SDK_UPGRADE_FROM` empty means *derive it*: the
// source repo's newest `v*` tag, and none resolvable is exit 2 rather than a skip
fn resolve_from(repo: &str) -> Result<String, Fail> {
    let mut from = knob("GATE_SDK_UPGRADE_FROM")?;
    if from.is_empty() {
        let c = proc::run(
            "git",
            &["-C", repo, "tag", "--list", "v*", "--sort=-v:refname"],
        )
        .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
        from = c
            .stdout()
            .map(|o| String::from_utf8_lossy(o).into_owned())
            .unwrap_or_default()
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
    }
    if from.is_empty() {
        return Err(broken(one(format!(
            "{}: no FROM ref — GATE_SDK_UPGRADE_FROM unset and no v* tag in {}; the baseline is unresolvable",
            NAME, repo
        ))));
    }
    if !resolves(repo, &from) {
        return Err(broken(one(format!(
            "{}: FROM ref does not resolve to a commit: {}",
            NAME, from
        ))));
    }
    Ok(from)
}

fn resolve_to(repo: &str) -> Result<String, Fail> {
    let to = knob("GATE_SDK_UPGRADE_TO")?;
    if !resolves(repo, &to) {
        return Err(broken(one(format!(
            "{}: TO ref does not resolve to a commit: {}",
            NAME, to
        ))));
    }
    Ok(to)
}

fn resolves(repo: &str, git_ref: &str) -> bool {
    let spec = format!("{}^{{commit}}", git_ref);
    matches!(
        proc::run("git", &["-C", repo, "rev-parse", "--verify", "-q", &spec]),
        Ok(c) if c.stdout().is_some()
    )
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the arm reaches gate-sdk's own library through the
// transported kit roots rather than through a path relative to itself: a binary the installer
// copied elsewhere cannot recover the shell form's `BASH_SOURCE` anchor (§lib/gate.sh).
fn sdk_root() -> Result<String, Fail> {
    let roots = walk::kit_roots_abs().map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    roots
        .into_iter()
        .find(|r| basename(r) == "gate-sdk")
        .ok_or_else(|| {
            broken(one(format!(
                "{}: GATE_KIT_ROOTS_HERE names no gate-sdk root, so the consumer-smoke library this suite drives cannot be found",
                NAME
            )))
        })
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the scratch base is `GATE_SDK_TMP_DIR`, absolutized the
// way the shell form's `cd && pwd` did, because every child below is spawned from another directory
fn scratch_base() -> Result<String, Fail> {
    let dir = knob("GATE_SDK_TMP_DIR")?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| broken(one(format!("{}: cannot create scratch base {}: {}", NAME, dir, e))))?;
    let here = walk::cwd().map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    Ok(walk::abs_against(&here, &dir))
}

fn mktemp_dir(base: &str) -> Result<String, Fail> {
    let template = format!("{}/upgrade-smoke.XXXXXX", base);
    let c = proc::run("mktemp", &["-d", &template])
        .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    match c.stdout() {
        Some(o) => Ok(String::from_utf8_lossy(o).trim().to_string()),
        None => Err(broken(one(format!(
            "{}: cannot create a scratch directory under {}",
            NAME, base
        )))),
    }
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — `git archive | tar -x`, the committed-content-only read
// the untagged-TO declaration arm depends on. The pipeline keeps its shell spelling because
// `pipefail` over an abandoned producer is the property being relied on.
fn extract(repo: &str, git_ref: &str, tree: &str) -> Result<(), Fail> {
    let script = r#"set -o pipefail; git -C "$1" archive "$2" | tar -x -C "$3""#;
    let done = bash(&[script, "bash", repo, git_ref, tree], Stderr::Inherit)?;
    if done.code() != 0 {
        return Err(broken(one(format!(
            "{}: git archive of {} failed",
            NAME, git_ref
        ))));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — a ref's vendorable kits are the dirs shipping
// `smoke/install.sh` (§Consumer smoke's per-kit contract), gate-sdk first
fn kit_dirs_in(tree: &str) -> Result<Vec<String>, Fail> {
    let mut out: Vec<String> = Vec::new();
    if Path::new(&format!("{}/gate-sdk/smoke/install.sh", tree)).is_file() {
        out.push(format!("{}/gate-sdk", tree));
    }
    let entries = walk::list_dir(Path::new(tree))
        .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    for (name, is_dir) in entries {
        if !is_dir || name.starts_with('.') || name == "gate-sdk" {
            continue;
        }
        if Path::new(&format!("{}/{}/smoke/install.sh", tree, name)).is_file() {
            out.push(format!("{}/{}", tree, name));
        }
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the three consumer-smoke helpers are **called in the
// library that owns them**, never reimplemented: the arm spawns `bash`, sources the unchanged
// library and reads back the one value the caller needs (§The port-candidate criteria, criterion 6).
fn csmoke(script: &str, args: &[&str], stderr: Stderr) -> Result<proc::Streamed, Fail> {
    let mut argv: Vec<&str> = vec![script, "bash"];
    argv.extend_from_slice(args);
    bash(&argv, stderr)
}

fn bash(argv: &[&str], stderr: Stderr) -> Result<proc::Streamed, Fail> {
    let mut args: Vec<&str> = vec!["-c"];
    args.extend_from_slice(argv);
    proc::run_streamed("bash", &args, b"", stderr)
        .map_err(|e| broken(one(format!("{}: {}", NAME, e))))
}

const SOURCE_CSMOKE: &str =
    r#"source "$1/lib/gate.sh"; source "$1/lib/consumer-smoke.sh";"#;

fn descriptors(sdk: &str, roots: &[String]) -> Result<i64, Fail> {
    let script = format!("{} shift; csmoke_gate_descriptors \"$@\"", SOURCE_CSMOKE);
    let refs: Vec<&str> = std::iter::once(sdk)
        .chain(roots.iter().map(String::as_str))
        .collect();
    let done = csmoke(&script, &refs, Stderr::Inherit)?;
    let text = String::from_utf8_lossy(done.stdout()).trim().to_string();
    text.parse::<i64>().map_err(|_| {
        broken(one(format!(
            "{}: csmoke_gate_descriptors answered '{}', which is not a count",
            NAME, text
        )))
    })
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — `csmoke_vendor_and_install` communicates by setting its
// caller's `SCRATCH`, which no process boundary carries, so the seam is the library's own contract
// read out on stdout: everything the helper prints goes to stderr and the directory comes back.
fn vendor_and_install(
    env: &mut Scratch,
    sdk: &str,
    base: &str,
    host: &str,
    roots: &[String],
    from: &str,
) -> Result<String, Fail> {
    let script = format!(
        "{} TMPDIR=\"$2\"; export TMPDIR; host=\"$3\"; shift 3; \
         csmoke_vendor_and_install \"$host\" \"$@\" 1>&2; st=$?; printf '%s' \"$SCRATCH\"; exit $st",
        SOURCE_CSMOKE
    );
    let refs: Vec<&str> = vec![sdk, base, host]
        .into_iter()
        .chain(roots.iter().map(String::as_str))
        .collect();
    let done = csmoke(&script, &refs, Stderr::Inherit)?;
    let scratch = String::from_utf8_lossy(done.stdout()).trim().to_string();
    env.consumer = scratch.clone();
    if done.code() != 0 || scratch.is_empty() {
        return Err(broken(one(format!(
            "{}: vendoring the FROM baseline ({}) failed — a broken tag, not an upgrade finding",
            NAME, from
        ))));
    }
    Ok(scratch)
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — `csmoke_place_binary` reads its caller's `SCRATCH` too,
// so the same seam runs in the input direction: the arm supplies the variable the helper's own
// contract names and the library is untouched.
fn place_binary(
    sdk: &str,
    consumer: &str,
    host: &str,
    roots: &[String],
    to: &str,
) -> Result<(), Fail> {
    let script = format!(
        "{} SCRATCH=\"$2\"; host=\"$3\"; shift 3; csmoke_place_binary \"$host\" \"$@\" 1>&2",
        SOURCE_CSMOKE
    );
    let refs: Vec<&str> = vec![sdk, consumer, host]
        .into_iter()
        .chain(roots.iter().map(String::as_str))
        .collect();
    let done = csmoke(&script, &refs, Stderr::Inherit)?;
    if done.code() != 0 {
        return Err(broken(one(format!(
            "{}: FAIL(env) — could not place TO ({})'s gate binary in the scratch consumer",
            NAME, to
        ))));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — a ref's binary is built from a **detached worktree** at
// that ref and never from the archive its kits come from: `native/build.rs` stamps its source with
// `git ls-files` and panics outside a checkout, which would read as a broken tag.
fn ref_binary_tree(
    env: &mut Scratch,
    git_ref: &str,
    label: &str,
    work: &str,
) -> Result<String, Fail> {
    if !proc::on_path("cargo") {
        return Err(broken(one(format!(
            "{}: FAIL(env) — the {} ref ({}) dispatches gate(s) to the binary and cargo is not on PATH; this suite builds one binary per ref",
            NAME, label, git_ref
        ))));
    }
    let wt = format!("{}/checkout-{}", work, label);
    let added = proc::run(
        "git",
        &[
            "-C", &env.repo, "worktree", "add", "--detach", "-q", &wt, git_ref,
        ],
    )
    .map(|c| c.stdout().is_some())
    .unwrap_or(false);
    if !added {
        return Err(broken(one(format!(
            "{}: FAIL(env) — could not add a detached worktree at the {} ref ({})",
            NAME, label, git_ref
        ))));
    }
    env.worktrees.push(wt.clone());
    // spec: gate-sdk/SPEC.md §upgrade-smoke — a ref that dispatches and carries no crate is a tag
    // fact; the one thing this must not do is fall back to the host's binary
    if !Path::new(&format!("{}/native", wt)).is_dir() {
        return Err(broken(one(format!(
            "{}: FAIL(env) — the {} ref ({}) dispatches gate(s) to the binary and carries no crate to build one from; a broken tag, not an upgrade finding",
            NAME, label, git_ref
        ))));
    }
    let built = proc::run_merged(
        "bash",
        &["-c", r#"cd "$1/native" && exec cargo build --release"#, "bash", &wt],
    )
    .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    if !built.succeeded() {
        return Err(broken(vec![
            format!(
                "{}: FAIL(env) — the {} ref ({}) will not build its gate binary under this toolchain; an environment or tag fact, never an upgrade finding",
                NAME, label, git_ref
            ),
            String::from_utf8_lossy(built.output()).into_owned(),
        ]));
    }
    Ok(wt)
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the scratch consumer's battery, run at its own ref
// against its own ref's binary, with the two streams merged as the shell form's `2>&1` merged them
fn run_battery(consumer: &str) -> Result<(i32, String), Fail> {
    let m = proc::run_merged(
        "bash",
        &[
            "-c",
            r#"cd "$1" && exec bash gate-sdk/bin/run-gates.sh"#,
            "bash",
            consumer,
        ],
    )
    .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    Ok((
        m.reported_code(),
        String::from_utf8_lossy(m.output()).into_owned(),
    ))
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the battery summary is read out of the *scratch
// consumer's* stdout, a producer at a different ref: the two literals are the recorded cross-ref
// coupling, and calling the runner in process would destroy the pairing the suite asserts.
fn green(out: &str) -> Result<bool, Fail> {
    Ok(compiled("All [0-9]+ gates passed")?.is_match(out))
}

fn compiled(pattern: &str) -> Result<Ere, Fail> {
    Ere::compile(pattern).map_err(|e| {
        broken(one(format!(
            "{}: cannot compile the battery-summary pattern {}: {}",
            NAME, pattern, e
        )))
    })
}

fn basename(p: &str) -> String {
    p.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(p)
        .to_string()
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — `cp -R` rather than a crate-side tree walk: the kit trees
// carry executable bits the installer runs off, and `cp` is on `GATE_SDK_PROGRAM_FLOOR`
fn copy_tree(src: &str, dest: &str) -> Result<(), Fail> {
    let done = bash(
        &[r#"cp -R "$1" "$2""#, "bash", src, dest],
        Stderr::Inherit,
    )?;
    if done.code() != 0 {
        return Err(broken(one(format!(
            "{}: cannot place the TO kit {} in the scratch consumer",
            NAME,
            basename(src)
        ))));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — determinism is measured on the **sync alone**, before any
// regen step has run, so the claim stays exactly the sync's: it loses nothing a consumer owns
fn determinism(consumer: &str, seen: &[String]) -> Result<(), Fail> {
    stage_all(consumer)?;
    let c = proc::run("git", &["-C", consumer, "diff", "--cached", "--name-only"])
        .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    let listed = c
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())
        .ok_or_else(|| {
            broken(one(format!(
                "{}: cannot read the scratch consumer's staged set",
                NAME
            )))
        })?;
    let stray: Vec<&str> = listed
        .lines()
        .filter(|p| !p.is_empty())
        .filter(|p| {
            let top = p.split('/').next().unwrap_or(p);
            !seen.iter().any(|k| k == top)
        })
        .collect();
    if stray.is_empty() {
        return Ok(());
    }
    let mut r = one(format!(
        "{}: FAIL — the phase-A kit sync is non-deterministic: it changed consumer files outside the kit roots:",
        NAME
    ));
    for p in stray {
        r.push(format!("  {}", p));
    }
    r.push(
        "  the wholesale kit-sync must lose nothing a consumer owns (docs/install.md §The upgrade contract).".to_string(),
    );
    Err(finding(r))
}

fn stage_all(consumer: &str) -> Result<(), Fail> {
    proc::run("git", &["-C", consumer, "add", "-A"])
        .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    Ok(())
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — phase A, step 2 of 2: the regen runs *after* the sync has
// been judged, and the artifact's path is resolved in the **consumer's** library rather than this
// process's, because the host's value is a different tree's.
fn regenerate(consumer: &str, to: &str) -> Result<(), Fail> {
    let hook = bash(
        &[
            r#"cd "$1" && exec bash gate-sdk/bin/gen-pre-commit.sh --write >/dev/null"#,
            "bash",
            consumer,
        ],
        Stderr::Inherit,
    )?;
    if hook.code() != 0 {
        return Err(broken(one(format!(
            "{}: phase A gen-pre-commit failed at TO ({})",
            NAME, to
        ))));
    }

    let resolved = bash(
        &[
            r#"cd "$1" && source gate-sdk/lib/gate.sh && printf '%s' "$GATE_SDK_GRAPH_ARTIFACT""#,
            "bash",
            consumer,
        ],
        Stderr::Inherit,
    )?;
    let artifact = String::from_utf8_lossy(resolved.stdout()).trim().to_string();
    if resolved.code() != 0 {
        return Err(broken(one(format!(
            "{}: could not resolve the graph artifact path at TO ({})",
            NAME, to
        ))));
    }
    if artifact.is_empty() {
        return Err(broken(one(format!(
            "{}: the consumer's library resolved an empty graph artifact path at TO ({})",
            NAME, to
        ))));
    }

    let emitted = bash(
        &[
            r#"cd "$1" && exec bash gate-sdk/bin/run-gates.sh --emit graph > "$2""#,
            "bash",
            consumer,
            &artifact,
        ],
        Stderr::Inherit,
    )?;
    if emitted.code() != 0 {
        return Err(broken(one(format!(
            "{}: phase A graph emit failed at TO ({})",
            NAME, to
        ))));
    }

    if Path::new(&format!("{}/doctrine-kit/bin/install-doctrine.sh", consumer)).is_file() {
        let doctrine = bash(
            &[
                r#"cd "$1" && exec bash doctrine-kit/bin/install-doctrine.sh >/dev/null"#,
                "bash",
                consumer,
            ],
            Stderr::Inherit,
        )?;
        if doctrine.code() != 0 {
            return Err(broken(one(format!(
                "{}: phase A install-doctrine failed at TO ({})",
                NAME, to
            ))));
        }
    }
    Ok(())
}

fn commit_phase_a(consumer: &str, to: &str) -> Result<(), Fail> {
    stage_all(consumer)?;
    let msg = format!("phase A: kits at {}", to);
    proc::run(
        "git",
        &[
            "-C", consumer, "-c", "user.email=smoke@example.invalid", "-c", "user.name=smoke",
            "commit", "-q", "--no-verify", "--allow-empty", "-m", &msg,
        ],
    )
    .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    Ok(())
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the declaration resolves on two arms over one token
// predicate, read **in crate** through `native/src/declaration.rs`: a tagged TO from the note whose
// front matter names its version, an untagged TO from the declaration surface in TO's own tree.
fn declared_set(repo: &str, to: &str, to_tree: &str) -> Result<(String, Vec<String>), Fail> {
    let ver = points_at(repo, to);
    let workflow = knob("GATE_SDK_WORKFLOW_DIR")?;
    let decl_file = format!("{}/{}/tightened-gates.txt", to_tree, workflow);

    let mut tokens: Vec<String> = Vec::new();
    let mut src = String::new();
    if !ver.is_empty() {
        if let Some(note) = release_note(to_tree, &ver)? {
            src = note.clone();
            match declaration::section_tokens(&read(&note)?, SECTION) {
                SectionVerdict::Absent => {
                    return Err(finding(vec![
                        format!(
                            "{}: FAIL — TO ({}) resolves note {}, which carries no '{}' section:",
                            NAME, ver, note, SECTION
                        ),
                        "  every release note carries the fixed sections its note grammar rosters (docs/install.md §The upgrade contract).".to_string(),
                    ]))
                }
                SectionVerdict::Unparsed(bad) => {
                    return Err(unparsed(&ver, &src, &bad));
                }
                SectionVerdict::ExplicitNone => {}
                SectionVerdict::Tokens(t) => tokens = t,
            }
        }
    } else if Path::new(&decl_file).is_file() {
        src = decl_file.clone();
        match declaration::record_tokens(&read(&decl_file)?) {
            Ok(t) => tokens = t,
            Err(bad) => return Err(unparsed(to, &src, &bad)),
        }
    }

    tokens.retain(|t| !t.trim().is_empty());
    tokens.sort();
    tokens.dedup();
    Ok((src, tokens))
}

fn unparsed(named: &str, src: &str, bad: &[String]) -> Fail {
    let mut r = one(format!(
        "{}: FAIL — TO ({})'s tightened-gates declaration does not parse, so it would resolve to a silently empty allowed-red set — {}:",
        NAME, named, src
    ));
    // spec: gate-sdk/SPEC.md §upgrade-smoke — the finding list is printed as the shell holder
    // printed it: one indented block, not one indent per line, so the two forms agree byte for byte
    if !bad.is_empty() {
        r.push(format!("  {}", bad.join("\n")));
    }
    r.push(
        "  a declaration is either an explicit 'None' or a non-empty set of bare gate names; in a note each is the backticked, unbolded lead token of a bullet (docs/install.md §The upgrade contract).".to_string(),
    );
    finding(r)
}

fn read(path: &str) -> Result<String, Fail> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| broken(one(format!("{}: cannot read {}: {}", NAME, path, e))))
}

fn points_at(repo: &str, to: &str) -> String {
    match proc::run(
        "git",
        &["-C", repo, "tag", "--points-at", to, "--list", "v*"],
    ) {
        Ok(c) => c
            .stdout()
            .map(|o| String::from_utf8_lossy(o).into_owned())
            .unwrap_or_default()
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string(),
        Err(_) => String::new(),
    }
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the note is the one under TO's `docs/posts/` whose front
// matter's `release:` names the tag, taken in glob order as the shell form took it
fn release_note(to_tree: &str, ver: &str) -> Result<Option<String>, Fail> {
    let dir = format!("{}/docs/posts", to_tree);
    if !Path::new(&dir).is_dir() {
        return Ok(None);
    }
    let entries = walk::list_dir(Path::new(&dir))
        .map_err(|e| broken(one(format!("{}: {}", NAME, e))))?;
    let re = compiled(&format!("^release:[[:space:]]+{}[[:space:]]*$", ver))?;
    for (name, is_dir) in entries {
        if is_dir || !name.ends_with(".md") {
            continue;
        }
        let path = format!("{}/{}", dir, name);
        if read(&path)?.lines().any(|l| re.is_match(l)) {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

// spec: gate-sdk/SPEC.md §upgrade-smoke — the phase-B red set, read off the summary line's own
// grammar: a red battery that printed no summary line is a finding rather than an empty red set
fn red_set(rc: i32, out: &str) -> Result<Vec<String>, Fail> {
    if rc == 0 && green(out)? {
        return Ok(Vec::new());
    }
    let re = compiled("^[0-9]+ of [0-9]+ gates FAILED:")?;
    let line = out.lines().rfind(|l| re.is_match(l));
    let Some(line) = line else {
        return Err(finding(vec![
            format!(
                "{}: FAIL — the phase-B battery is red but printed no 'FAILED:' summary line to read the red set from",
                NAME
            ),
            out.trim_end().to_string(),
        ]));
    };
    let tail = match line.split_once("FAILED: ") {
        Some((_, t)) => t,
        None => "",
    };
    Ok(tail.split_whitespace().map(String::from).collect())
}

fn no_declaration(repo: &str, to: &str, red: &[String]) -> Result<Fail, Fail> {
    let ver = points_at(repo, to);
    let named = if ver.is_empty() { to } else { &ver };
    let mut r = one(format!(
        "{}: FAIL — TO ({}) reddened gate(s) but declares no tightened-gates set anywhere:",
        NAME, named
    ));
    for g in red {
        r.push(format!("  {}", g));
    }
    if !ver.is_empty() {
        r.push(format!(
            "  no docs/posts note carries 'release: {}'; a red gate needs a note bullet (docs/install.md §The upgrade contract).",
            ver
        ));
    } else {
        let workflow = knob("GATE_SDK_WORKFLOW_DIR")?;
        r.push(format!(
            "  an untagged TO reads {}/tightened-gates.txt, which TO's tree does not carry; the build stage that lands or tightens a gate appends its name there (gate-sdk/SPEC.md §upgrade-smoke).",
            workflow
        ));
    }
    Ok(finding(r))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §upgrade-smoke — the two battery-summary literals are a cross-ref
    // coupling with a producer at another ref, so they are asserted rather than assumed
    #[test]
    fn the_summary_literals_read_the_producers_own_grammar() {
        assert!(green("All 108 gates passed.").expect("uncompilable"));
        assert!(!green("All gates passed").expect("uncompilable"));
        let red = red_set(1, "2 of 108 gates FAILED: check-a check-b\n")
            .expect("the summary line was unreadable");
        assert_eq!(red, vec!["check-a".to_string(), "check-b".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §upgrade-smoke — a red battery with no summary line is a finding, not
    // a silently empty red set that would contain inside any declaration
    #[test]
    fn a_red_battery_with_no_summary_line_is_a_finding() {
        let e = red_set(1, "the runner died\n").expect_err("an unreadable red set resolved");
        assert_eq!(e.code, 1);
    }

    // spec: gate-sdk/SPEC.md §upgrade-smoke — the argv question binds zero times: the member takes
    // no positional and no flag, so anything is a refusal
    #[test]
    fn the_member_takes_no_arguments() {
        assert_eq!(run(&["--anything".to_string()]), 2);
    }
}
