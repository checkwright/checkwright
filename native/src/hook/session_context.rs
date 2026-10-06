// spec: context-kit/SPEC.md §The session-context hook — the session-start member: it assembles the
// session brief on stdout, every step degrading silently, and never fails a session.
use crate::hook::{self, Payload};
use crate::{proc, programs, walk};
use serde_json::Value;
use std::path::Path;

const NAME: &str = "session-context";

pub const KNOBS: &[&str] = &[
    "CONTEXT_KIT_STATE_FILE",
    "CONTEXT_KIT_DRIFT_REPORT",
    "CONTEXT_KIT_STAGE_RULES",
    "CONTEXT_KIT_SESSION_ROLE_FILE",
    "CONTEXT_KIT_ENV_PROFILE_FILE",
    "CONTEXT_KIT_BRIEF_COMPONENT_MARK",
    "CONTEXT_KIT_BRIEF_NUDGES",
    "CONTEXT_KIT_BRIEF_FOOTER_FILE",
    "CONTEXT_KIT_BRIEF_COMMANDS",
    "GATE_SDK_TMP_DIR",
    "GATE_SDK_NATIVE_BIN",
];

const RULE_OPEN: &str = "── Session context (context-kit session-context hook) ──────────────────";
const RULE_CLOSE: &str = "────────────────────────────────────────────────────────────────────────";

// spec: context-kit/SPEC.md §The session-context hook — the scratch sweep's age horizon, one day
const STALE_SECS: u64 = 1440 * 60;

struct Knobs {
    state_file: String,
    drift_arm: String,
    stage_rules: String,
    role_file: String,
    env_profile: String,
    mark: String,
    nudges: Vec<(String, String)>,
    footer_file: String,
    commands: Vec<String>,
    tmp_dir: String,
    door: String,
}

fn knobs() -> Result<Knobs, String> {
    Ok(Knobs {
        state_file: walk::knob_scalar("CONTEXT_KIT_STATE_FILE")?,
        drift_arm: walk::knob_scalar("CONTEXT_KIT_DRIFT_REPORT")?,
        stage_rules: walk::knob_scalar("CONTEXT_KIT_STAGE_RULES")?,
        role_file: walk::knob_scalar("CONTEXT_KIT_SESSION_ROLE_FILE")?,
        env_profile: walk::knob_scalar("CONTEXT_KIT_ENV_PROFILE_FILE")?,
        mark: walk::knob_scalar("CONTEXT_KIT_BRIEF_COMPONENT_MARK")?,
        nudges: walk::knob_map("CONTEXT_KIT_BRIEF_NUDGES")?,
        footer_file: walk::knob_scalar("CONTEXT_KIT_BRIEF_FOOTER_FILE")?,
        commands: walk::knob_array("CONTEXT_KIT_BRIEF_COMMANDS")?,
        tmp_dir: walk::knob_scalar("GATE_SDK_TMP_DIR")?,
        door: crate::gates::door_spelling(&walk::knob_scalar("GATE_SDK_NATIVE_BIN")?),
    })
}

pub fn run(payload: &Payload) -> i32 {
    // spec: context-kit/SPEC.md §The session-context hook — a knob that cannot resolve is the one
    // path that prints no brief: the member declines before its first line
    let k = match knobs() {
        Ok(k) => k,
        Err(e) => return hook::decline(NAME, &e, payload.value()),
    };
    print!("{}", brief(&k, payload.value()));
    0
}

