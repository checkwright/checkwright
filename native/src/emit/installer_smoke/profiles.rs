// spec: installer/SPEC.md §The consumer smoke — the profile invariant and the per-profile loop it
// carries, then the seed arm, the demo arm and the two plan-parity legs over seeds
use super::consumer::{
    all_passed, commits, consumer, copy_tree, digest_of, exists, failed, find_line, first_starting, git, git_out,
    hash_in, head, is_file, is_hash, mkdir, out, porcelain, run_line, seam_bin, sorted, tree, write, Lock, GATES_DIR,
    PROFILE_DERIVED, QUEUE_FILE,
};
use super::{fail, lines, merged_in, refuse, say, show, Outcome, Run, Step};
use crate::{programs, walk};
use std::path::Path;

// spec: installer/SPEC.md §Profiles — every payload directory but the reserved one, which is read
// by content, the target roster the packer writes into it, never by its name
pub(super) fn reserved_payload_dir(state: &Run) -> String {
    payload_dirs(state)
        .into_iter()
        .find(|d| is_file(&format!("{}/payload/{}/targets.list", state.pkg_root, d)))
        .unwrap_or_default()
}

fn payload_dirs(state: &Run) -> Vec<String> {
    walk::list_dir(Path::new(&format!("{}/payload", state.pkg_root)))
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, dir)| *dir)
        .map(|(n, _)| n)
        .collect()
}

pub(super) fn payload_kits(state: &Run) -> Vec<String> {
    let reserved = reserved_payload_dir(state);
    payload_dirs(state).into_iter().filter(|d| reserved.is_empty() || *d != reserved).collect()
}

// spec: installer/SPEC.md §Profiles — the `<profile><TAB><kit>` rows, a `#` anywhere ending a line,
// read as the data the package carries rather than asked of the installer
fn profile_rows(state: &Run) -> Vec<(String, String)> {
    let body = std::fs::read_to_string(format!("{}/profiles.list", state.pkg_root)).unwrap_or_default();
    body.lines()
        .map(|l| l.split('#').next().unwrap_or(""))
        .filter(|l| !l.trim().is_empty())
        .map(|l| match l.split_once('\t') {
            Some((p, k)) => (p.trim_matches('\t').to_string(), k.trim_matches('\t').to_string()),
            None => (l.trim_matches('\t').to_string(), String::new()),
        })
        .collect()
}

fn profile_names(state: &Run) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for (p, _) in profile_rows(state) {
        if !p.is_empty() && !seen.contains(&p) {
            seen.push(p);
        }
    }
    seen.push(PROFILE_DERIVED.to_string());
    seen
}

pub(super) fn profile_kits(state: &Run, want: &str) -> Vec<String> {
    if want == PROFILE_DERIVED {
        return payload_kits(state);
    }
    let members: Vec<String> = profile_rows(state).into_iter().filter(|(p, _)| p == want).map(|(_, k)| k).collect();
    payload_kits(state).into_iter().filter(|k| members.contains(k)).collect()
}

// spec: installer/SPEC.md §Profiles — the order derived from set inclusion over the kit rosters,
// one pair per distinct profiles whose kit set is contained in the other's
fn profile_order(state: &Run) -> Vec<(String, String)> {
    let mut order = Vec::new();
    for a in &state.profiles {
        let sub = profile_kits(state, a);
        for b in &state.profiles {
            if a == b {
                continue;
            }
            let sup = profile_kits(state, b);
            if sub.iter().all(|k| sup.contains(k)) {
                order.push((a.clone(), b.clone()));
            }
        }
    }
    order
}

// spec: installer/SPEC.md §Profiles — the lattice is asserted against the installed payload, then
// every profile is installed, held, valued and reversed, then the value and monotonicity claims are
// made over the loop
pub(super) fn invariant(state: &mut Run) -> Step {
    state.payload_kits = payload_kits(state);
    if state.payload_kits.is_empty() {
        return Err(fail("the installed payload carries no kit"));
    }
    state.profiles = profile_names(state);
    say(&format!("profiles: {} ({} kits in the payload)", state.profiles.join(" "), state.payload_kits.len()));
    for p in &state.profiles {
        let members = profile_kits(state, p);
        let named: Vec<String> = profile_rows(state).into_iter().filter(|(q, _)| q == p).map(|(_, k)| k).collect();
        if members.is_empty() {
            return Err(fail(format!("profile '{}' resolves to no kit in the payload", p)));
        }
        if let Some(k) = named.iter().find(|k| !state.payload_kits.contains(k)) {
            return Err(fail(format!("profile '{}' names {}, which the payload does not carry", p, k)));
        }
    }
    state.order = profile_order(state);
    let n = state.profiles.len();
    let bound = |below: bool| -> Vec<String> {
        state
            .profiles
            .iter()
            .filter(|p| state.order.iter().filter(|(a, b)| if below { a == *p } else { b == *p }).count() == n - 1)
            .cloned()
            .collect()
    };
    let (minima, maxima) = (bound(true), bound(false));
    if minima.len() != 1 {
        return Err(fail(format!(
            "the profile order has {} minima [{}] where a bounded lattice has exactly one",
            minima.len(),
            minima.join(" ")
        )));
    }
    if maxima.len() != 1 {
        return Err(fail(format!(
            "the profile order has {} maxima [{}] where a bounded lattice has exactly one",
            maxima.len(),
            maxima.join(" ")
        )));
    }
    if maxima[0] != PROFILE_DERIVED {
        return Err(fail(format!(
            "the maximum profile is {} where the payload-derived profile {} is the top by construction",
            maxima[0], PROFILE_DERIVED
        )));
    }
    state.profile_min = minima[0].clone();
    say(&format!(
        "order: {} comparable pair(s), minimum {}, maximum {}",
        state.order.len(),
        state.profile_min,
        PROFILE_DERIVED
    ));
    for profile in state.profiles.clone() {
        one_profile(state, &profile)?;
    }
    if state.value_red.is_empty() {
        return Err(fail(
            "no profile's battery caught the planted prose defect — the install is green, idempotent and reversible, and worth nothing on a document",
        ));
    }
    if !state.value_red.iter().any(|p| p != PROFILE_DERIVED) {
        return Err(fail(format!(
            "only {} caught the planted prose defect — no profile short of everything delivers value on prose",
            PROFILE_DERIVED
        )));
    }
    say(&format!("value: caught by {}, at least one of them below {}", state.value_red.join(" "), PROFILE_DERIVED));
    for (a, b) in &state.order {
        let (Some(sub), Some(sup)) = (state.registry.get(a), state.registry.get(b)) else {
            return Err(fail(format!(
                "gate-roster monotonicity, {} ⊆ {}: the loop above recorded no installed registry for one of them, so the containment would hold by vacuity",
                a, b
            )));
        };
        if let Some(m) = sub.iter().find(|m| !sup.contains(m)) {
            return Err(fail(format!(
                "gate-roster monotonicity, {} ⊆ {}: {} is in the smaller set and missing from the larger",
                a, b, m
            )));
        }
    }
    say("gate rosters are monotone across every comparable pair of installed registries");
    Ok(())
}

