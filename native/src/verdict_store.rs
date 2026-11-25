// spec: gate-sdk/SPEC.md §run-gates — the verdict store: the key a judged registry run is kept
// under and the record kept there, shared by `--run`'s write and `--verdict-lookup`'s read so the
// key is computed in one place
use crate::exit::{self, Fault};
use crate::provenance::Provenance;
use crate::sarif::Scope;
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};

pub const KNOB: &str = "GATE_SDK_VERDICT_STORE";
const CONTRACT: &str = "1.0.0";
const TAG: &str = "verdict-lookup";

pub const KNOBS: &[&str] = &[KNOB];

pub fn file_name(tree: &str, version: &str, set: &str) -> String {
    let key = format!("verdict-key v1\n{}\n{}\n{}\n", tree, version, set);
    format!("{}.json", crate::sha256::hex(key.as_bytes()))
}

// spec: gate-sdk/SPEC.md §run-gates — unset or empty keeps nothing; a relative directory is refused
pub fn configured() -> Result<Option<PathBuf>, Fault> {
    let raw = std::env::var_os(KNOB).unwrap_or_default();
    if raw.is_empty() {
        return Ok(None);
    }
    let dir = raw
        .to_str()
        .ok_or_else(|| Fault::refused(crate::walk::not_utf8(KNOB, &crate::walk::escaped_os(&raw))))?;
    if !crate::walk::host_absolute(dir) {
        return Err(Fault::refused(format!("{} must name a directory by absolute path, not '{}'", KNOB, dir)));
    }
    Ok(Some(PathBuf::from(dir)))
}

// spec: gate-sdk/SPEC.md §run-gates — a run is kept only where its scope is the registry, it judged
// (`pass` or `fail`), and its provenance carries all three key fields
pub fn entry(scope: Scope, status: i32, p: &Provenance) -> Option<(String, String)> {
    let verdict = match status {
        0 => "pass",
        1 => "fail",
        _ => return None,
    };
    if scope != Scope::Registry {
        return None;
    }
    let tree = p.tree_id.as_deref().filter(|_| p.clean)?;
    let name = file_name(tree, p.tool_version.as_deref()?, p.gate_set.as_deref()?);
    let mut record = Map::new();
    record.insert("contract".into(), CONTRACT.into());
    record.insert("provenance".into(), p.block());
    record.insert("verdict".into(), verdict.into());
    Some((name, Value::Object(record).to_string()))
}

// spec: gate-sdk/SPEC.md §run-gates — a temporary file in the store renamed over the key, so a
// reader never sees a partial record
pub fn write(dir: &Path, name: &str, body: &str) -> Result<(), String> {
    let fail = |e: std::io::Error| format!("cannot write the verdict store {}: {}", dir.display(), e);
    std::fs::create_dir_all(dir).map_err(fail)?;
    let tmp = dir.join(format!(".{}.{}.tmp", name, std::process::id()));
    std::fs::write(&tmp, body).map_err(fail)?;
    std::fs::rename(&tmp, dir.join(name)).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        fail(e)
    })
}

fn version_token(v: &str) -> bool {
    !v.is_empty() && v.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'+' | b'-'))
}

fn operands(args: &[String]) -> Result<(&str, &str, &str), Fault> {
    let usage = "usage: --verdict-lookup <tree> <kit-ver> <digest>";
    let [tree, ver, set] = args else {
        return Err(Fault::usage(format!("takes three operands, got {}\n{}", args.len(), usage)));
    };
    if !crate::provenance::object_id(tree) {
        return Err(Fault::usage(format!("<tree> is not a tree object id: '{}'\n{}", tree, usage)));
    }
    if !version_token(ver) {
        return Err(Fault::usage(format!("<kit-ver> is not a release stamp: '{}'\n{}", ver, usage)));
    }
    if !crate::evidence::is_hex64(set) {
        return Err(Fault::usage(format!("<digest> is not a gate-set digest: '{}'\n{}", set, usage)));
    }
    Ok((tree, ver, set))
}

