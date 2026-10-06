// spec: installer/SPEC.md §The consumer smoke — the moves an install makes: the hooked profile move
// and upgrade, the same-version seam arm with its protection branch, the narrowing arm and the
// selection arm
use super::consumer::{
    append, commits, consumer, copy_tree, entry_run, failed, find_line, first_starting, git, git_out, hash_in, is_file,
    out, porcelain, seam_of, sorted, tree, write, Lock, GATES_DIR, PROFILE_DERIVED,
};
use super::profiles::{commit_all, kit_roots, live_members, profile_kits};
use super::{fail, lines, merged_in, say, Entry, Outcome, Run, Step};
use crate::{programs, walk};
use std::path::Path;

fn cw(state: &Run, c: &str, args: &[&str]) -> Result<crate::proc::Merged, Outcome> {
    entry_run(&state.installed(), c, args, &[])
}

fn init_ok(state: &Run, c: &str, args: &[&str], why: &str) -> Result<String, Outcome> {
    let m = cw(state, c, args)?;
    if !m.succeeded() {
        return Err(failed(&m, why));
    }
    Ok(out(&m))
}

fn gate_bin(c: &str) -> Option<String> {
    let bin = seam_of(c)?;
    let path = format!("{}/{}", c, bin);
    crate::proc::is_executable(Path::new(&path)).then_some(path)
}

