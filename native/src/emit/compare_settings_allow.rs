// spec: guard-kit/SPEC.md §compare-settings-allow — the prune-candidate set and the
// narrowing-candidate set: two questions over one match core, `guard::allow_match` with its
// arguments swapped between them. An advisory that renders no verdict, so every report path is 0.
use crate::guard;
use crate::walk;
use serde_json::Value;

// spec: guard-kit/SPEC.md §Layout and configuration — the four knobs `lib/guard.sh` defines and
// the bridge resolves by sourcing it; no default moves into the crate, so an absent guard-kit
// cannot resolve this arm at all and cannot name the two files it compares.
pub const KNOBS: &[&str] = &[
    "GUARD_KIT_SETTINGS",
    "GUARD_KIT_SETTINGS_LOCAL",
    "GUARD_KIT_BREADTH_PROBES",
    "GUARD_KIT_BREADTH_DECLARED",
];

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the usage the operand refusal prints. An
// `--emit-` member gets no front-end `case` arm and so no named help line, which is why the usage
// lives at the member's own shape refusal rather than retiring to the substrate.
const USAGE: &str = "usage: --emit compare-settings-allow [--count]";

const REDUNDANCY_HEADER: &str =
    "=== settings allowlist redundancy (advisory \u{2014} prune candidates) ===";
const BREADTH_HEADER: &str =
    "=== settings allowlist breadth (advisory \u{2014} narrowing candidates) ===";
const DECLARED_HEADER: &str =
    "=== settings allowlist breadth (advisory \u{2014} declared intended) ===";

// spec: guard-kit/SPEC.md §compare-settings-allow — the three readings of an allow list, kept
// apart because an unreadable file must not degrade into an empty one and print a clean line the
// document does not support.
enum AllowRead {
    Absent,
    Unparseable,
    Entries(Vec<String>),
}

// spec: guard-kit/SPEC.md §compare-settings-allow — `.permissions.allow[]?` read in-crate with
// `serde_json`: this member spawns no external program at all, so no machine that merely lacks a
// tool can turn a populated allowlist into an empty one.
fn read_allow(path: &str) -> AllowRead {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => return AllowRead::Absent,
    };
    let doc: Value = match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
        Ok(d) => d,
        Err(_) => return AllowRead::Unparseable,
    };
    AllowRead::Entries(
        doc.get("permissions")
            .and_then(|p| p.get("allow"))
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|e| e.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default(),
    )
}

// spec: guard-kit/SPEC.md §compare-settings-allow — the three report sets, each with its own
// reader: `redundant` the prune section and `--count`'s first integer, `too_broad` the narrowing
// section and its second, `declared` the declared-intended section and deliberately neither.
struct Report {
    redundant: Vec<String>,
    too_broad: Vec<String>,
    declared: Vec<String>,
}

// spec: guard-kit/SPEC.md §compare-settings-allow — redundancy asks whether a committed glob
// already grants a local entry; breadth is the same predicate with the arguments swapped, then
// partitioned on an exact-string declaration lookup.
fn partition(
    local: &[String],
    committed: &[String],
    probes: &[String],
    declarations: &[(String, String)],
) -> Report {
    let mut r = Report {
        redundant: Vec::new(),
        too_broad: Vec::new(),
        declared: Vec::new(),
    };
    for entry in local.iter().filter(|e| !e.is_empty()) {
        if let Some(pat) = committed
            .iter()
            .filter(|p| !p.is_empty())
            .find(|p| guard::allow_match(entry, p))
        {
            r.redundant
                .push(format!("{}  \u{2286}  {}", entry, pat));
        }
    }
    for entry in local.iter().filter(|e| !e.is_empty()) {
        let Some(probe) = probes
            .iter()
            .filter(|p| !p.is_empty())
            .find(|p| guard::allow_match(p, entry))
        else {
            continue;
        };
        match declarations.iter().find(|(k, _)| k == entry) {
            Some((_, reason)) => r.declared.push(format!(
                "{}  \u{2287}  {}  \u{2014} {}",
                entry, probe, reason
            )),
            None => r
                .too_broad
                .push(format!("{}  \u{2287}  {}", entry, probe)),
        }
    }
    r
}

fn listing(rows: &[String], out: &mut String) {
    out.push('\n');
    for row in rows {
        out.push_str(&format!("  {}\n", row));
    }
    out.push('\n');
}

