// spec: gate-sdk/SPEC.md §check-install-disposition — every shipped gate declares one install
// disposition, every zero-config gate is registrable in its kit's smoke, and the installer keeps
// no second copy of the roster
use crate::fresh;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §The install disposition — the vocabulary is closed, so an unrecognised
// value is a finding rather than a pass-through: it names install-time reachability and nothing
// else. The help line's alternation is joined from this set rather than spelled twice.
const VOCAB: &[&str] = &["zero-config", "on-surface", "never"];
const ZERO_CONFIG: &str = "zero-config";
const RECIPE: &str = "installer/lib/common/recipe.sh";
const DECL: &str = "# install:";

fn basename(p: &str) -> &str {
    p.rsplit_once('/').map(|(_, b)| b).unwrap_or(p)
}

// spec: gate-sdk/SPEC.md §check-install-disposition — `grep '^# install:'`: the count assertion
// reads the anchored prefix alone, so a line declaring nothing after the colon still counts as a
// declaration and is judged by its value rather than skipped as absent
fn install_lines(text: &str) -> usize {
    fresh::file_lines(text)
        .iter()
        .filter(|l| l.starts_with(DECL))
        .count()
}

// spec: gate-sdk/SPEC.md §check-install-disposition — awk's `sub(/^# install:[[:space:]]+/, "")`
// then `print $1`: the first line whose prefix *and* its trailing whitespace run both match, read
// down to its first field; one matching neither declares the empty value the vocabulary rejects.
fn declared_value(text: &str) -> String {
    for line in fresh::file_lines(text) {
        let Some(rest) = line.strip_prefix(DECL) else {
            continue;
        };
        let trimmed = rest.trim_start_matches(is_awk_space);
        if trimmed.len() == rest.len() {
            continue;
        }
        return trimmed
            .split(is_awk_space)
            .next()
            .unwrap_or_default()
            .to_string();
    }
    String::new()
}

// spec: gate-sdk/SPEC.md §check-gate-assertions — POSIX `[[:space:]]` inside one line, which is
// also awk's own field separator set: the C-locale reading the shell form ran under
fn is_awk_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\x0b' | '\x0c')
}

// spec: gate-sdk/SPEC.md §check-install-disposition — `grep -qxF "$name"`: a whole-line literal
// match, so a roster entry is the bare gate name on a line of its own and a name mentioned inside
// a comment or a path registers nothing
fn smoke_registers(smoke: &str, name: &str) -> bool {
    let text = std::fs::read(smoke)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    fresh::file_lines(&text).contains(&name)
}

// spec: gate-sdk/SPEC.md §check-install-disposition — the `§`-prefixed occurrence stripped before
// the match: `gsub(/§[A-Za-z0-9_-]+/, "")`, which needs at least one name character after the
// sign, so a bare `§` is left in place and matches nothing on its own
fn strip_section_citations(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(i) = rest.find('§') {
        let (before, at) = rest.split_at(i);
        let after = &at['§'.len_utf8()..];
        let taken = after
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
            .unwrap_or(after.len());
        if taken == 0 {
            out.push_str(before);
            out.push('§');
            rest = after;
            continue;
        }
        out.push_str(before);
        rest = &after[taken..];
    }
    out.push_str(rest);
    out
}

// spec: gate-sdk/SPEC.md §check-install-disposition — `check-[a-z0-9]+(-[a-z0-9]+)*`: the trailing
// group is optional, so the whole pattern matches exactly where `check-` is followed by one or
// more name characters. The 1-based line numbers are awk's `FNR`.
fn recipe_hits(text: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for (i, line) in fresh::file_lines(text).iter().enumerate() {
        let stripped = strip_section_citations(line);
        let mut rest = stripped.as_str();
        while let Some(at) = rest.find("check-") {
            rest = &rest[at + "check-".len()..];
            if rest.starts_with(|c: char| c.is_ascii_digit() || c.is_ascii_lowercase()) {
                out.push(i + 1);
                break;
            }
        }
    }
    out
}

