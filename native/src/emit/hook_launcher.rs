// spec: gate-sdk/SPEC.md §git-hook — the binary under a served hook's name: it starts the binary
// `GATE_SDK_NATIVE_BIN` names on the `--git-hook` arm and returns its status, never judging itself
// spec: gate-sdk/SPEC.md §install-hooks — the hooks directory's one derivation, read by the arm that
// places it and by the installer verbs that report and remove it
use crate::knobs::{self, gate_sdk};
use crate::{knobfile, proc, programs, walk};
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

pub fn same_dir(a: &Path, b: &Path) -> bool {
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

// spec: gate-sdk/SPEC.md §git-hook — the launcher's read of its one knob: a scalar's precedence over
// the two knob files, each parsed only on the lines naming it, so a name this build's table lacks is
// the started build's to judge
fn bin_knob() -> Result<String, String> {
    let named = std::env::var(gate_sdk::KIT.knob_file_var()).ok().filter(|v| !v.is_empty());
    bin_from(std::env::var(BIN_KNOB).ok(), &knobs::gates_dir(), named.as_deref())
}

fn bin_from(env: Option<String>, dir: &str, named: Option<&str>) -> Result<String, String> {
    let stem = gate_sdk::KIT.stem();
    let set = match env {
        Some(v) => Some(v),
        None => match in_file(&format!("{}/{}-config.local.knobs", dir, stem), false)? {
            Some(v) => Some(v),
            None => match named {
                Some(f) => in_file(f, true)?,
                None => in_file(&format!("{}/{}-config.knobs", dir, stem), false)?,
            },
        },
    };
    Ok(set.filter(|v| !v.is_empty()).unwrap_or_else(gate_sdk::host_native_bin))
}

fn in_file(path: &str, named: bool) -> Result<Option<String>, String> {
    if !Path::new(path).is_file() {
        if named {
            return Err(format!(
                "{} names {}, which does not exist, so {} cannot be read",
                gate_sdk::KIT.knob_file_var(),
                path,
                BIN_KNOB
            ));
        }
        return Ok(None);
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    let mut found: Option<String> = None;
    for (idx, line) in text.lines().enumerate().filter(|(_, l)| knobfile::names(l, BIN_KNOB)) {
        let lno = idx + 1;
        match knobfile::parse_line(line, path, lno)? {
            Some(e) if e.form == knobfile::Form::Scalar && found.is_none() => found = Some(e.value),
            Some(e) if e.form == knobfile::Form::Scalar => {
                return Err(format!("{}:{}: {} is given twice — keep one line for it", path, lno, BIN_KNOB));
            }
            _ => {
                return Err(format!(
                    "{}:{}: {} is a scalar and this line is not that form — write `{} = value`",
                    path, lno, BIN_KNOB, BIN_KNOB
                ));
            }
        }
    }
    Ok(found)
}

pub fn launch(hook: &str, args: &[String]) -> i32 {
    let bin = match bin_knob() {
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

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("checkwright-hook-launcher-{}.{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        d
    }

    // spec: gate-sdk/SPEC.md §git-hook — the launcher reads its one knob by name: a line naming any
    // other knob, a name no table holds and a line no grammar admits among them, is skipped unread
    #[test]
    fn the_read_returns_the_named_binary_past_lines_no_table_holds() {
        let d = scratch("read");
        let dir = d.display().to_string();
        let tracked = d.join("gate-sdk-config.knobs");
        let unheld = format!("{}MINTED_AFTER_THIS_BUILD", gate_sdk::KIT.prefix());
        std::fs::write(
            &tracked,
            format!("{} = x\nnot a knob line at all\n{}_OTHER[] = y\n  {} = bin/tracked\n", unheld, BIN_KNOB, BIN_KNOB),
        )
        .expect("write");
        let from_tracked = bin_from(None, &dir, None);
        let from_env = bin_from(Some("bin/env".to_string()), &dir, None);
        let empty_env = bin_from(Some(String::new()), &dir, None);
        std::fs::write(d.join("gate-sdk-config.local.knobs"), format!("{}[k] = 1\n{} = bin/local\n", unheld, BIN_KNOB)).expect("write");
        let from_local = bin_from(None, &dir, None);
        std::fs::write(d.join("gate-sdk-config.local.knobs"), "GATE_SDK_NATIVE_BIN =\n").expect("write");
        let empty_local = bin_from(None, &dir, None);
        std::fs::remove_file(d.join("gate-sdk-config.local.knobs")).expect("remove");
        let named = d.join("elsewhere.knobs");
        std::fs::write(&named, "GATE_SDK_NATIVE_BIN = bin/named\n").expect("write");
        let from_named = bin_from(None, &dir, Some(&named.display().to_string()));
        std::fs::remove_file(&tracked).expect("remove");
        let from_default = bin_from(None, &dir, None);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(from_tracked, Ok("bin/tracked".to_string()));
        assert_eq!(from_env, Ok("bin/env".to_string()));
        assert_eq!(empty_env, Ok(gate_sdk::host_native_bin()));
        assert_eq!(from_local, Ok("bin/local".to_string()));
        assert_eq!(empty_local, Ok(gate_sdk::host_native_bin()));
        assert_eq!(from_named, Ok("bin/named".to_string()));
        assert_eq!(from_default, Ok(gate_sdk::host_native_bin()));
    }

    // spec: gate-sdk/SPEC.md §git-hook — a line setting the knob that is no scalar, the knob given
    // twice, and a named knob file that is absent each leave no binary to start, so each refuses
    // naming its file
    #[test]
    fn a_line_setting_the_knob_that_is_no_scalar_refuses_with_its_file_and_line() {
        let d = scratch("refuse");
        let dir = d.display().to_string();
        let tracked = d.join("gate-sdk-config.knobs");
        let mut got: Vec<(String, String)> = Vec::new();
        for bad in [
            "GATE_SDK_NATIVE_BIN[] = a",
            "GATE_SDK_NATIVE_BIN[k] = a",
            "GATE_SDK_NATIVE_BIN",
            "GATE_SDK_NATIVE_BIN = a\tb",
            "GATE_SDK_NATIVE_BIN = a\n# c\nGATE_SDK_NATIVE_BIN = b",
        ] {
            std::fs::write(&tracked, format!("# lead\n{}\n", bad)).expect("write");
            got.push((bad.to_string(), bin_from(None, &dir, None).expect_err(bad)));
        }
        let absent = d.join("absent.knobs").display().to_string();
        let missing = bin_from(None, &dir, Some(&absent)).expect_err("a named file that is absent");
        let _ = std::fs::remove_dir_all(&d);
        let at = format!("{}/gate-sdk-config.knobs:", dir);
        for (bad, e) in &got {
            let lno = if bad.contains('#') { 4 } else { 2 };
            assert!(e.starts_with(&format!("{}{}: ", at, lno)), "{:?} gave {}", bad, e);
        }
        assert!(missing.contains(&absent) && missing.contains("GATE_SDK_KNOB_FILE"), "{}", missing);
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
