// spec: lifecycle-kit/SPEC.md §The stage-contract arm — a stage's contract as one document for a
// reader that loads no skill and holds no commit path: the frame, a separator line, the stage skill
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Emit` because the contract is a document and
// every failure is exit 2, the variant's own collapse
use crate::stages;
use crate::walk;

pub const KNOBS: &[&str] = &[
    "LIFECYCLE_KIT_STAGES",
    "LIFECYCLE_KIT_SKILLS_DIR",
    "LIFECYCLE_KIT_STAGE_CONTRACT_FRAME",
    "LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME",
];
pub const USAGE: &str = "usage: --emit stage-contract <stage> [--mode audit|sweep]\n  prints the mode's stage-contract frame, a separator line and the stage's skill, for a reader that loads no skill and commits nothing";

// spec: lifecycle-kit/SPEC.md §The stage-contract arm — the mode word is `--foreign-run`'s, `audit`
// by default, and each mode reads its own frame knob alone
#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    Audit,
    Sweep,
}

impl Mode {
    fn frame_knob(self) -> &'static str {
        match self {
            Mode::Audit => "LIFECYCLE_KIT_STAGE_CONTRACT_FRAME",
            Mode::Sweep => "LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME",
        }
    }
}

// spec: lifecycle-kit/SPEC.md §The stage-contract arm — `--mode` is lifted out ahead of a `--`
// separator, so every other dash-led token meets the positional refusal it met before
fn mode_of(args: &[String]) -> Result<(Mode, Vec<String>), String> {
    let mut mode: Option<Mode> = None;
    let mut rest: Vec<String> = Vec::with_capacity(args.len());
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--" {
            rest.push(a.clone());
            rest.extend(it.by_ref().cloned());
            break;
        }
        if a != "--mode" {
            rest.push(a.clone());
            continue;
        }
        if mode.is_some() {
            return Err(format!("--mode is taken once\n{}", USAGE));
        }
        mode = Some(match it.next().map(String::as_str) {
            Some("audit") => Mode::Audit,
            Some("sweep") => Mode::Sweep,
            None | Some("--") => return Err(format!("--mode takes a value, audit or sweep\n{}", USAGE)),
            Some(other) => return Err(format!("'{}' is not a mode; --mode takes audit or sweep\n{}", other, USAGE)),
        });
    }
    Ok((mode.unwrap_or(Mode::Audit), rest))
}

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
fn document(stage: &str, skills_dir: &str, frame_path: &str, frame_knob: &str) -> Result<String, String> {
    let skill_path = format!("{}/{}.md", skills_dir.trim_end_matches('/'), stage);
    let skill = super::read_text(&skill_path)
        .map_err(|_| format!("no skill file for stage '{}' at {} (LIFECYCLE_KIT_SKILLS_DIR)", stage, skill_path))?;
    let frame = super::read_text(frame_path).map_err(|_| format!("no frame file at {} ({})", frame_path, frame_knob))?;
    if frame.trim().is_empty() {
        return Err(format!("the frame file {} is empty ({})", frame_path, frame_knob));
    }
    Ok(compose(&frame, stage, &skill))
}

fn contract(args: &[String], known: &[String], path_of: impl Fn(&str) -> Result<String, String>) -> Result<String, String> {
    let (mode, rest) = mode_of(args)?;
    let stage = stage_of(&rest, known)?;
    let knob = mode.frame_knob();
    document(stage, &path_of("LIFECYCLE_KIT_SKILLS_DIR")?, &path_of(knob)?, knob)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let known = stages::stages()?;
    contract(args, &known, |knob| super::enter_stage::repo_anchored(&walk::knob_scalar(knob)?))
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
    const AUDIT: &str = "LIFECYCLE_KIT_STAGE_CONTRACT_FRAME";
    const SWEEP: &str = "LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME";

    // spec: lifecycle-kit/SPEC.md §The stage-contract arm — the knob paths a tree binds, the sweep
    // frame's left absent where `sweep` is None
    fn bound(t: &Tree, sweep: Option<&str>) -> impl Fn(&str) -> Result<String, String> {
        let skills = t.skills();
        let audit = t.write("audit-frame.md", "You write nothing.\n");
        let sweep = match sweep {
            Some(body) => t.write("sweep-frame.md", body),
            None => t.0.join("no-sweep-frame.md").display().to_string(),
        };
        move |knob: &str| match knob {
            "LIFECYCLE_KIT_SKILLS_DIR" => Ok(skills.clone()),
            AUDIT => Ok(audit.clone()),
            SWEEP => Ok(sweep.clone()),
            other => Err(format!("unbound knob {}", other)),
        }
    }

    // spec: lifecycle-kit/SPEC.md §The stage-contract arm — `sweep` composes from its own frame, on
    // either side of the stage and ahead of a `--`, and `audit` is the default
    #[test]
    fn a_sweep_document_is_composed_from_the_sweep_frame() {
        let t = Tree::new("sweep");
        t.write("skills/build.md", SHIM);
        let known = strings(&["build", "align"]);
        let path_of = bound(&t, Some("You edit your clone.\n"));
        let want = format!("You edit your clone.\n--- stage contract: build ---\n{}", SHIM);
        for argv in [&["build", "--mode", "sweep"][..], &["--mode", "sweep", "build"], &["--mode", "sweep", "--", "build"]] {
            assert_eq!(contract(&strings(argv), &known, &path_of).as_deref(), Ok(want.as_str()), "{:?}", argv);
        }
        let audit = format!("You write nothing.\n--- stage contract: build ---\n{}", SHIM);
        for argv in [&["build"][..], &["build", "--mode", "audit"]] {
            assert_eq!(contract(&strings(argv), &known, &path_of).as_deref(), Ok(audit.as_str()), "{:?}", argv);
        }
    }

    #[test]
    fn an_absent_sweep_frame_refuses_sweep_alone() {
        let t = Tree::new("no-sweep");
        t.write("skills/build.md", SHIM);
        let known = strings(&["build"]);
        let path_of = bound(&t, None);
        let err = contract(&strings(&["build", "--mode", "sweep"]), &known, &path_of).expect_err("an absent sweep frame composed");
        assert!(err.contains(SWEEP), "{}", err);
        assert!(contract(&strings(&["build"]), &known, &path_of).is_ok(), "audit mode read the sweep knob");
    }

    #[test]
    fn a_malformed_mode_is_refused_with_the_usage() {
        let known = strings(&["build"]);
        let unbound = |k: &str| Err(format!("unbound knob {}", k));
        for bad in [
            &["build", "--mode"][..],
            &["--mode", "--", "build"],
            &["build", "--mode", "write"],
            &["build", "--mode", "sweep", "--mode", "sweep"],
            &["build", "--mode", "audit", "--mode", "sweep"],
        ] {
            let err = contract(&strings(bad), &known, unbound).expect_err("a malformed --mode was admitted");
            assert!(err.contains("--mode") && err.contains("usage: --emit stage-contract"), "{:?}: {}", bad, err);
        }
        let err = contract(&strings(&["build", "--help"]), &known, unbound).expect_err("--help was admitted");
        assert!(err.contains("unrecognized option: --help"), "{}", err);
    }

    // spec: lifecycle-kit/SPEC.md §The stage-contract arm — both adoption modes ride one rule: the
    // skill lands verbatim after the frame and the separator, a shim's directive included
    #[test]
    fn the_document_is_the_frame_the_separator_and_the_skill_verbatim() {
        let t = Tree::new("compose");
        let frame = t.write("frame.md", "You write nothing.\n");
        t.write("skills/align.md", SHIM);
        t.write("skills/audit.md", STANDALONE);
        assert_eq!(
            document("align", &t.skills(), &frame, AUDIT).as_deref(),
            Ok(format!("You write nothing.\n--- stage contract: align ---\n{}", SHIM).as_str())
        );
        assert_eq!(
            document("audit", &t.skills(), &frame, AUDIT).as_deref(),
            Ok(format!("You write nothing.\n--- stage contract: audit ---\n{}", STANDALONE).as_str())
        );
        let unterminated = t.write("frame.md", "You write nothing.");
        let doc = document("align", &t.skills(), &unterminated, AUDIT).expect("an unterminated frame composes");
        assert!(doc.starts_with("You write nothing.\n--- stage contract: align ---\n"), "{}", doc);
    }

    #[test]
    fn an_absent_skill_and_an_absent_or_empty_frame_are_refused() {
        let t = Tree::new("refusals");
        let frame = t.write("frame.md", "You write nothing.\n");
        let err = document("align", &t.skills(), &frame, AUDIT).expect_err("an absent skill composed");
        assert!(err.contains("LIFECYCLE_KIT_SKILLS_DIR") && err.contains("align.md"), "{}", err);
        t.write("skills/align.md", SHIM);
        let absent = t.0.join("none.md").display().to_string();
        let err = document("align", &t.skills(), &absent, AUDIT).expect_err("an absent frame composed");
        assert!(err.contains("LIFECYCLE_KIT_STAGE_CONTRACT_FRAME"), "{}", err);
        let empty = t.write("empty.md", " \n\n");
        let err = document("align", &t.skills(), &empty, AUDIT).expect_err("an empty frame composed");
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
