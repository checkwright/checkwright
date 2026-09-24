// spec: installer/SPEC.md §The hosted install pin — the two hosted install scripts carry one pin
// each, the pins agree (A), and the pin is the newest release (B)
use super::release_channel_parity::newest_tag;
use crate::fresh;
use std::path::Path;

const DEFAULT_INSTALL_SH: &str = "docs/install.sh";
const DEFAULT_INSTALL_PS1: &str = "docs/install.ps1";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-pin: {}", e);
            2
        }
    }
}

// spec: installer/SPEC.md §The hosted install pin — `<major>.<minor>.<patch>`, each a run of ASCII
// digits, with no prefix and no suffix
fn is_triple(v: &str) -> bool {
    let fields: Vec<&str> = v.split('.').collect();
    fields.len() == 3
        && fields
            .iter()
            .all(|f| !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit()))
}

// spec: installer/SPEC.md §The hosted install pin — a pin line is any line whose trimmed form opens
// on the script's pin variable and an `=`; exactly one such line is admissible, and it must be the
// single-quoted triple
fn pin_of(path: &str, text: &str, name: &str) -> Result<String, String> {
    let lines: Vec<(usize, &str)> = fresh::file_lines(text)
        .into_iter()
        .enumerate()
        .map(|(n, l)| (n + 1, l.trim()))
        .filter(|(_, l)| {
            l.strip_prefix(name)
                .is_some_and(|rest| rest.trim_start().starts_with('='))
        })
        .collect();
    let (lineno, line) = match lines.as_slice() {
        [one] => *one,
        [] => {
            return Err(format!(
                "{} carries no '{}' pin line — the hosted install's version cannot be established (installer/SPEC.md §The hosted install pin)",
                path, name
            ))
        }
        many => {
            let shown: Vec<String> = many.iter().map(|(n, l)| format!("{}:{}", n, l)).collect();
            return Err(format!(
                "{} carries {} '{}' pin lines; exactly one is admissible:\n{}",
                path,
                many.len(),
                name,
                shown.join("\n")
            ));
        }
    };
    let value = line[name.len()..]
        .trim_start()
        .strip_prefix('=')
        .map(str::trim)
        .and_then(|v| v.strip_prefix('\''))
        .and_then(|v| v.strip_suffix('\''))
        .filter(|v| is_triple(v))
        .ok_or_else(|| {
            format!(
                "{}:{}: the pin is not a single-quoted <major>.<minor>.<patch>: {}",
                path, lineno, line
            )
        })?;
    Ok(value.to_string())
}

fn rule(args: &[String]) -> Result<i32, String> {
    let install_sh = fresh::positional(args, 0, DEFAULT_INSTALL_SH);
    let install_ps1 = fresh::positional(args, 1, DEFAULT_INSTALL_PS1);
    let version_arg = args.get(2).map(String::as_str).unwrap_or("");

    for p in [install_sh, install_ps1] {
        if !Path::new(p).is_file() {
            return Err(format!("not found: {}", p));
        }
    }
    let sh_pin = pin_of(install_sh, &fresh::read_captured(install_sh)?, "pin")?;
    let ps_pin = pin_of(install_ps1, &fresh::read_captured(install_ps1)?, "$pin")?;

    let mut findings: Vec<String> = Vec::new();
    if sh_pin != ps_pin {
        findings.push(format!(
            "  invariant A: the twins disagree — {} pins {} and {} pins {}",
            install_sh, sh_pin, install_ps1, ps_pin
        ));
    }

    // spec: installer/SPEC.md §The hosted install pin — B is dormant, and says so, where no tag exists
    let version = if version_arg.is_empty() {
        newest_tag()
    } else {
        version_arg.to_string()
    };
    let b_state = if version.is_empty() {
        None
    } else {
        let bare = version.strip_prefix('v').unwrap_or(&version).to_string();
        if !is_triple(&bare) {
            return Err(format!(
                "the newest tag ('{}') does not parse as <major>.<minor>.<patch>, so the pin cannot be compared against it",
                version
            ));
        }
        for (path, pin) in [(install_sh, &sh_pin), (install_ps1, &ps_pin)] {
            if *pin != bare {
                findings.push(format!(
                    "  invariant B: {} pins {}, and the newest release is v{}",
                    path, pin, bare
                ));
            }
        }
        Some(bare)
    };

    if !findings.is_empty() {
        println!("check-install-pin: the hosted install scripts' pin disagrees with a surface it is held to (installer/SPEC.md §The hosted install pin):");
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: set both scripts' pin line to the newest release tag, in the commit after that tag — RELEASING.md step 4 moves it with the release declaration surface's drain.");
        return Ok(1);
    }
    match b_state {
        None => println!(
            "INSTALL-PIN: clean (both scripts pin {}; invariant B dormant — no tags, so there is no newest release to compare)",
            sh_pin
        ),
        Some(v) => println!(
            "INSTALL-PIN: clean (both scripts pin {}, the newest release v{})",
            sh_pin, v
        ),
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The hosted install pin — the pin grammar is the bare triple
    #[test]
    fn only_a_bare_triple_is_a_pin() {
        assert!(is_triple("0.25.0"));
        for bad in ["v0.25.0", "0.25", "0.25.0-rc1", "0.25.x", "", "0..1"] {
            assert!(!is_triple(bad), "{} should not be a triple", bad);
        }
    }

    // spec: installer/SPEC.md §The hosted install pin — one pin line per script, in its own
    // grammar, indented or not
    #[test]
    fn a_pin_line_is_read_in_each_scripts_grammar() {
        assert_eq!(pin_of("s", "f() {\n    pin='1.2.3'\n}\n", "pin"), Ok("1.2.3".to_string()));
        assert_eq!(pin_of("p", "    $pin = '1.2.3'\n", "$pin"), Ok("1.2.3".to_string()));
        assert_eq!(pin_of("s", "pin_dir=x\npin='1.2.3'\n", "pin"), Ok("1.2.3".to_string()));
    }

    // spec: installer/SPEC.md §The hosted install pin — the exit-2 set: a missing, duplicated or
    // malformed pin line
    #[test]
    fn a_missing_duplicated_or_malformed_pin_fails_closed() {
        assert!(pin_of("s", "echo hi\n", "pin").is_err());
        assert!(pin_of("s", "pin='1.2.3'\npin='1.2.4'\n", "pin").is_err());
        assert!(pin_of("s", "pin=\"1.2.3\"\n", "pin").is_err());
        assert!(pin_of("p", "$pin = 'v1.2.3'\n", "$pin").is_err());
    }
}