// spec: gate-sdk/SPEC.md §run-gates — no record is `absent`; a record that does not answer for the
// key it is filed under is an input error, never an answer
fn answer(dir: &Path, tree: &str, ver: &str, set: &str) -> Result<&'static str, Fault> {
    let path = dir.join(file_name(tree, ver, set));
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok("absent"),
        Err(e) => return Err(Fault::input(format!("cannot read {}: {}", path.display(), e))),
    };
    let bad = |why: &str| Fault::input(format!("{} {}", path.display(), why));
    let record: Value = serde_json::from_str(&text).map_err(|_| bad("does not parse as JSON"))?;
    let major = |c: &str| c.split('.').next().map(str::to_string);
    if record["contract"].as_str().and_then(major) != major(CONTRACT) {
        return Err(bad("carries a contract major this binary does not read"));
    }
    let p = &record["provenance"];
    if p["treeId"] != tree || p["tool"]["version"] != ver || p["gateSet"] != set {
        return Err(bad("is a record of another key"));
    }
    match record["verdict"].as_str() {
        Some("pass") => Ok("pass"),
        Some("fail") => Ok("fail"),
        _ => Err(bad("carries no pass or fail verdict")),
    }
}

fn document(verdict: &str) -> String {
    let mut doc = Map::new();
    doc.insert("contract".into(), CONTRACT.into());
    doc.insert("verdict".into(), verdict.into());
    format!("{}\n", Value::Object(doc))
}

fn lookup_with(args: &[String], store: Result<Option<PathBuf>, Fault>) -> Result<&'static str, Fault> {
    let (tree, ver, set) = operands(args)?;
    let dir = store?.ok_or_else(|| Fault::refused(format!("{} is unset: no store to read", KNOB)))?;
    answer(&dir, tree, ver, set)
}

