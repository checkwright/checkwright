// spec: plugin/SPEC.md §check-plugin-parity — the plugin package matches this repository: its two
// manifests (A), its skills (B), its hooks (C), the marketplace pin (D), the pinned release (E) and
// the license text (F)
use super::install_pin::{is_triple, pin_of};
use super::pinned_release::{self, read, Disposition};
use super::skill_binding::template_of;
use crate::hook::antigravity;
use crate::hook::binding::{COMMAND, EVENT_BEFORE_TOOL_CALL, PROJECT_DIR_VAR, WIRING_ARGS};
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
fn fail_open() -> String {
    let v = PROJECT_DIR_VAR;
    // spec: gate-sdk/SPEC.md §The path-dialect contract — shell text the hook's own shell runs, so the
    // toplevel it names is produced and read there, never in the crate
    format!("r=$(git -C \"${{{v}:-.}}\" rev-parse --show-toplevel 2>/dev/null) && test -f \"$r/gate-sdk/bin/run-gates.sh\" || exit 0; {v}=$r; ")
}
// spec: plugin/SPEC.md §The guards — the shell-form front-end call a member is rendered behind
fn front_end_call() -> String {
    format!("bash \"${{{}}}/gate-sdk/bin/run-gates.sh\" --hook ", PROJECT_DIR_VAR)
}
// spec: plugin/SPEC.md §The guards — the Antigravity command's head: `sh` reads an assignment, and
// `cmd` runs `rem`, which ignores the rest of the line and prints nothing
const CMD_COMMENT: &str = "rem=1; ";
// spec: plugin/SPEC.md §The guards — the Antigravity prefix: the first workspace path read with `sed`,
// its toplevel asked of git, the front end and `bash` tested, else exit 0 with nothing on stdout
fn antigravity_prefix() -> String {
    let w = antigravity::WORKSPACE_PATHS;
    let read = format!("p=$(cat); w=$(printf '%s' \"$p\" | sed -n 's/.*\"{w}\"[[:space:]]*:[[:space:]]*\\[[[:space:]]*\"\\([^\"]*\\)\".*/\\1/p'); ");
    // spec: gate-sdk/SPEC.md §The path-dialect contract — shell text the hook's own shell runs, so the
    // toplevel it names is produced and read there, never in the crate
    let find = "test -n \"$w\" && r=$(git -C \"$w\" rev-parse --show-toplevel 2>/dev/null) && test -f \"$r/gate-sdk/bin/run-gates.sh\" && command -v bash >/dev/null 2>&1 || exit 0; ";
    format!("{read}{find}o=$(cd \"$r\" && printf '%s' \"$p\" | bash gate-sdk/bin/run-gates.sh --hook ")
}
// spec: plugin/SPEC.md §The guards — the Antigravity suffix: the front end's stdout printed only where
// it opens with the deny envelope, and exit 0 whatever happened
fn antigravity_suffix() -> String {
    format!(
        " --binding {} 2>/dev/null); case \"$o\" in '{{\"{}\":\"{}\"'*) printf '%s\\n' \"$o\";; esac; exit 0",
        antigravity::NAME,
        antigravity::DECISION,
        antigravity::DECISION_DENY
    )
}
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

// spec: plugin/SPEC.md §The guards — each block under `hooks` in order, each hook's command the
// fail-open prefix and the front-end call of the member the registration parser reads off it, and
// the template's `//` note dropped
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
                let member = match crate::hook::registration(&crate::hook::command_tokens(hook)) {
                    crate::hook::Registration::Member(m) => m.to_string(),
                    _ => {
                        return Err(format!(
                            "{} carries a hook that is no member registration, which the plugin has no command for: {}",
                            HOOKS_TEMPLATE, hook
                        ))
                    }
                };
                if let Some(fields) = hook.as_object_mut() {
                    fields.remove(WIRING_ARGS);
                    fields.insert(
                        COMMAND.to_string(),
                        Value::String(format!("{}{}{}", fail_open(), front_end_call(), member)),
                    );
                }
            }
        }
        out.insert(event.clone(), list);
    }
    let mut root = Map::new();
    root.insert("hooks".to_string(), Value::Object(out));
    Ok(Value::Object(root))
}

// spec: plugin/SPEC.md §The guards — the composed Antigravity command for one member
pub(crate) fn antigravity_command(member: &str) -> String {
    format!("{}{}{}{}", CMD_COMMENT, antigravity_prefix(), member, antigravity_suffix())
}

