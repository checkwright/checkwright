// spec: context-kit/SPEC.md §bin/env-probe — the marker-bounded machine profile, derived into the
// consumer-local, gitignored profile file; hand-authored gotchas live outside the markers and
// survive every re-probe. An action that reports: it rewrites a block and prints what it did.
use crate::proc;
use crate::toolfloor;

// spec: context-kit/SPEC.md §bin/env-probe — one bridged knob and no other: `lib/context.sh`
// defaults it and is the config bridge's sole resolver, so the value is computed in exactly one
// place. A hardcoded path would resolve `ENV.local.md` and ignore every consumer override.
pub const KNOBS: &[&str] = &["CONTEXT_KIT_ENV_PROFILE_FILE"];

const BEGIN: &str = "<!-- context-kit:env:begin -->";
const END: &str = "<!-- context-kit:env:end -->";

// spec: context-kit/SPEC.md §bin/env-probe — the package-manager detection walk; first present
// wins, ordered widest-family first.
const PM_CANDIDATES: &[&str] = &[
    "apt-get", "dnf", "yum", "pacman", "emerge", "zypper", "apk", "brew", "nix-env",
];

const SEED: &str = "# Local environment profile

Hand-authored gotchas go here, outside the generated block below, and survive
every re-probe — the facts a probe cannot know. For example: no `dig`/`host` on
this box; resolve names with `getent hosts` or a DoH `curl`.

";

// spec: context-kit/SPEC.md §bin/env-probe — both version probes read from an empty stdin: `-V`
// prints a banner for most tools but is an ordinary flag for some (GNU sort's version-sort), so a
// tool rejecting `--version` would otherwise fall through to a `-V` that hangs on inherited stdin.
fn banner(tool: &str, flag: &str) -> String {
    match proc::run_streamed(tool, &[flag], b"", proc::Stderr::Discard) {
        Ok(out) => String::from_utf8_lossy(out.stdout())
            .trim_end_matches('\n')
            .to_string(),
        Err(_) => String::new(),
    }
}

// spec: context-kit/SPEC.md §bin/env-probe — prefer the first line bearing an N.N version token
// (shellcheck buries it past a banner), else the first line, else the resolved path.
fn probe_version(tool: &str) -> Option<String> {
    let path = proc::which(tool)?;
    let mut raw = banner(tool, "--version");
    if raw.is_empty() {
        raw = banner(tool, "-V");
    }
    let dotted = crate::ere::Ere::compile("[0-9]+\\.[0-9]+").ok();
    let out = raw
        .lines()
        .find(|l| dotted.as_ref().is_some_and(|re| re.is_match(l)))
        .or_else(|| raw.lines().next())
        .map(str::to_string)
        .unwrap_or_else(|| format!("present ({})", path));
    Some(out.trim().to_string())
}

// spec: context-kit/SPEC.md §bin/env-probe — OS/distro: `uname -s -r -m`, and `/etc/os-release`'s
// `PRETTY_NAME` else its `ID`. The shell sourced that file; a port parses it, the rule
// check-install-toolchain's roster reader already states for an untrusted path.
fn os_line() -> String {
    let uname = match proc::run("uname", &["-s", "-r", "-m"]) {
        Ok(c) => c
            .stdout()
            .map(|o| String::from_utf8_lossy(o).trim().to_string())
            .unwrap_or_default(),
        Err(_) => String::new(),
    };
    let mut line = if uname.is_empty() {
        "unknown".to_string()
    } else {
        uname
    };
    if let Some(d) = distro() {
        if !d.is_empty() {
            line = format!("{} — {}", line, d);
        }
    }
    line
}

fn distro() -> Option<String> {
    let text = std::fs::read_to_string("/etc/os-release").ok()?;
    let field = |key: &str| -> Option<String> {
        text.lines()
            .filter_map(|l| l.strip_prefix(key))
            .next_back()
            .map(|v| v.trim_matches(['"', '\'']).to_string())
    };
    field("PRETTY_NAME=").or_else(|| field("ID="))
}

fn package_manager() -> String {
    for pm in PM_CANDIDATES {
        if let Some(path) = proc::which(pm) {
            return format!("{} ({})", pm, path);
        }
    }
    "none detected".to_string()
}

