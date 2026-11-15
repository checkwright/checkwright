// spec: gate-sdk/SPEC.md §run-gates — the provenance block's one producer: which binary and which
// kits produced a verdict, and on what tree; every field the run cannot fill is absent
use crate::installer::lock;
use crate::{proc, programs};
use serde_json::{json, Map, Value};
use std::path::Path;

pub const TOOL_NAME: &str = "checkwright-gates";

pub struct Kit {
    pub name: String,
    pub root: String,
    pub version: Option<String>,
}

pub struct Provenance {
    pub tool_version: Option<String>,
    pub exact: bool,
    pub source_stamp: Option<String>,
    pub target: Option<String>,
    pub kits: Vec<Kit>,
    pub commit: Option<String>,
    pub clean: bool,
}

fn present(s: &str) -> Option<String> {
    Some(s.to_string()).filter(|s| !s.is_empty())
}

impl Provenance {
    // spec: gate-sdk/SPEC.md §run-gates — the block's wire shape, keys sorted by the map itself
    pub fn block(&self) -> Value {
        let mut tool = Map::new();
        tool.insert("name".into(), json!(TOOL_NAME));
        let mut put = |k: &str, v: &Option<String>| {
            if let Some(v) = v {
                tool.insert(k.into(), json!(v));
            }
        };
        put("version", &self.tool_version);
        put("sourceStamp", &self.source_stamp);
        put("target", &self.target);
        let kits: Vec<Value> = self
            .kits
            .iter()
            .map(|k| {
                let mut o = Map::new();
                o.insert("name".into(), json!(k.name));
                o.insert("root".into(), json!(k.root));
                if let Some(v) = &k.version {
                    o.insert("version".into(), json!(v));
                }
                Value::Object(o)
            })
            .collect();
        let mut block = Map::new();
        block.insert("tool".into(), Value::Object(tool));
        block.insert("kits".into(), json!(kits));
        if let Some(c) = &self.commit {
            block.insert("commit".into(), json!(c));
            block.insert("worktree".into(), json!(if self.clean { "clean" } else { "modified" }));
        }
        Value::Object(block)
    }

    // spec: gate-sdk/SPEC.md §run-gates — the native `semanticVersion` mirror: an exact release
    // stamp that is a Semantic Versioning 2.0 string, else nothing
    pub fn semantic_version(&self) -> Option<&str> {
        let v = self.tool_version.as_deref()?;
        (self.exact && semver(v)).then_some(v)
    }
}

// spec: gate-sdk/SPEC.md §run-gates — Semantic Versioning 2.0's grammar: three numeric cores with
// no leading zero, an optional dot-separated pre-release and an optional build suffix
pub fn semver(v: &str) -> bool {
    let (rest, build) = match v.split_once('+') {
        Some((r, b)) => (r, Some(b)),
        None => (v, None),
    };
    let (core, pre) = match rest.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (rest, None),
    };
    let ident = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-');
    let numeric = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) && (s == "0" || !s.starts_with('0'));
    let nums: Vec<&str> = core.split('.').collect();
    nums.len() == 3
        && nums.iter().all(|n| numeric(n))
        && pre.map_or(true, |p| {
            p.split('.')
                .all(|i| ident(i) && (!i.bytes().all(|b| b.is_ascii_digit()) || numeric(i)))
        })
        && build.map_or(true, |b| b.split('.').all(ident))
}

fn git(args: &[&str]) -> Option<String> {
    let out = proc::run(&programs::GIT, args).ok()?;
    Some(String::from_utf8_lossy(out.stdout()?).trim().to_string())
}

// spec: gate-sdk/SPEC.md §run-gates — the block's commit read: `HEAD` as an object id, else nothing
pub fn head() -> Option<String> {
    git(&["rev-parse", "--verify", "--quiet", "HEAD"]).filter(|c| object_id(c))
}

// spec: gate-sdk/SPEC.md §run-gates — the block's worktree read, tracked files only; `None` where
// git could not answer
pub fn worktree_clean() -> Option<bool> {
    crate::listing::names(None, &["status", "--porcelain", "-z", "--untracked-files=no"])
        .ok()
        .map(|changed| changed.is_empty())
}

