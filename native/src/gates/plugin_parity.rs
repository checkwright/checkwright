// spec: plugin/SPEC.md §check-plugin-parity — the plugin package matches this repository: its two
// manifests (A), its skills (B), its hooks (C), the marketplace pin (D), the pinned release (E) and
// the license text (F)
use super::install_pin::{is_triple, pin_of};
use super::pinned_release::{self, read, Disposition};
use super::skill_binding::template_of;
use crate::{fresh, walk};
use serde_json::{Map, Value};
use std::collections::BTreeSet;
use std::path::Path;

const NAME: &str = "check-plugin-parity";
// consumer-value-exempt: the plugin package's license file name, a file under the plugin directory
const PLUGIN_LICENSE: &str = "LICENSE";
const HOOKS_TEMPLATE: &str = "guard-kit/templates/settings-hooks.json";
const PORTABLE: &str = "plugin.json";
const HARNESS: &str = ".claude-plugin/plugin.json";
const HOOKS: &str = "hooks/hooks.json";
const INSTALL_SKILL: &str = "install";
// spec: plugin/SPEC.md §The manifests — the fields a listing shows, equal across the two manifests
const SHARED: &[&str] = &["name", "description", "author", "homepage", "repository", "license"];
const SCHEMA_LEAD: &str = "https://agent-plugins.org/schemas/";
const SCHEMA_TAIL: &str = "/plugin.schema.json";
const REPO_HOST: &str = "https://github.com/";
// spec: plugin/SPEC.md §The guards — the fail-open prefix every rendered hook command carries
// spec: gate-sdk/SPEC.md §The path-dialect contract — shell text the hook's own shell runs, so the
// toplevel it names is produced and read there, never in the crate
const FAIL_OPEN: &str = "r=$(git -C \"${CLAUDE_PROJECT_DIR:-.}\" rev-parse --show-toplevel 2>/dev/null) && test -f \"$r/gate-sdk/bin/run-gates.sh\" || exit 0; CLAUDE_PROJECT_DIR=$r; ";
// spec: plugin/SPEC.md §The skills — the Agent Skills bound on a description's length
const DESCRIPTION_MAX: usize = 1024;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: plugin/SPEC.md §The skills — the one fixed body, its only variable the template path
pub(crate) fn skill_body(path: &str) -> String {
    format!(
        "Execute the template at `{p}` in this repository.\n\
         \n\
         - If this repository has a skill of its own that binds `{p}`, run that skill instead.\n\
         - Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.\n\
         - If `{p}` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.\n",
        p = path
    )
}

// spec: plugin/SPEC.md §The guards — each block under `hooks` in order, each command behind the
// fail-open prefix, and the template's `//` note dropped
pub(crate) fn render_hooks(template: &Value) -> Result<Value, String> {
    let blocks = template
        .get("hooks")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{} carries no 'hooks' object", HOOKS_TEMPLATE))?;
    let mut out = Map::new();
    for (event, list) in blocks {
        let mut list = list.clone();
        for block in list.as_array_mut().into_iter().flatten() {
            let hooks = block.get_mut("hooks").and_then(Value::as_array_mut);
            for hook in hooks.into_iter().flatten() {
                if let Some(Value::String(c)) = hook.get_mut("command") {
                    *c = format!("{}{}", FAIL_OPEN, c);
                }
            }
        }
        out.insert(event.clone(), list);
    }
    let mut root = Map::new();
    root.insert("hooks".to_string(), Value::Object(out));
    Ok(Value::Object(root))
}

fn json(path: &str) -> Result<Value, String> {
    serde_json::from_str(&read(path)?).map_err(|e| format!("{} is not JSON: {}", path, e))
}

fn show(v: Option<&Value>) -> String {
    v.map(Value::to_string).unwrap_or_else(|| "absent".to_string())
}

// spec: plugin/SPEC.md §The skills — front matter between two `---` lines, read as `key: value`
// scalars, and the body after it with its leading blank lines dropped
fn front_matter(text: &str) -> Option<(Vec<(String, String)>, String)> {
    let rest = text.strip_prefix("---\n")?;
    let end = rest.find("\n---\n")?;
    let keys = rest[..end]
        .lines()
        .filter_map(|l| l.split_once(':'))
        .map(|(k, v)| {
            let v = v.trim();
            let v = v
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| v.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
                .unwrap_or(v);
            (k.trim().to_string(), v.to_string())
        })
        .collect();
    Some((keys, rest[end + 5..].trim_start_matches('\n').to_string()))
}