fn one_profile(state: &mut Run, profile: &str) -> Step {
    println!("{}", profile);
    let c = consumer(state, profile)?;
    let seed = tree(&c)?;
    let plan = dry_plan(state, profile, &c)?;
    if tree(&c)? != seed || !porcelain(&c)?.is_empty() {
        return Err(fail(format!("{}: init --dry-run wrote to the consumer", profile)));
    }
    assert_install(state, profile, &c)?;
    let keep = format!("{}/seed-{}/.github/workflows", state.scratch, profile);
    mkdir(&keep)?;
    let seeded = format!("{}/.github/workflows/gates.yml", c);
    if is_file(&seeded) {
        std::fs::copy(&seeded, format!("{}/gates.yml", keep))
            .map_err(|e| fail(format!("{}: could not keep the seeded workflow for the seed arm: {}", profile, e)))?;
    }
    plan_parity(profile, &plan, &Lock::of(&c)?)?;
    if profile == PROFILE_DERIVED {
        state.seeded = seeded_paths(state, &Lock::of(&c)?);
    }
    if value(state, profile, &c)? {
        state.value_red.push(profile.to_string());
    }
    reversal(state, profile, &c, &seed)
}

// spec: installer/SPEC.md §init — the follow-up block's grammar is the operand: the indented lines
// after a `next:` line, each with its comment and surrounding blanks dropped
pub(super) fn followups(out: &str) -> Vec<String> {
    let mut cmds = Vec::new();
    let mut on = false;
    for line in lines::of("init's follow-up block", out.as_bytes()) {
        if !on {
            on = line == "next:";
            continue;
        }
        let indented = line.starts_with([' ', '\t']) && !line.trim().is_empty();
        if !indented {
            break;
        }
        cmds.push(line.split('#').next().unwrap_or("").trim().to_string());
    }
    cmds
}

// spec: installer/SPEC.md §The consumer smoke — every printed command resolves in the install with
// every flag it names accepted, probed in a throwaway copy against the target's own refusal
fn assert_followups(state: &Run, profile: &str, c: &str, init_out: &str) -> Step {
    let cmds = followups(init_out);
    if cmds.is_empty() {
        show(init_out);
        return Err(refuse(format!(
            "{}: init printed no follow-up block matching the grammar installer/SPEC.md §init states, so this arm has nothing to assert over.",
            profile
        )));
    }
    let sandbox = format!("{}/followup-{}", state.scratch, profile);
    let _ = std::fs::remove_dir_all(&sandbox);
    copy_tree(Path::new(c), Path::new(&sandbox))
        .map_err(|e| fail(format!("{}: could not copy the consumer for the follow-up probe: {}", profile, e)))?;
    let set = super::consumer::path_set(&state.run_path);
    for line in &cmds {
        let toks: Vec<&str> = line.split_whitespace().collect();
        let Some(ti) = toks.iter().position(|t| t.contains('/')) else {
            return Err(refuse(format!(
                "{}: the follow-up command '{}' names no repo-relative script path, so this arm cannot tell which file init is telling the adopter to run.",
                profile, line
            )));
        };
        let target = format!("{}/{}", c, toks[ti]);
        if !is_file(&target) {
            return Err(fail(format!(
                "{}: init told the adopter to run '{}', and {} is not a file in the tree init just wrote",
                profile, line, toks[ti]
            )));
        }
        if ti == 0 && !crate::proc::is_executable(Path::new(&target)) && cfg!(unix) {
            return Err(fail(format!(
                "{}: init told the adopter to run '{}' directly, and {} is not executable in the tree init just wrote",
                profile, line, toks[ti]
            )));
        }
        for i in ti + 1..toks.len() {
            let flag = toks[i];
            if !flag.starts_with('-') {
                continue;
            }
            let sentinel = format!("{}--checkwright-smoke-unknown", flag);
            let mut ctl_argv = toks.clone();
            ctl_argv[i] = &sentinel;
            let ctl_out = out(&run_line(&sandbox, &ctl_argv, &set)?);
            let Some(ctl) = find_line(&ctl_out, |l| l.contains(&sentinel)) else {
                return Err(refuse(format!(
                    "{}: {} does not refuse '{}' by name, so this arm cannot tell a flag it accepts from one it rejects.",
                    profile, toks[ti], sentinel
                )));
            };
            let expect = ctl.replace(&sentinel, flag);
            if out(&run_line(&sandbox, &toks, &set)?).contains(&expect) {
                return Err(fail(format!(
                    "{}: init told the adopter to run '{}', and {} refuses that flag — {}",
                    profile, line, toks[ti], expect
                )));
            }
        }
    }
    let _ = std::fs::remove_dir_all(&sandbox);
    say(&format!(
        "follow-up: {} printed command(s), each resolving in the install with every flag it names accepted",
        cmds.len()
    ));
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the registry's live membership: comment and blank
// lines dropped, sorted and unique
pub(super) fn live_members(list: &str) -> Vec<String> {
    let body = std::fs::read_to_string(list).unwrap_or_default();
    sorted(
        body.lines()
            .filter(|l| {
                let t = l.trim_start();
                !t.is_empty() && !t.starts_with('#')
            })
            .map(str::to_string)
            .collect(),
    )
}

// spec: installer/SPEC.md §The consumer smoke — one encoding of the post-conditions, read by every
// transport: install, follow-ups, battery, manifest, withholding, kit roots, queue, idempotence,
// artifact and doctor
pub(super) fn assert_install(state: &mut Run, profile: &str, c: &str) -> Step {
    let m = state.verb(c, &["init", "--profile", profile])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("init failed for the {} profile", profile)));
    }
    let init_out = out(&m);
    say(&format!("init: {}", first_starting(&init_out, "INIT:")));
    assert_followups(state, profile, c, &init_out)?;
    battery_green(state, profile, c)?;
    let lock_path = format!("{}/checkwright.lock", c);
    if !is_file(&lock_path) {
        return Err(fail(format!("{}: init wrote no checkwright.lock", profile)));
    }
    let lock = Lock::read(&lock_path)?;
    manifest(state, profile, c, &lock)?;
    let lock_kits = lock.list("kits");
    let want_kits = profile_kits(state, profile);
    if lock_kits != want_kits {
        return Err(fail(format!(
            "{}: manifest kits ({}) differ from the profile roster ({})",
            profile,
            lock_kits.join(" "),
            want_kits.join(" ")
        )));
    }
    say(&format!("manifest: {} file(s) agree with the tree, {} kit(s) recorded", lock.files().len(), lock_kits.len()));
    withholding(state, profile, c, &lock_kits)?;
    queue(state, profile, c, &lock, &want_kits)?;
    let before = tree(c)?;
    let m = state.verb(c, &["init"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("{}: the idempotent re-run of init failed", profile)));
    }
    if tree(c)? != before {
        return Err(fail(format!("{}: re-running init changed the tree — the install is not idempotent", profile)));
    }
    if !porcelain(c)?.is_empty() {
        return Err(fail(format!("{}: the re-run left the worktree dirty", profile)));
    }
    say("re-run: tree unchanged");
    placed_artifact(state, profile, c, &lock)?;
    doctor(state, profile, c)
}

