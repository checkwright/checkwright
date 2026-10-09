// spec: installer/SPEC.md §The packer — assemble the installer package out of tree and npm-pack
// it there; the payload is derived from the consumer's own kit roots at pack time, so no second
// copy of any kit is ever checked in or written inside the worktree
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` and not an `--emit-` member: the
// product is a tarball plus a receipt, so an emitting arm would return a receipt for a side effect
use super::close_surfaces;
use crate::ere::Ere;
use crate::proc::{self, Stderr};
use crate::programs;
use crate::walk;

// spec: gate-sdk/SPEC.md §The non-gate arm — the declared names, each defined in gate-sdk's knob
// table but the package directory, a descriptor knob; `INSTALLER_PACK_TMP_DIR` and its `TMPDIR` fallback are absent and must be, no kit library
// defining either
pub const KNOBS: &[&str] = &[
    "GATE_SDK_KIT_DIRS",
    "GATE_SDK_NATIVE_TARGETS_FILE",
    "GATE_SDK_NATIVE_BIN",
    "GATE_SDK_PAYLOAD_LICENSE",
    "GATE_SDK_PAYLOAD_RECIPES",
    "GATE_SDK_PAYLOAD_WITHHOLD",
    "GATE_SDK_SPEC_BASE_URL",
    INSTALLER_KNOB,
];

// spec: installer/SPEC.md §The packer — the package directory, a descriptor knob of the publishing
// repository, so a tree declaring none refuses by name
const INSTALLER_KNOB: &str = "GATE_LOCAL_INSTALLER_DIR";

const NAME: &str = "pack-installer";

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

fn report(r: &Refusal) -> i32 {
    eprintln!("{}: {}", NAME, r.cause);
    for h in &r.help {
        eprintln!("  help: {}", h);
    }
    2
}

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

// spec: installer/SPEC.md §The packer — the flag roster, whose one-tier `--help` rule retires
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
    // spec: gate-sdk/SPEC.md §Fail-closed contract — cwd-write-exempt: the arm's own process enters
    // the tree it packs, and no test reaches `pack()`
    std::env::set_current_dir(&root)
        .map_err(|e| refuse(format!("cannot enter the work tree at {}: {}", root, e)))?;
    let root = walk::cwd().map_err(refuse)?;

    // spec: installer/SPEC.md §The packer — the preflight tool set tracks the spawned set in
    // both directions: `jq` is unreached, so probing it would refuse on a program nothing runs,
    // and `mktemp` is reached, so omitting it reported a generic spawn failure
    for tool in [&programs::NPM, &programs::GIT, &programs::TAR, &programs::MKTEMP] {
        if !proc::on_path(tool) {
            return Err(refuse(format!(
                "{} not found on PATH — the pack step cannot run.",
                tool
            )));
        }
    }

    let installer = walk::knob_scalar(INSTALLER_KNOB).map_err(refuse)?;
    let installer = installer.trim_end_matches('/');
    let package = format!("{}/package.json", installer);
    if !std::path::Path::new(&package).is_file() {
        return Err(refuse(format!("{} not found — there is no package to pack.", package)));
    }

    // spec: installer/SPEC.md §The packer — the refusal asks about the payload's own footprint
    // rather than the whole worktree, on both grounds that section states: the stamp, on the two
    // members the worktree can reach, and the tree-under-test property across the whole footprint
    let spec = footprint(&root, &f.artifacts, installer)?;
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

    let license = license_text(&commit, &walk::knob_scalar("GATE_SDK_PAYLOAD_LICENSE").map_err(refuse)?)?;

    // spec: installer/SPEC.md §The packer — the scratch base and its `TMPDIR` fallback, read off
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

    // spec: gate-sdk/SPEC.md §Consumer payload — the withheld shape reaches the kit roots alone:
    // `installer/` is packed whole, its own non-shipping content decided by the package roster
    pack_tracked(&commit, installer, &asm, &[])?;
    place_license(license.as_deref(), &asm)?;

    // spec: installer/SPEC.md §The packer — payload recipes ride the package root beside the
    // payload, extracted like kits from the stamped commit
    let recipes = recipe_plan(&asm, &walk::knob_map("GATE_SDK_PAYLOAD_RECIPES").map_err(refuse)?)?;
    for (dir, into) in &recipes {
        if !tracked_under(&commit, dir)? {
            return Err(refuse_help(
                format!(
                    "GATE_SDK_PAYLOAD_RECIPES names {}, which holds no file tracked at {}.",
                    dir,
                    &commit[..12]
                ),
                &["a recipe the package cannot carry is a broken payload rather than a smaller one: commit the directory, or drop its pair."],
            ));
        }
        pack_tracked(&commit, dir, into, &[])?;
    }

    // spec: gate-sdk/SPEC.md §Consumer payload — one declared shape reaching every root the loop
    // yields, so the shipped set stays derived from the governed one; a per-kit roster would be the
    // maintained copy derivation-first refuses, and would let a kit fall out by being forgotten
    let withhold: Vec<String> = walk::knob_scalar("GATE_SDK_PAYLOAD_WITHHOLD")
        .map_err(refuse)?
        .split_whitespace()
        .map(String::from)
        .collect();

    // spec: gate-sdk/SPEC.md §Consumer payload — the publisher's SPEC base is resolved once, above
    // the kit loop, because it now has two consumers: the stamp `init` reads and the per-kit README
    // rewrite inside the loop. Read below the loop it would have been a knob the loop could not see
    let spec_base_url = walk::knob_scalar("GATE_SDK_SPEC_BASE_URL").map_err(refuse)?;

    // spec: installer/SPEC.md §The packer — the payload's kit set is `walk::kit_roots`, the same
    // derivation the battery runs on, so the shipped set cannot drift from the governed one
    // spec: gate-sdk/SPEC.md §Layout and configuration — statted and handed to an uncwd-anchored
    // git, so the working-directory spelling, while the payload leaf stays the basename
    mkdir(&format!("{}/payload", asm))?;
    let kits: Vec<String> = walk::kit_roots()
        .map_err(refuse)?
        .iter()
        .map(|k| k.trim_end_matches('/').to_string())
        .filter(|k| !k.is_empty() && std::path::Path::new(k).is_dir())
        .collect();
    // spec: gate-sdk/SPEC.md §Consumer payload — the packed-leaf set is resolved once, above the
    // loop, so a README's link to another kit's SPEC is rewritten only for a kit this pack packs
    let leaves: Vec<String> = kits
        .iter()
        .map(|k| k.rsplit('/').next().unwrap_or(k).to_string())
        .collect();
    let mut packed = 0usize;
    let mut rewritten = 0usize;
    let mut carried = 0usize;
    for (kit, leaf) in kits.iter().zip(&leaves) {
        let into = format!("{}/payload/{}", asm, leaf);
        refuse_tracked_carry(".", &commit, kit)?;
        pack_tracked(&commit, kit, &into, &withhold)?;
        rewritten += resolve_readme_links(&into, leaf, &leaves, &spec_base_url)?;
        place_license(license.as_deref(), &into)?;
        carried += carry_declarations(".", &commit, kit, &into, &withhold)?;
        packed += 1;
    }
    if packed == 0 {
        return Err(refuse(
            "no kit roots enumerated — the payload would be empty.",
        ));
    }

    let artifacts = pack_artifacts(&f.artifacts, &asm)?;

    stamp(&asm, &version, &commit, &spec_base_url)?;

    let tarball = npm_pack(&asm)?;
    let landed = format!("{}/{}", out.trim_end_matches('/'), tarball);
    move_file(&format!("{}/{}", asm, tarball), &landed)?;

    Ok(format!(
        "PACK: {} (version {}, commit {}, root {}, {} kit(s) in payload, {} README link(s) resolved to the published SPEC, {} close-surface declaration(s) carried, {} prebuilt gate binary/binaries)",
        landed,
        version,
        &commit[..12],
        root,
        packed,
        rewritten,
        carried,
        artifacts
    ))
}

// spec: gate-sdk/SPEC.md §Consumer payload — a packed README's link to any packed kit's SPEC is
// rewritten to the published location; the rewrite reaches only the extracted copy under
// `{asm}/payload/`, never the tracked README
fn resolve_readme_links(into: &str, leaf: &str, packed: &[String], base: &str) -> Result<usize, Refusal> {
    let path = format!("{}/README.md", into);
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(refuse(format!("cannot read {}: {}", path, e))),
    };
    let (body, count) = resolve_spec_links(&text, leaf, packed, base);
    if count == 0 {
        return Ok(0);
    }
    std::fs::write(&path, body).map_err(|e| refuse(format!("cannot write {}: {}", path, e)))?;
    Ok(count)
}

// spec: installer/SPEC.md §The packer — the license text comes from the stamped commit, so no
// worktree edit reaches the placed bytes; an explicitly empty knob places nothing, and a set one
// naming no regular file tracked there is a refusal rather than a redistribution without the text
fn license_text(commit: &str, name: &str) -> Result<Option<Vec<u8>>, Refusal> {
    let name = name.trim();
    if name.is_empty() {
        return Ok(None);
    }
    let short = &commit[..12.min(commit.len())];
    let untracked = || {
        refuse_help(
            format!(
                "GATE_SDK_PAYLOAD_LICENSE names {}, which is no regular file tracked at {}.",
                name, short
            ),
            &["name the license file the tree tracks, or set the knob explicitly empty to place none."],
        )
    };
    tracked_file(".", commit, name)?.map(Some).ok_or_else(untracked)
}

// spec: installer/SPEC.md §The packer — a path's bytes at the stamped commit when it is a regular
// file tracked there, and none when it is absent or anything else
fn tracked_file(repo: &str, commit: &str, path: &str) -> Result<Option<Vec<u8>>, Refusal> {
    let listing = git(&["-C", repo, "ls-tree", commit, "--", path])?;
    let mut lines = listing.lines();
    let (Some(line), None) = (lines.next(), lines.next()) else {
        return Ok(None);
    };
    let mut meta = line.split('\t').next().unwrap_or("").split_whitespace();
    let (Some(mode), Some("blob"), Some(object)) = (meta.next(), meta.next(), meta.next()) else {
        return Ok(None);
    };
    if !mode.starts_with("100") {
        return Ok(None);
    }
    let done = proc::run(&programs::GIT, &["-C", repo, "cat-file", "blob", object]).map_err(refuse)?;
    match done.stdout() {
        Some(b) => Ok(Some(b.to_vec())),
        None => Err(refuse(format!(
            "git cat-file could not read {} at {} — {}",
            path,
            &commit[..12.min(commit.len())],
            done.failure_report().unwrap_or_default()
        ))),
    }
}

// spec: gate-sdk/SPEC.md §Consumer payload — the carry would overwrite a shipped file of its name,
// so a kit root tracking one refuses before anything of the kit is extracted
fn refuse_tracked_carry(repo: &str, commit: &str, kit: &str) -> Result<(), Refusal> {
    let path = format!("{}/{}", kit.trim_end_matches('/'), close_surfaces::CARRIED);
    if git(&["-C", repo, "ls-tree", "--name-only", commit, "--", &path])?.is_empty() {
        return Ok(());
    }
    Err(refuse_help(
        format!("{} is tracked at {}, the name the packer carries the kit's close-surface declarations under.", path, &commit[..12.min(commit.len())]),
        &["rename or remove the tracked file; the carried one is generated from the kit's withheld spec at every pack."],
    ))
}

// spec: gate-sdk/SPEC.md §Consumer payload — each withheld member that is a file at the stamped
// commit gives up its `close-surface:` lines, read by the derivation's own line reader and written
// in source order beside the kit; a kit with none gets no file
fn carry_declarations(repo: &str, commit: &str, kit: &str, into: &str, withhold: &[String]) -> Result<usize, Refusal> {
    let mut lines: Vec<String> = Vec::new();
    for member in withhold {
        let member = member.trim_matches('/');
        if member.is_empty() {
            continue;
        }
        let path = format!("{}/{}", kit.trim_end_matches('/'), member);
        if let Some(bytes) = tracked_file(repo, commit, &path)? {
            let text = String::from_utf8_lossy(&bytes);
            lines.extend(close_surfaces::declaration_lines(&text).into_iter().map(str::to_string));
        }
    }
    write_carry(into, &lines)?;
    Ok(lines.len())
}

fn write_carry(into: &str, lines: &[String]) -> Result<(), Refusal> {
    if lines.is_empty() {
        return Ok(());
    }
    let mut body = String::from(CARRY_HEADER);
    for l in lines {
        body.push_str(l);
        body.push('\n');
    }
    let path = format!("{}/{}", into, close_surfaces::CARRIED);
    std::fs::write(&path, body).map_err(|e| refuse(format!("cannot write {}: {}", path, e)))
}

const CARRY_HEADER: &str = "# Generated at pack time from this kit's spec; lifecycle-kit/SPEC.md §The close-surface roster.\n";

// spec: installer/SPEC.md §The packer — each pair's name is typed by an adopter and carried by a
// directory, so a name outside `[a-z0-9][a-z0-9-]*` refuses before anything is extracted
fn recipe_plan(asm: &str, pairs: &[(String, String)]) -> Result<Vec<(String, String)>, Refusal> {
    let mut out = Vec::new();
    for (name, dir) in pairs {
        let lead = name.bytes().next();
        let ok = lead.is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
            && name.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
        if !ok {
            return Err(refuse_help(
                format!("GATE_SDK_PAYLOAD_RECIPES names the payload recipe '{}', outside [a-z0-9][a-z0-9-]*.", name),
                &["an adopter types the name and a directory carries it: lowercase letters, digits and hyphens, opening with a letter or digit."],
            ));
        }
        out.push((
            dir.trim().trim_end_matches('/').to_string(),
            format!("{}/recipes/{}", asm, name),
        ));
    }
    Ok(out)
}

fn tracked_under(commit: &str, dir: &str) -> Result<bool, Refusal> {
    if dir.is_empty() {
        return Ok(false);
    }
    Ok(!git(&["ls-tree", "-r", "--name-only", commit, "--", dir])?.is_empty())
}

fn place_license(text: Option<&[u8]>, dir: &str) -> Result<(), Refusal> {
    let Some(bytes) = text else {
        return Ok(());
    };
    let path = format!("{}/LICENSE", dir);
    std::fs::write(&path, bytes).map_err(|e| refuse(format!("cannot write {}: {}", path, e)))
}

// spec: gate-sdk/SPEC.md §check-packed-links — the rewrite the gate replays: one function, so the
// bytes the gate asserts on are the bytes the packer writes and no second implementation can drift
// spec: gate-sdk/SPEC.md §Consumer payload — an empty base rewrites nothing; a `../<leaf>/`
// target's leaf must be one `packed` names, and a fragment passes through unchanged
pub fn resolve_spec_links(text: &str, leaf: &str, packed: &[String], base: &str) -> (String, usize) {
    const OPEN: &str = "](";
    if base.is_empty() || leaf.is_empty() {
        return (text.to_string(), 0);
    }
    let base = base.trim_end_matches('/');
    let mut out = String::with_capacity(text.len());
    let mut count = 0usize;
    let mut rest = text;
    while let Some(at) = rest.find(OPEN) {
        let (head, tail) = rest.split_at(at);
        out.push_str(head);
        let after = &tail[OPEN.len()..];
        match spec_target(after, leaf, packed) {
            Some((target, frag, consumed)) => {
                out.push_str("](");
                out.push_str(base);
                out.push('/');
                out.push_str(target);
                out.push_str("/SPEC");
                if !frag.is_empty() {
                    out.push('#');
                    out.push_str(frag);
                }
                out.push(')');
                count += 1;
                rest = &after[consumed..];
            }
            None => {
                out.push_str(OPEN);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    (out, count)
}

// spec: gate-sdk/SPEC.md §Consumer payload — the target is `SPEC.md` or `../<packed>/SPEC.md`,
// exactly or with `#<fragment>`; yields its leaf, fragment and length through the closing paren
fn spec_target<'a>(after: &'a str, leaf: &'a str, packed: &'a [String]) -> Option<(&'a str, &'a str, usize)> {
    const NAME: &str = "SPEC.md";
    let (target, skip) = match after.strip_prefix("../") {
        Some(rel) => {
            let other = &rel[..rel.find('/')?];
            let p = packed.iter().find(|p| p.as_str() == other)?;
            (p.as_str(), "../".len() + other.len() + 1)
        }
        None => (leaf, 0),
    };
    let tail = after[skip..].strip_prefix(NAME)?;
    let used = skip + NAME.len();
    match tail.as_bytes().first() {
        Some(b')') => Some((target, "", used + 1)),
        Some(b'#') => tail.find(')').map(|end| (target, &tail[1..end], used + end + 1)),
        _ => None,
    }
}