enum Pinned {
    Unresolved,
    Carries,
    Lacks,
}

fn pinned_answer(path: &str) -> Result<Pinned, String> {
    if path == "-" {
        return Ok(Pinned::Unresolved);
    }
    match read(path)?.split_whitespace().next() {
        Some("carries") => Ok(Pinned::Carries),
        Some("lacks") => Ok(Pinned::Lacks),
        _ => Err(format!("{}: the pinned answer is neither 'carries' nor 'lacks'", path)),
    }
}

// spec: plugin/SPEC.md §check-plugin-parity — the binding shims' names, each with its template
fn shims(dir: &str) -> Result<Vec<(String, String)>, String> {
    if !Path::new(dir).is_dir() {
        return Err(format!("skills dir not found: {}", dir));
    }
    let mut out = Vec::new();
    for f in walk::glob_files(Path::new(dir), &["*.md".to_string()])? {
        let text = read(&f.to_string_lossy())?;
        if let Some(t) = template_of(&text) {
            let stem = f.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
            out.push((stem.to_string(), t.to_string()));
        }
    }
    Ok(out)
}

// spec: plugin/SPEC.md §check-plugin-parity — the marketplace pinned to the hosted install pin (D),
// and the pinned release carrying the package or pending this iteration's release (E)
fn publication(
    plugin: &str,
    marketplace: &str,
    portable: &Value,
    install_sh: &str,
    (disposition_path, queue_path): (&str, &str),
    args: &[String],
    findings: &mut Vec<String>,
) -> Result<String, String> {
    let pin = pin_of(install_sh, &fresh::read_captured(install_sh)?, "pin")?;
    let tag = format!("v{}", pin);
    let market = json(marketplace)?;
    let entries = market.get("plugins").and_then(Value::as_array).cloned().unwrap_or_default();
    let slug = portable
        .get("repository")
        .and_then(Value::as_str)
        .map(|r| r.strip_prefix(REPO_HOST).unwrap_or(r).to_string());
    if entries.len() != 1 {
        findings.push(format!("  {}: `plugins` carries {} entr(ies); expected exactly one", marketplace, entries.len()));
    } else {
        let e = &entries[0];
        let src = e.get("source");
        let expect: [(&str, Option<&Value>, Value); 6] = [
            ("name", e.get("name"), portable.get("name").cloned().unwrap_or(Value::Null)),
            ("source.source", src.and_then(|s| s.get("source")), Value::from("git-subdir")),
            ("source.path", src.and_then(|s| s.get("path")), Value::from(plugin)),
            ("source.url", src.and_then(|s| s.get("url")), slug.map(Value::from).unwrap_or(Value::Null)),
            ("source.ref", src.and_then(|s| s.get("ref")), Value::from(tag.clone())),
            ("version", e.get("version"), Value::from(pin.clone())),
        ];
        for (field, got, want) in expect {
            if got != Some(&want) {
                findings.push(format!("  {}: `{}` is {}; expected {}", marketplace, field, show(got), want));
            }
        }
    }

    let pinned = if !args.is_empty() {
        pinned_answer(&args[7])?
    } else if pinned_release::pinned_tag()?.is_none() {
        Pinned::Unresolved
    } else if pinned_release::carries(&tag, &format!("{}/{}", plugin, HARNESS))? {
        Pinned::Carries
    } else {
        Pinned::Lacks
    };
    let (iteration, disp) = pinned_release::iteration_disposition(disposition_path, queue_path)?;
    let state = match pinned {
        Pinned::Unresolved => format!("E dormant — {} does not resolve here", tag),
        Pinned::Carries => format!("{} carries the package", tag),
        Pinned::Lacks => match &disp {
            Disposition::Absent | Disposition::Release => format!("pending release: {} lacks the package", tag),
            Disposition::Withheld(field) => {
                findings.push(format!(
                    "  {}: the pinned release {} carries no {}/{} while iteration {}'s disposition is {}",
                    marketplace, tag, plugin, HARNESS, iteration, field
                ));
                String::new()
            }
        },
    };
    Ok(format!("marketplace pinned to {}, {}", tag, state))
}