fn battery_green(state: &Run, profile: &str, c: &str) -> Step {
    let m = state.front_end(c, &[])?;
    let said = out(&m);
    let Some(summary) = all_passed(&said).filter(|_| m.succeeded()) else {
        println!("{}", said);
        return Err(fail(format!("the battery is not green on the {} consumer init just made", profile)));
    };
    say(&format!("battery: {}", summary));
    let dirty = porcelain(c)?;
    if !dirty.is_empty() {
        show(&dirty);
        return Err(fail(format!(
            "{}: the battery left the worktree dirty — the paths above were written outside the self-ignoring scratch directory",
            profile
        )));
    }
    let m = state.front_end(c, &["--run", "--only", "check-tree-terms"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "{}: check-tree-terms, run by name, is not green over the tree init committed, the placed binary included",
            profile
        )));
    }
    if find_line(&out(&m), |l| l.contains("All 1 gates passed")).is_none() {
        return Err(failed(&m, format!(
            "{}: the by-name check-tree-terms run printed no one-gate summary, so it ran nothing",
            profile
        )));
    }
    say(&format!("{}: check-tree-terms, run by name, is green over the committed tree", profile));
    Ok(())
}

// spec: installer/SPEC.md §The manifest — the files[] hash agrees with the tree it describes: one
// --stdin-paths batch, a path it cannot carry taking the one-file call, the shape test on the
// failure branch and the report before the verdict
fn manifest(state: &Run, profile: &str, c: &str, lock: &Lock) -> Step {
    if lock.top("schema") != "checkwright-lock v1" {
        return Err(fail(format!("{}: manifest carries an unexpected schema", profile)));
    }
    if lock.top("version") != state.version {
        return Err(fail(format!(
            "{}: manifest records version {}, packed {}",
            profile,
            lock.top("version"),
            state.version
        )));
    }
    if !is_hash(&lock.top("commit")) {
        return Err(fail(format!("{}: manifest records no 40-hex commit", profile)));
    }
    if lock.top("profile") != profile {
        return Err(fail(format!("{}: manifest records the wrong profile", profile)));
    }
    let files = lock.files();
    let mut mismatch = 0usize;
    let mut hashed: Vec<(String, String, Option<String>)> = Vec::new();
    let mut batch: Vec<String> = Vec::new();
    for (path, want) in &files {
        let abs = format!("{}/{}", c, path);
        if !is_file(&abs) {
            println!("  manifest names a file that is not there: {}", path);
            mismatch += 1;
            continue;
        }
        if path.contains(['\r', '\n']) || path.starts_with('"') {
            let one = git_out(&state.root, &["hash-object", "--", &abs])?;
            hashed.push((path.clone(), want.clone(), Some(one.trim_end_matches('\n').to_string())));
        } else {
            hashed.push((path.clone(), want.clone(), None));
            batch.push(abs);
        }
    }
    let mut answers = hash_batch(state, profile, &batch)?.into_iter();
    let mut bad: Vec<(String, String, String)> = Vec::new();
    let mut malformed: Option<(String, String, String)> = None;
    let mut malformed_n = 0usize;
    for (path, want, single) in hashed {
        let got = match single {
            Some(g) => g,
            None => answers.next().unwrap_or_default(),
        };
        if got == want {
            continue;
        }
        println!("  manifest hash disagrees with the tree: {}", path);
        mismatch += 1;
        let tuple = (path, want, got);
        if !is_hash(&tuple.1) || !is_hash(&tuple.2) {
            malformed_n += 1;
            malformed.get_or_insert_with(|| tuple.clone());
        }
        bad.push(tuple);
    }
    if mismatch > 0 {
        super::report::manifest_report(state, profile, c, lock, mismatch, files.len(), malformed.as_ref(), malformed_n, &bad);
    }
    if let Some(w) = malformed.as_ref().filter(|_| mismatch > 0) {
        return Err(refuse(format!(
            "{}: on {} the shape test refused {}, and it refused {} of {} disagreeing entries. The report above samples that entry and prints each value's octet dump beside its length. Read the shape verdict in the parentheses beside that length: a first offending byte names where the operand stops being a hash, and none names a length other than 40. That is this harness's own precondition, not a finding about the consumer.",
            profile,
            w.0,
            super::report::malformed_operands(w),
            malformed_n,
            mismatch
        )));
    }
    if mismatch > 0 {
        return Err(fail(format!("{}: {} of {} manifest entries disagree with the tree", profile, mismatch, files.len())));
    }
    if files.is_empty() {
        return Err(fail(format!("{}: the manifest records no file", profile)));
    }
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — one --stdin-paths child, no fallback on failure, and
// an answer count other than the path count refused, since pairing by index needs equal counts
fn hash_batch(state: &Run, profile: &str, paths: &[String]) -> Result<Vec<String>, Outcome> {
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let input: String = paths.iter().map(|p| format!("{}\n", p)).collect();
    let done = super::in_consumer_fed(&programs::GIT, &["hash-object", "--stdin-paths"], &[], &state.root, input.as_bytes())?;
    let Some(stdout) = done.stdout() else {
        return Err(refuse(format!(
            "{}: the manifest arm's git hash-object --stdin-paths batch over {} path(s) failed: {}",
            profile,
            paths.len(),
            done.failure_report().unwrap_or_default()
        )));
    };
    let got = lines::of("the manifest hash batch", stdout);
    if got.len() != paths.len() {
        return Err(refuse(format!(
            "{}: the manifest arm's git hash-object --stdin-paths batch answered {} line(s) for {} path(s)",
            profile,
            got.len(),
            paths.len()
        )));
    }
    Ok(got)
}

// spec: gate-sdk/SPEC.md §Consumer payload — no vendored kit root carries a withheld path or lacks
// the license, the kit roots the battery resolves are the manifest's, and the reserved directory is
// in neither
fn withholding(state: &Run, profile: &str, c: &str, lock_kits: &[String]) -> Step {
    let mut withheld = Vec::new();
    for k in lock_kits {
        for leaf in ["SPEC.md", "smoke"] {
            if exists(&format!("{}/{}/{}", c, k, leaf)) {
                withheld.push(format!("{}/{}", k, leaf));
            }
        }
    }
    if !withheld.is_empty() {
        return Err(fail(format!(
            "{}: the payload vendored withheld path(s) into the consumer: {}",
            profile,
            withheld.join(" ")
        )));
    }
    say(&format!("withholding: no kit SPEC.md and no kit smoke/ across {} vendored kit root(s)", lock_kits.len()));
    let license = std::fs::read(format!("{}/LICENSE", state.root)).unwrap_or_default();
    let unlicensed: Vec<&String> = lock_kits
        .iter()
        .filter(|k| std::fs::read(format!("{}/{}/LICENSE", c, k)).ok().as_ref() != Some(&license))
        .collect();
    if !unlicensed.is_empty() {
        return Err(fail(format!(
            "{}: vendored kit root(s) carry no LICENSE equal to the repository's: {}",
            profile,
            unlicensed.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ")
        )));
    }
    say(&format!("license: {} vendored kit root(s) carry the repository's LICENSE", lock_kits.len()));
    let resolved = kit_roots(state, c)?;
    if sorted(resolved.clone()) != sorted(lock_kits.to_vec()) {
        return Err(fail(format!(
            "{}: the battery resolves kit root(s) ({}) that differ from the manifest's kits ({})",
            profile,
            resolved.join(" "),
            lock_kits.join(" ")
        )));
    }
    let reserved = reserved_payload_dir(state);
    if reserved.is_empty() {
        return Err(fail(format!(
            "{}: no payload directory carries targets.list, so the reserved artifact directory this assertion is about cannot be read independently — the assertion would pass vacuously",
            profile
        )));
    }
    if resolved.iter().chain(lock_kits).any(|k| *k == reserved) {
        return Err(fail(format!(
            "{}: '{}' is the payload's reserved artifact directory, and it is being carried as a kit root — a declared root takes no disk predicate, so every kit-root sweep would reach it",
            profile, reserved
        )));
    }
    say(&format!(
        "kit roots: the battery resolves all {} kit(s) the manifest records, and the reserved '{}' directory is in neither set",
        resolved.len(),
        reserved
    ));
    Ok(())
}

// spec: installer/SPEC.md §What init seeds — the kit roots the consumer's battery resolves, counted
// rather than read off a verdict
pub(super) fn kit_roots(state: &Run, c: &str) -> Result<Vec<String>, Outcome> {
    let set = super::consumer::path_set(&state.run_path);
    let done = super::consumer::front_end_split(c, &["--emit", "kit-roots"], &set)?;
    match done.stdout() {
        Some(o) => Ok(lines::of("resolved kit roots", o).into_iter().filter(|l| !l.is_empty()).collect()),
        None => Err(fail(format!(
            "the consumer's --emit kit-roots failed: {}",
            done.failure_report().unwrap_or_default()
        ))),
    }
}

// spec: installer/SPEC.md §What init seeds — the queue post-condition: the resolver init used,
// through the --install queue-source read op, then the section floor dispatched by its payload
// descriptor to the consumer's own gate binary
fn queue(state: &Run, profile: &str, c: &str, lock: &Lock, want_kits: &[String]) -> Step {
    let payload = format!("{}/payload", state.pkg_root);
    let kits = want_kits.join(",");
    let m = super::consumer::entry_run(
        &state.installed(),
        &state.scratch,
        &["--install", "queue-source", "--payload", &payload, "--kits", &kits],
        &[],
    )?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "{}: the --install queue-source read op exited {}, so the queue post-condition has no resolver to assert against",
            profile,
            m.reported_code()
        )));
    }
    let queue_path = format!("{}/{}", c, QUEUE_FILE);
    if out(&m).trim().is_empty() {
        if is_file(&queue_path) {
            return Err(fail(format!("{}: no kit in its set reads the queue file, yet init seeded one", profile)));
        }
        say("queue: none seeded, and none is owed to this kit set");
        return Ok(());
    }
    if !is_file(&queue_path) {
        return Err(fail(format!("{}: its kit set reads the queue file and init seeded none", profile)));
    }
    let seam = format!("{}/gate-sdk-config.knobs", GATES_DIR);
    let bin = if lock.has_file(&seam) { seam_bin(&format!("{}/{}", c, seam)).unwrap_or_default() } else { String::new() };
    let bin_path = format!("{}/{}", c, bin);
    if bin.is_empty() || !crate::proc::is_executable(Path::new(&bin_path)) {
        return Err(fail(format!(
            "{}: the seam names no executable gate binary at '{}', so the section floor has nothing to dispatch to on an install the bootstrap let proceed",
            profile,
            if bin.is_empty() { "<unset>" } else { &bin }
        )));
    }
    let descriptor = format!("{}/queue-kit/checks/check-queue-sections.gate", payload);
    if !is_file(&descriptor) {
        return Err(refuse(format!(
            "{}: the payload carries no {} for the section floor to dispatch through",
            profile, descriptor
        )));
    }
    let set = super::consumer::path_set(&state.run_path);
    let m = merged_in(&programs::CHECKWRIGHT_GATES.at(bin_path), &["check-queue-sections", QUEUE_FILE], &set, c)?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "{}: the queue file init seeded does not satisfy the section contract queue-kit's own gate reads",
            profile
        )));
    }
    say(&format!("queue: {}", first_starting(&out(&m), "QUEUE-SECTIONS:")));
    Ok(())
}