fn brief(k: &Knobs, payload: Option<&Value>) -> String {
    let mut out = format!("{}\n\n", RULE_OPEN);
    let stage = std::fs::read(&k.state_file)
        .map(|b| crate::stages::current_stage(&String::from_utf8_lossy(&b)))
        .unwrap_or_default();
    let lead = is_lead(&k.role_file, payload);

    let mut index = vec!["--emit", "queue-index"];
    if stage != "close" && stage != "scope" {
        index.push("--collapse-deferred");
    }
    match own_arm(&index) {
        Some((0, text)) => out.push_str(&text),
        _ => out.push_str("(queue-index unavailable)\n"),
    }
    out.push('\n');

    out.push_str(&dirty_surface(&k.mark));

    if !k.drift_arm.is_empty() {
        if let Some((_, text)) = own_arm(&["--emit", &k.drift_arm, "--trend"]) {
            let line = text.trim_end_matches('\n');
            if !line.is_empty() {
                out.push_str(&format!("{}  (full: {} --emit {})\n\n", line, k.door, k.drift_arm));
            }
        }
    }

    // spec: context-kit/SPEC.md §The session-context hook — a brief command's line is the point,
    // so it prints whatever status the command exits
    for cmd in &k.commands {
        let words: Vec<&str> = cmd.split_ascii_whitespace().collect();
        if let Some((_, text)) = configured(&words, "CONTEXT_KIT_BRIEF_COMMANDS") {
            if !text.trim().is_empty() {
                out.push_str(&with_newline(&text));
                out.push('\n');
            }
        }
    }

    if !lead {
        if let Some((_, file)) = k.nudges.iter().find(|(s, _)| *s == stage) {
            if let Ok(text) = std::fs::read_to_string(file) {
                out.push_str(&with_newline(&text));
                out.push('\n');
            }
        }
    }

    out.push_str(&memory_backstop());
    out.push_str(&sweep(&k.tmp_dir));
    out.push_str(&footer(k));

    if !lead && !stage.is_empty() && !k.stage_rules.is_empty() {
        let mut words: Vec<&str> = k.stage_rules.split_ascii_whitespace().collect();
        words.push(&stage);
        match configured(&words, "CONTEXT_KIT_STAGE_RULES") {
            Some((0, block)) => {
                let block = block.trim_end_matches('\n');
                if !block.is_empty() {
                    out.push_str(&format!(
                        "\nCraft rules for the {} stage — follow the doctrine link before the matching action:\n{}\n",
                        stage, block
                    ));
                }
            }
            failed => out.push_str(&format!(
                "\n⚠ craft rules unavailable: the CONTEXT_KIT_STAGE_RULES command exited {} — fix the knob (context-kit/SPEC.md §Layout and configuration).\n",
                failed.map_or(NOT_STARTED, |(rc, _)| rc)
            )),
        }
    }

    if Path::new(&k.env_profile).is_file() {
        let _ = own_arm(&["--emit", "env-probe"]);
        out.push_str(&format!(
            "\nLocal env profile ({}) — adapt commands to this box:\n",
            k.env_profile
        ));
        if let Ok(b) = std::fs::read(&k.env_profile) {
            out.push_str(&String::from_utf8_lossy(&b));
        }
    }

    if let Some((0, text)) = own_arm(&["--emit", "update-notice"]) {
        out.push_str(&text);
    }
    out.push_str(RULE_CLOSE);
    out.push('\n');
    out
}

// spec: context-kit/SPEC.md §The session-context hook — the status a shell reports for a command it
// could not start, which step 8's warning names
const NOT_STARTED: i32 = 127;

fn with_newline(text: &str) -> String {
    if text.ends_with('\n') {
        text.to_string()
    } else {
        format!("{}\n", text)
    }
}

// spec: gate-sdk/SPEC.md §run-gates — a sibling arm is this binary re-exec'd as a child, never an
// in-process dispatch: the child resolves its own knobs and its fault stays its own
fn own_arm(args: &[&str]) -> Option<(i32, String)> {
    let exe = std::env::current_exe().ok()?;
    let done = proc::run(&programs::CHECKWRIGHT_GATES.at(exe.display().to_string()), args).ok()?;
    Some((done.reported_code(), String::from_utf8_lossy(done.streams().0).into_owned()))
}

// spec: context-kit/SPEC.md §The session-context hook — a configured command is split on blanks and
// spawned with no shell: a first word beginning `--` names an arm of this binary, any other a program
fn configured(words: &[&str], knob: &'static str) -> Option<(i32, String)> {
    let (head, rest) = words.split_first()?;
    if head.starts_with("--") {
        return own_arm(words);
    }
    let done = proc::run(&programs::Program::consumer(knob, *head), rest).ok()?;
    Some((done.reported_code(), String::from_utf8_lossy(done.streams().0).into_owned()))
}