// spec: plugin/SPEC.md §The guards — one hook named for the package, one before-tool-call group per
// template group whose tools have an Antigravity spelling, its matcher those spellings joined by `|`
pub(crate) fn render_antigravity_hooks(template: &Value, package: &str) -> Result<Value, String> {
    let groups = template
        .get("hooks")
        .and_then(|h| h.get(EVENT_BEFORE_TOOL_CALL))
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{} carries no '{}' list under 'hooks'", HOOKS_TEMPLATE, EVENT_BEFORE_TOOL_CALL))?;
    let mut out = Vec::new();
    for group in groups {
        let mut tools: Vec<&str> = Vec::new();
        for t in group.get("matcher").and_then(Value::as_str).unwrap_or("").split('|') {
            for s in antigravity::spellings(t) {
                if !tools.contains(s) {
                    tools.push(s);
                }
            }
        }
        if tools.is_empty() {
            continue;
        }
        let mut group = group.clone();
        if let Some(fields) = group.as_object_mut() {
            fields.insert("matcher".to_string(), Value::String(tools.join("|")));
        }
        for hook in group.get_mut("hooks").and_then(Value::as_array_mut).into_iter().flatten() {
            let member = match crate::hook::registration(&crate::hook::command_tokens(hook)) {
                crate::hook::Registration::Member(m) => m.to_string(),
                _ => {
                    return Err(format!(
                        "{} carries a hook that is no member registration, which the plugin has no command for: {}",
                        HOOKS_TEMPLATE, hook
                    ))
                }
            };
            if let Some(fields) = hook.as_object_mut() {
                fields.remove(WIRING_ARGS);
                fields.insert(antigravity::WIRING_COMMAND.to_string(), Value::String(antigravity_command(&member)));
            }
        }
        out.push(group);
    }
    let mut events = Map::new();
    events.insert(antigravity::EVENT_BEFORE_TOOL_CALL.to_string(), Value::Array(out));
    let mut root = Map::new();
    root.insert(package.to_string(), Value::Object(events));
    Ok(Value::Object(root))
}

fn json(path: &str) -> Result<Value, String> {
    serde_json::from_str(&read(path)?).map_err(|e| format!("{} is not JSON: {}", path, e))
}

fn show(v: Option<&Value>) -> String {
    v.map(Value::to_string).unwrap_or_else(|| "absent".to_string())
}

struct Field {
    key: String,
    value: String,
    fault: Option<String>,
}

// spec: plugin/SPEC.md §The skills — front matter between two `---` lines, read as `key: value`
// scalars past its full-line comments, and the body after it with its leading blank lines dropped
fn front_matter(text: &str) -> Option<(Vec<Field>, String)> {
    let rest = text.strip_prefix("---\n")?;
    let end = rest.find("\n---\n")?;
    let keys = rest[..end]
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .filter_map(|l| l.split_once(':'))
        .map(|(k, v)| {
            let v = v.trim();
            let quoted = v
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| v.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')));
            Field {
                key: k.trim().to_string(),
                value: quoted.unwrap_or(v).to_string(),
                fault: if quoted.is_some() { None } else { plain_fault(v) },
            }
        })
        .collect();
    Some((keys, rest[end + 5..].trim_start_matches('\n').to_string()))
}

