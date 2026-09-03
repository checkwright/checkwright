// spec: lifecycle-kit/SPEC.md §check-merge-attrs — bidirectional parity between the derived
// iteration-scoped supersede set and the merge=iteration-scoped lines in .gitattributes, plus
// forward-only parity for the derived union set
use crate::stages;
use std::path::Path;

// spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — the rule the contract states
// is a set difference, so the port implements one rather than transliterating the shell form's
// `comm` pipeline and inheriting `sort`'s locale-dependent collation with it
fn sorted_unique(v: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = v.into_iter().filter(|s| !s.is_empty()).collect();
    out.sort();
    out.dedup();
    out
}

fn difference(a: &[String], b: &[String]) -> Vec<String> {
    a.iter().filter(|x| !b.contains(x)).cloned().collect()
}

// spec: lifecycle-kit/SPEC.md §check-merge-attrs — a `.gitattributes` line's first field is
// the pathspec and the rest are attributes; a comment line carries neither
fn attributed(text: &str, attr: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.trim_start().starts_with('#') {
            continue;
        }
        let mut f = line.split_whitespace();
        let Some(path) = f.next() else { continue };
        if f.any(|a| a == attr) {
            out.push(path.to_string());
        }
    }
    sorted_unique(out)
}

pub fn run(args: &[String]) -> i32 {
    let attrs = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => ".gitattributes".to_string(),
    };

    let derived = match stages::supersede_set() {
        Ok(v) => sorted_unique(v),
        Err(e) => {
            eprintln!("check-merge-attrs: {}", e);
            return 2;
        }
    };
    if derived.is_empty() {
        eprintln!("check-merge-attrs: the derived iteration-scoped supersede set is empty — the state machine names no boundary-truncated surface (a lifecycle always owns at least its state + lesson-evidence files)");
        return 2;
    }
    let union_derived = match stages::union_set() {
        Ok(v) => sorted_unique(v),
        Err(e) => {
            eprintln!("check-merge-attrs: {}", e);
            return 2;
        }
    };
    if union_derived.is_empty() {
        eprintln!("check-merge-attrs: the derived union-merge set is empty — the state machine names no append-only union surface (a lifecycle always owns at least its gap inbox)");
        return 2;
    }

    let (have, union_have) = if Path::new(&attrs).exists() {
        let text = match std::fs::read(&attrs) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(_) => {
                eprintln!(
                    "check-merge-attrs: {} exists but is not readable — the parity cannot be checked",
                    attrs
                );
                return 2;
            }
        };
        (
            attributed(&text, "merge=iteration-scoped"),
            attributed(&text, "merge=union"),
        )
    } else {
        (Vec::new(), Vec::new())
    };

    let missing = difference(&derived, &have);
    let extra = difference(&have, &derived);
    // spec: lifecycle-kit/SPEC.md §check-merge-attrs — forward only: the reverse edge is
    // deliberately absent, because merge=union is git-native and a consumer's own append log
    // legitimately carries it
    let union_missing = difference(&union_derived, &union_have);

    if !missing.is_empty() || !extra.is_empty() || !union_missing.is_empty() {
        println!(
            "check-merge-attrs: {} merge attributes are out of parity with the derived supersede/union sets:",
            attrs
        );
        for p in &missing {
            println!("  supersede path with no merge=iteration-scoped attribute (the merge-supersede rule is unmechanized for it): {}", p);
        }
        for p in &extra {
            println!("  merge=iteration-scoped on a path outside the derived set (a smuggled ours-driver silently discards merge content on a real surface): {}", p);
        }
        for p in &union_missing {
            println!("  union-merge path with no merge=union attribute (a gap filed on either side of a concurrent merge would be silently dropped): {}", p);
        }
        println!("  help: regenerate the marker block — bash gate-sdk/bin/run-gates.sh --install-lifecycle — which writes one 'merge=iteration-scoped' line per boundary-truncated surface (LIFECYCLE_KIT_STATE_FILE, LIFECYCLE_KIT_LESSON_EVIDENCE_FILE, LIFECYCLE_KIT_SURVEY_RECORD_FILE, and each LIFECYCLE_KIT_BOUNDARY_TRUNCATE member) and one 'merge=union' line per union surface (LIFECYCLE_KIT_GAP_INBOX_FILE). Remove any hand-added merge=iteration-scoped attribute on a path outside the supersede set.");
        return 1;
    }

    println!(
        "MERGE-ATTRS: clean ({} carries a merge=iteration-scoped line for each of the {} derived iteration-scoped surface(s) and no others, and a merge=union line for each of the {} union surface(s))",
        attrs,
        derived.len(),
        union_derived.len()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_set_difference_is_one_directional_and_membership_based() {
        let a = vec!["x".to_string(), "y".to_string()];
        let b = vec!["y".to_string(), "z".to_string()];
        assert_eq!(difference(&a, &b), vec!["x".to_string()]);
        assert_eq!(difference(&b, &a), vec!["z".to_string()]);
    }

    #[test]
    fn only_a_non_comment_line_naming_the_attribute_contributes_its_pathspec() {
        let t = "# c merge=union\na.txt merge=iteration-scoped\nb.txt merge=union\nc.txt text\n  # x merge=union\nb.txt merge=union\n";
        assert_eq!(attributed(t, "merge=union"), vec!["b.txt".to_string()]);
        assert_eq!(
            attributed(t, "merge=iteration-scoped"),
            vec!["a.txt".to_string()]
        );
    }
}
