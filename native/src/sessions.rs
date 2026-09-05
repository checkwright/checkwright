// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the session derivation's one in-crate holder.
// That section keeps ownership of the *contract*; this module is where the contract's single
// implementation lives, promoted out of `emit/session_id.rs` rather than written a second time.
// spec: drift-kit/SPEC.md §The overhead meter — the 2026-09-05 ruling that both drift meters adopt
// one derivation is satisfied by sharing this module: a copy beside it would be the divergence the
// ruling refused, relocated from two shell scripts into two Rust modules.
// spec: gate-sdk/SPEC.md §lib/gate.sh — nothing here reads the environment. Every input arrives on
// `Inputs`, so each *arm* resolves its own kit's knobs and hands the answer in, and this module
// declares no knob roster of its own.
use std::time::SystemTime;

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — every input the derivation order reads, taken
// as a value so the rule is exercised without writing the process environment. The sessions dir is
// a **field** rather than a knob name, which is the seam a second kit reads it through.
pub struct Inputs {
    pub session_id: String,
    pub harness_id: String,
    pub child: String,
    pub sessions_dir: String,
    pub config_home: String,
    pub home: String,
    pub here: String,
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the shared normalization: strip a leading
// `agent-` token if present, then take the first 8 characters.
pub fn normalize(id: &str) -> String {
    id.strip_prefix("agent-")
        .unwrap_or(id)
        .chars()
        .take(8)
        .collect()
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the composition `normalize(basename(path))`,
// held here rather than spelled at each of its readers, on gate-sdk/SPEC.md §lib/gate.sh's *exactly
// one place a value is computed*. A bare id passes through it unchanged.
pub fn key(path: &str) -> String {
    let base = path.rsplit('/').next().unwrap_or(path);
    normalize(base.strip_suffix(".jsonl").unwrap_or(base))
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the cwd slug: every non-alphanumeric character
// mapped to `-`, which is `sed 's/[^a-zA-Z0-9]/-/g'` over the same string.
fn slug(path: &str) -> String {
    path.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — source 3's sessions dir: the override, else
// `<config-home>/projects/<cwd-slug>` with `$CLAUDE_CONFIG_DIR` or `~/.claude` as the home. The
// override arrives as a field, so each kit resolves its own knob and hands the answer in.
pub fn sessions_dir(i: &Inputs) -> String {
    if !i.sessions_dir.is_empty() {
        return i.sessions_dir.clone();
    }
    let home = if i.config_home.is_empty() {
        format!("{}/.claude", i.home)
    } else {
        i.config_home.clone()
    };
    format!("{}/projects/{}", home, slug(&i.here))
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the two-tier candidate layout, in one place, so
// a change to it moves one glob rather than one per caller: the flat tier and the nested subagent
// tier, which `resolve`'s widened branch walks.
fn candidate_globs(dir: &str) -> [String; 2] {
    [
        format!("{}/*.jsonl", dir),
        format!("{}/*/subagents/*.jsonl", dir),
    ]
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — advance the candidate across one (possibly
// empty) glob, keeping bash's own `-e` skip and its `-nt` replacement on a *strictly* newer
// mtime, so a tie leaves the earlier glob-sorted candidate standing.
fn pick(newest: &mut Option<(String, SystemTime)>, pattern: &str) {
    for f in crate::walk::glob_entries(pattern) {
        let Ok(meta) = std::fs::metadata(&f) else {
            continue;
        };
        let when = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let replace = match newest {
            Some((_, best)) => when > *best,
            None => true,
        };
        if replace {
            *newest = Some((f, when));
        }
    }
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the derivation order and its exit-2 refusals,
// returning the *winning path* rather than the normalized key, so a caller needing the transcript
// does not re-glob for the id it was handed. `key` is the identity on the two early returns.
// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the child-narrowed scan verifies the flag
// rather than trusting it: an empty narrowed scan with a top-level transcript for the env uuid
// marks the flag spurious and falls back to source 2.
pub fn resolve(i: &Inputs) -> Result<String, String> {
    if !i.session_id.is_empty() {
        return Ok(i.session_id.clone());
    }
    if i.child.is_empty() && !i.harness_id.is_empty() {
        return Ok(i.harness_id.clone());
    }
    let dir = sessions_dir(i);
    if !std::path::Path::new(&dir).is_dir() {
        return Err(format!(
            "sessions dir not found: {}\n  help: set LIFECYCLE_KIT_SESSIONS_DIR to the agent \
             transcript directory for this tree.",
            dir
        ));
    }
    let narrowed = !i.child.is_empty() && !i.harness_id.is_empty();
    let mut newest: Option<(String, SystemTime)> = None;
    if narrowed {
        pick(
            &mut newest,
            &format!("{}/{}/subagents/*.jsonl", dir, i.harness_id),
        );
    } else {
        for g in candidate_globs(&dir) {
            pick(&mut newest, &g);
        }
    }
    let Some((path, _)) = newest else {
        if narrowed {
            let top = format!("{}/{}.jsonl", dir, i.harness_id);
            if std::path::Path::new(&top).exists() {
                return Ok(top);
            }
            return Err(format!(
                "no subagent transcript under {}/{}/subagents and no top-level {}\n  help: \
                 confirm this is the right sessions dir (LIFECYCLE_KIT_SESSIONS_DIR).",
                dir, i.harness_id, top
            ));
        }
        return Err(format!(
            "no transcript (*.jsonl) under {}\n  help: confirm this is the right sessions dir \
             (LIFECYCLE_KIT_SESSIONS_DIR).",
            dir
        ));
    };
    Ok(path)
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the inverse lookup: which transcript does
// this `session8` name. It walks the same `candidate_globs` `resolve` does — never its own copy of
// the two patterns — normalizes each candidate's basename with `key`, and keeps the newest match.
// spec: drift-kit/SPEC.md §The stage-economics meter — the raw-prefix trap the shell's own comment
// was written against: a stage session's transcript is named `agent-<hex>.jsonl` while its stamp is
// `<hex>` truncated, so the *candidate* is normalized and never the pattern.
pub fn find(i: &Inputs, session8: &str) -> Option<String> {
    let dir = sessions_dir(i);
    if !std::path::Path::new(&dir).is_dir() {
        return None;
    }
    let mut newest: Option<(String, SystemTime)> = None;
    for g in candidate_globs(&dir) {
        for f in crate::walk::glob_entries(&g) {
            if key(&f) != session8 {
                continue;
            }
            let Ok(meta) = std::fs::metadata(&f) else {
                continue;
            };
            let when = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let replace = match &newest {
                Some((_, best)) => when > *best,
                None => true,
            };
            if replace {
                newest = Some((f, when));
            }
        }
    }
    newest.map(|(p, _)| p)
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the whole two-tier transcript population,
// which the under-count bound walks. A third reader of `candidate_globs` rather than a third copy
// of the two patterns, which is what *one glob, not two* has to mean to be true.
pub fn every_transcript(i: &Inputs) -> Vec<String> {
    let dir = sessions_dir(i);
    if !std::path::Path::new(&dir).is_dir() {
        return Vec::new();
    }
    candidate_globs(&dir)
        .iter()
        .flat_map(|g| crate::walk::glob_entries(g))
        .collect()
}