// spec: installer/SPEC.md §The gate binary — an install that ran placed a verified artifact: the
// registry has live members and no omission, the binary matches its digest, and both it and the
// seam are on the roster
fn placed_artifact(state: &mut Run, profile: &str, c: &str, lock: &Lock) -> Step {
    let list = format!("{}/gates.list", GATES_DIR);
    if !lock.has_file(&list) {
        return Err(fail(format!("{}: the manifest records no gates.list", profile)));
    }
    let list_path = format!("{}/{}", c, list);
    let members = live_members(&list_path);
    if members.is_empty() {
        return Err(fail(format!(
            "{}: the registry init wrote declares no member at all — every gate this profile's kits register went missing rather than being installed",
            profile
        )));
    }
    let omitted = std::fs::read_to_string(&list_path).unwrap_or_default().lines().filter(|l| l.starts_with("# omitted:")).count();
    if omitted != 0 {
        return Err(fail(format!(
            "{}: the registry declares {} omitted member(s) on an install that placed a verified artifact — the installer writes no omission record at all now, so this is a producer that outlived the outcome it recorded",
            profile, omitted
        )));
    }
    let target = lock.artifact("target");
    if target.is_empty() {
        return Err(fail(format!(
            "{}: the manifest records no artifact, yet the bootstrap ran a verb — the only outcome that proceeds is a verified artifact, so an install with none is a refusal that did not refuse",
            profile
        )));
    }
    let seam = format!("{}/gate-sdk-config.knobs", GATES_DIR);
    if !lock.has_file(&seam) || !is_file(&format!("{}/{}", c, seam)) {
        return Err(fail(format!("{}: an artifact is recorded but no gate-sdk config seam names its path", profile)));
    }
    let bin = seam_bin(&format!("{}/{}", c, seam)).unwrap_or_default();
    if bin.is_empty() || !crate::proc::is_executable(Path::new(&format!("{}/{}", c, bin))) {
        return Err(fail(format!(
            "{}: no executable gate binary at '{}'",
            profile,
            if bin.is_empty() { "<unset>" } else { &bin }
        )));
    }
    if digest_of(&format!("{}/{}", c, bin)) != lock.artifact("digest") {
        return Err(fail(format!(
            "{}: the installed gate binary does not match the digest the manifest recorded",
            profile
        )));
    }
    for path in [&bin, &seam] {
        if !lock.has_file(path) {
            return Err(fail(format!(
                "{}: init wrote {} on the placement path but the manifest roster does not record it",
                profile, path
            )));
        }
    }
    say(&format!(
        "artifact: {} verified in place at {}, recorded with the seam, {} live member(s) and nothing omitted",
        target,
        bin,
        members.len()
    ));
    state.registry.insert(profile.to_string(), members);
    Ok(())
}