pub fn run(args: &[String]) -> i32 {
    // spec: gate-sdk/SPEC.md §check-install-disposition — the positional root, and the toplevel it
    // falls back to, anchor the installer recipe alone; the kit roots arrive already spelled
    // against the invoking directory, which §lib/gate.sh owns as the one place that anchor is set
    let root = match args.first() {
        Some(r) if !r.is_empty() => r.clone(),
        _ => match fresh::toplevel() {
            Ok(t) => t,
            Err(_) => {
                eprintln!("check-install-disposition: not a git repository and no root argument");
                return 2;
            }
        },
    };
    if !Path::new(&root).is_dir() {
        eprintln!("check-install-disposition: root not found: {}", root);
        return 2;
    }

    let kit_roots = match walk::kit_roots_abs() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-install-disposition: {}", e);
            return 2;
        }
    };
    if kit_roots.is_empty() {
        eprintln!("check-install-disposition: no kit roots enumerated");
        return 2;
    }

    let mut findings: Vec<String> = Vec::new();
    let (mut declared, mut zeroconf) = (0usize, 0usize);
    for abs in &kit_roots {
        let kit = basename(abs);
        if !Path::new(&format!("{}/checks", abs)).is_dir() {
            continue;
        }
        let smoke = format!("{}/smoke/install.sh", abs);
        // spec: gate-sdk/SPEC.md §check-install-disposition — both declaration spellings, in the
        // shell glob's own order: every `check-*.sh` then every `check-*.gate`, each sorted, so a
        // finding list is the same sequence on either substrate
        let hits = match walk::glob_files(
            Path::new(abs),
            &["checks/check-*.sh".to_string(), "checks/check-*.gate".to_string()],
        ) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("check-install-disposition: {}", e);
                return 2;
            }
        };
        for f in &hits {
            let path = f.display().to_string();
            let fname = basename(&path).to_string();
            let name = fname
                .rsplit_once('.')
                .map(|(a, _)| a)
                .unwrap_or(fname.as_str());
            let Ok(bytes) = std::fs::read(f) else {
                eprintln!(
                    "check-install-disposition: unreadable gate header: {}/checks/{}",
                    kit, fname
                );
                return 2;
            };
            let text = String::from_utf8_lossy(&bytes).into_owned();

            // assertion A: exactly one `# install:` line, its value in the closed vocabulary
            let count = install_lines(&text);
            if count != 1 {
                findings.push(format!(
                    "{}/checks/{}: {} '{}' line(s) where a gate declares exactly one",
                    kit, fname, count, DECL
                ));
                continue;
            }
            let value = declared_value(&text);
            if !VOCAB.contains(&value.as_str()) {
                findings.push(format!(
                    "{}/checks/{}: install disposition '{}' is outside the closed vocabulary",
                    kit,
                    fname,
                    if value.is_empty() {
                        "<empty>"
                    } else {
                        value.as_str()
                    }
                ));
                continue;
            }
            declared += 1;
            if value != ZERO_CONFIG {
                continue;
            }
            zeroconf += 1;
            // assertion B: the smoke's roster is a superset of what the installer registers, so a
            // zero-config member of a kit appears in that kit's smoke/install.sh
            if !Path::new(&smoke).is_file() {
                findings.push(format!(
                    "{}/checks/{}: declares {} where {} ships no smoke/install.sh to register it",
                    kit, fname, ZERO_CONFIG, kit
                ));
            } else if !smoke_registers(&smoke, name) {
                findings.push(format!(
                    "{}/checks/{}: declares {} but {}/smoke/install.sh does not register it",
                    kit, fname, ZERO_CONFIG, kit
                ));
            }
        }
    }
    if declared == 0 && findings.is_empty() {
        eprintln!("check-install-disposition: kit roots enumerated but no gate found under any checks/");
        return 2;
    }

    // assertion C: the installer's recipe carries no literal gate name. The file is absent in a
    // vendored consumer, which has no installer, so its absence is a skip reported on the clean
    // line and never a finding.
    let recipe = format!("{}/{}", root, RECIPE);
    let mut recipe_checked = "no";
    if Path::new(&recipe).is_file() {
        recipe_checked = "yes";
        let Ok(bytes) = std::fs::read(&recipe) else {
            eprintln!(
                "check-install-disposition: {}",
                fresh::fail_closed("awk", Some(2))
            );
            return 2;
        };
        let text = String::from_utf8_lossy(&bytes).into_owned();
        for n in recipe_hits(&text) {
            findings.push(format!(
                "{}:{}: literal gate name — the roster is derived from each gate's declaration, never listed here",
                RECIPE, n
            ));
        }
    }

    if !findings.is_empty() {
        println!("check-install-disposition: install-disposition finding(s):");
        for f in &findings {
            println!("  {}", f);
        }
        println!(
            "  help: give every gate exactly one '{} <{}>' line beside its",
            DECL,
            VOCAB.join("|")
        );
        println!("        '# graph:' directive; register each zero-config gate in its kit's");
        println!(
            "        smoke/install.sh; and keep {} free of literal",
            RECIPE
        );
        println!("        gate names — it derives the roster (gate-sdk/SPEC.md §The install disposition).");
        return 1;
    }

    println!(
        "INSTALL-DISPOSITION: clean ({} gate(s) declared, {} zero-config and registrable in their kit's smoke; recipe de-literalization checked: {})",
        declared, zeroconf, recipe_checked
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-install-disposition — the count and the value are read by two
    // different patterns, and the gap between them is where the `<empty>` finding lives
    #[test]
    fn a_declaration_with_no_value_counts_as_one_line_and_declares_nothing() {
        assert_eq!(install_lines("# install: never\n"), 1);
        assert_eq!(install_lines("# install:\n"), 1);
        assert_eq!(install_lines("#install: never\n"), 0);
        assert_eq!(install_lines("# install: a\n# install: b\n"), 2);
        assert_eq!(declared_value("# install:   on-surface  \n"), "on-surface");
        assert_eq!(declared_value("# install:\n"), "");
        assert_eq!(declared_value("# graph: x\n"), "");
        assert_eq!(
            declared_value("# install: never\n# install: sometimes\n"),
            "never"
        );
    }

    // spec: gate-sdk/SPEC.md §check-install-disposition — a `§`-prefixed occurrence is a
    // spec-section citation rather than a roster member: registering is the only thing assertion C
    // is about, so the citation is stripped and what is left is what the pattern reads
    #[test]
    fn a_section_citation_registers_nothing_but_a_bare_name_does() {
        assert_eq!(recipe_hits("see §check-graph for the shape\n"), Vec::<usize>::new());
        assert_eq!(recipe_hits("printf '%s' check-graph\n"), vec![1]);
        assert_eq!(recipe_hits("a\nb\n§check-a and check-b\n"), vec![3]);
        assert_eq!(recipe_hits("§ check-graph\n"), vec![1]);
        assert_eq!(recipe_hits("check- and check-\n"), Vec::<usize>::new());
        assert_eq!(
            strip_section_citations("§The install disposition"),
            " install disposition"
        );
    }
}
