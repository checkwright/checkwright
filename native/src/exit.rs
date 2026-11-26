// spec: gate-sdk/SPEC.md §Output contract — the one module spelling a status other than 0 and 1:
// the error kinds, their statuses, and the declared verdict-word statuses
use std::fmt;
use std::path::Path;

pub const USAGE: i32 = 2;
pub const REFUSED: i32 = 2;
pub const INPUT: i32 = 3;
pub const TOOL: i32 = 3;

// spec: gate-sdk/SPEC.md §The hook contract — a harness hook member keeps 2 for a refusal and a
// tool error alike, since the harness blocks on 2 alone
pub const HOOK_BLOCK: i32 = 2;

// spec: delegation-kit/SPEC.md §usage-verdict — a fail-soft verdict word, not an error
pub const STALE: i32 = 2;
// spec: delegation-kit/SPEC.md §model-verdict — a fail-soft verdict word, not an error
pub const UNKNOWN: i32 = 2;
// spec: delegation-kit/SPEC.md §The foreign-vendor run — the verdict line's `FAILED`
pub const FAILED: i32 = 2;
// spec: delegation-kit/SPEC.md §The foreign-vendor run — `--budget`'s `OFF`
pub const OFF: i32 = 2;
// spec: gate-sdk/SPEC.md §upgrade-smoke — `FAIL(env)`, the environment could not host the run
pub const FAIL_ENV: i32 = 2;
// spec: gate-sdk/SPEC.md §run-config-variant — no comparison made
pub const NO_COMPARISON: i32 = 2;
// spec: queue-kit/SPEC.md §The queue verbs — nothing written
pub const NOTHING_WRITTEN: i32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Usage,
    Refused,
    Input,
    Tool,
}

impl Kind {
    pub fn status(self) -> i32 {
        match self {
            Kind::Usage => USAGE,
            Kind::Refused => REFUSED,
            Kind::Input => INPUT,
            Kind::Tool => TOOL,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Kind::Usage => "usage",
            Kind::Refused => "refused",
            Kind::Input => "input",
            Kind::Tool => "tool",
        }
    }
}

// spec: gate-sdk/SPEC.md §Output contract — no conversion from a bare string exists, so every
// error site names its kind and the compiler finds the one that does not
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fault {
    pub kind: Kind,
    pub message: String,
    pub stdout: Option<String>,
}

impl Fault {
    fn of(kind: Kind, message: impl Into<String>) -> Self {
        Fault { kind, message: message.into(), stdout: None }
    }

    pub fn usage(message: impl Into<String>) -> Self {
        Fault::of(Kind::Usage, message)
    }

    pub fn refused(message: impl Into<String>) -> Self {
        Fault::of(Kind::Refused, message)
    }

    pub fn input(message: impl Into<String>) -> Self {
        Fault::of(Kind::Input, message)
    }

    pub fn tool(message: impl Into<String>) -> Self {
        Fault::of(Kind::Tool, message)
    }

    pub fn status(&self) -> i32 {
        self.kind.status()
    }

    pub fn prefixed(mut self, prefix: &str) -> Self {
        self.message = format!("{}: {}", prefix, self.message);
        self
    }

    pub fn with_stdout(mut self, line: impl Into<String>) -> Self {
        self.stdout = Some(line.into());
        self
    }

    // spec: gate-sdk/SPEC.md §Output contract — a configured file's read error splits on whether a
    // regular file stands at the path
    pub fn reading(path: impl AsRef<Path>, message: impl Into<String>) -> Self {
        Fault::of(if path.as_ref().is_file() { Kind::Input } else { Kind::Refused }, message)
    }
}

// spec: gate-sdk/SPEC.md §Output contract — a configured file read whole: absent refused, unreadable input
pub fn read_configured(path: impl AsRef<Path>, what: &str) -> Result<String, Fault> {
    let p = path.as_ref();
    if !p.is_file() {
        return Err(Fault::refused(format!("no {} at {}", what, p.display())));
    }
    std::fs::read_to_string(p).map_err(|e| Fault::input(format!("cannot read the {} {}: {}", what, p.display(), e)))
}