// spec: installer/SPEC.md §doctor — clean, the installed profile named, every disarmed member one
// this install registered, and whether bash is owed read off the selection's own report
fn doctor(state: &mut Run, profile: &str, c: &str) -> Step {
    let m = state.verb(c, &["doctor"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("{}: doctor exited {} inside the installed consumer", profile, m.reported_code())));
    }
    let said = lines::of("doctor's report", m.output());
    if !said.iter().any(|l| *l == format!("  profile      {}", profile)) {
        return Err(fail(format!("{}: doctor did not report the installed profile", profile)));
    }
    let disarmed: Vec<String> = said
        .iter()
        .filter_map(|l| {
            let rest = l.strip_prefix("  disarmed     ")?;
            let (member, tail) = rest.split_once(' ')?;
            tail.starts_with("asserts nothing until ").then(|| member.to_string())
        })
        .collect();
    if disarmed.is_empty() {
        return Err(failed(&m, format!(
            "{}: doctor names no disarmed member on a fresh install — every arming knob is still empty, so a declaring member it registered went unreported",
            profile
        )));
    }
    let registered = state.registry.get(profile).cloned().unwrap_or_default();
    if let Some(member) = disarmed.iter().find(|d| !registered.contains(d)) {
        return Err(fail(format!(
            "{}: doctor named {} disarmed, but this install did not register it",
            profile, member
        )));
    }
    let bash_row = said.iter().any(|l| l == "  bash" || l.starts_with("  bash "));
    let unprobed = said
        .iter()
        .any(|l| l.strip_prefix("  bash ").is_some_and(|r| r.trim_start().starts_with("not probed")));
    state.owes_bash.insert(profile.to_string(), bash_row && !unprobed);
    say(&format!(
        "doctor: clean, reports the installed profile and {} disarmed member(s)",
        disarmed.len()
    ));
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the value post-condition: a mistyped relative link
// in adopter prose, the battery's verdict on it recorded, then green once the link is fixed, and the
// consumer restored to the commit it found
fn value(state: &Run, profile: &str, c: &str) -> Result<bool, Outcome> {
    let found = head(c)?;
    write(&format!("{}/docs/README.md", c), "# Handbook\n\nStart with [the style guide](style-guid.md).\n")?;
    write(&format!("{}/docs/style-guide.md", c), "# Style guide\n\nWrite plainly, and link what you cite.\n")?;
    commit_all(c, "handbook", &format!("{}: could not commit the prose consumer's own content", profile))?;
    let red = !state.front_end(c, &[])?.succeeded();
    write(&format!("{}/docs/README.md", c), "# Handbook\n\nStart with [the style guide](style-guide.md).\n")?;
    commit_all(c, "fix the link", &format!("{}: could not commit the fix", profile))?;
    let m = state.front_end(c, &[])?;
    if !m.succeeded() || all_passed(&out(&m)).is_none() {
        println!("{}", out(&m));
        return Err(fail(format!(
            "{}: the battery is still not green on prose whose only defect was fixed",
            profile
        )));
    }
    say(&format!(
        "value: the planted prose defect is {} on this profile, green once fixed",
        if red { "red" } else { "green" }
    ));
    let restored = git(c, &["reset", "-q", "--hard", &found])?.succeeded() && git(c, &["clean", "-qfd"])?.succeeded();
    if !restored {
        return Err(fail(format!("{}: could not restore the consumer after the value arm", profile)));
    }
    Ok(red)
}

pub(super) fn commit_all(c: &str, message: &str, why: &str) -> Step {
    let added = git(c, &["add", "-A"])?;
    let committed = if added.succeeded() { git(c, &["commit", "-q", "-m", message])? } else { added };
    if committed.succeeded() {
        return Ok(());
    }
    Err(failed(&committed, why))
}

