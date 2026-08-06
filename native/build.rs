// spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the source stamp's producer; the
// three git invocations here are the ones gate_native_source_stamp re-runs tree-side
use std::io::Write;
use std::process::{Command, Stdio};

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

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the two events that can change the
    // stamp are the two triggers: each input's own path, and the index. No directory sweep
    for p in &paths {
        println!("cargo:rerun-if-changed={}", p);
    }
    let gitdir = git(&dir, &["rev-parse", "--absolute-git-dir"], None);
    println!("cargo:rerun-if-changed={}/index", gitdir.trim());
}
