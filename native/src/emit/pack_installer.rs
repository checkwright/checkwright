// spec: installer/README.md §The packer — assemble the installer package out of tree and npm-pack
// it there; the payload is derived from the consumer's own kit roots at pack time, so no second
// copy of any kit is ever checked in or written inside the worktree
// spec: gate-sdk/SPEC.md §The non-gate arm — a bridged `Arm::Run` and not an `--emit-` member: the
// product is a tarball plus a receipt, so an emitting arm would return a receipt for a side effect
use crate::ere::Ere;
use crate::proc::{self, Stderr};
use crate::walk;

// spec: gate-sdk/SPEC.md §The non-gate arm — the four declared names, each defined in
// `gate-sdk/lib/gate.sh`. `INSTALLER_PACK_TMP_DIR` and its `TMPDIR` fallback are absent and must
// be: no kit library defines either, so a declared row would fail-close the arm on every run
pub const KNOBS: &[&str] = &[
    "GATE_KIT_ROOTS_REL",
    "GATE_SDK_NATIVE_TARGETS_FILE",
    "GATE_SDK_NATIVE_BIN",
    "GATE_SDK_NATIVE_ARTIFACT_NAMES",
];

// spec: installer/README.md §The packer — the diagnostic prefix the shell form printed, kept across
// the cut so a reader of a finished CI log meets one name and not two
const NAME: &str = "pack-installer";

// spec: installer/README.md §The packer — one refusal type behind one formatter, so the arm has no
// exit path that prints nothing: the shell form ran without `-e` and could leave a reader a
// non-zero status with no cause at all
#[derive(Debug)]
struct Refusal {
    cause: String,
    help: Vec<String>,
}

fn refuse(cause: impl Into<String>) -> Refusal {
    Refusal {
        cause: cause.into(),
        help: Vec::new(),
    }
}

fn refuse_help(cause: impl Into<String>, help: &[&str]) -> Refusal {
    Refusal {
        cause: cause.into(),
        help: help.iter().map(|h| (*h).to_string()).collect(),
    }
}

// spec: installer/README.md §The packer — the single refusal formatter: the prefix, the cause and
// any help lines on stderr, exit 2; every refusal in this module returns through it
fn report(r: &Refusal) -> i32 {
    eprintln!("{}: {}", NAME, r.cause);
    for h in &r.help {
        eprintln!("  help: {}", h);
    }
    2
}

// spec: installer/README.md §The packer — the scratch teardown as the arm's own `Drop` rather than
// an `EXIT` trap, the shape `--run-demo` established: every return path unwinds through it
struct Scratch {
    dir: String,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if !self.dir.is_empty() {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }
}

#[derive(Default)]
struct Flags {
    version: String,
    out: String,
    artifacts: String,
    root: String,
}

pub fn run(args: &[String]) -> i32 {
    let mut scratch = Scratch {
        dir: String::new(),
    };
    match pack(args, &mut scratch) {
        Ok(receipt) => {
            println!("{}", receipt);
            0
        }
        Err(r) => report(&r),
    }
}

// spec: installer/README.md §The packer — the flag roster, whose one-tier `--help` rule retires
// with the shell file: an unknown argument is still a refusal, and the usage lives in
// `gate-sdk/bin/run-gates.sh`'s own help and in that section's prose
fn parse(args: &[String]) -> Result<Flags, Refusal> {
    let mut f = Flags::default();
    let mut i = 0;
    while i < args.len() {
        let take = |i: usize| -> String { args.get(i + 1).cloned().unwrap_or_default() };
        match args[i].as_str() {
            "--version" => f.version = take(i),
            "--out" => f.out = take(i),
            "--artifacts" => f.artifacts = take(i),
            "--root" => f.root = take(i),
            other => return Err(refuse(format!("unknown argument: {}", other))),
        }
        i += 2;
    }
    Ok(f)
}

