// spec: installer/SPEC.md §The consumer smoke — the artifact arm: the selection outcomes a single
// install cannot show, driven on a mutated copy of the packed payload, the shasum fallback and the
// foreign-architecture fallback cases
use super::consumer::{consumer, entry_run, extract, failed, is_file, mkdir, out, porcelain, tree, Lock};
use super::masked::untouched;
use super::profiles::assert_install;
use super::staging::fallback_of;
use super::{fail, say, Entry, Outcome, Run, Step};
use std::path::Path;

const NO_TARGET: &str = "maps to no target this payload declares";
const NO_ACTION: &str = "no adopter action to take";
const INCOMPLETE: &str = "carries no complete artifact for it";

fn arch(triple: &str) -> &str {
    triple.split('-').next().unwrap_or(triple)
}

fn init_min(state: &Run, c: &str, set: &[(String, String)]) -> Result<crate::proc::Merged, Outcome> {
    entry_run(&state.entry, c, &["init", "--profile", &state.profile_min], set)
}

// spec: installer/SPEC.md §The consumer smoke — the bootstrap's step 4 driven through `shasum -a 256`
// with sha256sum masked by absence, the farm's shasum proved on the payload's own sidecar first;
// a host with no shasum says so and skips
#[cfg(unix)]
fn shasum_leg(state: &Run, art: &str) -> Result<Option<String>, Outcome> {
    if !crate::proc::on_path(&crate::programs::SHASUM) {
        say("shasum fallback skipped: this host carries no shasum");
        return Ok(None);
    }
    let path = super::masked::path_without("sha256sum", &format!("{}/shasumfarm", state.scratch))?;
    if super::consumer::resolves("sha256sum", &path).is_some() {
        return Err(fail("the mask did not take: sha256sum still resolves under the shasum fallback's PATH"));
    }
    let set = super::consumer::path_set(&Some(path.clone()));
    let sidecar = format!("{}.sha256", state.bin_name);
    let checked = super::in_consumer(&crate::programs::SHASUM, &["-a", "256", "-c", &sidecar], &set, &format!("{}/{}", art, state.host))?;
    if checked.failure_report().is_some() {
        return Err(fail(
            "the shasum farm's shasum -a 256 -c does not accept the payload's sidecar — either the farm's shasum will not run or it cannot read the <hex>  <name> line, and the fallback below would fail for a reason that is not the bootstrap's",
        ));
    }
    let c = consumer(state, "artifact-shasum")?;
    let m = init_min(state, &c, &set)?;
    if !m.succeeded() {
        return Err(failed(&m, "init refused with sha256sum masked — the bootstrap's shasum branch did not verify an intact artifact"));
    }
    if !is_file(&format!("{}/checkwright.lock", c)) {
        return Err(fail("init with sha256sum masked exited 0 and wrote no manifest"));
    }
    say("shasum fallback: sha256sum masked, the farm's shasum checks the sidecar, and init verified and installed through it");
    Ok(Some(path))
}

#[cfg(not(unix))]
fn shasum_leg(_state: &Run, _art: &str) -> Result<Option<String>, Outcome> {
    say("shasum fallback skipped: the PowerShell bootstrap hashes in-process, so it has no shasum branch");
    Ok(None)
}

// spec: installer/SPEC.md §The consumer smoke — a payload triple directory filled from a hand-off
// triple's bytes and producer sidecar, nothing hashed here
fn place(state: &Run, art: &str, dst: &str, src: &str) -> Result<(), Outcome> {
    let handed = state.handed.clone().unwrap_or_default();
    let to = format!("{}/{}", art, dst);
    let _ = std::fs::remove_dir_all(&to);
    mkdir(&to)?;
    for leaf in [state.bin_name.clone(), format!("{}.sha256", state.bin_name)] {
        std::fs::copy(format!("{}/{}/{}", handed, src, leaf), format!("{}/{}", to, leaf))
            .map_err(|e| fail(format!("could not place the {} artifact as {}'s: {}", src, dst, e)))?;
    }
    executable(&format!("{}/{}", to, state.bin_name))
}

#[cfg(unix)]
fn executable(file: &str) -> Result<(), Outcome> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(file, std::fs::Permissions::from_mode(0o755))
        .map_err(|e| fail(format!("could not make {} executable: {}", file, e)))
}

#[cfg(not(unix))]
fn executable(_file: &str) -> Result<(), Outcome> {
    Ok(())
}

fn roster(art: &str, body: &str) -> Step {
    std::fs::write(format!("{}/targets.list", art), body).map_err(|e| fail(format!("could not write the payload roster: {}", e)))
}

