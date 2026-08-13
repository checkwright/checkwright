// spec: lifecycle-kit/SPEC.md §check-stage-skill-coverage — the configured stage set and the
// skills dir cover each other: every stage has a skill, every enter-stage-invoking skill
// names a live stage
use crate::stages;
use crate::walk;
use std::path::Path;

// spec: lifecycle-kit/SPEC.md §check-stage-skill-coverage — the shell form's
// `enter-stage\.sh[[:space:]]+[a-z][a-z-]*`: a gate-owned literal shape, so it is matched
// directly rather than compiled, and the invoked stage is the token after the run of spaces
fn invoked_stages(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in text.lines() {
        let b = line.as_bytes();
        let mut from = 0usize;
        while let Some(rel) = line[from..].find("enter-stage.sh") {
            let at = from + rel;
            let mut i = at + "enter-stage.sh".len();
            let start_ws = i;
            while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
                i += 1;
            }
            if i > start_ws && i < b.len() && b[i].is_ascii_lowercase() {
                let s = i;
                while i < b.len() && (b[i].is_ascii_lowercase() || b[i] == b'-') {
                    i += 1;
                }
                out.push(line[s..i].to_string());
            }
            from = at + "enter-stage.sh".len();
        }
    }
    out.sort();
    out.dedup();
    out
}

pub fn run(args: &[String]) -> i32 {
    let stage_set = match stages::stages() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-stage-skill-coverage: {}", e);
            return 2;
        }
    };
    let dir = match args.first().filter(|a| !a.is_empty()) {
        Some(d) => d.clone(),
        None => match walk::knob_scalar("LIFECYCLE_KIT_SKILLS_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-stage-skill-coverage: {}", e);
                return 2;
            }
        },
    };
    if !Path::new(&dir).is_dir() {
        eprintln!("check-stage-skill-coverage: skills dir not found: {}", dir);
        return 2;
    }

    let mut missing: Vec<String> = Vec::new();
    for s in &stage_set {
        if !Path::new(&format!("{}/{}.md", dir, s)).is_file() {
            missing.push(format!("{} (expected {}/{}.md)", s, dir, s));
        }
    }

    let files = match walk::glob_files(Path::new(&dir), &["*.md".to_string()]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-stage-skill-coverage: {}", e);
            return 2;
        }
    };
    let mut orphan: Vec<String> = Vec::new();
    for f in &files {
        let text = match std::fs::read(f) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => {
                eprintln!(
                    "check-stage-skill-coverage: cannot read {}: {}",
                    f.display(),
                    e
                );
                return 2;
            }
        };
        let base = f
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        for n in invoked_stages(&text) {
            if !stages::stage_known(&stage_set, &n) {
                orphan.push(format!(
                    "{} invokes enter-stage.sh '{}', not a lifecycle stage",
                    base, n
                ));
            }
        }
    }

    if !missing.is_empty() || !orphan.is_empty() {
        println!(
            "check-stage-skill-coverage: stage set ({}) and skills dir {}",
            stage_set.join(" "),
            dir
        );
        println!("are out of sync — a stage with no skill cannot be entered; an orphan stage skill");
        println!("is a retired stage's dead entry point:");
        for m in &missing {
            println!("  no skill for stage: {}", m);
        }
        for o in &orphan {
            println!("  orphan skill:       {}", o);
        }
        println!("  help: add the missing <stage>.md skill, or retire the orphan skill / fix the");
        println!("        stage name it invokes. The stage set is LIFECYCLE_KIT_STAGES (lifecycle-config.sh).");
        return 1;
    }

    println!(
        "STAGE-SKILL-COVERAGE: clean ({} stage(s) each have a skill; every enter-stage-invoking skill in {} names a live stage)",
        stage_set.len(),
        dir
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §check-stage-skill-coverage — the extractor is `grep -oE`'s
    // sorted-unique output, and the shapes that must not yield a stage are the ones a
    // substring search would wrongly take
    #[test]
    fn only_a_lowercase_token_after_a_run_of_spaces_is_an_invoked_stage() {
        assert_eq!(
            invoked_stages("run enter-stage.sh build now\nenter-stage.sh  align\n"),
            vec!["align".to_string(), "build".to_string()]
        );
        assert!(invoked_stages("enter-stage.sh").is_empty());
        assert!(invoked_stages("enter-stage.sh 9lives").is_empty());
        assert!(invoked_stages("enter-stage.shbuild").is_empty());
        assert_eq!(
            invoked_stages("enter-stage.sh build\nenter-stage.sh build\n"),
            vec!["build".to_string()]
        );
    }
}