// spec: installer/SPEC.md §The consumer smoke — diff clean, update --dry-run planning a deleted
// file's restore and writing nothing, uninstall --dry-run planning and writing nothing, then
// uninstall back to the pre-init tree object
pub(super) fn reversal(state: &Run, profile: &str, c: &str, seed: &str) -> Step {
    let m = state.verb(c, &["diff"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "{}: diff exited {} against the tree init just wrote — a freshly installed tree is the definition of no drift",
            profile,
            m.reported_code()
        )));
    }
    let said = out(&m);
    if !said.split('\n').any(|l| l.starts_with("DIFF: clean")) {
        return Err(failed(&m, format!("{}: diff exited 0 without reporting the tree clean", profile)));
    }
    say(&format!("diff: {}", first_starting(&said, "DIFF:")));
    let found = head(c)?;
    let lock = Lock::of(c)?;
    let victim = lock.keys().into_iter().find(|k| k.starts_with("gate-sdk/")).unwrap_or_default();
    if victim.is_empty() || !is_file(&format!("{}/{}", c, victim)) {
        return Err(fail(format!(
            "{}: the manifest records no gate-sdk file for the update --dry-run arm to delete",
            profile
        )));
    }
    let rm = git(c, &["rm", "-q", "--", &victim])?;
    let message = format!("chore: drop {}", victim);
    if !rm.succeeded() || !git(c, &["commit", "-q", "-m", &message])?.succeeded() {
        return Err(fail(format!(
            "{}: could not commit the deletion the update --dry-run arm plans against",
            profile
        )));
    }
    let before = tree(c)?;
    let m = state.verb(c, &["update", "--dry-run"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("{}: update --dry-run exited {}", profile, m.reported_code())));
    }
    let said = out(&m);
    if find_line(&said, |l| l == "DRY RUN: nothing was written.").is_none() {
        return Err(failed(&m, format!("{}: update --dry-run printed no dry-run verdict", profile)));
    }
    let quoted = format!("\"{}\"", victim);
    if find_line(&said, |l| l.contains(&quoted) && l.contains("(pending)")).is_none() {
        return Err(failed(&m, format!("{}: update --dry-run did not plan to restore the deleted {}", profile, victim)));
    }
    if tree(c)? != before || exists(&format!("{}/{}", c, victim)) {
        return Err(fail(format!("{}: update --dry-run wrote to the consumer", profile)));
    }
    if !porcelain(c)?.is_empty() {
        return Err(fail(format!("{}: update --dry-run left the worktree changed", profile)));
    }
    if !git(c, &["reset", "-q", "--hard", &found])?.succeeded() {
        return Err(fail(format!("{}: could not restore the consumer after the update --dry-run arm", profile)));
    }
    say(&format!(
        "update --dry-run: planned the deleted {} pending, tree object and worktree unchanged",
        victim
    ));
    let before = tree(c)?;
    let status = porcelain(c)?;
    let m = state.verb(c, &["uninstall", "--dry-run"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("{}: uninstall --dry-run exited {}", profile, m.reported_code())));
    }
    let planned: usize = find_line(&out(&m), |l| l.starts_with("would remove ") && l.ends_with(" file(s):"))
        .and_then(|l| l["would remove ".len()..l.len() - " file(s):".len()].parse().ok())
        .unwrap_or(0);
    if planned == 0 {
        return Err(failed(&m, format!(
            "{}: uninstall --dry-run planned no removal against an install it is about to reverse",
            profile
        )));
    }
    if tree(c)? != before {
        return Err(fail(format!("{}: uninstall --dry-run changed the tree object", profile)));
    }
    if porcelain(c)? != status {
        return Err(fail(format!("{}: uninstall --dry-run left the worktree changed", profile)));
    }
    say(&format!("uninstall --dry-run: {} file(s) planned, tree object and worktree unchanged", planned));
    let m = state.verb(c, &["uninstall"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("{}: uninstall exited {}", profile, m.reported_code())));
    }
    if tree(c)? != seed {
        return Err(failed(&m, format!(
            "{}: the tree after uninstall is not the tree from before init — either init wrote something it did not record, or the removal reached past the roster",
            profile
        )));
    }
    if !porcelain(c)?.is_empty() {
        return Err(fail(format!("{}: uninstall left the worktree dirty", profile)));
    }
    if is_file(&format!("{}/checkwright.lock", c)) {
        return Err(fail(format!(
            "{}: every recorded file was removed, yet a manifest survives asserting an install that is gone",
            profile
        )));
    }
    let said = out(&m);
    let line = first_starting(&said, "UNINSTALL: ");
    say(&format!(
        "uninstall: {} tree object is back to its pre-init state",
        line.strip_prefix("UNINSTALL: ").unwrap_or("")
    ));
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the dry plan is the manifest init --dry-run prints,
// its path keys sorted
fn dry_plan(state: &Run, profile: &str, c: &str) -> Result<Vec<String>, Outcome> {
    let m = state.verb(c, &["init", "--profile", profile, "--dry-run"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("{}: init --dry-run exited {}", profile, m.reported_code())));
    }
    let mut on = false;
    let mut body = String::new();
    for line in lines::of("init --dry-run's plan", m.output()) {
        if line.starts_with("DRY RUN:") {
            on = false;
        }
        if on {
            body.push_str(&line);
            body.push('\n');
        }
        if line.ends_with(" that would be written:") {
            on = true;
        }
    }
    let plan: serde_json::Value = serde_json::from_str(&body).map_err(|_| {
        show(&out(&m));
        refuse(format!("{}: the manifest init --dry-run printed would not parse as one", profile))
    })?;
    Ok(plan
        .get("files")
        .and_then(|f| f.as_object())
        .map(|f| f.keys().cloned().collect())
        .unwrap_or_default())
}

