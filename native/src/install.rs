// spec: installer/SPEC.md §The install boundary — the `--install <op>` arm family: the seam
// both bootstraps call, so the bash caller and its PowerShell twin issue byte-identical argv.
use crate::sha256;
// spec: installer/SPEC.md §The manifest — the recorded hash has one owner, the schema module's
// own `hash`, so this op and the `--init` arm that shares its claim rule cannot disagree about
// which identity a `files` entry carries.
use crate::installer::lock::hash as lock_hash;
use serde_json::Value;
use std::path::{Path, PathBuf};

const USAGE: &str = "  usage: checkwright-gates --install place-artifact --root <dir> --src <file> --dest <path> --seam <path> --target <triple> --digest <sha256> [--lock <path>] [--kits <kit>[ <kit>…]] [--spec-base-url <url>] [--force] [--dry-run]
         checkwright-gates --install queue-source --payload <dir> --kits <kit>[,<kit>…]";

// spec: installer/SPEC.md §The install boundary — the closed op set an unknown `<op>` is refused
// against, so a caller's typo exits 2 rather than reading as a step that did nothing.
const OPS: &[&str] = &["place-artifact", "queue-source"];

fn usage_error(what: &str) -> i32 {
    eprintln!(
        "checkwright-gates: --install {} — the install step could not run; nothing was written",
        what
    );
    eprintln!("{}", USAGE);
    2
}

// spec: installer/SPEC.md §The install boundary — `--install <op> [--<key> <value>]…`, each op
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

// spec: installer/SPEC.md §The install boundary — the manifest a previous run left: the hash
// recorded against a path about to be claimed, and the target and digest recorded against the
// artifact. Absent on a first install, which is why `--lock` is optional.
pub struct Recorded {
    files: Value,
    target: String,
    digest: String,
}

impl Recorded {
    pub fn none() -> Self {
        Recorded {
            files: Value::Null,
            target: String::new(),
            digest: String::new(),
        }
    }