// spec: installer/SPEC.md §The consumer smoke — a lattice-minimum install with its hooks on, moved
// to the maximum and then across a version whose payload changes the gate library, each move one
// commit through the hooks, and a co-staged gate edit still refused
pub(super) fn hooked_move(state: &mut Run) -> Step {
    let hc = consumer(state, "hooked-move")?;
    let min = state.profile_min.clone();
    init_ok(state, &hc, &["init", "--profile", &min], "the hooked move arm's starting install failed")?;
    let Some(bin) = gate_bin(&hc) else {
        return Err(fail(format!(
            "the hooked move arm's consumer names no gate binary at '{}', so it has no hooks to install",
            seam_of(&hc).unwrap_or_else(|| "<unset>".to_string())
        )));
    };
    let m = merged_in(&programs::CHECKWRIGHT_GATES.at(bin), &["--install-hooks"], &[], &hc)?;
    if !m.succeeded() {
        return Err(failed(&m, "the hooked move arm could not install the hooks"));
    }
    let hooks = git_out(&hc, &["config", "core.hooksPath"]).unwrap_or_default().trim().to_string();
    let placed = Path::new(&hooks).join(crate::emit::hook_launcher::file_name("pre-commit"));
    if hooks.is_empty() || !crate::proc::is_executable(&placed) {
        return Err(fail(
            "the hooked move arm's consumer has no executable pre-commit hook after --install-hooks, so its commits below are judged by nothing",
        ));
    }
    let before = commits(&hc)?;
    init_ok(
        state,
        &hc,
        &["init", "--profile", PROFILE_DERIVED],
        &format!(
            "init --profile {} over a hooked {} install exited non-zero — the move's own commit was refused by a gate it vendors",
            PROFILE_DERIVED, min
        ),
    )?;
    let list = std::fs::read_to_string(format!("{}/{}/gates.list", hc, GATES_DIR)).unwrap_or_default();
    if !list.lines().any(|l| l == "check-gate-tamper") {
        return Err(fail(format!(
            "the {} registry does not carry check-gate-tamper, so this arm's hooked commits meet no gate that judges a gate file co-staged with the manifest — re-scope the arm onto a profile that registers it, never drop the assertion",
            PROFILE_DERIVED
        )));
    }
    let dirty = porcelain(&hc)?;
    if commits(&hc)? != before + 1 || !dirty.is_empty() {
        super::show(&dirty);
        return Err(fail("the hooked profile move did not land as one commit leaving the worktree clean"));
    }
    let profile = Lock::of(&hc)?.top("profile");
    if profile != PROFILE_DERIVED {
        return Err(fail(format!(
            "the hooked profile move committed, but the manifest records {} rather than {}",
            profile, PROFILE_DERIVED
        )));
    }
    say(&format!("profile move: {} -> {} committed through the hooks in one commit", min, PROFILE_DERIVED));
    let hup = format!("{}/hooked-upgrade", state.scratch);
    copy_tree(Path::new(&format!("{}/package", state.up)), Path::new(&format!("{}/package", hup)))
        .map_err(|e| fail(format!("could not copy the upgrade package for the hooked hop: {}", e)))?;
    let lib = format!("{}/package/payload/gate-sdk/lib/gate.sh", hup);
    if !is_file(&lib) {
        return Err(fail(
            "the upgrade payload carries no gate-sdk/lib/gate.sh, so the hooked hop has no gate file to ship a change to",
        ));
    }
    append(&lib, "# A gate-library change this release ships.\n")?;
    let m = entry_run(&Entry::Extracted(format!("{}/package", hup)), &hc, &["init"], &[])?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "the hooked upgrade to {} exited non-zero — the upgrade's own commit was refused by a gate it vendors",
            state.up_version
        )));
    }
    let version = Lock::of(&hc)?.top("version");
    if version != state.up_version {
        return Err(fail(format!(
            "the hooked upgrade left the manifest at {} rather than {}",
            version, state.up_version
        )));
    }
    let staged = git_out(&hc, &["show", "--name-only", "--format=", "HEAD"])?;
    let names = lines::of("the hooked upgrade's commit", staged.as_bytes());
    if !names.iter().any(|n| n == "gate-sdk/lib/gate.sh") || !names.iter().any(|n| n == "checkwright.lock") {
        super::show(&staged);
        return Err(fail(
            "the hooked upgrade's commit does not carry both the gate library and the manifest, so it never staged the shape this arm is about — repair the hop, never drop the assertion",
        ));
    }
    if !porcelain(&hc)?.is_empty() {
        return Err(fail("the hooked upgrade left the worktree dirty"));
    }
    say(&format!(
        "upgrade: {} -> {} committed through the hooks, the gate library and the manifest in one commit",
        state.version, state.up_version
    ));
    append(&format!("{}/gate-sdk/lib/gate.sh", hc), "# An edit riding with product code.\n")?;
    write(&format!("{}/hooked-product.txt", hc), "product code\n")?;
    git(&hc, &["add", "gate-sdk/lib/gate.sh", "hooked-product.txt"])?;
    let m = git(&hc, &["commit", "-m", "chore: ride a gate edit on product code"])?;
    if m.succeeded() {
        return Err(failed(&m, "a gate edit co-staged with product code landed through the hooked consumer's hooks"));
    }
    if find_line(&out(&m), |l| l == "pre-commit: check-gate-tamper failed (see above).").is_none() {
        return Err(failed(&m, "the co-staged gate edit was refused, but not by check-gate-tamper"));
    }
    git(&hc, &["reset", "-q", "--hard", "HEAD"])?;
    say("control: a gate edit co-staged with product code is still refused by check-gate-tamper");
    Ok(())
}