// spec: gate-sdk/SPEC.md §run-gates — a machine caller's arm: its document on every path, never
// exit 1, since a stored `fail` is an answer
pub fn lookup(args: &[String]) -> i32 {
    match lookup_with(args, configured()) {
        Ok(v) => {
            print!("{}", document(v));
            0
        }
        Err(e) => {
            eprintln!("{}: {}", TAG, e);
            print!("{}", exit::error_document(CONTRACT, e.kind));
            e.status()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exit::Kind;
    use crate::sarif::tests::fixed;

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("cw-verdict-store-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    fn key(p: &Provenance) -> Vec<String> {
        [p.tree_id.clone(), p.tool_version.clone(), p.gate_set.clone()].into_iter().map(Option::unwrap).collect()
    }

    // spec: gate-sdk/SPEC.md §run-gates — only a judged registry run carrying all three key fields
    // is kept, and the record is the contract, the verdict and the provenance block whole
    #[test]
    fn only_a_judged_registry_run_with_a_full_key_is_kept() {
        let p = fixed();
        let (name, body) = entry(Scope::Registry, 1, &p).expect("a registry fail is kept");
        let k = key(&p);
        assert_eq!(name, file_name(&k[0], &k[1], &k[2]));
        let record: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(record["contract"], CONTRACT);
        assert_eq!(record["verdict"], "fail");
        assert_eq!(record["provenance"], p.block());
        assert!(!body.contains([' ', '\n']), "no insignificant whitespace: {}", body);
        assert_eq!(entry(Scope::Registry, 0, &p).map(|(_, b)| b.contains("\"verdict\":\"pass\"")), Some(true));
        assert!(entry(Scope::Registry, exit::TOOL, &p).is_none(), "an error run judged nothing");
        assert!(entry(Scope::Only, 0, &p).is_none(), "a selection answers for part of the set");
        assert!(entry(Scope::For, 1, &p).is_none(), "a selection answers for part of the set");
        let mut modified = fixed();
        modified.clean = false;
        assert!(entry(Scope::Registry, 0, &modified).is_none(), "a modified worktree has no tree to key");
        let mut unstamped = fixed();
        unstamped.tool_version = None;
        assert!(entry(Scope::Registry, 0, &unstamped).is_none(), "an unstamped binary keeps nothing");
        let mut setless = fixed();
        setless.gate_set = None;
        assert!(entry(Scope::Registry, 0, &setless).is_none(), "a set it could not digest keys nothing");
    }

    // spec: gate-sdk/SPEC.md §run-gates — the key is the digest of the three fields under its tag line
    #[test]
    fn the_file_name_is_the_digest_of_the_tagged_key() {
        let want = crate::sha256::hex(b"verdict-key v1\nt\n1.0.0\ns\n");
        assert_eq!(file_name("t", "1.0.0", "s"), format!("{}.json", want));
        assert_ne!(file_name("t", "1.0.0", "s"), file_name("t", "1.0.1", "s"));
    }

    // spec: gate-sdk/SPEC.md §run-gates — the arm's table: an answer at 0, a malformed operand at 2
    // `usage`, no absolute store at 2 `refused`, a record that cannot answer at 3 `input`
    #[test]
    fn the_lookup_answers_by_its_status_table() {
        let p = fixed();
        let k = key(&p);
        let dir = scratch("table");
        let ok = Ok(Some(dir.clone()));
        assert_eq!(lookup_with(&k, ok.clone()).unwrap(), "absent", "no store directory yet");
        let (name, body) = entry(Scope::Registry, 1, &p).unwrap();
        write(&dir, &name, &body).unwrap();
        assert_eq!(lookup_with(&k, ok.clone()).unwrap(), "fail", "a stored fail is an answer");
        let (name, body) = entry(Scope::Registry, 0, &p).unwrap();
        write(&dir, &name, &body).unwrap();
        assert_eq!(lookup_with(&k, ok.clone()).unwrap(), "pass", "a rerun replaces its key");
        let mut other = k.clone();
        other[2] = "1".repeat(64);
        assert_eq!(lookup_with(&other, ok.clone()).unwrap(), "absent");
        let temp = dir.join(format!(".{}.{}.tmp", name, std::process::id()));
        assert!(!temp.exists(), "no temporary file survives: {}", temp.display());

        let kind = |args: &[String], store| lookup_with(args, store).unwrap_err().kind;
        assert_eq!(kind(&k[..2], ok.clone()), Kind::Usage);
        for (i, bad) in [(0, "abc"), (1, "1.0 0"), (1, ""), (2, "ABC")] {
            let mut a = k.clone();
            a[i] = bad.to_string();
            assert_eq!(kind(&a, ok.clone()), Kind::Usage, "{:?}", bad);
        }
        assert_eq!(kind(&k, Ok(None)), Kind::Refused);
        assert_eq!(kind(&k, Err(Fault::refused("relative"))), Kind::Refused);

        let path = dir.join(file_name(&k[0], &k[1], &k[2]));
        for (bad, why) in [
            ("not json".to_string(), "unparseable"),
            (body.replace("\"contract\":\"1.0.0\"", "\"contract\":\"2.0.0\""), "another major"),
            (body.replace(&k[0], &"e".repeat(40)), "another key"),
            (body.replace("\"verdict\":\"pass\"", "\"verdict\":\"error\""), "no judged verdict"),
        ] {
            std::fs::write(&path, bad).unwrap();
            assert_eq!(kind(&k, ok.clone()), Kind::Input, "{}", why);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: gate-sdk/SPEC.md §run-gates — the goldens are the renderer's bytes on a found record, no
    // record and an unreadable one
    #[test]
    fn the_goldens_are_the_documents() {
        let p = fixed();
        let k = key(&p);
        let dir = scratch("golden");
        let ok = Ok(Some(dir.clone()));
        let absent = document(lookup_with(&k, ok.clone()).unwrap());
        let (name, body) = entry(Scope::Registry, 0, &p).unwrap();
        write(&dir, &name, &body).unwrap();
        let pass = document(lookup_with(&k, ok.clone()).unwrap());
        std::fs::remove_file(dir.join(&name)).unwrap();
        std::fs::create_dir(dir.join(&name)).unwrap();
        let err = lookup_with(&k, ok).unwrap_err();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(err.status(), exit::INPUT);
        assert_eq!(pass, include_str!("../../docs/contracts/verdict-lookup/v1/pass.json"));
        assert_eq!(absent, include_str!("../../docs/contracts/verdict-lookup/v1/absent.json"));
        assert_eq!(
            exit::error_document(CONTRACT, err.kind),
            include_str!("../../docs/contracts/verdict-lookup/v1/error.json")
        );
    }
}
