// spec: docs/install.md §Versioning — the declared release channel agrees with the publish
// workflow's prerelease posture (A) and with the project's own version line (B)
use crate::fresh;
use crate::proc;
use std::path::Path;

const DEFAULT_INSTALL_MD: &str = "docs/install.md";
const DEFAULT_PUBLISH_YML: &str = ".github/workflows/publish.yml";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-release-channel-parity: {}", e);
            2
        }
    }
}

// spec: docs/install.md §Versioning — sed's `s/^Release channel:[[:space:]]*\*\*([a-z]+)\*\*
// [[:space:]]*$/\1/p`, hand-compiled: a pattern the gate itself owns is not routed through the
// engine (gate-sdk/SPEC.md §The POSIX ERE matcher's boundary)
fn declared_channel(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("Release channel:")?;
    let rest = rest.trim_start_matches([' ', '\t']);
    let rest = rest.strip_prefix("**")?;
    let end = rest.find("**")?;
    let value = &rest[..end];
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_lowercase()) {
        return None;
    }
    if !rest[end + 2..]
        .chars()
        .all(|c| c == ' ' || c == '\t' || c == '\r')
    {
        return None;
    }
    Some(value)
}

// spec: docs/install.md §Versioning — `^v?([0-9]+)\.([0-9]+)\.([0-9]+)([-+].*)?$`, hand-compiled
// for the same reason; only the major is read
fn semver_major(v: &str) -> Option<u64> {
    let s = v.strip_prefix('v').unwrap_or(v);
    let mut fields = [0u64; 3];
    let mut rest = s;
    for (i, f) in fields.iter_mut().enumerate() {
        let end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        if end == 0 {
            return None;
        }
        *f = rest[..end].parse().ok()?;
        rest = &rest[end..];
        if i < 2 {
            rest = rest.strip_prefix('.')?;
        }
    }
    if rest.is_empty() || rest.starts_with('-') || rest.starts_with('+') {
        Some(fields[0])
    } else {
        None
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let install_md = fresh::positional(args, 0, DEFAULT_INSTALL_MD);
    let publish_yml = fresh::positional(args, 1, DEFAULT_PUBLISH_YML);
    let version_arg = args.get(2).map(String::as_str).unwrap_or("");

    if !Path::new(install_md).is_file() {
        return Err(format!("not found: {}", install_md));
    }
    if !Path::new(publish_yml).is_file() {
        return Err(format!("not found: {}", publish_yml));
    }

    let install_text = fresh::read_captured(install_md)?;
    let decls: Vec<String> = fresh::file_lines(&install_text)
        .iter()
        .enumerate()
        .filter(|(_, l)| l.starts_with("Release channel:"))
        .map(|(n, l)| format!("{}:{}", n + 1, l))
        .collect();

    if decls.is_empty() {
        return Err(format!(
            "{} carries no 'Release channel:' declaration line — the channel cannot be established (docs/install.md §Versioning owns the declaration)",
            install_md
        ));
    }
    if decls.len() > 1 {
        eprintln!(
            "check-release-channel-parity: {} carries {} 'Release channel:' declaration lines; exactly one is admissible:",
            install_md,
            decls.len()
        );
        eprintln!("{}", decls.join("\n"));
        return Ok(2);
    }

    let channels: Vec<&str> = fresh::file_lines(&install_text)
        .iter()
        .filter_map(|l| declared_channel(l))
        .collect();
    let channel = channels.join("\n");
    if channel != "preview" && channel != "stable" {
        let shown = if channel.is_empty() {
            "<unparseable>".to_string()
        } else {
            channel
        };
        return Err(format!(
            "{} declares an unrecognized channel value ({}); the two admissible values are 'preview' and 'stable' (docs/install.md §Versioning)",
            install_md, shown
        ));
    }

    let publish_text = fresh::read_captured(publish_yml)?;
    let create_step: Vec<String> = fresh::file_lines(&publish_text)
        .iter()
        .enumerate()
        .filter(|(_, l)| l.contains("gh release create"))
        .map(|(n, l)| format!("{}:{}", n + 1, l))
        .collect();
    if create_step.is_empty() {
        return Err(format!(
            "{} has no recognizable Release-creating step ('gh release create'); the prerelease posture cannot be established",
            publish_yml
        ));
    }
    let has_prerelease = create_step.iter().any(|l| l.contains("--prerelease"));

    let mut findings: Vec<String> = Vec::new();
    if channel == "preview" && !has_prerelease {
        findings.push(format!(
            "  invariant A: channel 'preview' demands --prerelease on the Release-creating step, and {} carries none:",
            publish_yml
        ));
        findings.push(indent4(&create_step));
    } else if channel == "stable" && has_prerelease {
        findings.push(format!(
            "  invariant A: channel 'stable' demands the absence of --prerelease, and {} carries it:",
            publish_yml
        ));
        findings.push(indent4(&create_step));
    }

    // spec: docs/install.md §Versioning — invariant B is dormant, and says so, where no tag exists
    let version = if version_arg.is_empty() {
        newest_tag()
    } else {
        version_arg.to_string()
    };

    let b_state;
    if version.is_empty() {
        b_state = "dormant".to_string();
    } else {
        let major = semver_major(&version).ok_or_else(|| {
            format!(
                "the version line ('{}') does not parse as semver, so the channel cannot be compared against it",
                version
            )
        })?;
        let bare = version.strip_prefix('v').unwrap_or(&version);
        if major == 0 {
            b_state = format!("v{} is 0.x, which demands channel 'preview'", bare);
            if channel != "preview" {
                findings.push(format!(
                    "  invariant B: version line {}, but '{}' is declared",
                    b_state, channel
                ));
            }
        } else {
            b_state = format!("v{} is 1.x or later, which demands channel 'stable'", bare);
            if channel != "stable" {
                findings.push(format!(
                    "  invariant B: version line {}, but '{}' is declared",
                    b_state, channel
                ));
            }
        }
    }

    if !findings.is_empty() {
        println!("check-release-channel-parity: the declared release channel disagrees with a surface it governs (docs/install.md §Versioning):");
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: bring the declaration, the Release-creating step's --prerelease posture, and the version line into agreement — the channel is derived from the version line, so 'preview' belongs to a 0.x line and 'stable' from v1.0.0 onward.");
        return Ok(1);
    }

    if b_state == "dormant" {
        println!(
            "RELEASE-CHANNEL-PARITY: clean (channel '{}' agrees with {}; invariant B dormant — no tags, so there is no version line to compare)",
            channel, publish_yml
        );
    } else {
        println!(
            "RELEASE-CHANNEL-PARITY: clean (channel '{}' agrees with {} and with the version line — {})",
            channel, publish_yml, b_state
        );
    }
    Ok(0)
}

fn indent4(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| format!("    {}", l))
        .collect::<Vec<String>>()
        .join("\n")
}

