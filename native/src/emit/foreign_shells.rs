// spec: gate-sdk/SPEC.md §with-foreign-shells — a command run with `pwsh`, `dash` and `sh` on
// `PATH`, copied out of pinned images and run natively; provisioning compiles on Linux alone
#[cfg(target_os = "linux")]
use crate::{proc, programs, walk};
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};

pub const KNOBS: &[&str] = &[
    "GATE_SDK_FOREIGN_PWSH_IMAGE",
    "GATE_SDK_FOREIGN_DASH_IMAGE",
    "GATE_SDK_TMP_DIR",
];

const NAME: &str = "with-foreign-shells";
const USAGE: &str = "usage: --with-foreign-shells <command> [<arg>...]";

// spec: gate-sdk/SPEC.md §with-foreign-shells — the in-image paths, bound to the images the knob
// defaults name
#[cfg(target_os = "linux")]
const PWSH_DIR: &str = "/opt/microsoft/powershell/7";
#[cfg(target_os = "linux")]
const DASH_FILE: &str = "/usr/bin/dash";
#[cfg(target_os = "linux")]
const MARKER: &str = ".provisioned";

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("{}: no command given, so nothing ran\n{}", NAME, USAGE);
        return 2;
    }
    #[cfg(target_os = "linux")]
    {
        run_on_linux(args)
    }
    #[cfg(not(target_os = "linux"))]
    {
        eprintln!(
            "{}",
            skip_line("this host is not Linux, and the copied shells are Linux binaries", args)
        );
        0
    }
}

fn skip_line(reason: &str, args: &[String]) -> String {
    format!("{}: skipped — {}; {} not run", NAME, reason, args.join(" "))
}

#[cfg(target_os = "linux")]
fn run_on_linux(args: &[String]) -> i32 {
    if let Some(reason) = docker_skip(std::env::var_os("PATH").as_deref()) {
        eprintln!("{}", skip_line(&reason, args));
        return 0;
    }
    let path = match provision().and_then(|bin| search_path(&bin)) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };
    let mut argv: Vec<&str> = vec!["-c", "exec \"$@\"", "bash"];
    argv.extend(args.iter().map(String::as_str));
    match proc::run_to_env(&programs::BASH, &argv, &[("PATH".to_string(), path)], &proc::Sink::Inherit) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §with-foreign-shells — the skip reads the `PATH` it is handed, so a test
// decides it without writing the process environment
#[cfg(target_os = "linux")]
fn docker_skip(path: Option<&std::ffi::OsStr>) -> Option<String> {
    if proc::resolve_on_path("docker", path, None, proc::is_executable).is_none() {
        return Some("no docker on PATH".to_string());
    }
    match proc::run(&programs::DOCKER, &["info"]) {
        Ok(done) if done.stdout().is_some() => None,
        Ok(done) => Some(format!(
            "the Docker daemon did not answer (docker info exited {})",
            done.reported_code()
        )),
        Err(_) => Some("docker info could not run".to_string()),
    }
}

#[cfg(target_os = "linux")]
fn search_path(bin: &Path) -> Result<String, String> {
    let inherited = std::env::var_os("PATH").unwrap_or_default();
    let joined = std::env::join_paths(std::iter::once(bin.to_path_buf()).chain(std::env::split_paths(&inherited)))
        .map_err(|e| format!("cannot put {} on PATH: {}", bin.display(), e))?;
    joined
        .into_string()
        .map_err(|_| format!("cannot put {} on PATH: PATH is not UTF-8", bin.display()))
}

