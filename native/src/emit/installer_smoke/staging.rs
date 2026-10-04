// spec: installer/SPEC.md §The consumer smoke — the staging and packing arms: the binary running
// the assertions is staged as the payload's artifact, the tree it runs in is packed, and the
// package is installed from the tarball with no registry access
use super::roster::undeclared;
use super::{fail, in_consumer, refuse, say, text, Outcome, Run, Step};
use crate::{proc, programs, walk};
use std::path::Path;

const REBUILD: &str = "run `bash gate-sdk/bin/build-native.sh`, then the smoke";

// spec: installer/SPEC.md §The consumer smoke — the host bootstrap's detector, run with no mapping
// held here: on a unix host the helper that extracts the POSIX bootstrap's four functions, and on
// native Windows the PowerShell bootstrap's three, extracted through PowerShell's parser
#[cfg(unix)]
fn detector(state: &Run, args: &[&str]) -> Result<proc::Completed, Outcome> {
    let helper = format!("{}/installer/consumer-smoke/host-target.sh", state.root);
    let mut argv = vec![helper.as_str()];
    argv.extend_from_slice(args);
    proc::run(&programs::SH, &argv).map_err(refuse)
}

#[cfg(not(unix))]
const DETECTOR: &str = "param([string] $Bootstrap, [string] $Fallback)
$ErrorActionPreference = 'Stop'
$ast = [System.Management.Automation.Language.Parser]::ParseFile($Bootstrap, [ref] $null, [ref] $null)
foreach ($name in 'Get-HostShape', 'Get-HostTarget', 'Get-FallbackTarget') {
    $fn = $ast.Find({ param($a) $a -is [System.Management.Automation.Language.FunctionDefinitionAst] -and $a.Name -eq $name }, $true)
    if (-not $fn) { [Console]::Error.WriteLine(\"host-target: $Bootstrap carries no $name\"); exit 2 }
    . ([scriptblock]::Create($fn.Extent.Text))
}
if ($PSBoundParameters.ContainsKey('Fallback')) { [Console]::Out.WriteLine((Get-FallbackTarget -Target $Fallback)); exit 0 }
$answer = Get-HostTarget
if (-not $answer) { [Console]::Error.WriteLine(\"host-target: the bootstrap maps this host, detected as $(Get-HostShape), to no triple\"); exit 2 }
[Console]::Out.WriteLine($answer)
";

#[cfg(not(unix))]
fn detector(state: &Run, args: &[&str]) -> Result<proc::Completed, Outcome> {
    let script = format!("{}/host-target.ps1", state.scratch);
    std::fs::write(&script, DETECTOR).map_err(|e| refuse(format!("could not write the detector extraction {}: {}", script, e)))?;
    let bootstrap = format!("{}/installer/bin/checkwright.ps1", state.root);
    let mut argv = vec!["-NoProfile", "-NonInteractive", "-File", script.as_str(), "-Bootstrap", bootstrap.as_str()];
    if let ["--fallback", triple] = args {
        argv.extend(["-Fallback", *triple]);
    }
    proc::run(&programs::PWSH, &argv).map_err(refuse)
}

// spec: installer/SPEC.md §The consumer smoke — the host triple is the host bootstrap's own
// detector's answer, so the arm holds no mapping
fn host_target(state: &Run) -> Result<String, Outcome> {
    let done = detector(state, &[])?;
    let triple = done.stdout().map(|o| text(o).trim().to_string()).unwrap_or_default();
    if done.failure_report().is_some() || triple.is_empty() {
        return Err(refuse(format!(
            "the bootstrap's detector gave no triple for this host — {} — the arm cannot tell \
             which roster line this machine satisfies.",
            done.failure_report().unwrap_or_default()
        )));
    }
    Ok(triple)
}

// spec: installer/SPEC.md §The consumer smoke — the triple the bootstrap's own fallback map names
// for one, through the same detector, empty where the map names none
pub(super) fn fallback_of(state: &Run, triple: &str) -> Result<String, Outcome> {
    let done = detector(state, &["--fallback", triple])?;
    match done.stdout() {
        Some(o) => Ok(text(o).trim().to_string()),
        None => Err(fail(format!(
            "the detector could not read the bootstrap's fallback map — {}",
            done.failure_report().unwrap_or_default()
        ))),
    }
}

fn exe() -> Result<String, Outcome> {
    std::env::current_exe()
        .map(|p| p.display().to_string())
        .map_err(|e| refuse(format!("cannot resolve the running binary: {}", e)))
}

// spec: installer/SPEC.md §The consumer smoke — a digest sidecar in the host hasher's own line
// shape, `<hex>  <name>`, emitted with the crate's hasher beside the bytes it names
pub(super) fn sidecar(file: &str) -> Step {
    let digest = crate::sha256::file_hex(Path::new(file)).map_err(fail)?;
    let name = Path::new(file).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    std::fs::write(format!("{}.sha256", file), format!("{}  {}\n", digest, name))
        .map_err(|e| fail(format!("could not emit the digest sidecar beside {}: {}", file, e)))
}

#[cfg(unix)]
fn drop_exec_mode(file: &str) -> Step {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(file, std::fs::Permissions::from_mode(0o644))
        .map_err(|e| fail(format!("could not plant the artifact transport's mode loss on {}: {}", file, e)))
}

#[cfg(not(unix))]
fn drop_exec_mode(_file: &str) -> Step {
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the build arm stages the binary it runs as and
// builds nothing: a producer's hand-off is adopted unchanged, else the running binary is copied,
// hashed and stripped of its mode as the transport strips it
pub(super) fn build(state: &mut Run) -> Step {
    let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN").map_err(refuse)?;
    let bin_name = bin.rsplit(['/', '\\']).next().unwrap_or(&bin).to_string();
    state.bin_name = bin_name.clone();
    state.host = host_target(state)?;
    // spec: installer/SPEC.md §The consumer smoke — the run steers its own roster at this host
    // unless the caller already set the knob, which keeps the override branch a live path
    let roster_file = match undeclared("GATE_SDK_NATIVE_TARGETS_FILE") {
        Some(f) => f,
        None => {
            let f = format!("{}/host-targets.list", state.scratch);
            std::fs::write(&f, format!("{}\n", state.host))
                .map_err(|e| refuse(format!("could not write the one-line host roster this smoke steers itself at: {}", e)))?;
            say(&format!("roster: no caller knob set, so this run is steered at {} alone", state.host));
            state.steer = Some(f.clone());
            f
        }
    };
    state.roster_file = walk::abs_against(&state.root, &roster_file);
    let roster_text = std::fs::read_to_string(&state.roster_file)
        .map_err(|e| refuse(format!("no declared target at {}: {}", roster_file, e)))?;
    let roster = crate::registry::members(&roster_text);
    if roster.is_empty() {
        return Err(refuse(format!(
            "no declared target at {} — there is no platform set to stage for.",
            roster_file
        )));
    }
    // spec: installer/SPEC.md §The consumer smoke — pack refuses a roster target no artifact
    // stands for, so one staged host binary satisfies it only while the roster is this host alone
    if roster != [state.host.clone()] {
        return Err(refuse(format!(
            "the roster declares {} and this host is {} — one staged host binary no longer \
             satisfies pack's all-targets demand. Steer this smoke's pack at the host alone with \
             GATE_SDK_NATIVE_TARGETS_FILE.",
            roster.join(" "),
            state.host
        )));
    }
    match undeclared("INSTALLER_SMOKE_ARTIFACTS_DIR") {
        Some(handed) => {
            let art = format!("{}/{}", handed.trim_end_matches(['/', '\\']), state.host);
            let staged = format!("{}/{}", art, bin_name);
            if !Path::new(&staged).is_file() || !Path::new(&format!("{}.sha256", staged)).is_file() {
                return Err(refuse(format!(
                    "{} carries no {} and .sha256 sidecar for {} — the hand-off this run was \
                     pointed at is not a producer's output for this host.",
                    handed, bin_name, state.host
                )));
            }
            state.pack_artifacts = handed.clone();
            state.handed = Some(handed);
            say(&format!(
                "adopted {} for {} from the hand-off, sidecar and all — nothing rebuilt, nothing rehashed",
                bin_name, state.host
            ));
        }
        None => {
            stage_running_binary(state, &bin_name)?;
            say(&format!(
                "staged the running {} for {} with the sidecar this arm emitted, non-executable so \
                 pack must restore the mode the transport drops",
                bin_name, state.host
            ));
        }
    }
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — a mid-run rebuild would pack a binary other than
// the one judging it, so a stale or foreign running binary refuses rather than being rebuilt
fn stage_running_binary(state: &mut Run, bin_name: &str) -> Step {
    let crate_dir = walk::knob_scalar("GATE_SDK_NATIVE_CRATE").map_err(refuse)?;
    let tree = crate::fresh::source_stamp(&walk::abs_against(&state.root, &crate_dir)).ok_or_else(|| {
        refuse(format!("git could not hash the tracked source under {} — the running binary's freshness cannot be read", crate_dir))
    })?;
    let baked = env!("CHECKWRIGHT_SOURCE_STAMP");
    if baked != tree {
        return Err(refuse(format!(
            "the running binary was built from source stamp {} and {} hashes to {} — {}.",
            baked, crate_dir, tree, REBUILD
        )));
    }
    let built_for = env!("CHECKWRIGHT_TARGET");
    if built_for != state.host {
        return Err(refuse(format!(
            "the running binary was built for {} and this host's bootstrap takes {} — hand the run \
             a producer's artifact directory for {} with INSTALLER_SMOKE_ARTIFACTS_DIR.",
            built_for, state.host, state.host
        )));
    }
    let art = format!("{}/artifacts/{}", state.scratch, state.host);
    std::fs::create_dir_all(&art).map_err(|e| refuse(format!("cannot create {}: {}", art, e)))?;
    let staged = format!("{}/{}", art, bin_name);
    std::fs::copy(exe()?, &staged).map_err(|e| fail(format!("could not stage the running binary for packing: {}", e)))?;
    sidecar(&staged)?;
    drop_exec_mode(&staged)?;
    state.pack_artifacts = format!("{}/artifacts", state.scratch);
    Ok(())
}

// spec: installer/SPEC.md §The packer — a pack spawn names both decisions: the root as its working
// directory and as `--root`, under the invoking environment, the steered roster and pack scratch
pub(super) fn pack_with(
    state: &Run,
    out: &str,
    roster: Option<&str>,
    version: &str,
    artifacts: bool,
) -> Result<proc::Merged, Outcome> {
    let mut env = vec![("INSTALLER_PACK_TMP_DIR".to_string(), state.scratch.clone())];
    if let Some(r) = roster.or(state.steer.as_deref()) {
        env.push(("GATE_SDK_NATIVE_TARGETS_FILE".to_string(), r.to_string()));
    }
    let mut args = vec!["--pack-installer", "--root", &state.root, "--version", version, "--out", out];
    if artifacts {
        args.extend(["--artifacts", state.pack_artifacts.as_str()]);
    }
    proc::run_merged_in(&programs::CHECKWRIGHT_GATES.at(exe()?), &args, &env, Some(Path::new(&state.root)))
        .map_err(refuse)
}

// spec: installer/SPEC.md §The packer — the newest reachable tag's version, else the smoke's own
fn version(root: &str) -> String {
    proc::run(&programs::GIT, &["-C", root, "describe", "--tags", "--abbrev=0"])
        .ok()
        .filter(|d| d.failure_report().is_none())
        .and_then(|d| d.stdout().map(|o| text(o).trim().trim_start_matches('v').to_string()))
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "0.0.0-smoke".to_string())
}

fn output(m: &proc::Merged) -> String {
    text(m.output())
}

// spec: installer/SPEC.md §The consumer smoke — the pack arm packs the tree once for every later
// arm, hands the tarball out where asked, and witnesses the two refusals a steered roster and a
// clean tree leave no ordinary path to reach
pub(super) fn pack(state: &mut Run) -> Step {
    state.version = version(&state.root);
    state.tarball = pack_one(state, &state.scratch, &state.version, true, "", "the pack step failed.")?;
    hand_out(state)?;
    planted_roster(state)?;
    footprint(state)
}

// spec: installer/SPEC.md §The consumer smoke — a pack the run installs from: refused at exit 2 when
// it fails, its `PACK:` line said, and exactly one tarball found in its output directory
pub(super) fn pack_one(
    state: &Run,
    out: &str,
    version: &str,
    artifacts: bool,
    which: &str,
    refusal: &str,
) -> Result<String, Outcome> {
    let packed = pack_with(state, out, None, version, artifacts)?;
    if !packed.succeeded() {
        eprintln!("{}", output(&packed));
        return Err(Outcome::Refuse(refusal.to_string()));
    }
    one_tarball(&packed, out, which)
}

// spec: installer/SPEC.md §The consumer smoke — a pack that succeeded: its `PACK:` line said, and
// exactly one tarball found in its output directory
pub(super) fn one_tarball(packed: &proc::Merged, out: &str, which: &str) -> Result<String, Outcome> {
    if let Some(line) = output(packed).lines().find(|l| l.starts_with("PACK:")) {
        say(line);
    }
    let tarballs: Vec<String> = walk::list_dir(Path::new(out))
        .map_err(refuse)?
        .into_iter()
        .filter(|(n, dir)| !dir && n.ends_with(".tgz"))
        .map(|(n, _)| format!("{}/{}", out, n))
        .collect();
    match tarballs.as_slice() {
        [tarball] => Ok(tarball.clone()),
        _ => Err(fail(format!("expected exactly one {}tarball, found {}", which, tarballs.len()))),
    }
}

// spec: installer/SPEC.md §The consumer smoke — INSTALLER_SMOKE_TARBALL_OUT receives the packed
// tarball and a digest sidecar before the scratch teardown
fn hand_out(state: &Run) -> Step {
    let Some(dir) = undeclared("INSTALLER_SMOKE_TARBALL_OUT") else {
        return Ok(());
    };
    if !Path::new(&dir).is_dir() {
        return Err(refuse(format!("tarball hand-out not a directory: {}", dir)));
    }
    let name = Path::new(&state.tarball).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let out = format!("{}/{}", dir.trim_end_matches(['/', '\\']), name);
    std::fs::copy(&state.tarball, &out)
        .map_err(|e| fail(format!("could not hand the packed tarball out to {}: {}", dir, e)))?;
    sidecar(&out)
}

// spec: installer/SPEC.md §The consumer smoke — the declared-target-with-no-artifact refusal,
// planted because steering the roster at this host removes it from every ordinary path
fn planted_roster(state: &Run) -> Step {
    let out = format!("{}/planted-pack", state.scratch);
    std::fs::create_dir_all(&out).map_err(|e| fail(format!("could not make the planted pack's output directory: {}", e)))?;
    let roster = format!("{}/planted-targets.list", state.scratch);
    let other = state.host.split_once('-').map_or(state.host.as_str(), |(_, rest)| rest);
    std::fs::write(&roster, format!("{}\nother-{}\n", state.host, other))
        .map_err(|e| fail(format!("could not write the planted roster: {}", e)))?;
    let planted = pack_with(state, &out, Some(&roster), &state.version, true)?;
    if planted.succeeded() {
        return Err(fail(
            "pack accepted a roster declaring a target the artifact directory has nothing for — a \
             broken payload packed as a narrower one",
        ));
    }
    if !output(&planted).contains("has no artifact directory") {
        eprintln!("{}", output(&planted));
        return Err(fail(
            "pack refused the planted roster, but for something other than the declared target it \
             had no artifact for",
        ));
    }
    say("pack: a declared target with no artifact directory refused, not packed narrower");
    Ok(())
}

// spec: installer/SPEC.md §The packer — the scoped refusal is witnessed by a pair of plants: one
// outside the payload's footprint packs clean, one inside refuses naming its path
fn footprint(state: &mut Run) -> Step {
    let out = format!("{}/footprint-pack", state.scratch);
    std::fs::create_dir_all(&out).map_err(|e| fail(format!("could not make the footprint witness's output directory: {}", e)))?;
    let (outside_plant, inside_plant) = (
        format!("{}/.pack-footprint-witness", state.root),
        format!("{}/installer/.pack-footprint-witness", state.root),
    );
    let outside = footprint_pack(state, &out, &outside_plant)?;
    if !outside.succeeded() {
        eprintln!("{}", output(&outside));
        return Err(fail(
            "an untracked path OUTSIDE the payload's footprint aborted the pack — the refusal is \
             still whole-tree, and a dirty path the payload neither ships nor reads still costs a \
             validate battery",
        ));
    }
    say("pack: a dirty path outside the footprint packs clean");
    let inside = footprint_pack(state, &out, &inside_plant)?;
    if inside.succeeded() {
        return Err(fail(
            "an untracked path INSIDE the payload's footprint packed clean — the narrowing dropped \
             a path the refusal must still cover",
        ));
    }
    let said = output(&inside);
    if !said.contains("the payload is assembled from are dirty") {
        eprintln!("{}", said);
        return Err(fail(
            "pack refused the footprint plant, but for something other than a dirty path inside the footprint",
        ));
    }
    if !said.contains(".pack-footprint-witness") {
        eprintln!("{}", said);
        return Err(fail(
            "the scoped refusal did not NAME the offending path, so a reader still has to re-run git status by hand",
        ));
    }
    say("pack: a dirty path inside the footprint refuses, naming the path it found");
    let _ = std::fs::remove_dir_all(&out);
    Ok(())
}

// spec: installer/SPEC.md §The packer — the plant is the one write this harness makes inside the
// worktree, so the run's teardown owns its removal while it stands
fn footprint_pack(state: &mut Run, out: &str, plant: &str) -> Result<proc::Merged, Outcome> {
    std::fs::write(plant, "").map_err(|e| fail(format!("could not plant {} for the footprint witness: {}", plant, e)))?;
    state.plant = Some(plant.to_string());
    let packed = pack_with(state, out, None, &state.version, true);
    std::fs::remove_file(plant)
        .map_err(|e| fail(format!("could not remove the footprint witness's plant at {}: {}", plant, e)))?;
    state.plant = None;
    packed
}

// spec: installer/SPEC.md §The consumer smoke — the install is from the packed tarball with
// `--offline`; the npm host is a scratch consumer of the package, so its spawn is scrubbed
pub(super) fn install(state: &mut Run) -> Step {
    let home = format!("{}/node-home", state.scratch);
    std::fs::create_dir_all(&home).map_err(|e| refuse(format!("cannot create {}: {}", home, e)))?;
    std::fs::write(
        format!("{}/package.json", home),
        "{\"name\":\"smoke-host\",\"version\":\"1.0.0\",\"private\":true}\n",
    )
    .map_err(|e| refuse(format!("cannot write the npm host's package.json: {}", e)))?;
    let args = ["install", "--offline", "--no-audit", "--no-fund", "--loglevel=error", &state.tarball];
    let done = in_consumer(&programs::NPM, &args, &[], &home)?;
    if done.failure_report().is_some() {
        let (out, err) = done.streams();
        eprintln!("{}{}", text(out), text(err));
        return Err(fail(
            "npm could not install the packed tarball offline — the package resolves something from a registry",
        ));
    }
    let entry = format!("{}/node_modules/.bin/checkwright", home);
    if !proc::is_executable(Path::new(&entry)) {
        return Err(fail("the installed package exposes no executable checkwright bin entry"));
    }
    state.cw = entry;
    state.pkg_root = format!("{}/node_modules/checkwright", home);
    state.entry = state.installed();
    let manifest = format!("{}/package.json", state.pkg_root);
    let version = std::fs::read_to_string(&manifest)
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v.get("version").and_then(|s| s.as_str()).map(str::to_string))
        .ok_or_else(|| fail(format!("the installed package's manifest names no version: {}", manifest)))?;
    let name = Path::new(&state.tarball).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    say(&format!("installed {} from {}", version, name));
    scratch_upstream(state)
}

// spec: installer/SPEC.md §The consumer smoke — one release tag a major version above the packed
// one, so it sorts above every version the upgrade arm derives
fn next_major(v: &str) -> String {
    let major = v.split(['.', '-', '+']).next().and_then(|m| m.parse::<u64>().ok()).unwrap_or(0);
    format!("{}.0.0", major + 1)
}

fn git_line(args: &[&str], input: &[u8]) -> Result<String, Outcome> {
    let done = proc::run_with_stdin(&programs::GIT, args, input).map_err(refuse)?;
    match done.stdout() {
        Some(out) => Ok(text(out).trim().to_string()),
        None => Err(refuse(format!(
            "could not build the scratch upstream: git {}: {}",
            args.join(" "),
            done.failure_report().unwrap_or_default()
        ))),
    }
}

// spec: installer/SPEC.md §The consumer smoke — the scratch upstream: a bare repository carrying the
// newer release tag, named as every scrubbed spawn's update-check upstream before the first consumer
fn scratch_upstream(state: &mut Run) -> Step {
    let bare = format!("{}/update-upstream.git", state.scratch);
    state.latest = next_major(&state.version);
    git_line(&["init", "-q", "--bare", &bare], b"")?;
    let tree = git_line(&["-C", &bare, "mktree"], b"")?;
    let id = ["-c", "user.name=installer-smoke", "-c", "user.email=installer-smoke@example.invalid"];
    let commit = git_line(&[&["-C", &bare][..], &id, &["commit-tree", &tree, "-m", "seed"]].concat(), b"")?;
    git_line(&["-C", &bare, "update-ref", &format!("refs/tags/v{}", state.latest), &commit], b"")?;
    if super::UPSTREAM.set(bare).is_err() {
        return Err(refuse("the scratch upstream was already set — the install arm ran twice in one process"));
    }
    say(&format!(
        "update check: every consumer probes a scratch upstream carrying v{}, never a live host",
        state.latest
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The consumer smoke — the scratch upstream's tag sorts above the packed
    // version and both patch hops, the fallback version included
    #[test]
    fn the_scratch_upstream_tag_is_a_major_above() {
        assert_eq!(next_major("0.9.14"), "1.0.0");
        assert_eq!(next_major("3.0.0"), "4.0.0");
        assert_eq!(next_major("0.0.0-smoke"), "1.0.0");
        let hop = super::super::upgrade::next_patch(&super::super::upgrade::next_patch("0.9.14"));
        assert!(super::super::upgrade::upgrades(&hop, &next_major("0.9.14")));
    }

    // spec: installer/SPEC.md §The consumer smoke — the sidecar is the host hasher's line shape
    // over the staged bytes, so pack's verification reads it as it reads a producer's
    #[test]
    fn the_sidecar_is_the_hasher_line_over_the_staged_bytes() {
        let dir = std::env::temp_dir().join(format!("cw-installer-smoke-sidecar-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let file = dir.join("bin");
        std::fs::write(&file, b"abc").expect("write the bytes");
        let made = sidecar(&file.display().to_string()).is_ok();
        let line = std::fs::read_to_string(dir.join("bin.sha256")).unwrap_or_default();
        let _ = std::fs::remove_dir_all(&dir);
        assert!(made, "the sidecar was not emitted");
        assert_eq!(line, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  bin\n");
    }

    // spec: installer/SPEC.md §The packer — a tree with no tag still packs, at the smoke's own
    // fallback version
    #[test]
    fn an_untagged_tree_packs_at_the_fallback_version() {
        assert_eq!(version("/nonexistent-tree-checkwright"), "0.0.0-smoke");
    }
}
