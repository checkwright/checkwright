// spec: gate-sdk/SPEC.md §check-gate-binary-fresh — whenever a registered member resolving to a
// .gate descriptor makes the binary load-bearing, the binary was built from the source now in
// the tree
use crate::fresh;
use crate::proc;
use crate::registry;
use crate::walk;
use std::path::Path;

const REBUILD: &str = "bash gate-sdk/bin/build-native.sh";

// spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the tree side of the source stamp: the same
// three git invocations native/build.rs bakes into the binary. `None` where git cannot answer,
// so a caller fails closed rather than comparing against an empty string.
fn source_stamp(crate_dir: &str) -> Option<String> {
    let listing = proc::run("git", &["-C", crate_dir, "ls-files"])
        .ok()?
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())?;
    let paths: Vec<&str> = listing.lines().filter(|l| !l.is_empty()).collect();
    if paths.is_empty() {
        return None;
    }
    let mut args: Vec<&str> = vec!["-C", crate_dir, "hash-object", "--"];
    args.extend(paths.iter().copied());
    let hashed = proc::run("git", &args)
        .ok()?
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())?;
    let hashes: Vec<&str> = hashed.lines().filter(|l| !l.is_empty()).collect();
    if hashes.len() != paths.len() {
        return None;
    }
    let mut manifest = String::new();
    for (h, p) in hashes.iter().zip(paths.iter()) {
        manifest.push_str(h);
        manifest.push(' ');
        manifest.push_str(p);
        manifest.push('\n');
    }
    let stamp = proc::run_with_stdin(
        "git",
        &["-C", crate_dir, "hash-object", "--stdin"],
        manifest.as_bytes(),
    )
    .ok()?
    .stdout()
    .map(|o| String::from_utf8_lossy(o).into_owned())?;
    let stamp = stamp.lines().next().unwrap_or("").to_string();
    if stamp.is_empty() {
        return None;
    }
    Some(stamp)
}