// spec: installer/SPEC.md §The packer — a caller that already holds the tree it means says so;
// the value is validated to a work-tree top level because silently promoting a subdirectory is the
// same correction the flag exists to remove
// spec: installer/SPEC.md §The packer — the cwd selects whose tooling runs and `--root` which
// tree is packed; the front-end resolved the first before this arm ran, and this resolves the
// second
fn resolve_root(named: &str) -> Result<String, Refusal> {
    if named.is_empty() {
        return walk::toplevel_opt().map_err(refuse)?.ok_or_else(|| {
            refuse("not inside a git work tree — the payload's commit stamp has no source.")
        });
    }
    if !std::path::Path::new(named).is_dir() {
        return Err(refuse(format!("--root is not a directory: {}", named)));
    }
    let top = walk::toplevel_in_opt(named)
        .map_err(refuse)?
        .ok_or_else(|| refuse(format!("--root is not inside a git work tree: {}", named)))?;
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

// spec: installer/SPEC.md §The packer — the version comes from the tag, never from an edit to
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

// spec: installer/SPEC.md §The packer — the payload's tree footprint, DERIVED from the same two
// resolvers the pack loop itself runs so the refusal's corpus cannot drift from the packed set;
// the roster rides on `--artifacts`, which is what makes it a payload input at all
fn footprint(root: &str, artifacts: &str, installer: &str) -> Result<Vec<String>, Refusal> {
    let mut spec = vec![installer.to_string()];
    // spec: installer/SPEC.md §The packer — the pack loop's own on-disk `is_dir` test is
    // deliberately NOT applied here: filtering the pathspec by it would blind the refusal to the
    // one divergence only it can see
    // spec: gate-sdk/SPEC.md §Layout and configuration — a git pathspec, and the same resolver the
    // pack loop takes, so the two cannot spell one kit two ways
    for kit in walk::kit_roots().map_err(refuse)? {
        if let Some(p) = inside(root, kit.trim_end_matches('/')) {
            spec.push(p);
        }
    }
    let license = walk::knob_scalar("GATE_SDK_PAYLOAD_LICENSE").map_err(refuse)?;
    if let Some(p) = inside(root, license.trim()).filter(|_| !license.trim().is_empty()) {
        spec.push(p);
    }
    for (_, dir) in walk::knob_map("GATE_SDK_PAYLOAD_RECIPES").map_err(refuse)? {
        if let Some(p) = inside(root, dir.trim().trim_end_matches('/')) {
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

// spec: installer/SPEC.md §The packer — a member outside the packed tree is dropped rather than
// handed to git, which refuses one; the test is lexical because a kit root deleted from the
// worktree still belongs in the pathspec and cannot be canonicalized
// spec: gate-sdk/SPEC.md §The path-dialect contract — absoluteness is a TWO-dialect question and
// `walk::path_root` is its single owner, asked here rather than re-implemented; a foreign
// backslash spelling never reaches this site, the contract crossing dialect once at the producer
fn inside(root: &str, path: &str) -> Option<String> {
    let root = root.trim_end_matches('/');
    let rel = if walk::path_root(path).is_some() {
        if !walk::at_or_under(root, path) {
            return None;
        }
        walk::rel_under(root, path).unwrap_or_default().to_string()
    } else {
        path.to_string()
    };
    if rel.split('/').any(|s| s == "..") {
        return None;
    }
    Some(if rel.is_empty() { ".".to_string() } else { rel })
}

// spec: installer/SPEC.md §The packer — the refusal names the entries it found, bounded and with
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
// comment-tier-exempt: trailing whitespace only: a porcelain entry's first two
// bytes ARE its state code, so trimming both ends would re-column the one line the dirty
// diagnostic prints first, and no other caller here reads a leading blank
fn git(args: &[&str]) -> Result<String, Refusal> {
    let done = proc::run(&programs::GIT, args).map_err(refuse)?;
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
    let made = proc::run(&programs::MKTEMP, &["-d", &template]).map_err(refuse)?;
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
// spec: gate-sdk/SPEC.md §Consumer payload — the withheld shape as `git archive` pathspec
// exclusions, a member naming a directory on disk contributing its subtree form too
fn pathspec(src: &str, withhold: &[String]) -> Vec<String> {
    let mut spec = vec![src.to_string()];
    for member in withhold {
        let member = member.trim_matches('/');
        if member.is_empty() {
            continue;
        }
        let path = format!("{}/{}", src, member);
        spec.push(format!(":(exclude){}", path));
        if std::path::Path::new(&path).is_dir() {
            spec.push(format!(":(exclude){}/*", path));
        }
    }
    spec
}

fn pack_tracked(commit: &str, src: &str, dst: &str, withhold: &[String]) -> Result<(), Refusal> {
    let src = src.trim_end_matches('/');
    let spec = pathspec(src, withhold);
    // spec: gate-sdk/SPEC.md §Consumer payload — refuse an unvendorable tracked symlink BEFORE the
    // pipeline: a failure after it lands mid-kit having written a partial vendor, where a pre-flight
    // writes nothing and names the cause
    // spec: gate-sdk/SPEC.md §Consumer payload — the pre-flight reads the pathspec the archive
    // reads, so the fail-closed guarantee covers precisely what is packed and no refusal names a
    // path the payload was never going to carry
    let mut ls: Vec<&str> = vec!["ls-files", "-s", "-z", "--"];
    ls.extend(spec.iter().map(String::as_str));
    let listing = git(&ls)?;
    let links: Vec<&str> = listing
        .split('\0')
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
    let mut archive_args: Vec<&str> = vec!["archive", commit, "--"];
    archive_args.extend(spec.iter().map(String::as_str));
    let archive = proc::run(&programs::GIT, &archive_args).map_err(refuse)?;
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
    let done = proc::run_streamed(&programs::TAR, &["-x", &strip, "-C", dst], &bytes, Stderr::Inherit)
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
    if !std::path::Path::new(&roster).is_file() {
        return Err(refuse(format!(
            "no target roster at {} — there is no declared platform set to pack artifacts for.",
            roster
        )));
    }
    let text = std::fs::read_to_string(&roster)
        .map_err(|e| refuse(format!("cannot read the target roster at {}: {}", roster, e)))?;
    let targets = crate::registry::members(&text);
    if targets.is_empty() {
        return Err(refuse(format!(
            "the target roster at {} declares no targets.",
            roster
        )));
    }
    mkdir(&format!("{}/payload/artifact", asm))?;
    let mut packed = 0usize;
    for target in &targets {
        let src = format!("{}/{}", dir.trim_end_matches('/'), target);
        if !std::path::Path::new(&src).is_dir() {
            return Err(refuse_help(
                format!("roster target '{}' has no artifact directory at {}.", target, src),
                &["every declared target's build leg must have run; a roster target no leg built is a broken payload, not a narrower one."],
            ));
        }
        let binary = artifact_name(&src)?;
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

// spec: gate-sdk/SPEC.md §Consumer payload — a target's artifact name is discovered, the bootstrap's
// own selection rule: exactly one regular file not named `*.sha256`, so no reader spells a suffix
fn artifact_name(src: &str) -> Result<String, Refusal> {
    let entries = walk::list_dir(std::path::Path::new(src)).map_err(refuse)?;
    let names: Vec<String> = entries
        .into_iter()
        .filter(|(n, _)| !n.ends_with(".sha256") && std::path::Path::new(&format!("{}/{}", src, n)).is_file())
        .map(|(n, _)| n)
        .collect();
    match names.as_slice() {
        [one] => Ok(one.clone()),
        _ => Err(refuse(format!(
            "{} holds {} artifact(s) beside its sidecars, and a target's directory holds exactly one.",
            src,
            names.len()
        ))),
    }
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

// spec: installer/SPEC.md §The packer — the version-and-commit stamp, a `serde_json` edit rather
// than a `jq` spawn: the tool's last reader of that program leaves with the port
fn stamp(asm: &str, version: &str, commit: &str, spec_base_url: &str) -> Result<(), Refusal> {
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
    // spec: installer/SPEC.md §The manifest — a stamped value is present exactly when the publisher
    // set one; an empty string written here would be a placeholder standing in for an omission, and
    // an empty base means *resolve in the tree*, which is a value with no location to record
    if !spec_base_url.is_empty() {
        slot.insert(
            "spec_base_url".into(),
            serde_json::Value::String(spec_base_url.into()),
        );
    }
    let mut body = serde_json::to_string_pretty(&doc)
        .map_err(|e| refuse(format!("could not stamp installer/package.json: {}", e)))?;
    body.push('\n');
    std::fs::write(&path, body)
        .map_err(|e| refuse(format!("could not write {}: {}", path, e)))
}

// comment-tier-exempt: `npm pack` stays a spawn deliberately: reproducing the
// package format in-crate is a second implementation of a format, not a port
fn npm_pack(asm: &str) -> Result<String, Refusal> {
    let done = proc::run_merged_in(&programs::NPM, &["pack"], &[], Some(std::path::Path::new(asm)))
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

// comment-tier-exempt: the tarball is moved out of the scratch before the
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

    // spec: installer/SPEC.md §The packer — the flag roster's unknown-argument refusal survives
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

    // spec: installer/SPEC.md §The packer — the version regex admits a prerelease suffix, which
    // is the property installer/SPEC.md §Versioning rests its prerelease claim on.
    #[test]
    fn the_version_regex_admits_a_prerelease_suffix() {
        assert!(resolve_version("1.2.3").is_ok());
        assert!(resolve_version("1.2.3-rc.1").is_ok());
        assert!(resolve_version("1.2.3+build.5").is_ok());
        assert!(resolve_version("v1.2.3").is_err());
        assert!(resolve_version("1.2").is_err());
    }

    // spec: installer/SPEC.md §The packer — the commit stamp is a 40-hex object name, so a short
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

    #[test]
    fn every_refusal_carries_the_prefix_and_exits_two() {
        assert_eq!(report(&refuse("cause")), 2);
        assert_eq!(report(&refuse_help("cause", &["do this"])), 2);
    }

    // spec: installer/SPEC.md §The packer — a footprint member outside the packed tree is
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

    // spec: installer/SPEC.md §The packer — a worktree-deleted kit root is exactly what the
    // refusal must still see, so the whole tree is the pathspec when a member resolves to the root
    // itself rather than the member being silently dropped.
    #[test]
    fn a_member_that_is_the_root_itself_spells_the_whole_tree() {
        assert_eq!(inside("/w", "/w"), Some(".".to_string()));
    }

    // spec: gate-sdk/SPEC.md §Consumer payload — the declared shape becomes pathspec exclusions
    // beside the root, a directory member contributing its subtree form too; an empty shape packs
    // the root whole, which is what the `installer/` call site passes.
    #[test]
    fn the_withheld_shape_becomes_pathspec_exclusions_beside_the_root() {
        let dir = std::env::temp_dir().join(format!("cw-pack-pathspec.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("smoke")).expect("scratch kit root");
        std::fs::write(dir.join("SPEC.md"), "").expect("scratch spec file");
        let root = dir.display().to_string();

        assert_eq!(pathspec(&root, &[]), vec![root.clone()]);
        assert_eq!(
            pathspec(&root, &["SPEC.md".to_string()]),
            vec![root.clone(), format!(":(exclude){}/SPEC.md", root)]
        );
        assert_eq!(
            pathspec(&root, &["smoke".to_string()]),
            vec![
                root.clone(),
                format!(":(exclude){}/smoke", root),
                format!(":(exclude){}/smoke/*", root),
            ]
        );
        // comment-tier-exempt: a blank member is a local property of a whitespace-split value, and
        // the assertion is that it contributes nothing rather than excluding the root itself
        assert_eq!(
            pathspec(&root, &["/".to_string(), String::new()]),
            vec![root.clone()]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: installer/SPEC.md §The packer — the license text lands at the package root and in a
    // packed kit root byte for byte, and an explicitly empty knob places nothing in either.
    #[test]
    fn the_license_text_is_placed_at_both_sites_and_empty_places_nothing() {
        let dir = std::env::temp_dir().join(format!("cw-pack-license.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let leaf = dir.join("payload").join("k-kit");
        std::fs::create_dir_all(&leaf).expect("scratch assembly");
        let asm = dir.display().to_string();
        let kit = leaf.display().to_string();

        assert_eq!(license_text(&"a".repeat(40), "").expect("empty is no refusal"), None);
        assert_eq!(license_text(&"a".repeat(40), "  ").expect("blank is no refusal"), None);
        place_license(None, &asm).expect("nothing to place");
        assert!(!dir.join("LICENSE").exists());

        let text = b"Apache License\nVersion 2.0\n";
        place_license(Some(text), &asm).expect("package root placement");
        place_license(Some(text), &kit).expect("kit root placement");
        assert_eq!(std::fs::read(dir.join("LICENSE")).expect("root copy"), text);
        assert_eq!(std::fs::read(leaf.join("LICENSE")).expect("kit copy"), text);
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: installer/SPEC.md §The packer — two pairs place two recipes under the package root's
    // `recipes/`, the empty default places none, and a name outside the grammar refuses
    #[test]
    fn each_recipe_pair_lands_under_the_package_root_and_a_bad_name_refuses() {
        let pairs = vec![
            ("one".to_string(), "a/one/".to_string()),
            ("two-2".to_string(), "b/two".to_string()),
        ];
        assert_eq!(
            recipe_plan("/asm", &pairs).expect("two good pairs refused"),
            vec![
                ("a/one".to_string(), "/asm/recipes/one".to_string()),
                ("b/two".to_string(), "/asm/recipes/two-2".to_string()),
            ]
        );
        assert!(recipe_plan("/asm", &[]).expect("the empty default refused").is_empty());
        for bad in ["", "-lead", "Upper", "a/b", "dot.ted"] {
            let r = recipe_plan("/asm", &[(bad.to_string(), "d".to_string())]).expect_err(bad);
            assert!(r.cause.contains("outside [a-z0-9][a-z0-9-]*"), "{}", r.cause);
        }
    }

    // spec: installer/SPEC.md §The packer — a recipe directory holding no tracked file at the commit
    // is refused by the caller, whichever tree the test runs in
    #[test]
    fn a_recipe_directory_with_no_tracked_file_reads_as_none() {
        assert!(!tracked_under("HEAD", "no-such-recipe-dir").expect("the probe failed"));
        assert!(!tracked_under("HEAD", "").expect("the probe failed"));
        assert!(tracked_under("HEAD", "src").expect("the probe failed"));
    }

    // spec: installer/SPEC.md §The packer — a set knob naming no regular file tracked at the commit
    // is a refusal: an absent path and a directory both refuse, whichever tree the test runs in.
    #[test]
    fn a_license_name_tracked_as_no_file_is_a_refusal() {
        let absent = license_text("HEAD", "no-such-license-file.txt").expect_err("absent path refuses");
        assert!(absent.cause.contains("GATE_SDK_PAYLOAD_LICENSE names no-such-license-file.txt"));
        assert!(license_text("HEAD", "src").is_err());
        assert!(license_text(&"0".repeat(40), "LICENSE").is_err());
    }

    // spec: gate-sdk/SPEC.md §Consumer payload — the carry reads the withheld spec at the commit:
    // an unfenced declaration is carried and a fenced one is not, a spec with none writes no file,
    // a withheld directory contributes nothing, and a kit root tracking the carried name refuses
    #[test]
    fn the_carry_takes_unfenced_declarations_alone_and_a_tracked_carried_name_refuses() {
        let dir = std::env::temp_dir().join(format!("pack-installer-carry-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for kit in ["a-kit/smoke", "b-kit", "c-kit", "into-a", "into-b"] {
            std::fs::create_dir_all(dir.join(kit)).expect("mk");
        }
        std::fs::write(
            dir.join("a-kit/SPEC.md"),
            "# A\n\nclose-surface: .workflow/a.log advisory reclaim=x --emit capture-drain .workflow/a.log\n\n```\nclose-surface: <path> <mode>\n```\n",
        )
        .expect("w");
        std::fs::write(dir.join("a-kit/smoke/s.sh"), "close-surface: .workflow/no.log advisory\n").expect("w");
        std::fs::write(dir.join("b-kit/SPEC.md"), "# B, declaring nothing\n").expect("w");
        std::fs::write(dir.join("c-kit").join(close_surfaces::CARRIED), "shipped\n").expect("w");
        let repo = dir.display().to_string();
        let git_in = |a: &[&str]| {
            let mut v = vec!["-C", repo.as_str(), "-c", "user.email=t@t.invalid", "-c", "user.name=t"];
            v.extend_from_slice(a);
            proc::run(&programs::GIT, &v).expect("git").stdout().is_some()
        };
        assert!(git_in(&["init", "-q"]) && git_in(&["add", "a-kit", "b-kit", "c-kit"]) && git_in(&["commit", "-qm", "base"]));
        let withhold = vec!["SPEC.md".to_string(), "smoke".to_string()];
        let into_a = dir.join("into-a").display().to_string();
        let into_b = dir.join("into-b").display().to_string();
        let a = carry_declarations(&repo, "HEAD", "a-kit", &into_a, &withhold).expect("a carries");
        let b = carry_declarations(&repo, "HEAD", "b-kit", &into_b, &withhold).expect("b carries");
        let carried = std::fs::read_to_string(dir.join("into-a").join(close_surfaces::CARRIED)).unwrap_or_default();
        let b_wrote = dir.join("into-b").join(close_surfaces::CARRIED).exists();
        let ok = refuse_tracked_carry(&repo, "HEAD", "a-kit").is_ok();
        let refused = refuse_tracked_carry(&repo, "HEAD", "c-kit").err().map(|r| r.cause).unwrap_or_default();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(a, 1);
        assert_eq!(
            carried,
            format!("{}close-surface: .workflow/a.log advisory reclaim=x --emit capture-drain .workflow/a.log\n", CARRY_HEADER)
        );
        assert_eq!(b, 0);
        assert!(!b_wrote, "a spec declaring nothing wrote a carried file");
        assert!(ok);
        assert!(refused.contains("c-kit/close-surfaces.txt is tracked"), "{}", refused);
    }

    // spec: gate-sdk/SPEC.md §Consumer payload — an empty base rewrites nothing and returns the
    // source bytes, which is the knob's *resolve in the tree* meaning and the direction
    // `check-packed-links` assertion B holds: an unconditional rewrite fails this.
    #[test]
    fn an_empty_base_rewrites_nothing() {
        let src = "See [SPEC.md](SPEC.md), [SPEC.md](SPEC.md#stage-rules) and [g](../gate-sdk/SPEC.md).\n";
        let packed = vec!["queue-kit".to_string(), "gate-sdk".to_string()];
        assert_eq!(resolve_spec_links(src, "queue-kit", &packed, ""), (src.to_string(), 0));
        assert_eq!(resolve_spec_links(src, "", &packed, "https://h.test"), (src.to_string(), 0));
    }

    // spec: gate-sdk/SPEC.md §check-packed-links — the leaf is the packer's own directory name and
    // the fragment passes through unchanged; a trailing slash on the base is trimmed, matching
    // `resolved_location`'s spelling of the same published location.
    #[test]
    fn an_own_spec_link_resolves_to_the_published_target_with_its_fragment() {
        let (got, n) = resolve_spec_links(
            "a [SPEC.md](SPEC.md) b [x](SPEC.md#the-monitor-boundary) c\n",
            "site-kit",
            &["site-kit".to_string()],
            "https://h.test/",
        );
        assert_eq!(n, 2);
        assert_eq!(
            got,
            "a [SPEC.md](https://h.test/site-kit/SPEC) b [x](https://h.test/site-kit/SPEC#the-monitor-boundary) c\n"
        );
    }

    // spec: gate-sdk/SPEC.md §Consumer payload — a packed kit's `../<leaf>/SPEC.md` is rewritten
    // with its fragment; an unpacked leaf, a longer path, a `./` prefix, a link title and a prose
    // mention pass through, so no published location is invented for a leaf the pack lacks
    #[test]
    fn only_an_exact_packed_spec_target_is_rewritten() {
        let packed = vec!["k-kit".to_string(), "gate-sdk".to_string()];
        let (got, n) = resolve_spec_links("[a](../gate-sdk/SPEC.md#x)\n", "k-kit", &packed, "https://h.test");
        assert_eq!(n, 1);
        assert_eq!(got, "[a](https://h.test/gate-sdk/SPEC#x)\n");
        let src = "[a](../omega/SPEC.md) [b](docs/SPEC.md) [c](SPEC.md \"t\") `SPEC.md` [d](SPEC.mdx) \
                   [e](../gate-sdk/SPEC.mdx) [f](../gate-sdk/docs/SPEC.md) [g](./SPEC.md)\n";
        assert_eq!(resolve_spec_links(src, "k-kit", &packed, "https://h.test"), (src.to_string(), 0));
    }

    // spec: installer/SPEC.md §The packer — the diagnostic names the entries it found, bounded,
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
