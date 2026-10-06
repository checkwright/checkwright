// spec: gate-sdk/SPEC.md §git-hook — the binary under a served hook's name: it starts the binary
// `GATE_SDK_NATIVE_BIN` names on the `--git-hook` arm and returns its status, never judging itself
// spec: gate-sdk/SPEC.md §install-hooks — the hooks directory's one derivation, read by the arm that
// places it and by the installer verbs that report and remove it
use crate::{proc, programs, walk};
use std::path::{Path, PathBuf};

pub const SERVED: &[&str] = &["pre-commit", "commit-msg"];

pub const DIR: &str = "gate-hooks";

const BIN_KNOB: &str = "GATE_SDK_NATIVE_BIN";

fn strip_suffix_of<'a>(name: &'a str, suffix: &str) -> &'a str {
    if suffix.is_empty() || name.len() <= suffix.len() || !name.is_char_boundary(name.len() - suffix.len()) {
        return name;
    }
    let (stem, tail) = name.split_at(name.len() - suffix.len());
    if tail.eq_ignore_ascii_case(suffix) {
        stem
    } else {
        name
    }
}

fn served_under(started_as: &str, suffix: &str) -> Option<&'static str> {
    let name = Path::new(started_as).file_name()?.to_str()?;
    let stem = strip_suffix_of(name, suffix);
    SERVED.iter().copied().find(|h| *h == stem)
}

pub fn served(started_as: &str) -> Option<&'static str> {
    served_under(started_as, std::env::consts::EXE_SUFFIX)
}

pub fn file_name(hook: &str) -> String {
    format!("{}{}", hook, std::env::consts::EXE_SUFFIX)
}

pub fn hooks_dir(root: &Path) -> Option<PathBuf> {
    let r = root.to_string_lossy().into_owned();
    let out = proc::run(&programs::GIT, &["-C", &r, "rev-parse", "--git-common-dir"]).ok()?;
    let dir = String::from_utf8_lossy(out.stdout()?).trim().to_string();
    if dir.is_empty() {
        return None;
    }
    Some(PathBuf::from(walk::abs_against(&r, &dir)).join(DIR))
}

fn same_dir(a: &Path, b: &Path) -> bool {
    match (walk::canonicalize(a), walk::canonicalize(b)) {
        (Some(x), Some(y)) => x == y,
        _ => a == b,
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — opted in is `core.hooksPath` naming the hooks directory
pub fn opted_in(root: &Path, dir: &Path) -> bool {
    let r = root.to_string_lossy().into_owned();
    let Ok(out) = proc::run(&programs::GIT, &["-C", &r, "config", "--get", "core.hooksPath"]) else {
        return false;
    };
    let value = String::from_utf8_lossy(out.stdout().unwrap_or_default()).trim().to_string();
    !value.is_empty() && same_dir(Path::new(&walk::abs_against(&r, &value)), dir)
}

pub fn missing(dir: &Path) -> Vec<String> {
    SERVED.iter().map(|h| file_name(h)).filter(|f| !dir.join(f).is_file()).collect()
}

fn refusal(hook: &str, bin: &str, cause: &str) -> String {
    format!(
        "{}: cannot start the gate binary at {} ({}) — the hook could not run; treating as failure (not clean)\n  help: place or build the binary {} names, or bypass once with: git commit --no-verify",
        hook, bin, cause, BIN_KNOB
    )
}

pub fn launch(hook: &str, args: &[String]) -> i32 {
    let bin = match walk::knob_scalar(BIN_KNOB) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}: {} — the hook could not run; treating as failure (not clean)", hook, e);
            return 2;
        }
    };
    if served(&bin).is_some() {
        eprintln!(
            "{}: {} names {}, itself a hook's name, so the launcher would start itself — the hook could not run; treating as failure (not clean)\n  help: point {} at the gate binary",
            hook, BIN_KNOB, bin, BIN_KNOB
        );
        return 2;
    }
    let mut argv: Vec<&str> = vec!["--git-hook", hook];
    argv.extend(args.iter().map(String::as_str));
    match proc::run_to(&programs::CHECKWRIGHT_GATES.at(bin.clone()), &argv, &proc::Sink::Inherit) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}", refusal(hook, &bin, &e));
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §git-hook — the served set is closed, read off the start name's stem
    // with the host's executable suffix removed
    #[test]
    fn only_a_served_hook_name_under_the_host_suffix_is_a_launcher() {
        assert_eq!(served_under("/r/.git/gate-hooks/pre-commit", ""), Some("pre-commit"));
        assert_eq!(served_under("commit-msg", ""), Some("commit-msg"));
        assert_eq!(served_under("C:/r/.git/gate-hooks/pre-commit.exe", ".exe"), Some("pre-commit"));
        assert_eq!(served_under("commit-msg.EXE", ".exe"), Some("commit-msg"));
        assert_eq!(served_under("pre-commit", ".exe"), Some("pre-commit"));
        for other in ["pre-commit.exe", "pre-push", "checkwright-gates", "scripts/pre-commit.sh", ""] {
            assert_eq!(served_under(other, ""), None, "{}", other);
        }
        assert_eq!(served_under("pre-push.exe", ".exe"), None);
        assert_eq!(served_under(".exe", ".exe"), None);
    }

    // spec: gate-sdk/SPEC.md §git-hook — a binary that cannot be started is refused naming the
    // resolved path and both remedies
    #[test]
    fn the_refusal_names_the_path_and_both_remedies() {
        let r = refusal("pre-commit", "scripts/absent", "no such file");
        assert!(r.starts_with("pre-commit: cannot start the gate binary at scripts/absent"), "{}", r);
        assert!(r.contains("place or build the binary GATE_SDK_NATIVE_BIN names"), "{}", r);
        assert!(r.contains("git commit --no-verify"), "{}", r);
    }

    #[test]
    fn a_hooks_directory_missing_a_served_file_names_it() {
        let d = std::env::temp_dir().join(format!("checkwright-hook-launcher.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        std::fs::write(d.join(file_name("pre-commit")), "").expect("write");
        let gone = missing(&d);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(gone, vec![file_name("commit-msg")]);
    }
}
