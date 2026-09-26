// spec: gate-sdk/SPEC.md §check-release-assets — what a Release carries is declared once in
// §Consumer payload and held at the tag, before the Release exists
use crate::{fresh, registry, walk};
use std::collections::BTreeSet;
use std::path::Path;

const DEFAULT_DOC: &str = "gate-sdk/SPEC.md";
const OPEN: &str = "<!-- release-assets:";
const CLOSE: &str = "-->";
const USAGE: &str = "usage: check-release-assets [<doc> <workflow>] [--dist <dir> <version>]";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-release-assets: {}", e);
            2
        }
    }
}

struct Args {
    doc: String,
    workflow: Option<String>,
    dist: Option<(String, String)>,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut positionals: Vec<&str> = Vec::new();
    let mut dist = None;
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--dist" {
            match (args.get(i + 1), args.get(i + 2)) {
                (Some(d), Some(v)) => dist = Some((d.clone(), v.clone())),
                _ => return Err(format!("--dist takes a directory and a version\n{}", USAGE)),
            }
            i += 3;
            continue;
        }
        if a.starts_with("--") {
            return Err(format!("unknown argument '{}'\n{}", a, USAGE));
        }
        positionals.push(a);
        i += 1;
    }
    if positionals.len() > 2 {
        return Err(format!("at most two positionals are admitted\n{}", USAGE));
    }
    Ok(Args {
        doc: positionals
            .first()
            .filter(|p| !p.is_empty())
            .map_or(DEFAULT_DOC.to_string(), |p| p.to_string()),
        workflow: positionals.get(1).filter(|p| !p.is_empty()).map(|p| p.to_string()),
        dist,
    })
}

// spec: gate-sdk/SPEC.md §check-release-assets — a declaration is a full line, opened at column
// zero and closed at its end; its templates are the whitespace-separated words between
fn declaration(line: &str) -> Option<Vec<&str>> {
    let body = line.trim_end().strip_prefix(OPEN)?.strip_suffix(CLOSE)?;
    Some(body.split_whitespace().collect())
}

fn template_fault(t: &str) -> Option<&'static str> {
    if t.contains('/') {
        return Some("carries a '/', and an asset name is flat");
    }
    if t.replace("{version}", "").replace("{target}", "").contains(['{', '}']) {
        return Some("carries a placeholder other than {version} and {target}");
    }
    if !t.contains("{version}") {
        return Some("lacks the required {version}");
    }
    None
}

fn expand(templates: &[&str], version: &str, targets: &[String]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for t in templates {
        let v = t.replace("{version}", version);
        if v.contains("{target}") {
            for target in targets {
                out.insert(v.replace("{target}", target));
            }
        } else {
            out.insert(v);
        }
    }
    out
}

fn version_fault(v: &str) -> Option<&'static str> {
    if v.is_empty() {
        Some("the version is empty")
    } else if v.contains(char::is_whitespace) || v.contains('/') {
        Some("the version carries whitespace or a '/'")
    } else {
        None
    }
}

fn roster(text: &str, path: &str) -> Result<Vec<String>, String> {
    let targets = registry::members(text)
        .iter()
        .map(|t| t.trim().to_string())
        .collect::<Vec<String>>();
    if targets.is_empty() {
        return Err(format!(
            "the target roster at {} declares no target, so no per-target asset can be expanded",
            path
        ));
    }
    Ok(targets)
}

fn is_call(line: &str) -> bool {
    !line.trim_start().starts_with('#')
        && line.contains("check-release-assets")
        && line.split_whitespace().any(|w| w == "--dist")
}

// spec: gate-sdk/SPEC.md §check-release-assets — the battery half: one declaration, every template
// well-formed and unique; the returned set is every well-formed template the publish half expands
fn battery_half<'a>(doc: &str, text: &'a str, findings: &mut Vec<String>) -> Vec<&'a str> {
    let decls: Vec<(usize, Vec<&str>)> = fresh::file_lines(text)
        .iter()
        .enumerate()
        .filter_map(|(n, l)| declaration(l).map(|t| (n + 1, t)))
        .collect();
    if decls.is_empty() {
        findings.push(format!(
            "{}: no '{} <template>... {}' declaration line, so nothing owns the Release's asset set",
            doc, OPEN, CLOSE
        ));
    }
    if decls.len() > 1 {
        let at: Vec<String> = decls.iter().map(|(n, _)| n.to_string()).collect();
        findings.push(format!(
            "{}: {} declaration lines (lines {}), and exactly one is admissible",
            doc,
            decls.len(),
            at.join(", ")
        ));
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut kept: Vec<&str> = Vec::new();
    for (n, templates) in &decls {
        if templates.is_empty() {
            findings.push(format!("{}:{}: the declaration names no asset", doc, n));
        }
        for t in templates {
            if let Some(fault) = template_fault(t) {
                findings.push(format!("{}:{}: template '{}' {}", doc, n, t, fault));
            } else if !seen.insert(t) {
                findings.push(format!("{}:{}: template '{}' is declared twice", doc, n, t));
            } else {
                kept.push(t);
            }
        }
    }
    kept
}