fn pack(args: &[String], scratch: &mut Scratch) -> Result<String, Refusal> {
    let f = parse(args)?;
    let root = resolve_root(&f.root)?;
    std::env::set_current_dir(&root)
        .map_err(|e| refuse(format!("cannot enter the work tree at {}: {}", root, e)))?;
    let root = walk::cwd().map_err(refuse)?;

    // spec: installer/README.md §The packer — the preflight tool set tracks the spawned set in
    // both directions: `jq` is unreached, so probing it would refuse on a program nothing runs,
    // and `mktemp` is reached, so omitting it reported a generic spawn failure
    for tool in ["npm", "git", "tar", "mktemp"] {
        if !proc::on_path(tool) {
            return Err(refuse(format!(
                "{} not found on PATH — the pack step cannot run.",
                tool
            )));
        }
    }

    if !std::path::Path::new("installer/package.json").is_file() {
        return Err(refuse(
            "installer/package.json not found — there is no package to pack.",
        ));
    }

    // spec: installer/README.md §The packer — the refusal asks about the payload's own footprint
    // rather than the whole worktree, on both grounds that section states: the stamp, on the two
    // members the worktree can reach, and the tree-under-test property across the whole footprint
    let spec = footprint(&root, &f.artifacts)?;
    let mut status: Vec<&str> = vec!["status", "--porcelain", "--"];
    status.extend(spec.iter().map(String::as_str));
    let dirty = git(&status)?;
    if !dirty.is_empty() {
        return Err(refuse_help(
            dirty_cause(&root, &dirty),
            &[
                "this is checked once per invocation, against the tree as it is now — not as it was when your run started, so a concurrent edit during a long run trips it here rather than at the point you invoked the run.",
                "commit or stash first; a dirty path in this footprint is either one the stamp would misdescribe or one that makes the tree under test not the tree that gets packed.",
            ],
        ));
    }

    let commit = git(&["rev-parse", "HEAD"])?;
    if !is_commit(&commit) {
        return Err(refuse("could not resolve HEAD to a 40-hex commit."));
    }

    let version = resolve_version(&f.version)?;

    // spec: installer/README.md §The packer — the scratch base and its `TMPDIR` fallback, read off
    // the process environment: neither name is a kit knob, so neither may be declared
    let base = env_or("INSTALLER_PACK_TMP_DIR")
        .or_else(|| env_or("TMPDIR"))
        .unwrap_or_else(|| "/tmp".to_string());
    if !std::path::Path::new(&base).is_dir() {
        return Err(refuse(format!("scratch base not a directory: {}", base)));
    }
    let asm = make_scratch(&base, scratch)?;
    let out = if f.out.is_empty() { base } else { f.out };
    if !std::path::Path::new(&out).is_dir() {
        return Err(refuse(format!("output directory not found: {}", out)));
    }

    pack_tracked(&commit, "installer", &asm)?;

    // spec: installer/README.md §The packer — the payload's kit set is gate_kit_roots_rel, the same
    // derivation the battery runs on, so the shipped set cannot drift from the governed one
    mkdir(&format!("{}/payload", asm))?;
    let mut packed = 0usize;
    for kit in walk::kit_roots_rel().map_err(refuse)? {
        let kit = kit.trim_end_matches('/');
        if kit.is_empty() || !std::path::Path::new(kit).is_dir() {
            continue;
        }
        let leaf = kit.rsplit('/').next().unwrap_or(kit);
        pack_tracked(&commit, kit, &format!("{}/payload/{}", asm, leaf))?;
        packed += 1;
    }
    if packed == 0 {
        return Err(refuse(
            "no kit roots enumerated — the payload would be empty.",
        ));
    }

    let artifacts = pack_artifacts(&f.artifacts, &asm)?;

    stamp(&asm, &version, &commit)?;

    let tarball = npm_pack(&asm)?;
    let landed = format!("{}/{}", out.trim_end_matches('/'), tarball);
    move_file(&format!("{}/{}", asm, tarball), &landed)?;

    Ok(format!(
        "PACK: {} (version {}, commit {}, root {}, {} kit(s) in payload, {} prebuilt gate binary/binaries)",
        landed,
        version,
        &commit[..12],
        root,
        packed,
        artifacts
    ))
}

