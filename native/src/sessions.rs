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
    let base = path.rsplit(['/', '\\']).next().unwrap_or(path);
    normalize(base.strip_suffix(".jsonl").unwrap_or(base))
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the harness's own cut and hash, values it owns,
// so neither is a knob
const SLUG_CAP: usize = 200;

fn harness_hash(units: &[u16]) -> i32 {
    units
        .iter()
        .fold(0i32, |h, &c| (h << 5).wrapping_sub(h).wrapping_add(i32::from(c)))
}

fn base36(mut n: u32) -> String {
    let mut digits = Vec::new();
    loop {
        digits.push(char::from_digit(n % 36, 36).unwrap_or('0'));
        n /= 36;
        if n == 0 {
            break;
        }
    }
    digits.iter().rev().collect()
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the cwd slug is the harness's encoder: every
// UTF-16 unit outside ASCII `[A-Za-z0-9]` maps to `-`, and past the cap the slug is cut and
// suffixed with `-` and the base-36 absolute value of the raw path's 32-bit hash
pub(crate) fn slug(path: &str) -> String {
    let units: Vec<u16> = path.encode_utf16().collect();
    let folded: String = units
        .iter()
        .map(|&u| match u8::try_from(u) {
            Ok(b) if b.is_ascii_alphanumeric() => char::from(b),
            _ => '-',
        })
        .collect();
    if units.len() <= SLUG_CAP {
        return folded;
    }
    format!("{}-{}", &folded[..SLUG_CAP], base36(harness_hash(&units).unsigned_abs()))
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — source 3's sessions dir: the override, else
// `<config-home>/projects/<cwd-slug>` with `$CLAUDE_CONFIG_DIR` or `~/.claude` as the home. The
// override arrives as a field, so each kit resolves its own knob and hands the answer in.
pub fn sessions_dir(i: &Inputs) -> String {
    if !i.sessions_dir.is_empty() {
        return i.sessions_dir.clone();
    }
    format!("{}/projects/{}", config_home(&i.config_home, &i.home), slug(&i.here))
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the variable `~` is read from, one name for
// every reader of the harness's home
pub const HOME_VAR: &str = if cfg!(windows) { "USERPROFILE" } else { "HOME" };

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the harness's config home, `$CLAUDE_CONFIG_DIR`
// when set non-empty, else `~/.claude`: the one derivation every reader of that home shares
pub fn config_home(config_dir: &str, home: &str) -> String {
    if config_dir.is_empty() {
        format!("{}/.claude", home)
    } else {
        config_dir.to_string()
    }
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

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the child-flag verification as its own verdict.
// `Delegated` carries the narrowed scan's newest pick, which is source 3's answer and never the
// delegated session's own transcript: the overhead meter must not measure it.
pub enum Delegation {
    TopLevel(String),
    Delegated(String),
    Undetermined,
}

pub fn delegation(i: &Inputs) -> Delegation {
    if i.harness_id.is_empty() {
        return Delegation::Undetermined;
    }
    if i.child.is_empty() {
        return Delegation::TopLevel(i.harness_id.clone());
    }
    let dir = sessions_dir(i);
    let mut newest: Option<(String, SystemTime)> = None;
    pick(&mut newest, &narrowed_glob(&dir, &i.harness_id));
    if let Some((path, _)) = newest {
        return Delegation::Delegated(path);
    }
    if std::path::Path::new(&top_level(&dir, &i.harness_id)).exists() {
        return Delegation::TopLevel(i.harness_id.clone());
    }
    Delegation::Undetermined
}

fn narrowed_glob(dir: &str, harness_id: &str) -> String {
    format!("{}/{}/subagents/*.jsonl", dir, harness_id)
}

// spec: lifecycle-kit/SPEC.md §The journal arm — the candidates of the scan `resolve` selects
// from, newest first, a tie keeping glob order as `pick` does; empty where `resolve` answers from
// the environment and scans nothing
pub fn scan_newest_first(i: &Inputs) -> Vec<String> {
    if !i.session_id.is_empty() || (!i.harness_id.is_empty() && i.child.is_empty()) {
        return Vec::new();
    }
    let dir = sessions_dir(i);
    let globs = if i.harness_id.is_empty() {
        candidate_globs(&dir).to_vec()
    } else {
        vec![narrowed_glob(&dir, &i.harness_id)]
    };
    let mut found: Vec<(String, SystemTime)> = globs
        .iter()
        .flat_map(|g| crate::walk::glob_entries(g))
        .filter_map(|f| {
            let when = std::fs::metadata(&f).ok()?.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            Some((f, when))
        })
        .collect();
    found.sort_by_key(|a| std::cmp::Reverse(a.1));
    found.into_iter().map(|(f, _)| f).collect()
}

pub fn top_level(dir: &str, harness_id: &str) -> String {
    format!("{}/{}.jsonl", dir, harness_id)
}

pub fn newest(i: &Inputs) -> Option<String> {
    let dir = sessions_dir(i);
    let mut newest: Option<(String, SystemTime)> = None;
    for g in candidate_globs(&dir) {
        pick(&mut newest, &g);
    }
    newest.map(|(p, _)| p)
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the derivation order and its exit-2 refusals,
// returning the *winning path* rather than the normalized key, so a caller needing the transcript
// does not re-glob for the id it was handed. `key` is the identity on the two early returns.
pub fn resolve(i: &Inputs) -> Result<String, String> {
    if !i.session_id.is_empty() {
        return Ok(i.session_id.clone());
    }
    let verdict = delegation(i);
    if let (Delegation::TopLevel(id), true) = (&verdict, i.child.is_empty()) {
        return Ok(id.clone());
    }
    let dir = sessions_dir(i);
    if !std::path::Path::new(&dir).is_dir() {
        return Err(format!(
            "sessions dir not found: {}\n  help: set LIFECYCLE_KIT_SESSIONS_DIR to the agent \
             transcript directory for this tree.",
            dir
        ));
    }
    match verdict {
        Delegation::TopLevel(id) => Ok(top_level(&dir, &id)),
        Delegation::Delegated(path) => Ok(path),
        Delegation::Undetermined if !i.harness_id.is_empty() => Err(format!(
            "no subagent transcript under {}/{}/subagents and no top-level {}\n  help: confirm \
             this is the right sessions dir (LIFECYCLE_KIT_SESSIONS_DIR).",
            dir,
            i.harness_id,
            top_level(&dir, &i.harness_id)
        )),
        Delegation::Undetermined => newest(i).ok_or_else(|| {
            format!(
                "no transcript (*.jsonl) under {}\n  help: confirm this is the right sessions \
                 dir (LIFECYCLE_KIT_SESSIONS_DIR).",
                dir
            )
        }),
    }
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

#[cfg(test)]
mod tests {
    use super::{key, slug};

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — a reader spelling `HOME` itself misses the
    // harness's home on Windows, so every reader names `HOME_VAR`
    #[test]
    fn no_reader_spells_the_home_variable_itself() {
        let _knobs = crate::knobenv::lock();
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files = crate::walk::find_files(&src, &["rs"]).expect("cannot enumerate the crate's sources");
        assert!(!files.is_empty(), "no crate source found to scan");
        let hits: Vec<_> = files
            .iter()
            .filter(|f| std::fs::read_to_string(f).is_ok_and(|s| s.contains("var(\"HOME\")")))
            .collect();
        assert!(hits.is_empty(), "{:?}", hits);
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — each case is a spelling the harness was seen
    // to create: the Linux root from an unauthenticated run, the Windows one from the CI probe of
    // a checkout at `D:\a\checkwright\checkwright`
    #[test]
    fn the_slug_is_the_harness_encoder() {
        assert_eq!(slug("/srv/my_proj dir.x"), "-srv-my-proj-dir-x");
        assert_eq!(slug("D:/a/checkwright/checkwright"), "D--a-checkwright-checkwright");
        assert_eq!(slug(r"D:\a\checkwright\checkwright"), "D--a-checkwright-checkwright");
        assert_eq!(slug("/x/\u{1F600}y"), "-x---y");
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the suffix is the harness's own expression,
    // evaluated with `node -e 'function fQ(t){let e=0;for(let n=0;n<t.length;n++)e=(e<<5)-e+t.charCodeAt(n)|0;return e}console.log(Math.abs(fQ("/"+"a".repeat(259))).toString(36))'`
    #[test]
    fn a_slug_past_the_cap_is_cut_and_hash_suffixed() {
        let path = format!("/{}", "a".repeat(259));
        assert_eq!(slug(&path), format!("-{}-z255qm", "a".repeat(199)));
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the basename under either separator, so a
    // Windows transcript path keys by its file name rather than its drive
    #[test]
    fn a_key_takes_the_basename_under_either_separator() {
        assert_eq!(key("/s/lead/subagents/agent-abcdef1234.jsonl"), "abcdef12");
        assert_eq!(key(r"C:\s\lead\subagents\agent-abcdef1234.jsonl"), "abcdef12");
        assert_eq!(key("0123456789ab"), "01234567");
    }
}
