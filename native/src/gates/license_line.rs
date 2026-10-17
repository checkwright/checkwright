// spec: docs/site-architecture.md §The license line — every README's License section and the
// site footer carry the one line the source renders for that site, and every package manifest's
// license field the source's license
use crate::fresh;
use crate::walk;
use std::path::Path;

const NAME: &str = "check-license-line";
const LEAD: &str = "Licensed under";

#[derive(Debug, PartialEq)]
enum Form {
    Link(String),
    At(String),
}

#[derive(Debug)]
struct Conf {
    license: String,
    text: String,
    pages: Option<String>,
    wheres: Vec<(String, String)>,
    sites: Vec<(String, Form)>,
    manifests: Vec<(String, Kind)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Kind {
    Cargo,
    Json,
    Yaml,
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {} — the check could not run; treating as failure (not clean)", NAME, e);
            2
        }
    }
}

fn parse(path: &str, text: &str) -> Result<Conf, String> {
    let mut license: Option<String> = None;
    let mut lic_text: Option<String> = None;
    let mut pages: Option<String> = None;
    let mut wheres: Vec<(String, String)> = Vec::new();
    let mut sites: Vec<(String, Form)> = Vec::new();
    let mut manifests: Vec<(String, Kind)> = Vec::new();
    for (n, line) in fresh::file_lines(text).into_iter().enumerate() {
        if !fresh::live_line(line) {
            continue;
        }
        let at = format!("{}:{}", path, n + 1);
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "license" | "text" | "pages" => {
                if words.len() != 2 {
                    return Err(format!("{}: '{}' takes one value", at, words[0]));
                }
                let slot = match words[0] {
                    "license" => &mut license,
                    "text" => &mut lic_text,
                    _ => &mut pages,
                };
                if slot.is_some() {
                    return Err(format!("{}: '{}' repeated", at, words[0]));
                }
                *slot = Some(words[1].to_string());
            }
            "where" => {
                if words.len() < 3 {
                    return Err(format!("{}: 'where' takes a name and a clause", at));
                }
                if wheres.iter().any(|(w, _)| w == words[1]) {
                    return Err(format!("{}: where '{}' declared twice", at, words[1]));
                }
                wheres.push((words[1].to_string(), words[2..].join(" ")));
            }
            "site" => {
                if words.len() != 4 {
                    return Err(format!("{}: 'site' takes a path, a form and its value", at));
                }
                let form = match words[2] {
                    "link" => Form::Link(words[3].to_string()),
                    "at" => Form::At(words[3].to_string()),
                    other => {
                        return Err(format!("{}: site form '{}' is neither 'link' nor 'at'", at, other))
                    }
                };
                if sites.iter().any(|(s, _)| s == words[1]) {
                    return Err(format!("{}: site '{}' declared twice", at, words[1]));
                }
                sites.push((words[1].to_string(), form));
            }
            "manifest" => {
                if words.len() != 2 {
                    return Err(format!("{}: 'manifest' takes one path", at));
                }
                let Some(k) = kind(words[1]) else {
                    return Err(format!(
                        "{}: manifest '{}' is of a kind with no reader (Cargo.toml, .json, .yml, .yaml)",
                        at, words[1]
                    ));
                };
                if manifests.iter().any(|(m, _)| m == words[1]) {
                    return Err(format!("{}: manifest '{}' declared twice", at, words[1]));
                }
                manifests.push((words[1].to_string(), k));
            }
            other => {
                return Err(format!(
                    "{}: unknown key '{}' (admitted: license, text, pages, where, site, manifest)",
                    at, other
                ))
            }
        }
    }
    for (site, form) in &sites {
        if let Form::At(name) = form {
            if !wheres.iter().any(|(w, _)| w == name) {
                return Err(format!("{}: site '{}' names the undeclared where '{}'", path, site, name));
            }
        }
    }
    Ok(Conf {
        license: license.ok_or_else(|| format!("{}: key 'license' missing", path))?,
        text: lic_text.ok_or_else(|| format!("{}: key 'text' missing", path))?,
        pages,
        wheres,
        sites,
        manifests,
    })
}

