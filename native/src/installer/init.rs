// spec: installer/README.md §init — vendors the selected profile's kit source out of the package
// payload into the consumer's repository and commits it, so what governs their tree afterwards is
// committed, auditable source rather than something resolved at their build time.
use super::{lock, profile, recipe, refuse, Package, Refusal, AGENT_FILE, GATES_DIR, QUEUE_FILE};
use crate::{install, sha256, toolfloor};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

struct Flags {
    profile: String,
    dry: bool,
    force: bool,
    commit: bool,
}

fn help(pkg: Option<&Package>) {
    println!("usage: checkwright init [--profile <name>] [--dry-run] [--force] [--no-commit]\n");
    println!("Vendors pinned kit source into this repository and commits it.");
    println!("Nothing is fetched: the source comes from this package.\n");
    let names = match pkg {
        Some(p) => profile::names(&p.root).join(" "),
        None => String::new(),
    };
    println!("profiles: {} ", names);
}

fn parse(args: &[String], pkg: Option<&Package>) -> Result<Option<Flags>, Refusal> {
    let mut f = Flags {
        profile: String::new(),
        dry: false,
        force: false,
        commit: true,
    };
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "--profile" => {
                f.profile = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            _ if a.starts_with("--profile=") => {
                f.profile = a["--profile=".len()..].to_string();
                i += 1;
            }
            "--dry-run" => {
                f.dry = true;
                i += 1;
            }
            "--force" => {
                f.force = true;
                i += 1;
            }
            "--no-commit" => {
                f.commit = false;
                i += 1;
            }
            "-h" | "--help" => {
                help(pkg);
                return Ok(None);
            }
            other => return Err(refuse(format!("unknown argument: {}", other), "", 2)),
        }
    }
    Ok(Some(f))
}

pub fn run(args: &[String]) -> i32 {
    // spec: installer/README.md §The verbs — `--help` answers before every precondition, including
    // the package's own: an adopter asking what the verb takes is not asking for an install.
    let pkg = super::package(
        "init copies kit source",
    );
    let parsed = match parse(args, pkg.as_ref().ok()) {
        Ok(None) => return 0,
        Ok(Some(f)) => f,
        Err(r) => return super::finish("init", Err(r)),
    };
    let pkg = match pkg {
        Ok(p) => p,
        Err(r) => return super::finish("init", Err(r)),
    };
    super::finish("init", vendor(&pkg, &parsed))
}

// spec: installer/README.md §The manifest — a recorded hash is what init last wrote at that path,
// so an entry init did not write this run carries its hash forward verbatim: hashing the tree at
// emit time would file the adopter's own content as init's and let the next run write through it.
struct Roster {
    written: Vec<String>,
    is_written: BTreeSet<String>,
    carried: BTreeMap<String, String>,
    changed: Vec<String>,
}

impl Roster {
    fn new() -> Roster {
        Roster {
            written: Vec::new(),
            is_written: BTreeSet::new(),
            carried: BTreeMap::new(),
            changed: Vec::new(),
        }
    }

    // spec: installer/README.md §The manifest — membership in the written set is held as a key
    // rather than re-derived by scanning the list, and the key is written where the path is
    // recorded so the two cannot part company.
    fn record(&mut self, path: &str, carried: Option<&str>) {
        self.written.push(path.to_string());
        self.is_written.insert(path.to_string());
        if let Some(h) = carried {
            self.carried.insert(path.to_string(), h.to_string());
        }
    }
}

// spec: installer/README.md §init — the non-destructive re-run: a file still at its recorded hash
// is init's to rewrite, one changed since is the adopter's and is reported instead.
// spec: installer/README.md §The manifest — the carry-forward belongs to the refusal rather than to
// each caller: this is the single point where the roster would otherwise lose the path, and absence
// of a key reads as "never installed" on the next run.
fn claim(root: &Path, rel: &str, prior: &BTreeMap<String, String>, force: bool, r: &mut Roster) -> bool {
    let Some(want) = prior.get(rel) else {
        return true;
    };
    let file = root.join(rel);
    if !file.is_file() {
        return true;
    }
    if lock::hash(&file).unwrap_or_default() == *want || force {
        return true;
    }
    r.changed.push(rel.to_string());
    r.record(rel, Some(want));
    false
}