// spec: gate-sdk/SPEC.md §with-foreign-shells — one directory per pair of image references, reused
// once its marker exists; a failed build removes what it copied
#[cfg(target_os = "linux")]
fn provision() -> Result<PathBuf, String> {
    let pwsh = walk::knob_scalar("GATE_SDK_FOREIGN_PWSH_IMAGE")?;
    let dash = walk::knob_scalar("GATE_SDK_FOREIGN_DASH_IMAGE")?;
    let tmp = walk::abs_against(&walk::cwd()?, &walk::knob_scalar("GATE_SDK_TMP_DIR")?);
    let dir = Path::new(&tmp).join("foreign-shells").join(key(&pwsh, &dash));
    if dir.join(MARKER).is_file() {
        return Ok(dir.join("bin"));
    }
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
    if let Err(e) = populate(&dir, &pwsh, &dash) {
        let _ = std::fs::remove_dir_all(&dir);
        return Err(e);
    }
    Ok(dir.join("bin"))
}

#[cfg(target_os = "linux")]
fn key(pwsh: &str, dash: &str) -> String {
    crate::sha256::hex(format!("{}\n{}", pwsh, dash).as_bytes())[..16].to_string()
}

#[cfg(target_os = "linux")]
fn populate(dir: &Path, pwsh: &str, dash: &str) -> Result<(), String> {
    if !pwsh.is_empty() {
        copy_out(pwsh, PWSH_DIR, &dir.join("pwsh"))?;
    }
    if !dash.is_empty() {
        copy_out(dash, DASH_FILE, &dir.join("dash"))?;
    }
    let bin = link_bin(dir, !pwsh.is_empty(), !dash.is_empty())?;
    if !pwsh.is_empty() {
        probe(&programs::PWSH, &bin, &["-NoProfile", "-Command", "exit"], pwsh)?;
    }
    if !dash.is_empty() {
        probe(&programs::DASH, &bin, &["-c", ":"], dash)?;
    }
    std::fs::write(dir.join(MARKER), "").map_err(|e| format!("cannot write {}: {}", dir.join(MARKER).display(), e))
}

#[cfg(target_os = "linux")]
fn copy_out(image: &str, from: &str, to: &Path) -> Result<(), String> {
    eprintln!("{}: copying {} out of {}", NAME, from, image);
    let created = proc::run(&programs::DOCKER, &["create", image])?;
    let id = match created.stdout() {
        Some(out) => String::from_utf8_lossy(out).trim().lines().last().unwrap_or("").to_string(),
        None => {
            return Err(format!(
                "docker create {} failed: {}",
                image,
                created.failure_report().unwrap_or_default()
            ))
        }
    };
    if id.is_empty() {
        return Err(format!("docker create {} printed no container id", image));
    }
    let dest = to.display().to_string();
    let copied = proc::run(&programs::DOCKER, &["cp", &format!("{}:{}", id, from), &dest]);
    let _ = proc::run(&programs::DOCKER, &["rm", &id]);
    match copied?.failure_report() {
        Some(r) => Err(format!("docker cp {} out of {} failed: {}", from, image, r)),
        None => Ok(()),
    }
}

// spec: gate-sdk/SPEC.md §with-foreign-shells — `bin/` holds relative links, so the directory
// stays valid wherever the scratch root sits
#[cfg(target_os = "linux")]
fn link_bin(dir: &Path, pwsh: bool, dash: bool) -> Result<PathBuf, String> {
    let bin = dir.join("bin");
    std::fs::create_dir_all(&bin).map_err(|e| format!("cannot create {}: {}", bin.display(), e))?;
    let mut links: Vec<(&str, &str)> = Vec::new();
    if pwsh {
        links.push(("pwsh", "../pwsh/pwsh"));
    }
    if dash {
        links.push(("dash", "../dash"));
        links.push(("sh", "../dash"));
    }
    for (name, target) in links {
        std::os::unix::fs::symlink(target, bin.join(name))
            .map_err(|e| format!("cannot link {}: {}", bin.join(name).display(), e))?;
    }
    Ok(bin)
}

