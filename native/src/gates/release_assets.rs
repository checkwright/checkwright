// spec: gate-sdk/SPEC.md §check-release-assets — what a Release carries, each asset's sidecar and
// its attestation subject are declared once and held at the tag, before the Release exists
use crate::exit::Fault;
use crate::{fresh, proc, programs, registry, sha256, walk};
use std::collections::BTreeSet;
use std::path::Path;

pub(crate) const DEFAULT_DOC: &str = "gate-sdk/SPEC.md";
const OPEN: &str = "<!-- release-assets:";
const CLOSE: &str = "-->";
const SIDECAR: &str = ".sha256";
const ATTEST_ACTIONS: [&str; 2] = ["actions/attest-build-provenance", "actions/attest"];
const USAGE: &str = "usage: check-release-assets [<doc> <workflow>] [--dist <dir> <version>]";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-release-assets: {}", e);
            e.status()
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
pub(crate) fn declaration(line: &str) -> Option<Vec<&str>> {
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

// spec: gate-sdk/SPEC.md §check-release-assets — every asset carries its sidecar, and every sidecar
// template vouches for a declared stem
fn sidecar_rules(doc: &str, kept: &[&str], findings: &mut Vec<String>) {
    let declared: BTreeSet<&str> = kept.iter().copied().collect();
    for t in kept {
        match t.strip_suffix(SIDECAR) {
            Some(stem) if !declared.contains(stem) => findings.push(format!(
                "{}: sidecar template '{}' vouches for '{}', which is not declared",
                doc, t, stem
            )),
            Some(_) => {}
            None if !declared.contains(format!("{}{}", t, SIDECAR).as_str()) => findings.push(format!(
                "{}: template '{}' has no sidecar template '{}{}' declared beside it",
                doc, t, t, SIDECAR
            )),
            None => {}
        }
    }
}

fn indent(l: &str) -> usize {
    l.len() - l.trim_start_matches(' ').len()
}

fn opaque(l: &str) -> bool {
    let t = l.trim();
    t.is_empty() || t.starts_with('#')
}

// spec: gate-sdk/SPEC.md §check-release-assets — a list item's key sits at the column after its
// `- `, so a step's first key and its later keys share one column
fn key_line(l: &str) -> (usize, &str) {
    let t = l.trim_start_matches(' ');
    match t.strip_prefix("- ") {
        Some(r) => {
            let r2 = r.trim_start_matches(' ');
            (indent(l) + 2 + (r.len() - r2.len()), r2)
        }
        None => (indent(l), t),
    }
}

fn scalar(v: &str) -> &str {
    let v = v.trim();
    let v = match v.find(" #") {
        Some(i) => v[..i].trim_end(),
        None => v,
    };
    v.trim_matches(|c| c == '"' || c == '\'')
}

fn is_attest_uses(v: &str) -> bool {
    let name = scalar(v).split('@').next().unwrap_or("");
    ATTEST_ACTIONS.contains(&name)
}

const FOLD: char = '\0';

// spec: gate-sdk/SPEC.md §check-release-assets — a key's value as the lines the action splits: inline,
// a `|` block's lines indented past the key, or a `>` block's paragraphs, each folded to one line
// with `FOLD` where YAML joins two lines with a space
fn key_value(lines: &[&str], at: usize, col: usize, inline: &str) -> Vec<String> {
    let v = inline.trim();
    if !(v.starts_with('|') || v.starts_with('>')) {
        return vec![scalar(v).to_string()];
    }
    let block = lines[at + 1..]
        .iter()
        .take_while(|l| l.trim().is_empty() || indent(l) > col)
        .map(|l| l.trim())
        .filter(|l| !l.starts_with('#'));
    if v.starts_with('|') {
        return block.map(String::from).collect();
    }
    let mut out: Vec<String> = Vec::new();
    let mut open = false;
    for l in block {
        match (l.is_empty(), open) {
            (true, _) => open = false,
            (false, true) => {
                let last = out.last_mut().expect("an open paragraph has a line");
                last.push(FOLD);
                last.push_str(l);
            }
            (false, false) => {
                out.push(l.to_string());
                open = true;
            }
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §check-release-assets — one attest step's entries, and those a `>` block
// folded across lines, spelled with the space the action reads
#[derive(Debug, Default, PartialEq)]
pub(crate) struct Subjects {
    entries: Vec<String>,
    folded: Vec<String>,
}

// spec: gate-sdk/SPEC.md §check-release-assets — the subject read: each attest step's
// `subject-path` under its `with:`, split on newlines and commas, a `#`-led line no entry
pub(crate) fn attest_subjects(text: &str) -> Vec<Subjects> {
    let lines = fresh::file_lines(text);
    let mut out = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let (col, rest) = key_line(l);
        match rest.strip_prefix("uses:") {
            Some(v) if is_attest_uses(v) => {}
            _ => continue,
        }
        let mut start = i;
        while start > 0 && !l.trim_start().starts_with("- ") {
            let prev = lines[start - 1];
            if opaque(prev) || indent(prev) >= col {
                start -= 1;
                continue;
            }
            if key_line(prev).0 == col && prev.trim_start().starts_with("- ") {
                start -= 1;
            }
            break;
        }
        let mut end = i + 1;
        while end < lines.len() && (opaque(lines[end]) || indent(lines[end]) >= col) {
            end += 1;
        }
        let mut step = Subjects::default();
        for w in start..end {
            let (wcol, wrest) = key_line(lines[w]);
            if wcol != col || wrest.trim_end() != "with:" {
                continue;
            }
            for k in w + 1..end {
                if !opaque(lines[k]) && indent(lines[k]) <= col {
                    break;
                }
                let (kcol, krest) = key_line(lines[k]);
                if let Some(v) = krest.strip_prefix("subject-path:") {
                    for line in key_value(&lines[..end], k, kcol, v) {
                        if line.starts_with('#') {
                            continue;
                        }
                        for e in line.split(',').map(|e| e.trim_matches(|c: char| c.is_whitespace() || c == FOLD)) {
                            match e {
                                "" => {}
                                e if e.contains(FOLD) => step.folded.push(e.replace(FOLD, " ")),
                                e => step.entries.push(e.to_string()),
                            }
                        }
                    }
                }
            }
        }
        out.push(step);
    }
    out
}

// spec: gate-sdk/SPEC.md §check-release-assets — an entry's last component against a template's
// text, the entry's `*` and the template's placeholders each standing as any run: true when some
// asset name both could spell
fn subject_matches(entry: &str, template: &str) -> bool {
    let glob = entry.rsplit('/').next().unwrap_or(entry).as_bytes();
    let mut tpl: Vec<Option<u8>> = Vec::new();
    let mut rest = template.as_bytes();
    while let Some(&b) = rest.first() {
        if let Some(r) = rest.strip_prefix(b"{version}").or_else(|| rest.strip_prefix(b"{target}")) {
            tpl.push(None);
            rest = r;
        } else {
            tpl.push(Some(b));
            rest = &rest[1..];
        }
    }
    let (gn, tn) = (glob.len(), tpl.len());
    let mut seen = vec![vec![false; tn + 1]; gn + 1];
    let mut stack = vec![(0usize, 0usize)];
    while let Some((g, t)) = stack.pop() {
        if seen[g][t] {
            continue;
        }
        seen[g][t] = true;
        let gstar = g < gn && glob[g] == b'*';
        let tany = t < tn && tpl[t].is_none();
        if gstar {
            stack.push((g + 1, t));
        }
        if tany {
            stack.push((g, t + 1));
        }
        match (g < gn, t < tn) {
            (true, true) if !gstar && !tany => {
                if Some(glob[g]) == tpl[t] {
                    stack.push((g + 1, t + 1));
                }
            }
            (true, true) if gstar && !tany => stack.push((g, t + 1)),
            (true, true) if !gstar && tany => stack.push((g + 1, t)),
            _ => {}
        }
    }
    seen[gn][tn]
}

// spec: gate-sdk/SPEC.md §check-release-assets — every asset other than a sidecar is attested, and a
// path folded from several lines is no subject; returns the templates some entry matched
fn attest_rule<'a>(workflows: &str, kept: &[&'a str], subjects: &Subjects, findings: &mut Vec<String>) -> Vec<&'a str> {
    for f in &subjects.folded {
        findings.push(format!(
            "{}: subject-path entry '{}' is folded from several lines into one path, which names no asset",
            workflows, f
        ));
    }
    let mut matched = Vec::new();
    for t in kept.iter().filter(|t| !t.ends_with(SIDECAR)) {
        if subjects.entries.iter().any(|e| subject_matches(e, t)) {
            matched.push(*t);
        } else {
            findings.push(format!(
                "{}: template '{}' is matched by no attest step's subject-path, so its asset ships unattested",
                workflows, t
            ));
        }
    }
    matched
}