// spec: installer/SPEC.md §What init seeds — the two surfaces init rewrites on every run, edited by
// the adopter and kept through a same-version re-run, then the protection branch over a tampered
// gate binary
pub(super) fn seam(state: &mut Run) -> Step {
    let sc = consumer(state, "seam")?;
    let said = init_ok(state, &sc, &["init", "--profile", PROFILE_DERIVED], "the seam arm's install failed")?;
    say(&format!("init: {}", first_starting(&said, "INIT:")));
    let edited = [format!("{}/queue-config.knobs", GATES_DIR), format!("{}/msg-patterns.list", GATES_DIR)];
    let lock = Lock::of(&sc)?;
    let mut init_hash = Vec::new();
    let mut want = Vec::new();
    for f in &edited {
        if !lock.has_file(f) {
            return Err(fail(format!(
                "the {} manifest does not record {} — the seam arm has nothing whose adopter edit it can assert",
                PROFILE_DERIVED, f
            )));
        }
        init_hash.push(lock.file(f));
        append(&format!("{}/{}", sc, f), "\n# An adopter edited this line.\n")?;
        want.push(hash_in(&sc, f)?);
    }
    let added = git(&sc, &["add", "--", &edited[0], &edited[1]])?;
    if !added.succeeded() || !git(&sc, &["commit", "-q", "-m", "edit the seam surfaces"])?.succeeded() {
        return Err(fail("could not commit the adopter's seam edits in the scratch consumer"));
    }
    let m = cw(state, &sc, &["init"])?;
    if !m.succeeded() {
        return Err(failed(&m, "the seam arm's same-version re-run of init failed"));
    }
    let lock = Lock::of(&sc)?;
    for (i, f) in edited.iter().enumerate() {
        if hash_in(&sc, f)? != want[i] {
            return Err(fail(format!(
                "the re-run overwrote {} — it is copied into the consumer outside claim(), so the comparison ran against the copy rather than the adopter's content",
                f
            )));
        }
        if !out(&m).contains(f.as_str()) {
            return Err(failed(&m, format!("the re-run left {} alone but did not report it as changed", f)));
        }
        if lock.file(f) != init_hash[i] {
            return Err(fail(format!("the re-run recorded {} at a hash other than the one init wrote there", f)));
        }
    }
    if !porcelain(&sc)?.is_empty() {
        return Err(fail("the seam arm's re-run left the worktree dirty"));
    }
    say(&format!("seam: {} preserved, reported and still recorded at init's hash", edited.join(" ")));
    let bin = seam_of(&sc).unwrap_or_default();
    if bin.is_empty() || !is_file(&format!("{}/{}", sc, bin)) {
        return Err(fail(format!(
            "the seam arm's consumer names no gate binary at '{}', so the protection branch has no artifact row to tamper",
            if bin.is_empty() { "<unset>" } else { &bin }
        )));
    }
    append(&format!("{}/{}", sc, bin), "x")?;
    let added = git(&sc, &["add", "--", &bin])?;
    if !added.succeeded() || !git(&sc, &["commit", "-q", "-m", "tamper the gate binary"])?.succeeded() {
        return Err(fail("could not commit the tampered gate binary in the scratch consumer"));
    }
    protection(state, &sc, &edited, &init_hash, &want, &bin)
}