// spec: gate-sdk/SPEC.md §Output contract — an optional configured file: nothing at the path
// contributes nothing, and anything there that is not a readable regular file is input
pub fn read_optional(path: &str, what: &str) -> Result<Option<String>, Fault> {
    let p = Path::new(path);
    if std::fs::symlink_metadata(p).is_err() {
        return Ok(None);
    }
    if !p.is_file() {
        return Err(Fault::input(format!("{} is not a regular file: {}", what, path)));
    }
    std::fs::read_to_string(p)
        .map(Some)
        .map_err(|e| Fault::input(format!("cannot read the {} {}: {}", what, path, e)))
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

// spec: gate-sdk/SPEC.md §Output contract — the message is what a test and a relaying caller read
impl std::ops::Deref for Fault {
    type Target = str;
    fn deref(&self) -> &str {
        &self.message
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the error document a `--json` arm prints where it
// refuses or fails itself
pub fn error_document(contract: &str, kind: Kind) -> String {
    format!("{{\"contract\":\"{}\",\"error\":\"{}\"}}\n", contract, kind.word())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn each_kind_exits_its_side_of_the_cut() {
        assert_eq!(Fault::usage("x").status(), 2);
        assert_eq!(Fault::refused("x").status(), 2);
        assert_eq!(Fault::input("x").status(), 3);
        assert_eq!(Fault::tool("x").status(), 3);
        assert_eq!(Fault::tool("m").prefixed("--arm").message, "--arm: m");
    }

    #[test]
    fn a_configured_file_is_refused_absent_and_input_unreadable() {
        let dir = std::env::temp_dir().join(format!("exit-configured-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot make the scratch dir");
        let bad = dir.join("bad.txt");
        std::fs::write(&bad, [0xffu8, 0xfe]).expect("cannot write the scratch file");
        assert_eq!(read_configured(dir.join("gone.txt"), "list").unwrap_err().kind, Kind::Refused);
        assert_eq!(read_configured(&dir, "list").unwrap_err().kind, Kind::Refused);
        assert_eq!(read_configured(&bad, "list").unwrap_err().kind, Kind::Input);
        assert_eq!(Fault::reading(dir.join("gone.txt"), "m").kind, Kind::Refused);
        assert_eq!(Fault::reading(&bad, "m").kind, Kind::Input);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_error_document_names_the_kind() {
        assert_eq!(error_document("1.1.0", Kind::Input), "{\"contract\":\"1.1.0\",\"error\":\"input\"}\n");
    }

    // spec: gate-sdk/SPEC.md §Output contract — the literal-status roster: the statuses a gate or
    // an `Arm::Run` arm returns are integers no type holds, so this is the worklist for them
    fn literal_status(line: &str) -> bool {
        let t = line.trim();
        if t.starts_with("//") {
            return false;
        }
        let code = t.split("//").next().unwrap_or("").trim_end();
        ["return 2;", "return 3;", "exit(2)", "exit(3)", "Some(2)", "Some(3)"].iter().any(|s| code.contains(s))
            || code == "2"
            || code == "3"
            || code.ends_with("=> 2,")
            || code.ends_with("=> 3,")
            || code.ends_with("=> 2")
            || code.ends_with("=> 3")
    }

    #[test]
    fn the_literal_detector_reads_code_only() {
        assert!(literal_status("        return 2;"));
        assert!(literal_status("            2"));
        assert!(literal_status("        Err(_) => 3,"));
        assert!(!literal_status("        return exit::INPUT;"));
        assert!(!literal_status("    // return 2;"));
        assert!(!literal_status("        x.len() == 2"));
    }

    #[test]
    fn no_shipped_module_spells_a_literal_status_outside_this_one() {
        let _knobs = crate::knobenv::lock();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files = crate::walk::find_files(&src, &["rs"]).expect("cannot enumerate the crate modules");
        assert!(!files.is_empty(), "no module found to scan");
        let root = std::fs::read_to_string(src.join("main.rs")).expect("cannot read main.rs");
        // spec: gate-sdk/SPEC.md §Fail-closed contract — test scope dropped by derivation, as the
        // spawn-funnel test drops it
        let test_only: Vec<String> = root
            .lines()
            .collect::<Vec<&str>>()
            .windows(2)
            .filter(|w| w[0].trim() == "#[cfg(test)]")
            .filter_map(|w| w[1].trim().strip_prefix("mod ").and_then(|m| m.strip_suffix(';')).map(|m| format!("{}.rs", m)))
            .collect();
        let mut offenders: Vec<String> = Vec::new();
        for f in &files {
            let leaf = f.file_name().unwrap_or_default().to_string_lossy().into_owned();
            if leaf == "exit.rs" || test_only.contains(&leaf) {
                continue;
            }
            let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            let shipped = crate::proc::tests::shipped_scope(&text);
            for line in shipped.lines() {
                if literal_status(line) && !line.contains("status-literal-exempt:") {
                    offenders.push(format!("{}: {}", f.display(), line.trim()));
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "a shipped module spells a status 2 or 3 as a literal ({} sites): {:#?} — return the \
             cut's named status from native/src/exit.rs (gate-sdk/SPEC.md §Output contract), or \
             mark a literal that is no status with a `status-literal-exempt:` cause on its line",
            offenders.len(),
            offenders
        );
    }
}