    pub fn read(path: &Path) -> Result<Self, String> {
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

// spec: installer/SPEC.md §The install boundary — the non-destructive re-run, which is the
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

// spec: installer/SPEC.md §The install boundary — the executable bit is set where the platform
// has one, so the Windows half of the boundary needs no branch of its own in either bootstrap.
#[cfg(not(unix))]
pub fn make_executable(_file: &Path) -> Result<(), String> {
    Ok(())
}

// spec: installer/SPEC.md §The gate binary — the seam is a knob file rewritten preserving every line
// except those whose head is a knob this op owns: the artifact path, plus whatever the caller
// declares
fn seam_text(existing: Option<&str>, dest: &str, declared: &[(String, String)]) -> String {
    let owned = |head: &str| {
        head == "GATE_SDK_NATIVE_BIN" || declared.iter().any(|(k, _)| k == head)
    };
    let mut out = String::new();
    for line in existing.unwrap_or_default().lines() {
        if line.split_once('=').is_some_and(|(head, _)| owned(head.trim())) {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out.push_str("GATE_SDK_NATIVE_BIN = ");
    out.push_str(dest);
    out.push('\n');
    for (name, value) in declared {
        out.push_str(name);
        out.push_str(" = ");
        out.push_str(value);
        out.push('\n');
    }
    out
}

// spec: installer/SPEC.md §The install boundary — the seam write is a temporary beside the
// target and a rename, so no reader sees a half-written knob file and a failed write leaves
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

// spec: installer/SPEC.md §The install boundary — the op's argv after resolution, carried as one
// value so the two paths a placement takes read against the same resolved inputs.
pub struct Placement<'a> {
    pub root: PathBuf,
    pub src: &'a str,
    pub dest: &'a str,
    pub seam: &'a str,
    // spec: installer/SPEC.md §The gate binary — the seam's declared lines: knob/value pairs the
    // install resolves rather than ships, supplied by the caller because the values are its own
    pub declared: &'a [(String, String)],
    pub target: &'a str,
    pub digest: &'a str,
    pub force: bool,
    pub dry: bool,
}

// spec: installer/SPEC.md §The install boundary — the two stdout verbs, each with one reader in
// the caller: `own` is a path it records and stages, `kept` a path it leaves alone and carries
// forward at the hash the manifest already holds.
pub fn place(p: &Placement, recorded: &Recorded) -> Result<Vec<String>, String> {
    let mut records = Vec::new();
    let dest_path = p.root.join(p.dest);

    // spec: installer/SPEC.md §The gate binary — the artifact path is exempt from the ownership
    // rule the seam below still runs, so a substituted binary is rewritten rather than kept and
    // the remedy §doctor prints is one a bare re-run performs.
    if !p.dry {
        // spec: installer/SPEC.md §The manifest — an on-disk artifact that still verifies
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
                write_atomically(
                    &seam_path,
                    &seam_text(existing.as_deref(), p.dest, p.declared),
                )?;
            }
            records.push(format!("own\t{}", p.seam));
        }
    }

    Ok(records)
}

// spec: installer/SPEC.md §The gate binary — the seam's declared lines, built once for both
// callers; an empty value is omitted rather than written blank
pub fn declared_lines(kits: &str, spec_base_url: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (name, value) in [
        ("GATE_SDK_KIT_DIRS", kits.trim()),
        ("GATE_SDK_SPEC_BASE_URL", spec_base_url.trim()),
    ] {
        if !value.is_empty() {
            out.push((name.to_string(), value.to_string()));
        }
    }
    out
}

fn place_artifact(args: &[String]) -> i32 {
    let parsed = match parse(
        args,
        &["root", "src", "dest", "seam", "target", "digest", "lock", "kits", "spec-base-url"],
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
    // spec: installer/SPEC.md §The install boundary — the two declared-line keys are optional and
    // omitting one omits its knob rather than writing a blank: an empty `GATE_SDK_KIT_DIRS` means
    // *derive the set*, so a placeholder line would assert a configuration nobody asked for.
    let declared = declared_lines(
        parsed.get("kits").unwrap_or_default(),
        parsed.get("spec-base-url").unwrap_or_default(),
    );
    let placement = Placement {
        root: PathBuf::from(&resolved[0]),
        src: &resolved[1],
        dest: &resolved[2],
        seam: &resolved[3],
        declared: &declared,
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

// spec: installer/SPEC.md §The install boundary — the family's one READ op: which template the
// queue is seeded from is a derivation the package owns, so a caller outside the package boundary
// reads it across this wire rather than carrying a second implementation of the rule.
fn queue_source_records(payload: &str, kits: &str) -> Vec<String> {
    let kits: Vec<String> = kits
        .split(',')
        .filter(|k| !k.is_empty())
        .map(str::to_string)
        .collect();
    // spec: installer/SPEC.md §What init seeds — a kit set owed no queue emits no record, so an
    // empty wire is the answer rather than a record whose field a caller must then interpret.
    match crate::installer::recipe::queue_source(Path::new(payload), &kits) {
        Some(src) => vec![format!("queue-source\t{}", src)],
        None => Vec::new(),
    }
}

fn queue_source(args: &[String]) -> i32 {
    let parsed = match parse(args, &["payload", "kits"], &[]) {
        Ok(a) => a,
        Err(e) => return usage_error(&format!("queue-source: {}", e)),
    };
    let resolved: Result<Vec<String>, String> =
        ["payload", "kits"].iter().map(|k| parsed.required(k)).collect();
    let resolved = match resolved {
        Ok(v) => v,
        Err(e) => return usage_error(&format!("queue-source: {}", e)),
    };
    for r in queue_source_records(&resolved[0], &resolved[1]) {
        println!("{}", r);
    }
    0
}

// spec: installer/SPEC.md §The install boundary — the family's entry point, resolved in `main`
// before the registry lookup, and the family's exit statuses: 0 performed or planned, 1 an
// adopter-actionable refusal, 2 usage or harness error.
pub fn run(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("place-artifact") => place_artifact(&args[1..]),
        Some("queue-source") => queue_source(&args[1..]),
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

    // spec: installer/SPEC.md §The install boundary — the read op's wire: one tab-separated record
    // when the kit set is owed a queue and an EMPTY wire when it is not, so "nonempty means owed" is
    // the caller's whole reading rather than a field it has to interpret.
    #[test]
    fn the_queue_source_op_emits_one_record_or_an_empty_wire() {
        let dir = std::env::temp_dir().join(format!("cw-qs-op-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(dir.join("queue-kit/templates")).expect("cannot make the tree");
        std::fs::write(dir.join("queue-kit/templates/TASK-QUEUE.md"), "# q\n")
            .expect("cannot seed the template");
        let p = dir.to_string_lossy().into_owned();
        assert!(queue_source_records(&p, "gate-sdk").is_empty());
        let owed = queue_source_records(&p, "gate-sdk,queue-kit");
        assert_eq!(owed.len(), 1);
        assert!(owed[0].starts_with("queue-source\t"));
        assert!(owed[0].ends_with("queue-kit/templates/TASK-QUEUE.md"));
        std::fs::remove_dir_all(&dir).ok();
    }

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
            seam: "scripts/gate-sdk-config.knobs",
            declared: &[],
            target: "x86_64-unknown-linux-gnu",
            digest,
            force: false,
            dry,
        }
    }

    // spec: installer/SPEC.md §The install boundary — an unknown key exits 2 rather than being
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

    // spec: installer/SPEC.md §The gate binary — the seam rewrite preserves every line except those
    // whose head is a knob this op owns, whatever blanks surround its `=`
    #[test]
    fn the_seam_rewrite_keeps_every_other_line_and_replaces_the_owned_one() {
        assert_eq!(seam_text(None, "scripts/checkwright-gates", &[]), "GATE_SDK_NATIVE_BIN = scripts/checkwright-gates\n");
        let existing = "# a comment\nGATE_SDK_TMP_DIR = .scratch\nGATE_SDK_NATIVE_BIN=stale\n  GATE_SDK_NATIVE_BIN = older\n";
        let rewritten = seam_text(Some(existing), "scripts/checkwright-gates", &[]);
        assert_eq!(
            rewritten,
            "# a comment\nGATE_SDK_TMP_DIR = .scratch\nGATE_SDK_NATIVE_BIN = scripts/checkwright-gates\n"
        );
        // comment-tier-exempt: a source file with no closing newline is a local property of the
        // input, not a rule either tier owns — `grep -v` terminated its last line and so must this
        assert_eq!(seam_text(Some("A = 1"), "b", &[]), "A = 1\nGATE_SDK_NATIVE_BIN = b\n");
    }

    // spec: installer/SPEC.md §What init seeds — a declared line is owned exactly as the artifact
    // path is: a stale spelling of it is replaced rather than duplicated, an adopter's own knob
    // survives, and an empty value is omitted rather than written blank.
    #[test]
    fn a_declared_line_is_owned_replaced_and_omitted_when_empty() {
        let declared = declared_lines("gate-sdk canon-kit", "https://example.test");
        assert_eq!(
            declared,
            vec![
                ("GATE_SDK_KIT_DIRS".to_string(), "gate-sdk canon-kit".to_string()),
                ("GATE_SDK_SPEC_BASE_URL".to_string(), "https://example.test".to_string()),
            ]
        );
        assert_eq!(declared_lines("  ", ""), Vec::new());
        assert_eq!(
            declared_lines("gate-sdk", ""),
            vec![("GATE_SDK_KIT_DIRS".to_string(), "gate-sdk".to_string())]
        );

        let existing = "GATE_SDK_TMP_DIR = .scratch\nGATE_SDK_KIT_DIRS = stale one\nGATE_SDK_NATIVE_BIN = old\n";
        assert_eq!(
            seam_text(Some(existing), "scripts/checkwright-gates", &declared),
            "GATE_SDK_TMP_DIR = .scratch\nGATE_SDK_NATIVE_BIN = scripts/checkwright-gates\n\
             GATE_SDK_KIT_DIRS = gate-sdk canon-kit\nGATE_SDK_SPEC_BASE_URL = https://example.test\n"
        );
    }

    // spec: installer/SPEC.md §The install boundary — claim's three ways to reach `Take`: no
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

    // spec: installer/SPEC.md §The install boundary — the two wire verbs and the skip-rewrite
    // branch over a scratch tree: a first placement owns both paths, a bare re-run copies nothing,
    // a changed dest is kept at its hash, and `--dry-run` plans the same and writes none of it.
    #[test]
    fn a_placement_owns_both_paths_and_a_bare_re_run_rewrites_nothing() {
        let dir = scratch("place");
        let src = dir.join("payload-binary");
        std::fs::write(&src, "artifact bytes\n").expect("cannot write the scratch artifact");
        let digest = sha256::file_hex(&src).expect("cannot hash the scratch artifact");
        let src_s = src.to_string_lossy().into_owned();
        let (dest, seam) = ("scripts/checkwright-gates", "scripts/gate-sdk-config.knobs");
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
            .contains(&format!("GATE_SDK_NATIVE_BIN = {}\n", dest)));

        // spec: installer/SPEC.md §The manifest — an unreadable `--src` is what proves the
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

        // spec: installer/SPEC.md §The gate binary — the substitution case, which is the one
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