// spec: installer/SPEC.md §The consumer smoke — the protection branch: diff exits 1 naming both
// edits, uninstall keeps and reports them and removes every other recorded file, the tampered
// binary and a planted update-check cache included, leaving the residual manifest
fn protection(state: &Run, sc: &str, edited: &[String], init_hash: &[String], want: &[String], bin: &str) -> Step {
    let roster = Lock::of(sc)?.keys();
    if roster.len() <= edited.len() {
        return Err(fail(format!(
            "the seam manifest records {} file(s), so the protection chain has nothing whose removal it can assert beside the two it keeps",
            roster.len()
        )));
    }
    let m = cw(state, sc, &["diff"])?;
    if m.code() != Some(1) {
        return Err(failed(&m, format!(
            "diff exited {} on a consumer carrying two adopter-edited vendored files — the drift verdict is the exit status, and 1 is what a CI step gating on a pristine vendored tree reads",
            m.reported_code()
        )));
    }
    if let Some(f) = edited.iter().find(|f| !out(&m).contains(f.as_str())) {
        return Err(failed(&m, format!("diff reported drift without naming {}", f)));
    }
    say(&format!("diff: {}, naming {}", first_starting(&out(&m), "DIFF:"), edited.join(" ")));
    let cache = format!("{}/.git/{}", sc, crate::emit::update_notice::CACHE_FILE);
    std::fs::write(&cache, "0 - planted\n").map_err(|e| fail(format!("could not plant the update check's cache: {}", e)))?;
    let m = cw(state, sc, &["uninstall"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("uninstall exited {} on the seam arm's consumer", m.reported_code())));
    }
    if Path::new(&cache).exists() {
        return Err(fail(
            "uninstall left the update check's cache in the clone's git directory — the manifest it answered for carries no version any more",
        ));
    }
    for (i, f) in edited.iter().enumerate() {
        if hash_in(sc, f)? != want[i] {
            return Err(fail(format!(
                "uninstall removed or rewrote {}, which the adopter had changed since init wrote it",
                f
            )));
        }
        if !out(&m).contains(f.as_str()) {
            return Err(failed(&m, format!("uninstall kept {} but did not report it", f)));
        }
    }
    if let Some(f) = roster.iter().filter(|f| !edited.contains(f)).find(|f| Path::new(&format!("{}/{}", sc, f)).exists()) {
        return Err(fail(format!(
            "uninstall left {} on the tree, which the adopter never touched — the removal stopped short of the roster it was given",
            f
        )));
    }
    if Path::new(&format!("{}/{}", sc, bin)).exists() {
        return Err(fail(format!(
            "uninstall left the tampered gate binary {} on the tree — the binary is never the adopter's, so a hash mismatch must not keep it",
            bin
        )));
    }
    if find_line(&out(&m), |l| l == format!("  {}", bin)).is_some() {
        return Err(failed(&m, format!("uninstall reported the tampered gate binary {} among the files it kept", bin)));
    }
    let lock_path = format!("{}/checkwright.lock", sc);
    if !is_file(&lock_path) {
        return Err(fail(format!(
            "uninstall kept {} file(s) and deleted the manifest — the next init would read them as never installed and write straight through them",
            edited.len()
        )));
    }
    let lock = Lock::read(&lock_path)?;
    let top: Vec<String> = lock.value().as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default();
    if top.join(" ") != "files schema" {
        return Err(fail(format!(
            "the residual manifest carries [{}] where an install that no longer exists may assert only its schema and the files it still owns",
            top.join(" ")
        )));
    }
    let survivors = sorted(edited.to_vec());
    if lock.keys() != survivors {
        return Err(fail(format!(
            "the residual roster is [{}] where the survivors are [{}]",
            lock.keys().join(" "),
            survivors.join(" ")
        )));
    }
    for (i, f) in edited.iter().enumerate() {
        if lock.file(f) != init_hash[i] {
            return Err(fail(format!(
                "the residual manifest records {} at a hash other than the one init wrote there — the next init would find it unchanged and claim it",
                f
            )));
        }
    }
    if lock.has("artifact") {
        return Err(fail("the residual manifest carries an artifact key — an omitted field leaves the key absent, never null"));
    }
    let rendered = serde_json::to_string_pretty(lock.value()).unwrap_or_default() + "\n";
    let sorted_lines = lines::of("the residual manifest's recursive sort", rendered.as_bytes());
    let stored = lines::of(
        "the residual manifest as the crate wrote it",
        &std::fs::read(&lock_path).unwrap_or_default(),
    );
    if sorted_lines.len() != stored.len() {
        return Err(fail(format!(
            "the residual manifest re-renders to {} line(s) where the stored file carries {}",
            sorted_lines.len(),
            stored.len()
        )));
    }
    if sorted_lines != stored {
        return Err(fail(
            "the residual manifest does not match its own recursive sort — the one writer of the wire shape emitted an order its second writer could not reproduce",
        ));
    }
    say(&format!(
        "protection: {} kept and reported, {} recorded file(s) removed, manifest narrowed to schema + the survivors at init's hashes",
        edited.join(" "),
        roster.len() - edited.len()
    ));
    Ok(())
}