fn kind(path: &str) -> Option<Kind> {
    let name = path.rsplit('/').next().unwrap_or(path);
    if name == "Cargo.toml" {
        Some(Kind::Cargo)
    } else if name.ends_with(".json") {
        Some(Kind::Json)
    } else if name.ends_with(".yml") || name.ends_with(".yaml") {
        Some(Kind::Yaml)
    } else {
        None
    }
}

// spec: docs/site-architecture.md §The license line — a manifest's license field read by its kind;
// `Ok(None)` is a manifest carrying no field, `Err` one the gate cannot read
fn manifest_license(kind: Kind, text: &str) -> Result<Option<String>, String> {
    match kind {
        Kind::Cargo => {
            let lines = fresh::file_lines(text);
            let Some(start) = lines.iter().position(|l| l.trim() == "[package]") else {
                return Ok(None);
            };
            for line in lines[start + 1..].iter().map(|l| l.trim()) {
                if line.starts_with('[') {
                    break;
                }
                let Some((key, value)) = line.split_once('=') else {
                    continue;
                };
                if key.trim() != "license" {
                    continue;
                }
                let value = value.trim();
                return match value.strip_prefix('"').and_then(|v| v.split_once('"')) {
                    Some((lic, rest)) if rest.trim().is_empty() || rest.trim().starts_with('#') => {
                        Ok(Some(lic.to_string()))
                    }
                    _ => Err(format!("the [package] license value {} is not a basic string", value)),
                };
            }
            Ok(None)
        }
        Kind::Json => {
            let doc: serde_json::Value =
                serde_json::from_str(text).map_err(|e| format!("does not parse as JSON ({})", e))?;
            match doc.get("license") {
                None => Ok(None),
                Some(serde_json::Value::String(s)) => Ok(Some(s.clone())),
                Some(other) => Err(format!("the top-level license value {} is not a string", other)),
            }
        }
        Kind::Yaml => {
            for line in fresh::file_lines(text) {
                let Some((key, value)) = line.trim_start().split_once(':') else {
                    continue;
                };
                if key.trim_end() != "license" {
                    continue;
                }
                let value = value.trim();
                if value.is_empty() {
                    return Err("the license key carries no scalar value".to_string());
                }
                let unquoted = ['"', '\'']
                    .iter()
                    .find_map(|q| value.strip_prefix(*q).and_then(|v| v.strip_suffix(*q)))
                    .unwrap_or(value);
                return Ok(Some(unquoted.to_string()));
            }
            Ok(None)
        }
    }
}

fn is_html(site: &str) -> bool {
    site.ends_with(".html")
}

fn render(conf: &Conf, site: &str, form: &Form) -> Result<String, String> {
    match form {
        Form::Link(target) if is_html(site) => Ok(format!(
            "{} <a href=\"{}\">{}</a>.",
            LEAD, target, conf.license
        )),
        Form::Link(target) => Ok(format!("{} [{}]({}).", LEAD, conf.license, target)),
        Form::At(_) if is_html(site) => Err(format!(
            "{}: an .html site takes the 'link' form, never 'at'",
            site
        )),
        Form::At(name) => {
            let clause = conf
                .wheres
                .iter()
                .find(|(w, _)| w == name)
                .map(|(_, c)| c.as_str())
                .unwrap_or_default();
            Ok(format!(
                "{} {}, with the text in `{}` {}.",
                LEAD, conf.license, conf.text, clause
            ))
        }
    }
}

// spec: docs/site-architecture.md §The license line — a section runs from its heading to the next
// heading of level one or two, or the end of the file; `None` when the heading is absent
fn license_section<'a>(text: &'a str, heading: &str) -> Option<Vec<&'a str>> {
    let lines = fresh::file_lines(text);
    let start = lines.iter().position(|l| l.trim_end() == heading)?;
    Some(
        lines[start + 1..]
            .iter()
            .take_while(|l| !(l.starts_with("# ") || l.starts_with("## ")))
            .map(|l| l.trim_end())
            .filter(|l| !l.trim().is_empty())
            .collect(),
    )
}