fn copy_in(
    root: &Path,
    src: &Path,
    dest: &str,
    prior: &BTreeMap<String, String>,
    f: &Flags,
    r: &mut Roster,
) -> Result<(), Refusal> {
    if !claim(root, dest, prior, f.force, r) {
        return Ok(());
    }
    if !f.dry {
        let target = root.join(dest);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| refuse(format!("could not write {}: {}", dest, e), "", 2))?;
        }
        std::fs::copy(src, &target)
            .map_err(|e| refuse(format!("could not write {}: {}", dest, e), "", 2))?;
    }
    r.record(dest, None);
    Ok(())
}

// spec: installer/README.md §What init seeds — a starting-roster member is written by name and
// nothing else: the install-time omission retired with the bootstrap's one success path (§The gate
// binary), so no reason token is resolved and no second roster of ported gates is maintained.
fn plan_gates(pkg: &Package, kits: &[String], profile_name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# Checkwright gate registry — written by 'checkwright init' (profile: {}).\n",
        profile_name
    ));
    out.push_str("# Each kit's starting subset; its README names the full roster to grow into.\n");
    // spec: installer/README.md §Profiles — the gate set is derived once, by the function the
    // smoke's monotonicity assertion also reads, so the registry and the invariant over it share
    // one derivation. The loop only sections it by kit; a member several kits register lands once.
    let mut pending: BTreeSet<String> = profile::gate_set(&pkg.root, profile_name)
        .into_iter()
        .collect();
    for kit in kits {
        let members = recipe::gates(&pkg.payload.join(kit), profile_name);
        if members.is_empty() {
            continue;
        }
        out.push_str(&format!("# {}\n", kit));
        for m in members {
            if pending.remove(&m) {
                out.push_str(&m);
                out.push('\n');
            }
        }
    }
    out
}

fn read_package_field(pkg: &Package, path: &[&str]) -> String {
    let Ok(text) = std::fs::read_to_string(pkg.root.join("package.json")) else {
        return String::new();
    };
    let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else {
        return String::new();
    };
    let mut cur = &doc;
    for step in path {
        match cur.get(step) {
            Some(v) => cur = v,
            None => return String::new(),
        }
    }
    cur.as_str().unwrap_or_default().to_string()
}

