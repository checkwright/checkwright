// spec: gate-sdk/SPEC.md §check-readme-roster — every kit README's gate-roster marker block
// holds name-set parity with the kit's shipped checks/ basenames, both directions
use crate::gates::smoke_entry_guard::{kit_name, scan_root};
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-readme-roster — the marker vocabulary stays a kit literal in the
// implementation rather than a knob: it is gate-sdk's own mechanism, not a consumer's document
// vocabulary, the seam ruling doctrine-kit's section headings already took
const BEGIN: &str = "<!-- gate-roster:begin -->";
const END: &str = "<!-- gate-roster:end -->";

// spec: gate-sdk/SPEC.md §check-readme-roster — marker lines may carry leading indentation,
// because a README nests the fenced block inside an install list item; the scan trims
// surrounding whitespace (the shell's `gsub(/^[ \t]+|[ \t\r]+$/, "")`) before matching
fn trimmed(line: &str) -> &str {
    line.trim_start_matches([' ', '\t'])
        .trim_end_matches([' ', '\t', '\r'])
}

// spec: gate-sdk/SPEC.md §check-readme-roster — inside the markers, a line's first
// `check-`-prefixed token is a roster name and everything after it is annotation prose the gate
// never reads; the name ends at the first character outside `[[:alnum:]_-]`
fn roster_names(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut inside = false;
    for line in text.split('\n') {
        let t = trimmed(line);
        if t == BEGIN {
            inside = true;
            continue;
        }
        if t == END {
            inside = false;
            continue;
        }
        if !inside {
            continue;
        }
        for field in line.split_whitespace() {
            if !field.starts_with("check-") {
                continue;
            }
            let lead = "check-".len();
            let end = field[lead..]
                .char_indices()
                .find(|(_, c)| !(c.is_ascii_alphanumeric() || *c == '_' || *c == '-'))
                .map(|(i, _)| lead + i)
                .unwrap_or(field.len());
            // spec: gate-sdk/SPEC.md §check-readme-roster — the `+` in the shell form's
            // `^check-[[:alnum:]_-]+`: a bare `check-` matches no name, and the scan still moves
            // to the next line rather than looking past it for a second token
            if end > lead {
                out.insert(field[..end].to_string());
            }
            break;
        }
    }
    out
}

