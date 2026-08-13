// spec: lifecycle-kit/SPEC.md §check-lesson-disposition — every Lessons entry present at HEAD
// and gone from the worktree leaves a well-formed disposition stamp in the evidence file
use crate::proc;
use crate::walk;
use std::path::Path;

const SEP: &str = " — ";

// spec: lifecycle-kit/SPEC.md §check-lesson-disposition — one normalized Lessons lead line per
// top-level entry: the bullet's own text, lead and trailing space stripped
fn lessons_of(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inl = false;
    for line in text.lines() {
        if line.starts_with("## Lessons Learned")
            && line["## Lessons Learned".len()..]
                .chars()
                .all(|c| c == ' ' || c == '\t')
        {
            inl = true;
            continue;
        }
        if line.starts_with("## ") {
            inl = false;
        }
        if inl && (line.starts_with("- ") || line.starts_with("-\t")) {
            let t = line.trim_start_matches([' ', '\t']);
            let t = t.strip_prefix('-').unwrap_or(t).trim_start_matches([' ', '\t']);
            out.push(t.trim_end_matches([' ', '\t']).to_string());
        }
    }
    out
}

// spec: lifecycle-kit/SPEC.md §check-lesson-disposition — the disposition grammar's left half,
// `<iteration> lesson <rule|task|harvest|discard> <ref>`, checked before the ' — ' separator
fn dispositions(field1: &str) -> bool {
    let Some(sp) = field1.find(' ') else {
        return false;
    };
    if sp == 0 {
        return false;
    }
    let Some(rest) = field1[sp + 1..].strip_prefix("lesson ") else {
        return false;
    };
    ["rule", "task", "harvest", "discard"]
        .iter()
        .any(|k| matches!(rest.strip_prefix(k), Some(r) if r.starts_with(' ')))
}

pub fn run(args: &[String]) -> i32 {
    let hermetic = args.first().filter(|a| !a.is_empty()).is_some();
    let (head_text, work_text, evid_file) = if hermetic {
        let head = args[0].clone();
        let work = args.get(1).cloned().unwrap_or_default();
        let evid = args.get(2).cloned().unwrap_or_default();
        for (label, p) in [
            ("queue-head", &head),
            ("queue-worktree", &work),
            ("evidence file", &evid),
        ] {
            if p.is_empty() || !Path::new(p).is_file() {
                eprintln!("check-lesson-disposition: {} not found: {}", label, p);
                return 2;
            }
        }
        match (read(&head), read(&work)) {
            (Ok(h), Ok(w)) => (h, w, evid),
            (Err(e), _) | (_, Err(e)) => {
                eprintln!("check-lesson-disposition: {}", e);
                return 2;
            }
        }
    } else {
        let queue = match walk::knob_scalar("LIFECYCLE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-lesson-disposition: {}", e);
                return 2;
            }
        };
        let in_repo = proc::run("git", &["rev-parse", "--git-dir"])
            .map(|c| c.stdout().is_some())
            .unwrap_or(false);
        if !in_repo {
            println!("LESSON-DISPOSITION: clean (no git repository — no HEAD baseline to compare)");
            return 0;
        }
        let spec = format!("HEAD:{}", queue);
        let shown = match proc::run("git", &["show", &spec]) {
            Ok(c) => c.stdout().map(|b| String::from_utf8_lossy(b).into_owned()),
            Err(e) => {
                eprintln!("check-lesson-disposition: {}", e);
                return 2;
            }
        };
        let Some(head) = shown else {
            println!(
                "LESSON-DISPOSITION: clean ({} not at HEAD — no prior lessons to disposition)",
                queue
            );
            return 0;
        };
        if !Path::new(&queue).is_file() {
            eprintln!(
                "check-lesson-disposition: worktree queue not found: {}",
                queue
            );
            return 2;
        }
        let evid = match walk::knob_scalar("LIFECYCLE_KIT_LESSON_EVIDENCE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-lesson-disposition: {}", e);
                return 2;
            }
        };
        if !Path::new(&evid).is_file() {
            eprintln!("check-lesson-disposition: evidence file not found: {}", evid);
            return 2;
        }
        match read(&queue) {
            Ok(w) => (head, w, evid),
            Err(e) => {
                eprintln!("check-lesson-disposition: {}", e);
                return 2;
            }
        }
    };

    let evid_text = match read(&evid_file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("check-lesson-disposition: {}", e);
            return 2;
        }
    };

    let mut malformed: Vec<String> = Vec::new();
    let mut prefixes: Vec<String> = Vec::new();
    for line in evid_text.lines() {
        let t = line.trim_start_matches([' ', '\t']);
        if t.starts_with('#') || t.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(SEP).collect();
        if parts.len() < 2 || !dispositions(parts[0]) {
            malformed.push(line.to_string());
            continue;
        }
        prefixes.push(parts[1..].join(SEP));
    }

    if !malformed.is_empty() {
        println!(
            "check-lesson-disposition: malformed disposition line(s) in {}",
            evid_file
        );
        println!("(grammar: <iteration> lesson <rule <file>|task <slug>|harvest <tag>|discard <reason>> — <lead-line prefix>):");
        for m in &malformed {
            println!("  {}", m);
        }
        println!("  help: rewrite each line to the grammar above; the ' — ' separates the disposition from the lead-line prefix it dispositions.");
        return 1;
    }

    let in_work = lessons_of(&work_text);
    let mut undispositioned: Vec<String> = Vec::new();
    let mut matched = 0usize;
    for entry in lessons_of(&head_text) {
        if entry.is_empty() || in_work.contains(&entry) {
            continue;
        }
        if prefixes
            .iter()
            .any(|p| !p.is_empty() && entry.starts_with(p.as_str()))
        {
            matched += 1;
        } else {
            undispositioned.push(entry);
        }
    }

    if !undispositioned.is_empty() {
        println!("check-lesson-disposition: Lessons entr(y|ies) removed since HEAD with no disposition");
        println!(
            "stamp in {} (a lesson cleared without a rule/task/harvest/discard record):",
            evid_file
        );
        for u in &undispositioned {
            println!("  {}", u);
        }
        println!("  help: stamp each removed lesson in {} — '<iteration> lesson <kind> <ref> — <lead-line prefix>' — or restore the entry.", evid_file);
        return 1;
    }

    println!(
        "LESSON-DISPOSITION: clean ({} removed lesson(s) each matched a disposition stamp; grammar holds in {})",
        matched, evid_file
    );
    0
}

fn read(p: &str) -> Result<String, String> {
    std::fs::read(p)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", p, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_bullets_under_the_lessons_heading_are_lessons() {
        let q = "## New Features\n- not a lesson\n## Lessons Learned\n- **a** — one  \n  - nested\n- **b** — two\n## Done\n- after\n";
        assert_eq!(
            lessons_of(q),
            vec!["**a** — one".to_string(), "**b** — two".to_string()]
        );
    }

    #[test]
    fn the_disposition_grammar_needs_an_iteration_the_word_lesson_a_kind_and_a_ref() {
        assert!(dispositions("iter-name lesson task some-slug"));
        assert!(dispositions("iter lesson rule some/file.md"));
        assert!(!dispositions("iter lesson bogus ref"));
        assert!(!dispositions("iter lesson task"));
        assert!(!dispositions("lesson task ref"));
    }
}
