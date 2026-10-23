// spec: drift-kit/SPEC.md §The agent-selection probe — each bound agent, or each one named, asked
// every bound prompt from a fresh empty directory outside every work tree, one verdict line per call
// in a private trend record and each answer kept beside it.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` though its exit grammar is the `--emit-`
// collapse: it spawns a consumer's agent programs, which reach the network, and that family is
// fence-safe by derivation.
use crate::proc;
use crate::programs::Program;
use crate::walk;
use std::path::{Path, PathBuf};

pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_SELECTION_AGENTS",
    "DRIFT_KIT_SELECTION_PROMPTS",
    "DRIFT_KIT_SELECTION_SUBJECT",
    "DRIFT_KIT_SELECTION_TIMEOUT",
    "DRIFT_KIT_SELECTION_RECORD",
    "DRIFT_KIT_SELECTION_ANSWERS_DIR",
];

const NAME: &str = "--selection-probe";

const USAGE: &str =
    "usage: --selection-probe [<agent>...]   (each bound agent, or each one named, asked every bound prompt)";

pub fn run(args: &[String]) -> i32 {
    match probe(args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("checkwright-gates: {}: {}", NAME, e);
            2
        }
    }
}

struct Call {
    agent: String,
    argv: Vec<String>,
    prompt: String,
    text: String,
}

fn probe(args: &[String]) -> Result<(), String> {
    let agents = walk::knob_map("DRIFT_KIT_SELECTION_AGENTS")?;
    let prompts = walk::knob_map("DRIFT_KIT_SELECTION_PROMPTS")?;
    let subject = walk::knob_array("DRIFT_KIT_SELECTION_SUBJECT")?;
    let Some(calls) = plan(&agents, &prompts, &subject, args)? else {
        println!("selection-probe: off");
        return Ok(());
    };
    let secs: u64 = walk::knob_scalar("DRIFT_KIT_SELECTION_TIMEOUT")?
        .parse()
        .ok()
        .filter(|s| *s > 0)
        .ok_or_else(|| "DRIFT_KIT_SELECTION_TIMEOUT is not a positive integer".to_string())?;
    let mut seq = 0usize;
    let first = cold_dir(&mut seq)?;
    if let Err(e) = cold(&first) {
        let _ = std::fs::remove_dir_all(&first);
        return Err(e);
    }
    let (record, record_spelled) = super::file_survey::anchored_capture("DRIFT_KIT_SELECTION_RECORD")?;
    let (answers, answers_spelled) =
        super::file_survey::anchored_capture("DRIFT_KIT_SELECTION_ANSWERS_DIR")?;
    let today = super::kpi::today_iso();
    let mut dir = Some(first);
    let mut named = 0usize;
    for c in &calls {
        let here = match dir.take() {
            Some(d) => d,
            None => cold_dir(&mut seq)?,
        };
        let (verdict, answer) = ask(c, &subject, secs, &here);
        let _ = std::fs::remove_dir_all(&here);
        if verdict == "named" {
            named += 1;
        }
        let file = format!("{}-{}-{}.txt", today, c.agent, c.prompt);
        std::fs::create_dir_all(&answers)
            .and_then(|()| std::fs::write(walk::child(Path::new(&answers), &file), &answer))
            .map_err(|e| format!("cannot write {}/{}: {}", answers_spelled, file, e))?;
        let line = format!("{} {} {} {}", today, c.agent, c.prompt, verdict);
        let is_key = |l: &str| same_call(l, &today, &c.agent, &c.prompt);
        super::file_install::write_line(Path::new(&record), &record_spelled, &line, Some(&is_key))?;
        println!("{}", line);
    }
    println!("selection-probe: {} named of {} asked", named, calls.len());
    Ok(())
}

// spec: drift-kit/SPEC.md §The agent-selection probe — the calls to make, or `None` where no agent
// is bound; every refusal is decided here, before a directory is made or a line written
fn plan(
    agents: &[(String, String)],
    prompts: &[(String, String)],
    subject: &[String],
    names: &[String],
) -> Result<Option<Vec<Call>>, String> {
    if agents.is_empty() {
        return Ok(None);
    }
    if prompts.is_empty() {
        return Err("agents are bound and no prompt is — set DRIFT_KIT_SELECTION_PROMPTS".to_string());
    }
    if !subject.iter().any(|s| !s.is_empty()) {
        return Err("agents are bound and no subject is — set DRIFT_KIT_SELECTION_SUBJECT".to_string());
    }
    for (knob, map) in [("DRIFT_KIT_SELECTION_AGENTS", agents), ("DRIFT_KIT_SELECTION_PROMPTS", prompts)] {
        if let Some((k, _)) = map.iter().find(|(k, _)| !key_shaped(k)) {
            return Err(format!("{} key '{}' is not shaped [a-z0-9-]+", knob, k));
        }
    }
    if let Some((k, _)) = agents.iter().find(|(_, v)| v.split_ascii_whitespace().next().is_none()) {
        return Err(format!("DRIFT_KIT_SELECTION_AGENTS[{}] binds no argv", k));
    }
    if let Some(n) = names.iter().find(|n| !agents.iter().any(|(k, _)| k == *n)) {
        return Err(format!("'{}' is not a bound agent\n{}", n, USAGE));
    }
    let mut calls = Vec::new();
    for (agent, argv) in agents.iter().filter(|(k, _)| names.is_empty() || names.contains(k)) {
        for (prompt, text) in prompts {
            calls.push(Call {
                agent: agent.clone(),
                argv: argv.split_ascii_whitespace().map(String::from).collect(),
                prompt: prompt.clone(),
                text: text.clone(),
            });
        }
    }
    Ok(Some(calls))
}