// spec: installer/SPEC.md §The consumer smoke — the set init --dry-run names equals the set the real
// run records, both ways, and an empty plan reds
fn plan_parity(label: &str, plan: &[String], lock: &Lock) -> Step {
    if plan.is_empty() {
        return Err(fail(format!("{}: init --dry-run planned no file, so parity would hold by vacuity", label)));
    }
    let real = lock.keys();
    if plan != real.as_slice() {
        for p in plan.iter().filter(|p| !real.contains(p)) {
            eprintln!("< {}", p);
        }
        for r in real.iter().filter(|r| !plan.contains(r)) {
            eprintln!("> {}", r);
        }
        return Err(fail(format!(
            "{}: init --dry-run planned a different file set from the one the real run recorded (< planned only, > recorded only)",
            label
        )));
    }
    say(&format!("plan parity: init --dry-run named the {} file(s) the run recorded", real.len()));
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the seeded surfaces, read off what the maximum's
// install recorded: every path outside a vendored kit and the gates directory
fn seeded_paths(state: &Run, lock: &Lock) -> Vec<String> {
    lock.keys()
        .into_iter()
        .filter(|p| {
            !state
                .payload_kits
                .iter()
                .map(String::as_str)
                .chain([GATES_DIR])
                .any(|k| walk::under(k, p))
        })
        .collect()
}

// spec: installer/SPEC.md §The consumer smoke — the seed arm: the workflow every profile seeded names
// the packed repository, commit and version, and the payload binary's workflow gates are green on it
pub(super) fn seed(state: &mut Run) -> Step {
    let pkg: serde_json::Value = std::fs::read_to_string(format!("{}/package.json", state.pkg_root))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default();
    let repo = pkg.get("repository");
    let url = repo.and_then(|r| r.get("url")).or(repo).and_then(|u| u.as_str()).unwrap_or("");
    let slug = url.strip_prefix("git+").unwrap_or(url);
    let slug = slug.strip_prefix("https://github.com/").unwrap_or(slug);
    let slug = slug.strip_suffix(".git").unwrap_or(slug).to_string();
    let commit = pkg.get("checkwright").and_then(|c| c.get("commit")).and_then(|c| c.as_str()).unwrap_or("");
    if slug.is_empty() || !is_hash(commit) {
        return Err(refuse(
            "seed arm: the packed package.json names no repository slug or no stamped commit, so there is no step to expect",
        ));
    }
    let step = format!("uses: {}/installer@{} # v{}", slug, commit, state.version);
    let bin = format!("{}/payload/artifact/{}/{}", state.pkg_root, state.host, state.bin_name);
    if !crate::proc::is_executable(Path::new(&bin)) {
        return Err(refuse(format!(
            "seed arm: the packed payload carries no executable {} binary at {} to run the workflow gates with",
            state.host, bin
        )));
    }
    for profile in &state.profiles {
        let dir = format!("{}/seed-{}", state.scratch, profile);
        let seeded = format!("{}/.github/workflows/gates.yml", dir);
        if !is_file(&seeded) {
            return Err(fail(format!("{}: init seeded no .github/workflows/gates.yml", profile)));
        }
        let body = std::fs::read_to_string(&seeded).unwrap_or_default();
        if !body.contains(&step) {
            show(&body);
            return Err(fail(format!("{}: the seeded workflow does not carry the step '{}'", profile, step)));
        }
        for gate in ["check-action-pinning", "check-action-permissions"] {
            let m = merged_in(&programs::CHECKWRIGHT_GATES.at(bin.clone()), &[gate], &[], &dir)?;
            if !m.succeeded() {
                return Err(failed(&m, format!("{}: {} is not green over the workflow init seeded", profile, gate)));
            }
        }
    }
    say(&format!(
        "seed: {} profile(s) seeded the workflow calling {}/installer at the packed commit, pinning and permissions green over each",
        state.profiles.len(),
        slug
    ));
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the demo arm: the verb green from inside an installed
// consumer, act 3 red, the front page's proof block in its output, the consumer unchanged, its
// scratch torn down and an operand refused
pub(super) fn demo(state: &mut Run) -> Step {
    let c = consumer(state, "demo")?;
    let m = state.verb(&c, &["init", "--profile", &state.profile_min])?;
    if !m.succeeded() {
        show(&out(&m));
        return Err(refuse(format!(
            "demo arm: init --profile {} exited non-zero on the consumer the verb is to run inside",
            state.profile_min
        )));
    }
    let scratch = format!("{}/demo-scratch", state.scratch);
    std::fs::create_dir_all(&scratch)
        .map_err(|_| refuse("demo arm: could not make the scratch base the verb is pointed at"))?;
    let (before, status) = (tree(&c)?, porcelain(&c)?);
    let m = state.verb_with(&c, &["demo"], &[("DEMO_TMP_DIR".to_string(), scratch.clone())])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("demo arm: checkwright demo exited {}", m.reported_code())));
    }
    let said = out(&m);
    if !said.contains("DEMO: clean") {
        return Err(failed(&m, "demo arm: checkwright demo exited 0 without its DEMO: clean line"));
    }
    let said_lines = lines::of("the demo's output", m.output());
    let mut on = false;
    let mut act3_red = 0usize;
    for line in &said_lines {
        if line.starts_with("  ACT 3 ") {
            on = true;
            continue;
        }
        if line.starts_with("  ACT 4 ") {
            on = false;
        }
        if on && line.contains("FAIL: ") {
            act3_red += 1;
        }
    }
    if act3_red == 0 {
        return Err(failed(&m, "demo arm: checkwright demo printed no FAIL: verdict line between its act 3 and act 4 banners"));
    }
    let index = std::fs::read_to_string(format!("{}/docs/index.md", state.root)).unwrap_or_default();
    let mut on = false;
    let mut proof = Vec::new();
    for line in index.lines() {
        if line.starts_with("<!-- demo-proof:begin -->") {
            on = true;
            continue;
        }
        if line.starts_with("<!-- demo-proof:end -->") {
            on = false;
        }
        if on && !line.starts_with("```") && !line.trim().is_empty() {
            proof.push(line);
        }
    }
    if proof.is_empty() {
        return Err(fail("demo arm: docs/index.md carries no demo-proof block to hold against the demo's output"));
    }
    if let Some(line) = proof.iter().find(|l| !said_lines.iter().any(|s| s.contains(*l))) {
        return Err(failed(&m, format!(
            "demo arm: a line of docs/index.md's demo-proof block is not in the demo's output — recapture it from this run: {}",
            line
        )));
    }
    if tree(&c)? != before {
        return Err(fail("demo arm: checkwright demo changed the invoking repository's tree object"));
    }
    if porcelain(&c)? != status {
        return Err(fail("demo arm: checkwright demo left the invoking repository's worktree changed"));
    }
    let left = walk::list_dir(Path::new(&scratch)).unwrap_or_default();
    if !left.is_empty() {
        let names: Vec<String> = left.into_iter().map(|(n, _)| n).collect();
        return Err(fail(format!(
            "demo arm: checkwright demo left its scratch behind in DEMO_TMP_DIR: {}",
            names.join(" ")
        )));
    }
    say(&format!(
        "demo: {} green battery run(s), {} FAIL: verdict line(s) in act 3, {} front-page proof line(s) found in the output, the invoking tree and worktree unchanged, the scratch torn down",
        said_lines.iter().filter(|l| l.contains("gates passed")).count(),
        act3_red,
        proof.len()
    ));
    let m = state.verb(&c, &["demo", "extra"])?;
    if m.code() != Some(2) {
        return Err(failed(&m, format!(
            "demo arm: checkwright demo extra exited {}, not the usage refusal's 2",
            m.reported_code()
        )));
    }
    if !out(&m).contains("usage: checkwright demo") {
        return Err(failed(&m, "demo arm: checkwright demo extra refused without the usage line"));
    }
    say("demo extra: refused at exit 2 with the usage line");
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the held-seed leg: a fresh consumer already holding
// every surface init seeds, where a seed's absence guard decides
pub(super) fn held_seeds(state: &mut Run) -> Step {
    if state.seeded.is_empty() {
        return Err(fail(format!(
            "the {} install recorded no seeded surface, so the held-seed arm would plant nothing",
            PROFILE_DERIVED
        )));
    }
    let c = consumer(state, "held-seeds")?;
    for p in &state.seeded {
        write(&format!("{}/{}", c, p), "# held by the adopter before init\n")?;
    }
    commit_all(&c, "held seeds", "could not commit the held seeds")?;
    let plan = dry_plan(state, PROFILE_DERIVED, &c)?;
    let m = state.verb(&c, &["init", "--profile", PROFILE_DERIVED])?;
    if !m.succeeded() {
        return Err(failed(&m, "init failed on the held-seed consumer"));
    }
    plan_parity(&format!("held seeds [{} ]", state.seeded.join(" ")), &plan, &Lock::of(&c)?)
}

