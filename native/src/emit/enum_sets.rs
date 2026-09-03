// spec: canon-kit/SPEC.md §check-prose-enum — the bundled enum-set emitter: the queue tag
// vocabulary plus the derived roster families over the kit tree, one `<set-name>`⇥`<member>` line
// per member, every member read from the tree or from the gate that owns it
// spec: gate-sdk/SPEC.md §The non-gate arm — a two-kit declared roster, resolved by the
// partitioning bridge a slice at a time
use crate::proc;
use crate::walk;

pub const KNOBS: &[&str] = &["GATE_KIT_ROOTS_REL", "QUEUE_KIT_LESSON_TAGS"];

// spec: canon-kit/SPEC.md §check-prose-enum — the Lessons channel is queue-kit's own `[attend]`
// (queue-kit/SPEC.md §The Lessons Learned channel) plus the consumer's configured harvest tags;
// every other governed tag is a task/selection tag
const ATTEND: &str = "attend";

// spec: canon-kit/SPEC.md §check-prose-enum — the tag vocabulary is referenced off the class table
// check-tag-lead-line owns, never re-listed, so a rename cannot leave two spellings disagreeing;
// a compiled reference is unambiguous, which is why the shell form's parse anchors retire
fn all_tags() -> Vec<String> {
    let mut out: Vec<String> = crate::gates::tag_lead_line::CLASSES
        .iter()
        .map(|c| crate::gates::tag_lead_line::tag_name(c).to_string())
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

// spec: canon-kit/SPEC.md §check-prose-enum — a member is a file basename, never a path: the
// basename matches prose spelling the file kit-relative, repo-relative or bare, because the
// matcher's word boundary accepts the leading slash
fn emit_set(out: &mut String, set: &str, members: &[String]) {
    for m in members {
        out.push_str(set);
        out.push('\t');
        out.push_str(m.rsplit('/').next().unwrap_or(m));
        out.push('\n');
    }
}

// spec: canon-kit/SPEC.md §check-prose-enum — the family listing is *tracked*, not a filesystem
// walk, so an untracked new sibling does not enrol
// spec: gate-sdk/SPEC.md §The port-candidate criteria — `git` is criterion 7's one sanctioned
// exception on GATE_SDK_PROGRAM_FLOOR
fn tracked_under(dir: &str, suffix: &str) -> Result<Vec<String>, String> {
    let listed = proc::run("git", &["ls-files", "--", dir])?;
    let text = listed
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())
        .unwrap_or_default();
    let prefix = format!("{}/", dir);
    Ok(text
        .lines()
        .filter(|p| match p.strip_prefix(&prefix) {
            Some(rest) => !rest.contains('/') && rest.ends_with(suffix),
            None => false,
        })
        .map(str::to_string)
        .collect())
}

pub fn emit(_args: &[String]) -> Result<String, String> {
    let lesson_tags = walk::knob_array("QUEUE_KIT_LESSON_TAGS")?;
    let mut lessons: Vec<String> = vec![ATTEND.to_string()];
    lessons.extend(lesson_tags);

    let mut out = String::new();
    for t in all_tags() {
        if !lessons.contains(&t) {
            out.push_str(&format!("queue-task-tag\t{}\n", t));
        }
    }
    for t in &lessons {
        out.push_str(&format!("queue-lessons-tag\t{}\n", t));
    }

    // spec: canon-kit/SPEC.md §check-prose-enum — the kit-root anchor for every derived family is
    // gate-sdk's own kit-root derivation, so the sets cannot enumerate a different tree than the
    // battery runs on
    let kits: Vec<String> = walk::kit_roots_rel()?
        .into_iter()
        .filter(|k| !k.is_empty())
        .map(|k| k.trim_end_matches('/').to_string())
        .collect();
    if kits.is_empty() {
        return Err("gate_kit_roots_rel enumerated no kit roots".to_string());
    }

    for kit in &kits {
        // spec: canon-kit/SPEC.md §check-prose-enum — a lib/ that tracks no top-level *.sh is a
        // layout this derivation can no longer read, so it fail-closes rather than emitting the
        // silently empty set
        let lib = format!("{}/lib", kit);
        if std::path::Path::new(&lib).is_dir() {
            let libs = tracked_under(&lib, ".sh")?;
            if libs.is_empty() {
                return Err(format!("{}/lib tracks no top-level *.sh", kit));
            }
            emit_set(&mut out, &format!("{}-lib", kit), &libs);
        }
        // spec: canon-kit/SPEC.md §check-prose-enum — a gate-tests/ holding only good/bad fixture
        // directories ships no bespoke unit test; that empty set is a measured normal state, not a
        // broken derivation
        let tests = format!("{}/gate-tests", kit);
        if std::path::Path::new(&tests).is_dir() {
            let found = tracked_under(&tests, ".test.sh")?;
            emit_set(&mut out, &format!("{}-gate-test", kit), &found);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: canon-kit/SPEC.md §check-prose-enum — the tag vocabulary comes off the class table the
    // gate owns, terminator-stripped by that module's own holder, sorted and deduped
    #[test]
    fn the_tag_vocabulary_is_the_gates_own_class_table_terminator_stripped() {
        let tags = all_tags();
        assert!(tags.contains(&"blocked-by".to_string()));
        assert!(tags.contains(&"design-pending".to_string()));
        assert!(tags.contains(&ATTEND.to_string()));
        assert!(
            !tags.iter().any(|t| t.ends_with(':') || t.ends_with(']')),
            "a bracket terminator survived the strip: {:?}",
            tags
        );
        let mut sorted = tags.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(tags, sorted, "the vocabulary is not sorted and deduped");
    }

    // spec: canon-kit/SPEC.md §check-prose-enum — a member is a basename and the listing is
    // top-level only: a nested tracked path is not a member of the family
    #[test]
    fn a_member_is_a_top_level_basename() {
        let mut out = String::new();
        emit_set(&mut out, "x-lib", &["x/lib/a.sh".to_string()]);
        assert_eq!(out, "x-lib\ta.sh\n");
    }
}