pub(super) fn artifact(state: &mut Run) -> Step {
    let artp = format!("{}/artifact-pack", state.scratch);
    mkdir(&artp)?;
    extract(&state.tarball, &artp)?;
    let art = format!("{}/package/payload/artifact", artp);
    let bin = format!("{}/{}/{}", art, state.host, state.bin_name);
    if !is_file(&format!("{}/targets.list", art)) || !is_file(&bin) || !is_file(&format!("{}.sha256", bin)) {
        return Err(fail(format!(
            "the packed payload carries no complete {} artifact beside a verbatim roster copy",
            state.host
        )));
    }
    say(&format!(
        "the packed payload carries {} for {} with the sidecar the build leg emitted",
        state.bin_name, state.host
    ));
    state.entry = Entry::Extracted(format!("{}/package", artp));
    state.run_path = None;
    let c = consumer(state, "artifact")?;
    let min = state.profile_min.clone();
    assert_install(state, &min, &c)?;
    let lock = Lock::of(&c)?;
    if lock.artifact("target") != state.host {
        return Err(fail(format!(
            "init selected '{}' where the bootstrap's detector maps this host to {}",
            lock.artifact("target"),
            state.host
        )));
    }
    // spec: installer/SPEC.md §The consumer smoke — the detector's architecture held to the running
    // binary's own build triple, the one derivation independent of the detector
    let built = env!("CHECKWRIGHT_TARGET");
    if arch(built) != arch(&state.host) {
        return Err(fail(format!(
            "the bootstrap's detector maps this host to {} where the running binary was built for the {} architecture",
            state.host,
            arch(built)
        )));
    }
    let emitted = std::fs::read_to_string(format!("{}/{}/{}.sha256", state.pack_artifacts, state.host, state.bin_name))
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string();
    if lock.artifact("digest") != emitted {
        return Err(fail("the manifest records a digest other than the one this arm's build leg emitted"));
    }
    let other = state.host.split_once('-').map_or(state.host.as_str(), |(_, rest)| rest).to_string();
    roster(&art, &format!("other-{}\n", other))?;
    let nc = consumer(state, "artifact-undeclared")?;
    let seed = tree(&nc)?;
    let m = init_min(state, &nc, &[])?;
    let undeclared = out(&m);
    if m.succeeded() {
        return Err(failed(&m, "init installed on a host the payload never committed to — this platform is refused at the bootstrap rather than served an install whose battery cannot run"));
    }
    if !undeclared.contains(NO_TARGET) {
        return Err(failed(&m, "the unrostered host was refused without being told that this platform is the thing the payload carries nothing for"));
    }
    if !undeclared.contains(NO_ACTION) {
        return Err(failed(&m, "the unrostered refusal names no remedy, so an adopter cannot tell it from the broken-payload one they are supposed to act on"));
    }
    if !untouched(&nc, &seed)? {
        return Err(fail("the unrostered refusal wrote into the consumer — a refusal that wrote first is a partial install, not a refusal"));
    }
    say("host off the payload roster: refused naming the platform, no adopter action, nothing written");
    std::fs::copy(&state.roster_file, format!("{}/targets.list", art)).map_err(|e| fail(format!("could not restore the payload roster: {}", e)))?;
    let shasum = shasum_leg(state, &art)?;
    super::consumer::append(&bin, "tampered\n")?;
    let tc = consumer(state, "artifact-tampered")?;
    let seed = tree(&tc)?;
    let m = init_min(state, &tc, &[])?;
    if m.succeeded() {
        return Err(failed(&m, "init installed a gate binary whose bytes do not match the digest published beside it"));
    }
    if !untouched(&tc, &seed)? {
        return Err(fail("the digest refusal left the consumer changed — it was checked after something was written, not before"));
    }
    say("tampered artifact: refused with nothing written");
    if let Some(path) = &shasum {
        let m = init_min(state, &tc, &super::consumer::path_set(&Some(path.clone())))?;
        if m.succeeded() {
            return Err(failed(&m, "init installed a tampered gate binary with sha256sum masked — the shasum branch compared nothing"));
        }
        if !out(&m).contains("does not match its published digest") {
            return Err(failed(&m, "the tampered artifact was refused with sha256sum masked, but not by the digest comparison"));
        }
        if tree(&tc)? != seed || is_file(&format!("{}/checkwright.lock", tc)) {
            return Err(fail("the shasum branch's digest refusal left the consumer changed"));
        }
        say("tampered artifact, sha256sum masked: refused by the shasum branch's digest comparison, nothing written");
    }
    std::fs::remove_file(&bin).map_err(|e| fail(format!("could not remove the declared target's binary: {}", e)))?;
    let ac = consumer(state, "artifact-absent")?;
    let m = init_min(state, &ac, &[])?;
    let absent = out(&m);
    if m.succeeded() {
        return Err(failed(&m, format!(
            "the payload declares {} and carries no artifact for it, and init installed anyway — a broken payload read as a narrower one",
            state.host
        )));
    }
    if !absent.contains(INCOMPLETE) {
        return Err(failed(&m, "the declared-but-absent target was refused without naming the incomplete artifact as the cause"));
    }
    if is_file(&format!("{}/checkwright.lock", ac)) || !porcelain(&ac)?.is_empty() {
        return Err(fail("the broken-payload refusal still wrote into the consumer"));
    }
    say("declared target with no artifact: refused, and not with the unrostered host's answer");
    if undeclared == absent {
        return Err(failed(&m, "the unrostered host and the broken payload printed the same thing — these are two of the three selection refusals, and collapsing them tells an adopter with no action to take to go and act"));
    }
    if absent.contains(NO_ACTION) {
        return Err(failed(&m, "the broken-payload refusal wears the unrostered host's remedy — it is the one refusal an adopter CAN act on, by re-downloading"));
    }
    say("the two refusals differ in message and remedy, which is what the exit status does not carry");
    fallbacks(state, &art, &undeclared, &absent)
}