// spec: plugin/SPEC.md §check-plugin-parity — what a YAML plain scalar cannot carry, so an unquoted value YAML rejects or cuts short reds here
fn plain_fault(v: &str) -> Option<String> {
    let first = v.chars().next()?;
    let spaced = |c: char| v.strip_prefix(c).is_some_and(|r| r.is_empty() || r.starts_with([' ', '\t']));
    if "[]{},#&*!|>'\"%@`".contains(first) || ['-', '?', ':'].into_iter().any(spaced) {
        return Some(format!("opens with the YAML indicator `{}`", first));
    }
    if v.contains(": ") || v.contains(":\t") || v.ends_with(':') {
        return Some("carries a `:` before a blank, which YAML reads as a mapping".to_string());
    }
    if v.contains(" #") || v.contains("\t#") {
        return Some("carries a `#` after a blank, which YAML reads as a comment".to_string());
    }
    None
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
        for f in &keys {
            if let Some(why) = &f.fault {
                findings.push(format!(
                    "  {}: `{}` is an unquoted YAML plain scalar that {}; expected it quoted",
                    path, f.key, why
                ));
            }
        }
        let get = |k: &str| keys.iter().find(|f| f.key == k).map(|f| f.value.as_str());
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

    let template = json(&hooks_template)?;
    let package = portable.get("name").and_then(Value::as_str).unwrap_or("");
    for (path, kind, want) in [
        (format!("{}/{}", plugin, HOOKS), "rendering", render_hooks(&template)?),
        (
            format!("{}/{}", plugin, antigravity::HOOK_FILE),
            "Antigravity rendering",
            render_antigravity_hooks(&template, package)?,
        ),
    ] {
        if json(&path)? != want {
            let pretty = serde_json::to_string_pretty(&want).unwrap_or_default();
            let shown: Vec<String> = pretty.lines().map(|l| format!("    | {}", l)).collect();
            findings.push(format!(
                "  {}: not the {} of {}; expected, compared as JSON:\n{}",
                path,
                kind,
                hooks_template,
                shown.join("\n")
            ));
        }
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

    // spec: plugin/SPEC.md §The guards — each command composed from the member, exec form and shell
    // form alike, blocks in order, the note dropped; a hook that is no member registration refused
    #[test]
    fn the_hooks_rendering_composes_each_command_from_its_member_and_drops_the_note() {
        let t: Value = serde_json::from_str(
            r#"{"//":"merge advice","hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"${CLAUDE_PROJECT_DIR}/@GATE_SDK_NATIVE_BIN@","args":["--hook","a"]}]},{"matcher":"Write","hooks":[{"type":"command","command":"bash x/run-gates.sh --hook b"}]}]}}"#,
        )
        .unwrap();
        let call = |m: &str| format!("{}{}{}", fail_open(), front_end_call(), m);
        let want = serde_json::json!({"hooks":{"PreToolUse":[
            {"matcher":"Bash","hooks":[{"type":"command","command":call("a")}]},
            {"matcher":"Write","hooks":[{"type":"command","command":call("b")}]}
        ]}});
        assert_eq!(render_hooks(&t).unwrap(), want);
        let stray: Value = serde_json::from_str(
            r#"{"hooks":{"Stop":[{"hooks":[{"type":"command","command":"bash scripts/x.sh"}]}]}}"#,
        )
        .unwrap();
        assert!(render_hooks(&stray).unwrap_err().contains("no member registration"));
    }
    // spec: plugin/SPEC.md §The guards — the Antigravity rendering: one hook named for the package,
    // each group's matcher the Antigravity spellings of its tools, a group with none left out
    #[test]
    fn the_antigravity_rendering_maps_each_group_and_drops_a_group_with_no_spelling() {
        let t: Value = serde_json::from_str(
            r#"{"//":"note","hooks":{"PreToolUse":[
                {"matcher":"Bash|PowerShell","hooks":[{"type":"command","command":"${CLAUDE_PROJECT_DIR}/@GATE_SDK_NATIVE_BIN@","args":["--hook","shell-guard"]}]},
                {"matcher":"ScheduleWakeup|CronCreate","hooks":[{"type":"command","command":"${CLAUDE_PROJECT_DIR}/@GATE_SDK_NATIVE_BIN@","args":["--hook","wakeup-guard"]}]},
                {"matcher":"Write|Edit","hooks":[{"type":"command","command":"${CLAUDE_PROJECT_DIR}/@GATE_SDK_NATIVE_BIN@","args":["--hook","workflow-state-guard"]}]}
            ]}}"#,
        )
        .unwrap();
        let want = serde_json::json!({"pkg":{"PreToolUse":[
            {"matcher":"run_command","hooks":[{"type":"command","command":antigravity_command("shell-guard")}]},
            {"matcher":"write_file|edit_file|write_to_file|replace_file_content|multi_replace_file_content",
             "hooks":[{"type":"command","command":antigravity_command("workflow-state-guard")}]}
        ]}});
        assert_eq!(render_antigravity_hooks(&t, "pkg").unwrap(), want);
        let c = antigravity_command("shell-guard");
        assert!(c.starts_with("rem=1; p=$(cat); "), "{}", c);
        assert!(c.contains("--hook shell-guard --binding antigravity"), "{}", c);
        assert!(c.ends_with("exit 0"), "{}", c);
    }

    // spec: plugin/SPEC.md §The guards — under `sh` the command never fails a call: a payload whose
    // workspace is no repository exits 0 with nothing on stdout
    #[cfg(unix)]
    #[test]
    fn the_antigravity_command_exits_0_silently_outside_a_repository() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let payload = serde_json::json!({"workspacePaths":[dir.join("checkwright-no-such-workspace").display().to_string()],
            "toolCall":{"name":"run_command","args":{"CommandLine":"ls"}}});
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(antigravity_command("shell-guard"))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("sh must spawn");
        child.stdin.take().unwrap().write_all(payload.to_string().as_bytes()).unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success());
        assert!(out.stdout.is_empty(), "{}", String::from_utf8_lossy(&out.stdout));
    }

    // spec: plugin/SPEC.md §The guards — Go's `syscall.EscapeArg`, the composition of the one argument
    // `cmd /c` receives
    #[cfg(windows)]
    fn go_escape_arg(s: &str) -> String {
        if s.is_empty() {
            return "\"\"".to_string();
        }
        let has_space = s.contains(' ') || s.contains('\t');
        if !has_space && !s.contains('"') && !s.contains('\\') {
            return s.to_string();
        }
        let mut out = String::new();
        if has_space {
            out.push('"');
        }
        let mut slashes = 0;
        for c in s.chars() {
            match c {
                '\\' => {
                    slashes += 1;
                    out.push(c);
                }
                '"' => {
                    out.extend(std::iter::repeat('\\').take(slashes + 1));
                    slashes = 0;
                    out.push(c);
                }
                _ => {
                    slashes = 0;
                    out.push(c);
                }
            }
        }
        if has_space {
            out.extend(std::iter::repeat('\\').take(slashes));
            out.push('"');
        }
        out
    }

    // spec: plugin/SPEC.md §The guards — on native Windows `cmd` reads the command as a comment:
    // exit 0 and nothing on stdout, so the package runs no guard there rather than failing a call
    #[cfg(windows)]
    #[test]
    fn cmd_reads_the_antigravity_command_as_a_comment() {
        use std::io::Write;
        use std::os::windows::process::CommandExt;
        let payload = r#"{"workspacePaths":["C:\\w"],"toolCall":{"name":"run_command","args":{"CommandLine":"dir"}}}"#;
        let mut child = std::process::Command::new(antigravity::WINDOWS_SHELL)
            .raw_arg(antigravity::WINDOWS_SHELL_RUN)
            .raw_arg(go_escape_arg(&antigravity_command("shell-guard")))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("cmd must spawn");
        child.stdin.take().unwrap().write_all(payload.as_bytes()).unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success(), "cmd exited {:?}", out.status);
        assert!(out.stdout.is_empty(), "cmd printed: {}", String::from_utf8_lossy(&out.stdout));
    }

    // spec: plugin/SPEC.md §The skills — the front matter's scalars and the body after it
    #[test]
    fn the_front_matter_reader_takes_scalars_and_the_body() {
        let (k, b) = front_matter("---\nname: scope\ndescription: 'Runs it.'\n---\n\nbody\n").unwrap();
        let pairs: Vec<(&str, &str)> = k.iter().map(|f| (f.key.as_str(), f.value.as_str())).collect();
        assert_eq!(pairs, [("name", "scope"), ("description", "Runs it.")]);
        assert!(k.iter().all(|f| f.fault.is_none()));
        assert_eq!(b, "body\n");
        assert!(front_matter("no front matter\n").is_none());
        let (k, _) = front_matter("---\n# Note: use x: y\n  # indented: too\nname: scope\n---\nbody\n").unwrap();
        let keys: Vec<&str> = k.iter().map(|f| f.key.as_str()).collect();
        assert_eq!(keys, ["name"]);
        assert!(k.iter().all(|f| f.fault.is_none()));
    }

    // spec: plugin/SPEC.md §check-plugin-parity — an unquoted value YAML rejects or cuts short is a fault; its quoted twin and a safe plain scalar are none
    #[test]
    fn a_plain_scalar_yaml_cannot_carry_is_a_fault_and_quoting_clears_it() {
        let faulted = |d: &str| {
            let (k, _) = front_matter(&format!("---\ndescription: {}\n---\nbody\n", d)).unwrap();
            k[0].fault.is_some()
        };
        for bad in ["Runs it: always.", "Runs it:", "Runs it #now", "- a list", "[x]", "* star", "'unclosed", "@at", ": lead"] {
            assert!(faulted(bad), "{:?} passed", bad);
        }
        for good in ["\"Runs it: always.\"", "'Runs it: always.'", "Runs it.", "a:b", "C# code", "-flag", "the user's own", ""] {
            assert!(!faulted(good), "{:?} faulted", good);
        }
    }
}