// spec: gate-sdk/SPEC.md §check-release-assets — coverage at the tag: each name a matched template
// expands to is itself matched, so a glob narrowed to one target or version is a finding
fn coverage_rule(dir: &str, matched: &[&str], version: &str, targets: &[String], subjects: &Subjects, findings: &mut Vec<String>) -> usize {
    let names = expand(matched, version, targets);
    for name in &names {
        if !subjects.entries.iter().any(|e| subject_matches(e, name)) {
            findings.push(format!(
                "{}: asset '{}' is matched by no attest step's subject-path, so it ships unattested",
                dir, name
            ));
        }
    }
    names.len()
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

// spec: gate-sdk/SPEC.md §check-release-assets — one line, the stem's in-process digest and its
// bare name, the shape `sha256sum -c` reads beside the downloaded asset
fn sidecar_fault(dir: &str, sidecar: &str, stem: &str, text: &str, digest: &str) -> Option<String> {
    let line = text.strip_suffix('\n').unwrap_or(text);
    let hex_led = line.len() > 66 && line.as_bytes()[..64].iter().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b));
    let shaped = (hex_led && !line.contains('\n'))
        .then(|| line.split_at(64))
        .and_then(|(hex, rest)| rest.strip_prefix("  ").or_else(|| rest.strip_prefix(" *")).map(|n| (hex, n)));
    match shaped {
        None => Some(format!(
            "{}: sidecar '{}' is not one '<64 lowercase hex>  {}' line",
            dir, sidecar, stem
        )),
        Some((_, name)) if name != stem => Some(format!(
            "{}: sidecar '{}' names '{}', not its asset's bare name '{}'",
            dir, sidecar, name, stem
        )),
        Some((hex, _)) if hex != digest => Some(format!(
            "{}: sidecar '{}' carries digest {}, and '{}' hashes to {}",
            dir, sidecar, hex, stem, digest
        )),
        Some(_) => None,
    }
}