// spec: installer/SPEC.md §The consumer smoke — the fallback cases, wherever the hand-off carries a
// foreign-architecture preferred artifact beside the host's own fallback upload
fn fallbacks(state: &Run, art: &str, undeclared: &str, absent: &str) -> Step {
    let host_fallback = fallback_of(state, &state.host)?;
    let mut foreign = String::new();
    if let (Some(handed), false) = (&state.handed, host_fallback.is_empty()) {
        for (t, _) in crate::walk::list_dir(Path::new(handed)).unwrap_or_default().into_iter().filter(|(_, d)| *d) {
            let b = format!("{}/{}/{}", handed, t, state.bin_name);
            if t == state.host || !is_file(&b) || !is_file(&format!("{}.sha256", b)) {
                continue;
            }
            if !fallback_of(state, &t)?.is_empty() {
                foreign = t;
                break;
            }
        }
    }
    let handed = state.handed.clone().unwrap_or_default();
    if foreign.is_empty() || !is_file(&format!("{}/{}/{}.sha256", handed, host_fallback, state.bin_name)) {
        say("fallback cases skipped: this host's hand-off carries no foreign-architecture preferred artifact beside its own fallback upload");
        return Ok(());
    }
    place(state, art, &state.host, &foreign)?;
    // spec: installer/SPEC.md §The consumer smoke — the stand-in's premise is proved, not assumed: a
    // foreign artifact that starts here, as under emulation, is no preferred artifact that cannot
    let placed = format!("{}/{}/{}", art, state.host, state.bin_name);
    if crate::proc::run(&crate::programs::CHECKWRIGHT_GATES.at(placed), &["--help"]).is_ok() {
        say(&format!(
            "fallback cases skipped: the foreign {} artifact starts on this host, so it cannot stand in for a preferred artifact that does not run",
            foreign
        ));
        return Ok(());
    }
    place(state, art, &host_fallback, &host_fallback)?;
    roster(art, &format!("{}\n{}\n", state.host, host_fallback))?;
    let fc = consumer(state, "artifact-fallback")?;
    let m = init_min(state, &fc, &[])?;
    if !m.succeeded() {
        return Err(failed(&m, "init refused where the preferred artifact does not run and its fallback is rostered — selection did not pass to the fallback"));
    }
    let got = Lock::of(&fc)?.artifact("target");
    if got != host_fallback {
        return Err(fail(format!(
            "the preferred artifact does not run here, and init recorded '{}' rather than the fallback {}",
            got, host_fallback
        )));
    }
    say(&format!(
        "falls back: the preferred {} artifact did not run, and init installed {}",
        state.host, host_fallback
    ));
    roster(art, &format!("{}\n", host_fallback))?;
    let hc = consumer(state, "artifact-held-preferred")?;
    let m = init_min(state, &hc, &[])?;
    if !m.succeeded() {
        return Err(failed(&m, "init refused a host whose preferred triple is held and whose fallback is rostered"));
    }
    let got = Lock::of(&hc)?.artifact("target");
    if got != host_fallback {
        return Err(fail(format!(
            "with only the fallback rostered, init recorded '{}' rather than {}",
            got, host_fallback
        )));
    }
    say(&format!("held preferred: {} off the roster, and init installed {}", state.host, host_fallback));
    roster(art, &format!("{}\n", state.host))?;
    let xc = consumer(state, "artifact-no-fallback")?;
    let seed = tree(&xc)?;
    let m = init_min(state, &xc, &[])?;
    let norun = out(&m);
    if m.succeeded() {
        return Err(failed(&m, "init installed where the preferred artifact does not run and no fallback is rostered"));
    }
    if !norun.contains("does not run on this host") {
        return Err(failed(&m, "the preferred artifact that does not run was refused without saying so"));
    }
    if !norun.contains("report the host, the triple and the release") {
        return Err(failed(&m, "the does-not-run refusal names no remedy"));
    }
    if !untouched(&xc, &seed)? {
        return Err(fail("the does-not-run refusal wrote into the consumer"));
    }
    if norun == undeclared || norun == absent {
        return Err(failed(&m, "the does-not-run refusal printed what another selection refusal prints"));
    }
    if norun.contains(NO_ACTION) {
        return Err(failed(&m, "the does-not-run refusal wears the unrostered host's remedy"));
    }
    if norun.contains(INCOMPLETE) {
        return Err(failed(&m, "the does-not-run refusal wears the broken payload's message"));
    }
    say("no fallback: the preferred artifact that does not run is refused, nothing written, and not with another refusal's words");
    Ok(())
}
