// spec: gate-sdk/SPEC.md §check-packed-links — a packed kit README carries no link to a path the
// payload withholds, and with an empty base the packed README is its tracked source byte for byte
use crate::emit::pack_installer::resolve_own_spec_links;
use crate::gates::smoke_entry_guard::{kit_abs, kit_name, scan_root};
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-packed-links — the base the gate packs under when the tree's own
// knob is empty: an assertion that only fires for a configured publisher is one that reads clean on
// every unconfigured tree, which is a vacuous green rather than a narrower check
const PROBE_BASE: &str = "https://probe.invalid";

// spec: gate-sdk/SPEC.md §check-packed-links — the payload root the packed README sits under, so a
// link's target resolves against the same tree the adopter clicks it in; a target escaping it is a
// path the payload never carried and this gate has nothing to say about
const PAYLOAD: &str = "payload";

// spec: gate-sdk/SPEC.md §check-packed-links — a markdown link's target ends at the first
// whitespace (a link title follows it) and its fragment is not part of the path; an anchor-only
// target, an absolute URL and a root-absolute path name nothing inside the payload
fn payload_path(target: &str, leaf: &str) -> Option<(String, String)> {
    let target = target.split_whitespace().next().unwrap_or_default();
    let target = target.split('#').next().unwrap_or_default();
    // path-dialect-exempt: a markdown link target — a root-absolute one names a location in the
    // published site's namespace, never a path inside the payload
    if target.is_empty() || target.starts_with('/') {
        return None;
    }
    // spec: gate-sdk/SPEC.md §check-packed-links — a scheme-bearing target is a location, not a
    // path: the colon precedes the first separator exactly when the head is a URL scheme
    if let Some((head, _)) = target.split_once(':') {
        if !head.contains('/') {
            return None;
        }
    }
    let mut seg: Vec<&str> = vec![PAYLOAD, leaf];
    for part in target.split('/') {
        match part {
            "" | "." => continue,
            ".." => {
                if seg.len() <= 1 {
                    return None;
                }
                seg.pop();
            }
            p => seg.push(p),
        }
    }
    if seg.len() < 3 {
        return None;
    }
    // spec: gate-sdk/SPEC.md §check-packed-links — the landing path and the landing kit's own
    // relative remainder: a `../<kit>/` target lands under a *different* kit, so a finding that
    // named the linking kit's directory would name a path the target never reaches
    Some((seg.join("/"), seg[2..].join("/")))
}

// spec: gate-sdk/SPEC.md §check-packed-links — the withheld set is read from the knob and never
// re-listed, so a member added to the withhold list extends the assertion with no gate edit
fn withheld<'a>(rest: &str, withhold: &'a [String]) -> Option<&'a str> {
    withhold.iter().map(String::as_str).find(|m| {
        let m = m.trim_matches('/');
        !m.is_empty() && crate::walk::at_or_under(m, rest)
    })
}

fn targets(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find("](") {
        let after = &rest[at + 2..];
        let Some(end) = after.find(')') else { break };
        let cand = &after[..end];
        // spec: gate-sdk/SPEC.md §check-packed-links — a candidate spanning a later `](` means the
        // link started there instead, so the scan advances to it rather than reading the prose
        // between two links as one target
        if cand.contains("](") {
            rest = after;
            continue;
        }
        out.push(cand);
        rest = &after[end + 1..];
    }
    out
}

fn knob(name: &str, gate: &str) -> Option<String> {
    match walk::knob_scalar(name) {
        Ok(v) => Some(v),
        Err(e) => {
            eprintln!("{}: {}", gate, e);
            None
        }
    }
}

