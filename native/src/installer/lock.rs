// spec: installer/README.md §The manifest — the crate's owner of the `checkwright.lock` schema:
// the wire key, the field accessors, and how a recorded content hash is obtained, so the arm that
// writes the manifest and the arms that read it share one definition instead of a copy each.
use crate::proc;
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};

// spec: installer/README.md §The manifest — the versioned wire key; a build refuses a schema it
// does not know rather than guessing at an unknown shape.
pub const SCHEMA: &str = "checkwright-lock v1";
pub const FILE: &str = "checkwright.lock";

// spec: installer/README.md §The manifest — the fields whose wire type is an array rather than a
// string, which is knowledge only the schema owner has: `emit` splits exactly these on the space
// the reader below joins them with, so one module holds both halves of the representation.
const ARRAY_FIELDS: &[&str] = &["kits"];

pub fn path(root: &Path) -> PathBuf {
    root.join(FILE)
}

// spec: installer/README.md §The manifest — git's object hash, never a SHA-256: macOS ships
// `shasum` and not `sha256sum`, and git is already a floor-contract member, so the manifest's
// change detection stays inside the toolchain the contract asserts.
pub fn hash(file: &Path) -> Result<String, String> {
    let p = file.to_string_lossy().into_owned();
    let out = proc::run("git", &["hash-object", "--", &p])?;
    let text = out
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())
        .ok_or_else(|| format!("git hash-object could not hash {}", p))?;
    Ok(text.trim().to_string())
}

pub struct Manifest {
    doc: Value,
}

impl Manifest {
    pub fn read(path: &Path) -> Option<Manifest> {
        let text = std::fs::read_to_string(path).ok()?;
        let doc: Value = serde_json::from_str(&text).ok()?;
        Some(Manifest { doc })
    }

    // spec: installer/README.md §The manifest — a build refuses a schema it does not know; an
    // unparseable file and a wrong key are one answer, because neither is a manifest this build
    // may act on.
    pub fn schema_ok(&self) -> bool {
        self.doc.get("schema").and_then(Value::as_str) == Some(SCHEMA)
    }

    // spec: installer/README.md §The manifest — arrays space-joined on read, the inverse of the
    // split `emit` makes on write, so a caller needs no second grammar to pass one.
    pub fn field(&self, name: &str) -> String {
        match self.doc.get(name) {
            None | Some(Value::Null) => String::new(),
            Some(Value::Array(items)) => items
                .iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .collect::<Vec<_>>()
                .join(" "),
            Some(Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
        }
    }

    // spec: installer/README.md §The manifest — the recorded roster, in the wire's own key order.
    pub fn files(&self) -> Vec<(String, String)> {
        match self.doc.get("files") {
            Some(Value::Object(map)) => map
                .iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
                .collect(),
            _ => Vec::new(),
        }
    }

    pub fn file_count(&self) -> usize {
        match self.doc.get("files") {
            Some(Value::Object(map)) => map.len(),
            _ => 0,
        }
    }

    // spec: installer/README.md §The manifest — resolve one of the consumer's *own* seam files by
    // exact key, which is what `files` holds. A tail match cannot: the vendored kits carry fixture
    // trees with their own `scripts/gates.list`, so a suffix picks whichever sorts first.
    pub fn own_file(&self, rel: &str) -> String {
        match self.doc.get("files") {
            Some(Value::Object(map)) if map.contains_key(rel) => rel.to_string(),
            _ => String::new(),
        }
    }

    pub fn artifact(&self) -> (String, String) {
        let at = |k: &str| {
            self.doc
                .get("artifact")
                .and_then(|v| v.get(k))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        };
        (at("target"), at("digest"))
    }
}

// spec: installer/README.md §The manifest — the single writer of the wire shape, so a second
// writing arm cannot drift from the first: keys sorted at every nesting level, and an identity
// field present exactly when the caller supplied it rather than as an empty placeholder.
pub struct Emit {
    idents: Vec<(String, String)>,
    artifact: Option<(String, String)>,
    files: Vec<(String, String)>,
}

impl Emit {
    pub fn new() -> Emit {
        Emit {
            idents: Vec::new(),
            artifact: None,
            files: Vec::new(),
        }
    }