// spec: gate-sdk/SPEC.md §check-release-assets — the publish half: the directory's own entries,
// one level, equal the declared set; a subdirectory is an entry like any file
fn publish_half(
    dir: &str,
    declared: &BTreeSet<String>,
    entries: &[(String, bool)],
    findings: &mut Vec<String>,
) {
    let present: BTreeSet<&str> = entries.iter().map(|(n, _)| n.as_str()).collect();
    for name in declared {
        if !present.contains(name.as_str()) {
            findings.push(format!("{}: declared asset '{}' is absent", dir, name));
        }
    }
    for (name, is_dir) in entries {
        if !declared.contains(name) {
            let kind = if *is_dir { "directory" } else { "entry" };
            findings.push(format!("{}: {} '{}' is declared by no template", dir, kind, name));
        }
    }
}

fn read(path: &str, what: &str) -> Result<String, String> {
    if !Path::new(path).is_file() {
        return Err(format!("{} not found: {}", what, path));
    }
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {} {}: {}", what, path, e))
}

fn rule(args: &[String]) -> Result<i32, String> {
    let a = parse(args)?;
    let doc_text = read(&a.doc, "declaring doc")?;
    let workflows = match &a.workflow {
        Some(w) => vec![w.clone()],
        None => walk::knob_words("GATE_SDK_NATIVE_PUBLISH_WORKFLOW")?,
    };
    if workflows.is_empty() {
        return Err("GATE_SDK_NATIVE_PUBLISH_WORKFLOW names no workflow".to_string());
    }
    let mut wired = false;
    for w in &workflows {
        wired |= fresh::file_lines(&read(w, "publish workflow")?)
            .iter()
            .any(|l| is_call(l));
    }

    let mut findings: Vec<String> = Vec::new();
    let templates = battery_half(&a.doc, &doc_text, &mut findings);
    if !wired {
        findings.push(format!(
            "{}: no step invokes check-release-assets with --dist, so no tag's asset set is held",
            workflows.join(" ")
        ));
    }

    let mut dist_note = String::new();
    if let Some((dir, version)) = &a.dist {
        if let Some(fault) = version_fault(version) {
            return Err(format!("{}: '{}'", fault, version));
        }
        let entries = walk::list_dir(Path::new(dir))?;
        let roster_path = walk::knob_scalar("GATE_SDK_NATIVE_TARGETS_FILE")?;
        let targets = roster(&read(&roster_path, "target roster")?, &roster_path)?;
        let declared = expand(&templates, version, &targets);
        publish_half(dir, &declared, &entries, &mut findings);
        dist_note = format!(
            "; {} carries exactly the {} declared asset(s) for version {}",
            dir,
            declared.len(),
            version
        );
    }

    if !findings.is_empty() {
        println!("check-release-assets: the Release's asset set disagrees with its declaration (gate-sdk/SPEC.md §check-release-assets):");
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: keep exactly one well-formed release-assets line in §Consumer payload, keep the --dist call in the publish workflow's pack job, and make the pack output and the line name the same files.");
        return Ok(1);
    }
    println!(
        "RELEASE-ASSETS: clean ({} template(s) declared in {}, the publish half wired in {}{})",
        templates.len(),
        a.doc,
        workflows.join(" "),
        dist_note
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — the declaration is a full line; a quoted or
    // indented spelling is not one
    #[test]
    fn only_a_full_line_comment_is_a_declaration() {
        assert_eq!(
            declaration("<!-- release-assets: a-{version}.tgz b-{version} -->"),
            Some(vec!["a-{version}.tgz", "b-{version}"])
        );
        assert_eq!(declaration("<!-- release-assets: -->  "), Some(vec![]));
        assert_eq!(declaration("`<!-- release-assets: a-{version} -->`"), None);
        assert_eq!(declaration("  <!-- release-assets: a-{version} -->"), None);
        assert_eq!(declaration("<!-- release-assets: a-{version}"), None);
    }

    #[test]
    fn a_template_admits_only_version_and_target_and_needs_version() {
        assert_eq!(template_fault("x-{version}-{target}.tar.gz"), None);
        assert_eq!(template_fault("x-{version}.tgz"), None);
        assert!(template_fault("x.tgz").unwrap().contains("{version}"));
        assert!(template_fault("x-{target}.tgz").unwrap().contains("{version}"));
        assert!(template_fault("x-{version}-{arch}").unwrap().contains("placeholder"));
        assert!(template_fault("x-{version").unwrap().contains("placeholder"));
        assert!(template_fault("dir/x-{version}").unwrap().contains("'/'"));
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — a doc with no declaration is a finding, not a
    // refusal
    #[test]
    fn a_doc_with_no_declaration_is_a_finding() {
        let mut f = Vec::new();
        let kept = battery_half("d.md", "# title\n\nprose\n", &mut f);
        assert!(kept.is_empty());
        assert_eq!(f.len(), 1);
        assert!(f[0].contains("no '<!-- release-assets:"));
    }

    #[test]
    fn a_second_line_and_a_repeated_template_are_findings() {
        let mut f = Vec::new();
        let text = "<!-- release-assets: a-{version} a-{version} -->\n<!-- release-assets: b-{version} -->\n";
        let kept = battery_half("d.md", text, &mut f);
        assert_eq!(kept, vec!["a-{version}", "b-{version}"]);
        assert!(f.iter().any(|l| l.contains("2 declaration lines (lines 1, 2)")));
        assert!(f.iter().any(|l| l.contains("d.md:1: template 'a-{version}' is declared twice")));
    }

    #[test]
    fn a_target_template_expands_once_per_roster_line() {
        let got = expand(&["p-{version}.tgz", "g-{version}-{target}.tar.gz"], "1.2.3", &s(&["t1", "t2"]));
        let want: BTreeSet<String> =
            s(&["p-1.2.3.tgz", "g-1.2.3-t1.tar.gz", "g-1.2.3-t2.tar.gz"]).into_iter().collect();
        assert_eq!(got, want);
    }

    #[test]
    fn the_publish_half_names_each_absent_and_each_undeclared_entry() {
        let declared: BTreeSet<String> = s(&["a", "b"]).into_iter().collect();
        let entries = vec![("a".to_string(), false), ("c".to_string(), false), ("sub".to_string(), true)];
        let mut f = Vec::new();
        publish_half("out", &declared, &entries, &mut f);
        assert_eq!(
            f,
            s(&[
                "out: declared asset 'b' is absent",
                "out: entry 'c' is declared by no template",
                "out: directory 'sub' is declared by no template",
            ])
        );
    }

    #[test]
    fn a_commented_or_flagless_line_is_no_call() {
        assert!(is_call("  bash gate-sdk/bin/run-gates.sh --only check-release-assets -- --dist \"$out\" \"$version\""));
        assert!(!is_call("  # bash gate-sdk/bin/run-gates.sh --only check-release-assets -- --dist x y"));
        assert!(!is_call("  bash gate-sdk/bin/run-gates.sh --only check-release-assets"));
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — every exit-2 path
    #[test]
    fn each_refusal_exits_two() {
        assert_eq!(run(&s(&["no/such/doc.md", "no/such/wf.yml"])), 2);
        assert_eq!(run(&s(&["Cargo.toml", "no/such/wf.yml"])), 2);
        assert_eq!(run(&s(&["Cargo.toml", "Cargo.toml", "--dist", "no/such/dir", "1.0.0"])), 2);
        assert_eq!(run(&s(&["Cargo.toml", "Cargo.toml", "--dist", "src", ""])), 2);
        assert_eq!(run(&s(&["Cargo.toml", "Cargo.toml", "--dist", "src", "1.0 .0"])), 2);
        assert_eq!(run(&s(&["Cargo.toml", "Cargo.toml", "--dist", "src", "1/0"])), 2);
        assert_eq!(run(&s(&["--dist", "src"])), 2);
        assert_eq!(run(&s(&["--bogus"])), 2);
        assert_eq!(run(&s(&["a", "b", "c"])), 2);
        assert!(roster("# only a comment\n\n", "r").is_err());
        assert_eq!(roster("# c\nt1\n", "r").unwrap(), s(&["t1"]));
    }
}