// spec: context-kit/SPEC.md §bin/env-probe — the audience marker, spelled once and appended by
// every line that names a member, so a reader tells a floor that is theirs apart from one they are
// not on the hook for.
fn audience_mark(element: &str) -> String {
    let a = toolfloor::parse(element).audience;
    if a.is_empty() {
        String::new()
    } else {
        format!("{}-only", a)
    }
}

// spec: context-kit/SPEC.md §bin/env-probe — the constrained member's parenthetical; an
// unconstrained member carries none, so the roster's optional axis stays optional on the page too.
fn render_floor(element: &str, verdict: &str) -> String {
    let e = toolfloor::parse(element);
    let mut desc = String::new();
    if !e.min.is_empty() {
        desc = format!("floor {}", e.min);
    }
    if !e.imp.is_empty() {
        if !desc.is_empty() {
            desc.push_str(", ");
        }
        desc.push_str(&format!("requires {}", e.imp));
    }
    let mark = audience_mark(element);
    if !mark.is_empty() {
        if !desc.is_empty() {
            desc.push_str(", ");
        }
        desc.push_str(&mark);
    }
    if desc.is_empty() {
        return String::new();
    }
    match verdict.split_whitespace().next().unwrap_or("") {
        "ok" => format!(" ({}, ok)", desc),
        "uncomparable" => format!(" ({} — unverified)", desc),
        _ => format!(" ({} — below contract)", desc),
    }
}

fn body(roster: &[String], date: &str) -> String {
    let mut tool_lines = String::new();
    let mut absent: Vec<String> = Vec::new();
    let mut below: Vec<String> = Vec::new();
    for element in roster {
        let e = toolfloor::parse(element);
        let mark = audience_mark(element);
        let suffix = |lead: &str| -> String {
            if mark.is_empty() {
                String::new()
            } else {
                format!("{}{}", lead, mark)
            }
        };
        let ver = probe_version(&e.name).unwrap_or_default();
        let verdict = toolfloor::check(element, &ver).rendered();
        let mut fields = verdict.split_whitespace();
        let kind = fields.next().unwrap_or("");
        let found = fields.next().unwrap_or("");
        let floor = fields.next().unwrap_or("");
        match kind {
            "absent" => {
                let paren = if mark.is_empty() {
                    String::new()
                } else {
                    format!(" ({})", mark)
                };
                absent.push(format!("`{}`{}", e.name, paren));
                continue;
            }
            "below" => below.push(format!(
                "`{}` (found {}, floor {}{})",
                e.name,
                found,
                floor,
                suffix(", ")
            )),
            "wrong-impl" => below.push(format!(
                "`{}` (found {}, requires {}{})",
                e.name,
                found,
                e.imp,
                suffix(", ")
            )),
            "uncomparable" => below.push(format!(
                "`{}` (unverified against floor {}{})",
                e.name,
                e.min,
                suffix(", ")
            )),
            _ => {}
        }
        tool_lines.push_str(&format!(
            "  - `{}` — {}{}\n",
            e.name,
            ver,
            render_floor(element, &verdict)
        ));
    }
    // spec: context-kit/SPEC.md §bin/env-probe — `printf '%s '` over the absent list leaves its
    // trailing space, and the below list joins on `; ` with the trailing separator stripped; both
    // spellings are the shell form's bytes and are preserved rather than tidied inside a port.
    let absent_line = if absent.is_empty() {
        "none".to_string()
    } else {
        format!("{} ", absent.join(" "))
    };
    let below_line = if below.is_empty() {
        "none".to_string()
    } else {
        below.join("; ")
    };
    format!(
        "_Probed {} by context-kit env-probe — do not hand-edit inside the markers._\n\n\
         - **OS:** {}\n\
         - **Package manager:** {}\n\
         - **Toolchain:**\n{}\
         - **Absent:** {}\n\
         - **Below contract:** {}\n",
        date,
        os_line(),
        package_manager(),
        tool_lines,
        absent_line,
        below_line
    )
}