// spec: guard-kit/SPEC.md §compare-settings-allow — a declared glob is not dropped from the
// report: it moves out of the narrowing set and prints with its reason, so the ruling has a reader.
fn declared_section(r: &Report, out: &mut String) {
    if r.declared.is_empty() {
        return;
    }
    if !r.too_broad.is_empty() {
        out.push('\n');
    }
    out.push_str(DECLARED_HEADER);
    out.push('\n');
    out.push_str(&format!(
        "{} over-broad local allow entr(ies) whose breadth was ruled intended:\n",
        r.declared.len()
    ));
    listing(&r.declared, out);
    out.push_str("help: no narrowing is owed \u{2014} each glob on the left auto-allows the probe beside\n");
    out.push_str("      it, and that breadth was ruled intended. The ruling lives with the probes\n");
    out.push_str("      in the committed guard-kit config, as a GUARD_KIT_BREADTH_DECLARED entry.\n");
    out.push_str("      A declaration is keyed on the exact glob string it ruled, so re-spelling\n");
    out.push_str("      or narrowing that glob returns the entry to the narrowing candidates.\n");
}

// spec: guard-kit/SPEC.md §compare-settings-allow — an over-broad set that is entirely declared
// prints the declared section and no narrowing section: the clean line would be false there.
fn breadth_section(r: &Report, local_path: &str, out: &mut String) {
    if r.too_broad.is_empty() && !r.declared.is_empty() {
        declared_section(r, out);
        return;
    }
    out.push_str(BREADTH_HEADER);
    out.push('\n');
    if r.too_broad.is_empty() {
        out.push_str(&format!(
            "no over-broad local entries (no configured probe is auto-allowed by a {} glob)\n",
            local_path
        ));
        return;
    }
    out.push_str(&format!(
        "{} local allow entr(ies) auto-allow a configured probe \u{2014} candidates to narrow in {}:\n",
        r.too_broad.len(),
        local_path
    ));
    listing(&r.too_broad, out);
    out.push_str("help: narrow each listed glob on the left, or record that the breadth is\n");
    out.push_str("      intended \u{2014} the probe on the right witnesses what it auto-allows.\n");
    out.push_str("      Probes are witnesses, not a roster: no completeness is claimed, so an\n");
    out.push_str("      empty report is not a proof that every local glob is narrow enough.\n");
    declared_section(r, out);
}

fn redundancy_section(r: &Report, local_path: &str, out: &mut String) {
    out.push_str(REDUNDANCY_HEADER);
    out.push('\n');
    if r.redundant.is_empty() {
        out.push_str(&format!(
            "no redundant local entries (every {} allow entry adds coverage)\n",
            local_path
        ));
        return;
    }
    out.push_str(&format!(
        "{} local allow entr(ies) already granted by a committed glob \u{2014} safe to prune from {}:\n",
        r.redundant.len(),
        local_path
    ));
    listing(&r.redundant, out);
    out.push_str(&format!(
        "help: remove each listed entry from {} \u{2014} the committed pattern on the\n",
        local_path
    ));
    out.push_str("      right already grants it (run at close, triage step 4).\n");
}

fn render(local_path: &str, r: &Report, probes: &[String], count_only: bool) -> String {
    if count_only {
        return format!("{} {}\n", r.redundant.len(), r.too_broad.len());
    }
    let mut out = String::new();
    redundancy_section(r, local_path, &mut out);
    // spec: guard-kit/SPEC.md §Layout and configuration — an empty probe set omits the whole
    // breadth section rather than printing a clean line, which is what keeps a consumer that
    // declared no vocabulary from reading silence as coverage.
    if probes.is_empty() {
        return out;
    }
    out.push('\n');
    breadth_section(r, local_path, &mut out);
    out
}