pub fn run(args: &[String]) -> i32 {
    let root = match scan_root(args, "check-readme-roster") {
        Some(r) => r,
        None => return 2,
    };
    if !Path::new(&root).is_dir() {
        eprintln!("check-readme-roster: root not found: {}", root);
        return 2;
    }

    let kit_roots = match walk::kit_roots() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-readme-roster: {}", e);
            return 2;
        }
    };
    if kit_roots.is_empty() {
        eprintln!("check-readme-roster: no kit roots enumerated");
        return 2;
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — every kit's `checks/` listing goes through one
    // `glob_files` call anchored at the scan root, so the member observes a single walk root
    // rather than one per configured kit
    let mut relative: Vec<String> = Vec::new();
    let mut absolute: Vec<String> = Vec::new();
    for raw in &kit_roots {
        let r = raw.trim_end_matches('/');
        if r.starts_with('/') {
            absolute.push(r.to_string());
        } else {
            relative.push(format!("{}/checks/*.sh", r));
            relative.push(format!("{}/checks/*.gate", r));
        }
    }
    let mut declarations: Vec<String> = Vec::new();
    if !relative.is_empty() {
        match walk::glob_files(Path::new(&root), &relative) {
            Ok(hits) => declarations.extend(hits.iter().map(|p| p.display().to_string())),
            Err(e) => {
                eprintln!(
                    "check-readme-roster: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        }
    }
    for r in &absolute {
        match walk::glob_files(
            Path::new(r),
            &["checks/*.sh".to_string(), "checks/*.gate".to_string()],
        ) {
            Ok(hits) => declarations.extend(hits.iter().map(|p| p.display().to_string())),
            Err(e) => {
                eprintln!(
                    "check-readme-roster: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        }
    }

    let mut findings: Vec<String> = Vec::new();
    let mut help_block = false;
    let mut help_parity = false;
    let mut swept = 0usize;
    let mut skipped = 0usize;

    for raw in &kit_roots {
        let r = raw.trim_end_matches('/');
        // spec: gate-sdk/SPEC.md §check-readme-roster — the kit's path is built by the *same*
        // join the listing above walks through, so the attribution below compares two spellings
        // that cannot disagree; an absolute root replaces the base, the shell form's `/*` branch
        let abs = Path::new(&root).join(r).display().to_string();
        let kit = kit_name(r);
        if !Path::new(&format!("{}/checks", abs)).is_dir() {
            skipped += 1;
            continue;
        }
        swept += 1;

        // spec: gate-sdk/SPEC.md §check-readme-roster — both declaration spellings, attributed
        // back to the kit off each hit's own path, which is what lets the listing above be one call
        let prefix = format!("{}/checks/", abs);
        let mut shipped: BTreeSet<String> = BTreeSet::new();
        for d in &declarations {
            let Some(base) = d.strip_prefix(&prefix) else {
                continue;
            };
            if base.contains('/') {
                continue;
            }
            let stem = base
                .strip_suffix(".sh")
                .or_else(|| base.strip_suffix(".gate"));
            if let Some(s) = stem {
                shipped.insert(s.to_string());
            }
        }

        let readme = format!("{}/README.md", abs);
        let text = match std::fs::read(&readme) {
            Ok(b) => Some(String::from_utf8_lossy(&b).into_owned()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            // spec: gate-sdk/SPEC.md §check-readme-roster — an unreadable README marker scan is
            // exit 2, never a false clean and never the marker-block finding, which asserts the
            // block is absent rather than that the scan could not run
            Err(e) => {
                eprintln!(
                    "check-readme-roster: README not readable: {} ({}) — the check could not run; treating as failure (not clean)",
                    readme, e
                );
                return 2;
            }
        };
        let Some(text) = text.filter(|t| t.contains(BEGIN)) else {
            findings.push(format!(
                "{}: README.md has no gate-roster marker block beside checks/",
                kit
            ));
            help_block = true;
            continue;
        };

        let roster = roster_names(&text);
        // assertion A: every shipped check appears in the README's roster block
        for n in shipped.difference(&roster) {
            findings.push(format!(
                "{}: shipped check absent from the README roster: {}",
                kit, n
            ));
            help_parity = true;
        }
        // assertion B: every roster name resolves to a shipped check
        for n in roster.difference(&shipped) {
            findings.push(format!("{}: roster names no shipped check: {}", kit, n));
            help_parity = true;
        }
    }

    if !findings.is_empty() {
        println!("check-readme-roster: kit README gate roster(s) out of parity with checks/:");
        for f in &findings {
            println!("  {}", f);
        }
        if help_block {
            println!("  help: wrap the kit README's register-the-gates block in");
            println!("        '{}' / '{}' markers — a kit", BEGIN, END);
            println!("        shipping checks/ registers them (gate-sdk/SPEC.md §Consumer smoke).");
        }
        if help_parity {
            println!("  help: keep the marker block's check-* names in name-set parity with the");
            println!("        kit's checks/ script basenames — add the missing roster line or drop");
            println!("        the stale one.");
        }
        return 1;
    }

    println!(
        "README-ROSTER: clean ({} kit README roster(s) in name-set parity with checks/; {} root(s) without checks/ skipped)",
        swept, skipped
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_first_check_token_between_indented_markers_is_a_roster_name() {
        let text = "prose naming check-outside must not join\n\
                    \x20  <!-- gate-roster:begin -->\n\
                    \x20  ```\n\
                    \x20  check-alpha-one   # the first, check-decoy in the annotation\n\
                    \x20  check-alpha-two\n\
                    \x20  ```\n\
                    \x20  <!-- gate-roster:end -->\n\
                    check-after must not join either\n";
        let names = roster_names(text);
        let got: Vec<&str> = names.iter().map(String::as_str).collect();
        assert_eq!(got, vec!["check-alpha-one", "check-alpha-two"]);
    }

    // spec: gate-sdk/SPEC.md §check-readme-roster — the name ends at the first character outside
    // the token class, so a backticked or punctuated roster line still yields the bare name
    #[test]
    fn a_roster_name_ends_at_the_first_character_outside_the_token_class() {
        let text = format!("{}\ncheck-alpha-one`, annotated\n{}\n", BEGIN, END);
        let names = roster_names(&text);
        assert_eq!(
            names.iter().map(String::as_str).collect::<Vec<_>>(),
            vec!["check-alpha-one"]
        );
    }
}
