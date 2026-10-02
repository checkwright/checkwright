// spec: installer/SPEC.md §The consumer smoke — the preflight and the arm roster: the headers are
// a parsed scenario roster (evidence-kit/SPEC.md §Layout and configuration), declared at the log's
// head from this table and printed from it, so the two cannot differ
use super::{refuse, Run, Step};
use crate::emit::parse_smoke_log::ROSTER_LINE;
use crate::{proc, programs, walk};

type Arm = fn(&mut Run) -> Step;

pub(super) struct Row {
    pub name: &'static str,
    pub detail: &'static str,
    pub arm: Arm,
}

const fn row(name: &'static str, detail: &'static str, arm: Arm) -> Row {
    Row { name, detail, arm }
}

// spec: installer/SPEC.md §The consumer smoke — the arms in run order under the names the
// baseline's rows carry; a parenthetical's `{min}`, `{max}`, `{version}`, `{up}` and `{bare}` are
// read off the run when its header prints, each set by an arm before it
pub(super) const ARMS: &[Row] = &[
    row("build", "the host gate binary the main payload carries", super::staging::build),
    row("pack", "", super::staging::pack),
    row("install", "from the tarball, --offline", super::staging::install),
    row("profile invariant", "", super::profiles::invariant),
    row("seed arm", "the CI workflow init seeded, every profile", super::profiles::seed),
    row("demo arm", "checkwright demo from the packed package, inside an installed {min} consumer", super::profiles::demo),
    row(
        "companion arm for speckit",
        "the Spec Kit recipe on its fixture tree, through the extension install command's line",
        super::companion::speckit,
    ),
    row(
        "companion arm for speckit full",
        "the extension install command's full line on its fixture tree",
        super::companion::speckit_full,
    ),
    row(
        "companion arm for speckit complement",
        "the extension install command's complement line on its fixture tree, the left-out kits held to companion/exclusions.list",
        super::companion::speckit_complement,
    ),
    row(
        "companion arm for openspec",
        "the OpenSpec recipe on its fixture tree, through the OpenSpec page's line",
        super::companion::openspec,
    ),
    row(
        "companion arm for openspec complement",
        "the OpenSpec page's complement line on its fixture tree, the left-out kits held to companion/exclusions.list",
        super::companion::openspec_complement,
    ),
    row(
        "companion arm for openspec lifecycle",
        "the OpenSpec page's full line, check-stage-entry over the lifecycle overlay",
        super::companion::openspec_lifecycle,
    ),
    row("plan parity over held seeds", "{max}", super::profiles::held_seeds),
    row("plan parity over deleted seeds with stale hashes", "{max}", super::profiles::re_seed),
    row("artifact-less refusal leg", "{bare}, payload packed with no artifact", super::masked::artifact_less),
    row("download arm", "{max}, node/npm masked", super::masked::download),
    row("toolchain-free arm", "{max}, cargo/rustc masked", super::masked::toolchain_free),
    row("jq-less arm", "{min}, jq absent from the verbs' PATH", super::masked::jq_less),
    row("bash-less arm", "the profiles owing no bash, bash absent from PATH", super::masked::bash_less),
    row(
        "upgrade arm",
        "two cross-version hops, {min} profile — the lattice minimum, so the arm is the smallest install that carries the manifest behavior it asserts",
        super::upgrade::upgrade,
    ),
    row("cross-version reversal arm", "three versions, no adopter edit, {min}", super::upgrade::cross_version_reversal),
    row(
        "newer-verb reversal arm",
        "installed at {version}, reversed by {up} with no upgrade, {min}",
        super::upgrade::newer_verb_reversal,
    ),
    row("hooked move arm", "{min} to {max}, then to {up}, hooks on", super::moves::hooked_move),
    row("seam arm", "same-version re-run, {max} profile", super::moves::seam),
    row("narrowing arm", "{max} installed, re-run at {min}", super::moves::narrowing),
    row("selection arm", "{min} with a kit added, a gate dropped and one added", super::moves::selection),
    row("artifact arm", "selection outcomes, on a mutated copy of the packed payload", super::artifact::artifact),
];

// spec: installer/SPEC.md §The consumer smoke — an arm's header: its name, then its parenthetical
// with the run's values in place
pub(super) fn header(row: &Row, state: &Run) -> String {
    if row.detail.is_empty() {
        return row.name.to_string();
    }
    let detail = row
        .detail
        .replace("{min}", &state.profile_min)
        .replace("{max}", super::consumer::PROFILE_DERIVED)
        .replace("{version}", &state.version)
        .replace("{up}", &state.up_version)
        .replace("{bare}", super::masked::BARE_PROFILE);
    format!("{} ({})", row.name, detail)
}

// spec: evidence-kit/SPEC.md §Layout and configuration — the completion marker, the roster's last
// name
pub(super) const MARKER: &str = "INSTALLER-SMOKE: clean";