fn is_executable(p: &str) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(p)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub fn run(args: &[String]) -> i32 {
    let gates_dir = match args.first() {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("GATE_SDK_GATES_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-gate-binary-fresh: {}", e);
                return 2;
            }
        },
    };
    let stamp_file = args.get(1).cloned().unwrap_or_default();
    let list = format!("{}/gates.list", gates_dir);
    let bin = match walk::knob_scalar("GATE_SDK_NATIVE_BIN") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-gate-binary-fresh: {}", e);
            return 2;
        }
    };
    let crate_dir = match walk::knob_scalar("GATE_SDK_NATIVE_CRATE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-gate-binary-fresh: {}", e);
            return 2;
        }
    };
    let mut resolve_dirs = vec![gates_dir.clone()];
    match walk::kit_roots_rel() {
        Ok(v) => resolve_dirs.extend(
            v.into_iter()
                .filter(|r| !r.is_empty())
                .map(|r| format!("{}/checks", r.trim_end_matches('/'))),
        ),
        Err(e) => {
            eprintln!("check-gate-binary-fresh: {}", e);
            return 2;
        }
    }

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the descriptor set is the trigger, derived
    // over the resolve dirs as check-gate-substrate-parity assertion B already derives it
    let mut descriptors: Vec<String> = Vec::new();
    for d in &resolve_dirs {
        if !fresh::is_dir(d) {
            continue;
        }
        for f in walk::glob_entries(&format!("{}/*.gate", d)) {
            if !Path::new(&f).is_file() {
                continue;
            }
            let base = f.rsplit('/').next().unwrap_or(&f);
            if let Some(stem) = base.strip_suffix(".gate") {
                descriptors.push(stem.to_string());
            }
        }
    }
    descriptors.sort();
    descriptors.dedup();

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a descriptor on disk is a declaration; a
    // registered member resolving to one is a dispatch, and only a dispatch makes the binary
    // load-bearing. The live registry is therefore an input, and an absent one is "cannot verify".
    if !Path::new(&list).is_file() {
        eprintln!("check-gate-binary-fresh: no gate registry at {} — the live member set is what decides whether the binary is load-bearing, so the check could not run; treating as failure (not clean)", list);
        eprintln!("  help: pass the gates dir carrying gates.list as the first argument, or set GATE_SDK_GATES_DIR.");
        return 2;
    }
    let listing = std::fs::read(&list)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    let dispatching: Vec<String> = registry::members(&listing)
        .into_iter()
        .filter(|m| registry::resolve(m, &resolve_dirs).map(|s| s.ends_with(".gate")) == Some(true))
        .collect();

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — nothing dispatching is a clean report, not
    // a skipped assertion: no gate dispatches to the binary, so nothing can run stale. Both counts
    // are named, so a reader can tell "no descriptors" from "descriptors nothing dispatches to".
    if dispatching.is_empty() {
        println!(
            "GATE-BINARY-FRESH: clean ({} .gate descriptor(s) across {} resolve dir(s), 0 dispatched to by a live member of {}, so nothing dispatches to {} and no build can be stale; crate {} unread)",
            descriptors.len(),
            resolve_dirs.len(),
            list,
            bin,
            crate_dir
        );
        return 0;
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — with the binary load-bearing, an absent or
    // unreadable one is "cannot verify", which must not share an exit code with "verified fresh"
    if !is_executable(&bin) {
        eprintln!("check-gate-binary-fresh: {} is absent or not executable, but {} registered member(s) dispatch to it — the check could not run; treating as failure (not clean)", bin, dispatching.len());
        eprintln!("  help: build it — {}", REBUILD);
        return 2;
    }

    let stamped = match proc::run(&bin, &["--source-stamp"]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("check-gate-binary-fresh: {}", e);
            return 2;
        }
    };
    let Some(out) = stamped.stdout() else {
        eprintln!(
            "check-gate-binary-fresh: {}",
            fresh::fail_closed(&format!("{} --source-stamp", bin), stamped.code())
        );
        return 2;
    };
    let baked = String::from_utf8_lossy(out).lines().next().unwrap_or("").to_string();
    if baked.is_empty() {
        eprintln!("check-gate-binary-fresh: {} --source-stamp reported no stamp — the check could not run; treating as failure (not clean)", bin);
        eprintln!("  help: rebuild it — {}", REBUILD);
        return 2;
    }

    let (tree, source_desc) = if !stamp_file.is_empty() {
        if std::fs::File::open(&stamp_file).is_err() {
            eprintln!(
                "check-gate-binary-fresh: tree-stamp file not readable: {}",
                stamp_file
            );
            return 2;
        }
        let text = match fresh::read_captured(&stamp_file) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("check-gate-binary-fresh: {}", e);
                return 2;
            }
        };
        (
            text.lines().next().unwrap_or("").to_string(),
            stamp_file.clone(),
        )
    } else {
        match source_stamp(&crate_dir) {
            Some(t) => (t, crate_dir.clone()),
            None => {
                eprintln!("check-gate-binary-fresh: git could not hash the tracked source under {} — the check could not run; treating as failure (not clean)", crate_dir);
                eprintln!("  help: the stamp is git's content identity for the crate's tracked source set, so the crate root must be a tracked directory inside a git worktree.");
                return 2;
            }
        }
    };
    if tree.is_empty() {
        eprintln!(
            "check-gate-binary-fresh: no tree-side stamp from {}",
            source_desc
        );
        return 2;
    }

    if baked != tree {
        println!("check-gate-binary-fresh: the gate binary was not built from the source now in the tree:");
        println!("  {} reports source stamp {}", bin, baked);
        println!("  {} hashes to {}", source_desc, tree);
        println!(
            "  {} descriptor(s) dispatch to that binary: {}",
            dispatching.len(),
            dispatching.join(" ")
        );
        println!("  help: rebuild it — {}", REBUILD);
        println!("        Until then the descriptor-named gate(s) above run the old implementation");
        println!("        and pass on code that is not what is committed.");
        return 1;
    }

    println!(
        "GATE-BINARY-FRESH: clean ({} .gate descriptor(s) across {} resolve dir(s), {} dispatched to by a live member of {}; {} built from the source now in {}, stamp {})",
        descriptors.len(),
        resolve_dirs.len(),
        dispatching.len(),
        list,
        bin,
        source_desc,
        baked
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the derivation the fixture pair cannot
    // reach: both its cases pass an explicit stamp file, so the git-based arm the gate exists for
    // is exercised here against the crate the test itself is compiled from.
    #[test]
    fn the_tree_side_stamp_is_the_one_the_build_baked() {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let stamp = source_stamp(crate_dir).expect("git could not hash the crate's tracked source");
        assert_eq!(
            stamp.len(),
            40,
            "the stamp is a git object name, so it is 40 hex characters: {}",
            stamp
        );
        assert_eq!(
            stamp,
            env!("CHECKWRIGHT_SOURCE_STAMP"),
            "the runtime derivation disagrees with the one build.rs baked — the two are one \
             algorithm and a divergence makes the freshness verdict meaningless"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — a crate root git cannot answer for is
    // "cannot verify", never a stamp compared against the empty string
    #[test]
    fn a_crate_root_git_cannot_hash_yields_no_stamp() {
        assert_eq!(source_stamp("/nonexistent-crate-root-checkwright"), None);
    }
}
