// spec: installer/SPEC.md §The consumer smoke — the upgrade family: two cross-version hops carrying a
// relinquish and its re-add, the cross-version reversal over those hops, and the newer-verb reversal
use super::consumer::{
    append, consumer, entry_run, extract, failed, git, hash_in, is_file, mkdir, out, porcelain, tree, Lock,
    GATES_DIR,
};
use super::profiles::{live_members, reversal};
use super::{fail, refuse, say, Entry, Outcome, Run, Step};
use std::path::Path;

// spec: installer/SPEC.md §The consumer smoke — the adopter-edited file, and the relinquish subject:
// a minimum-profile payload file no init step and neither generated projection reads
const EDITED: &str = "gate-sdk/README.md";
const RELINQUISHED: &str = "gate-sdk/templates/check-skeleton.sh";

// spec: installer/SPEC.md §The consumer smoke — each version is the one before it at the next patch,
// its pre-release and build suffix dropped
pub(super) fn next_patch(v: &str) -> String {
    let core = v.split(['-', '+']).next().unwrap_or(v);
    let mut parts = core.split('.').map(|p| p.parse::<u64>().unwrap_or(0));
    let (a, b, c) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    format!("{}.{}.{}", a, b, c + 1)
}

// spec: installer/SPEC.md §The consumer smoke — the derived version sorts strictly above, compared
// field by field, numerically where both fields are numbers
pub(super) fn upgrades(from: &str, to: &str) -> bool {
    let key = |v: &str| -> Vec<(u64, String)> {
        v.split(['.', '-', '+'])
            .map(|f| match f.parse::<u64>() {
                Ok(n) => (n, String::new()),
                Err(_) => (u64::MAX, f.to_string()),
            })
            .collect()
    };
    from != to && key(from) < key(to)
}

// spec: installer/SPEC.md §The consumer smoke — free space and the scratch tree's size, read in
// process; it never changes a verdict and says where it has no reading
fn scratch_witness(state: &Run, moment: &str) {
    say(&format!("scratch witness, {}:", moment));
    match free_kib(&state.scratch) {
        Some((avail, total)) => say(&format!("  free: {} KiB available of {} KiB on the scratch filesystem", avail, total)),
        None => say("  free: no reading on this host"),
    }
    say(&format!("  size: {} KiB under {}", tree_bytes(Path::new(&state.scratch)) / 1024, state.scratch));
}

#[cfg(unix)]
fn free_kib(dir: &str) -> Option<(u64, u64)> {
    let path = std::ffi::CString::new(dir).ok()?;
    let mut fs: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(path.as_ptr(), &mut fs) } != 0 {
        return None;
    }
    let unit = fs.f_frsize as u64;
    Some((fs.f_bavail as u64 * unit / 1024, fs.f_blocks as u64 * unit / 1024))
}

#[cfg(not(unix))]
fn free_kib(_dir: &str) -> Option<(u64, u64)> {
    None
}

fn tree_bytes(dir: &Path) -> u64 {
    crate::walk::list_dir(dir)
        .unwrap_or_default()
        .into_iter()
        .map(|(name, is_dir)| {
            let p = dir.join(name);
            match std::fs::symlink_metadata(&p) {
                _ if is_dir => tree_bytes(&p),
                Ok(m) if m.is_file() => m.len(),
                _ => 0,
            }
        })
        .sum()
}

// spec: installer/SPEC.md §The gate binary — every cross-version pack carries the artifact
// directory the main pack used, and a failed one prints the scratch witness before it refuses
fn hop(state: &Run, dir: &str, version: &str, which: &str, moment: &str) -> Result<String, Outcome> {
    mkdir(dir)?;
    let packed = super::staging::pack_with(state, dir, None, version, true)?;
    if !packed.succeeded() {
        super::show(&out(&packed));
        scratch_witness(state, &format!("the {} pack step failed", moment));
        return Err(refuse(format!("the {} pack step failed.", moment)));
    }
    let tarball = super::staging::one_tarball(&packed, dir, which)?;
    extract(&tarball, dir)?;
    Ok(format!("{}/package/bin/checkwright.sh", dir))
}

fn cw(state: &Run) -> Entry {
    Entry::Bin(state.cw.clone())
}

fn bootstrap(dir: &str) -> Entry {
    Entry::Sh(format!("{}/package/bin/checkwright.sh", dir))
}

