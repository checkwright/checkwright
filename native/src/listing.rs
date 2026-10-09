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
    let mut argv: Vec<&str> = top.map(|t| vec!["-C", t]).unwrap_or_default();
    argv.extend(["ls-files", "-z", "--"]);
    argv.extend(pathspecs);
    let ls = proc::run(&programs::GIT, &argv).map_err(Refusal::Spawn)?;
    let Some(out) = ls.stdout() else {
        return Err(Refusal::Failed(ls.code()));
    };
    names(out)
}

fn names(listing: &[u8]) -> Result<Vec<String>, Refusal> {
    let mut out: Vec<String> = Vec::new();
    for raw in listing.split(|b| *b == 0).filter(|n| !n.is_empty()) {
        match std::str::from_utf8(raw) {
            Ok(n) => out.push(n.to_string()),
            Err(_) => {
                return Err(Refusal::NotUtf8(format!(
                    "tracked member '{}' has a name that is not UTF-8 — the tracked set cannot be read; treating as failure (not clean)",
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

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the line-form roster: a file, and why its listing token keeps the line form
    const LINE_FORM: &[(&str, &str)] = &[
        ("build.rs", "the source stamp: four holders write a path as listed, and two are shell"),
        ("src/fresh.rs", "the source stamp: four holders write a path as listed, and two are shell"),
        ("src/emit/git_hook.rs", "the hook-environment test reproduces a hook's own line-form read"),
        ("src/emit/scan_prompts.rs", "the read-only subcommand roster: a word list, no argv"),
    ];

    // spec: gate-sdk/SPEC.md §Fail-closed contract — a quoted listing token whose argv carries neither the NUL flag nor a probe's flag reds outside the roster
    #[test]
    fn every_listing_argv_in_the_crate_is_nul_terminated_or_rostered() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = vec![root.join("build.rs")];
        files.extend(crate::walk::find_files(&root.join("src"), &["rs"]).expect("the crate source is listable"));
        let token = format!("\"ls-{}\"", "files");
        let mut line_form: std::collections::BTreeMap<String, usize> = Default::default();
        for f in &files {
            let text = std::fs::read_to_string(f).expect("a crate source reads");
            let rel = f.strip_prefix(root).expect("under the crate").display().to_string().replace('\\', "/");
            let mut at = 0usize;
            while let Some(hit) = text[at..].find(&token) {
                let start = at + hit;
                let end = text[start..].find(']').map_or(text.len(), |e| start + e);
                let argv = &text[start..end];
                if !argv.contains("\"-z\"") && !argv.contains("\"--error-unmatch\"") {
                    *line_form.entry(rel.clone()).or_default() += 1;
                }
                at = start + token.len();
            }
        }
        let rostered: std::collections::BTreeMap<String, usize> =
            LINE_FORM.iter().map(|(f, _)| (f.to_string(), 1)).collect();
        assert_eq!(
            line_form, rostered,
            "a listing argv reads git's line form outside the roster, or a roster row names no such \
             token: list NUL-terminated through listing::tracked, or add the file and its reason to LINE_FORM"
        );
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
