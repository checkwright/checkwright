// spec: gate-sdk/SPEC.md §gen-pre-commit — the two generated git hooks, emitted from the per-gate
// `# graph:` manifests; the arm and `check-graph` assertion D both read this one emission
use crate::{registry, walk};
use std::path::Path;

pub const KNOBS: &[&str] = &[
    "GATE_SDK_HOOKS_DIR",
    "GATE_SDK_NATIVE_BIN",
    "GATE_SDK_KIT_DIRS",
    registry::EVERY_COUPLES_KNOB,
];

const USAGE: &str = "usage: --emit git-hooks pre-commit|commit-msg|--write";

// spec: gate-sdk/SPEC.md §gen-pre-commit — each header names the binary the hook bakes as its
// regeneration and install door
fn pre_commit_head(bin: &str) -> String {
    format!(
        r#"#!/bin/sh
# pre-commit - GENERATED, DO NOT EDIT (except gen=manual regions between sentinels).
#
# Emitted from the per-gate `# graph:` manifests by:
#     {bin} --emit git-hooks --write
# Edit a gate's manifest (couples=/trigger=/mode=/gen=), or a gen=manual region
# below, then regenerate. check-graph asserts this file equals
# `--emit git-hooks pre-commit`. Each gate prints its own per-finding + `help:`
# lines before this hook reports the failure. The hook, a gen=manual region
# included, is POSIX sh.
#
# Install (opt-in, per clone):   {bin} --install-hooks
# Bypass once (use sparingly):   git commit --no-verify
#
# This is the *triggered subset* of the gates.list battery: every check here
# also runs whole-tree via `{bin} --run`.
set -eu

staged_all=$(git diff --cached --name-only --diff-filter=ACMR)
[ -n "$staged_all" ] || exit 0

# True if any staged path matches one of the given globs (a `case` pattern: `*` spans '/').
staged_matches() {{
"#
    )
}

const RUN_GATE: &str = r#"
GATE_SDK_VERBOSE="${GATE_SDK_VERBOSE:-}"
_ran=0
# Capture a gate's output; reprint it only on failure, or with GATE_SDK_VERBOSE.
run_gate() {
    _rg_name=$1; shift
    _rg_ok=1
    _rg_out=$("$@" 2>&1) || _rg_ok=0
    _ran=$((_ran + 1))
    if [ "$_rg_ok" -eq 0 ]; then
        if [ -n "$_rg_out" ]; then printf '%s\n' "$_rg_out"; fi
        hook_fail "$_rg_name"
    fi
    if [ -n "$GATE_SDK_VERBOSE" ]; then
        if [ -n "$_rg_out" ]; then printf '%s\n' "$_rg_out"; fi
        printf '  PASS: %s\n' "$_rg_name"
    fi
}
"#;

fn commit_msg_head(bin: &str) -> String {
    format!(
        r#"#!/bin/sh
# commit-msg - GENERATED, DO NOT EDIT.
#
# Emitted from the tier=commit-msg `# graph:` manifests by:
#     {bin} --emit git-hooks --write
# Edit a gate's manifest, then regenerate. check-graph asserts this file equals
# `--emit git-hooks commit-msg`. git feeds the prospective message file as $1; each gate
# prints its own per-finding + `help:` lines before this hook reports failure.
#
# Install (opt-in, per clone):   {bin} --install-hooks
# Bypass once (use sparingly):   git commit --no-verify
set -eu

msg_file="${{1:?commit-msg: git did not pass the message-file path}}"
"#
    )
}

fn hook_fail(hook: &str) -> String {
    format!(
        "\n# Uniform failure: the captured output was already reprinted above.\nhook_fail() {{\n    echo \"\"\n    echo \"{}: $1 failed (see above).\"\n    echo \"  Bypass once (use sparingly): git commit --no-verify\"\n    exit 1\n}}\n",
        hook
    )
}

fn tail(hook: &str) -> String {
    format!("\nprintf '{}: %d gate(s) passed.\\n' \"$_ran\"\nexit 0\n", hook)
}

const MANUAL_TODO: &str = "    # TODO: fill this manual region, then re-run --emit\n";

// spec: gate-sdk/SPEC.md §gen-pre-commit — what both hooks resolve once per emission: the registry,
// the kit roots spelled relative to the repository root, since a hook glob matches a staged path,
// and the invocation's binary
struct Ctx {
    gates_dir: String,
    members: Vec<(String, Vec<(String, String)>)>,
    check_dirs: Vec<String>,
    kit_roots: Vec<String>,
    native_bin: String,
}

fn context(root: &str, gates_dir: &str) -> Result<Ctx, String> {
    let list = registry::list_path(gates_dir);
    if !Path::new(&list).is_file() {
        return Err(format!("no registry at {}", list));
    }
    let text = super::read_text(&list)?;
    let kit_roots: Vec<String> =
        walk::kit_roots_abs()?.iter().map(|k| walk::relative_to(root, k)).collect();
    let mut check_dirs = vec![gates_dir.to_string()];
    for k in &kit_roots {
        check_dirs.push(format!("{}/checks", k));
    }
    // spec: gate-sdk/SPEC.md §gen-pre-commit — each manifest is read through the dirs anchored at the
    // root the emission names, so what the hook couples does not hang on the working directory
    let anchored: Vec<String> = check_dirs.iter().map(|d| walk::abs_against(root, d)).collect();
    let members = resolved_members(&text, &anchored);
    Ok(Ctx {
        gates_dir: gates_dir.to_string(),
        members,
        check_dirs,
        kit_roots,
        native_bin: walk::knob_scalar("GATE_SDK_NATIVE_BIN")?,
    })
}

fn resolved_members(text: &str, anchored: &[String]) -> Vec<(String, Vec<(String, String)>)> {
    let mut names: Vec<String> = Vec::new();
    for m in registry::members(text) {
        if !names.contains(&m) {
            names.push(m);
        }
    }
    let mut members = Vec::new();
    for n in names {
        let fields = match registry::resolve(&n, anchored) {
            Some(src) => {
                let body = std::fs::read(&src)
                    .map(|b| String::from_utf8_lossy(&b).into_owned())
                    .unwrap_or_default();
                registry::manifest_line(&body)
                    .map(registry::manifest_fields)
                    .unwrap_or_default()
            }
            None => Vec::new(),
        };
        members.push((n, fields));
    }
    members
}

fn is_commit_msg(fields: &[(String, String)]) -> bool {
    registry::field(fields, "tier") == "commit-msg"
}

// spec: installer/SPEC.md §init — the conditional `commit_msg` emits on, asked of a registry text
// and absolute resolve dirs, so a caller planning a tree that does not exist yet reads this
// predicate rather than restating it.
pub fn owes_commit_msg(registry: &str, anchored: &[String]) -> bool {
    resolved_members(registry, anchored).iter().any(|(_, f)| is_commit_msg(f))
}

// spec: gate-sdk/SPEC.md §gen-pre-commit — shell-inert verbatim, anything else POSIX single-quoted
// by a fixed rule so the committed hook is byte-identical across clones
fn quote_elem(s: &str) -> String {
    let inert = |c: char| c.is_ascii_alphanumeric() || "_./:=+,@%-".contains(c);
    if !s.is_empty() && s.chars().all(inert) {
        return s.to_string();
    }
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn invocation(ctx: &Ctx, name: &str) -> String {
    match registry::resolve(name, &ctx.check_dirs) {
        Some(p) if p.ends_with(".gate") => {
            format!("{} {}", quote_elem(&ctx.native_bin), quote_elem(name))
        }
        Some(p) => quote_elem(&p),
        None => format!("{}/{}.sh", ctx.gates_dir, name),
    }
}

// spec: gate-sdk/SPEC.md §gen-pre-commit — a comma list split as bash `read -a` splits it: one
// trailing empty field is no field
fn globs(trigger: &str) -> Vec<&str> {
    let mut out: Vec<&str> = trigger.split(',').collect();
    if out.last() == Some(&"") {
        out.pop();
    }
    out
}

// spec: gate-sdk/SPEC.md §gen-pre-commit — the current hook's `gen=manual` regions by member, read
// before emission so a regeneration carries each one back
fn manual_regions(hook: &str) -> Vec<(String, String)> {
    let text = match std::fs::read(hook) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => return Vec::new(),
    };
    let sentinel = |line: &str, mark: &str| -> Option<String> {
        let rest = line
            .trim_start_matches(|c: char| c.is_ascii_whitespace())
            .strip_prefix(mark)?;
        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
            .collect();
        (!name.is_empty()).then_some(name)
    };
    let mut out: Vec<(String, String)> = Vec::new();
    let mut open: Option<(String, String)> = None;
    let complete = text.len() - text.rsplit_once('\n').map_or(text.len(), |(_, t)| t.len());
    for line in text[..complete].split_terminator('\n') {
        if let Some(name) = sentinel(line, "# >>> manual: ") {
            open = Some((name, String::new()));
            continue;
        }
        if open.is_some() && sentinel(line, "# <<< manual: ").is_some() {
            let (name, buf) = open.take().expect("checked open");
            out.retain(|(n, _)| *n != name);
            out.push((name, buf));
            continue;
        }
        if let Some((_, buf)) = open.as_mut() {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §gen-pre-commit — the matcher body is lib/gate.sh's, read as text
fn matcher_body() -> Result<String, String> {
    let lib = format!("{}/lib/gate.sh", walk::sdk_root().trim_end_matches('/'));
    let text = super::read_text(&lib)?;
    let mut lines = text.lines().skip_while(|l| *l != "gate_staged_matches() {");
    if lines.next().is_none() {
        return Err(format!("{} defines no gate_staged_matches() function to splice into the hook", lib));
    }
    let body: Vec<&str> = lines.take_while(|l| *l != "}").collect();
    let body = body.join("\n");
    let body = body.trim_end_matches('\n');
    if body.is_empty() {
        return Err(format!("{}'s gate_staged_matches() has an empty body to splice into the hook", lib));
    }
    Ok(format!("{}\n", body))
}

fn block(ctx: &Ctx, name: &str, fields: &[(String, String)], manual: &[(String, String)]) -> Result<String, String> {
    let couples = registry::field(fields, "couples");
    let authored = registry::field(fields, "trigger");
    let authored = if authored.is_empty() { couples } else { authored };
    let mut trigger = registry::expand_couples(&authored, &ctx.kit_roots)?;
    let knob_files = registry::knob_files(name, &ctx.check_dirs)?;
    if trigger != "*" && !knob_files.is_empty() {
        if !trigger.is_empty() {
            trigger.push(',');
        }
        trigger.push_str(&knob_files.join(","));
    }
    let mut out = String::from("\n");
    if registry::field(fields, "gen") == "manual" {
        out.push_str(&format!("# >>> manual: {}\n", name));
        match manual.iter().find(|(n, _)| n == name) {
            Some((_, region)) => out.push_str(region),
            None => out.push_str(MANUAL_TODO),
        }
        out.push_str(&format!("# <<< manual: {}\n", name));
        return Ok(out);
    }
    let quoted: Vec<String> = globs(&trigger).iter().map(|g| format!("'{}'", g)).collect();
    let quoted = quoted.join(" ");
    let inv = invocation(ctx, name);
    if trigger == "*" {
        out.push_str(&format!("run_gate {} {}\n", name, inv));
    } else if registry::field(fields, "mode") == "staged" {
        out.push_str("_staged_run() {\n");
        out.push_str("    set --\n");
        out.push_str("    while IFS= read -r _sr_f; do\n");
        out.push_str("        if [ -f \"$_sr_f\" ]; then set -- \"$@\" \"$_sr_f\"; fi\n");
        out.push_str("    done <<_CHECKWRIGHT_STAGED_\n");
        out.push_str(&format!(
            "$(git diff --cached --name-only --diff-filter=ACMR -- {})\n",
            quoted
        ));
        out.push_str("_CHECKWRIGHT_STAGED_\n");
        out.push_str("    if [ \"$#\" -gt 0 ]; then\n");
        out.push_str(&format!("        run_gate {} {} \"$@\"\n", name, inv));
        out.push_str("    fi\n");
        out.push_str("}\n");
        out.push_str("_staged_run\n");
    } else {
        out.push_str(&format!("if staged_matches {}; then\n", quoted));
        out.push_str(&format!("    run_gate {} {}\n", name, inv));
        out.push_str("fi\n");
    }
    Ok(out)
}

fn hook_path(name: &str) -> Result<String, String> {
    Ok(format!("{}/{}", walk::knob_scalar("GATE_SDK_HOOKS_DIR")?, name))
}

pub fn pre_commit(root: &str, gates_dir: &str) -> Result<String, String> {
    let ctx = context(root, gates_dir)?;
    let manual = manual_regions(&hook_path("pre-commit")?);
    let mut out = pre_commit_head(&quote_elem(&ctx.native_bin));
    out.push_str(&matcher_body()?);
    out.push_str("}\n");
    out.push_str(&hook_fail("pre-commit"));
    out.push_str(RUN_GATE);
    for (name, fields) in &ctx.members {
        if registry::field(fields, "tier") == "precommit" {
            out.push_str(&block(&ctx, name, fields, &manual)?);
        }
    }
    out.push_str(&tail("pre-commit"));
    Ok(out)
}

// spec: gate-sdk/SPEC.md §gen-pre-commit — `None` is the one statement of the conditional: no
// registered member is `tier=commit-msg`, so no commit-msg hook is owed
pub fn commit_msg(root: &str, gates_dir: &str) -> Result<Option<String>, String> {
    let ctx = context(root, gates_dir)?;
    let gates: Vec<&String> = ctx
        .members
        .iter()
        .filter(|(_, f)| is_commit_msg(f))
        .map(|(n, _)| n)
        .collect();
    if gates.is_empty() {
        return Ok(None);
    }
    let mut out = commit_msg_head(&quote_elem(&ctx.native_bin));
    out.push_str(&hook_fail("commit-msg"));
    out.push_str(RUN_GATE);
    for name in gates {
        out.push_str(&format!("\nrun_gate {} {} \"$msg_file\"\n", name, invocation(&ctx, name)));
    }
    out.push_str(&tail("commit-msg"));
    Ok(Some(out))
}

fn write(path: &str, text: &str) -> Result<String, String> {
    std::fs::write(path, text).map_err(|e| format!("cannot write {}: {}", path, e))?;
    crate::install::make_executable(Path::new(path))?;
    Ok(format!("git-hooks: wrote {}\n", path))
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let op = match args {
        [op] if op == "pre-commit" || op == "commit-msg" || op == "--write" => op.as_str(),
        _ => return Err(USAGE.to_string()),
    };
    let root = walk::toplevel_opt()?.ok_or_else(|| "not inside a git repository".to_string())?;
    let gates_dir = crate::knobs::gates_dir();
    match op {
        "pre-commit" => pre_commit(&root, &gates_dir),
        "commit-msg" => commit_msg(&root, &gates_dir)?.ok_or_else(|| {
            "no registered member is tier=commit-msg, so there is no commit-msg hook to print".to_string()
        }),
        _ => {
            let dir = walk::knob_scalar("GATE_SDK_HOOKS_DIR")?;
            let hook = pre_commit(&root, &gates_dir)?;
            let msg = commit_msg(&root, &gates_dir)?;
            std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {}", dir, e))?;
            let mut out = write(&hook_path("pre-commit")?, &hook)?;
            if let Some(m) = msg {
                out.push_str(&write(&hook_path("commit-msg")?, &m)?);
            }
            Ok(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_inert_element_is_verbatim_and_any_other_is_single_quoted() {
        assert_eq!(quote_elem("native/target/release/checkwright-gates"), "native/target/release/checkwright-gates");
        assert_eq!(quote_elem(""), "''");
        assert_eq!(quote_elem("a b"), "'a b'");
        assert_eq!(quote_elem("t\ta'\\\n"), "'t\ta'\\''\\\n'");
    }

    #[test]
    fn one_trailing_empty_field_is_no_field() {
        assert!(globs("").is_empty());
        assert_eq!(globs("a,"), vec!["a"]);
        assert_eq!(globs("a,,"), vec!["a", ""]);
        assert_eq!(globs(",a"), vec!["", "a"]);
    }

    #[test]
    fn a_manual_region_is_read_by_its_opening_name_and_an_unterminated_line_is_dropped() {
        let d = std::env::temp_dir().join(format!("checkwright-git-hooks.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let hook = d.join("pre-commit");
        std::fs::write(
            &hook,
            "x\n# >>> manual: check-a\n  body one\n\n# <<< manual: check-other\nstray\n  # >>> manual: check-b\nlast",
        )
        .expect("write");
        let got = manual_regions(&hook.display().to_string());
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(got, vec![("check-a".to_string(), "  body one\n\n".to_string())]);
    }

    // spec: gate-sdk/SPEC.md §check-crate-arms — the hook emits under every shipped install
    // profile's kit set, set through a knob file in a scratch gates dir; a tree without the
    // installer's profile roster has nothing to hold
    #[test]
    fn the_pre_commit_hook_emits_under_every_install_profile() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root").to_path_buf();
        let Ok(roster) = std::fs::read_to_string(repo.join("installer/profiles.list")) else {
            return;
        };
        let mut profiles: Vec<(String, Vec<String>)> = Vec::new();
        for line in roster.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')) {
            let (p, kit) = line.split_once('\t').expect("a `<profile><TAB><kit>` row");
            match profiles.iter_mut().find(|(n, _)| n == p) {
                Some((_, kits)) => kits.push(kit.to_string()),
                None => profiles.push((p.to_string(), vec![kit.to_string()])),
            }
        }
        assert!(!profiles.is_empty(), "no profile parsed from installer/profiles.list");
        let mut full: Vec<String> = walk::list_dir(&repo)
            .expect("repo root")
            .into_iter()
            .map(|(n, _)| n)
            .filter(|n| repo.join(n).join("checks").is_dir() || repo.join(n).join("smoke").is_dir())
            .collect();
        full.sort();
        profiles.push(("full".to_string(), full));

        let env = crate::knobenv::lock();
        let d = std::env::temp_dir().join(format!("checkwright-hook-profiles.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        std::fs::copy(repo.join("scripts/gates.list"), d.join("gates.list")).expect("registry");
        let saved: Vec<(&str, Option<String>)> = ["GATE_SDK_GATES_DIR", "GATE_SDK_KIT_DIRS", "GATE_SDK_ROOT"]
            .into_iter()
            .map(|k| (k, std::env::var(k).ok()))
            .collect();
        env.set("GATE_SDK_GATES_DIR", &d.display().to_string());
        env.remove("GATE_SDK_KIT_DIRS");
        env.set("GATE_SDK_ROOT", &repo.join("gate-sdk").display().to_string());
        let mut failed: Vec<String> = Vec::new();
        for (name, kits) in &profiles {
            let dirs: Vec<String> = kits.iter().map(|k| repo.join(k).display().to_string()).collect();
            std::fs::write(d.join("gate-sdk-config.knobs"), format!("GATE_SDK_KIT_DIRS = {}\n", dirs.join(" ")))
                .expect("knob file");
            crate::knobs::reset(&env);
            if let Err(e) = pre_commit(&repo.display().to_string(), &d.display().to_string()) {
                failed.push(format!("{}: {}", name, e));
            }
        }
        for (k, v) in saved {
            match v {
                Some(v) => env.set(k, &v),
                None => env.remove(k),
            }
        }
        crate::knobs::reset(&env);
        let _ = std::fs::remove_dir_all(&d);
        assert!(failed.is_empty(), "the pre-commit hook does not emit under: {:?}", failed);
    }
}