// spec: installer/SPEC.md §The consumer smoke — the upgrade arm: a minimum install with two committed
// adopter edits carried across two hops, the first relinquishing a path its payload stopped
// shipping and the second re-adding it
pub(super) fn upgrade(state: &mut Run) -> Step {
    state.up_version = next_patch(&state.version);
    if !upgrades(&state.version, &state.up_version) {
        return Err(fail(format!(
            "the arm derived {} from {}, which is not the upgrade direction — it would assert the downgrade refusal instead",
            state.up_version, state.version
        )));
    }
    state.up = format!("{}/upgrade", state.scratch);
    scratch_witness(state, "before the first upgrade pack");
    hop(state, &state.up, &state.up_version, "upgrade ", "upgrade")?;
    let c = consumer(state, "upgrade")?;
    let min = state.profile_min.clone();
    let m = entry_run(&cw(state), &c, &["init", "--profile", &min], &[])?;
    if !m.succeeded() {
        return Err(failed(&m, "the upgrade arm's starting install failed"));
    }
    let lock = Lock::of(&c)?;
    if lock.top("version") != state.version {
        return Err(fail(format!("the upgrade arm did not start at {}", state.version)));
    }
    let was_kits = lock.list("kits").join(" ");
    say(&format!("installed {} at the {} profile ({})", state.version, min, was_kits));
    if !lock.has_file(EDITED) {
        return Err(fail(format!(
            "the {} manifest does not record {} — the arm has nothing whose adopter edit it can assert",
            min, EDITED
        )));
    }
    if !lock.has_file(RELINQUISHED) {
        return Err(fail(format!(
            "the {} manifest does not record {} — the relinquish arm has no subject",
            min, RELINQUISHED
        )));
    }
    let r_init = lock.file(RELINQUISHED);
    append(&format!("{}/{}", c, EDITED), "\nAn adopter edited this line.\n")?;
    append(&format!("{}/{}", c, RELINQUISHED), "\n# An adopter edited this line.\n")?;
    let (edited_want, r_want) = (hash_in(&c, EDITED)?, hash_in(&c, RELINQUISHED)?);
    let added = git(&c, &["add", "--", EDITED, RELINQUISHED])?;
    if !added.succeeded() || !git(&c, &["commit", "-q", "-m", "edit two vendored files"])?.succeeded() {
        return Err(fail("could not commit the adopter edits in the scratch consumer"));
    }
    let dropped = format!("{}/package/payload/{}", state.up, RELINQUISHED);
    let _ = std::fs::remove_file(&dropped);
    if is_file(&dropped) {
        return Err(fail(format!(
            "the upgrade payload still ships {} — the relinquish hop would assert nothing",
            RELINQUISHED
        )));
    }
    let m = entry_run(&bootstrap(&state.up), &c, &["init"], &[])?;
    if !m.succeeded() {
        return Err(failed(&m, "the cross-version re-run of init failed — the version check did not fall through in the upgrade direction"));
    }
    let said = out(&m);
    let lock = Lock::of(&c)?;
    if lock.top("version") != state.up_version {
        return Err(fail(format!(
            "the manifest records {} after upgrading to {}",
            lock.top("version"),
            state.up_version
        )));
    }
    if lock.top("profile") != min {
        return Err(fail(format!(
            "the upgrade was run with no --profile and did not re-read {} from the manifest",
            min
        )));
    }
    if lock.list("kits").join(" ") != was_kits {
        return Err(fail(format!(
            "the upgrade changed the recorded kit set from '{}' to '{}'",
            was_kits,
            lock.list("kits").join(" ")
        )));
    }
    if hash_in(&c, EDITED)? != edited_want {
        return Err(fail(format!("the upgrade overwrote {}, which the adopter had changed since init wrote it", EDITED)));
    }
    if !said.contains(EDITED) {
        return Err(failed(&m, format!("the upgrade left {} alone but did not report it as changed", EDITED)));
    }
    let live = live_members(&format!("{}/{}/gates.list", c, GATES_DIR)).len();
    if live == 0 || lock.artifact("target").is_empty() {
        return Err(fail(format!(
            "the upgrade hop's {} install left {} live registry member(s) and {} artifact, so the clean-worktree assertion below holds over a hop that rewrote nothing — repair the hop, never drop the assertion",
            min,
            live,
            if lock.has("artifact") { "an" } else { "no" }
        )));
    }
    say(&format!(
        "tripwire: {} live member(s) and a placed artifact on this hop, so the clean-worktree assertion has something to be about",
        live
    ));
    if !porcelain(&c)?.is_empty() {
        return Err(fail("the upgrade left the worktree dirty"));
    }
    if !lock.has_file(EDITED) {
        return Err(fail(format!(
            "the upgrade dropped {} from the manifest roster — the next run would read its absence as 'never installed'",
            EDITED
        )));
    }
    if lock.file(EDITED) == edited_want {
        return Err(fail(format!(
            "the upgrade recorded the adopter's own hash for {} — the next run would find it unchanged and claim it",
            EDITED
        )));
    }
    if hash_in(&c, RELINQUISHED)? != r_want {
        return Err(fail(format!("the upgrade touched {}, which its payload no longer ships", RELINQUISHED)));
    }
    if !lock.has_file(RELINQUISHED) {
        return Err(fail(format!(
            "the upgrade disowned {} because its payload stopped shipping it — the next release to re-add the path would write straight through the adopter's edits",
            RELINQUISHED
        )));
    }
    if lock.file(RELINQUISHED) != r_init {
        return Err(fail(format!(
            "the upgrade kept {} on the roster at a hash other than the one init wrote there",
            RELINQUISHED
        )));
    }
    say(&format!(
        "upgrade: {} -> {}, profile re-read, {} preserved and reported, {} relinquished and still owned",
        state.version, state.up_version, EDITED, RELINQUISHED
    ));
    state.up2_version = next_patch(&state.up_version);
    if !upgrades(&state.up_version, &state.up2_version) {
        return Err(fail(format!(
            "the arm derived {} from {}, which is not the upgrade direction — it would assert the downgrade refusal instead",
            state.up2_version, state.up_version
        )));
    }
    state.up2 = format!("{}/upgrade2", state.scratch);
    hop(state, &state.up2, &state.up2_version, "second-upgrade ", "second upgrade")?;
    let m = entry_run(&bootstrap(&state.up2), &c, &["init"], &[])?;
    if !m.succeeded() {
        return Err(failed(&m, "the second cross-version re-run of init failed"));
    }
    let said = out(&m);
    let lock = Lock::of(&c)?;
    if lock.top("version") != state.up2_version {
        return Err(fail(format!(
            "the manifest records {} after upgrading to {}",
            lock.top("version"),
            state.up2_version
        )));
    }
    if hash_in(&c, EDITED)? != edited_want {
        return Err(fail(format!(
            "the second upgrade overwrote {} — the protection lasted one upgrade and then inverted",
            EDITED
        )));
    }
    if !said.contains(EDITED) {
        return Err(failed(&m, format!("the second upgrade left {} alone but did not report it as changed", EDITED)));
    }
    if hash_in(&c, RELINQUISHED)? != r_want {
        return Err(fail(format!(
            "the re-adding payload overwrote {} — the roster did not carry the ownership across the relinquish",
            RELINQUISHED
        )));
    }
    if !said.contains(RELINQUISHED) {
        return Err(failed(&m, format!(
            "the re-adding payload left {} alone but did not report it as changed",
            RELINQUISHED
        )));
    }
    if lock.file(RELINQUISHED) != r_init {
        return Err(fail(format!(
            "the re-adding payload recorded {} at a hash other than the one init wrote there",
            RELINQUISHED
        )));
    }
    if !porcelain(&c)?.is_empty() {
        return Err(fail("the second upgrade left the worktree dirty"));
    }
    say(&format!(
        "second upgrade: {} -> {}, {} still the adopter's and still reported, {} re-added and refused",
        state.up_version, state.up2_version, EDITED, RELINQUISHED
    ));
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — an unedited minimum install carried across the same
// three versions and reversed by the latest verb back to its pre-init tree object
pub(super) fn cross_version_reversal(state: &mut Run) -> Step {
    let c = consumer(state, "cross-version-reversal")?;
    let seed = tree(&c)?;
    let min = state.profile_min.clone();
    let m = entry_run(&cw(state), &c, &["init", "--profile", &min], &[])?;
    if !m.succeeded() {
        return Err(failed(&m, "the cross-version reversal arm's starting install failed"));
    }
    if is_file(&format!("{}/package/payload/{}", state.up, RELINQUISHED)) {
        return Err(fail(format!(
            "the upgrade package still ships {}, so this arm's three hops carry identical payloads and its reversal is an ordinary install wearing a cross-version name — keep the arm below the relinquish that the upgrade arm performs, never drop the assertion",
            RELINQUISHED
        )));
    }
    for (dir, which) in [(&state.up, "relinquishing"), (&state.up2, "re-adding")] {
        let m = entry_run(&bootstrap(dir), &c, &["init"], &[])?;
        if !m.succeeded() {
            return Err(failed(&m, format!("the cross-version reversal arm's {} hop failed", which)));
        }
    }
    let lock = Lock::of(&c)?;
    if lock.top("version") != state.up2_version {
        return Err(fail(format!(
            "the cross-version reversal consumer records {} rather than {}, so it fell through a hop and the reversal runs on a shorter history than the arm claims — repair the hop, never drop the assertion",
            lock.top("version"),
            state.up2_version
        )));
    }
    if !lock.has_file(RELINQUISHED) {
        return Err(fail(format!(
            "the cross-version reversal consumer's roster lost {} across the payload hole, so uninstall has nothing cross-version to clear and this arm reverses an ordinary install — re-scope the relinquish subject onto a path the {} profile records, never drop the assertion",
            RELINQUISHED, min
        )));
    }
    say(&format!(
        "premise: three hops landed at {}, the relinquished path crossed the payload hole and is on the roster",
        state.up2_version
    ));
    state.entry = bootstrap(&state.up2);
    state.run_path = None;
    reversal(state, &min, &c, &seed)?;
    say(&format!(
        "cross-version reversal: {} -> {} -> {} reversed to the pre-init tree object",
        state.version, state.up_version, state.up2_version
    ));
    Ok(())
}

// spec: installer/SPEC.md §uninstall — a first-version install reversed by the next version's
// uninstall with no upgrade between, its payload lacking the relinquished path
pub(super) fn newer_verb_reversal(state: &mut Run) -> Step {
    let c = consumer(state, "newer-verb-reversal")?;
    let seed = tree(&c)?;
    let min = state.profile_min.clone();
    let m = entry_run(&cw(state), &c, &["init", "--profile", &min], &[])?;
    if !m.succeeded() {
        return Err(failed(&m, "the newer-verb reversal arm's install failed"));
    }
    let lock = Lock::of(&c)?;
    if lock.top("version") != state.version {
        return Err(fail(format!(
            "the newer-verb reversal consumer records {} rather than {}, so a later verb would reverse a same-version install",
            lock.top("version"),
            state.version
        )));
    }
    if !lock.has_file(RELINQUISHED) || is_file(&format!("{}/package/payload/{}", state.up, RELINQUISHED)) {
        return Err(fail(format!(
            "the newer-verb reversal arm's premise fails: the install must record {} and the {} payload must lack it",
            RELINQUISHED, state.up_version
        )));
    }
    let m = entry_run(&bootstrap(&state.up), &c, &["uninstall"], &[])?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "the {} uninstall exited {} against a {} install",
            state.up_version,
            m.reported_code(),
            state.version
        )));
    }
    if tree(&c)? != seed {
        return Err(failed(&m, format!(
            "the {} uninstall did not bring a {} install back to its pre-init tree object",
            state.up_version, state.version
        )));
    }
    if !porcelain(&c)?.is_empty() || is_file(&format!("{}/checkwright.lock", c)) {
        return Err(fail(format!(
            "the {} uninstall left the worktree dirty or a manifest behind",
            state.up_version
        )));
    }
    say(&format!(
        "newer-verb reversal: a {} install reversed by the {} uninstall to the pre-init tree object",
        state.version, state.up_version
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The consumer smoke — the next patch drops a suffix, and the direction
    // test is numeric rather than lexical
    #[test]
    fn each_hop_is_the_next_patch_and_strictly_above() {
        assert_eq!(next_patch("0.31.0"), "0.31.1");
        assert_eq!(next_patch("0.0.0-smoke"), "0.0.1");
        assert!(upgrades("0.31.9", "0.31.10"));
        assert!(!upgrades("0.31.1", "0.31.1"));
        assert!(!upgrades("0.31.1", "0.31.0"));
    }
}