// spec: docs/install.md §Versioning — the newest tag by creator date. The shell form silences
// this probe and lets an unanswerable one yield no version line, so invariant B goes dormant
// rather than red; the conflation is preserved, because a port proves parity.
fn newest_tag() -> String {
    proc::run(
        "git",
        &[
            "for-each-ref",
            "--sort=-creatordate",
            "--count=1",
            "--format=%(refname:strip=2)",
            "refs/tags",
        ],
    )
    .ok()
    .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
    .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: docs/install.md §Versioning — the declaration line is exact: the value is bolded,
    // lower-case, and nothing but whitespace follows it
    #[test]
    fn the_declaration_grammar_admits_only_a_bolded_lowercase_value() {
        assert_eq!(declared_channel("Release channel: **preview**"), Some("preview"));
        assert_eq!(declared_channel("Release channel:  **stable**  "), Some("stable"));
        assert_eq!(declared_channel("Release channel: **Preview**"), None);
        assert_eq!(declared_channel("Release channel: preview"), None);
        assert_eq!(declared_channel("Release channel: **preview** and more"), None);
        assert_eq!(declared_channel("  Release channel: **preview**"), None);
    }

    // spec: docs/install.md §Versioning — the semver line, with an optional `v` and an optional
    // prerelease or build suffix; only the major decides the channel
    #[test]
    fn only_a_semver_triple_parses_and_the_major_is_what_is_read() {
        assert_eq!(semver_major("0.21.0"), Some(0));
        assert_eq!(semver_major("v1.2.3"), Some(1));
        assert_eq!(semver_major("2.0.0-rc1"), Some(2));
        assert_eq!(semver_major("2.0.0+build"), Some(2));
        for bad in ["1.2", "1.2.3.4", "x.y.z", "", "1.2.x"] {
            assert_eq!(semver_major(bad), None, "{} should not parse", bad);
        }
    }
}