pub fn run(args: &[String]) -> i32 {
    const GATE: &str = "check-packed-links";
    let Some(root) = scan_root(args, GATE) else {
        return 2;
    };
    if !Path::new(&root).is_dir() {
        eprintln!("{}: root not found: {}", GATE, root);
        return 2;
    }
    let kit_roots = match walk::kit_roots() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", GATE, e);
            return 2;
        }
    };
    if kit_roots.is_empty() {
        eprintln!("{}: no kit roots enumerated", GATE);
        return 2;
    }
    let Some(withhold) = knob("GATE_SDK_PAYLOAD_WITHHOLD", GATE) else {
        return 2;
    };
    let withhold: Vec<String> = withhold.split_whitespace().map(String::from).collect();
    if withhold.is_empty() {
        eprintln!(
            "{}: GATE_SDK_PAYLOAD_WITHHOLD names nothing — there is no withheld path to assert against",
            GATE
        );
        return 2;
    }
    let Some(configured) = knob("GATE_SDK_SPEC_BASE_URL", GATE) else {
        return 2;
    };
    let base = if configured.is_empty() {
        PROBE_BASE
    } else {
        configured.as_str()
    };

    let mut findings: Vec<String> = Vec::new();
    let mut swept = 0usize;
    let mut skipped = 0usize;
    let mut resolved = 0usize;

    for raw in &kit_roots {
        let r = raw.trim_end_matches('/');
        let abs = kit_abs(&root, r);
        let leaf = kit_name(r);
        let readme = format!("{}/README.md", abs);
        let text = match std::fs::read_to_string(&readme) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                skipped += 1;
                continue;
            }
            // spec: gate-sdk/SPEC.md §Fail-closed contract — an unreadable README is exit 2, never
            // a false clean and never the withheld-target finding, which asserts about bytes read
            Err(e) => {
                eprintln!(
                    "{}: README not readable: {} ({}) — the check could not run; treating as failure (not clean)",
                    GATE, readme, e
                );
                return 2;
            }
        };
        swept += 1;

        // assertion A: with a non-empty base, no packed README links a path the payload withholds
        let (packed, count) = resolve_own_spec_links(&text, leaf, base);
        resolved += count;
        for (i, line) in packed.split('\n').enumerate() {
            for t in targets(line) {
                let Some((full, rest)) = payload_path(t, leaf) else {
                    continue;
                };
                if let Some(member) = withheld(&rest, &withhold) {
                    findings.push(format!(
                        "{}: README.md:{}: packed link target '{}' resolves to {}, which the payload withholds ('{}')",
                        leaf, i + 1, t, full, member
                    ));
                }
            }
        }

        // assertion B: with an empty base the packed README is its tracked source byte for byte
        let (unset, n) = resolve_own_spec_links(&text, leaf, "");
        if n != 0 || unset != text {
            let at = unset
                .as_bytes()
                .iter()
                .zip(text.as_bytes())
                .position(|(a, b)| a != b)
                .unwrap_or(text.len().min(unset.len()));
            findings.push(format!(
                "{}: README.md: an empty base rewrote {} link(s); first differing byte at offset {}",
                leaf, n, at
            ));
        }
    }

    if !findings.is_empty() {
        println!(
            "{}: packed kit README(s) link a path the vendored payload does not carry:",
            GATE
        );
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: a link into a withheld path dangles in every installed tree — retarget it");
        println!("        at the published location the packer resolves, or drop the path from");
        println!("        GATE_SDK_PAYLOAD_WITHHOLD so the payload carries it");
        println!("        (gate-sdk/SPEC.md §Consumer payload).");
        return 1;
    }

    println!(
        "PACKED-LINKS: clean ({} packed kit README(s) carry no withheld link target; {} own-SPEC link(s) resolved to the published location; {} kit root(s) without a README skipped)",
        swept, resolved, skipped
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-packed-links — a target is judged where it lands inside the
    // payload, so a sibling kit's withheld file is caught as readily as the README's own, and a
    // target escaping the payload root is not this gate's to judge.
    #[test]
    fn a_target_is_judged_where_it_lands_inside_the_payload() {
        let at = |t: &str| payload_path(t, "queue-kit");
        let landed = |full: &str, rest: &str| Some((full.to_string(), rest.to_string()));
        assert_eq!(at("SPEC.md"), landed("payload/queue-kit/SPEC.md", "SPEC.md"));
        assert_eq!(at("./smoke/install.sh"), landed("payload/queue-kit/smoke/install.sh", "smoke/install.sh"));
        assert_eq!(at("../gate-sdk/SPEC.md"), landed("payload/gate-sdk/SPEC.md", "SPEC.md"));
        assert_eq!(at("SPEC.md \"a title\""), landed("payload/queue-kit/SPEC.md", "SPEC.md"));
        assert_eq!(at("SPEC.md#frag"), landed("payload/queue-kit/SPEC.md", "SPEC.md"));
        assert_eq!(at("../gate-sdk/"), None);
        assert_eq!(at("../../outside/SPEC.md"), None);
        assert_eq!(at("#quick-start"), None);
        assert_eq!(at("https://h.test/queue-kit/SPEC"), None);
        assert_eq!(at("mailto:a@b.test"), None);
        assert_eq!(at("/etc/passwd"), None);
    }

    // spec: gate-sdk/SPEC.md §check-packed-links — membership is the knob's own members, a
    // directory member covering its subtree; a member no target matches contributes nothing.
    #[test]
    fn membership_is_the_knobs_own_members_and_a_directory_covers_its_subtree() {
        let w = vec!["SPEC.md".to_string(), "smoke".to_string()];
        assert_eq!(withheld("SPEC.md", &w), Some("SPEC.md"));
        assert_eq!(withheld("smoke/install.sh", &w), Some("smoke"));
        assert_eq!(withheld("smoke", &w), Some("smoke"));
        assert_eq!(withheld("README.md", &w), None);
        assert_eq!(withheld("smokey.md", &w), None);
        assert_eq!(withheld("DOCTRINE.md", &w), None);
    }

    // spec: gate-sdk/SPEC.md §check-packed-links — every link on a line is read, and an unclosed
    // one ends the scan rather than swallowing the rest of the line as a target.
    #[test]
    fn every_closed_link_on_a_line_is_a_target() {
        assert_eq!(targets("a [x](one) b [y](two#f) c"), vec!["one", "two#f"]);
        assert_eq!(targets("a [x](unclosed b [y](two)"), vec!["two"]);
        assert!(targets("no links here").is_empty());
    }
}