fn key_shaped(k: &str) -> bool {
    !k.is_empty() && k.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

// spec: drift-kit/SPEC.md §The agent-selection probe — a fresh empty directory under the host's
// temporary directory, made exclusively so no earlier run's leftover is reused
fn cold_dir(seq: &mut usize) -> Result<PathBuf, String> {
    let d = proc::temp_base().join(format!("checkwright-selection-probe.{}.{}", std::process::id(), seq));
    *seq += 1;
    std::fs::create_dir(&d).map_err(|e| format!("cannot create {}: {}", d.display(), e))?;
    Ok(d)
}

// spec: drift-kit/SPEC.md §The agent-selection probe — the coldness test, through the crosser
// (gate-sdk/SPEC.md §The crate's crosser): a work tree above the directory, or a repository git
// refuses there, is a refusal, since an agent run there would read it
fn cold(dir: &Path) -> Result<(), String> {
    let at = walk::cross_arg(&dir.display().to_string());
    match walk::toplevel_in_opt(&at)? {
        None => Ok(()),
        Some(top) => Err(format!(
            "the temporary directory {} lies inside the work tree {}, where an agent would read a \
             repository — point the host's temporary directory outside every work tree",
            at, top
        )),
    }
}

// spec: drift-kit/SPEC.md §The agent-selection probe — one call: argv with the prompt appended, no
// shell, stdin empty, the environment inherited, bounded with its process tree killed
fn ask(c: &Call, subject: &[String], secs: u64, dir: &Path) -> (&'static str, Vec<u8>) {
    let program = Program::consumer("DRIFT_KIT_SELECTION_AGENTS", c.argv[0].as_str());
    let mut rest: Vec<&str> = c.argv[1..].iter().map(String::as_str).collect();
    rest.push(c.text.as_str());
    match proc::run_bounded_capture(&program, &rest, secs, &[], Some(dir)) {
        Err(_) => ("failed", Vec::new()),
        Ok(None) => ("timed-out", Vec::new()),
        Ok(Some((0, bytes))) => {
            let verdict = if names_subject(&String::from_utf8_lossy(&bytes), subject) {
                "named"
            } else {
                "unnamed"
            };
            (verdict, bytes)
        }
        Ok(Some((_, bytes))) => ("failed", bytes),
    }
}

// spec: drift-kit/SPEC.md §The agent-selection probe — a subject name occurring case-insensitively
// with no word character, an ASCII letter, digit or `_`, on either side of it
fn names_subject(answer: &str, subject: &[String]) -> bool {
    let hay = answer.as_bytes();
    let word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    subject.iter().map(String::as_bytes).filter(|n| !n.is_empty()).any(|n| {
        hay.len() >= n.len()
            && (0..=hay.len() - n.len()).any(|i| {
                hay[i..i + n.len()].eq_ignore_ascii_case(n)
                    && (i == 0 || !word(hay[i - 1]))
                    && (i + n.len() == hay.len() || !word(hay[i + n.len()]))
            })
    })
}

// spec: drift-kit/SPEC.md §The agent-selection probe — a line replaces on `<date> <agent> <prompt>`
fn same_call(line: &str, date: &str, agent: &str, prompt: &str) -> bool {
    let f: Vec<&str> = line.split_whitespace().collect();
    f.len() >= 3 && f[0] == date && f[1] == agent && f[2] == prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pairs(p: &[(&str, &str)]) -> Vec<(String, String)> {
        p.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    fn strings(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    // spec: drift-kit/SPEC.md §The agent-selection probe — case-insensitive, at word boundaries, and
    // never inside a longer word
    #[test]
    fn a_subject_name_matches_whole_words_in_any_case() {
        let s = strings(&["checkwright"]);
        assert!(names_subject("Try Checkwright.", &s));
        assert!(names_subject("CHECKWRIGHT", &s));
        assert!(names_subject("checkwright-gates and more", &s), "a hyphen is no word character");
        assert!(names_subject("(checkwright)", &s));
        assert!(!names_subject("checkwrights", &s), "a name inside a longer word");
        assert!(!names_subject("precheckwright", &s));
        assert!(!names_subject("check_wright checkwright_x", &s), "`_` is a word character");
        assert!(!names_subject("", &s));
        assert!(names_subject("uses gatekeeper", &strings(&["", "gatekeeper"])), "an empty name is skipped");
        assert!(!names_subject("anything", &strings(&[""])));
    }

    // spec: drift-kit/SPEC.md §The agent-selection probe — keys are shaped `[a-z0-9-]+`
    #[test]
    fn a_key_is_lowercase_digits_and_hyphens() {
        for ok in ["claude", "agy", "a-1", "0"] {
            assert!(key_shaped(ok), "{}", ok);
        }
        for bad in ["", "Claude", "a_b", "a b", "a.b", "ä"] {
            assert!(!key_shaped(bad), "{}", bad);
        }
    }

    // spec: drift-kit/SPEC.md §The agent-selection probe — off with nothing bound, then each refusal
    // class, decided before any call
    #[test]
    fn each_refusal_class_refuses_and_nothing_bound_is_off() {
        let agents = pairs(&[("a", "bash x"), ("b", "bash y")]);
        let prompts = pairs(&[("p", "Which tools?")]);
        let subject = strings(&["sub"]);
        assert!(plan(&[], &prompts, &subject, &[]).unwrap().is_none());
        assert!(plan(&[], &[], &[], &strings(&["a"])).unwrap().is_none(), "off whatever it names");
        let err = |a: &[(String, String)], p: &[(String, String)], s: &[String], n: &[String]| {
            plan(a, p, s, n).err().expect("a refusal")
        };
        assert!(err(&agents, &[], &subject, &[]).contains("no prompt"));
        assert!(err(&agents, &prompts, &[], &[]).contains("no subject"));
        assert!(err(&agents, &prompts, &strings(&[""]), &[]).contains("no subject"));
        assert!(err(&pairs(&[("A", "x")]), &prompts, &subject, &[]).contains("key 'A'"));
        assert!(err(&agents, &pairs(&[("p q", "x")]), &subject, &[]).contains("PROMPTS key"));
        assert!(err(&pairs(&[("a", " ")]), &prompts, &subject, &[]).contains("binds no argv"));
        assert!(err(&agents, &prompts, &subject, &strings(&["c"])).contains("'c' is not a bound agent"));
        assert!(err(&agents, &prompts, &subject, &strings(&["--help"])).contains("usage:"));
    }

    // spec: drift-kit/SPEC.md §The agent-selection probe — every bound prompt to each bound agent, or
    // to each one named, the argv split on blanks
    #[test]
    fn the_plan_asks_each_selected_agent_every_prompt() {
        let agents = pairs(&[("a", "bash  x.sh"), ("b", "y")]);
        let prompts = pairs(&[("p", "one"), ("q", "two")]);
        let subject = strings(&["s"]);
        let all = plan(&agents, &prompts, &subject, &[]).unwrap().unwrap();
        let keys: Vec<String> = all.iter().map(|c| format!("{}/{}", c.agent, c.prompt)).collect();
        assert_eq!(keys, ["a/p", "a/q", "b/p", "b/q"]);
        assert_eq!(all[0].argv, ["bash", "x.sh"]);
        assert_eq!(all[1].text, "two");
        let one = plan(&agents, &prompts, &subject, &strings(&["b"])).unwrap().unwrap();
        assert_eq!(one.iter().map(|c| c.agent.as_str()).collect::<Vec<_>>(), ["b", "b"]);
    }

    // spec: drift-kit/SPEC.md §The agent-selection probe — the record key is the first three fields
    #[test]
    fn a_line_replaces_on_its_date_agent_and_prompt() {
        assert!(same_call("2026-01-01 a p named", "2026-01-01", "a", "p"));
        assert!(!same_call("2026-01-02 a p named", "2026-01-01", "a", "p"));
        assert!(!same_call("2026-01-01 a q named", "2026-01-01", "a", "p"));
        let body = "2026-01-01 a p unnamed\n2026-01-01 a q named\n";
        let out = super::super::file_install::replace_in_place(
            body,
            &|l| same_call(l, "2026-01-01", "a", "p"),
            "2026-01-01 a p named",
        );
        assert_eq!(out.as_deref(), Some("2026-01-01 a p named\n2026-01-01 a q named\n"));
    }

    // spec: drift-kit/SPEC.md §The agent-selection probe — a directory inside a work tree refuses
    #[test]
    fn a_directory_inside_a_work_tree_is_not_cold() {
        let _env = crate::knobenv::lock();
        let base = std::env::temp_dir().join(format!("cw-selection-warm-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let sub = base.join("sub");
        std::fs::create_dir_all(&sub).expect("scratch");
        let init = proc::run(&crate::programs::GIT, &["init", "-q", &base.display().to_string()]);
        let verdict = cold(&sub);
        let _ = std::fs::remove_dir_all(&base);
        assert!(init.is_ok_and(|c| c.stdout().is_some()), "git init failed");
        assert!(verdict.is_err(), "a directory under a repository read as cold");
    }
}