// spec: installer/SPEC.md §The manifest — the narrowing arm: a maximum install re-run at the
// minimum, so files[] outlives kits, every recorded seam path still resolving to the consumer's own
// on the narrowed and the kits-stripped residual shape
pub(super) fn narrowing(state: &mut Run) -> Step {
    let nc = consumer(state, "narrowing")?;
    init_ok(state, &nc, &["init", "--profile", PROFILE_DERIVED], "the narrowing arm's wide install failed")?;
    let wide = Lock::of(&nc)?.list("kits").len();
    let min = state.profile_min.clone();
    init_ok(state, &nc, &["init", "--profile", &min], "the narrowing re-run failed")?;
    let lock = Lock::of(&nc)?;
    let narrow = lock.list("kits").len();
    if narrow >= wide {
        return Err(fail(format!(
            "the re-run recorded {} kit(s) where the wide install recorded {} — the arm did not narrow anything",
            narrow, wide
        )));
    }
    let mut residual = lock.value().clone();
    if let Some(o) = residual.as_object_mut() {
        o.remove("kits");
    }
    let residual_has = |f: &str| residual.get("files").and_then(|x| x.get(f)).is_some();
    let seam_files = [format!("{}/gates.list", GATES_DIR), format!("{}/gate-sdk-config.knobs", GATES_DIR)];
    let keys = lock.keys();
    let (mut checked, mut shadowed) = (0usize, 0usize);
    for f in &seam_files {
        if !lock.has_file(f) {
            continue;
        }
        checked += 1;
        let base = f.rsplit('/').next().unwrap_or(f);
        if keys.iter().any(|k| k.ends_with(&format!("/{}", base)) && k != f) {
            shadowed += 1;
        }
        if !residual_has(f) {
            return Err(fail(format!(
                "on a manifest carrying no kits, the consumer's own {} resolves to '<nothing>' — the residual shape has no kit set to exclude anything with",
                f
            )));
        }
    }
    if checked == 0 {
        return Err(fail(format!(
            "the narrowed manifest records none of [{}] — the arm has no seam path to resolve",
            seam_files.join(" ")
        )));
    }
    if shadowed == 0 {
        println!("  narrowed manifest: {} key(s), kits {} -> {}", keys.len(), wide, narrow);
        for f in &seam_files {
            let base = f.rsplit('/').next().unwrap_or(f);
            println!("  recorded {:<28} {}", f, lock.has_file(f));
            let sharing: Vec<&String> = keys.iter().filter(|k| k.ends_with(&format!("/{}", base))).collect();
            println!("  keys sharing its basename: {}", sharing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" "));
        }
        println!("  a sample of the roster, to show the separator and depth it actually carries:");
        for k in keys.iter().take(5) {
            println!("    {}", k);
        }
        return Err(fail("no vendored fixture shadows any checked seam basename — the arm would pass without asserting"));
    }
    let m = cw(state, &nc, &["doctor"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("doctor exited {} on the narrowed consumer", m.reported_code())));
    }
    let registry = format!("  registry     {}/gates.list", GATES_DIR);
    if find_line(&out(&m), |l| l == registry).is_none() {
        return Err(failed(&m, format!(
            "doctor did not name the consumer's own {}/gates.list as the registry it inspected",
            GATES_DIR
        )));
    }
    say(&format!(
        "narrowing: {} kit(s) -> {}, {} recorded seam path(s) ({} shadowed) still resolve to the consumer's own on the narrowed and the residual shape, doctor names the registry",
        wide, narrow, checked, shadowed
    ));
    Ok(())
}

