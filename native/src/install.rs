// spec: installer/README.md §The install boundary — the `--install <op>` arm family: the seam
// both bootstraps call, so the bash caller and its PowerShell twin issue byte-identical argv.
// spec: gate-sdk/SPEC.md §The non-gate arm — the class's first deliberately unbridged member.
use crate::{proc, sha256};
use serde_json::Value;
use std::path::{Path, PathBuf};

const USAGE: &str = "  usage: checkwright-gates --install place-artifact --root <dir> --src <file> --dest <path> --seam <path> --target <triple> --digest <sha256> [--lock <path>] [--force] [--dry-run]";

// spec: installer/README.md §The install boundary — the closed op set an unknown `<op>` is refused
// against, so a caller's typo exits 2 rather than reading as a step that did nothing.
const OPS: &[&str] = &["place-artifact"];

fn usage_error(what: &str) -> i32 {
    eprintln!(
        "checkwright-gates: --install {} — the install step could not run; nothing was written",
        what
    );
    eprintln!("{}", USAGE);
    2
}

// spec: installer/README.md §The install boundary — `--install <op> [--<key> <value>]…`, each op
// declaring its value keys and its bare flags; an unknown key exits 2 rather than defaulting.
struct Argv {
    values: Vec<(String, String)>,
    flags: Vec<String>,
}

fn parse(args: &[String], value_keys: &[&str], flag_keys: &[&str]) -> Result<Argv, String> {
    let mut out = Argv {
        values: Vec::new(),
        flags: Vec::new(),
    };
    let mut i = 0;
    while i < args.len() {
        let key = match args[i].strip_prefix("--") {
            Some(k) if !k.is_empty() => k,
            _ => return Err(format!("expected a --<key>, got '{}'", args[i])),
        };
        if flag_keys.contains(&key) {
            out.flags.push(key.to_string());
            i += 1;
        } else if value_keys.contains(&key) {
            let v = args
                .get(i + 1)
                .ok_or_else(|| format!("--{} needs a value", key))?;
            out.values.push((key.to_string(), v.clone()));
            i += 2;
        } else {
            return Err(format!("unknown key --{}", key));
        }
    }
    Ok(out)
}

impl Argv {
    fn get(&self, key: &str) -> Option<&str> {
        self.values
            .iter()
            .rev()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    fn required(&self, key: &str) -> Result<String, String> {
        match self.get(key) {
            Some(v) if !v.is_empty() => Ok(v.to_string()),
            _ => Err(format!("--{} is required and must not be empty", key)),
        }
    }

    fn set(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }
}

// spec: installer/README.md §The install boundary — the manifest a previous run left: the hash
// recorded against a path about to be claimed, and the target and digest recorded against the
// artifact. Absent on a first install, which is why `--lock` is optional.
struct Recorded {
    files: Value,
    target: String,
    digest: String,
}

impl Recorded {
    fn none() -> Self {
        Recorded {
            files: Value::Null,
            target: String::new(),
            digest: String::new(),
        }
    }

    fn read(path: &Path) -> Result<Self, String> {
        if !path.is_file() {
            return Ok(Recorded::none());
        }
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read {}: {}", path.display(), e))?;
        let doc: Value = serde_json::from_str(&text)
            .map_err(|e| format!("cannot parse {}: {}", path.display(), e))?;
        let str_at = |a: &str, b: &str| {
            doc.get(a)
                .and_then(|v| v.get(b))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        };
        Ok(Recorded {
            files: doc.get("files").cloned().unwrap_or(Value::Null),
            target: str_at("artifact", "target"),
            digest: str_at("artifact", "digest"),
        })
    }

