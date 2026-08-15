// spec: CLAUDE.md §Housekeeping — the installer package declares no resolvable-dependency field
// and no install-time lifecycle script, the two shapes that would turn a one-shot vendoring
// installer into a dependency channel or a run-on-install code path
use crate::fresh;
use serde_json::Value;
use std::path::Path;

const DEFAULT_PKG: &str = "installer/package.json";
// spec: CLAUDE.md §Housekeeping — the field's presence is the finding, not its emptiness: an
// empty dependency map declares a channel with nothing in it yet, and a lifecycle hook is code
// that runs on install whatever its body
const DEP_FIELDS: &[&str] = &["dependencies", "peerDependencies", "optionalDependencies"];
const LIFECYCLE_KEYS: &[&str] = &["preinstall", "install", "postinstall"];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-installer-no-deps: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let pkg = fresh::positional(args, 0, DEFAULT_PKG);
    if !Path::new(pkg).is_file() {
        return Err(format!("package file not found: {}", pkg));
    }
    let text = fresh::read_captured(pkg)?;
    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — the linked reader replaces `jq`,
    // so the `command -v jq` guard has no compiled counterpart; a document the reader cannot
    // parse keeps `fail_closed`'s exit and its wording, naming the parse rather than the program
    let doc: Value = serde_json::from_str(&text).map_err(|e| {
        format!(
            "{} is not readable as JSON ({}) — the check could not run; treating as failure (not clean)",
            pkg, e
        )
    })?;

    let has = |v: Option<&Value>, k: &str| -> bool {
        matches!(v, Some(Value::Object(m)) if m.contains_key(k))
    };
    let mut findings: Vec<String> = Vec::new();
    for k in DEP_FIELDS {
        if has(Some(&doc), k) {
            findings.push(format!("resolvable-dependency field declared: {}", k));
        }
    }
    for k in LIFECYCLE_KEYS {
        if has(doc.get("scripts"), k) {
            findings.push(format!("install-time lifecycle script declared: scripts.{}", k));
        }
    }

    if !findings.is_empty() {
        println!(
            "check-installer-no-deps: {} would make the installer a resolved-dependency channel rather than a one-shot vendoring:",
            pkg
        );
        println!();
        // spec: gate-sdk/SPEC.md §The consumer remainder cohort — the shell form indents the
        // *capture*, not each finding: `printf '  %s\n' "$findings"` takes one argument, so only
        // the first line carries the lead. Reproduced, because criterion 2 is byte parity.
        println!("  {}", findings.join("\n"));
        println!("  help: drop the field. The installer copies bundled source and commits it —");
        println!("        nothing may resolve at an adopter's build time and nothing may run at");
        println!("        install time. A payload the package needs is assembled at pack time and");
        println!("        shipped inside the tarball, never fetched.");
        return Ok(1);
    }

    println!(
        "INSTALLER-NO-DEPS: clean ({} declares no dependency field and no install-time lifecycle script)",
        pkg
    );
    Ok(0)
}
