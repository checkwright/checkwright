// spec: evidence-kit/SPEC.md §Layout and configuration — the
// EVIDENCE_KIT_PARSER_installer_smoke adapter: one scenario per arm, the roster a driver's
// top-level headers or, given the log alone, the log's own `smoke-roster:` head
// spec: gate-sdk/SPEC.md §The non-gate arm — an empty roster of the *happens to read nothing*
// kind; the driver is this product's file, so it is an operand rather than a crate literal
pub const KNOBS: &[&str] = &[];
pub const USAGE: &str = "usage: --emit parse-smoke-log [<driver.sh>] <log>";

// spec: evidence-kit/SPEC.md §Layout and configuration — the roster line's lead, the one spelling
// a compiled driver prints and the log-only form reads
pub const ROSTER_LINE: &str = "smoke-roster: ";

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

// spec: evidence-kit/SPEC.md §Layout and configuration — the log-only form: the roster is the
// log's `smoke-roster:` lines, in order, the last naming the marker; every one precedes the first
// header, and fewer than two is the driver form's own exit 2
fn roster_from_log(log_text: &str) -> Result<(Vec<String>, String), String> {
    let mut names: Vec<String> = Vec::new();
    let mut last = 0;
    for (i, line) in log_text.lines().enumerate() {
        if let Some(name) = line.strip_prefix(ROSTER_LINE) {
            names.push(name.to_string());
            last = i;
        }
    }
    if names.len() < 2 {
        return Err(format!(
            "fewer than two '{}' lines in the log ({}) — the log-only form reads its arm roster \
             from the log's head, so it cannot separate an arm from the completion marker or judge \
             this run",
            ROSTER_LINE.trim_end(),
            names.len()
        ));
    }
    let marker = names.pop().expect("the length was just checked");
    let head: String = log_text.lines().take(last).map(|l| format!("{}\n", l)).collect();
    if let Some(early) = reached(&head, &names, &marker).0.first() {
        return Err(format!(
            "the header '{}' precedes the log's last '{}' line — the roster must be declared \
             before the first arm runs",
            early,
            ROSTER_LINE.trim_end()
        ));
    }
    if head.lines().any(|l| l.starts_with(marker.as_str())) {
        return Err(format!(
            "the completion marker '{}' precedes the log's last '{}' line",
            marker,
            ROSTER_LINE.trim_end()
        ));
    }
    Ok((names, marker))
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
    let (driver, log) = match args {
        [l] if !l.is_empty() => (None, l.as_str()),
        [d, l] if !d.is_empty() && !l.is_empty() => (Some(d.as_str()), l.as_str()),
        _ => {
            return Err(format!(
                "needs a log, after the driver where the roster is a driver's headers — the \
                 driver is the consumer's own file, so the arm holds no default for it\n{}",
                USAGE
            ))
        }
    };
    if !std::path::Path::new(log).is_file() {
        return Err(format!("log not found: {}", log));
    }
    let log_text = super::read_text(log)?;
    let (arms, marker) = match driver {
        None => roster_from_log(&log_text)?,
        Some(d) if !std::path::Path::new(d).is_file() => {
            return Err(format!(
                "smoke driver not found: {} — the arm roster is derived from it",
                d
            ))
        }
        Some(d) => roster(&super::read_text(d)?)?,
    };
    let (seen, clean) = reached(&log_text, &arms, &marker);
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

    // spec: evidence-kit/SPEC.md §Layout and configuration — the log-only form reads the same
    // scenarios off the log's own roster head as the driver form reads off the driver
    #[test]
    fn the_log_only_form_reads_the_roster_off_the_log_head() {
        let head = "smoke-roster: build\nsmoke-roster: pack\nsmoke-roster: profile invariant\n\
                    smoke-roster: SMOKE: clean\n";
        let aborted = format!("{}build (the host gate binary)\npack\nboom\n", head);
        let clean = format!("{}build (x)\npack\nprofile invariant\nSMOKE: clean (3)\n", head);
        let dir = std::env::temp_dir().join(format!("cw-parse-smoke-log-only-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let run = |name: &str, body: &str| {
            let p = dir.join(name);
            std::fs::write(&p, body).expect("write log");
            emit(&[p.display().to_string()])
        };
        let got = (run("aborted.log", &aborted), run("clean.log", &clean));
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(got.0.expect("the log-only form refused a roster-bearing log"), "build pass\npack fail\n");
        assert_eq!(
            got.1.expect("the log-only form refused a roster-bearing log"),
            "build pass\npack pass\nprofile-invariant pass\n"
        );
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — fewer than two roster lines, and a
    // header or the marker ahead of the last roster line, are each exit 2
    #[test]
    fn a_log_roster_too_short_or_declared_late_fails_closed() {
        assert!(roster_from_log("build\npack\n").is_err(), "a roster-less log was accepted");
        assert!(roster_from_log("smoke-roster: only\nonly\n").is_err(), "one roster line was accepted");
        assert!(
            roster_from_log("smoke-roster: build\nbuild (x)\nsmoke-roster: DONE\n").is_err(),
            "a header ahead of the last roster line was accepted"
        );
        assert!(
            roster_from_log("smoke-roster: build\nDONE early\nsmoke-roster: DONE\n").is_err(),
            "the marker ahead of the last roster line was accepted"
        );
        let (arms, marker) = roster_from_log("note\nsmoke-roster: a\nsmoke-roster: b c\nsmoke-roster: DONE\na\n")
            .expect("a well-formed head was refused");
        assert_eq!((arms, marker.as_str()), (vec!["a".to_string(), "b c".to_string()], "DONE"));
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — the driver is an operand with no
    // crate default, so a missing or unresolvable one is exit 2 naming it
    #[test]
    fn a_missing_operand_or_an_unresolvable_one_fails_closed() {
        assert!(emit(&[]).is_err(), "no operand at all was accepted");
        assert!(
            emit(&["/nonexistent/only-one.log".to_string()]).is_err(),
            "a lone operand naming no log was accepted"
        );
        assert!(
            emit(&["a".to_string(), "b".to_string(), "c".to_string()]).is_err(),
            "a third operand was accepted"
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