fn check_sidecars(dir: &str, present: &BTreeSet<&str>, findings: &mut Vec<String>) -> Result<usize, Fault> {
    let base = dir.trim_end_matches('/');
    let mut checked = 0usize;
    for sidecar in present.iter().filter(|n| n.ends_with(SIDECAR)) {
        let stem = &sidecar[..sidecar.len() - SIDECAR.len()];
        if !present.contains(stem) {
            continue;
        }
        let path = format!("{}/{}", base, sidecar);
        let text = std::fs::read(&path)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| Fault::input(format!("cannot read sidecar {}: {}", path, e)))?;
        let digest = sha256::file_hex(Path::new(&format!("{}/{}", base, stem))).map_err(Fault::input)?;
        findings.extend(sidecar_fault(dir, sidecar, stem, &text, &digest));
        checked += 1;
    }
    Ok(checked)
}

fn binary_archives(templates: &[&str], version: &str, targets: &[String]) -> BTreeSet<String> {
    let per_target: Vec<&str> = templates
        .iter()
        .copied()
        .filter(|t| t.contains("{target}") && t.ends_with(".tar.gz"))
        .collect();
    expand(&per_target, version, targets)
}

// spec: gate-sdk/SPEC.md §check-release-assets — the archive's binary is its one regular entry not
// named `*.sha256`, the packer's own discovery rule, and it ships at exactly `-rwxr-xr-x`
fn binary_mode_fault(dir: &str, archive: &str, listing: &str) -> Option<String> {
    let binaries: Vec<(&str, &str)> = listing
        .lines()
        .filter_map(|l| {
            let mut words = l.split_whitespace();
            let mode = words.next()?;
            let path = words.last()?;
            (mode.starts_with('-') && !path.ends_with(".sha256")).then_some((mode, path))
        })
        .collect();
    match binaries.as_slice() {
        [(mode, _)] if *mode == "-rwxr-xr-x" => None,
        [(mode, path)] => Some(format!(
            "{}: archive '{}' carries its binary '{}' at mode {}, not -rwxr-xr-x, so an adopter must chmod it before it runs",
            dir, archive, path, mode
        )),
        other => Some(format!(
            "{}: archive '{}' holds {} regular entr(ies) beside its sidecars, and a per-target archive holds exactly one binary",
            dir,
            archive,
            other.len()
        )),
    }
}