fn declaration() -> String {
    ARMS.iter()
        .map(|r| r.name)
        .chain([MARKER])
        .map(|n| format!("{}{}\n", ROSTER_LINE, n))
        .collect()
}

pub(super) fn declare() {
    print!("{}", declaration());
}

// spec: installer/SPEC.md §The consumer smoke — the programs the driver spawns beside the running
// binary: git, npm and tar, and the host bootstrap's shell
#[cfg(unix)]
fn host_shell() -> programs::Program {
    programs::SH
}

#[cfg(not(unix))]
fn host_shell() -> programs::Program {
    programs::PWSH
}

// spec: installer/SPEC.md §The consumer smoke — every precondition refuses at exit 2 before the
// first arm: the scratch base and a set hand-off are directories, each program resolves, and the
// tree the run packs is clean
pub(super) fn preflight(state: &mut Run) -> Step {
    state.root = walk::toplevel().map_err(refuse)?;
    let base = undeclared("INSTALLER_SMOKE_TMP_DIR")
        .or_else(|| undeclared("TMPDIR"))
        .unwrap_or_else(|| std::env::temp_dir().display().to_string());
    if !std::path::Path::new(&base).is_dir() {
        return Err(refuse(format!("scratch base not a directory: {}", base)));
    }
    state.scratch = make_scratch(&base)?;
    if let Some(d) = undeclared("INSTALLER_SMOKE_ARTIFACTS_DIR") {
        if !std::path::Path::new(&d).is_dir() {
            return Err(refuse(format!("artifact hand-off not a directory: {}", d)));
        }
    }
    for tool in [programs::GIT, programs::NPM, programs::TAR, host_shell()] {
        if !proc::on_path(&tool) {
            return Err(refuse(format!("{} not found on PATH — the smoke cannot run.", tool)));
        }
    }
    let status = proc::run(&programs::GIT, &["-C", &state.root, "status", "--porcelain"]).map_err(refuse)?;
    match status.stdout() {
        Some([]) => Ok(()),
        Some(_) => Err(refuse(
            "the worktree is dirty — the pack step refuses to stamp a commit the payload does not \
             match. Commit or stash first.",
        )),
        None => Err(refuse(format!(
            "git status could not read {} — {}",
            state.root,
            status.failure_report().unwrap_or_default()
        ))),
    }
}

// spec: installer/SPEC.md §The consumer smoke — a value read off the process environment, set when
// non-empty
pub(super) fn undeclared(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

fn make_scratch(base: &str) -> Result<String, super::Outcome> {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let dir = format!(
        "{}/installer-smoke.{}.{}",
        base.trim_end_matches(['/', '\\']),
        std::process::id(),
        nanos
    );
    std::fs::create_dir(&dir).map_err(|e| refuse(format!("cannot create the scratch dir {}: {}", dir, e)))?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The consumer smoke — a scenario name is an arm's header up to its
    // parenthetical, so a rostered name carries none, and each names one arm
    #[test]
    fn the_roster_names_each_arm_once_and_bare() {
        let names: Vec<&str> = ARMS.iter().map(|r| r.name).collect();
        for (i, n) in names.iter().enumerate() {
            assert!(!names[..i].contains(n), "{} is rostered twice", n);
            assert!(!n.contains(" ("), "{} carries a parenthetical in its name", n);
        }
        assert!(MARKER.starts_with(super::super::VERDICT));
    }

    // spec: installer/SPEC.md §The consumer smoke — every parenthetical renders with no
    // placeholder left, and a bare row prints its name alone
    #[test]
    fn every_header_renders_with_no_placeholder_left() {
        let mut state = Run::default();
        state.profile_min = "starter".to_string();
        state.version = "1.2.3".to_string();
        state.up_version = "1.2.4".to_string();
        for r in ARMS {
            let h = header(r, &state);
            assert!(h.starts_with(r.name), "{} renders as {}", r.name, h);
            assert!(!h.contains('{') && !h.contains('}'), "{} left a placeholder: {}", r.name, h);
        }
        assert_eq!(header(&ARMS[1], &state), "pack");
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — the declaration this arm prints is
    // what the parser's log-only form reads: a clean log yields every arm `pass`, in table order
    #[test]
    fn the_declared_roster_is_the_one_the_log_only_parser_reads() {
        let mut log = declaration();
        for r in ARMS {
            log.push_str(&format!("{} (detail)\n  narration\n", r.name));
        }
        log.push_str(&format!("{} (summary)\n", MARKER));
        let path = std::env::temp_dir().join(format!("cw-installer-smoke-roster-{}.log", std::process::id()));
        std::fs::write(&path, log).expect("write the log");
        let got = crate::emit::parse_smoke_log::emit(&[path.display().to_string()]);
        let _ = std::fs::remove_file(&path);
        let want: String = ARMS.iter().map(|r| format!("{} pass\n", r.name.replace(' ', "-"))).collect();
        assert_eq!(got.expect("the parser refused the arm's own declaration"), want);
    }
}
