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

// spec: gate-sdk/SPEC.md §Fail-closed contract — a directory that cannot be read is
// an error the caller must surface, never a silently smaller corpus: an unreadable
// tree reported as clean is the vacuity the substrate port exists not to introduce.
pub fn find_files(root: &Path, exts: &[&str]) -> Result<Vec<PathBuf>, String> {
    // spec: gate-sdk/SPEC.md §check-reads-couples — every walk passes here, so recording at
    // this one line is what makes unit test A's observation complete.
    #[cfg(test)]
    recorder::note(&root.display().to_string());
    let prune = prune_dirs()?;
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
                if prune.contains(&name) {
                    continue;
                }
                stack.push(p);
            } else if meta.is_file() {
                if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    if exts.contains(&ext) {
                        out.push(p);
                    }
                }
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
        static OBSERVED: RefCell<Option<Vec<String>>> = RefCell::new(None);
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
    let script = "source \"$1\"; g=\"$2\"; shift 2; \
                  for k in \"$@\"; do v=\"$(_gate_knob_value \"$k\" \"$g\")\" || exit 2; \
                  printf '%s=%s\\n' \"$k\" \"$v\"; done";
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
        std::env::set_var(format!("GATE_SDK_KNOB_{}", name), value);
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

    // spec: gate-sdk/SPEC.md §check-reads-couples — the same assertion's other half: a
    // vendored walker cannot be caught by a spelling roster, so the empty dependency set
    // that rules one out is asserted instead of assumed.
    #[test]
    fn the_crate_vendors_no_walker_because_it_vendors_nothing() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let text = fs::read_to_string(&manifest)
            .unwrap_or_else(|e| panic!("cannot read {}: {}", manifest.display(), e));
        let mut in_deps = false;
        let mut declared: Vec<String> = Vec::new();
        for line in text.lines() {
            let t = line.trim();
            if t.starts_with('[') {
                in_deps = t == "[dependencies]";
                continue;
            }
            if in_deps && !t.is_empty() && !t.starts_with('#') {
                declared.push(t.to_string());
            }
        }
        assert!(
            declared.is_empty(),
            "the crate now declares dependencies ({:?}). A vendored walker would bypass \
             walk.rs's recorder and unverify unit test A, and an empty dependency set is \
             also what gate-sdk/SPEC.md's vendoring model states the payload rests on — \
             confirm the new dependency performs no filesystem walk, then widen this test \
             deliberately rather than deleting it",
            declared
        );
    }
}
