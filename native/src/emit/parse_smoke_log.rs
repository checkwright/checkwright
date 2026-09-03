// spec: evidence-kit/SPEC.md §Layout and configuration — the
// EVIDENCE_KIT_PARSER_installer_smoke adapter: one scenario per arm of the driver named on argv,
// the roster derived from that driver's own top-level headers
// spec: gate-sdk/SPEC.md §The non-gate arm — an empty roster of the *happens to read nothing*
// kind; the driver is this product's file, so it is an operand rather than a crate literal
pub const KNOBS: &[&str] = &[];

// spec: evidence-kit/SPEC.md §Layout and configuration — a header is a top-level `printf` of a
// `\n`-terminated literal with no redirect, named up to its parenthetical; an empty literal and a
// bare format specifier name no stable scenario
fn header(line: &str) -> Option<String> {
    let body = line.strip_prefix("printf '")?;
    if line.contains('>') {
        return None;
    }
    let name = &body[..body.find("\\n")?];
    let name = match name.find(" (") {
        Some(i) => &name[..i],
        None => name,
    };
    if name.is_empty() || name.starts_with('%') {
        return None;
    }
    Some(name.to_string())
}

// spec: evidence-kit/SPEC.md §Layout and configuration — a driver's *last* top-level header is
// its completion marker and the ones before it are its arms, so fewer than two is exit 2
fn roster(driver_text: &str) -> Result<(Vec<String>, String), String> {
    let mut headers: Vec<String> = driver_text.lines().filter_map(header).collect();
    if headers.len() < 2 {
        return Err(format!(
            "fewer than two top-level headers derived from the driver ({}) — the parser cannot \
             separate an arm from the completion marker, so it cannot judge this run",
            headers.len()
        ));
    }
    let marker = headers.pop().expect("the length was just checked");
    Ok((headers, marker))
}

// spec: evidence-kit/SPEC.md §Layout and configuration — an arm's line in the log is its header
// name alone or followed by its parenthetical; the first reach wins
fn reached(log_text: &str, arms: &[String], marker: &str) -> (Vec<String>, bool) {
    let mut seen: Vec<String> = Vec::new();
    let mut clean = false;
    for line in log_text.lines() {
        if line.starts_with(marker) {
            clean = true;
        }
        for a in arms {
            if line != a && !line.starts_with(&format!("{} (", a)) {
                continue;
            }
            if seen.iter().any(|r| r == a) {
                continue;
            }
            seen.push(a.clone());
            break;
        }
    }
    (seen, clean)
}

