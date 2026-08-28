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

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_path_pruned`: the prune-dir set matched as a
// leading, `./`-led or interior path component, beside the `prune_dirs` whose value it reads
pub fn path_pruned(p: &str, prune: &[String]) -> bool {
    prune.iter().any(|d| {
        p.starts_with(&format!("{}/", d))
            || p.starts_with(&format!("./{}/", d))
            || p.contains(&format!("/{}/", d))
    })
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the compiled face of the `--tree` corpus
// rule: tracked `*.sh`, minus the `*.test.sh` suffix, minus `path_pruned` above. A tree with no
// tracked set yields none, on `authoring_tree`'s own degrade; an unresolved knob still fails closed
pub fn tracked_shell_tree() -> Result<Vec<String>, String> {
    let prune = prune_dirs()?;
    let bytes = match crate::proc::run("git", &["ls-files", "--", "*.sh"]) {
        Ok(c) => match c.stdout() {
            Some(b) => b.to_vec(),
            None => return Ok(Vec::new()),
        },
        Err(_) => return Ok(Vec::new()),
    };
    Ok(String::from_utf8_lossy(&bytes)
        .lines()
        .filter(|l| !l.is_empty() && !l.ends_with(".test.sh") && !path_pruned(l, &prune))
        .map(String::from)
        .collect())
}

// spec: gate-sdk/SPEC.md §port-blockers — the file's own header block, the leading run of shebang,
// comment and blank lines. It sits on this universal layer beside the corpus rule it is the other
// half of, and both readers of the disposition below are readers of it.
pub fn header_block(text: &str) -> String {
    let mut out = String::new();
    for (idx, line) in text.lines().enumerate() {
        let t = line.trim_start_matches([' ', '\t']);
        if !(t.is_empty() || t.starts_with('#') || (idx == 0 && line.starts_with("#!"))) {
            break;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

// spec: gate-sdk/SPEC.md §port-blockers — the port disposition a header block declares. There is no
// fourth member because there is no fourth disposition, and *held* is separated from *no-port*
// because a temporary hold is not a permanent one: folding them silently falsifies a subtraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
    Owed,
    NoPort,
    PortUntil(String),
}

// spec: gate-sdk/SPEC.md §port-blockers — one well-formedness rule rather than three: an empty
// cause, a missing slug, a doubled field and a file carrying **both** are each `Owed`, because a
// file that has not made a reviewable declaration has not made one.
pub fn disposition(header: &str) -> Disposition {
    let no_port = header_field(header, "no-port:");
    let port_until = header_field(header, "port-until:");
    match (no_port.as_slice(), port_until.as_slice()) {
        ([cause], []) => {
            if cause.chars().all(char::is_whitespace) {
                Disposition::Owed
            } else {
                Disposition::NoPort
            }
        }
        ([], [payload]) => {
            let t = payload.trim_start_matches([' ', '\t']).as_bytes();
            let mut n = 0usize;
            while n < t.len() && (t[n].is_ascii_lowercase() || t[n].is_ascii_digit() || t[n] == b'-')
            {
                n += 1;
            }
            if n == 0 {
                Disposition::Owed
            } else {
                Disposition::PortUntil(String::from_utf8_lossy(&t[..n]).into_owned())
            }
        }
        _ => Disposition::Owed,
    }
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a header field opens the line: '#' in column
// one, optional blanks, the field name. An indented '#' is a comment inside a block rather than a
// header field, which is what keeps a nested declaration out of the read.
fn header_field<'a>(header: &'a str, name: &str) -> Vec<&'a str> {
    header
        .lines()
        .filter_map(|l| l.strip_prefix('#'))
        .filter_map(|r| r.trim_start_matches([' ', '\t']).strip_prefix(name))
        .collect()
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

// spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged read of a *keyed* knob, the map counterpart
// of knob_array: each element splits on its first `=`, absent is an error where empty is a
// resolved-empty map, and pairs arrive in the sorted order the wire carries.
pub fn knob_map(knob: &str) -> Result<Vec<(String, String)>, String> {
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
    let mut out: Vec<(String, String)> = Vec::new();
    for el in raw.split('\t') {
        match el.split_once('=') {
            Some((k, v)) => out.push((k.to_string(), v.to_string())),
            None => {
                return Err(format!(
                    "{} carries the element '{}', which has no '=' — a keyed knob's element \
                     is <key>=<value>, so {} could not be read as a map",
                    var, el, knob
                ))
            }
        }
    }
    Ok(out)
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

// spec: context-kit/SPEC.md §check-memory-off — bash pathname expansion of one word, over
// entries of *any* kind rather than files only, existence-filtered: a metacharacter-free word
// expands to itself, so one filter covers that branch and the globbed one as `[[ -e ]]` did.
// spec: gate-sdk/SPEC.md §check-reads-couples — like `glob_files` it resolves corpus roots
// rather than reading a corpus, so directories opened while expanding are not noted to the
// recorder; the caller's own walk of each resolved root is what unit test A observes.
pub fn glob_entries(pattern: &str) -> Vec<String> {
    if pattern.is_empty() {
        return Vec::new();
    }
    if !has_meta(pattern) {
        return if Path::new(pattern).exists() {
            vec![pattern.to_string()]
        } else {
            Vec::new()
        };
    }
    let comps: Vec<&str> = pattern.split('/').filter(|c| !c.is_empty()).collect();
    let anchor = if pattern.starts_with('/') { "/" } else { "" };
    let mut out: Vec<String> = Vec::new();
    expand_entries(anchor, &comps, &mut out);
    out.sort();
    out
}

fn glob_join(prefix: &str, name: &str) -> String {
    match prefix {
        "" => name.to_string(),
        "/" => format!("/{}", name),
        p => format!("{}/{}", p, name),
    }
}

fn expand_entries(prefix: &str, comps: &[&str], out: &mut Vec<String>) {
    let Some((head, rest)) = comps.split_first() else {
        if !prefix.is_empty() && Path::new(prefix).exists() {
            out.push(prefix.to_string());
        }
        return;
    };
    if !has_meta(head) {
        expand_entries(&glob_join(prefix, head), rest, out);
        return;
    }
    let dir = if prefix.is_empty() { "." } else { prefix };
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut names: Vec<String> = Vec::new();
    for ent in rd.flatten() {
        if let Some(n) = ent.file_name().to_str() {
            names.push(n.to_string());
        }
    }
    names.sort();
    for n in names {
        if match_component(head, &n) {
            expand_entries(&glob_join(prefix, &n), rest, out);
        }
    }
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
pub fn bridge_declared_knobs(knobs: &crate::knobenv::KnobEnv) {
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
        knobs.set("GATE_SDK_KNOB_GATE_PRUNE_DIRS", &value);
    });
}

// spec: gate-sdk/SPEC.md §run-gate-tests — a bridged member's knob values resolve inside the
// case dir, so this asks the one owner from the cwd the runner uses; resolving at the repo
// root would make the crate's case runner the second oracle that section was repaired for
#[cfg(test)]
pub fn bridge_case_knobs(env: &crate::knobenv::KnobEnv, case: &Path, gate: &str, knobs: &[&str]) {
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
        env.set(name, value);
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

    // spec: gate-sdk/SPEC.md §port-blockers — the header block ends at the first line that is
    // neither shebang, comment nor blank, which is what stops a declaration being read out of a
    // heredoc literal in a script that writes shell
    #[test]
    fn the_header_block_ends_at_the_first_code_line() {
        let text = "#!/usr/bin/env bash\n# no-port: a cause\n\nset -e\n# port-until: later\n";
        let header = header_block(text);
        assert!(header.contains("# no-port: a cause"));
        assert!(
            !header.contains("port-until"),
            "a field below the first code line was read as a header field"
        );
        assert_eq!(disposition(&header), Disposition::NoPort);
    }

    // spec: gate-sdk/SPEC.md §port-blockers — one well-formedness rule rather than three: an empty
    // cause, a missing slug, a doubled field and a file carrying both are each `Owed`
    #[test]
    fn a_declaration_that_is_not_reviewable_is_owed() {
        assert_eq!(disposition("# no-port:\n"), Disposition::Owed);
        assert_eq!(disposition("# no-port:   \n"), Disposition::Owed);
        assert_eq!(disposition("# port-until:\n"), Disposition::Owed);
        assert_eq!(disposition("# port-until: NotASlug\n"), Disposition::Owed);
        assert_eq!(
            disposition("# no-port: a cause\n# port-until: a-slug\n"),
            Disposition::Owed,
            "a file carrying both fields declared neither"
        );
        assert_eq!(
            disposition("# no-port: one\n# no-port: two\n"),
            Disposition::Owed,
            "two causes are not one reviewable declaration"
        );
        assert_eq!(disposition("#!/usr/bin/env bash\n\n"), Disposition::Owed);
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the slug is the leading run of `[a-z0-9-]` after the
    // field, and an indented '#' is a comment inside a block rather than a header field
    #[test]
    fn a_held_declaration_yields_its_slug_and_an_indented_one_yields_none() {
        assert_eq!(
            disposition("#port-until: a-slug trailing prose\n"),
            Disposition::PortUntil("a-slug".to_string())
        );
        assert_eq!(
            disposition("#\tport-until:\ta-slug\n"),
            Disposition::PortUntil("a-slug".to_string())
        );
        assert_eq!(disposition("  # port-until: a-slug\n"), Disposition::Owed);
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the prune predicate is a path *component* test, so a
    // basename that merely starts with a prune name is not pruned. One test, because there is now
    // one predicate: it sat beside each consuming member while each carried its own copy.
    #[test]
    fn the_prune_predicate_matches_components_and_not_prefixes() {
        let p = vec![
            "gate-tests".to_string(),
            "worktrees".to_string(),
            ".git".to_string(),
        ];
        assert!(path_pruned("kit/gate-tests/x/good/a.md", &p));
        assert!(path_pruned("gate-tests/a.md", &p));
        assert!(path_pruned("./worktrees/a.md", &p));
        assert!(path_pruned("worktrees/a.md", &p));
        assert!(!path_pruned("kit/gate-tests-notes/a.md", &p));
        assert!(!path_pruned("a/b.md", &p));
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the prefix form's receiving half: the family is keyed
    // by the suffix, sorted independently of the environment's order, and a lookup is by name
    // because a prefix is a resolution set rather than a roster.
    #[test]
    fn a_knob_family_is_keyed_by_suffix_and_read_by_name_not_enumerated() {
        let knobs = crate::knobenv::lock();
        knobs.set("GATE_SDK_KNOB_PROBEFAM_beta", "b");
        knobs.set("GATE_SDK_KNOB_PROBEFAM_alpha", "a");
        knobs.set("GATE_SDK_KNOB_PROBEFAM_ID", "not-a-member");
        let fam = knob_prefix("PROBEFAM_");
        let keys: Vec<&str> = fam.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, vec!["ID", "alpha", "beta"], "family is sorted by suffix");
        assert_eq!(knob_in_family(&fam, "alpha").as_deref(), Some("a"));
        assert_eq!(knob_in_family(&fam, "absent"), None);
        // spec: gate-sdk/SPEC.md §lib/gate.sh — the decoy resolves and is simply never looked up,
        // which is what keeps EVIDENCE_KIT_RUN_ID out of the emitted suite roster
        assert!(knob_in_family(&fam, "ID").is_some());
        for k in ["beta", "alpha", "ID"] {
            knobs.remove(&format!("GATE_SDK_KNOB_PROBEFAM_{}", k));
        }
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — unit test B: a walk outside this file
    // would be invisible to the recorder and would unverify test A, so the roster of
    // filesystem-walk spellings is asserted absent from every other module.
    const WALK_APIS: &[&str] = &["read_dir", "ReadDir"];

    #[test]
    fn no_module_outside_this_one_walks_the_filesystem() {
        bridge_declared_knobs(&crate::knobenv::lock());
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