// spec: gate-sdk/SPEC.md §with-foreign-shells — an image built against a newer C library than the
// host's yields a copy that cannot start, which is refused here rather than inside the command
#[cfg(target_os = "linux")]
fn probe(shell: &programs::Program, bin: &Path, args: &[&str], image: &str) -> Result<(), String> {
    let name = shell.name();
    let copy = shell.clone().at(bin.join(&name).display().to_string());
    let failed = match proc::run(&copy, args) {
        Ok(done) => done.failure_report(),
        Err(e) => Some(e),
    };
    match failed {
        Some(r) => Err(format!(
            "the {} copied out of {} fails its probe, so it cannot run on this host: {}",
            name, image, r
        )),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §with-foreign-shells — no command is the arm's own exit 2
    #[test]
    fn no_command_is_a_refusal() {
        assert_eq!(run(&[]), 2);
    }

    // spec: gate-sdk/SPEC.md §with-foreign-shells — the skip line names its reason and the command
    // it did not run, which is what tells a skip from a run
    #[test]
    fn the_skip_line_names_the_reason_and_the_command_not_run() {
        let args = ["bash".to_string(), "run-smoke.sh".to_string()];
        assert_eq!(
            skip_line("no docker on PATH", &args),
            "with-foreign-shells: skipped — no docker on PATH; bash run-smoke.sh not run"
        );
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn off_linux_every_command_skips() {
        assert_eq!(run(&["true".to_string()]), 0);
    }

    #[cfg(target_os = "linux")]
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("checkwright-foreign-shells.{}.{}", std::process::id(), name));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    // spec: gate-sdk/SPEC.md §with-foreign-shells — a `PATH` resolving no `docker` skips before any
    // daemon question is asked
    #[cfg(target_os = "linux")]
    #[test]
    fn a_path_with_no_docker_skips() {
        let empty = scratch("no-docker");
        assert_eq!(docker_skip(Some(empty.as_os_str())).as_deref(), Some("no docker on PATH"));
        assert_eq!(docker_skip(None).as_deref(), Some("no docker on PATH"));
        let _ = std::fs::remove_dir_all(&empty);
    }

    // spec: gate-sdk/SPEC.md §with-foreign-shells — `bin/` over a constructed copy directory: `pwsh`
    // reaches the copied tree, `dash` and `sh` the one dash, and an empty knob links nothing
    #[cfg(target_os = "linux")]
    #[test]
    fn bin_links_each_provisioned_shell_and_sh_to_dash() {
        let dir = scratch("layout");
        std::fs::create_dir_all(dir.join("pwsh")).expect("pwsh tree");
        std::fs::write(dir.join("pwsh").join("pwsh"), "p").expect("pwsh");
        std::fs::write(dir.join("dash"), "d").expect("dash");
        let bin = link_bin(&dir, true, true).expect("links");
        assert_eq!(bin, dir.join("bin"));
        for (name, body) in [("pwsh", "p"), ("dash", "d"), ("sh", "d")] {
            assert_eq!(std::fs::read_to_string(bin.join(name)).expect(name), body, "{}", name);
        }
        let only_dash = scratch("dash-only");
        std::fs::write(only_dash.join("dash"), "d").expect("dash");
        let bin = link_bin(&only_dash, false, true).expect("links");
        assert!(std::fs::symlink_metadata(bin.join("pwsh")).is_err());
        assert!(bin.join("sh").is_file());
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::remove_dir_all(&only_dash);
    }

    // spec: gate-sdk/SPEC.md §with-foreign-shells — a changed reference on either knob is a new
    // directory, so a bump never reuses the old copies
    #[cfg(target_os = "linux")]
    #[test]
    fn each_reference_pair_keys_its_own_directory() {
        let k = key("p@sha256:1", "d@sha256:2");
        assert_eq!(k.len(), 16);
        assert_ne!(k, key("p@sha256:1", "d@sha256:3"));
        assert_ne!(k, key("d@sha256:2", "p@sha256:1"));
        assert_ne!(key("", "x"), key("x", ""));
    }
}