// spec: evidence-kit/SPEC.md §Layout and configuration — the smoke aborts at its first failure,
// so every arm but the last one reached is proved by the arm that followed it; the last is proved
// by the run's own completion marker and is `fail` without it
pub fn emit(args: &[String]) -> Result<String, String> {
    let (driver, log) = match (args.first(), args.get(1)) {
        (Some(d), Some(l)) if !d.is_empty() && !l.is_empty() => (d.as_str(), l.as_str()),
        _ => {
            return Err(
                "usage: --emit parse-smoke-log <driver.sh> <log> — the driver is the consumer's \
                 own file, so the arm holds no default for it"
                    .to_string(),
            )
        }
    };
    if !std::path::Path::new(log).is_file() {
        return Err(format!("log not found: {}", log));
    }
    if !std::path::Path::new(driver).is_file() {
        return Err(format!(
            "smoke driver not found: {} — the arm roster is derived from it",
            driver
        ));
    }
    let (arms, marker) = roster(&super::read_text(driver)?)?;
    let (seen, clean) = reached(&super::read_text(log)?, &arms, &marker);
    let mut out = String::new();
    for (i, name) in seen.iter().enumerate() {
        let status = if i + 1 == seen.len() && !clean {
            "fail"
        } else {
            "pass"
        };
        out.push_str(&format!("{} {}\n", name.replace(' ', "-"), status));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DRIVER: &str = "#!/usr/bin/env bash\n\
        printf 'build (the host gate binary)\\n'\n\
        printf 'pack\\n'\n\
        printf '{\"name\":\"x\"}\\n' > \"$HOME/package.json\"\n\
        printf 'profile invariant\\n'\n\
        printf '%s\\n' \"$X\" > \"$F\"\n\
        printf '\\nAn adopter edited this line.\\n' >> \"$C\"\n\
        printf 'SMOKE: clean (%d profile(s))\\n' \"3\"\n";

    // spec: evidence-kit/SPEC.md §Layout and configuration — the ways a `printf` line is not a
    // header: a redirect, an empty literal, a bare format specifier, no `\n`, an indent
    #[test]
    fn a_redirect_an_empty_literal_and_a_bare_specifier_are_not_headers() {
        assert_eq!(header("printf 'pack\\n'").as_deref(), Some("pack"));
        assert_eq!(
            header("printf 'build (the host gate binary)\\n'").as_deref(),
            Some("build")
        );
        assert_eq!(header("printf 'x\\n' > \"$F\""), None);
        assert_eq!(header("printf '%s\\n' \"$X\""), None);
        assert_eq!(header("printf '\\nfoo\\n' >> \"$C\""), None);
        assert_eq!(header("    printf 'indented\\n'"), None);
        assert_eq!(header("echo 'pack'"), None);
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — the marker is the driver's last
    // top-level header, so the roster is every header before it
    #[test]
    fn the_last_top_level_header_is_the_marker_and_the_rest_are_arms() {
        let (arms, marker) = roster(DRIVER).expect("the roster derivation refused a live driver");
        assert_eq!(arms, vec!["build", "pack", "profile invariant"]);
        assert_eq!(marker, "SMOKE: clean");
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — fewer than two headers cannot yield
    // an arm and a marker: the zero-header refusal reached one case earlier
    #[test]
    fn fewer_than_two_headers_fails_closed() {
        assert!(roster("echo hi\n").is_err(), "a header-less driver was accepted");
        assert!(
            roster("printf 'only\\n'\n").is_err(),
            "a driver with one header yielded an arm and a marker from the same line"
        );
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — the fail-fast attribution, and an
    // arm the run never reached emitted as nothing at all
    #[test]
    fn the_last_arm_reached_carries_the_runs_own_verdict() {
        let driver = DRIVER.to_string();
        let dir = std::env::temp_dir().join("cw-parse-smoke-log");
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let dpath = dir.join("run-smoke.sh");
        std::fs::write(&dpath, &driver).expect("write driver");

        let aborted = dir.join("aborted.log");
        std::fs::write(&aborted, "build (the host gate binary)\npack\nboom\n").expect("write log");
        let args = vec![dpath.display().to_string(), aborted.display().to_string()];
        assert_eq!(emit(&args).expect("the arm refused a live log"), "build pass\npack fail\n");

        let clean = dir.join("clean.log");
        std::fs::write(
            &clean,
            "build (the host gate binary)\npack\nprofile invariant\nSMOKE: clean (3 profile(s))\n",
        )
        .expect("write log");
        let args = vec![dpath.display().to_string(), clean.display().to_string()];
        assert_eq!(
            emit(&args).expect("the arm refused a live log"),
            "build pass\npack pass\nprofile-invariant pass\n"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — the driver is an operand with no
    // crate default, so a missing or unresolvable one is exit 2 naming it
    #[test]
    fn a_missing_operand_or_an_unresolvable_one_fails_closed() {
        assert!(emit(&[]).is_err(), "no operand at all was accepted");
        assert!(
            emit(&["only-one.sh".to_string()]).is_err(),
            "a lone operand was accepted where the arm takes a driver and a log"
        );
        assert!(
            emit(&[
                "/nonexistent/run-smoke.sh".to_string(),
                "/nonexistent/run.log".to_string()
            ])
            .is_err(),
            "unresolvable operands were accepted"
        );
    }
}