// spec: context-kit/SPEC.md §The session-context hook — lead iff the marker's id is the 8-char
// prefix of this fire's payload session id; an absent or unparseable payload is an absent signal
fn is_lead(role_file: &str, payload: Option<&Value>) -> bool {
    let Ok(text) = std::fs::read_to_string(role_file) else { return false };
    let mut fields = text.lines().next().unwrap_or("").split_whitespace();
    let (role, sid) = (fields.next().unwrap_or(""), fields.next().unwrap_or(""));
    let own: String = hook::field(payload, &["session_id"]).chars().take(8).collect();
    role == "lead" && !sid.is_empty() && own == sid
}

// spec: context-kit/SPEC.md §The session-context hook — step 2: a component is a top-level
// directory holding the configured mark, and an empty mark turns the step off
fn changed_components(porcelain: &str, mark: &str, holds_mark: &dyn Fn(&str) -> bool) -> Vec<String> {
    if mark.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<String> = porcelain
        .lines()
        .filter_map(|l| l.rsplit(' ').next())
        .filter_map(|p| p.split_once('/').map(|(top, _)| top.to_string()))
        .filter(|top| holds_mark(top))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn dirty_surface(mark: &str) -> String {
    let Ok(done) = proc::run(&programs::GIT, &["status", "--porcelain"]) else { return String::new() };
    let Some(status) = done.stdout() else { return String::new() };
    let changed = changed_components(&String::from_utf8_lossy(status), mark, &|top| {
        Path::new(top).join(mark).is_dir()
    });
    if changed.is_empty() {
        return String::new();
    }
    let mut out = format!(
        "Uncommitted changes touch: {}\nPublic API surface of those components (pub-index — read the file for bodies):\n\n",
        changed.join(" ")
    );
    for c in &changed {
        if let Some((0, text)) = own_arm(&["--emit", "pub-index", &format!("{}/{}/", c, mark)]) {
            out.push_str(&text);
        }
    }
    out.push('\n');
    out
}

fn holds_content(dir: &Path) -> bool {
    let Ok(entries) = walk::list_dir(dir) else { return false };
    entries.iter().any(|(name, is_dir)| {
        if *is_dir {
            holds_content(&dir.join(name))
        } else {
            name != ".gitkeep"
        }
    })
}

fn memory_backstop() -> String {
    let Some((0, dirs)) = own_arm(&["--emit", "memory-dirs"]) else { return String::new() };
    match dirs.lines().find(|d| Path::new(d).is_dir() && holds_content(Path::new(d))) {
        Some(d) => format!(
            "⚠ harness memory dir holds content ({}) — durable facts belong in a tracked surface (context-kit/SPEC.md §The memory-off doctrine), not per-session memory.\n\n",
            d
        ),
        None => String::new(),
    }
}

fn stale(path: &Path, now: std::time::SystemTime) -> bool {
    path.symlink_metadata()
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| now.duration_since(t).ok())
        .is_some_and(|age| age.as_secs() > STALE_SECS)
}

// spec: context-kit/SPEC.md §The session-context hook — step 6, depth-first so a stale directory
// goes once it is empty; the count is what was removed
fn sweep_dir(dir: &Path, now: std::time::SystemTime) -> usize {
    let Ok(entries) = walk::list_dir(dir) else { return 0 };
    let mut swept = 0;
    for (name, is_dir) in entries {
        if name == ".gitkeep" || name == ".gitignore" {
            continue;
        }
        let path = dir.join(&name);
        let old = stale(&path, now);
        if is_dir {
            swept += sweep_dir(&path, now);
            if old && std::fs::remove_dir(&path).is_ok() {
                swept += 1;
            }
        } else if old && std::fs::remove_file(&path).is_ok() {
            swept += 1;
        }
    }
    swept
}

fn sweep(tmp_dir: &str) -> String {
    let dir = Path::new(tmp_dir);
    if !dir.is_dir() {
        return String::new();
    }
    match sweep_dir(dir, std::time::SystemTime::now()) {
        0 => String::new(),
        n => format!("Tidied {} stale scratch path(s) from {}/.\n\n", n, tmp_dir),
    }
}