// spec: guard-kit/SPEC.md §compare-settings-allow — the no-overlay reading: the header and
// `no <path> — nothing to compare`, or `0 0` under `--count`. An overlay that exists but cannot be
// parsed takes this same path, because an unreadable overlay is not an overlay that grants nothing.
fn no_overlay(local_path: &str, count_only: bool) -> String {
    if count_only {
        return "0 0\n".to_string();
    }
    format!(
        "{}\nno {} \u{2014} nothing to compare\n",
        REDUNDANCY_HEADER, local_path
    )
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the `*)` refusal crosses the port: the one
// operand is drawn from a closed set, so validating membership already validates shape.
fn parse(args: &[String]) -> Result<bool, String> {
    let mut count_only = false;
    for a in args {
        if a.is_empty() {
            continue;
        }
        if a != "--count" {
            return Err(format!("unrecognized operand: {}\n{}", a, USAGE));
        }
        count_only = true;
    }
    Ok(count_only)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let count_only = parse(args)?;
    let committed_path = walk::knob_scalar("GUARD_KIT_SETTINGS")?;
    let local_path = walk::knob_scalar("GUARD_KIT_SETTINGS_LOCAL")?;
    let local = match read_allow(&local_path) {
        AllowRead::Entries(e) => e,
        AllowRead::Absent | AllowRead::Unparseable => {
            return Ok(no_overlay(&local_path, count_only))
        }
    };
    let committed = match read_allow(&committed_path) {
        AllowRead::Entries(e) => e,
        AllowRead::Absent => Vec::new(),
        // spec: guard-kit/SPEC.md §compare-settings-allow — a committed allowlist that exists and
        // no parser can read is a refusal with the path named, never zero entries: every local
        // entry would otherwise report as adding coverage it may already have.
        AllowRead::Unparseable => {
            return Err(format!(
                "cannot parse {} as JSON — the committed allowlist could not be read, and reading \
                 it as empty would report every local entry as adding coverage",
                committed_path
            ))
        }
    };
    let probes = walk::knob_array("GUARD_KIT_BREADTH_PROBES")?;
    let declarations = walk::knob_map("GUARD_KIT_BREADTH_DECLARED")?;
    let r = partition(&local, &committed, &probes, &declarations);
    Ok(render(&local_path, &r, &probes, count_only))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the `*)` refusal crosses the port, and
    // `--help` lands at it rather than at a front-end arm this member does not have
    #[test]
    fn an_operand_outside_the_closed_set_is_refused_and_help_lands_at_the_refusal() {
        assert!(!parse(&argv(&[])).expect("an empty argv was refused"));
        assert!(parse(&argv(&["--count"])).expect("a legitimate operand was refused"));
        for bad in ["--help", "-h", "--counts", "count"] {
            let err = parse(&argv(&[bad])).expect_err("an unrecognized operand was absorbed");
            assert!(err.contains(bad), "the refusal named no offender: {}", err);
            assert!(
                err.contains("--emit compare-settings-allow"),
                "the refusal printed the holder's old spelling: {}",
                err
            );
        }
    }

    // spec: guard-kit/SPEC.md §compare-settings-allow — the breadth question is the redundancy
    // question's predicate with the arguments swapped, and the declaration lookup is on the exact
    // glob string, so a declaration one character off silences nothing
    #[test]
    fn the_partition_swaps_the_arguments_and_looks_the_declaration_up_exactly() {
        let local = vec!["Bash(git *)".to_string(), "Bash(gh *)".to_string()];
        let committed = vec!["Bash(git status)".to_string()];
        let probes = vec![
            "Bash(git reset --hard)".to_string(),
            "Bash(gh repo delete)".to_string(),
        ];
        let declared = vec![("Bash(git *)".to_string(), "sandbox repo".to_string())];
        let r = partition(&local, &committed, &probes, &declared);
        assert!(r.redundant.is_empty(), "a committed literal granted a local glob");
        assert_eq!(r.too_broad, vec!["Bash(gh *)  \u{2287}  Bash(gh repo delete)"]);
        assert_eq!(
            r.declared,
            vec!["Bash(git *)  \u{2287}  Bash(git reset --hard)  \u{2014} sandbox repo"]
        );
        let off = vec![("Bash(git*)".to_string(), "one character off".to_string())];
        let r = partition(&local, &committed, &probes, &off);
        assert!(r.declared.is_empty(), "an inexact declaration silenced an entry");
        assert_eq!(r.too_broad.len(), 2);
    }

    // spec: guard-kit/SPEC.md §compare-settings-allow — an unreadable allowlist never reads as an
    // empty one: an unparseable overlay takes the no-overlay path and an unparseable committed
    // file is a refusal, while an absent one is honestly empty
    #[test]
    fn an_unparseable_allowlist_is_distinguished_from_an_absent_one() {
        let dir = std::env::temp_dir().join(format!("cw-csa-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot make the fixture dir");
        let bad = dir.join("bad.json");
        std::fs::write(&bad, "{not json").expect("cannot write the fixture");
        assert!(matches!(
            read_allow(&bad.display().to_string()),
            AllowRead::Unparseable
        ));
        let good = dir.join("good.json");
        std::fs::write(&good, r#"{"permissions":{"allow":["Bash(ls)","Read(x)"]}}"#)
            .expect("cannot write the fixture");
        match read_allow(&good.display().to_string()) {
            AllowRead::Entries(e) => assert_eq!(e, vec!["Bash(ls)", "Read(x)"]),
            _ => panic!("a readable allowlist did not parse"),
        }
        assert!(matches!(
            read_allow(&dir.join("nothing.json").display().to_string()),
            AllowRead::Absent
        ));
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: guard-kit/SPEC.md §Layout and configuration — an empty probe set omits the whole
    // breadth section, and `--count` still answers two integers
    #[test]
    fn an_empty_probe_set_omits_the_breadth_section_entirely() {
        let r = partition(
            &["Bash(git *)".to_string()],
            &["Bash(git *)".to_string()],
            &[],
            &[],
        );
        let out = render(".claude/settings.local.json", &r, &[], false);
        assert!(out.contains("settings allowlist redundancy"));
        assert!(
            !out.contains("settings allowlist breadth"),
            "an empty probe set printed a breadth section: {}",
            out
        );
        assert_eq!(render(".claude/settings.local.json", &r, &[], true), "1 0\n");
    }
}