fn archive_listing(path: &str) -> Result<String, Fault> {
    let done = proc::run(&programs::TAR, &["-tvzf", path]).map_err(Fault::input)?;
    match done.stdout() {
        Some(out) => Ok(String::from_utf8_lossy(out).into_owned()),
        None => Err(Fault::input(format!(
            "cannot list archive {}: {}",
            path,
            done.failure_report().unwrap_or_default()
        ))),
    }
}

fn read(path: &str, what: &str) -> Result<String, Fault> {
    if !Path::new(path).is_file() {
        return Err(Fault::refused(format!("{} not found: {}", what, path)));
    }
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| Fault::input(format!("cannot read {} {}: {}", what, path, e)))
}

fn rule(args: &[String]) -> Result<i32, Fault> {
    let a = parse(args).map_err(Fault::usage)?;
    let doc_text = read(&a.doc, "declaring doc")?;
    let workflows = match &a.workflow {
        Some(w) => vec![w.clone()],
        None => walk::knob_words("GATE_SDK_NATIVE_PUBLISH_WORKFLOW").map_err(Fault::refused)?,
    };
    if workflows.is_empty() {
        return Err(Fault::refused("GATE_SDK_NATIVE_PUBLISH_WORKFLOW names no workflow"));
    }
    let mut wired = false;
    let mut steps = 0usize;
    let mut subjects = Subjects::default();
    for w in &workflows {
        let text = read(w, "publish workflow")?;
        wired |= fresh::file_lines(&text).iter().any(|l| is_call(l));
        for step in attest_subjects(&text) {
            steps += 1;
            subjects.entries.extend(step.entries);
            subjects.folded.extend(step.folded);
        }
    }

    let mut findings: Vec<String> = Vec::new();
    let templates = battery_half(&a.doc, &doc_text, &mut findings);
    sidecar_rules(&a.doc, &templates, &mut findings);
    let matched = attest_rule(&workflows.join(" "), &templates, &subjects, &mut findings);
    if !wired {
        findings.push(format!(
            "{}: no step invokes check-release-assets with --dist, so no tag's asset set is held",
            workflows.join(" ")
        ));
    }

    let mut dist_note = String::new();
    if let Some((dir, version)) = &a.dist {
        if let Some(fault) = version_fault(version) {
            return Err(Fault::usage(format!("{}: '{}'", fault, version)));
        }
        let entries = walk::list_dir(Path::new(dir)).map_err(Fault::input)?;
        let roster_path = walk::knob_scalar("GATE_SDK_NATIVE_TARGETS_FILE").map_err(Fault::refused)?;
        let targets = roster(&read(&roster_path, "target roster")?, &roster_path).map_err(Fault::input)?;
        let declared = expand(&templates, version, &targets);
        publish_half(dir, &declared, &entries, &mut findings);
        let attested = coverage_rule(dir, &matched, version, &targets, &subjects, &mut findings);
        let present: BTreeSet<&str> = entries.iter().filter(|(_, d)| !d).map(|(n, _)| n.as_str()).collect();
        let mut read_archives = 0usize;
        for archive in binary_archives(&templates, version, &targets) {
            if !present.contains(archive.as_str()) {
                continue;
            }
            let listing = archive_listing(&format!("{}/{}", dir.trim_end_matches('/'), archive))?;
            findings.extend(binary_mode_fault(dir, &archive, &listing));
            read_archives += 1;
        }
        let sidecars = check_sidecars(dir, &present, &mut findings)?;
        dist_note = format!(
            "; {} carries exactly the {} declared asset(s) for version {}, {} attested by name, {} archive binar(ies) at -rwxr-xr-x, {} sidecar(s) verified",
            dir,
            declared.len(),
            version,
            attested,
            read_archives,
            sidecars
        );
    }

    if !findings.is_empty() {
        println!("check-release-assets: the Release's asset set disagrees with its declaration (gate-sdk/SPEC.md §check-release-assets):");
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: keep exactly one well-formed release-assets line in §Consumer payload, declaring each asset's .sha256 beside it; name every asset other than a sidecar, for every target and version, in an attest step's subject-path, listed in a | block since a > block folds its lines into one path, and keep the --dist call in the publish workflow's pack job; make the pack output and the line name the same files, write each sidecar from the bytes it vouches for, and draw each archive's binary from the packed payload, whose packer sets its mode.");
        return Ok(1);
    }
    println!(
        "RELEASE-ASSETS: clean ({} template(s) declared in {}, the publish half wired in {}, {} attest step(s) read{})",
        templates.len(),
        a.doc,
        workflows.join(" "),
        steps,
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

    // spec: gate-sdk/SPEC.md §check-release-assets — only a per-target `.tar.gz` holds a binary
    #[test]
    fn only_a_per_target_tar_gz_is_read_for_its_binary() {
        let got = binary_archives(&["p-{version}.tgz", "g-{version}-{target}.tar.gz", "z-{version}-{target}.zip"], "1", &s(&["t"]));
        assert_eq!(got, s(&["g-1-t.tar.gz"]).into_iter().collect::<BTreeSet<String>>());
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — GNU's and bsdtar's listings both lead with the
    // mode and end with the path
    #[test]
    fn the_binary_entry_ships_at_exactly_0755() {
        let gnu = "drwxr-xr-x 0/0 0 2026-01-01 00:00 g/\n-rwxr-xr-x 0/0 5 2026-01-01 00:00 g/bin\n-rw-r--r-- 0/0 70 2026-01-01 00:00 g/bin.sha256\n";
        assert_eq!(binary_mode_fault("d", "g.tar.gz", gnu), None);
        let bsd = "-rwxr-xr-x  0 0      0           5 Jan  1  2026 g/bin.exe\n-rw-r--r--  0 0      0          70 Jan  1  2026 g/bin.exe.sha256\n";
        assert_eq!(binary_mode_fault("d", "g.tar.gz", bsd), None);
        let flat = gnu.replace("-rwxr-xr-x 0/0 5", "-rw-r--r-- 0/0 5");
        assert_eq!(
            binary_mode_fault("d", "g.tar.gz", &flat).unwrap(),
            "d: archive 'g.tar.gz' carries its binary 'g/bin' at mode -rw-r--r--, not -rwxr-xr-x, so an adopter must chmod it before it runs"
        );
        assert!(binary_mode_fault("d", "g.tar.gz", "-rwxr--r-- 0/0 5 x y g/bin\n").unwrap().contains("-rwxr--r--"));
        let none = "-rw-r--r-- 0/0 70 2026-01-01 00:00 g/bin.sha256\n";
        assert!(binary_mode_fault("d", "g.tar.gz", none).unwrap().contains("holds 0 regular"));
        let two = format!("{}-rwxr-xr-x 0/0 5 2026-01-01 00:00 g/other\n", gnu);
        assert!(binary_mode_fault("d", "g.tar.gz", &two).unwrap().contains("holds 2 regular"));
    }

    #[test]
    fn a_commented_or_flagless_line_is_no_call() {
        assert!(is_call("  bash gate-sdk/bin/run-gates.sh --only check-release-assets -- --dist \"$out\" \"$version\""));
        assert!(!is_call("  # bash gate-sdk/bin/run-gates.sh --only check-release-assets -- --dist x y"));
        assert!(!is_call("  bash gate-sdk/bin/run-gates.sh --only check-release-assets"));
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — a template with no sidecar template and an
    // orphan sidecar template are each a finding
    #[test]
    fn every_asset_carries_its_sidecar_template_and_every_sidecar_a_stem() {
        let mut f = Vec::new();
        sidecar_rules("d.md", &["a-{version}.tgz", "a-{version}.tgz.sha256", "b-{version}.zip", "c-{version}.sha256"], &mut f);
        assert_eq!(
            f,
            s(&[
                "d.md: template 'b-{version}.zip' has no sidecar template 'b-{version}.zip.sha256' declared beside it",
                "d.md: sidecar template 'c-{version}.sha256' vouches for 'c-{version}', which is not declared",
            ])
        );
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — the subject read: inline, a block, a comma
    // list, the `actions/attest` spelling, a `#`-led entry, and a step's later `uses:` key
    #[test]
    fn the_subject_read_takes_each_attest_step_s_entries() {
        let wf = "jobs:\n  pack:\n    steps:\n      - uses: actions/attest-build-provenance@abc # v4\n        with:\n          subject-path: ${{ runner.temp }}/dist/a-*.tgz\n      - uses: actions/attest@def\n        with:\n          subject-path: |\n            x/b-*.tar.gz\n            # x/z-*.zip\n            x/c-*.zip, y/d-*.zip\n          other: 1\n      - name: third\n        uses: \"actions/attest-build-provenance@abc\"\n        with:\n          subject-path: >-\n            e-*.zip\n      - uses: actions/attest-sbom@abc\n        with:\n          subject-path: n-*.zip\n      - uses: actions/upload-artifact@x\n        with:\n          path: p\n";
        let entries = |v: &[&str]| Subjects {
            entries: s(v),
            folded: Vec::new(),
        };
        assert_eq!(
            attest_subjects(wf),
            vec![
                entries(&["${{ runner.temp }}/dist/a-*.tgz"]),
                entries(&["x/b-*.tar.gz", "x/c-*.zip", "y/d-*.zip"]),
                entries(&["e-*.zip"]),
            ]
        );
        assert_eq!(attest_subjects("on: push\n"), Vec::<Subjects>::new());
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — a `>` block folds a paragraph's lines into one
    // path; a blank line or a comma still ends an entry
    #[test]
    fn a_folded_block_reads_as_the_one_path_yaml_makes_of_it() {
        let wf = "steps:\n  - uses: actions/attest@x\n    with:\n      subject-path: >\n        d/a-*.tgz\n        d/b-*.tgz\n\n        d/c-*.zip,\n        d/e-*.zip\n";
        assert_eq!(
            attest_subjects(wf),
            vec![Subjects {
                entries: s(&["d/c-*.zip", "d/e-*.zip"]),
                folded: s(&["d/a-*.tgz d/b-*.tgz"]),
            }]
        );
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — the entry's last component is matched, a `*`
    // and a placeholder each any run
    #[test]
    fn an_entry_matches_a_template_some_name_both_could_spell() {
        assert!(subject_matches("${{ runner.temp }}/dist/checkwright-*.tgz", "checkwright-{version}.tgz"));
        assert!(subject_matches("checkwright-gates-*.tar.gz", "checkwright-gates-{version}-{target}.tar.gz"));
        assert!(subject_matches("dist/checkwright-0.*.tgz", "checkwright-{version}.tgz"));
        assert!(subject_matches("checkwright-1.0.tgz", "checkwright-{version}.tgz"));
        assert!(!subject_matches("checkwright-*.tgz", "checkwright-gates-{version}-{target}.tar.gz"));
        assert!(!subject_matches("checkwright-*.tgz", "checkwright-companion-{version}.zip"));
        assert!(!subject_matches("checkwright-gates-*.tar.gz", "checkwright-{version}.tgz"));
        assert!(!subject_matches("other-*.tgz", "checkwright-{version}.tgz"));
    }

    #[test]
    fn a_non_sidecar_template_no_entry_matches_is_a_finding() {
        let mut f = Vec::new();
        let subjects = Subjects {
            entries: s(&["d/a-*.tgz"]),
            folded: s(&["d/x-*.tgz d/y-*.tgz"]),
        };
        let matched = attest_rule("w.yml", &["a-{version}.tgz", "a-{version}.tgz.sha256", "g-{version}-{target}.tar.gz"], &subjects, &mut f);
        assert_eq!(matched, vec!["a-{version}.tgz"]);
        assert_eq!(
            f,
            s(&[
                "w.yml: subject-path entry 'd/x-*.tgz d/y-*.tgz' is folded from several lines into one path, which names no asset",
                "w.yml: template 'g-{version}-{target}.tar.gz' is matched by no attest step's subject-path, so its asset ships unattested",
            ])
        );
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — a glob narrowed to one target or one version
    // overlaps its template and still leaves an expanded name unattested
    #[test]
    fn coverage_names_each_expanded_asset_no_entry_matches() {
        let mut f = Vec::new();
        let subjects = Subjects {
            entries: s(&["d/g-*-t1.tar.gz", "d/a-1.0.*.tgz"]),
            folded: Vec::new(),
        };
        let checked = coverage_rule("out", &["g-{version}-{target}.tar.gz", "a-{version}.tgz"], "2.0.0", &s(&["t1", "t2"]), &subjects, &mut f);
        assert_eq!(checked, 3);
        assert_eq!(
            f,
            s(&[
                "out: asset 'a-2.0.0.tgz' is matched by no attest step's subject-path, so it ships unattested",
                "out: asset 'g-2.0.0-t2.tar.gz' is matched by no attest step's subject-path, so it ships unattested",
            ])
        );
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — a sidecar holds one line naming its stem bare
    // with the stem's digest; `*` binary mode is admitted
    #[test]
    fn a_sidecar_holds_one_line_its_stem_s_digest_and_bare_name() {
        let d = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(sidecar_fault("o", "a.tgz.sha256", "a.tgz", &format!("{}  a.tgz\n", d), d), None);
        assert_eq!(sidecar_fault("o", "a.tgz.sha256", "a.tgz", &format!("{} *a.tgz", d), d), None);
        assert!(sidecar_fault("o", "a.tgz.sha256", "a.tgz", "", d).unwrap().contains("is not one"));
        assert!(sidecar_fault("o", "a.tgz.sha256", "a.tgz", &format!("{}  a.tgz\n{}  a.tgz\n", d, d), d).unwrap().contains("is not one"));
        assert!(sidecar_fault("o", "a.tgz.sha256", "a.tgz", &format!("{}  a.tgz\n", d.to_uppercase()), d).unwrap().contains("is not one"));
        assert!(sidecar_fault("o", "a.tgz.sha256", "a.tgz", &format!("{} a.tgz\n", d), d).unwrap().contains("is not one"));
        assert_eq!(
            sidecar_fault("o", "a.tgz.sha256", "a.tgz", &format!("{}  dist/a.tgz\n", d), d).unwrap(),
            "o: sidecar 'a.tgz.sha256' names 'dist/a.tgz', not its asset's bare name 'a.tgz'"
        );
        let other = "0".repeat(64);
        assert!(sidecar_fault("o", "a.tgz.sha256", "a.tgz", &format!("{}  a.tgz\n", other), d).unwrap().contains("carries digest 000"));
        assert!(sidecar_fault("o", "a.sha256", "a", &format!("{}  x\n", "\u{e9}".repeat(40)), d).unwrap().contains("is not one"));
    }

    // spec: gate-sdk/SPEC.md §check-release-assets — every exit-2 and exit-3 path
    #[test]
    fn each_refusal_exits_two_and_each_unread_input_three() {
        let _knobs = crate::knobenv::lock();
        assert_eq!(run(&s(&["no/such/doc.md", "no/such/wf.yml"])), crate::exit::REFUSED);
        assert_eq!(run(&s(&["Cargo.toml", "no/such/wf.yml"])), crate::exit::REFUSED);
        assert_eq!(run(&s(&["Cargo.toml", "Cargo.toml", "--dist", "no/such/dir", "1.0.0"])), crate::exit::INPUT);
        assert_eq!(run(&s(&["Cargo.toml", "Cargo.toml", "--dist", "src", ""])), 2);
        assert_eq!(run(&s(&["Cargo.toml", "Cargo.toml", "--dist", "src", "1.0 .0"])), 2);
        assert_eq!(run(&s(&["Cargo.toml", "Cargo.toml", "--dist", "src", "1/0"])), 2);
        assert_eq!(run(&s(&["--dist", "src"])), 2);
        assert_eq!(run(&s(&["--bogus"])), 2);
        assert_eq!(run(&s(&["a", "b", "c"])), 2);
        assert_eq!(archive_listing("Cargo.toml").err().map(|e| e.status()), Some(crate::exit::INPUT));
        let present: BTreeSet<&str> = ["src", "src.sha256"].into_iter().collect();
        assert_eq!(check_sidecars(".", &present, &mut Vec::new()).err().map(|e| e.status()), Some(crate::exit::INPUT));
        assert!(roster("# only a comment\n\n", "r").is_err());
        assert_eq!(roster("# c\nt1\n", "r").unwrap(), s(&["t1"]));
    }
}