// spec: installer/README.md §The packer — a caller that already holds the tree it means says so;
// the value is validated to a work-tree top level because silently promoting a subdirectory is the
// same correction the flag exists to remove
// spec: installer/README.md §The packer — the cwd selects whose tooling runs and `--root` which
// tree is packed; the front-end resolved the first before this arm ran, and this resolves the
// second
fn resolve_root(named: &str) -> Result<String, Refusal> {
    if named.is_empty() {
        return walk::toplevel_opt()
            .ok()
            .flatten()
            .ok_or_else(|| {
                refuse("not inside a git work tree — the payload's commit stamp has no source.")
            });
    }
    if !std::path::Path::new(named).is_dir() {
        return Err(refuse(format!("--root is not a directory: {}", named)));
    }
    let top = walk::toplevel_in(named)
        .map_err(|_| refuse(format!("--root is not inside a git work tree: {}", named)))?;
    let here = walk::canonicalize(named)
        .ok_or_else(|| refuse(format!("--root is not a directory: {}", named)))?;
    let there = walk::canonicalize(&top).unwrap_or_else(|| top.clone());
    if here != there {
        return Err(refuse_help(
            format!(
                "--root names a subdirectory of the work tree at {}, not its top level: {}",
                top, named
            ),
            &["pass the top level; promoting a subdirectory to it would pack a tree you did not name."],
        ));
    }
    Ok(top)
}

// spec: installer/README.md §The packer — the version comes from the tag, never from an edit to
// installer/package.json; the regex is the one that also admits a prerelease suffix
fn resolve_version(given: &str) -> Result<String, Refusal> {
    let mut version = given.to_string();
    if version.is_empty() {
        version = git(&["describe", "--tags", "--abbrev=0"]).unwrap_or_default();
        version = version.trim_start_matches('v').to_string();
    }
    let re = Ere::compile("^[0-9]+\\.[0-9]+\\.[0-9]+([-+][0-9A-Za-z.-]+)?$")
        .map_err(|e| refuse(e.to_string()))?;
    if !re.is_match(&version) {
        return Err(refuse_help(
            "no usable version — pass --version, or tag the commit being packed.",
            &["the version comes from the tag, never from an edit to installer/package.json."],
        ));
    }
    Ok(version)
}

// spec: installer/README.md §The packer — the payload's tree footprint, DERIVED from the same two
// resolvers the pack loop itself runs so the refusal's corpus cannot drift from the packed set;
// the roster rides on `--artifacts`, which is what makes it a payload input at all
fn footprint(root: &str, artifacts: &str) -> Result<Vec<String>, Refusal> {
    let mut spec = vec!["installer".to_string()];
    // spec: installer/README.md §The packer — the pack loop's own on-disk `is_dir` test is
    // deliberately NOT applied here: filtering the pathspec by it would blind the refusal to the
    // one divergence only it can see
    for kit in walk::kit_roots_rel().map_err(refuse)? {
        if let Some(p) = inside(root, kit.trim_end_matches('/')) {
            spec.push(p);
        }
    }
    if !artifacts.is_empty() {
        let roster = walk::knob_scalar("GATE_SDK_NATIVE_TARGETS_FILE").map_err(refuse)?;
        if let Some(p) = inside(root, &roster) {
            spec.push(p);
        }
    }
    Ok(spec)
}

// spec: installer/README.md §The packer — a member outside the packed tree is dropped rather than
// handed to git, which refuses one; the test is lexical because a kit root deleted from the
// worktree still belongs in the pathspec and cannot be canonicalized
// spec: gate-sdk/SPEC.md §The path-dialect contract — absoluteness is a TWO-dialect question and
// `walk::path_root` is its single owner, asked here rather than re-implemented; a foreign
// backslash spelling never reaches this site, the contract crossing dialect once at the producer
fn inside(root: &str, path: &str) -> Option<String> {
    let root = root.trim_end_matches('/');
    let rel = if walk::path_root(path).is_some() {
        let tail = path.strip_prefix(root)?;
        if !tail.is_empty() && !tail.starts_with('/') {
            return None;
        }
        tail.trim_start_matches('/').to_string()
    } else {
        path.to_string()
    };
    if rel.split('/').any(|s| s == "..") {
        return None;
    }
    Some(if rel.is_empty() { ".".to_string() } else { rel })
}

// spec: installer/README.md §The packer — the refusal names the entries it found, bounded and with
// a total, so a reader tells a shipping path from scratch without re-running `git status` by hand
const DIRTY_SHOWN: usize = 10;

