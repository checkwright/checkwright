// spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the source stamp's producer; the source
// stamp's three git invocations are the ones gate_native_source_stamp re-runs tree-side
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[path = "src/release_stamp.rs"]
mod release_stamp;

// spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a binary carrying no stamp is
// exactly the artifact the oracle cannot hold, so the failure lands at build time
// where its cause is legible, never as an unstamped binary the gate reads as stale.
fn fail(what: &str) -> ! {
    panic!(
        "checkwright-gates build: {} — the source stamp could not be computed, and a \
         binary with no stamp is one no freshness oracle can hold. This crate builds \
         inside its own git checkout by construction (it is never vendored: \
         gate-sdk/SPEC.md §Consumer payload).",
        what
    );
}

fn git(dir: &str, args: &[&str], stdin: Option<&str>) -> String {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(dir).args(args);
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    if stdin.is_some() {
        cmd.stdin(Stdio::piped());
    }
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => fail(&format!("could not run `git {}`: {}", args.join(" "), e)),
    };
    if let Some(s) = stdin {
        let mut pipe = match child.stdin.take() {
            Some(p) => p,
            None => fail("git stdin pipe was not created"),
        };
        if let Err(e) = pipe.write_all(s.as_bytes()) {
            fail(&format!("could not write to `git {}`: {}", args.join(" "), e));
        }
    }
    let out = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => fail(&format!("could not read from `git {}`: {}", args.join(" "), e)),
    };
    if !out.status.success() {
        fail(&format!(
            "`git {}` exited {}: {}",
            args.join(" "),
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    match String::from_utf8(out.stdout) {
        Ok(s) => s,
        Err(_) => fail(&format!("`git {}` emitted non-UTF-8 output", args.join(" "))),
    }
}

fn main() {
    let dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(d) => d,
        Err(_) => fail("CARGO_MANIFEST_DIR is unset"),
    };

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the input set is derived, never
    // maintained; ls-files' index order is the shared ordering, taken rather than re-sorted
    let listing = git(&dir, &["ls-files"], None);
    let paths: Vec<&str> = listing.lines().filter(|l| !l.is_empty()).collect();
    if paths.is_empty() {
        fail("git ls-files reports no tracked file under the crate root");
    }

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — worktree content, not the index
    let mut args: Vec<&str> = vec!["hash-object", "--"];
    args.extend(paths.iter().copied());
    let hashed = git(&dir, &args, None);
    let hashes: Vec<&str> = hashed.lines().filter(|l| !l.is_empty()).collect();
    if hashes.len() != paths.len() {
        fail(&format!(
            "git hash-object returned {} hash(es) for {} path(s)",
            hashes.len(),
            paths.len()
        ));
    }

    let mut manifest = String::new();
    for (h, p) in hashes.iter().zip(paths.iter()) {
        manifest.push_str(h);
        manifest.push(' ');
        manifest.push_str(p);
        manifest.push('\n');
    }
    let stamp = git(&dir, &["hash-object", "--stdin"], Some(&manifest));
    let stamp = stamp.trim();
    if stamp.is_empty() {
        fail("git hash-object --stdin emitted no stamp");
    }
    println!("cargo:rustc-env=CHECKWRIGHT_SOURCE_STAMP={}", stamp);

    // spec: installer/SPEC.md §The consumer smoke — the triple the binary was built for, read by
    // `--installer-smoke` against the host bootstrap's detector, so no triple mapping is held here
    match std::env::var("TARGET") {
        Ok(t) => println!("cargo:rustc-env=CHECKWRIGHT_TARGET={}", t),
        Err(_) => panic!("checkwright-gates build: TARGET is unset"),
    }

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the two events that can change the
    // stamp are the two triggers: each input's own path, and the index. No directory sweep
    for p in &paths {
        println!("cargo:rerun-if-changed={}", p);
    }
    let gitdir = git(&dir, &["rev-parse", "--absolute-git-dir"], None);
    println!("cargo:rerun-if-changed={}/index", gitdir.trim());

    release(&dir);
    gate_modules(&dir);
}

// spec: gate-sdk/SPEC.md §run-gates — the release stamp and its exactness; a describe that fails
// on a tagless or shallow checkout empties the stamp and never fails the build
fn release(dir: &str) {
    let input = std::env::var("CHECKWRIGHT_BUILD_VERSION").ok();
    let described = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["describe", "--tags", "--match", "v[0-9]*"])
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok());
    let (stamp, exact) = release_stamp::stamp(input.as_deref(), described.as_deref());
    println!("cargo:rustc-env=CHECKWRIGHT_RELEASE={}", stamp);
    println!("cargo:rustc-env=CHECKWRIGHT_RELEASE_EXACT={}", if exact { "1" } else { "" });
    println!("cargo:rerun-if-env-changed=CHECKWRIGHT_BUILD_VERSION");

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — tags live under the common git directory,
    // and a trigger path is emitted only where it exists, since cargo reads a missing one as changed
    let common = git(dir, &["rev-parse", "--git-common-dir"], None);
    let common = Path::new(dir).join(common.trim());
    let tags = common.join("refs").join("tags");
    let watched = if tags.is_dir() { tags } else { common.join("refs") };
    for p in [watched, common.join("packed-refs")] {
        if p.exists() {
            println!("cargo:rerun-if-changed={}", p.display());
        }
    }
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the member-to-module map, read off each
// `REGISTRY` row's `<module>::run` path, so the registry stays the one table
fn gate_modules(dir: &str) {
    let path = format!("{}/src/gates/mod.rs", dir);
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => panic!("checkwright-gates build: cannot read {}: {}", path, e),
    };
    let mut out = String::from("pub const GATE_MODULES: &[(&str, &str)] = &[\n");
    let mut name: Option<&str> = None;
    for line in text.lines().map(str::trim) {
        if let Some(n) = line.strip_prefix('"').and_then(|l| l.strip_suffix("\",")) {
            name = Some(n);
        } else if let (Some(n), Some(m)) = (name, line.strip_suffix("::run,")) {
            out.push_str(&format!("    ({:?}, {:?}),\n", n, m));
            name = None;
        } else if !line.is_empty() && !line.starts_with("//") {
            name = None;
        }
    }
    out.push_str("];\n");
    let out_dir = match std::env::var("OUT_DIR") {
        Ok(d) => d,
        Err(_) => panic!("checkwright-gates build: OUT_DIR is unset"),
    };
    let dest = format!("{}/gate_modules.rs", out_dir);
    if let Err(e) = std::fs::write(&dest, out) {
        panic!("checkwright-gates build: cannot write {}: {}", dest, e);
    }
}
