// spec: gate-sdk/SPEC.md §Consumer smoke — the scratch-consumer builder: one holder of the build
// every consumer-smoke caller shares, so no caller keeps a copy of the mechanics.
use crate::proc::{self, Sink};
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §Consumer smoke — the built consumer: `dir` is what every caller's later
// steps and teardown read, `installed` the count the harness's clean line prints.
pub struct Scratch {
    pub dir: String,
    pub installed: usize,
}

// spec: gate-sdk/SPEC.md §Consumer smoke — a failed build still hands back the directory it
// created (empty when none was), so the caller's teardown and `--keep` see it.
pub struct BuildError {
    pub dir: String,
    pub message: String,
}

// spec: gate-sdk/SPEC.md §Consumer smoke — each variant is an environment failure, which every
// caller renders in its own verdict grammar.
pub enum PlaceError {
    NoHost { descriptors: usize },
    ArtifactUnusable { descriptors: usize, path: String },
    Io(String),
}

impl PlaceError {
    pub fn lines(&self) -> Vec<String> {
        match self {
            PlaceError::NoHost { descriptors } => vec![format!(
                "{} vendored .gate descriptor(s) need the gate binary and the caller named no checkout to take one from",
                descriptors
            )],
            PlaceError::ArtifactUnusable { descriptors, path } => vec![
                format!(
                    "{} vendored .gate descriptor(s) need the gate binary, but {} is absent or not executable",
                    descriptors, path
                ),
                "  help: build it — bash gate-sdk/bin/build-native.sh — then re-run.".to_string(),
            ],
            PlaceError::Io(e) => vec![e.clone()],
        }
    }
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the one derivation of whether a kit set needs the binary
pub fn gate_descriptors(roots: &[String]) -> usize {
    roots
        .iter()
        .map(|r| {
            walk::list_dir(&Path::new(r).join("checks"))
                .map(|entries| {
                    entries
                        .iter()
                        .filter(|(n, _)| !n.starts_with('.') && n.ends_with(".gate"))
                        .count()
                })
                .unwrap_or(0)
        })
        .sum()
}

// spec: gate-sdk/SPEC.md §Consumer smoke — a copy out of the checkout the caller names, never a
// build, landing at the kit-default path with no consumer config written
pub fn place_binary(consumer: &str, host: &str, roots: &[String]) -> Result<(), PlaceError> {
    let descriptors = gate_descriptors(roots);
    if descriptors == 0 {
        return Ok(());
    }
    if host.is_empty() {
        return Err(PlaceError::NoHost { descriptors });
    }
    let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN").map_err(PlaceError::Io)?;
    let from = format!("{}/{}", host, bin);
    if !proc::is_executable(Path::new(&from)) {
        return Err(PlaceError::ArtifactUnusable {
            descriptors,
            path: from,
        });
    }
    let to = format!("{}/{}", consumer, bin);
    if let Some(parent) = Path::new(&to).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| PlaceError::Io(format!("cannot create {}: {}", parent.display(), e)))?;
    }
    std::fs::copy(&from, &to)
        .map_err(|e| PlaceError::Io(format!("cannot copy {} to {}: {}", from, to, e)))?;
    Ok(())
}

fn basename(p: &str) -> &str {
    p.trim_end_matches('/').rsplit('/').next().unwrap_or(p)
}

fn git(dir: &str, args: &[&str]) -> Result<(), String> {
    let mut argv: Vec<&str> = vec!["-C", dir];
    argv.extend_from_slice(args);
    let done = proc::run("git", &argv)?;
    match done.failure_report() {
        None => Ok(()),
        Some(r) => Err(format!("git {} failed in {} — {}", args.join(" "), dir, r)),
    }
}

fn commit(dir: &str, extra: &str, message: &str) -> Result<(), String> {
    git(dir, &["add", "-A"])?;
    git(
        dir,
        &[
            "-c",
            "user.email=smoke@example.invalid",
            "-c",
            "user.name=smoke",
            "commit",
            "-q",
            extra,
            "-m",
            message,
        ],
    )
}