pub fn object_id(s: &str) -> bool {
    matches!(s.len(), 40 | 64) && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

// spec: gate-sdk/SPEC.md §run-gates — a kit's name is its root's last path component, in either
// separator, so the block's `kits[].name` and a rule's `properties.kit` agree
pub fn kit_name(root: &str) -> String {
    root.trim_end_matches(['/', '\\']).rsplit(['/', '\\']).next().unwrap_or_default().to_string()
}

// spec: gate-sdk/SPEC.md §run-gates — the block for the tree at the working directory, the kit
// roots handed in as the caller spelled them relative to it, in `GATE_SDK_KIT_DIRS` order
pub fn collect(kit_roots: &[String]) -> Provenance {
    let manifest = lock::Manifest::read(&lock::path(Path::new("."))).filter(lock::Manifest::schema_ok);
    let (release, vendored) = match &manifest {
        Some(m) => (m.field("version"), m.field("kits")),
        None => (String::new(), String::new()),
    };
    let kits = kit_roots
        .iter()
        .map(|r| {
            let root = r.replace('\\', "/").trim_end_matches('/').to_string();
            let name = kit_name(&root);
            let named = vendored.split_whitespace().any(|k| k == name);
            Kit {
                version: present(&release).filter(|_| named),
                name,
                root,
            }
        })
        .collect();
    let commit = head();
    let clean = commit.is_some() && worktree_clean() == Some(true);
    Provenance {
        tool_version: present(env!("CHECKWRIGHT_RELEASE")),
        exact: !env!("CHECKWRIGHT_RELEASE_EXACT").is_empty(),
        source_stamp: present(env!("CHECKWRIGHT_SOURCE_STAMP")),
        target: present(env!("CHECKWRIGHT_TARGET")),
        kits,
        commit,
        clean,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bare() -> Provenance {
        Provenance {
            tool_version: None,
            exact: false,
            source_stamp: None,
            target: None,
            kits: vec![Kit {
                name: "gate-sdk".into(),
                root: "gate-sdk".into(),
                version: None,
            }],
            commit: None,
            clean: false,
        }
    }

    // spec: gate-sdk/SPEC.md §run-gates — a field the run cannot fill is absent, never null or blank
    #[test]
    fn every_optional_field_is_absent_when_unfilled() {
        let b = bare().block();
        assert_eq!(b["tool"], json!({ "name": TOOL_NAME }));
        assert_eq!(b["kits"], json!([{ "name": "gate-sdk", "root": "gate-sdk" }]));
        assert!(b.get("commit").is_none());
        assert!(b.get("worktree").is_none());
    }

    // spec: gate-sdk/SPEC.md §run-gates — a SHA-256 object id is a commit, and `worktree` rides it
    #[test]
    fn a_sha256_commit_is_carried_with_its_worktree_state() {
        let id = "a".repeat(64);
        assert!(object_id(&id));
        assert!(object_id(&"0".repeat(40)));
        assert!(!object_id(&"A".repeat(40)));
        assert!(!object_id(&"a".repeat(41)));
        let mut p = bare();
        p.commit = Some(id.clone());
        p.clean = true;
        let b = p.block();
        assert_eq!(b["commit"], json!(id));
        assert_eq!(b["worktree"], "clean");
    }

    // spec: gate-sdk/SPEC.md §run-gates — `semanticVersion` only for an exact, valid release stamp
    #[test]
    fn the_semantic_version_needs_an_exact_valid_stamp() {
        let mut p = bare();
        p.tool_version = Some("0.36.0".into());
        p.exact = true;
        assert_eq!(p.semantic_version(), Some("0.36.0"));
        p.tool_version = Some("0.36.0-6-gabc".into());
        p.exact = false;
        assert_eq!(p.semantic_version(), None);
        p.exact = true;
        p.tool_version = Some("0.36".into());
        assert_eq!(p.semantic_version(), None);
    }

    // spec: gate-sdk/SPEC.md §run-gates — the Semantic Versioning 2.0 grammar the mirror holds to
    #[test]
    fn the_semver_grammar_admits_only_valid_strings() {
        for ok in ["0.36.0", "1.0.0-rc.1", "1.0.0-0.3.7", "1.0.0+20130313144700", "1.0.0-alpha+001"] {
            assert!(semver(ok), "{}", ok);
        }
        for bad in ["0.36", "01.0.0", "1.0.0-", "1.0.0-01", "1.0.0+", "v1.0.0", "1.0.0-a..b"] {
            assert!(!semver(bad), "{}", bad);
        }
    }
}