// spec: context-kit/SPEC.md §The session-context hook — step 7: a configured footer file prints
// verbatim, else the built-in one, its public-surface line spelled with the component mark
fn footer(k: &Knobs) -> String {
    if !k.footer_file.is_empty() {
        return std::fs::read_to_string(&k.footer_file)
            .map(|t| with_newline(&t))
            .unwrap_or_default();
    }
    let mut out = String::from(
        "Before opening source for a task, run the matching surface index first\n(index, then read the one you need):\n",
    );
    if !k.mark.is_empty() {
        out.push_str(&format!(
            "  • {} --emit pub-index <component>/{}/    — public API surface (ships rust, ts)\n",
            k.door, k.mark
        ));
    }
    out.push_str(&format!(
        "  • {door} --emit md-index <file.md>            — large markdown / SPEC outline\n  • {door} --emit md-section <file.md> \"<head>\" — extract one section by heading\n",
        door = k.door
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §The session-context hook — a changed path names its top-level
    // directory, which is a component only where it holds the mark; an empty mark is no step
    #[test]
    fn a_component_is_a_changed_top_level_directory_holding_the_mark() {
        let porcelain = " M alpha/src/lib.rs\n?? beta/notes.md\n M README.md\nR  old -> alpha/src/new.rs\n";
        let holds = |top: &str| top == "alpha";
        assert_eq!(changed_components(porcelain, "src", &holds), vec!["alpha"]);
        assert!(changed_components(porcelain, "", &holds).is_empty());
        assert!(changed_components(" M README.md\n", "src", &holds).is_empty());
    }

    // spec: context-kit/SPEC.md §The session-context hook — the marker's id against the payload's
    // own, eight characters of it; no payload, another id and another role are each no signal
    #[test]
    fn the_lead_signal_is_the_markers_id_matching_the_payloads() {
        let dir = std::env::temp_dir().join(format!("cw-session-role-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let file = dir.join("session-role");
        std::fs::write(&file, "lead abcd1234\n").expect("marker");
        let f = file.display().to_string();
        let own: Value = serde_json::from_str(r#"{"session_id":"abcd1234-ffff"}"#).expect("json");
        let other: Value = serde_json::from_str(r#"{"session_id":"00001234-ffff"}"#).expect("json");
        assert!(is_lead(&f, Some(&own)));
        assert!(!is_lead(&f, Some(&other)));
        assert!(!is_lead(&f, None));
        std::fs::write(&file, "stage abcd1234\n").expect("marker");
        assert!(!is_lead(&f, Some(&own)));
        assert!(!is_lead(&dir.join("absent").display().to_string(), Some(&own)));
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: context-kit/SPEC.md §The session-context hook — the sweep spares the two keep files and
    // anything younger than the horizon, and takes a stale directory once it is empty
    #[test]
    fn the_sweep_takes_only_stale_paths_and_spares_the_keep_files() {
        let dir = std::env::temp_dir().join(format!("cw-session-sweep-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("old")).expect("scratch dir");
        for f in ["fresh.txt", ".gitkeep", ".gitignore", "old/inner.txt"] {
            std::fs::write(dir.join(f), "x").expect("scratch file");
        }
        let now = std::time::SystemTime::now();
        assert_eq!(sweep_dir(&dir, now), 0);
        let later = now + std::time::Duration::from_secs(STALE_SECS + 60);
        assert_eq!(sweep_dir(&dir, later), 3);
        assert!(dir.join(".gitkeep").is_file() && dir.join(".gitignore").is_file());
        assert!(!dir.join("old").exists() && !dir.join("fresh.txt").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: context-kit/SPEC.md §The session-context hook — the built-in footer drops its
    // public-surface line with the mark, and a footer file replaces all of it
    #[test]
    fn the_footer_follows_the_mark_and_yields_to_a_file() {
        let k = |mark: &str, footer_file: &str| Knobs {
            state_file: String::new(),
            drift_arm: String::new(),
            stage_rules: String::new(),
            role_file: String::new(),
            env_profile: String::new(),
            mark: mark.to_string(),
            nudges: Vec::new(),
            footer_file: footer_file.to_string(),
            commands: Vec::new(),
            tmp_dir: String::new(),
            door: "./gates".to_string(),
        };
        let built_in = footer(&k("lib", ""));
        assert!(built_in.contains("./gates --emit pub-index <component>/lib/"));
        assert_eq!(built_in.lines().count(), 5);
        let no_mark = footer(&k("", ""));
        assert!(!no_mark.contains("pub-index") && no_mark.contains("md-section"));
        assert_eq!(footer(&k("src", "/no/such/footer/file")), "");
    }
}