// spec: gate-sdk/SPEC.md §Consumer smoke — temp dir under `base`, seed commit, vendor by copy, the
// placement, each kit's `smoke/install.sh` in root order with its output routed to `out`, then the
// installed-baseline commit; the caller owns cleanup and every assertion after it
pub fn vendor_and_install(
    host: &str,
    roots: &[String],
    base: &str,
    out: &Sink,
) -> Result<Scratch, BuildError> {
    let template = format!("{}/consumer-smoke.XXXXXX", base.trim_end_matches('/'));
    let made = proc::run("mktemp", &["-d", &template]).map_err(|e| BuildError {
        dir: String::new(),
        message: e,
    })?;
    let dir = made
        .stdout()
        .map(|o| String::from_utf8_lossy(o).trim().to_string())
        .filter(|d| !d.is_empty())
        .ok_or_else(|| BuildError {
            dir: String::new(),
            message: format!("cannot create a scratch directory under {}", base),
        })?;
    let fail = |message: String| BuildError {
        dir: dir.clone(),
        message,
    };

    git(&dir, &["init", "-q"]).map_err(fail)?;
    // spec: gate-sdk/SPEC.md §Consumer smoke — the placed binary is ignored rather than tracked,
    // because the violation restore's `git clean -fd` spares ignored paths
    let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN").map_err(fail)?;
    std::fs::write(format!("{}/.gitignore", dir), format!(".tmp/\n{}\n", bin))
        .map_err(|e| fail(format!("cannot write {}/.gitignore: {}", dir, e)))?;
    commit(&dir, "--allow-empty", "seed").map_err(fail)?;

    for r in roots {
        let into = format!("{}/{}", dir, basename(r));
        let copied = proc::run("cp", &["-R", r, &into]).map_err(fail)?;
        if let Some(report) = copied.failure_report() {
            return Err(fail(format!("could not vendor {} — {}", r, report)));
        }
    }

    place_binary(&dir, host, roots).map_err(|e| fail(e.lines().join("\n")))?;

    let mut installed = 0;
    for r in roots {
        let kit = basename(r);
        let script = format!("{}/{}/smoke/install.sh", dir, kit);
        let env = [
            ("GATE_SDK_ROOT".to_string(), format!("{}/gate-sdk", dir)),
            ("SMOKE_KIT_ROOT".to_string(), format!("{}/{}", dir, kit)),
        ];
        let code = proc::run_to_in("bash", &[&script], &env, Some(Path::new(&dir)), out)
            .map_err(fail)?;
        if code != 0 {
            return Err(fail(format!(
                "{}/smoke/install.sh failed (a broken installer is an environment failure)",
                kit
            )));
        }
        installed += 1;
    }

    commit(&dir, "--no-verify", "installed baseline").map_err(fail)?;
    Ok(Scratch { dir, installed })
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §Consumer smoke — only `.gate` descriptors directly under each root's
    // `checks/` count, and a root without `checks/` counts none rather than failing
    #[test]
    fn descriptors_count_gate_files_under_checks_alone() {
        let base = std::env::temp_dir().join(format!("csmoke-desc.{}", std::process::id()));
        let kit = base.join("kit");
        std::fs::create_dir_all(kit.join("checks")).unwrap();
        for f in ["check-a.gate", "check-b.gate", "check-c.sh", ".hidden.gate"] {
            std::fs::write(kit.join("checks").join(f), "").unwrap();
        }
        let bare = base.join("bare");
        std::fs::create_dir_all(&bare).unwrap();
        let roots = vec![kit.display().to_string(), bare.display().to_string()];
        assert_eq!(gate_descriptors(&roots), 2);
        assert!(place_binary("/nonexistent", "", &roots[1..]).is_ok());
        assert!(matches!(
            place_binary("/nonexistent", "", &roots),
            Err(PlaceError::NoHost { descriptors: 2 })
        ));
        let _ = std::fs::remove_dir_all(&base);
    }
}
