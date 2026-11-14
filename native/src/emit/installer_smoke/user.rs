// spec: installer/SPEC.md §The consumer smoke — the user-install arm: `init --user` from outside any
// repository under scratch per-user roots, the printed entry fired as the harness would fire it, a
// bare `update --user` leaving the data dir byte-identical, and `uninstall --user` keeping the state dir
use super::consumer::{failed, find_line, mkdir, out};
use super::{fail, in_consumer_fed, say, text, Outcome, Run, Step};
use crate::installer::files_under;
use crate::{programs, walk};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const MEMBER: &str = "subagent-stop-liveness";
const QUIET: &str = r#"{"session_id":"smoke-user","hook_event_name":"SubagentStop"}"#;
const RUNNING: &str = r#"{"session_id":"smoke-user","hook_event_name":"SubagentStop","background_tasks":[{"id":"t1","type":"shell","status":"running"}]}"#;

// spec: installer/SPEC.md §The user install — the roots the arm points the install at, and the data
// and state dirs the base-directory rule derives under them on this host
fn roots(base: &str) -> (Vec<(String, String)>, PathBuf, PathBuf) {
    let at = |rel: &str| format!("{}/{}", base, rel);
    if cfg!(windows) {
        let data = Path::new(&at("local")).join("checkwright");
        let state = data.join("state");
        (vec![("LOCALAPPDATA".to_string(), at("local"))], data, state)
    } else {
        (
            vec![("XDG_DATA_HOME".to_string(), at("data")), ("XDG_STATE_HOME".to_string(), at("state"))],
            Path::new(&at("data")).join("checkwright"),
            Path::new(&at("state")).join("checkwright"),
        )
    }
}

// spec: installer/SPEC.md §The user install — the data dir's bytes less the state dir, which sits
// under it on Windows and holds what the arm writes
fn snapshot(data: &Path, state: &Path) -> Result<BTreeMap<String, Vec<u8>>, Outcome> {
    if !data.is_dir() {
        return Ok(BTreeMap::new());
    }
    let state = state.to_string_lossy();
    let mut out = BTreeMap::new();
    for rel in files_under(data).map_err(|e| fail(format!("user-install arm: cannot walk {}: {}", data.display(), e)))? {
        if walk::at_or_under(&state, &data.join(&rel).to_string_lossy()) {
            continue;
        }
        let bytes = std::fs::read(data.join(&rel)).map_err(|e| fail(format!("user-install arm: cannot read {}: {}", rel, e)))?;
        out.insert(rel, bytes);
    }
    Ok(out)
}

// spec: installer/SPEC.md §The user install — the entry's command and args, read off the `settings:`
// block's JSON line as a harness reads the merged file
fn entry(said: &str) -> Result<(String, Vec<String>), Outcome> {
    let line = said
        .split('\n')
        .map(|l| l.trim_end_matches('\r'))
        .skip_while(|l| *l != "settings:")
        .map(str::trim)
        .find(|l| l.starts_with('{'))
        .ok_or_else(|| fail("user-install arm: init --user printed no settings block carrying a JSON entry"))?;
    let doc: serde_json::Value =
        serde_json::from_str(line).map_err(|e| fail(format!("user-install arm: the printed entry is not JSON ({}): {}", e, line)))?;
    let hook = &doc[crate::hook::binding::EVENT_CHILD_TURN_END][0]["hooks"][0];
    let command = hook["command"].as_str().unwrap_or_default().to_string();
    let args: Vec<String> = hook["args"].as_array().into_iter().flatten().filter_map(|v| v.as_str().map(String::from)).collect();
    if command.is_empty() || args.first().map(String::as_str) != Some("--hook") {
        return Err(fail(format!("user-install arm: the printed entry names no command running --hook: {}", line)));
    }
    Ok((command, args))
}

pub(super) fn user_install(state: &mut Run) -> Step {
    let base = format!("{}/user-install", state.scratch);
    let free = format!("{}/free", base);
    mkdir(&free)?;
    let (mut set, data, user_state) = roots(&base);
    set.push(("GIT_CEILING_DIRECTORIES".to_string(), state.scratch.clone()));

    let m = state.verb_with(&free, &["init", "--user", MEMBER], &set)?;
    let said = out(&m);
    let installed = format!("INIT: {} installed for this user at ", MEMBER);
    if !m.succeeded() || find_line(&said, |l| l.starts_with(&installed)).is_none() {
        return Err(failed(&m, format!("user-install arm: init --user {} did not report the install", MEMBER)));
    }
    let (command, args) = entry(&said)?;
    if !walk::under(&data.to_string_lossy(), &command) {
        return Err(fail(format!("user-install arm: the entry's command is not the binary under {}: {}", data.display(), command)));
    }
    say(&format!("install: init --user {} from outside any repository, its entry naming the binary under the data dir", MEMBER));

    let program = programs::CHECKWRIGHT_GATES.at(command.clone());
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let quiet = in_consumer_fed(&program, &argv, &set, &free, QUIET.as_bytes())?;
    let quiet_out = text(quiet.stdout().unwrap_or_default());
    if quiet.code() != Some(0) || !quiet_out.trim().is_empty() {
        return Err(fail(format!(
            "user-install arm: the entry over a quiet turn end exited {} or spoke: {}",
            quiet.reported_code(),
            quiet_out
        )));
    }
    let running = in_consumer_fed(&program, &argv, &set, &free, RUNNING.as_bytes())?;
    let log = user_state.join(format!("{}.log", MEMBER));
    let logged = std::fs::read_to_string(&log).unwrap_or_default();
    if running.code() != Some(2) || !logged.contains("decision=refuse") {
        return Err(fail(format!(
            "user-install arm: the entry over a running shell task exited {}, or {} logged no refusal",
            running.reported_code(),
            log.display()
        )));
    }
    say("fire: the entry allows a quiet turn end silently and refuses one over a running shell task, logging under the state dir");

    let before = snapshot(&data, &user_state)?;
    let m = state.verb_with(&free, &["update", "--user"], &set)?;
    if !m.succeeded() || snapshot(&data, &user_state)? != before {
        return Err(failed(&m, "user-install arm: a bare update --user failed or changed the data dir"));
    }
    say("re-run: update --user exit 0, the data dir byte-identical");

    let m = state.verb_with(&free, &["uninstall", "--user"], &set)?;
    let said = out(&m);
    let reminded = find_line(&said, |l| l.starts_with("settings: remove the ")).is_some();
    if !m.succeeded() || !snapshot(&data, &user_state)?.is_empty() || !reminded {
        return Err(failed(&m, "user-install arm: uninstall --user failed, left a file in the data dir, or printed no settings reminder"));
    }
    if !log.is_file() {
        return Err(failed(&m, "user-install arm: uninstall --user removed the state dir's log"));
    }
    if files_under(Path::new(&free)).map(|f| !f.is_empty()).unwrap_or(true) {
        return Err(fail(format!("user-install arm: a verb or the hook wrote into the working directory {}", free)));
    }
    say("reversal: uninstall --user exit 0, the data dir emptied, the state dir's log kept, the entry's removal recalled");
    Ok(())
}