fn vendor(pkg: &Package, f: &Flags) -> Result<i32, Refusal> {
    // spec: installer/README.md §init — the three preconditions all refuse rather than warn, and
    // all three are checked before any file is written: a partial install is the outcome none of
    // them may produce.
    let root = super::repo_root().ok_or_else(|| {
        refuse(
            "not inside a git work tree",
            "the vendored source is meant to be committed, which is what makes it auditable. Run 'git init' first, or run init inside the repository you want governed.",
            2,
        )
    })?;

    // spec: installer/README.md §init — the clean-worktree precondition exists so the one commit
    // init makes is exactly what it vendored; --no-commit is its valve, because an operator staging
    // the vendoring themselves has taken that guarantee on.
    if !f.dry && f.commit {
        let dirty = super::git_capture(&root, &["status", "--porcelain"]).unwrap_or_default();
        if !dirty.trim().is_empty() {
            return Err(refuse(
                "the worktree is not clean",
                "init makes one commit, and a dirty tree would fold your work into it — so a reviewer's diff would no longer be the whole of what was vendored. Commit or stash first, or pass --no-commit to stage the vendoring yourself.",
                1,
            ));
        }
    }

    let version = read_package_field(pkg, &["version"]);
    let commit = read_package_field(pkg, &["checkwright", "commit"]);
    if version.is_empty() {
        return Err(refuse(
            "this package carries no version stamp",
            "the version is stamped at pack time from the release tag; a package without one was not assembled by the pack step.",
            2,
        ));
    }

    let lock_path = lock::path(&root);
    let mut prior: BTreeMap<String, String> = BTreeMap::new();
    let mut profile_name = f.profile.clone();
    if lock_path.is_file() {
        let manifest = lock::Manifest::read(&lock_path)
            .filter(lock::Manifest::schema_ok)
            .ok_or_else(|| {
                refuse(
                    format!("{} carries a schema this build does not know", lock::FILE),
                    "this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for.",
                    2,
                )
            })?;
        let prior_version = manifest.field("version");
        // spec: installer/README.md §The manifest — the version field's re-run reader: a payload
        // older than the recorded install is a silent downgrade, so it refuses, and `--force` is
        // what makes a rollback deliberate.
        // spec: context-kit/SPEC.md §bin/env-probe — the comparator is the crate's one `sort -V`
        // holder, so the install path and the floor predicate cannot disagree about version order.
        let downgrading = !f.force
            && !prior_version.is_empty()
            && prior_version != version
            && toolfloor::floor_met(&prior_version, &version) == Some(false);
        if downgrading {
            return Err(refuse(
                format!(
                    "this package is {} but {} records {} — refusing a silent downgrade",
                    version,
                    lock::FILE,
                    prior_version
                ),
                "run the release you already have, or pass --force if rolling back is what you meant.",
                1,
            ));
        }
        if profile_name.is_empty() {
            profile_name = manifest.field("profile");
        }
        prior = manifest.files().into_iter().collect();
    }

    if profile_name.is_empty() {
        profile_name = "starter".to_string();
    }
    if !profile::known(&pkg.root, &profile_name) {
        return Err(refuse(
            format!("unknown profile: {}", profile_name),
            format!(
                "selectable profiles: {} ",
                profile::names(&pkg.root).join(" ")
            ),
            2,
        ));
    }

    // spec: installer/README.md §init — doctor is the last precondition and still runs before any
    // file is written; running it after the manifest and profile are resolved keeps a bad manifest
    // from being reported as a toolchain fault.
    let verdict = super::doctor::diagnose();
    if verdict.code != 0 {
        eprint!("{}{}", verdict.out, verdict.err);
        return Err(refuse(
            "the toolchain is below contract — refusing to install",
            "the floors above are what the gate battery needs to run, so installing first would leave you a vendored tree that cannot be checked. Fix them and re-run.",
            verdict.code,
        ));
    }

    let kits = profile::kits(&pkg.root, &profile_name);
    if kits.is_empty() {
        return Err(refuse(
            format!("profile '{}' resolves to no kit in this payload", profile_name),
            "every kit a profile names must exist in the package payload; this one names none that do.",
            2,
        ));
    }

    // spec: installer/README.md §The install boundary — the artifact is the binary this arm runs
    // from: the bootstrap resolved and verified it at step 4, so nothing here re-selects a platform
    // and nothing hashes with an external tool.
    let artifact_digest = sha256::file_hex(&pkg.artifact)
        .map_err(|e| refuse(format!("could not hash the gate binary: {}", e), "", 2))?;
    let artifact_name = pkg
        .artifact
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let artifact_dest = format!("{}/{}", GATES_DIR, artifact_name);

    let mut r = Roster::new();

    for kit in &kits {
        let kit_payload = pkg.payload.join(kit);
        if !kit_payload.is_dir() {
            return Err(refuse(
                format!("profile '{}' names {}, which this payload does not carry", profile_name, kit),
                "the payload's kit set is derived from the source tree at pack time; a profile naming a kit that is not there is a roster that has drifted.",
                2,
            ));
        }
        // spec: installer/README.md §What init seeds — the count is asserted rather than trusted:
        // a failed enumeration and an empty kit reach this loop identically, so the next construct
        // that fails on an untested host would again install a tree silently missing a kit.
        let files = super::files_under(&kit_payload).map_err(|e| refuse(e, "", 2))?;
        if files.is_empty() {
            return Err(refuse(
                format!("the payload carries {} but enumerating its files produced nothing", kit),
                format!("a kit directory this payload ships is never empty, so either the package is damaged or this host refused the walk. Re-install the package; if it still refuses, report what this prints: ls -R {}", kit_payload.display()),
                2,
            ));
        }
        for rel in files {
            copy_in(&root, &kit_payload.join(&rel), &format!("{}/{}", kit, rel), &prior, f, &mut r)?;
        }
    }

    let _ = std::fs::create_dir_all(root.join(GATES_DIR));
    let _ = std::fs::create_dir_all(root.join(".workflow"));
    let gates_list = format!("{}/gates.list", GATES_DIR);
    if claim(&root, &gates_list, &prior, f.force, &mut r) {
        if !f.dry {
            std::fs::write(root.join(&gates_list), plan_gates(pkg, &kits, &profile_name))
                .map_err(|e| refuse(format!("could not write {}: {}", gates_list, e), "", 2))?;
        }
        r.record(&gates_list, None);
    }

    for kit in &kits {
        let kit_payload = pkg.payload.join(kit);
        for (src, dest) in recipe::config_seam_plan(&kit_payload, GATES_DIR) {
            copy_in(&root, Path::new(&src), &dest, &prior, f, &mut r)?;
        }
        // spec: installer/README.md §init — `--dry-run` resolves the same seam plan the real run
        // would, out of the same enumerator: the copy already writes nothing, so the plan needs no
        // dry variant. What it cannot do is *seed*, so the seeded set is predicted below.
        // spec: installer/README.md §What init seeds — the agent file is predicted once over the
        // whole kit set rather than once per kit that needs it, because the seeding arm below is
        // guarded on the file's absence and therefore fires at most once for a run.
        let seeds_agent = recipe::needs_agent_file(kit)
            && !root.join(AGENT_FILE).is_file()
            && !r.is_written.contains(AGENT_FILE);
        if f.dry {
            let mut planned = dry_seed_paths(kit);
            if seeds_agent {
                planned.insert(0, AGENT_FILE.to_string());
            }
            for rel in planned {
                if claim(&root, &rel, &prior, f.force, &mut r) {
                    r.record(&rel, None);
                }
            }
            continue;
        }
        if seeds_agent {
            // spec: installer/README.md §What init seeds — the seeded agent file carries the
            // section heading context-kit's brevity gate reads by default, so the gate init
            // registers has the surface it was pointed at from the first commit.
            let body = format!(
                "# {}\n\nResident instructions for agent sessions in this repository.\n\n## Shared conventions\n\n- **Terse:** one line per rule here; the mechanism behind the pointer.\n",
                AGENT_FILE
            );
            std::fs::write(root.join(AGENT_FILE), body)
                .map_err(|e| refuse(format!("could not seed {}: {}", AGENT_FILE, e), "", 2))?;
            if claim(&root, AGENT_FILE, &prior, f.force, &mut r) {
                r.record(AGENT_FILE, None);
            }
        }
        for seeded in recipe::seed(kit, &kit_payload, &root)
            .map_err(|e| refuse(e, "", 2))?
        {
            match seeded {
                recipe::Seeded::Path(p) => {
                    if claim(&root, &p, &prior, f.force, &mut r) {
                        r.record(&p, None);
                    }
                }
                recipe::Seeded::Plan(src, dest) => {
                    copy_in(&root, Path::new(&src), &dest, &prior, f, &mut r)?
                }
            }
        }
    }

    // spec: installer/README.md §What init seeds — the queue seed runs here, once, over the whole
    // resolved kit set: inside the loop above the first kit reached decided the source before any
    // kit shipping a template got a turn. One resolver serves the dry plan and the run alike.
    if let Some(src) = recipe::queue_source(&pkg.payload, &kits) {
        if !root.join(QUEUE_FILE).is_file() {
            if !f.dry {
                recipe::write_queue(&src, &root, QUEUE_FILE).map_err(|e| refuse(e, "", 2))?;
            }
            if claim(&root, QUEUE_FILE, &prior, f.force, &mut r) {
                r.record(QUEUE_FILE, None);
            }
        }
    }

    // spec: installer/README.md §The gate binary — the write comes after every config seam and
    // before the hook is generated: the generator resolves each member's argv and a `.gate` member
    // resolves to this binary, so the knob must name it and the file must be there.
    // spec: installer/README.md §The install boundary — the placement is the `--install
    // place-artifact` op called in-process: one derivation, two callers, this arm and that flag.
    let seam = format!("{}/gate-sdk-config.sh", GATES_DIR);
    let src = pkg.artifact.to_string_lossy().into_owned();
    let placement = install::Placement {
        root: root.clone(),
        src: &src,
        dest: &artifact_dest,
        seam: &seam,
        target: &pkg.target,
        digest: &artifact_digest,
        force: f.force,
        dry: f.dry,
    };
    let recorded = install::Recorded::read(&lock_path).map_err(|e| refuse(e, "", 2))?;
    let placed = install::place(&placement, &recorded)
        .map_err(|e| refuse(format!("the gate binary could not be placed: {}", e), "", 2))?;
    for line in placed {
        let mut fields = line.split('\t');
        match (fields.next(), fields.next(), fields.next()) {
            (Some("own"), Some(p), _) => {
                if !r.is_written.contains(p) {
                    r.record(p, None);
                }
            }
            (Some("kept"), Some(p), h) => {
                r.changed.push(p.to_string());
                if !r.is_written.contains(p) {
                    r.record(p, h);
                }
            }
            _ => {}
        }
    }

    // spec: installer/README.md §init — the generated projections are produced by the vendored
    // tools themselves, never restated by the installer: the hook generator and the graph emitter
    // are gate-sdk's, so a consumer's artifacts are the ones their own gate-sdk makes.
    let mut generated = vec![
        format!("{}/git-hooks/pre-commit", GATES_DIR),
        format!("{}/CHECK-GRAPH.html", GATES_DIR),
    ];
    if !f.dry {
        run_vendored(&root, "gate-sdk/bin/gen-pre-commit.sh", &["--write"], None)?;
        run_vendored(
            &root,
            "gate-sdk/bin/run-gates.sh",
            &["--emit", "graph"],
            Some(&root.join(GATES_DIR).join("CHECK-GRAPH.html")),
        )?;
    }
    if root.join(format!("{}/git-hooks/commit-msg", GATES_DIR)).is_file() {
        generated.push(format!("{}/git-hooks/commit-msg", GATES_DIR));
    }
    for g in &generated {
        if !f.dry && !root.join(g).is_file() {
            continue;
        }
        r.record(g, None);
    }

    // spec: installer/README.md §The manifest — the roster's exit condition, and the whole rule:
    // init owns a path because it wrote the file there, so ownership ends when the file leaves the
    // tree and at no other moment — a payload that stops shipping one is not that moment.
    for (p, h) in &prior {
        if r.is_written.contains(p) || !root.join(p).is_file() {
            continue;
        }
        r.record(p, Some(h));
    }

    // spec: installer/README.md §The manifest — what init wrote this run is a subset of the roster
    // it records, and staging takes the written set: folding an adopter's file into the vendoring
    // commit is what the clean-worktree precondition exists to prevent.
    let mut stage: Vec<String> = r
        .written
        .iter()
        .filter(|p| !r.carried.contains_key(*p))
        .cloned()
        .collect();

    let manifest_text = |r: &Roster| -> String {
        let mut e = lock::Emit::new()
            .ident("version", &version)
            .ident("profile", &profile_name)
            .ident("kits", &kits.join(" "));
        // spec: installer/README.md §The manifest — an identity field is present exactly when the
        // caller supplied it; an empty commit written as "" would be a placeholder standing in for
        // an omission.
        if !commit.is_empty() {
            e = e.ident("commit", &commit);
        }
        e = e.artifact(&pkg.target, &artifact_digest);
        for p in &r.written {
            let h = match r.carried.get(p) {
                Some(h) => h.clone(),
                None if f.dry && !root.join(p).is_file() => "(pending)".to_string(),
                None => lock::hash(&root.join(p)).unwrap_or_default(),
            };
            e = e.file(p, &h);
        }
        e.render()
    };

    if f.dry {
        println!(
            "checkwright init --dry-run (profile: {}, version: {})\n",
            profile_name, version
        );
        println!("would vendor {} kit(s): {}", kits.len(), kits.join(" "));
        println!("would write {} file(s), including:", stage.len() + 1);
        println!("  {}", gates_list);
        println!("  {}", lock::FILE);
        for kit in &kits {
            let files = super::files_under(&pkg.payload.join(kit)).unwrap_or_default();
            println!("  {}/ ({} files)", kit, files.len());
        }
        println!(
            "\nwould place the {} gate binary at {} (digest verified against the payload sidecar)",
            pkg.target, artifact_dest
        );
        if !r.changed.is_empty() {
            println!(
                "\nwould leave {} changed file(s) alone (--force to overwrite):",
                r.changed.len()
            );
            for p in &r.changed {
                println!("  {}", p);
            }
        }
        println!("\n{} that would be written:", lock::FILE);
        print!("{}", manifest_text(&r));
        println!("\nDRY RUN: nothing was written.");
        return Ok(0);
    }

    std::fs::write(&lock_path, manifest_text(&r))
        .map_err(|e| refuse(format!("could not write {}: {}", lock::FILE, e), "", 2))?;
    r.written.push(lock::FILE.to_string());
    stage.push(lock::FILE.to_string());

    if !r.changed.is_empty() {
        println!(
            "\n{} file(s) have changed since init wrote them and were left alone:",
            r.changed.len()
        );
        for p in &r.changed {
            println!("  {}", p);
        }
        println!("  help: review the differences; re-run with --force to take the packaged version.\n");
    }

    super::git_batched(&root, &["add"], &stage)
        .map_err(|e| refuse(format!("could not stage the vendored files: {}", e), "", 2))?;

    // spec: installer/README.md §init — idempotence is a property of the tree, so a re-run that
    // changed nothing exits clean rather than on an empty commit. The predicate reads the index and
    // stays the guard on whether a commit is attempted at all.
    let message = format!(
        "chore: vendor Checkwright kits ({} profile, v{})",
        profile_name, version
    );
    if super::git_code(&root, &["diff", "--cached", "--quiet"]) == Some(0) {
        // spec: installer/README.md §init — a run init considers a no-op still commits what it
        // rewrote, so this asks what the predicate above cannot: whether the roster differs from the
        // committed tree. `--no-commit` is exempt, having waived the attribution it rests on.
        if f.commit {
            let residue_status = super::git_batched(&root, &["status", "--porcelain"], &r.written)
                .map_err(|e| refuse(format!("could not read the install's own status: {}", e), "", 2))?;
            if !String::from_utf8_lossy(&residue_status).trim().is_empty() {
                super::git_batched(&root, &["add"], &r.written).map_err(|_| {
                    refuse(
                        "could not stage the files init rewrote but left out of the vendoring commit",
                        "",
                        2,
                    )
                })?;
                if super::git_code(&root, &["diff", "--cached", "--quiet"]) != Some(0) {
                    let listed = super::git_capture(&root, &["diff", "--cached", "--name-only"])
                        .unwrap_or_default();
                    let residue = listed.lines().filter(|l| !l.is_empty()).count();
                    if super::git_code(&root, &["commit", "-q", "-m", &message]) != Some(0) {
                        return Err(refuse(
                            "the commit failed — the files are staged; commit them yourself to finish the install",
                            "",
                            1,
                        ));
                    }
                    println!(
                        "INIT: already at the {} profile (v{}) — {} file(s) checked; {} that init had rewritten were uncommitted and are now committed.",
                        profile_name, version, r.written.len(), residue
                    );
                    return Ok(0);
                }
            }
        }
        println!(
            "INIT: already at the {} profile (v{}) — {} file(s) checked, nothing to change and nothing it rewrote left uncommitted.",
            profile_name, version, r.written.len()
        );
        return Ok(0);
    }

    if f.commit {
        // spec: installer/README.md §init — the commit is the distribution model, not a
        // convenience: vendored-and-committed is what makes the tree auditable, so leaving it dirty
        // would hand the adopter the step that does the proving.
        if super::git_code(&root, &["commit", "-q", "-m", &message]) != Some(0) {
            return Err(refuse(
                "the commit failed — the files are staged; commit them yourself to finish the install",
                "",
                1,
            ));
        }
        println!(
            "INIT: vendored {} kit(s) at the {} profile (v{}) and committed them.",
            kits.len(),
            profile_name,
            version
        );
    } else {
        println!(
            "INIT: vendored {} kit(s) at the {} profile (v{}) and staged them — --no-commit, so the commit is yours.",
            kits.len(),
            profile_name,
            version
        );
    }

    // spec: installer/README.md §init — the follow-up block is a STATED GRAMMAR with a named
    // reader, not a layout choice: the consumer smoke parses it out of what this prints, so
    // reflowing these lines reds that arm instead of silently un-covering the pair.
    println!("\nnext:");
    println!("  bash gate-sdk/bin/run-gates.sh --install-hooks   # opt this clone into the generated pre-commit hook");
    println!("  bash gate-sdk/bin/run-gates.sh       # the battery, green on what was just vendored");
    Ok(0)
}