    fn hash_of(&self, rel: &str) -> String {
        self.files
            .get(rel)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string()
    }
}

// spec: installer/README.md §The install boundary — installer/lib/common/lock.sh's `lock_hash`,
// spawned rather than reimplemented: the manifest records git's object hash, and a second
// computation of it would be a second identity for the caller and the op to disagree about.
fn lock_hash(file: &Path) -> Result<String, String> {
    let path = file.to_string_lossy().into_owned();
    let out = proc::run("git", &["hash-object", "--", &path])?;
    let text = out
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())
        .ok_or_else(|| format!("git hash-object could not hash {}", path))?;
    Ok(text.trim().to_string())
}

// spec: installer/README.md §The install boundary — the non-destructive re-run, which is the
// caller's `claim` moved behind the invoke: a file whose recorded hash still matches is the
// installer's to rewrite, one that has changed since is the adopter's and is kept.
enum Claim {
    Take,
    Kept(String),
}

fn claim(root: &Path, rel: &str, recorded: &Recorded, force: bool) -> Result<Claim, String> {
    let want = recorded.hash_of(rel);
    if want.is_empty() {
        return Ok(Claim::Take);
    }
    let file = root.join(rel);
    if !file.is_file() {
        return Ok(Claim::Take);
    }
    if lock_hash(&file)? == want || force {
        return Ok(Claim::Take);
    }
    Ok(Claim::Kept(want))
}

#[cfg(unix)]
pub fn make_executable(file: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(file)
        .map_err(|e| format!("cannot stat {}: {}", file.display(), e))?
        .permissions();
    perms.set_mode(perms.mode() | 0o111);
    std::fs::set_permissions(file, perms)
        .map_err(|e| format!("cannot set the executable bit on {}: {}", file.display(), e))
}

// spec: installer/README.md §The install boundary — the executable bit is set where the platform
// has one, so the Windows half of the boundary needs no branch of its own in either bootstrap.
#[cfg(not(unix))]
pub fn make_executable(_file: &Path) -> Result<(), String> {
    Ok(())
}

// spec: installer/README.md §The gate binary — the seam is rewritten preserving every line except
// the knob this op owns, and the two shellcheck directives are seeded only when the file is
// absent: a consumer's own check-shellcheck reds on a sourced file written without them.
fn seam_text(existing: Option<&str>, dest: &str) -> String {
    let mut out = String::new();
    match existing {
        None => {
            out.push_str("# shellcheck shell=bash\n");
            out.push_str(
                "# shellcheck disable=SC2034  # consumed by gate-sdk/lib/gate.sh after sourcing\n",
            );
        }
        Some(text) => {
            for line in text.lines() {
                if line.starts_with("GATE_SDK_NATIVE_BIN=") {
                    continue;
                }
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    out.push_str("GATE_SDK_NATIVE_BIN=");
    out.push_str(dest);
    out.push('\n');
    out
}

// spec: installer/README.md §The install boundary — the seam write is a temporary beside the
// target and a rename, so no reader sees a half-written sourced file and a failed write leaves
// whatever was there intact.
fn write_atomically(file: &Path, body: &str) -> Result<(), String> {
    let mut tmp: PathBuf = file.to_path_buf();
    let mut name = file
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    name.push_str(".tmp");
    tmp.set_file_name(name);
    std::fs::write(&tmp, body).map_err(|e| format!("cannot write {}: {}", tmp.display(), e))?;
    std::fs::rename(&tmp, file).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("cannot write {}: {}", file.display(), e)
    })
}

// spec: installer/README.md §The install boundary — the op's argv after resolution, carried as one
// value so the two paths a placement takes read against the same resolved inputs.
struct Placement<'a> {
    root: PathBuf,
    src: &'a str,
    dest: &'a str,
    seam: &'a str,
    target: &'a str,
    digest: &'a str,
    force: bool,
    dry: bool,
}

