// spec: gate-sdk/SPEC.md §lib/gate.sh — the Rust counterpart of gate_find's pruned walk,
// reading across the config bridge the value gate_find's own GATE_PRUNE_DIRS array already
// resolved, so one computation of the set serves both substrates
use std::fs;
use std::path::{Path, PathBuf};

// spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged value, tab-split. The crate holds no
// default for a bridged knob, so an absent variable is an error rather than a fallback;
// an empty one is a resolved-empty set, which is why the two part company here.
pub fn prune_dirs() -> Result<Vec<String>, String> {
    let raw = std::env::var("GATE_SDK_KNOB_GATE_PRUNE_DIRS").map_err(|_| {
        "GATE_SDK_KNOB_GATE_PRUNE_DIRS is unset — the gate was invoked without the config \
         bridge gate_command emits, so the prune set could not be resolved"
            .to_string()
    })?;
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    Ok(raw.split('\t').map(String::from).collect())
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged read of one tab-joined array knob,
// the shape `prune_dirs` above has; an absent variable is an error because the crate holds
// no default for a bridged knob, and an empty one is a resolved-empty set.
pub fn knob_array(knob: &str) -> Result<Vec<String>, String> {
    let var = format!("GATE_SDK_KNOB_{}", knob);
    let raw = std::env::var(&var).map_err(|_| {
        format!(
            "{} is unset — the gate was invoked without the config bridge gate_command \
             emits, so {} could not be resolved",
            var, knob
        )
    })?;
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    Ok(raw.split('\t').map(String::from).collect())
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged read of a knob *family*, the prefix form's
// receiving half: every `GATE_SDK_KNOB_<prefix>…` variable, keyed by the suffix the prefix leaves.
// Sorted, so a reader's output order does not depend on the environment's.
pub fn knob_prefix(prefix: &str) -> Vec<(String, String)> {
    let var_prefix = format!("GATE_SDK_KNOB_{}", prefix);
    let mut out: Vec<(String, String)> = std::env::vars()
        .filter_map(|(k, v)| k.strip_prefix(&var_prefix).map(|s| (s.to_string(), v)))
        .collect();
    out.sort();
    out
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — a prefix is a *resolution set, never a roster*: this
// answers "what is <name>'s value", and the caller's roster comes from its own roster knob. A
// reader enumerating `knob_prefix` instead would publish `EVIDENCE_KIT_RUN_ID` as a suite.
pub fn knob_in_family(family: &[(String, String)], name: &str) -> Option<String> {
    family
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.clone())
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the binary side of `gate_kit_roots`: transported,
// never re-derived, because the fallback predicate is anchored at the shell library's own
// location and a binary the installer copies elsewhere cannot recover it
pub fn kit_roots() -> Result<Vec<String>, String> {
    knob_array("GATE_KIT_ROOTS_HERE")
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_kit_roots_rel`'s value, bridged rather than
// derived from `kit_roots` above: the anchor relating the two spellings is not recoverable
// from the absolute set once GATE_SDK_KIT_DIRS overrides it.
pub fn kit_roots_rel() -> Result<Vec<String>, String> {
    knob_array("GATE_KIT_ROOTS_REL")
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged read of one scalar knob. A scalar is a
// one-element array in the wire format, so this is `bridged_array`'s single-value face and an
// absent variable is the same error for the same reason: the crate holds no default.
pub fn knob_scalar(knob: &str) -> Result<String, String> {
    let var = format!("GATE_SDK_KNOB_{}", knob);
    std::env::var(&var).map_err(|_| {
        format!(
            "{} is unset — the gate was invoked without the config bridge gate_command \
             emits, so {} could not be resolved",
            var, knob
        )
    })
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — a bridged root crosses spelled relative to the invoking
// directory, and re-absolutising it against the binary's own cwd is how the reader recovers
// exactly the path the shell library computed
pub fn kit_roots_abs() -> Result<Vec<String>, String> {
    let here = cwd()?;
    Ok(kit_roots()?
        .into_iter()
        .filter(|r| !r.is_empty())
        .map(|r| abs_against(&here, r.trim_end_matches('/')))
        .collect())
}

fn cwd() -> Result<String, String> {
    Ok(std::env::current_dir()
        .map_err(|e| format!("cannot read the current directory: {}", e))?
        .display()
        .to_string()
        .trim_end_matches('/')
        .to_string())
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — a relative bridged root may climb out with `..`, so the
// join is normalised rather than concatenated: an unnormalised `..` component makes every later
// path-prefix comparison fail silently, the defect the canon-kit cohort's edge tree caught.
fn abs_against(here: &str, p: &str) -> String {
    if p.starts_with('/') {
        return normalize_abs(p);
    }
    if p == "." {
        return here.to_string();
    }
    normalize_abs(&format!("{}/{}", here, p.strip_prefix("./").unwrap_or(p)))
}

pub fn normalize_abs(abs: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for seg in abs.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            s => stack.push(s),
        }
    }
    format!("/{}", stack.join("/"))
}

// spec: gate-sdk/SPEC.md §The port-candidate criteria — bash's `[[ str == pat ]]`: whole
// string, no pathname semantics, so `*` and `?` cross `/`. `glob_files`' per-component
// matcher is the pathname-expansion counterpart, and is a different rule from this one.
pub fn pattern_match(pat: &str, s: &str) -> bool {
    glob_here(pat.as_bytes(), s.as_bytes())
}

// spec: canon-kit/SPEC.md §lib/spec.sh — the walk `gate_find <root> -name <n> -type f`
// performs, for a finder selecting by whole filename rather than by extension
pub fn find_named(root: &Path, names: &[&str]) -> Result<Vec<PathBuf>, String> {
    let all = find_any(root)?;
    Ok(all
        .into_iter()
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| names.contains(&n))
                .unwrap_or(false)
        })
        .collect())
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — a directory that cannot be read is
// an error the caller must surface, never a silently smaller corpus: an unreadable
// tree reported as clean is the vacuity the substrate port exists not to introduce.
pub fn find_files(root: &Path, exts: &[&str]) -> Result<Vec<PathBuf>, String> {
    let all = find_any(root)?;
    Ok(all
        .into_iter()
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| exts.contains(&e))
                .unwrap_or(false)
        })
        .collect())
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the pruned walk itself, with no selection:
// the extension filter and the filename filter are both applied by their callers above, so
// one traversal serves both and the recorder still observes every walk at one line.
fn find_any(root: &Path) -> Result<Vec<PathBuf>, String> {
    let prune = prune_dirs()?;
    find_with_prune(root, &|n| prune.iter().any(|d| d == n))
}

// spec: gate-sdk/SPEC.md §The workflow directory — a single-level listing, immediate children
// only and both entry kinds, for a member that inspects one directory's own membership (tracked
// vs ignored) rather than walking a tree beneath it
pub fn list_dir(root: &Path) -> Result<Vec<(String, bool)>, String> {
    #[cfg(test)]
    recorder::note(&root.display().to_string());
    let rd = fs::read_dir(root)
        .map_err(|e| format!("cannot read directory {}: {}", root.display(), e))?;
    let mut out: Vec<(String, bool)> = Vec::new();
    for ent in rd {
        let ent = ent.map_err(|e| format!("cannot read entry in {}: {}", root.display(), e))?;
        let name = ent
            .file_name()
            .into_string()
            .map_err(|_| format!("non-UTF-8 entry name under {}", root.display()))?;
        let is_dir = ent
            .file_type()
            .map_err(|e| format!("cannot stat {} in {}: {}", name, root.display(), e))?
            .is_dir();
        out.push((name, is_dir));
    }
    out.sort();
    Ok(out)
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the same traversal with the prune predicate
// supplied by the caller, because `gate_find`'s bridged set is not every shell form's rule: a
// member whose original reached for a bare `find` prunes what that `find` pruned.
pub fn find_with_prune(
    root: &Path,
    prune: &dyn Fn(&str) -> bool,
) -> Result<Vec<PathBuf>, String> {
    // spec: gate-sdk/SPEC.md §check-reads-couples — every walk passes here, so recording at
    // this one line is what makes unit test A's observation complete.
    #[cfg(test)]
    recorder::note(&root.display().to_string());
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let rd = fs::read_dir(&dir)
            .map_err(|e| format!("cannot read directory {}: {}", dir.display(), e))?;
        let mut entries: Vec<PathBuf> = Vec::new();
        for ent in rd {
            let ent = ent.map_err(|e| format!("cannot read entry in {}: {}", dir.display(), e))?;
            entries.push(ent.path());
        }
        entries.sort();
        for p in entries {
            let name = match p.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            let meta = fs::symlink_metadata(&p)
                .map_err(|e| format!("cannot stat {}: {}", p.display(), e))?;
            if meta.is_dir() {
                if prune(&name) {
                    continue;
                }
                stack.push(p);
            } else if meta.is_file() {
                out.push(p);
            }
        }
    }
    out.sort();
    Ok(out)
}

// spec: gate-sdk/SPEC.md §The port-candidate criteria — one glob component, matched as bash
// matches a pathname component: `*` and `?` never cross `/`, and a leading `.` is literal
// unless the pattern spells it
fn match_component(pat: &str, name: &str) -> bool {
    if name.starts_with('.') && !pat.starts_with('.') {
        return false;
    }
    glob_here(pat.as_bytes(), name.as_bytes())
}

fn glob_here(p: &[u8], s: &[u8]) -> bool {
    let (mut pi, mut si) = (0usize, 0usize);
    let (mut star_p, mut star_s) = (usize::MAX, 0usize);
    while si < s.len() {
        if pi < p.len() {
            match p[pi] {
                b'*' => {
                    star_p = pi;
                    pi += 1;
                    star_s = si;
                    continue;
                }
                b'?' => {
                    pi += 1;
                    si += 1;
                    continue;
                }
                b'[' => {
                    if let Some((ok, next)) = bracket(p, pi, s[si]) {
                        if ok {
                            pi = next;
                            si += 1;
                            continue;
                        }
                    } else if p[pi] == s[si] {
                        pi += 1;
                        si += 1;
                        continue;
                    }
                }
                c => {
                    if c == s[si] {
                        pi += 1;
                        si += 1;
                        continue;
                    }
                }
            }
        }
        if star_p != usize::MAX {
            star_s += 1;
            si = star_s;
            pi = star_p + 1;
            continue;
        }
        return false;
    }
    while pi < p.len() && p[pi] == b'*' {
        pi += 1;
    }
    pi == p.len()
}

// spec: gate-sdk/SPEC.md §The port-candidate criteria — a bracket expression with `!`/`^`
// negation and `a-z` ranges; an unterminated `[` is a literal, as the shell reads it
fn bracket(p: &[u8], at: usize, c: u8) -> Option<(bool, usize)> {
    let mut i = at + 1;
    let neg = matches!(p.get(i), Some(&b'!') | Some(&b'^'));
    if neg {
        i += 1;
    }
    let mut matched = false;
    let mut first = true;
    while i < p.len() {
        if p[i] == b']' && !first {
            return Some((matched != neg, i + 1));
        }
        first = false;
        if i + 2 < p.len() && p[i + 1] == b'-' && p[i + 2] != b']' {
            if p[i] <= c && c <= p[i + 2] {
                matched = true;
            }
            i += 3;
            continue;
        }
        if p[i] == c {
            matched = true;
        }
        i += 1;
    }
    None
}

fn has_meta(s: &str) -> bool {
    s.bytes().any(|b| b == b'*' || b == b'?' || b == b'[')
}

// spec: gate-sdk/SPEC.md §The port-candidate criteria — `**`-capable list matching over a
// bridged glob array, the semantics committed once there rather than re-decided per port.
// Bash-faithful: no prune set applies, because pathname expansion has none.
pub fn glob_files(root: &Path, globs: &[String]) -> Result<Vec<PathBuf>, String> {
    #[cfg(test)]
    recorder::note(&root.display().to_string());
    let mut out = Vec::new();
    for g in globs {
        let comps: Vec<&str> = g.split('/').filter(|c| !c.is_empty()).collect();
        let mut hits: Vec<PathBuf> = Vec::new();
        expand(root, &comps, &mut hits)?;
        hits.sort();
        out.extend(hits);
    }
    Ok(out)
}

fn expand(base: &Path, comps: &[&str], out: &mut Vec<PathBuf>) -> Result<(), String> {
    if comps.is_empty() {
        if base.is_file() {
            out.push(base.to_path_buf());
        }
        return Ok(());
    }
    let (head, rest) = (comps[0], &comps[1..]);
    if head == "**" {
        expand(base, rest, out)?;
        for d in subdirs(base)? {
            expand(&d, comps, out)?;
        }
        return Ok(());
    }
    if !has_meta(head) {
        return expand(&base.join(head), rest, out);
    }
    let rd = match fs::read_dir(base) {
        Ok(r) => r,
        Err(_) => return Ok(()),
    };
    let mut names: Vec<PathBuf> = Vec::new();
    for ent in rd {
        let ent = ent.map_err(|e| format!("cannot read entry in {}: {}", base.display(), e))?;
        names.push(ent.path());
    }
    names.sort();
    for p in names {
        let name = match p.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if match_component(head, name) {
            expand(&p, rest, out)?;
        }
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — bash's `[[ -r "$d" && -x "$d" ]]` on a
// directory, answered by attempting the open. It enumerates nothing into a corpus, so it is
// deliberately not noted to the recorder unit test A observes through.
pub fn dir_readable(p: &Path) -> bool {
    fs::read_dir(p).is_ok()
}

fn subdirs(base: &Path) -> Result<Vec<PathBuf>, String> {
    let rd = match fs::read_dir(base) {
        Ok(r) => r,
        Err(_) => return Ok(Vec::new()),
    };
    let mut out = Vec::new();
    for ent in rd {
        let ent = ent.map_err(|e| format!("cannot read entry in {}: {}", base.display(), e))?;
        let p = ent.path();
        let meta = fs::symlink_metadata(&p).map_err(|e| format!("cannot stat {}: {}", p.display(), e))?;
        if meta.is_dir() {
            let hidden = p
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with('.'))
                .unwrap_or(false);
            if !hidden {
                out.push(p);
            }
        }
    }
    out.sort();
    Ok(out)
}

// spec: gate-sdk/SPEC.md §check-reads-couples — the recorder unit test A observes through.
// Test-scoped deliberately: a production recorder would be state with no reader.
#[cfg(test)]
pub mod recorder {
    use std::cell::RefCell;

    thread_local! {
        static OBSERVED: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
    }

    pub fn start() {
        OBSERVED.with(|o| *o.borrow_mut() = Some(Vec::new()));
    }

    pub fn stop() -> Vec<String> {
        OBSERVED.with(|o| o.borrow_mut().take()).unwrap_or_default()
    }

    pub fn note(root: &str) {
        OBSERVED.with(|o| {
            if let Some(v) = o.borrow_mut().as_mut() {
                if !v.iter().any(|e| e == root) {
                    v.push(root.to_string());
                }
            }
        });
    }
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the unit tests reach find_files without going through
// gate_command, so they stand in for the bridge by asking its one owner, the kit's shell
// library, for the resolved value; a literal here would restore the deleted second default
#[cfg(test)]
pub fn bridge_declared_knobs() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let out = std::process::Command::new("bash")
            .arg("-c")
            .arg("source gate-sdk/lib/gate.sh; IFS=$'\\t'; printf '%s' \"${GATE_PRUNE_DIRS[*]}\"")
            .current_dir(&root)
            .output()
            .expect("cannot run the shell library's knob resolution");
        assert!(
            out.status.success(),
            "gate-sdk/lib/gate.sh could not resolve GATE_PRUNE_DIRS: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
        let value = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            !value.is_empty(),
            "the shell library resolved GATE_PRUNE_DIRS to nothing — the tests would walk an \
             unpruned tree and observe roots no production invocation reaches"
        );
        std::env::set_var("GATE_SDK_KNOB_GATE_PRUNE_DIRS", value);
    });
}

// spec: gate-sdk/SPEC.md §run-gate-tests — a bridged member's knob values resolve inside the
// case dir, so this asks the one owner from the cwd the runner uses; resolving at the repo
// root would make the crate's case runner the second oracle that section was repaired for
#[cfg(test)]
pub fn bridge_case_knobs(case: &Path, gate: &str, knobs: &[&str]) {
    if knobs.is_empty() {
        return;
    }
    let lib = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("gate-sdk/lib/gate.sh");
    // spec: gate-sdk/SPEC.md §lib/gate.sh — resolution goes through the library's own per-name
    // dispatch, so the prefix form has one implementation rather than one here and one there
    let script = "source \"$1\"; g=\"$2\"; shift 2; \
                  for k in \"$@\"; do gate_knob_env_one \"$k\" \"$g\" || exit 2; done";
    let out = std::process::Command::new("bash")
        .arg("-c")
        .arg(script)
        .arg("bash")
        .arg(&lib)
        .arg(gate)
        .args(knobs)
        .current_dir(case)
        .output()
        .expect("cannot run the shell library's knob resolution");
    assert!(
        out.status.success(),
        "the config bridge could not resolve {}'s knobs in {}: {}",
        gate,
        case.display(),
        String::from_utf8_lossy(&out.stderr).trim()
    );
    // spec: gate-sdk/SPEC.md §lib/gate.sh — the wire format is one tab-joined value per knob
    // and the library refuses an element carrying a newline, so one line is one knob and the
    // first `=` is its only separator
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let (name, value) = line
            .split_once('=')
            .unwrap_or_else(|| panic!("unparseable knob line from the bridge: {}", line));
        std::env::set_var(name, value);
    }
}

// spec: gate-sdk/SPEC.md §check-reads-couples — unit test A's case-dir lookup lives here
// rather than beside the registry so that the directory scan it needs obeys test B instead
// of being excused from it.
#[cfg(test)]
pub fn fixture_case_dirs(gate: &str) -> Vec<PathBuf> {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let rd = fs::read_dir(&repo)
        .unwrap_or_else(|e| panic!("cannot read repo root {}: {}", repo.display(), e));
    let mut kits: Vec<PathBuf> = Vec::new();
    for ent in rd {
        kits.push(ent.expect("cannot read a repo-root entry").path());
    }
    kits.sort();
    let mut out = Vec::new();
    for kit in kits {
        for case in ["good", "bad"] {
            let d = kit.join("gate-tests").join(gate).join(case);
            if d.is_dir() {
                out.push(d);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the prefix form's receiving half: the family is keyed
    // by the suffix, sorted independently of the environment's order, and a lookup is by name
    // because a prefix is a resolution set rather than a roster.
    #[test]
    fn a_knob_family_is_keyed_by_suffix_and_read_by_name_not_enumerated() {
        std::env::set_var("GATE_SDK_KNOB_PROBEFAM_beta", "b");
        std::env::set_var("GATE_SDK_KNOB_PROBEFAM_alpha", "a");
        std::env::set_var("GATE_SDK_KNOB_PROBEFAM_ID", "not-a-member");
        let fam = knob_prefix("PROBEFAM_");
        let keys: Vec<&str> = fam.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, vec!["ID", "alpha", "beta"], "family is sorted by suffix");
        assert_eq!(knob_in_family(&fam, "alpha").as_deref(), Some("a"));
        assert_eq!(knob_in_family(&fam, "absent"), None);
        // spec: gate-sdk/SPEC.md §lib/gate.sh — the decoy resolves and is simply never looked up,
        // which is what keeps EVIDENCE_KIT_RUN_ID out of the emitted suite roster
        assert!(knob_in_family(&fam, "ID").is_some());
        for k in ["beta", "alpha", "ID"] {
            std::env::remove_var(format!("GATE_SDK_KNOB_PROBEFAM_{}", k));
        }
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — unit test B: a walk outside this file
    // would be invisible to the recorder and would unverify test A, so the roster of
    // filesystem-walk spellings is asserted absent from every other module.
    const WALK_APIS: &[&str] = &["read_dir", "ReadDir"];

    #[test]
    fn no_module_outside_this_one_walks_the_filesystem() {
        bridge_declared_knobs();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files = find_files(&src, &["rs"]).expect("cannot enumerate the crate's sources");
        assert!(!files.is_empty(), "no crate source found to scan");
        let mut offenders: Vec<String> = Vec::new();
        for f in &files {
            if f.file_name().and_then(|n| n.to_str()) == Some("walk.rs") {
                continue;
            }
            let text = fs::read_to_string(f)
                .unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            for api in WALK_APIS {
                if text.contains(api) {
                    offenders.push(format!("{} names {}", f.display(), api));
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "a filesystem walk outside walk.rs is invisible to the recorder, so unit test A \
             would assert over an incomplete observation: {:?} — route it through \
             walk::find_files",
            offenders
        );
    }

    // spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — one entry
    // per crate in the resolved graph, with the clause of the dependency bar it was admitted
    // under
    const ADMITTED_CRATES: &[(&str, &str)] = &[
        ("itoa", "integer formatting; no walk, no subprocess, no socket"),
        ("memchr", "byte search; no walk, no subprocess, no socket"),
        (
            "proc-macro2",
            "build-time token plumbing, lock-only (not feature-activated); no walk",
        ),
        (
            "quote",
            "build-time token plumbing, lock-only (not feature-activated); no walk",
        ),
        (
            "serde",
            "serialization traits, lock-only (not feature-activated); no walk",
        ),
        ("serde_core", "serialization traits; no walk, no subprocess, no socket"),
        (
            "serde_derive",
            "proc macro, build-time and lock-only (not feature-activated); no walk",
        ),
        (
            "serde_json",
            "the JSON reader this cohort took; parses from a string the gate reads, so no walk of its own",
        ),
        (
            "syn",
            "build-time parser, lock-only (not feature-activated); no walk",
        ),
        (
            "unicode-ident",
            "build-time character classification, lock-only (not feature-activated); no walk",
        ),
        ("zmij", "float formatting; no walk, no subprocess, no socket"),
    ];

    // spec: gate-sdk/SPEC.md §check-reads-couples — the same assertion's other half: a vendored
    // walker cannot be caught by a spelling roster, so the graph that could carry one is held to
    // an allowlist instead of assumed empty
    #[test]
    fn every_crate_in_the_resolved_graph_is_admitted_by_name() {
        let lock = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
        let text = fs::read_to_string(&lock).unwrap_or_else(|e| {
            panic!(
                "cannot read {}: {} — the lock is tracked precisely so this assertion has a \
                 graph to read without resolving one from the network",
                lock.display(),
                e
            )
        });
        let mut resolved: Vec<String> = Vec::new();
        for line in text.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("name = \"") {
                if let Some(name) = rest.strip_suffix('"') {
                    if name != "checkwright-gates" {
                        resolved.push(name.to_string());
                    }
                }
            }
        }
        assert!(
            !resolved.is_empty(),
            "the lock named no dependency — with an empty graph this assertion holds over \
             nothing, so read it as unverified rather than as clean"
        );
        let unadmitted: Vec<&String> = resolved
            .iter()
            .filter(|n| !ADMITTED_CRATES.iter().any(|(a, _)| *a == n.as_str()))
            .collect();
        assert!(
            unadmitted.is_empty(),
            "the resolved graph carries crate(s) no clause admits ({:?}). A vendored walker \
             would bypass walk.rs's recorder and unverify unit test A — confirm each new crate \
             performs no filesystem walk, spawns no subprocess and opens no socket, then add it \
             to ADMITTED_CRATES with the clause it was admitted under",
            unadmitted
        );
        let stale: Vec<&str> = ADMITTED_CRATES
            .iter()
            .map(|(n, _)| *n)
            .filter(|n| !resolved.iter().any(|r| r == n))
            .collect();
        assert!(
            stale.is_empty(),
            "ADMITTED_CRATES names crate(s) the resolved graph no longer carries ({:?}) — an \
             allowlist that outlives its graph stops being the machine-held form of the bar",
            stale
        );
    }
}
