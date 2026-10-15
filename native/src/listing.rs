use crate::{proc, programs};

// spec: gate-sdk/SPEC.md §Fail-closed contract — a failed listing and a name that is not UTF-8 are two classes: a caller may degrade the first, never the second
#[derive(Debug)]
pub enum Refusal {
    Spawn(String),
    Failed(Option<i32>),
    NotUtf8(String),
}

impl Refusal {
    pub fn text_or(self, failed: impl FnOnce(Option<i32>) -> String) -> String {
        match self {
            Refusal::Spawn(e) => e,
            Refusal::Failed(code) => failed(code),
            Refusal::NotUtf8(e) => e,
        }
    }

    pub fn text(self) -> String {
        self.text_or(|code| crate::fresh::fail_closed("git-ls-files", code))
    }
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the crate's tracked-set listing: NUL-terminated, a name returned as written
pub fn tracked(top: Option<&str>, pathspecs: &[&str]) -> Result<Vec<String>, Refusal> {
    let mut argv: Vec<&str> = vec!["ls-files", "-z", "--"];
    argv.extend(pathspecs);
    names(top, &argv)
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — any NUL-terminated name read: the caller's argv carries `-z`
pub fn names(top: Option<&str>, args: &[&str]) -> Result<Vec<String>, Refusal> {
    decode(&raw(top, args)?)
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — one `status --porcelain -z` entry: its two-byte code and the path it names now
#[derive(Debug, PartialEq)]
pub struct Status {
    pub code: String,
    pub path: String,
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — a rename or copy entry carries its source as the next field, which names no current path
pub fn status(top: Option<&str>, pathspecs: &[&str]) -> Result<Vec<Status>, Refusal> {
    let mut argv: Vec<&str> = vec!["status", "--porcelain", "-z", "--"];
    argv.extend(pathspecs);
    let mut fields = decode(&raw(top, &argv)?)?.into_iter();
    let mut out: Vec<Status> = Vec::new();
    while let Some(entry) = fields.next() {
        let (code, path) = (entry.get(..2).unwrap_or(""), entry.get(3..).unwrap_or(""));
        if code.contains(['R', 'C']) {
            fields.next();
        }
        out.push(Status { code: code.to_string(), path: path.to_string() });
    }
    Ok(out)
}

fn raw(top: Option<&str>, args: &[&str]) -> Result<Vec<u8>, Refusal> {
    let mut argv: Vec<&str> = top.map(|t| vec!["-C", t]).unwrap_or_default();
    argv.extend(args);
    let ls = proc::run(&programs::GIT, &argv).map_err(Refusal::Spawn)?;
    match ls.stdout() {
        Some(out) => Ok(out.to_vec()),
        None => Err(Refusal::Failed(ls.code())),
    }
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the one decode of a NUL-terminated listing a caller spawned itself
pub fn decode(listing: &[u8]) -> Result<Vec<String>, Refusal> {
    let mut out: Vec<String> = Vec::new();
    for raw in listing.split(|b| *b == 0).filter(|n| !n.is_empty()) {
        match std::str::from_utf8(raw) {
            Ok(n) => out.push(n.to_string()),
            Err(_) => {
                return Err(Refusal::NotUtf8(format!(
                    "member '{}' has a name that is not UTF-8 — git's listing of it cannot be read; treating as failure (not clean)",
                    String::from_utf8_lossy(raw)
                )))
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> (std::path::PathBuf, String) {
        let base = std::env::temp_dir().join(format!("checkwright-listing-{}.{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).expect("mkdir");
        let top = base.display().to_string();
        assert!(git(&["-C", &top, "init", "-q"]));
        (base, top)
    }

    fn git(args: &[&str]) -> bool {
        proc::run(&programs::GIT, args).expect("git runs").stdout().is_some()
    }

    #[test]
    fn a_name_git_would_quote_is_listed_under_the_spelling_that_opens_it() {
        let (base, top) = scratch("quoted");
        let mut written = vec!["na\u{ef}ve.md", "plain.md"];
        if cfg!(unix) {
            written.extend(["a\"quote.md", "back\\slash.md"]);
        }
        written.sort_unstable();
        for n in &written {
            std::fs::write(base.join(n), "x\n").expect("write");
            assert!(git(&["-C", &top, "add", "--", n]));
        }
        let listed = tracked(Some(&top), &["."]);
        let opened = listed.iter().flatten().all(|l| std::fs::read(base.join(l)).is_ok());
        let _ = std::fs::remove_dir_all(&base);
        let mut listed = listed.expect("the listing succeeds");
        listed.sort_unstable();
        assert_eq!(listed, written);
        assert!(opened);
    }

    const EMPTINESS: &str = "an emptiness test or a count: it reads no name";

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the line-form roster: a file, its line-form name-listing argvs, and why each keeps the form
    const LINE_FORM: &[(&str, usize, &str)] = &[
        ("build.rs", 1, "the source stamp: four holders write a path as listed, and two are shell"),
        ("src/fresh.rs", 1, "the source stamp: four holders write a path as listed, and two are shell"),
        ("src/emit/git_hook.rs", 1, "the hook-environment test reproduces a hook's own line-form read"),
        ("src/emit/scan_prompts.rs", 1, "the read-only subcommand roster: a word list, no argv"),
        ("src/emit/installer_smoke/report.rs", 1, "a failure report printed for a person, who reads git's quoted spelling"),
        ("src/emit/trajectory.rs", 1, "an amendment's name is a key paired with itself inside one log, never opened"),
        ("src/emit/pack_installer.rs", 4, "emptiness tests, the dirty one printing git's spelling for a person, and an entry read for its object"),
        ("src/emit/enter_stage.rs", 1, EMPTINESS),
        ("src/emit/foreign_run.rs", 1, EMPTINESS),
        ("src/emit/installer_smoke/consumer.rs", 1, "an emptiness and equality test, a dirty set printed for a person"),
        ("src/emit/installer_smoke/roster.rs", 1, EMPTINESS),
        ("src/installer/init.rs", 3, EMPTINESS),
        ("src/installer/uninstall.rs", 1, EMPTINESS),
    ];

    // spec: gate-sdk/SPEC.md §Fail-closed contract — a name-listing argv carrying neither the NUL flag nor a probe's flag reds outside the roster
    #[test]
    fn every_name_listing_argv_in_the_crate_is_nul_terminated_or_rostered() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = vec![root.join("build.rs")];
        files.extend(crate::walk::find_files(&root.join("src"), &["rs"]).expect("the crate source is listable"));
        let q = |w: &str| format!("\"{}\"", w);
        let tokens: Vec<String> = "ls-files ls-tree --name-only --name-status --porcelain".split(' ').map(q).collect();
        let porcelain = &tokens[4];
        let mut line_form: std::collections::BTreeMap<String, usize> = Default::default();
        for f in &files {
            let text = std::fs::read_to_string(f).expect("a crate source reads");
            let rel = f.strip_prefix(root).expect("under the crate").display().to_string().replace('\\', "/");
            let mut argvs: std::collections::BTreeSet<(usize, usize)> = Default::default();
            for token in &tokens {
                for (hit, _) in text.match_indices(token.as_str()) {
                    let start = text[..hit].rfind('[').unwrap_or(0);
                    let end = text[hit..].find(']').map_or(text.len(), |e| hit + e);
                    let argv = &text[start..end];
                    if token == porcelain && !argv.contains(&q("status")) {
                        continue;
                    }
                    if !argv.contains(&q("-z")) && !argv.contains(&q("--error-unmatch")) {
                        argvs.insert((start, end));
                    }
                }
            }
            if !argvs.is_empty() {
                line_form.insert(rel, argvs.len());
            }
        }
        let rostered: std::collections::BTreeMap<String, usize> =
            LINE_FORM.iter().map(|(f, n, _)| (f.to_string(), *n)).collect();
        assert_eq!(
            line_form, rostered,
            "a name-listing argv reads git's line form outside the roster, or a roster row names no \
             such argv: read it NUL-terminated through listing::names or listing::status, or add the \
             file and its reason to LINE_FORM"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the own-decode roster: a file, its NUL splits outside the shared decode, and the ground each keeps
    const OWN_DECODE: &[(&str, usize, &str)] = &[
        ("src/gates/docs_cmd.rs", 1, "a deleted name: refusing it would block its own deletion, and history's names never change"),
        ("src/queue.rs", 1, "a stem matched only against a kebab slug, which no replacement spelling equals"),
        ("src/spec.rs", 1, "the manifest set compares a name's bytes and never decodes one"),
    ];

    // spec: gate-sdk/SPEC.md §Fail-closed contract — a NUL split outside the shared decode reds outside the roster
    #[test]
    fn every_nul_listing_in_the_crate_decodes_through_the_shared_reader_or_is_rostered() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = vec![root.join("build.rs")];
        files.extend(crate::walk::find_files(&root.join("src"), &["rs"]).expect("the crate source is listable"));
        let nul = "\\0";
        let spellings = [format!("split('{}')", nul), format!("split(['{}'", nul), "split(|b| *b == 0)".to_string()];
        let mut own: std::collections::BTreeMap<String, usize> = Default::default();
        for f in &files {
            let rel = f.strip_prefix(root).expect("under the crate").display().to_string().replace('\\', "/");
            if rel == "src/listing.rs" {
                continue;
            }
            let text = std::fs::read_to_string(f).expect("a crate source reads");
            let n: usize = spellings.iter().map(|s| text.matches(s.as_str()).count()).sum();
            if n > 0 {
                own.insert(rel, n);
            }
        }
        let rostered: std::collections::BTreeMap<String, usize> =
            OWN_DECODE.iter().map(|(f, n, _)| (f.to_string(), *n)).collect();
        assert_eq!(
            own, rostered,
            "a NUL-terminated listing is split outside the shared decode, or a roster row names no such \
             split: decode it through listing::names, listing::tracked or listing::decode, or add the \
             file, its count and its ground to OWN_DECODE"
        );
    }

    #[test]
    fn a_decoded_listing_refuses_a_name_that_is_not_utf8_where_a_lossy_read_would_rename_it() {
        let got = decode(b"plain.md\0bad\xff.md\0");
        assert!(matches!(&got, Err(Refusal::NotUtf8(t)) if t.contains("'bad\u{fffd}.md'")), "{:?}", got);
        assert_eq!(decode(b"a.md\0b c.md\0").expect("decodes"), ["a.md", "b c.md"]);
    }

    #[test]
    fn a_status_entry_names_its_current_path_and_drops_a_renames_source() {
        let (base, top) = scratch("status");
        for n in ["a b.md", "na\u{ef}ve.md"] {
            std::fs::write(base.join(n), "x\n").expect("write");
        }
        assert!(git(&["-C", &top, "add", "--", "."]));
        assert!(git(&["-C", &top, "-c", "user.email=a@b", "-c", "user.name=a", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "x"]));
        assert!(git(&["-C", &top, "mv", "a b.md", "c d.md"]));
        std::fs::write(base.join("na\u{ef}ve.md"), "y\n").expect("write");
        let got = status(Some(&top), &[]);
        let opened = got.iter().flatten().all(|s| base.join(&s.path).exists());
        let _ = std::fs::remove_dir_all(&base);
        let mut got = got.expect("the status reads");
        got.sort_by(|a, b| a.path.cmp(&b.path));
        let want = [("R ", "c d.md"), (" M", "na\u{ef}ve.md")];
        assert_eq!(got.iter().map(|s| (s.code.as_str(), s.path.as_str())).collect::<Vec<_>>(), want);
        assert!(opened);
    }

    #[test]
    fn a_listing_git_cannot_produce_is_the_failed_class() {
        let base = std::env::temp_dir().join(format!("checkwright-listing-absent.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let got = tracked(Some(&base.display().to_string()), &["."]);
        assert!(matches!(got, Err(Refusal::Failed(_))), "{:?}", got);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn a_name_that_is_not_utf8_refuses_and_names_the_member() {
        use std::os::unix::ffi::OsStrExt;
        let (base, top) = scratch("non-utf8");
        std::fs::write(base.join("plain.md"), "x\n").expect("write");
        std::fs::write(base.join(std::ffi::OsStr::from_bytes(b"bad\xff.md")), "x\n").expect("write");
        assert!(git(&["-C", &top, "add", "--", "."]));
        let got = tracked(Some(&top), &["."]);
        let _ = std::fs::remove_dir_all(&base);
        match got {
            Err(Refusal::NotUtf8(text)) => {
                assert!(text.contains("'bad\u{fffd}.md'"), "{}", text);
                assert!(text.contains("is not UTF-8"), "{}", text);
            }
            other => panic!("expected the not-UTF-8 refusal, got {:?}", other),
        }
    }
}