// spec: installer/SPEC.md §Selecting kits and gates — the on-surface member a payload kit outside
// the minimum ships, read off each gate's `# install:` line
fn on_surface(state: &Run, min_kits: &[String]) -> Option<(String, String)> {
    for k in state.payload_kits.iter().filter(|k| !min_kits.contains(k)) {
        let checks = format!("{}/payload/{}/checks", state.pkg_root, k);
        let names: Vec<String> = walk::list_dir(Path::new(&checks))
            .unwrap_or_default()
            .into_iter()
            .filter(|(n, dir)| !dir && n.starts_with("check-"))
            .map(|(n, _)| n)
            .collect();
        let ordered = names.iter().filter(|n| n.ends_with(".sh")).chain(names.iter().filter(|n| n.ends_with(".gate")));
        for n in ordered {
            let body = std::fs::read_to_string(format!("{}/{}", checks, n)).unwrap_or_default();
            let declares = body.lines().any(|l| {
                l.strip_prefix("# install:")
                    .filter(|r| r.starts_with([' ', '\t']))
                    .map(|r| r.trim_start())
                    .and_then(|r| r.strip_prefix("on-surface"))
                    .is_some_and(|r| r.is_empty() || r.starts_with([' ', '\t']))
            });
            if declares {
                let stem = n.rsplit_once('.').map_or(n.as_str(), |(s, _)| s);
                return Some((k.clone(), stem.to_string()));
            }
        }
    }
    None
}