fn rule(args: &[String]) -> Result<i32, String> {
    let positional = !args.is_empty();
    if positional && args.len() != 9 {
        return Err("usage: check-plugin-parity [plugin-dir marketplace skills-dir hooks-template install-sh disposition queue pinned license]".to_string());
    }
    let arg = |i: usize, d: &str| if positional { args[i].clone() } else { d.to_string() };
    let knob = |i: usize, k: &str| if positional { Ok(args[i].clone()) } else { walk::knob_scalar(k) };
    let plugin = knob(0, "GATE_LOCAL_PLUGIN_DIR")?;
    let marketplace = knob(1, "GATE_LOCAL_MARKETPLACE_FILE")?;
    let skills_dir = if positional { args[2].clone() } else { walk::knob_scalar("LIFECYCLE_KIT_SKILLS_DIR")? };
    let hooks_template = arg(3, HOOKS_TEMPLATE);
    let install_sh = knob(4, "GATE_LOCAL_INSTALL_SH")?;
    let root_license = knob(8, "GATE_SDK_PAYLOAD_LICENSE")?;
    let (disposition_path, queue_path) = if positional {
        (args[5].clone(), args[6].clone())
    } else {
        pinned_release::live_paths()?
    };

    let mut findings: Vec<String> = Vec::new();

    let portable_path = format!("{}/{}", plugin, PORTABLE);
    let harness_path = format!("{}/{}", plugin, HARNESS);
    let portable = json(&portable_path)?;
    let harness = json(&harness_path)?;
    for field in SHARED {
        let (p, h) = (portable.get(*field), harness.get(*field));
        if p.is_none() || p != h {
            findings.push(format!(
                "  {}: `{}` is {}, and {}'s is {}; the two manifests carry it, equal",
                harness_path,
                field,
                show(h),
                portable_path,
                show(p)
            ));
        }
    }
    for (path, m) in [(&portable_path, &portable), (&harness_path, &harness)] {
        if m.get("version").is_some() {
            findings.push(format!(
                "  {}: `version` is {}; expected absent, since the marketplace pin carries the version",
                path,
                show(m.get("version"))
            ));
        }
    }
    let schema_ok = portable
        .get("$schema")
        .and_then(Value::as_str)
        .and_then(|s| s.strip_prefix(SCHEMA_LEAD))
        .and_then(|s| s.strip_suffix(SCHEMA_TAIL))
        .is_some_and(is_triple);
    if !schema_ok {
        findings.push(format!(
            "  {}: `$schema` is {}; expected {}<X.Y.Z>{}, a published Agent Plugins schema",
            portable_path,
            show(portable.get("$schema")),
            SCHEMA_LEAD,
            SCHEMA_TAIL
        ));
    }

    let shim_list = shims(&skills_dir)?;
    let mut expected: BTreeSet<String> = shim_list.iter().map(|(n, _)| n.clone()).collect();
    expected.insert(INSTALL_SKILL.to_string());
    let skills_root = format!("{}/skills", plugin);
    let mut present: BTreeSet<String> = BTreeSet::new();
    for (name, is_dir) in walk::list_dir(Path::new(&skills_root))? {
        if is_dir {
            present.insert(name);
        }
    }
    for n in expected.difference(&present) {
        findings.push(format!(
            "  {}/{}: absent; expected a skill for each binding shim in {} and `{}`",
            skills_root, n, skills_dir, INSTALL_SKILL
        ));
    }
    for n in present.difference(&expected) {
        findings.push(format!(
            "  {}/{}: no binding shim in {} names it, and it is not `{}`",
            skills_root, n, skills_dir, INSTALL_SKILL
        ));
    }
    let mut skills = 0usize;
    for n in present.intersection(&expected) {
        skills += 1;
        let path = format!("{}/{}/SKILL.md", skills_root, n);
        let text = read(&path)?;
        let Some((keys, body)) = front_matter(&text) else {
            findings.push(format!("  {}: no front matter; expected `---`, `name: {}`, a `description:`, `---`", path, n));
            continue;
        };
        let get = |k: &str| keys.iter().find(|(key, _)| key == k).map(|(_, v)| v.as_str());
        if get("name") != Some(n.as_str()) {
            findings.push(format!("  {}: `name` is {:?}; expected `{}`, its directory", path, get("name"), n));
        }
        let dlen = get("description").map(|d| d.chars().count()).unwrap_or(0);
        if dlen == 0 || dlen > DESCRIPTION_MAX {
            findings.push(format!(
                "  {}: `description` is {} character(s); expected 1 to {}",
                path, dlen, DESCRIPTION_MAX
            ));
        }
        let Some((_, tmpl)) = shim_list.iter().find(|(s, _)| s == n) else {
            continue;
        };
        if !Path::new(tmpl).is_file() {
            findings.push(format!("  {}: its shim names template `{}`, and no such file exists", path, tmpl));
        }
        let want = skill_body(tmpl);
        if body != want {
            let shown: Vec<String> = want.lines().map(|l| format!("    | {}", l)).collect();
            findings.push(format!(
                "  {}: the body is not the rendering for `{}`; expected, after the front matter:\n{}",
                path,
                tmpl,
                shown.join("\n")
            ));
        }
    }

    let hooks_path = format!("{}/{}", plugin, HOOKS);
    let want_hooks = render_hooks(&json(&hooks_template)?)?;
    if json(&hooks_path)? != want_hooks {
        let pretty = serde_json::to_string_pretty(&want_hooks).unwrap_or_default();
        let shown: Vec<String> = pretty.lines().map(|l| format!("    | {}", l)).collect();
        findings.push(format!(
            "  {}: not the rendering of {}; expected, compared as JSON:\n{}",
            hooks_path,
            hooks_template,
            shown.join("\n")
        ));
    }

    // spec: plugin/SPEC.md §check-plugin-parity — the marketplace installs `plugin/` alone, so its
    // license text is a tracked byte copy of the root file (F)
    let want_license = std::fs::read(&root_license).map_err(|e| format!("cannot read {}: {}", root_license, e))?;
    let license_path = format!("{}/{}", plugin, PLUGIN_LICENSE);
    let license_state = match std::fs::read(&license_path) {
        Ok(got) if got == want_license => None,
        Ok(_) => Some("differs from"),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Some("is absent; expected a byte copy of"),
        Err(e) => return Err(format!("cannot read {}: {}", license_path, e)),
    };
    if let Some(state) = license_state {
        findings.push(format!(
            "  {}: {} {}, since the marketplace installs {} alone; copy it: cp {} {}",
            license_path, state, root_license, plugin, root_license, license_path
        ));
    }

    let publication = if Path::new(&marketplace).exists() {
        publication(&plugin, &marketplace, &portable, &install_sh, (&disposition_path, &queue_path), args, &mut findings)?
    } else {
        format!("D and E dormant — {} is withdrawn", marketplace)
    };

    if !findings.is_empty() {
        println!("{}: the plugin package does not match this repository (plugin/SPEC.md §check-plugin-parity):", NAME);
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: edit the named file to the expected value; a skill body or the hooks file is a rendering, so copy the expected text above, the license text is copied as its line names, and a pinned release lacking the package is released (RELEASING.md) or the marketplace file withdrawn.");
        return Ok(1);
    }
    println!(
        "PLUGIN-PARITY: clean (manifests agree, {} skill(s) match {} and `{}`, hooks match {}, license text matches {}, {})",
        skills, skills_dir, INSTALL_SKILL, hooks_template, root_license, publication
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: plugin/SPEC.md §The guards — every command prefixed, blocks in order, the note dropped
    #[test]
    fn the_hooks_rendering_prefixes_each_command_and_drops_the_note() {
        let t: Value = serde_json::from_str(
            r#"{"//":"merge advice","hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"bash x --hook a"}]},{"matcher":"Write","hooks":[{"type":"command","command":"bash x --hook b"}]}]}}"#,
        )
        .unwrap();
        let want = serde_json::json!({"hooks":{"PreToolUse":[
            {"matcher":"Bash","hooks":[{"type":"command","command":format!("{}bash x --hook a", FAIL_OPEN)}]},
            {"matcher":"Write","hooks":[{"type":"command","command":format!("{}bash x --hook b", FAIL_OPEN)}]}
        ]}});
        assert_eq!(render_hooks(&t).unwrap(), want);
    }
    // spec: plugin/SPEC.md §The skills — the front matter's scalars and the body after it
    #[test]
    fn the_front_matter_reader_takes_scalars_and_the_body() {
        let (k, b) = front_matter("---\nname: scope\ndescription: 'Runs it.'\n---\n\nbody\n").unwrap();
        assert_eq!(k, vec![("name".to_string(), "scope".to_string()), ("description".to_string(), "Runs it.".to_string())]);
        assert_eq!(b, "body\n");
        assert!(front_matter("no front matter\n").is_none());
    }
}