// spec: installer/README.md §The install boundary — the two stdout verbs, each with one reader in
// the caller: `own` is a path it records and stages, `kept` a path it leaves alone and carries
// forward at the hash the manifest already holds.
fn place(p: &Placement, recorded: &Recorded) -> Result<Vec<String>, String> {
    let mut records = Vec::new();
    let dest_path = p.root.join(p.dest);

    // spec: installer/README.md §The gate binary — the artifact path is exempt from the ownership
    // rule the seam below still runs, so a substituted binary is rewritten rather than kept and
    // the remedy §doctor prints is one a bare re-run performs.
    if !p.dry {
        // spec: installer/README.md §The manifest — an on-disk artifact that still verifies
        // against the recorded digest is not rewritten, which is what makes a bare re-run
        // leave the tree byte-identical.
        let stale = recorded.target != p.target
            || recorded.digest != p.digest
            || !dest_path.is_file()
            || sha256::file_hex(&dest_path)? != p.digest;
        if stale {
            std::fs::copy(p.src, &dest_path)
                .map_err(|e| format!("could not write {}: {}", p.dest, e))?;
            make_executable(&dest_path)?;
        }
    }
    records.push(format!("own\t{}", p.dest));

    match claim(&p.root, p.seam, recorded, p.force)? {
        Claim::Kept(h) => records.push(format!("kept\t{}\t{}", p.seam, h)),
        Claim::Take => {
            if !p.dry {
                let seam_path = p.root.join(p.seam);
                let existing = if seam_path.is_file() {
                    Some(
                        std::fs::read_to_string(&seam_path)
                            .map_err(|e| format!("cannot read {}: {}", p.seam, e))?,
                    )
                } else {
                    None
                };
                write_atomically(&seam_path, &seam_text(existing.as_deref(), p.dest))?;
            }
            records.push(format!("own\t{}", p.seam));
        }
    }

    Ok(records)
}

fn place_artifact(args: &[String]) -> i32 {
    let parsed = match parse(
        args,
        &["root", "src", "dest", "seam", "target", "digest", "lock"],
        &["force", "dry-run"],
    ) {
        Ok(a) => a,
        Err(e) => return usage_error(&format!("place-artifact: {}", e)),
    };
    let resolved: Result<Vec<String>, String> = ["root", "src", "dest", "seam", "target", "digest"]
        .iter()
        .map(|k| parsed.required(k))
        .collect();
    let resolved = match resolved {
        Ok(v) => v,
        Err(e) => return usage_error(&format!("place-artifact: {}", e)),
    };
    let placement = Placement {
        root: PathBuf::from(&resolved[0]),
        src: &resolved[1],
        dest: &resolved[2],
        seam: &resolved[3],
        target: &resolved[4],
        digest: &resolved[5],
        force: parsed.set("force"),
        dry: parsed.set("dry-run"),
    };

    let recorded = match parsed.get("lock") {
        Some(rel) if !rel.is_empty() => Recorded::read(&placement.root.join(rel)),
        _ => Ok(Recorded::none()),
    };
    let outcome = recorded.and_then(|r| place(&placement, &r));
    match outcome {
        Ok(records) => {
            for r in records {
                println!("{}", r);
            }
            0
        }
        Err(e) => {
            eprintln!("checkwright-gates: --install place-artifact: {}", e);
            2
        }
    }
}