// spec: installer/SPEC.md §Selecting kits and gates — the selection arm: a kit added, a gate dropped
// and one added at the minimum, recorded, kept by a bare re-run, cleared by --no-selection, and
// three unhonourable selections refused writing nothing
pub(super) fn selection(state: &mut Run) -> Step {
    let min = state.profile_min.clone();
    let min_kits = profile_kits(state, &min);
    let Some((kit, gate)) = on_surface(state, &min_kits) else {
        return Err(fail(format!(
            "selection arm: no payload kit outside {}'s set ships an on-surface member, so the arm has no gate to add",
            min
        )));
    };
    let min_registry = state.registry.get(&min).cloned().unwrap_or_default();
    let Some(drop) = min_registry.first().cloned() else {
        return Err(fail(format!("selection arm: the {} install recorded no registry member to drop", min)));
    };
    let sc = consumer(state, "selection")?;
    let before = commits(&sc)?;
    init_ok(
        state,
        &sc,
        &["init", "--profile", &min, "--with-kit", &kit, "--without-gate", &drop, "--with-gate", &gate],
        "selection arm: init with a selection failed",
    )?;
    if commits(&sc)? != before + 1 {
        return Err(fail("selection arm: the selected install did not make exactly one commit"));
    }
    let lock = Lock::of(&sc)?;
    let kits = lock.list("kits");
    let want_kits = sorted(min_kits.iter().cloned().chain([kit.clone()]).collect());
    if sorted(kits.clone()) != want_kits {
        return Err(fail(format!(
            "selection arm: the manifest records kits [{}] where the selection is [{} {}]",
            kits.join(" "),
            min_kits.join(" "),
            kit
        )));
    }
    let list = format!("{}/{}/gates.list", sc, GATES_DIR);
    let registered = std::fs::read_to_string(&list).unwrap_or_default();
    if registered.lines().any(|l| l == drop) {
        return Err(fail(format!("selection arm: the registry still carries {} after --without-gate", drop)));
    }
    if !registered.lines().any(|l| l == gate) {
        return Err(fail(format!("selection arm: the registry does not carry {} after --with-gate", gate)));
    }
    let want = serde_json::json!({"with-gates": [gate], "with-kits": [kit], "without-gates": [drop]});
    let got = lock.value().get("selection").cloned().unwrap_or_default();
    if got != want {
        return Err(fail(format!(
            "selection arm: the manifest records selection {} where the run passed {}",
            got, want
        )));
    }
    let roots = kit_roots(state, &sc)?;
    if sorted(roots.clone()) != sorted(kits.clone()) {
        return Err(fail(format!(
            "selection arm: the battery resolves kit root(s) ({}) that differ from the manifest's kits ({})",
            roots.join(" "),
            kits.join(" ")
        )));
    }
    let t = tree(&sc)?;
    init_ok(state, &sc, &["init"], "selection arm: a bare re-run failed")?;
    if tree(&sc)? != t || !porcelain(&sc)?.is_empty() {
        return Err(fail(
            "selection arm: a bare re-run changed the tree, so it did not re-apply the recorded selection",
        ));
    }
    init_ok(state, &sc, &["init", "--no-selection"], "selection arm: init --no-selection failed")?;
    if live_members(&list) != min_registry {
        return Err(fail(format!("selection arm: --no-selection did not restore {}'s registry", min)));
    }
    let lock = Lock::of(&sc)?;
    if lock.list("kits") != min_kits {
        return Err(fail(format!("selection arm: --no-selection did not restore {}'s kits", min)));
    }
    if lock.has("selection") {
        return Err(fail("selection arm: --no-selection left a selection recorded"));
    }
    if !Path::new(&format!("{}/{}", sc, kit)).is_dir() {
        return Err(fail(format!(
            "selection arm: --no-selection deleted {}'s directory, and init never deletes",
            kit
        )));
    }
    // spec: installer/SPEC.md §The consumer smoke — a kept registry reports a member init's registry
    // stopped starting, and a --with-gate naming it ends the report
    let selected = ["init", "--profile", &min, "--with-kit", &kit, "--without-gate", &drop, "--with-gate", &gate];
    init_ok(state, &sc, &selected, "selection arm: re-applying the selection failed")?;
    append(&list, "# kept by the adopter\n")?;
    commit_all(&sc, "chore: keep the registry", "selection arm: committing the edited registry failed")?;
    let retire = format!("retire: {}", gate);
    let o = init_ok(state, &sc, &["init", "--no-selection"], "selection arm: a kept --no-selection run failed")?;
    let named = format!("{} — init's registry no longer starts it (# install: on-surface)", retire);
    if !o.lines().any(|l| l.trim() == named) {
        return Err(fail(format!(
            "selection arm: a kept registry still registering {} after --no-selection printed no '{}' line",
            gate, named
        )));
    }
    if !std::fs::read_to_string(&list).unwrap_or_default().lines().any(|l| l == gate) {
        return Err(fail(format!("selection arm: the kept registry lost {} — init wrote a file it does not own", gate)));
    }
    let o = init_ok(state, &sc, &selected, "selection arm: re-selecting over the kept registry failed")?;
    if o.contains(&retire) {
        return Err(fail(format!("selection arm: {} named with --with-gate is still reported retired", gate)));
    }
    let t = tree(&sc)?;
    let pid = std::process::id();
    let first_min = min_kits.first().cloned().unwrap_or_default();
    let refusals = [
        vec!["--without-kit".to_string(), first_min],
        vec!["--with-kit".to_string(), format!("no-such-kit-{}", pid)],
        vec!["--with-gate".to_string(), format!("check-no-such-gate-{}", pid)],
    ];
    for refused in &refusals {
        let mut argv = vec!["init"];
        argv.extend(refused.iter().map(String::as_str));
        let m = cw(state, &sc, &argv)?;
        if m.code() != Some(2) {
            return Err(failed(&m, format!(
                "selection arm: init {} exited {}, not the refusal's 2",
                refused.join(" "),
                m.reported_code()
            )));
        }
        if tree(&sc)? != t || !porcelain(&sc)?.is_empty() {
            return Err(fail(format!("selection arm: the refused init {} wrote to the tree", refused.join(" "))));
        }
    }
    say(&format!(
        "selection: {} added and {} registered, {} dropped, all recorded; a bare re-run unchanged; --no-selection restores {} with {} left on disk; a kept registry reports {} retired until --with-gate names it; three refusals at exit 2 writing nothing",
        kit, gate, drop, min, kit, gate
    ));
    Ok(())
}