// spec: installer/README.md §What init seeds — the dry plan names the surfaces the recipe would
// seed, read off the same membership predicates the seeding arms key on rather than a second
// roster: a kit gains a seeded surface by gaining an arm, and both sides read the same fact.
fn dry_seed_paths(kit: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    match kit {
        "gate-sdk" => out.push(format!("{}/msg-patterns.list", GATES_DIR)),
        "evidence-kit" => {
            out.push(".workflow/validate-baseline.txt".to_string());
            out.push(".workflow/validate-evidence.txt".to_string());
        }
        "lifecycle-kit" => out.push(".workflow/WORKFLOW-STATE.txt".to_string()),
        _ => {}
    }
    out
}

// spec: installer/README.md §init — the vendored tool runs out of the consumer's own tree, so the
// artifacts a consumer ends up with are the ones their own gate-sdk makes rather than ones this
// arm restated.
fn run_vendored(
    root: &Path,
    script: &str,
    args: &[&str],
    capture_to: Option<&Path>,
) -> Result<(), Refusal> {
    let script_path = root.join(script).to_string_lossy().into_owned();
    let mut argv: Vec<&str> = vec![&script_path];
    argv.extend_from_slice(args);
    let out = crate::proc::run_merged_in("bash", &argv, &[], Some(root))
        .map_err(|e| refuse(format!("{} failed: {}", script, e), "", 2))?;
    if !out.succeeded() {
        eprintln!("{}", String::from_utf8_lossy(out.output()));
        return Err(refuse(format!("{} failed", script), "", 2));
    }
    if let Some(dest) = capture_to {
        if let Some(parent) = dest.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(dest, out.output())
            .map_err(|e| refuse(format!("could not write {}: {}", dest.display(), e), "", 2))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/README.md §The verbs — the flag grammar: both `--profile` spellings, the three
    // bare flags, `--help` answering on its own, and an unknown argument refusing.
    #[test]
    fn the_flag_grammar_is_the_verbs_own_and_an_unknown_token_refuses() {
        let parsed = parse(
            &[
                "--profile=full".to_string(),
                "--dry-run".to_string(),
                "--force".to_string(),
                "--no-commit".to_string(),
            ],
            None,
        )
        .expect("the declared flags were refused")
        .expect("--help was taken for a flag");
        assert_eq!(parsed.profile, "full");
        assert!(parsed.dry && parsed.force && !parsed.commit);

        let spaced = parse(&["--profile".to_string(), "starter".to_string()], None)
            .expect("the spaced spelling was refused")
            .expect("--help was taken for a flag");
        assert_eq!(spaced.profile, "starter");
        assert!(parse(&["--nope".to_string()], None).is_err());
        assert!(parse(&["--help".to_string()], None).expect("help refused").is_none());
    }

    // spec: installer/README.md §What init seeds — the registry a fresh consumer receives carries
    // each kit's own zero-config members under that kit's heading, once each, and no omission
    // record: the install-time omission retired with the bootstrap's one success path.
    #[test]
    fn the_registry_sections_by_kit_and_records_no_omission() {
        let dir = std::env::temp_dir().join(format!("cw-init-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        for kit in ["a-kit", "b-kit"] {
            std::fs::create_dir_all(dir.join("payload").join(kit).join("checks"))
                .expect("cannot make the scratch tree");
        }
        std::fs::write(dir.join("package.json"), "{}").expect("cannot write the scratch package");
        std::fs::write(dir.join("profiles.list"), "small\ta-kit\nsmall\tb-kit\n")
            .expect("cannot write the scratch roster");
        // spec: gate-sdk/SPEC.md §check-install-disposition — the fixture names are composed for
        // the reason the recipe module's own tests compose theirs: the derivation must carry no
        // literal gate name, and a corpus spelling one out reads to that assertion as a roster.
        let one = format!("{}-one.gate", "check");
        let two = format!("{}-two.gate", "check");
        for (kit, name) in [("a-kit", &one), ("b-kit", &two), ("b-kit", &one)] {
            std::fs::write(
                dir.join("payload").join(kit).join("checks").join(name),
                "# install: zero-config\n",
            )
            .expect("cannot write a scratch gate");
        }

        let pkg = Package {
            root: dir.clone(),
            payload: dir.join("payload"),
            artifact: dir.join("payload/artifact/t/checkwright-gates"),
            target: "t".to_string(),
        };
        let text = plan_gates(&pkg, &["a-kit".to_string(), "b-kit".to_string()], "small");
        let stem = one.trim_end_matches(".gate");
        assert!(!text.contains("# omitted:"), "the registry recorded an omission");
        assert_eq!(text.matches(stem).count(), 1, "a shared member landed twice");
        let a = text.find("# a-kit").expect("no a-kit section");
        let at = text.find(&format!("\n{}", stem)).expect("the shared member is missing");
        let b = text.find("# b-kit").expect("no b-kit section");
        assert!(a < at && at < b, "a shared member left its first kit's section");
        std::fs::remove_dir_all(&dir).ok();
    }
}