fn dirty_cause(root: &str, dirty: &str) -> String {
    let entries: Vec<&str> = dirty.lines().collect();
    let mut cause = format!(
        "{} path(s) the payload is assembled from are dirty in the worktree at {}:",
        entries.len(),
        root
    );
    for e in entries.iter().take(DIRTY_SHOWN) {
        cause.push_str(&format!("\n  {}", e));
    }
    if entries.len() > DIRTY_SHOWN {
        cause.push_str(&format!("\n  (and {} more)", entries.len() - DIRTY_SHOWN));
    }
    cause
}

fn is_commit(c: &str) -> bool {
    c.len() == 40 && c.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn env_or(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — git's stdout is reachable only through the
// accessor that read the status, so a failed probe cannot be read as an empty answer
// spec: installer/README.md §The packer — trailing whitespace only: a porcelain entry's first two
// bytes ARE its state code, so trimming both ends would re-column the one line the dirty
// diagnostic prints first, and no other caller here reads a leading blank
fn git(args: &[&str]) -> Result<String, Refusal> {
    let done = proc::run("git", args).map_err(refuse)?;
    match done.stdout() {
        Some(o) => Ok(String::from_utf8_lossy(o).trim_end().to_string()),
        None => Err(refuse(format!(
            "git {} failed — {}",
            args.join(" "),
            done.failure_report().unwrap_or_default()
        ))),
    }
}

fn mkdir(path: &str) -> Result<(), Refusal> {
    std::fs::create_dir_all(path)
        .map_err(|e| refuse(format!("cannot create {}: {}", path, e)))
}

fn make_scratch(base: &str, scratch: &mut Scratch) -> Result<String, Refusal> {
    let template = format!("{}/checkwright-pack.XXXXXX", base.trim_end_matches('/'));
    let made = proc::run("mktemp", &["-d", &template]).map_err(refuse)?;
    let dir = made
        .stdout()
        .map(|o| String::from_utf8_lossy(o).trim().to_string())
        .filter(|d| !d.is_empty())
        .ok_or_else(|| refuse(format!("cannot create the assembly scratch under {}", base)))?;
    scratch.dir = dir.clone();
    Ok(dir)
}

// spec: gate-sdk/SPEC.md §Consumer payload — vendor git-tracked paths only, unconditionally: the
// clean-tree check does not see ignored paths, so `git archive` at the stamped commit is the
// tracked-set boundary and the strip count peels exactly the source's own path depth
fn pack_tracked(commit: &str, src: &str, dst: &str) -> Result<(), Refusal> {
    let src = src.trim_end_matches('/');
    // spec: gate-sdk/SPEC.md §Consumer payload — refuse an unvendorable tracked symlink BEFORE the
    // pipeline: a failure after it lands mid-kit having written a partial vendor, where a pre-flight
    // writes nothing and names the cause
    let listing = git(&["ls-files", "-s", "--", src])?;
    let links: Vec<&str> = listing
        .lines()
        .filter(|l| l.starts_with("120000 "))
        .filter_map(|l| l.split_once('\t'))
        .map(|(_, p)| p)
        .collect();
    if !links.is_empty() {
        let mut cause = format!(
            "{} carries tracked symlink(s), which the payload may not:",
            src
        );
        for p in &links {
            cause.push_str(&format!("\n  {}", p));
        }
        return Err(refuse_help(cause, &[
            "the payload reproduces the tracked set with tar, and a host that cannot create a symlink aborts the extraction part-way through the kit.",
            "remove it from the packed set; an assertion that needs one constructs it at run time in its own sandbox instead.",
        ]));
    }
    let depth = 1 + src.matches('/').count();
    mkdir(dst)?;
    let archive = proc::run("git", &["archive", commit, "--", src]).map_err(refuse)?;
    let bytes = match archive.stdout() {
        Some(b) => b.to_vec(),
        None => {
            return Err(refuse(format!(
                "git archive could not read {} at {} — {}",
                src,
                &commit[..12.min(commit.len())],
                archive.failure_report().unwrap_or_default()
            )))
        }
    };
    let strip = format!("--strip-components={}", depth);
    let done = proc::run_streamed("tar", &["-x", &strip, "-C", dst], &bytes, Stderr::Inherit)
        .map_err(refuse)?;
    if done.code() != 0 {
        return Err(refuse(format!(
            "tar could not extract {} into {} (exit {})",
            src,
            dst,
            done.code()
        )));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §Consumer payload — the prebuilt binaries ride beside the kit roots, one
// directory per roster target with the sidecar its build leg emitted; the arm never builds one, so
// a locally-built binary can never substitute for a released one
fn pack_artifacts(dir: &str, asm: &str) -> Result<usize, Refusal> {
    if dir.is_empty() {
        return Ok(0);
    }
    if !std::path::Path::new(dir).is_dir() {
        return Err(refuse(format!("artifact directory not found: {}", dir)));
    }
    let roster = walk::knob_scalar("GATE_SDK_NATIVE_TARGETS_FILE").map_err(refuse)?;
    // spec: gate-sdk/SPEC.md §lib/gate.sh — the per-target artifact NAME crosses the bridge as a
    // value, so the executable suffix keeps one owner and this arm derives nothing
    let names = walk::knob_map("GATE_SDK_NATIVE_ARTIFACT_NAMES").map_err(refuse)?;
    if names.is_empty() {
        if !std::path::Path::new(&roster).is_file() {
            return Err(refuse(format!(
                "no target roster at {} — there is no declared platform set to pack artifacts for.",
                roster
            )));
        }
        return Err(refuse(format!(
            "the target roster at {} declares no targets.",
            roster
        )));
    }
    mkdir(&format!("{}/payload/artifact", asm))?;
    let mut packed = 0usize;
    for (target, binary) in &names {
        let src = format!("{}/{}", dir.trim_end_matches('/'), target);
        if !std::path::Path::new(&src).is_dir() {
            return Err(refuse_help(
                format!("roster target '{}' has no artifact directory at {}.", target, src),
                &["every declared target's build leg must have run; a roster target no leg built is a broken payload, not a narrower one."],
            ));
        }
        let bin = format!("{}/{}", src, binary);
        let side = format!("{}.sha256", bin);
        if !std::path::Path::new(&bin).is_file() || !std::path::Path::new(&side).is_file() {
            return Err(refuse(format!(
                "{} is missing {} or its .sha256 sidecar in {}.",
                target, binary, src
            )));
        }
        verify_sidecar(&bin, &side, target, &src)?;
        let into = format!("{}/payload/artifact/{}", asm, target);
        mkdir(&into)?;
        let placed = format!("{}/{}", into, binary);
        copy(&bin, &placed)?;
        copy(&side, &format!("{}.sha256", placed))?;
        // spec: gate-sdk/SPEC.md §Consumer payload — restore the executable mode here and in no
        // workflow: this is the one seam both artifact transports reach. Set absolutely, so this
        // process's umask cannot decide what an adopter receives
        set_mode_755(&placed)?;
        packed += 1;
    }
    // spec: gate-sdk/SPEC.md §Consumer payload — the roster's one publication, copied verbatim,
    // never regenerated or filtered
    copy(&roster, &format!("{}/payload/artifact/targets.list", asm))?;
    Ok(packed)
}

// spec: gate-sdk/SPEC.md §Consumer payload — the digest is emitted once by the build leg and only
// ever moved, so this reader checks a value it did not compute; in-crate, `sha256sum` having left
// with the port
fn verify_sidecar(bin: &str, side: &str, target: &str, src: &str) -> Result<(), Refusal> {
    let want = std::fs::read_to_string(side)
        .map_err(|e| refuse(format!("cannot read {}: {}", side, e)))?
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let got = crate::sha256::file_hex(std::path::Path::new(bin)).map_err(refuse)?;
    if want == got {
        return Ok(());
    }
    Err(refuse_help(
        format!("{}'s sidecar does not match the binary beside it in {}.", target, src),
        &["the digest is emitted once by the build leg and only ever moved; a mismatch here means the bytes changed after it was written."],
    ))
}

fn copy(from: &str, to: &str) -> Result<(), Refusal> {
    std::fs::copy(from, to)
        .map(|_| ())
        .map_err(|e| refuse(format!("cannot copy {} to {}: {}", from, to, e)))
}

#[cfg(unix)]
fn set_mode_755(path: &str) -> Result<(), Refusal> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
        .map_err(|e| refuse(format!("cannot set mode 755 on {}: {}", path, e)))
}

// spec: gate-sdk/SPEC.md §Consumer payload — the mode is restored where the platform has one, the
// branch `install.rs`'s executable bit already takes for the same boundary
#[cfg(not(unix))]
fn set_mode_755(_path: &str) -> Result<(), Refusal> {
    Ok(())
}

// spec: installer/README.md §The packer — the version-and-commit stamp, a `serde_json` edit rather
// than a `jq` spawn: the tool's last reader of that program leaves with the port
fn stamp(asm: &str, version: &str, commit: &str) -> Result<(), Refusal> {
    let path = format!("{}/package.json", asm);
    let text = std::fs::read_to_string(&path)
        .map_err(|e| refuse(format!("could not read {}: {}", path, e)))?;
    let mut doc: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| refuse(format!("could not stamp installer/package.json: {}", e)))?;
    let obj = doc
        .as_object_mut()
        .ok_or_else(|| refuse("could not stamp installer/package.json."))?;
    obj.insert("version".into(), serde_json::Value::String(version.into()));
    let slot = obj
        .entry("checkwright")
        .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
    let slot = slot
        .as_object_mut()
        .ok_or_else(|| refuse("could not stamp installer/package.json."))?;
    slot.insert("commit".into(), serde_json::Value::String(commit.into()));
    let mut body = serde_json::to_string_pretty(&doc)
        .map_err(|e| refuse(format!("could not stamp installer/package.json: {}", e)))?;
    body.push('\n');
    std::fs::write(&path, body)
        .map_err(|e| refuse(format!("could not write {}: {}", path, e)))
}

// spec: installer/README.md §The packer — `npm pack` stays a spawn deliberately: reproducing the
// package format in-crate is a second implementation of a format, not a port
fn npm_pack(asm: &str) -> Result<String, Refusal> {
    let done = proc::run_merged_in("npm", &["pack"], &[], Some(std::path::Path::new(asm)))
        .map_err(refuse)?;
    if !done.succeeded() {
        return Err(refuse(format!(
            "npm pack failed in {}.\n{}",
            asm,
            String::from_utf8_lossy(done.output()).trim()
        )));
    }
    let mut tarballs: Vec<String> = walk::list_dir(std::path::Path::new(asm))
        .map_err(refuse)?
        .into_iter()
        .filter(|(name, is_dir)| !*is_dir && name.ends_with(".tgz"))
        .map(|(name, _)| name)
        .collect();
    tarballs.sort();
    match tarballs.len() {
        1 => Ok(tarballs.remove(0)),
        n => Err(refuse(format!(
            "expected exactly one tarball in {}, found {}.",
            asm, n
        ))),
    }
}

// spec: installer/README.md §The packer — the tarball is moved out of the scratch before the
// teardown runs; the copy-then-remove fallback is what a cross-filesystem rename needs, which two
// separately configurable directories make reachable
fn move_file(from: &str, to: &str) -> Result<(), Refusal> {
    if std::fs::rename(from, to).is_ok() {
        return Ok(());
    }
    copy(from, to)?;
    std::fs::remove_file(from)
        .map_err(|e| refuse(format!("cannot remove {} after moving it: {}", from, e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/README.md §The packer — the flag roster's unknown-argument refusal survives
    // the cut, and `--help` is no longer one of the four: its one-tier rule retired with the file.
    #[test]
    fn an_unknown_argument_is_a_refusal() {
        assert!(parse(&["--nope".to_string()]).is_err());
        assert!(parse(&["--help".to_string()]).is_err());
        let f = parse(&[
            "--version".to_string(),
            "1.2.3".to_string(),
            "--root".to_string(),
            "/tmp".to_string(),
        ])
        .expect("the four declared flags parse");
        assert_eq!(f.version, "1.2.3");
        assert_eq!(f.root, "/tmp");
        assert!(f.out.is_empty() && f.artifacts.is_empty());
    }

    // spec: installer/README.md §The packer — the version regex admits a prerelease suffix, which
    // is the property docs/install.md rests its prerelease claim on.
    #[test]
    fn the_version_regex_admits_a_prerelease_suffix() {
        assert!(resolve_version("1.2.3").is_ok());
        assert!(resolve_version("1.2.3-rc.1").is_ok());
        assert!(resolve_version("1.2.3+build.5").is_ok());
        assert!(resolve_version("v1.2.3").is_err());
        assert!(resolve_version("1.2").is_err());
    }

    // spec: installer/README.md §The packer — the commit stamp is a 40-hex object name, so a short
    // or an abbreviated one is refused rather than stamped.
    #[test]
    fn only_a_forty_hex_commit_stamps() {
        assert!(is_commit(&"a".repeat(40)));
        assert!(!is_commit(&"a".repeat(39)));
        assert!(!is_commit(&"A".repeat(40)));
        assert!(!is_commit("g".repeat(40).as_str()));
    }

    // spec: gate-sdk/SPEC.md §Consumer payload — the strip count peels exactly the source's own
    // path depth, so a nested kit root does not nest one level too deep in the payload.
    #[test]
    fn the_strip_count_is_the_sources_own_depth() {
        assert_eq!(1 + "installer".matches('/').count(), 1);
        assert_eq!(1 + "vendor/gate-sdk".matches('/').count(), 2);
        assert_eq!(1 + "a/b/c".matches('/').count(), 3);
    }

    // spec: installer/README.md §The packer — the single formatter prints the prefix, the cause and
    // every help line, so no refusal reaches a reader as a bare non-zero status.
    #[test]
    fn every_refusal_carries_the_prefix_and_exits_two() {
        assert_eq!(report(&refuse("cause")), 2);
        assert_eq!(report(&refuse_help("cause", &["do this"])), 2);
    }

    // spec: installer/README.md §The packer — a footprint member outside the packed tree is
    // dropped rather than handed to git, and one inside it crosses as its repo-relative spelling.
    #[test]
    fn only_members_inside_the_packed_tree_enter_the_pathspec() {
        assert_eq!(inside("/w", "gate-sdk"), Some("gate-sdk".to_string()));
        assert_eq!(inside("/w", "/w/native/targets.list"), Some("native/targets.list".to_string()));
        assert_eq!(inside("/w/", "/w/installer"), Some("installer".to_string()));
        assert_eq!(inside("/w", "/tmp/host-targets.list"), None);
        assert_eq!(inside("/w", "/w-other/targets.list"), None);
        assert_eq!(inside("/w", "../outside"), None);
        assert_eq!(inside("/w", "kits/../../outside"), None);
    }

    // spec: gate-sdk/SPEC.md §The path-dialect contract — a drive-rooted path is absolute too and
    // a leading-slash test answers false on it; both sides are pinned because only the pair
    // separates the fix from a test that drops every drive-lettered member.
    #[test]
    fn a_drive_rooted_member_is_judged_by_the_same_two_dialect_rule() {
        assert_eq!(inside("D:/w", "D:/_temp/targets.list"), None);
        assert_eq!(
            inside("D:/w", "D:/w/native/targets.list"),
            Some("native/targets.list".to_string())
        );
        assert_eq!(inside("D:/w", "D:/w-other/targets.list"), None);
        assert_eq!(inside("D:/w", "D:/w"), Some(".".to_string()));
        assert_eq!(inside("D:/w", "gate-sdk"), Some("gate-sdk".to_string()));
    }

    // spec: installer/README.md §The packer — a worktree-deleted kit root is exactly what the
    // refusal must still see, so the whole tree is the pathspec when a member resolves to the root
    // itself rather than the member being silently dropped.
    #[test]
    fn a_member_that_is_the_root_itself_spells_the_whole_tree() {
        assert_eq!(inside("/w", "/w"), Some(".".to_string()));
    }

    // spec: installer/README.md §The packer — the diagnostic names the entries it found, bounded,
    // and states the total so a truncated list is never read as the whole of it.
    #[test]
    fn the_diagnostic_is_bounded_and_states_the_total() {
        let few = dirty_cause("/w", " M installer/README.md\n?? installer/x");
        assert!(few.starts_with("2 path(s) the payload is assembled from are dirty in the worktree at /w:"));
        assert!(few.contains("\n   M installer/README.md") && few.contains("\n  ?? installer/x"));
        assert!(!few.contains("(and"));

        let many: Vec<String> = (0..DIRTY_SHOWN + 3).map(|i| format!("?? installer/f{}", i)).collect();
        let lots = dirty_cause("/w", &many.join("\n"));
        assert!(lots.starts_with(&format!("{} path(s)", DIRTY_SHOWN + 3)));
        assert_eq!(lots.matches("?? installer/f").count(), DIRTY_SHOWN);
        assert!(lots.ends_with("(and 3 more)"));
    }
}