// spec: installer/README.md §The install boundary — the family's entry point, resolved in `main`
// before the registry lookup, and the family's exit statuses: 0 performed or planned, 1 an
// adopter-actionable refusal, 2 usage or harness error.
pub fn run(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("place-artifact") => place_artifact(&args[1..]),
        Some(op) => usage_error(&format!(
            "unknown op '{}' — this binary carries: {}",
            op,
            OPS.join(", ")
        )),
        None => usage_error(&format!(
            "needs an op — this binary carries: {}",
            OPS.join(", ")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recorded(files: &str, target: &str, digest: &str) -> Recorded {
        Recorded {
            files: serde_json::from_str(files).expect("bad fixture"),
            target: target.to_string(),
            digest: digest.to_string(),
        }
    }

    fn scratch(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "checkwright-install-{}.{}",
            label,
            std::process::id()
        ));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(dir.join("scripts")).expect("cannot make the scratch tree");
        dir
    }

    fn placement<'a>(root: &Path, src: &'a str, digest: &'a str, dry: bool) -> Placement<'a> {
        Placement {
            root: root.to_path_buf(),
            src,
            dest: "scripts/checkwright-gates",
            seam: "scripts/gate-sdk-config.sh",
            target: "x86_64-unknown-linux-gnu",
            digest,
            force: false,
            dry,
        }
    }

    // spec: installer/README.md §The install boundary — an unknown key exits 2 rather than being
    // ignored, which is the property that keeps the two bootstraps' argv equivalent.
    #[test]
    fn an_unknown_key_is_refused_and_a_declared_flag_takes_no_value() {
        assert!(parse(
            &["--nope".to_string(), "x".to_string()],
            &["root"],
            &["force"]
        )
        .is_err());
        let a = parse(
            &["--force".to_string(), "--root".to_string(), "/r".to_string()],
            &["root"],
            &["force"],
        )
        .expect("the declared flag and key were refused");
        assert!(a.set("force"));
        assert_eq!(a.get("root"), Some("/r"));
        assert!(parse(&["--root".to_string()], &["root"], &[]).is_err());
        assert_eq!(run(&["no-such-op".to_string()]), 2);
        assert_eq!(run(&[]), 2);
    }

    // spec: installer/README.md §The gate binary — the seam rewrite preserves every line except
    // the knob this op owns, and seeds the two directives only when the file is absent.
    #[test]
    fn the_seam_rewrite_keeps_every_other_line_and_seeds_only_when_absent() {
        let fresh = seam_text(None, "scripts/checkwright-gates");
        assert!(fresh.starts_with("# shellcheck shell=bash\n"));
        assert!(fresh.ends_with("GATE_SDK_NATIVE_BIN=scripts/checkwright-gates\n"));
        let existing = "# shellcheck shell=bash\nOTHER_KNOB=1\nGATE_SDK_NATIVE_BIN=stale\n";
        let rewritten = seam_text(Some(existing), "scripts/checkwright-gates");
        assert_eq!(
            rewritten,
            "# shellcheck shell=bash\nOTHER_KNOB=1\nGATE_SDK_NATIVE_BIN=scripts/checkwright-gates\n"
        );
        assert_eq!(rewritten.matches("GATE_SDK_NATIVE_BIN=").count(), 1);
        // comment-tier-exempt: a source file with no closing newline is a local property of the
        // input, not a rule either tier owns — `grep -v` terminated its last line and so must this
        assert_eq!(seam_text(Some("A=1"), "b"), "A=1\nGATE_SDK_NATIVE_BIN=b\n");
    }

    // spec: installer/README.md §The install boundary — claim's three ways to reach `Take`: no
    // recorded hash, no file on disk, and a file still at the hash the manifest records.
    #[test]
    fn an_unrecorded_or_unmoved_path_is_claimable_and_a_changed_one_is_kept() {
        let dir = scratch("claim");
        let rel = "scripts/seam.sh";
        std::fs::write(dir.join(rel), "A=1\n").expect("cannot write the scratch file");
        let hash = lock_hash(&dir.join(rel)).expect("git could not hash the scratch file");
        let moved = format!(r#"{{"{}":"{}"}}"#, rel, "0".repeat(40));

        assert!(matches!(
            claim(&dir, rel, &recorded("{}", "", ""), false),
            Ok(Claim::Take)
        ));
        assert!(matches!(
            claim(
                &dir,
                "scripts/absent.sh",
                &recorded(r#"{"scripts/absent.sh":"deadbeef"}"#, "", ""),
                false
            ),
            Ok(Claim::Take)
        ));
        assert!(matches!(
            claim(
                &dir,
                rel,
                &recorded(&format!(r#"{{"{}":"{}"}}"#, rel, hash), "", ""),
                false
            ),
            Ok(Claim::Take)
        ));
        assert!(matches!(
            claim(&dir, rel, &recorded(&moved, "", ""), false),
            Ok(Claim::Kept(_))
        ));
        assert!(
            matches!(
                claim(&dir, rel, &recorded(&moved, "", ""), true),
                Ok(Claim::Take)
            ),
            "--force must reclaim a path the adopter changed"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/README.md §The install boundary — the two wire verbs and the skip-rewrite
    // branch over a scratch tree: a first placement owns both paths, a bare re-run copies nothing,
    // a changed dest is kept at its hash, and `--dry-run` plans the same and writes none of it.
    #[test]
    fn a_placement_owns_both_paths_and_a_bare_re_run_rewrites_nothing() {
        let dir = scratch("place");
        let src = dir.join("payload-binary");
        std::fs::write(&src, "artifact bytes\n").expect("cannot write the scratch artifact");
        let digest = sha256::file_hex(&src).expect("cannot hash the scratch artifact");
        let src_s = src.to_string_lossy().into_owned();
        let (dest, seam) = ("scripts/checkwright-gates", "scripts/gate-sdk-config.sh");
        let owned = vec![format!("own\t{}", dest), format!("own\t{}", seam)];

        let first = place(
            &placement(&dir, &src_s, &digest, false),
            &Recorded::none(),
        )
        .expect("the first placement failed");
        assert_eq!(first, owned, "a first install owns both paths");
        assert_eq!(
            sha256::file_hex(&dir.join(dest)).expect("no artifact was placed"),
            digest
        );
        assert!(std::fs::read_to_string(dir.join(seam))
            .expect("no seam was written")
            .contains(&format!("GATE_SDK_NATIVE_BIN={}\n", dest)));

        // spec: installer/README.md §The manifest — an unreadable `--src` is what proves the
        // skip-rewrite branch was taken: a re-run that copied would fail on it rather than pass
        // for the same reason an idempotent one does.
        let files = format!(
            r#"{{"{}":"{}","{}":"{}"}}"#,
            dest,
            lock_hash(&dir.join(dest)).expect("cannot hash the placed artifact"),
            seam,
            lock_hash(&dir.join(seam)).expect("cannot hash the written seam"),
        );
        let again = place(
            &placement(&dir, "/checkwright-no-such-source", &digest, false),
            &recorded(&files, "x86_64-unknown-linux-gnu", &digest),
        )
        .expect("the bare re-run failed");
        assert_eq!(again, owned);

        // spec: installer/README.md §The gate binary — the substitution case, which is the one
        // §doctor reports as a digest mismatch: the artifact carries no adopter-authored version,
        // so a re-run rewrites it from the verified payload rather than reporting it kept.
        std::fs::write(dir.join(dest), "substituted bytes\n").expect("cannot substitute");
        let substituted = format!(r#"{{"{}":"{}"}}"#, dest, "0".repeat(40));
        let rewritten = place(
            &placement(&dir, &src_s, &digest, false),
            &recorded(&substituted, "x86_64-unknown-linux-gnu", &digest),
        )
        .expect("the substituted run failed");
        assert_eq!(rewritten[0], format!("own\t{}", dest));
        assert_eq!(
            sha256::file_hex(&dir.join(dest)).expect("the artifact went missing"),
            digest,
            "a substituted artifact must be rewritten from the verified payload"
        );

        std::fs::remove_file(dir.join(dest)).expect("cannot clear the placed artifact");
        let planned = place(&placement(&dir, &src_s, &digest, true), &Recorded::none())
            .expect("the dry run failed");
        assert_eq!(planned, owned);
        assert!(!dir.join(dest).exists(), "--dry-run wrote the artifact");
        std::fs::remove_dir_all(&dir).ok();
    }
}
