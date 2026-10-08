// spec: lifecycle-kit/SPEC.md §The stage-contract arm — a stage's contract as one document for a
// reader that loads no skill and holds no write path: the frame, a separator line, the stage skill
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Emit` because the contract is a document and
// every failure is exit 2, the variant's own collapse
use crate::stages;
use crate::walk;

pub const KNOBS: &[&str] = &[
    "LIFECYCLE_KIT_STAGES",
    "LIFECYCLE_KIT_SKILLS_DIR",
    "LIFECYCLE_KIT_STAGE_CONTRACT_FRAME",
];
pub const USAGE: &str = "usage: --emit stage-contract <stage>\n  prints the stage-contract frame, a separator line and the stage's skill, for a reader that loads no skill and writes nothing";

// spec: lifecycle-kit/SPEC.md §The stage-contract arm — the frame closed by a newline, the one
// separator line naming the stage, then the skill verbatim
pub fn compose(frame: &str, stage: &str, skill: &str) -> String {
    let mut out = String::with_capacity(frame.len() + skill.len() + 64);
    out.push_str(frame);
    if !frame.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(&format!("--- stage contract: {} ---\n", stage));
    out.push_str(skill);
    out
}

fn stage_of<'a>(args: &'a [String], known: &[String]) -> Result<&'a str, String> {
    let fields = super::file_survey::positionals(args, "stage").map_err(|e| format!("{}\n{}", e, USAGE))?;
    match fields {
        [one] if stages::stage_known(known, one) => Ok(one),
        [one] => Err(format!(
            "'{}' is not a stage in LIFECYCLE_KIT_STAGES ({})\n{}",
            one,
            known.join(" "),
            USAGE
        )),
        [] => Err(USAGE.to_string()),
        _ => Err(format!("one stage is taken, and {} operands were given\n{}", fields.len(), USAGE)),
    }
}

// spec: lifecycle-kit/SPEC.md §The stage-contract arm — the arm resolves no template: a shim's
// directive names its template by a path the reader opens in its clone
fn document(stage: &str, skills_dir: &str, frame_path: &str) -> Result<String, String> {
    let skill_path = format!("{}/{}.md", skills_dir.trim_end_matches('/'), stage);
    let skill = super::read_text(&skill_path)
        .map_err(|_| format!("no skill file for stage '{}' at {} (LIFECYCLE_KIT_SKILLS_DIR)", stage, skill_path))?;
    let frame = super::read_text(frame_path)
        .map_err(|_| format!("no frame file at {} (LIFECYCLE_KIT_STAGE_CONTRACT_FRAME)", frame_path))?;
    if frame.trim().is_empty() {
        return Err(format!("the frame file {} is empty (LIFECYCLE_KIT_STAGE_CONTRACT_FRAME)", frame_path));
    }
    Ok(compose(&frame, stage, &skill))
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let known = stages::stages()?;
    let stage = stage_of(args, &known)?;
    let anchored = |knob: &str| super::enter_stage::repo_anchored(&walk::knob_scalar(knob)?);
    document(stage, &anchored("LIFECYCLE_KIT_SKILLS_DIR")?, &anchored("LIFECYCLE_KIT_STAGE_CONTRACT_FRAME")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    struct Tree(std::path::PathBuf);

    impl Tree {
        fn new(tag: &str) -> Tree {
            let root = std::env::temp_dir().join(format!("checkwright-stage-contract.{}.{}", tag, std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(root.join("skills")).expect("the sandbox must be creatable");
            Tree(root)
        }
        fn write(&self, rel: &str, body: &str) -> String {
            let p = self.0.join(rel);
            std::fs::write(&p, body).expect("a sandbox file");
            p.display().to_string()
        }
        fn skills(&self) -> String {
            self.0.join("skills").display().to_string()
        }
    }

    impl Drop for Tree {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    const SHIM: &str = "Execute the template at kit/templates/stages/align.md, applying the bindings below.\n\n## Bindings\n\n**gate** — the battery.\n";
    const STANDALONE: &str = "The align stage. Audit the specs.\n\nNo template is bound.";

    // spec: lifecycle-kit/SPEC.md §The stage-contract arm — both adoption modes ride one rule: the
    // skill lands verbatim after the frame and the separator, a shim's directive included
    #[test]
    fn the_document_is_the_frame_the_separator_and_the_skill_verbatim() {
        let t = Tree::new("compose");
        let frame = t.write("frame.md", "You write nothing.\n");
        t.write("skills/align.md", SHIM);
        t.write("skills/audit.md", STANDALONE);
        assert_eq!(
            document("align", &t.skills(), &frame).as_deref(),
            Ok(format!("You write nothing.\n--- stage contract: align ---\n{}", SHIM).as_str())
        );
        assert_eq!(
            document("audit", &t.skills(), &frame).as_deref(),
            Ok(format!("You write nothing.\n--- stage contract: audit ---\n{}", STANDALONE).as_str())
        );
        let unterminated = t.write("frame.md", "You write nothing.");
        let doc = document("align", &t.skills(), &unterminated).expect("an unterminated frame composes");
        assert!(doc.starts_with("You write nothing.\n--- stage contract: align ---\n"), "{}", doc);
    }

    #[test]
    fn an_absent_skill_and_an_absent_or_empty_frame_are_refused() {
        let t = Tree::new("refusals");
        let frame = t.write("frame.md", "You write nothing.\n");
        let err = document("align", &t.skills(), &frame).expect_err("an absent skill composed");
        assert!(err.contains("LIFECYCLE_KIT_SKILLS_DIR") && err.contains("align.md"), "{}", err);
        t.write("skills/align.md", SHIM);
        let absent = t.0.join("none.md").display().to_string();
        let err = document("align", &t.skills(), &absent).expect_err("an absent frame composed");
        assert!(err.contains("LIFECYCLE_KIT_STAGE_CONTRACT_FRAME"), "{}", err);
        let empty = t.write("empty.md", " \n\n");
        let err = document("align", &t.skills(), &empty).expect_err("an empty frame composed");
        assert!(err.contains("is empty"), "{}", err);
    }

    #[test]
    fn a_stage_outside_the_roster_and_a_surplus_operand_are_refused() {
        let known = strings(&["scope", "align"]);
        assert_eq!(stage_of(&strings(&["align"]), &known), Ok("align"));
        for bad in [&["nope"][..], &["align", "scope"], &[], &["--help"]] {
            let err = stage_of(&strings(bad), &known).expect_err("a malformed argv was admitted");
            assert!(err.contains("usage: --emit stage-contract"), "{:?}: {}", bad, err);
        }
    }
}
