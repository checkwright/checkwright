// spec: installer/SPEC.md §The consumer smoke — the artifact-less refusal leg and the masked arms:
// the download transport with node/npm masked, the toolchain-free arm, and the jq-less and
// bash-less arms masked by absence
use super::consumer::{
    consumer, entry_run, exists, extract, failed, find_line, invoking_path, is_file, mkdir, out, path_set, porcelain,
    resolves, run_line, seam_of, tree, write, GATES_DIR, PROFILE_DERIVED,
};
use super::profiles::{assert_install, followups, profile_kits, reversal};
use super::{fail, in_consumer_fed, lines, merged_in, refuse, say, show, text, Entry, Outcome, Run, Step};
use crate::{programs, walk};
use std::path::Path;

// spec: installer/SPEC.md §The consumer smoke — the profile the artifact-less leg is scoped to, a
// choice of which invocation to make, re-scoped on a declared profile and never dropped
pub(super) const BARE_PROFILE: &str = "prose";

const NO_TARGET: &str = "maps to no target this payload declares";
const NO_ACTION: &str = "no adopter action to take";

// spec: installer/SPEC.md §The consumer smoke — the refusal is asserted on the consumer, not only on
// the exit code: a refusal that wrote first would exit non-zero too
pub(super) fn untouched(c: &str, seed: &str) -> Result<bool, Outcome> {
    Ok(tree(c)? == seed && porcelain(c)?.is_empty() && !is_file(&format!("{}/checkwright.lock", c)))
}