// spec: context-kit/SPEC.md §bin/env-probe — change-detection compares every line but the derived
// `Probed <date>` one, so an unchanged box writes nothing and the date stays a last-changed signal.
fn comparable(block: &str) -> String {
    block
        .lines()
        .filter(|l| !l.starts_with("_Probed "))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn emit(_args: &[String]) -> Result<String, String> {
    let file = crate::walk::knob_scalar("CONTEXT_KIT_ENV_PROFILE_FILE")?;
    let roster_text = crate::fresh::read_captured(toolfloor::ROSTER)?;
    let roster = toolfloor::probe_set(&roster_text)
        .ok_or_else(|| format!("no PROBE_SET=(...) array in {}", toolfloor::ROSTER))?;

    let date = proc::run("date", &["+%F"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
        .ok_or_else(|| "cannot read the probe date (date +%F unavailable)".to_string())?;
    let new_body = body(&roster, &date);

    // spec: context-kit/SPEC.md §bin/env-probe — seed the gotchas scaffold once, outside the
    // markers; every re-probe replaces only the block.
    if !std::path::Path::new(&file).is_file() {
        std::fs::write(&file, SEED).map_err(|e| format!("cannot write {}: {}", file, e))?;
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — the presence test is whole-line on both halves, the
    // ruled resolution of the shell form's substring `grep` guarding a whole-line `awk` extract.
    let text = crate::fresh::read_captured(&file)?;
    if crate::fresh::file_lines(&text).contains(&BEGIN) {
        let old = crate::marker::read_block(&text, BEGIN, END);
        if comparable(&new_body) == comparable(&old) {
            return Ok(format!(
                "env-probe: env profile block unchanged in {} (Probed date preserved)\n",
                file
            ));
        }
    }

    let action = crate::marker::install_block(&file, BEGIN, END, &new_body)
        .map_err(|e| format!("failed to write profile block: {}", e))?;
    Ok(format!(
        "env-probe: {} the env profile block in {}\n",
        action, file
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §bin/env-probe — the rendered verdict's parenthetical, every axis
    // and every arm: an unconstrained member carries none at all
    #[test]
    fn the_parenthetical_renders_each_axis_and_each_verdict_arm() {
        assert_eq!(render_floor("jq", "ok"), "");
        assert_eq!(render_floor("bash:4.3", "ok"), " (floor 4.3, ok)");
        assert_eq!(
            render_floor("awk::GNU", "wrong-impl mawk"),
            " (requires GNU — below contract)"
        );
        assert_eq!(
            render_floor("cargo:1.71::contributor", "uncomparable"),
            " (floor 1.71, contributor-only — unverified)"
        );
        assert_eq!(audience_mark("cargo:1.71::contributor"), "contributor-only");
        assert_eq!(audience_mark("bash:4.3"), "");
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the change-detection comparison drops the derived
    // date line and nothing else, so a re-probe on an unchanged box writes nothing
    #[test]
    fn change_detection_ignores_the_probe_date_alone() {
        let a = "_Probed 2026-01-01 by context-kit env-probe_\n\n- **OS:** x\n";
        let b = "_Probed 2026-09-03 by context-kit env-probe_\n\n- **OS:** x\n";
        let c = "_Probed 2026-09-03 by context-kit env-probe_\n\n- **OS:** y\n";
        assert_eq!(comparable(a), comparable(b));
        assert_ne!(comparable(b), comparable(c));
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the block's five bullets, the absent list's
    // trailing space and the below list's `; ` join, over a roster no host can satisfy
    #[test]
    fn the_block_carries_five_bullets_and_the_shell_forms_list_spellings() {
        let roster = vec![
            "checkwright-no-such-tool".to_string(),
            "checkwright-other-absent:::contributor".to_string(),
        ];
        let out = body(&roster, "2026-09-03");
        assert!(out.starts_with("_Probed 2026-09-03 by context-kit env-probe — "), "{}", out);
        assert!(out.contains("\n- **OS:** "), "{}", out);
        assert!(out.contains("\n- **Package manager:** "), "{}", out);
        assert!(out.contains("\n- **Toolchain:**\n"), "{}", out);
        assert!(
            out.contains("\n- **Absent:** `checkwright-no-such-tool` `checkwright-other-absent` (contributor-only) \n"),
            "{}",
            out
        );
        assert!(out.ends_with("\n- **Below contract:** none\n"), "{}", out);
    }
}
