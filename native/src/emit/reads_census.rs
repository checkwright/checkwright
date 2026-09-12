// spec: gate-sdk/SPEC.md §check-reads-couples — the `?` population, measured by an oracle instead
// of hand-swept: one line per registry member declaring an undecidable walk root. A projection over
// registry field 2 and nothing else — no new field, no config, no source parsing.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member rather than a hardcoded flag, for the
// family's own reason: the flag family the bridged-arm table keys is what `no_such_arm` prints, so
// an arm outside the table is an arm a mistyped `--emit` can never be steered to.

pub const KNOBS: &[&str] = &[];

// spec: gate-sdk/SPEC.md §check-reads-couples — no count line, for the same reason `--reads` has
// none: the population is derivable from the lines and a transcribed total is a second source for
// it. A caller wanting the member count counts lines; the root total sums column 2.
pub fn emit(_args: &[String]) -> Result<String, String> {
    let mut out = String::new();
    for (name, _, roots, _, _, _) in crate::gates::REGISTRY {
        let grounds: Vec<&str> = roots
            .iter()
            .filter(|(r, _, _, _)| *r == "?")
            .map(|(_, _, _, g)| *g)
            .collect();
        if grounds.is_empty() {
            continue;
        }
        // spec: gate-sdk/SPEC.md §check-reads-couples — the declared roots print their root column
        // alone. The census's question is how much of a member is bounded, not what it filters, and
        // `--reads <name>` is one command away for the field detail that would widen every line.
        let declared: Vec<&str> = roots
            .iter()
            .filter(|(r, _, _, _)| *r != "?")
            .map(|(r, _, _, _)| *r)
            .collect();
        let declared = if declared.is_empty() {
            "-".to_string()
        } else {
            declared.join(",")
        };
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\n",
            name,
            grounds.len(),
            declared,
            grounds.join(",")
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    // spec: gate-sdk/SPEC.md §check-reads-couples — the arm is held to the registry it projects,
    // never to a transcribed figure: each assertion recomputes its side from `REGISTRY` so the
    // test cannot pass against a stale copy of the census.
    #[test]
    fn census_lines_match_the_registry() {
        let body = super::emit(&[]).expect("census emits");
        let mut seen = 0usize;
        for line in body.lines() {
            let cols: Vec<&str> = line.split('\t').collect();
            assert_eq!(cols.len(), 4, "four columns per line: {}", line);
            let roots = crate::gates::roots(cols[0]).expect("a census line names a live member");
            let undecidable = roots.iter().filter(|(r, _, _, _)| *r == "?").count();
            let grounds: Vec<&str> =
                roots.iter().filter(|(r, _, _, _)| *r == "?").map(|(_, _, _, g)| *g).collect();
            assert_eq!(cols[3], grounds.join(","), "{}'s ground column is its grounds", cols[0]);
            assert!(undecidable > 0, "{} declares no ? yet is censused", cols[0]);
            assert_eq!(
                cols[1].parse::<usize>().expect("column 2 is a count"),
                undecidable,
                "{}'s ? count is the registry's",
                cols[0]
            );
            let declared = roots.iter().filter(|(r, _, _, _)| *r != "?").count();
            if declared == 0 {
                assert_eq!(cols[2], "-", "{} declares nothing and prints -", cols[0]);
            } else {
                assert_eq!(
                    cols[2].split(',').count(),
                    declared,
                    "{}'s declared-root column is its declared roots",
                    cols[0]
                );
            }
            seen += 1;
        }
        let expected = crate::gates::REGISTRY
            .iter()
            .filter(|(_, _, roots, _, _, _)| roots.iter().any(|(r, _, _, _)| *r == "?"))
            .count();
        assert_eq!(seen, expected, "every member declaring a ? is censused");
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — a member declaring no `?` is absent, which is
    // what makes the line count the population rather than the registry.
    #[test]
    fn a_member_with_no_undecidable_root_is_absent() {
        let body = super::emit(&[]).expect("census emits");
        let censused: Vec<&str> = body.lines().filter_map(|l| l.split('\t').next()).collect();
        let mut checked = 0usize;
        for (name, _, roots, _, _, _) in crate::gates::REGISTRY {
            if roots.iter().any(|(r, _, _, _)| *r == "?") {
                continue;
            }
            assert!(!censused.contains(name), "{} declares no ? but is censused", name);
            checked += 1;
        }
        assert!(checked > 0, "the registry carries a member with no ? to check against");
    }
}