// spec: installer/SPEC.md §The consumer smoke — the artifact-less leg drives the packer's own
// artifact-free output: the license placement read off it, then one bootstrap refusal for init,
// doctor, diff and a bare invocation, naming the platform and writing nothing
pub(super) fn artifact_less(state: &mut Run) -> Step {
    if !state.profiles.iter().any(|p| p == BARE_PROFILE) {
        return Err(fail(format!(
            "the artifact-less leg is scoped to '{}' and the payload declares [{}] — re-scope it on a declared profile, never drop the leg",
            BARE_PROFILE,
            state.profiles.join(" ")
        )));
    }
    let bare = format!("{}/bare", state.scratch);
    mkdir(&bare)?;
    let tarball = super::staging::pack_one(state, &bare, &state.version, false, "artifact-less ", "the artifact-less pack step failed.")?;
    extract(&tarball, &bare)?;
    if exists(&format!("{}/package/payload/artifact", bare)) {
        return Err(fail(
            "the artifact-less payload carries an artifact directory — the leg would assert a refusal against a payload with something to run",
        ));
    }
    let license = std::fs::read(format!("{}/LICENSE", state.root)).unwrap_or_default();
    if std::fs::read(format!("{}/package/LICENSE", bare)).ok() != Some(license.clone()) {
        return Err(fail(
            "the packed package carries no LICENSE equal to the repository's — the packer did not place the license text at the package root",
        ));
    }
    let mut licensed = 0usize;
    let payload = format!("{}/package/payload", bare);
    for (kit, _) in walk::list_dir(Path::new(&payload)).unwrap_or_default().into_iter().filter(|(_, d)| *d) {
        if !is_file(&format!("{}/{}/README.md", payload, kit)) {
            continue;
        }
        if std::fs::read(format!("{}/{}/LICENSE", payload, kit)).ok() != Some(license.clone()) {
            return Err(fail(format!(
                "the packed kit {} carries no LICENSE equal to the repository's — the packer did not place the license text in it",
                kit
            )));
        }
        licensed += 1;
    }
    if licensed == 0 {
        return Err(fail("the packed payload carries no kit README, so the license assertion checked nothing"));
    }
    say(&format!("license: the package root and {} packed kit root(s) carry the repository's LICENSE", licensed));
    let c = consumer(state, "artifact-less")?;
    let seed = tree(&c)?;
    let entry = Entry::Extracted(format!("{}/package", bare));
    let m = entry_run(&entry, &c, &["init", "--profile", BARE_PROFILE], &[])?;
    if m.succeeded() {
        return Err(failed(&m, "a payload the packer produced with no artifact directory installed anyway — selection has one success path, and this is not it"));
    }
    if !out(&m).contains(NO_TARGET) {
        return Err(failed(&m, "the artifact-less payload refused without naming the platform as the thing it carries nothing for"));
    }
    if !out(&m).contains(NO_ACTION) {
        return Err(failed(&m, "the artifact-less refusal carries no remedy line, so an adopter cannot tell a platform they can do nothing about from a payload they should re-download"));
    }
    if !untouched(&c, &seed)? {
        return Err(fail("the artifact-less refusal left the consumer changed — it refused after writing something, not before"));
    }
    say("artifact-less payload: refused naming the platform, with a remedy, and nothing written");
    for argv in [&["doctor"][..], &["diff"], &[]] {
        let m = entry_run(&entry, &c, argv, &[])?;
        if m.succeeded() || !out(&m).contains(NO_TARGET) {
            return Err(failed(&m, format!(
                "'checkwright {}' on the artifact-less payload answered differently from init — the refusal is the bootstrap's and precedes every verb",
                argv.join(" ")
            )));
        }
    }
    say("the same refusal answers doctor, diff and a bare invocation — it is the bootstrap's, not a verb's");
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — a reach mask: on a unix host a PATH-first directory
// of shims that exit non-zero naming themselves, so a latent reach fails loudly
#[cfg(unix)]
fn reach_mask(dir: &str, names: &[&str], arm: &str, what: &str) -> Result<String, Outcome> {
    use std::os::unix::fs::PermissionsExt;
    mkdir(dir)?;
    for n in names {
        let shim = format!("{}/{}", dir, n);
        write(&shim, &format!("#!/bin/sh\necho \"{}: {} was reached — {}\" >&2\nexit 127\n", arm, n, what))?;
        std::fs::set_permissions(&shim, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| fail(format!("could not make the shim {} executable: {}", shim, e)))?;
    }
    Ok(format!("{}:{}", dir, invoking_path()))
}

// spec: installer/SPEC.md §The consumer smoke — on native Windows the directories holding a masked
// program leave PATH, so the program resolves to nothing
#[cfg(not(unix))]
fn reach_mask(_dir: &str, names: &[&str], _arm: &str, _what: &str) -> Result<String, Outcome> {
    dropped_path(names)
}

// spec: installer/SPEC.md §The consumer smoke — the mask is proved rather than assumed: each masked
// name resolves to its shim, or on native Windows to nothing
fn prove_reach(dir: &str, names: &[&str], path: &str, arm: &str) -> Step {
    for n in names {
        let got = resolves(n, path);
        let want = cfg!(unix).then(|| format!("{}/{}", dir, n));
        if got != want {
            return Err(fail(format!(
                "the mask did not take: {} resolves to '{}', not {}",
                n,
                got.unwrap_or_else(|| "nothing".to_string()),
                want.map_or_else(|| "nothing".to_string(), |w| format!("the shim at {}", w))
            )));
        }
    }
    prove_control(&programs::GIT, &["--version"], path, arm, &names.join("/"))
}

// spec: installer/SPEC.md §The consumer smoke — the exclusion is the masked program's case-folded
// stem, so no suffix is named
pub(super) fn stem_is(stem: &str, name: &str) -> bool {
    name.split('.').next().unwrap_or(name).to_lowercase() == stem
}

// spec: installer/SPEC.md §The consumer smoke — an absence mask derived from the live PATH: a
// directory carrying the masked program is replaced by a farm of links to its other programs, and
// every other directory kept verbatim after it
#[cfg(unix)]
pub(super) fn path_without(stem: &str, farm: &str) -> Result<String, Outcome> {
    mkdir(farm)?;
    let mut keep: Vec<String> = Vec::new();
    for d in std::env::split_paths(&invoking_path()).filter(|d| d.is_dir()) {
        let names: Vec<String> = walk::list_dir(&d)
            .unwrap_or_default()
            .into_iter()
            .map(|(n, _)| n)
            .filter(|n| !n.starts_with('.'))
            .collect();
        if !names.iter().any(|n| stem_is(stem, n)) {
            keep.push(d.display().to_string());
            continue;
        }
        for n in names.iter().filter(|n| !stem_is(stem, n)) {
            let (from, to) = (d.join(n), Path::new(farm).join(n));
            if crate::proc::is_executable(&from) && !to.exists() {
                let _ = std::os::unix::fs::symlink(&from, &to);
            }
        }
    }
    Ok(std::iter::once(farm.to_string()).chain(keep).collect::<Vec<_>>().join(":"))
}

// spec: installer/SPEC.md §The consumer smoke — on native Windows the directories holding the
// masked program leave PATH, and git is re-added alone through Git's `cmd` directory
#[cfg(not(unix))]
pub(super) fn path_without(stem: &str, _farm: &str) -> Result<String, Outcome> {
    dropped_path(&[stem])
}

// spec: installer/SPEC.md §The consumer smoke — the Windows column's PATH: each directory holding a
// masked program dropped, the system directory spared, and each re-added directory appended
#[cfg_attr(unix, allow(dead_code))]
pub(super) fn dropping(
    dirs: &[std::path::PathBuf],
    holds: impl Fn(&Path) -> bool,
    spared: impl Fn(&Path) -> bool,
    readd: &[std::path::PathBuf],
) -> Vec<std::path::PathBuf> {
    let mut kept: Vec<std::path::PathBuf> = dirs.iter().filter(|d| spared(d) || !holds(d)).cloned().collect();
    for r in readd {
        if !kept.contains(r) {
            kept.push(r.clone());
        }
    }
    kept
}

#[cfg(not(unix))]
fn dropped_path(names: &[&str]) -> Result<String, Outcome> {
    let dirs: Vec<std::path::PathBuf> = std::env::split_paths(&invoking_path()).collect();
    let root = std::env::var("SystemRoot").unwrap_or_default();
    let holds = |d: &Path| {
        walk::list_dir(d)
            .unwrap_or_default()
            .iter()
            .any(|(n, dir)| !dir && names.iter().any(|s| stem_is(s, n)))
    };
    let spared = |d: &Path| !root.is_empty() && crate::proc::is_windows_system_dir(d, &root);
    let join = |kept: &[std::path::PathBuf]| -> Result<String, Outcome> {
        std::env::join_paths(kept)
            .map(|p| p.to_string_lossy().into_owned())
            .map_err(|e| refuse(format!("the masked PATH cannot be joined: {}", e)))
    };
    let kept = dropping(&dirs, holds, spared, &[]);
    if resolves("git", &join(&kept)?).is_some() {
        return join(&kept);
    }
    let readd: Vec<std::path::PathBuf> = git_cmd_dir().into_iter().filter(|d| !holds(d)).collect();
    join(&dropping(&dirs, holds, spared, &readd))
}

// spec: installer/SPEC.md §The consumer smoke — Git for Windows' `cmd` directory, three levels above
// its exec path, the one holding git and no shell
#[cfg(not(unix))]
fn git_cmd_dir() -> Option<std::path::PathBuf> {
    let done = crate::proc::run(&programs::GIT, &["--exec-path"]).ok()?;
    let exec = text(done.stdout()?).trim().to_string();
    let cmd = Path::new(&exec).parent()?.parent()?.parent()?.join("cmd");
    cmd.join("git.exe").is_file().then_some(cmd)
}

// spec: installer/SPEC.md §The consumer smoke — a control program resolved on the masked PATH and
// run at what it resolved to, since the spawn funnel resolves a bare name against this process's
// own PATH
fn prove_control(program: &crate::programs::Program, args: &[&str], path: &str, arm: &str, masked: &str) -> Step {
    let set = path_set(&Some(path.to_string()));
    let ran = resolves(&program.name(), path).is_some_and(|at| {
        super::in_consumer(&program.clone().at(at), args, &set, &invoking_dir()).is_ok_and(|d| d.failure_report().is_none())
    });
    if !ran {
        return Err(fail(format!(
            "the {} arm's {} will not run on its masked PATH — the PATH resolves entries this host cannot execute, so every step below would fail for a reason that is not {}",
            arm,
            program.name(),
            masked
        )));
    }
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the absence mask proved both ways: the program
// resolves to nothing, and a control program still runs
pub(super) fn prove_absent(stem: &str, path: &str, arm: &str) -> Step {
    if resolves(stem, path).is_some() {
        return Err(fail(format!("the mask did not take: {} still resolves under the arm's PATH", stem)));
    }
    prove_control(&programs::GIT, &["--version"], path, arm, stem)
}

fn invoking_dir() -> String {
    std::env::temp_dir().display().to_string()
}

// spec: installer/SPEC.md §The consumer smoke — the download transport: the tarball verified
// against its digest and extracted with tar, then the same post-conditions and reversal with
// node/npm masked
pub(super) fn download(state: &mut Run) -> Step {
    let dl = format!("{}/download", state.scratch);
    mkdir(&dl)?;
    let name = Path::new(&state.tarball).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let copy = format!("{}/{}", dl, name);
    std::fs::copy(&state.tarball, &copy).map_err(|e| fail(format!("could not copy the tarball for the download arm: {}", e)))?;
    super::staging::sidecar(&copy)?;
    let line = std::fs::read_to_string(format!("{}.sha256", copy)).unwrap_or_default();
    if line.split_whitespace().next() != Some(super::consumer::digest_of(&copy).as_str()) {
        return Err(fail(
            "the packed tarball does not verify against its own sha256 digest — the checksum step the install page documents would not work",
        ));
    }
    extract(&copy, &dl)?;
    state.dl_package = format!("{}/package", dl);
    let extracted = Entry::Extracted(state.dl_package.clone());
    if !is_file(&extracted.bootstrap()) {
        return Err(fail(format!(
            "the extracted tarball carries no package/{} — the Node-free entry point is not in the payload",
            super::consumer::BOOTSTRAP
        )));
    }
    say(&format!("verified {} against its digest and extracted package/ with tar", name));
    let mask = format!("{}/mask", state.scratch);
    let masked = ["node", "npm", "npx"];
    let path = reach_mask(&mask, &masked, "download arm", "the tarball path is not Node-free")?;
    state.entry = extracted;
    state.run_path = Some(path.clone());
    prove_reach(&mask, &masked, &path, "download")?;
    say("mask: node, npm and npx are masked off the arm's PATH, and its git still runs");
    let c = consumer(state, "download")?;
    let seed = tree(&c)?;
    assert_install(state, PROFILE_DERIVED, &c)?;
    reversal(state, PROFILE_DERIVED, &c, &seed)
}

// spec: installer/SPEC.md §The consumer smoke — the toolchain-free arm: doctor before init and a full
// init with cargo and rustc masked off PATH
pub(super) fn toolchain_free(state: &mut Run) -> Step {
    let mask = format!("{}/toolmask", state.scratch);
    let masked = ["cargo", "rustc"];
    let path = reach_mask(&mask, &masked, "toolchain-free arm", "the install path is not free of the Rust toolchain")?;
    state.entry = state.installed();
    state.run_path = Some(path.clone());
    prove_reach(&mask, &masked, &path, "toolchain-free")?;
    say("mask: cargo and rustc are masked off the arm's PATH, and its git still runs");
    let c = consumer(state, "toolchain-free")?;
    let m = state.verb(&c, &["doctor"])?;
    if !m.succeeded() {
        return Err(failed(&m, "doctor is below contract on a machine carrying no Rust toolchain — a contributor-audience member is reaching the adopter's verdict"));
    }
    let said = out(&m);
    if find_line(&said, |l| l.starts_with("DOCTOR: clean")).is_none() {
        return Err(failed(&m, "doctor exited 0 on a toolchain-free machine without reporting clean"));
    }
    if find_line(&said, |l| l.starts_with("  cargo ") || l.starts_with("  rustc ")).is_some() {
        return Err(failed(&m, "doctor rendered a contributor-audience member to an adopter — such a member is omitted from the consumer verdict, not reported as informational"));
    }
    say("doctor: clean with no Rust toolchain on PATH, and silent about the members that need one");
    assert_install(state, PROFILE_DERIVED, &c)
}

// spec: installer/SPEC.md §The consumer smoke — the jq-less arm: the verbs and what they install run
// clean with jq absent from their PATH, and doctor names it nowhere
pub(super) fn jq_less(state: &mut Run) -> Step {
    let path = path_without("jq", &format!("{}/jqfarm", state.scratch))?;
    prove_absent("jq", &path, "jq-less")?;
    say("mask: jq resolves to nothing, and the masked PATH's git still runs");
    let set = path_set(&Some(path.clone()));
    let entry = state.installed();
    let silent = |label: &str, c: &str| -> Step {
        let m = entry_run(&entry, c, &["doctor"], &set)?;
        if !m.succeeded() {
            return Err(failed(&m, format!(
                "doctor ({}) exited {} with jq absent — no selection owes jq, so its absence cannot set the verdict",
                label,
                m.reported_code()
            )));
        }
        if out(&m).contains("jq") {
            return Err(failed(&m, format!(
                "doctor ({}) named jq — a contributor member is skipped outright, so an adopter is told about a program nothing they install runs",
                label
            )));
        }
        say(&format!("doctor ({}): clean, and names jq nowhere", label));
        Ok(())
    };
    let c = consumer(state, "jq-less")?;
    silent("no install", &c)?;
    let min = state.profile_min.clone();
    let m = entry_run(&entry, &c, &["init", "--profile", &min], &set)?;
    if !m.succeeded() {
        return Err(failed(&m, format!("init --profile {} refused on a jq-less machine — no profile's floor carries jq", min)));
    }
    say(&format!("init --profile {}: installs with no jq on PATH, exit 0", min));
    silent(&format!("inside {}", min), &c)?;
    let guard_kit = "guard-kit";
    let Some(guard_profile) = state.profiles.iter().find(|p| profile_kits(state, p).iter().any(|k| k == guard_kit)).cloned() else {
        return Err(fail(format!(
            "no profile carries {}, so the arm cannot assert that its hook runs on a jq-less machine",
            guard_kit
        )));
    };
    let c2 = consumer(state, "jq-less-guard")?;
    let m = entry_run(&entry, &c2, &["init", "--profile", &guard_profile], &set)?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "init --profile {} refused on a jq-less machine — {}'s hook reads JSON through the gate binary, so the profile owes no jq",
            guard_profile, guard_kit
        )));
    }
    say(&format!("init --profile {}: installs with no jq on PATH, exit 0", guard_profile));
    silent(&format!("inside {}", guard_profile), &c2)?;
    let bin = seam_of(&c2).unwrap_or_default();
    let bin_path = format!("{}/{}", c2, bin);
    if bin.is_empty() || !crate::proc::is_executable(Path::new(&bin_path)) {
        return Err(fail(format!(
            "no executable gate binary at '{}' inside {}, so the installed {} hook cannot be driven",
            if bin.is_empty() { "<unset>" } else { &bin },
            c2,
            guard_kit
        )));
    }
    let payload = r#"{"tool_name":"Bash","tool_input":{"command":"cat one.md two.md"}}"#;
    let done = in_consumer_fed(&programs::CHECKWRIGHT_GATES.at(bin_path), &["--hook", "shell-guard"], &set, &c2, payload.as_bytes())?;
    let (o, e) = done.streams();
    let said = format!("{}{}", text(o), text(e));
    if done.failure_report().is_some() || !said.contains(r#""permissionDecision":"allow""#) {
        show(&said);
        return Err(fail(format!(
            "the installed {} hook exited {} on a jq-less machine without an allow envelope for a read-only command — its payload read or its render reached for a program that is not there",
            guard_kit,
            done.reported_code()
        )));
    }
    say(&format!("{} hook: answers a payload with an allow envelope, no jq on PATH", guard_kit));
    for (label, argv) in [("diff", &["diff"][..]), ("uninstall --dry-run", &["uninstall", "--dry-run"])] {
        let m = entry_run(&entry, &c, argv, &set)?;
        if !m.succeeded() {
            return Err(failed(&m, format!(
                "{} exited {} on a jq-less machine — nothing behind the invoke reads JSON with jq, so a verb that needs it is reaching for a program the relocation removed the dependency on",
                label,
                m.reported_code()
            )));
        }
        if out(&m).contains("jq") {
            return Err(failed(&m, format!(
                "{} ran on a jq-less machine but mentioned jq — the verb still has an opinion about a program it no longer uses",
                label
            )));
        }
        say(&format!("{}: runs clean with no jq on PATH, exit 0", label));
    }
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — the bash-less arm: every profile whose own doctor
// report owes no bash installs, runs its printed follow-ups and commits through the installed hooks
// once clean and once refused, with bash absent from PATH
pub(super) fn bash_less(state: &mut Run) -> Step {
    let path = path_without("bash", &format!("{}/bashfarm", state.scratch))?;
    prove_absent("bash", &path, "bash-less")?;
    let set = path_set(&Some(path.clone()));
    let shell = super::consumer::host_shell();
    prove_control(&shell, super::consumer::SHELL_NOOP, &path, "bash-less", "bash")?;
    say(&format!(
        "mask: bash resolves to nothing, and the masked PATH's git and {} still run",
        shell.name()
    ));
    state.entry = Entry::Extracted(state.dl_package.clone());
    let c = consumer(state, "bash-less-probe")?;
    let m = entry_run(&state.entry, &c, &["doctor"], &set)?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "doctor exited {} with no install and bash absent — with no selection bash is undecided rather than owed, so it cannot set the verdict",
            m.reported_code()
        )));
    }
    let unprobed = lines::of("doctor's report", m.output()).iter().any(|l| {
        l.strip_prefix("  bash ")
            .map(str::trim_start)
            .and_then(|r| r.strip_prefix("not probed — owed where "))
            .and_then(|r| r.strip_suffix(" is selected"))
            .is_some_and(|who| !who.is_empty())
    });
    if !unprobed {
        return Err(failed(&m, "doctor with no install did not render bash as not probed, naming the audience it is owed by"));
    }
    say("doctor (no install): clean with no bash on PATH, bash rendered unprobed");
    let mut subjects = Vec::new();
    for p in &state.profiles {
        match state.owes_bash.get(p) {
            None => {
                return Err(fail(format!(
                    "the profile loop recorded no bash verdict for {}, so the bash-less arm cannot tell whether it owes bash",
                    p
                )))
            }
            Some(false) => subjects.push(p.clone()),
            Some(true) => {}
        }
    }
    if !subjects.contains(&state.profile_min) {
        return Err(fail(format!(
            "the lattice minimum {} owes bash by its own doctor report, so the arm has no bash-free profile to install",
            state.profile_min
        )));
    }
    for p in &subjects {
        bash_less_profile(state, p, &set)?;
    }
    say(&format!(
        "bash-less: {} profile(s) owing no bash installed and committed through the hooks: {}",
        subjects.len(),
        subjects.join(" ")
    ));
    Ok(())
}