// spec: installer/SPEC.md §The consumer smoke — the re-seed leg: deleted seeds whose recorded hashes
// are stale are init's own fresh writes, recorded at what was written and never reported changed
pub(super) fn re_seed(state: &mut Run) -> Step {
    let c = consumer(state, "re-seed")?;
    let m = state.verb(&c, &["init", "--profile", PROFILE_DERIVED])?;
    if !m.succeeded() {
        return Err(failed(&m, "init failed on the re-seed consumer"));
    }
    let stale = hash_stdin(&c, "a prior version of this seed\n")?;
    let lock_path = format!("{}/checkwright.lock", c);
    let mut doc = Lock::read(&lock_path)?.value().clone();
    for p in &state.seeded {
        let _ = std::fs::remove_file(format!("{}/{}", c, p));
        if let Some(files) = doc.get_mut("files").and_then(|f| f.as_object_mut()) {
            files.insert(p.clone(), serde_json::Value::String(stale.clone()));
        }
    }
    let rendered = serde_json::to_string_pretty(&doc).map_err(|e| fail(format!("could not stale the recorded hashes: {}", e)))?;
    write(&lock_path, &format!("{}\n", rendered))?;
    commit_all(&c, "stale seeds", "could not commit the stale seeds")?;
    let plan = dry_plan(state, PROFILE_DERIVED, &c)?;
    let m = state.verb(&c, &["init", "--profile", PROFILE_DERIVED])?;
    if !m.succeeded() {
        return Err(failed(&m, "init failed re-seeding the re-seed consumer"));
    }
    let lock = Lock::of(&c)?;
    plan_parity(&format!("re-seeded [{} ]", state.seeded.join(" ")), &plan, &lock)?;
    let mut on = false;
    let mut changed = Vec::new();
    for line in lines::of("init's changed-file report", m.output()) {
        if line.contains("have changed since init wrote them") {
            on = true;
            continue;
        }
        if line.starts_with("  help:") {
            on = false;
        }
        if on {
            changed.push(line);
        }
    }
    for p in &state.seeded {
        if !is_file(&format!("{}/{}", c, p)) {
            return Err(fail(format!("re-seed: init did not re-seed the deleted {}", p)));
        }
        if lock.file(p) != hash_in(&c, p)? {
            return Err(fail(format!("re-seed: {} is recorded at a hash other than the one init just wrote", p)));
        }
        if changed.iter().any(|l| *l == format!("  {}", p)) {
            return Err(failed(&m, format!("re-seed: init reported its own re-seed of {} as a changed file", p)));
        }
    }
    say(&format!(
        "re-seed: {} deleted seed(s) re-seeded, recorded at what was written, none reported changed",
        state.seeded.len()
    ));
    Ok(())
}

fn hash_stdin(c: &str, body: &str) -> Result<String, Outcome> {
    let done = super::in_consumer_fed(&programs::GIT, &["hash-object", "--stdin"], &[], c, body.as_bytes())?;
    done.stdout()
        .map(|o| super::text(o).trim().to_string())
        .ok_or_else(|| refuse(format!("git hash-object --stdin failed: {}", done.failure_report().unwrap_or_default())))
}

// spec: installer/SPEC.md §The consumer smoke — the completion marker's parenthetical, stating the
// binary's provenance rather than asserting this run built one
pub(super) fn summary(state: &Run) -> String {
    let provenance = if state.handed.is_some() {
        "the gate binary adopted from the hand-off, unrebuilt"
    } else {
        "the gate binary this run runs as, staged unrebuilt"
    };
    format!(
        "{} profile(s) installed from the packed tarball with no registry access, each carrying {}, each put in front of a real prose defect (caught by {}) and each reversed back to its pre-init tree object, with gate rosters monotone across every comparable pair of the registries those installs wrote, plus the artifact-less {} leg driving a payload the packer itself produced with no artifact and asserting one refusal for init, doctor, diff and a bare invocation alike, naming the platform and writing nothing, the demo arm running checkwright demo green inside an installed consumer with its act 3 red read off a FAIL: line, the front-page proof block found in its output and that consumer unchanged and its scratch torn down, the companion arm applying each spec-toolkit recipe on its fixture tree green and catching each planted defect by its own gate, every complement line holding its left-out kits out, and the OpenSpec lifecycle layer firing check-stage-entry on a two-capability change and clearing on one, the extracted-tarball arm with node/npm masked and reversed the same way, the toolchain-free arm driving doctor and a full init with cargo/rustc masked, the jq-less arm asserting diff, uninstall and a lattice-minimum and a guard-kit init run clean with no jq on PATH, the installed guard hook answering a payload and doctor naming jq nowhere, the bash-less arm installing every profile that owes no bash and committing through its hooks clean and refused with no bash on PATH, the two-hop cross-version upgrade arm carrying the relinquish and re-add, the cross-version reversal arm reversing an unedited consumer back to its pre-init tree object after those same three hops, the newer-verb reversal arm reversing a first-version install by the next version with no upgrade, the hooked move arm committing a profile move and an upgrade shipping a gate-library change through the installed hooks while check-gate-tamper still refuses a gate edit co-staged with product code, the same-version seam arm and the protection branch chained onto it over a tampered gate binary, the narrowing arm re-running init at a smaller profile so files[] outlives kits, the selection arm adding a kit and a gate and dropping one at the minimum profile and clearing them with --no-selection, and the artifact arm driving the selection outcomes on a mutated copy of that payload, with its refusals asserted to differ in message and remedy and its fallback cases run wherever the hand-off carries a foreign-architecture preferred artifact, and the bootstrap's shasum fallback verifying and refusing with sha256sum masked wherever the host carries shasum",
        state.profiles.len(),
        provenance,
        state.value_red.join(" "),
        super::masked::BARE_PROFILE
    )
}

pub(super) fn commits_since(c: &str, before: usize) -> Result<bool, Outcome> {
    Ok(commits(c)? == before + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §init — the follow-up block is the indented run after `next:`, each
    // line's comment dropped, ending at the first line that is not indented
    #[test]
    fn the_follow_up_block_is_read_off_the_printed_grammar() {
        let out = "INIT: vendored\n\nnext:\n  ./scripts/x --install-hooks   # opt in\n  ./scripts/x --run   # battery\nafter\n  ./not-this\n";
        assert_eq!(followups(out), vec!["./scripts/x --install-hooks", "./scripts/x --run"]);
        assert!(followups("INIT: vendored\n").is_empty());
    }
}