    pub fn ident(mut self, key: &str, value: &str) -> Emit {
        self.idents.push((key.to_string(), value.to_string()));
        self
    }

    pub fn artifact(mut self, target: &str, digest: &str) -> Emit {
        self.artifact = Some((target.to_string(), digest.to_string()));
        self
    }

    pub fn file(mut self, path: &str, hash: &str) -> Emit {
        self.files.push((path.to_string(), hash.to_string()));
        self
    }

    pub fn render(&self) -> String {
        let mut doc = Map::new();
        doc.insert("schema".to_string(), Value::String(SCHEMA.to_string()));
        for (k, v) in &self.idents {
            let value = if ARRAY_FIELDS.contains(&k.as_str()) {
                Value::Array(
                    v.split_whitespace()
                        .map(|s| Value::String(s.to_string()))
                        .collect(),
                )
            } else {
                Value::String(v.clone())
            };
            doc.insert(k.clone(), value);
        }
        let mut files = Map::new();
        for (p, h) in &self.files {
            files.insert(p.clone(), Value::String(h.clone()));
        }
        doc.insert("files".to_string(), Value::Object(files));
        if let Some((target, digest)) = &self.artifact {
            let mut a = Map::new();
            a.insert("target".to_string(), Value::String(target.clone()));
            a.insert("digest".to_string(), Value::String(digest.clone()));
            doc.insert("artifact".to_string(), Value::Object(a));
        }
        // comment-tier-exempt: serde_json's map is a BTreeMap, so every nesting level is emitted in
        // key order without a sorting pass — the property the wire shape asserts, obtained rather
        // than imposed
        let mut text = serde_json::to_string_pretty(&Value::Object(doc)).unwrap_or_default();
        text.push('\n');
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/README.md §The manifest — the wire shape: the schema key, sorted keys at
    // every level, an array field split on the space the reader joins it with, and an identity
    // field present exactly when the caller supplied it.
    #[test]
    fn the_wire_shape_is_sorted_and_omits_what_the_caller_did_not_supply() {
        let text = Emit::new()
            .ident("version", "1.2.3")
            .ident("kits", "gate-sdk canon-kit")
            .file("b.txt", "hb")
            .file("a.txt", "ha")
            .render();
        assert!(text.ends_with("\n"));
        assert!(!text.contains("artifact"), "an unsupplied key was written");
        let doc: Value = serde_json::from_str(&text).expect("the emitted manifest is not JSON");
        assert_eq!(doc["schema"], Value::String(SCHEMA.to_string()));
        assert_eq!(doc["kits"][0], Value::String("gate-sdk".to_string()));
        assert!(
            text.find("\"a.txt\"").unwrap() < text.find("\"b.txt\"").unwrap(),
            "the files map is not in key order"
        );

        let with = Emit::new().artifact("t", "d").render();
        let doc: Value = serde_json::from_str(&with).expect("the emitted manifest is not JSON");
        assert_eq!(doc["artifact"]["target"], Value::String("t".to_string()));
    }

    // spec: installer/README.md §The manifest — the reader's inverse: an array joins on the space
    // the writer split, an absent field reads empty, and `own_file` is an exact key lookup rather
    // than a tail match.
    #[test]
    fn the_reader_inverts_the_writer_and_owns_by_exact_key() {
        let dir = std::env::temp_dir().join(format!("cw-lock-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot make the scratch tree");
        let p = path(&dir);
        std::fs::write(
            &p,
            Emit::new()
                .ident("kits", "gate-sdk canon-kit")
                .file("scripts/gates.list", "h")
                .render(),
        )
        .expect("cannot write the scratch manifest");
        let m = Manifest::read(&p).expect("the scratch manifest did not parse");
        assert!(m.schema_ok());
        assert_eq!(m.field("kits"), "gate-sdk canon-kit");
        assert_eq!(m.field("version"), "");
        assert_eq!(m.own_file("scripts/gates.list"), "scripts/gates.list");
        assert_eq!(m.own_file("gates.list"), "");
        assert_eq!(m.file_count(), 1);
        std::fs::write(&p, "{\"schema\":\"other\"}").expect("cannot rewrite the scratch manifest");
        assert!(!Manifest::read(&p).expect("did not parse").schema_ok());
        std::fs::remove_dir_all(&dir).ok();
    }
}