fn bash_less_profile(state: &Run, p: &str, set: &[(String, String)]) -> Step {
    let c = consumer(state, &format!("bash-less-{}", p))?;
    let m = entry_run(&state.entry, &c, &["init", "--profile", p], set)?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "init --profile {} failed with no bash on PATH — the profile owes no bash, so init reached for it",
            p
        )));
    }
    let cmds = followups(&out(&m));
    if cmds.is_empty() {
        show(&out(&m));
        return Err(refuse(format!("{}: init printed no follow-up block, so the bash-less arm has no command to execute.", p)));
    }
    for line in &cmds {
        let toks: Vec<&str> = line.split_whitespace().collect();
        let r = run_line(&c, &toks, set)?;
        if !r.succeeded() {
            return Err(failed(&r, format!("{}: the follow-up command '{}' failed with no bash on PATH", p, line)));
        }
    }
    say(&format!("{}: init and {} follow-up command(s) ran with no bash on PATH", p, cmds.len()));
    write(&format!("{}/bash-less-note.txt", c), "a clean note\n")?;
    super::consumer::git(&c, &["add", "bash-less-note.txt"])?;
    let r = merged_in(&programs::GIT, &["commit", "-m", "docs: add a note"], set, &c)?;
    if !r.succeeded() {
        return Err(failed(&r, format!("{}: a clean commit through the installed hooks failed with no bash on PATH", p)));
    }
    let passed = find_line(&out(&r), |l| {
        l.strip_prefix("pre-commit: ")
            .and_then(|x| x.strip_suffix(" gate(s) passed."))
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
    })
    .is_some();
    if !passed {
        return Err(failed(&r, format!(
            "{}: the clean commit landed without the installed pre-commit hook reporting — the hook did not run",
            p
        )));
    }
    say(&format!("{}: a clean commit passed through the installed hooks", p));
    write(
        &format!("{}/bash-less-plant.sh", c),
        "#!/usr/bin/env bash\nset -o pipefail\nnames=(a b c)\nif printf \"%s\\n\" \"${names[@]}\" | grep -q b; then echo found; fi\n",
    )?;
    super::consumer::git(&c, &["add", "bash-less-plant.sh"])?;
    let r = merged_in(&programs::GIT, &["commit", "-m", "chore: add a pipeline that can lose its match"], set, &c)?;
    if r.succeeded() {
        return Err(failed(&r, format!(
            "{}: a commit planting a shell pipeline that can lose its match landed through the installed hooks",
            p
        )));
    }
    let gate = find_line(&out(&r), |l| l.starts_with("pre-commit: ") && l.ends_with(" failed (see above)."))
        .map(|l| l["pre-commit: ".len()..l.len() - " failed (see above).".len()].to_string())
        .filter(|g| !g.is_empty() && g.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'))
        .unwrap_or_default();
    let list = std::fs::read_to_string(format!("{}/{}/gates.list", c, GATES_DIR)).unwrap_or_default();
    if gate.is_empty() || !list.lines().any(|l| l == gate) {
        return Err(failed(&r, format!(
            "{}: the refused commit was not refused by a registered gate the pre-commit hook named",
            p
        )));
    }
    super::consumer::git(&c, &["reset", "-q", "--hard", "HEAD"])?;
    say(&format!(
        "{}: a planted shell pipeline that can lose its match was refused by {} through the installed hook",
        p, gate
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The consumer smoke — the stem rule matches a program under any
    // extension and case, and never a longer name sharing its head
    #[test]
    fn the_stem_rule_matches_any_suffix_and_no_longer_name() {
        assert!(stem_is("jq", "jq"));
        assert!(stem_is("jq", "JQ.EXE"));
        assert!(stem_is("jq", "jq.cmd"));
        assert!(!stem_is("jq", "jqx"));
        assert!(!stem_is("bash", "bashbug"));
    }

    // spec: installer/SPEC.md §The consumer smoke — the Windows column's mask: a directory holding a
    // masked program leaves PATH, the system directory stays though it holds one, a re-added
    // directory lands once at the end, and every other directory keeps its order
    #[test]
    fn the_windows_mask_drops_holders_spares_the_system_directory_and_re_adds_alone() {
        let p = |s: &str| std::path::PathBuf::from(s);
        let dirs = [
            p(r"C:\Windows\System32"),
            p(r"C:\Program Files\Git\bin"),
            p(r"C:\tools"),
            p(r"C:\Program Files\Git\usr\bin"),
        ];
        let holds = |d: &Path| {
            let s = d.to_string_lossy();
            s.contains("Git") || s.contains("System32")
        };
        let spared = |d: &Path| crate::proc::is_windows_system_dir(d, r"C:\Windows");
        let got = dropping(&dirs, holds, spared, &[p(r"C:\Program Files\Git\cmd"), p(r"C:\tools")]);
        assert_eq!(got, vec![p(r"C:\Windows\System32"), p(r"C:\tools"), p(r"C:\Program Files\Git\cmd")]);
    }
}