fn generated(text: &str) -> bool {
    let lines = fresh::file_lines(text);
    let span = crate::spec::front_matter_span(&lines);
    lines[..span].iter().any(|l| l.trim() == "generated: true")
}

// spec: docs/site-architecture.md §The license line — a walked README is compared to the source's
// `/`-spelled site paths by its normal components, so the host's separator never decides coverage
fn site_key(p: &Path) -> String {
    p.components()
        .filter_map(|c| match c {
            std::path::Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn rule(args: &[String]) -> Result<i32, String> {
    let source = fresh::positional_or_knob(args, 0, "GATE_LOCAL_LICENSE_CONF")?;
    let source = source.as_str();
    let root = fresh::strip_trailing_slash(fresh::positional(args, 1, ".")).to_string();
    if !fresh::is_dir(&root) {
        return Err(format!("root not found: {}", root));
    }
    let conf = parse(
        source,
        &fresh::read_captured(source).map_err(|e| format!("{}: {}", source, e))?,
    )?;
    let heading = walk::knob_scalar("GATE_LOCAL_LICENSE_HEADING")?;
    let heading = heading.as_str();
    let readme = walk::knob_scalar("GATE_LOCAL_README_FILE")?;
    let at_root = |p: &str| Path::new(&root).join(p).display().to_string();

    let mut findings: Vec<String> = Vec::new();
    for (site, form) in &conf.sites {
        let want = render(&conf, site, form)?;
        let text = fresh::read_captured(&at_root(site)).map_err(|e| format!("{}: {}", site, e))?;
        let got: Vec<&str> = if is_html(site) {
            let lines: Vec<&str> = fresh::file_lines(&text)
                .into_iter()
                .map(str::trim)
                .filter(|l| l.starts_with(LEAD))
                .collect();
            if lines.len() != 1 {
                findings.push(format!(
                    "  {}: carries {} '{}' line(s), not one",
                    site,
                    lines.len(),
                    LEAD
                ));
                continue;
            }
            lines
        } else {
            match license_section(&text, heading) {
                None => {
                    findings.push(format!("  {}: no '{}' section", site, heading));
                    continue;
                }
                Some(lines) if lines.len() != 1 => {
                    findings.push(format!(
                        "  {}: the '{}' section carries {} line(s), not one",
                        site,
                        heading,
                        lines.len()
                    ));
                    continue;
                }
                Some(lines) => lines,
            }
        };
        if got[0] != want {
            findings.push(format!("  {}: the license line differs from its rendering", site));
            findings.push(format!("    expected: {}", want));
        }
    }

    let mut covered = 0usize;
    for p in walk::find_named(Path::new(&root), &[readme.as_str()])? {
        let rel = site_key(p.strip_prefix(&root).unwrap_or(&p));
        let text = fresh::read_captured(&p.display().to_string()).map_err(|e| format!("{}: {}", rel, e))?;
        if generated(&text) || !fresh::file_lines(&text).iter().any(|l| l.trim_end() == heading) {
            continue;
        }
        covered += 1;
        if !conf.sites.iter().any(|(s, _)| *s == rel) {
            findings.push(format!(
                "  {}: carries '{}' but is not a declared site in {}",
                rel, heading, source
            ));
        }
    }

    // spec: docs/site-architecture.md §The license line — the site chrome states the license on
    // every page, so a hand-written page under `pages` carrying the heading repeats it; a README
    // there is the coverage walk's above
    let mut pages = 0usize;
    if let Some(dir) = &conf.pages {
        for p in walk::find_files(&Path::new(&root).join(dir), &["md"])? {
            let rel = site_key(p.strip_prefix(&root).unwrap_or(&p));
            if p.file_name().is_some_and(|n| n == readme.as_str()) {
                continue;
            }
            let text = fresh::read_captured(&p.display().to_string()).map_err(|e| format!("{}: {}", rel, e))?;
            if generated(&text) {
                continue;
            }
            pages += 1;
            if license_section(&text, heading).is_some() {
                findings.push(format!(
                    "  {}: a page under {} carries '{}', which the site chrome already states",
                    rel, dir, heading
                ));
            }
        }
    }

    let read_manifest = |rel: &str, k: Kind| -> Result<Option<String>, String> {
        let text = fresh::read_captured(&at_root(rel)).map_err(|e| format!("{}: {}", rel, e))?;
        manifest_license(k, &text).map_err(|e| format!("{}: {}", rel, e))
    };
    for (m, k) in &conf.manifests {
        match read_manifest(m, *k)? {
            None => findings.push(format!("  {}: carries no license field", m)),
            Some(got) if got != conf.license => {
                findings.push(format!("  {}: the license field reads '{}'", m, got));
                findings.push(format!("    expected: {}", conf.license));
            }
            Some(_) => {}
        }
    }
    let manifest_names = walk::knob_array("GATE_LOCAL_LICENSE_MANIFEST_NAMES")?;
    let manifest_names: Vec<&str> = manifest_names.iter().map(String::as_str).collect();
    for p in walk::find_named(Path::new(&root), &manifest_names)? {
        let rel = site_key(p.strip_prefix(&root).unwrap_or(&p));
        let k = kind(&rel).ok_or_else(|| {
            format!(
                "{}: GATE_LOCAL_LICENSE_MANIFEST_NAMES names a kind with no reader (Cargo.toml, .json, .yml, .yaml)",
                rel
            )
        })?;
        if read_manifest(&rel, k)?.is_some() && !conf.manifests.iter().any(|(m, _)| *m == rel) {
            findings.push(format!(
                "  {}: carries a license field but is not a declared manifest in {}",
                rel, source
            ));
        }
    }

    if !findings.is_empty() {
        println!("{}: license line(s) out of step with {}:", NAME, source);
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: paste each expected line as its site's one license line, declare every README");
        println!("        carrying the heading in {}, and drop a page's own License section; a", source);
        println!("        manifest's license field takes the declared license, and every manifest carrying");
        println!("        one is declared (docs/site-architecture.md §The license line).");
        return Ok(1);
    }
    println!(
        "LICENSE-LINE: clean ({} site(s) carry the license line; {} README(s) declared; {} page(s) carry none; {} manifest(s) hold the license)",
        conf.sites.len(),
        covered,
        pages,
        conf.manifests.len()
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conf() -> Conf {
        parse(
            "t.conf",
            "license Apache-2.0\ntext LICENSE\nwhere root at the repository root\n\
             site README.md link LICENSE\nsite k/README.md at root\nsite f.html link https://x/LICENSE\n",
        )
        .unwrap()
    }

    #[test]
    fn each_form_renders_its_one_sentence() {
        let c = conf();
        assert_eq!(render(&c, "README.md", &c.sites[0].1).unwrap(), "Licensed under [Apache-2.0](LICENSE).");
        assert_eq!(
            render(&c, "k/README.md", &c.sites[1].1).unwrap(),
            "Licensed under Apache-2.0, with the text in `LICENSE` at the repository root."
        );
        assert_eq!(
            render(&c, "f.html", &c.sites[2].1).unwrap(),
            "Licensed under <a href=\"https://x/LICENSE\">Apache-2.0</a>."
        );
        assert!(render(&c, "f.html", &Form::At("root".into())).is_err());
    }

    #[test]
    fn a_malformed_source_is_refused() {
        for bad in [
            "text LICENSE\n",
            "license A\nlicense B\ntext LICENSE\n",
            "license A\ntext LICENSE\nbogus x\n",
            "license A\ntext LICENSE\nwhere a x\nwhere a y\n",
            "license A\ntext LICENSE\nsite R.md at nowhere\n",
            "license A\ntext LICENSE\nsite R.md link L\nsite R.md link L\n",
            "license A\ntext LICENSE\nsite R.md beside L\n",
            "license A\ntext LICENSE\npages d\npages e\n",
            "license A\ntext LICENSE\nmanifest a/Cargo.toml\nmanifest a/Cargo.toml\n",
            "license A\ntext LICENSE\nmanifest setup.cfg\n",
            "license A\ntext LICENSE\nmanifest a/package.json extra\n",
        ] {
            assert!(parse("t.conf", bad).is_err(), "accepted: {:?}", bad);
        }
    }

    #[test]
    fn a_declared_manifest_keeps_its_kind() {
        let c = parse(
            "t.conf",
            "license A\ntext LICENSE\nmanifest n/Cargo.toml\nmanifest p/package.json\nmanifest e/extension.yml\n",
        )
        .unwrap();
        assert_eq!(
            c.manifests,
            vec![
                ("n/Cargo.toml".to_string(), Kind::Cargo),
                ("p/package.json".to_string(), Kind::Json),
                ("e/extension.yml".to_string(), Kind::Yaml),
            ]
        );
    }

    #[test]
    fn a_cargo_manifest_is_read_for_its_package_tables_license() {
        let read = |t: &str| manifest_license(Kind::Cargo, t);
        assert_eq!(
            read("[package]\nname = \"x\"\nlicense = \"MIT\" # note\n[dependencies]\n").unwrap(),
            Some("MIT".into())
        );
        assert_eq!(read("[package]\nname = \"x\"\n[other]\nlicense = \"MIT\"\n").unwrap(), None);
        assert_eq!(read("[package]\nlicense-file = \"L\"\n").unwrap(), None);
        assert_eq!(read("[workspace]\nmembers = []\n").unwrap(), None);
        assert!(read("[package]\nlicense.workspace = true\nlicense = 3\n").is_err());
    }

    #[test]
    fn a_json_manifest_is_read_for_its_top_level_license_string() {
        let read = |t: &str| manifest_license(Kind::Json, t);
        assert_eq!(read("{\"license\": \"Apache-2.0\"}").unwrap(), Some("Apache-2.0".into()));
        assert_eq!(read("{\"plugins\": [{\"license\": \"MIT\"}]}").unwrap(), None);
        assert!(read("{\"license\": {\"type\": \"MIT\"}}").is_err());
        assert!(read("{\"license\": ").is_err());
    }

    #[test]
    fn a_yaml_manifest_is_read_for_its_first_license_key_unquoted() {
        let read = |t: &str| manifest_license(Kind::Yaml, t);
        assert_eq!(read("a:\n  license: \"Apache-2.0\"\nlicense: MIT\n").unwrap(), Some("Apache-2.0".into()));
        assert_eq!(read("license: 'MIT'\n").unwrap(), Some("MIT".into()));
        assert_eq!(read("license: MIT\n").unwrap(), Some("MIT".into()));
        assert_eq!(read("# license: MIT\nname: x\n").unwrap(), None);
        assert!(read("license:\n  name: MIT\n").is_err());
    }

    #[test]
    fn the_section_ends_at_the_next_heading_and_drops_blanks() {
        let text = "# T\n\n## License\n\nLine one.\n\n### Sub\nmore\n## Next\nafter\n";
        assert_eq!(license_section(text, "## License").unwrap(), vec!["Line one.", "### Sub", "more"]);
        assert_eq!(license_section("# T\n## Other\n", "## License"), None);
    }

    #[test]
    fn only_a_leading_front_matter_block_marks_a_mirror_generated() {
        assert!(generated("---\ntitle: README\ngenerated: true\n---\n# x\n"));
        assert!(!generated("# x\n---\ngenerated: true\n---\n"));
        assert!(!generated("---\ntitle: README\n---\ngenerated: true\n"));
        assert!(generated("---\ntitle: README\ngenerated: true\n...\n# x\n"));
        assert!(!generated("---\ntitle: README\n...\ngenerated: true\n---\n"));
        assert!(!generated("---\ntitle: README\ngenerated: true\n"));
    }
}
