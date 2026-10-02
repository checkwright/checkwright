// spec: delegation-kit/SPEC.md §model-verdict — the model a session runs on, read off its transcript,
// and with `--expect <class>` judged against the tier binding: 0 READ/OK, 1 BELOW/UNBOUND, 2 UNKNOWN.
use crate::emit::stage_economics::{self, Tokens};
use crate::sessions::{self, Delegation, Inputs};
use crate::tier;
use crate::walk;

pub const KNOBS: &[&str] = &["DELEGATION_KIT_TIER_MODEL", "DELEGATION_KIT_SESSIONS_DIR"];

const USAGE: &str = "usage: --model-verdict [--expect <class>] [--] [<transcript.jsonl | session8>]\n  --expect names the floor class (judgment, routing or mechanical); the operand names the transcript by path or by an eight-character session id, and \"--\" takes one beginning with \"-\"";

const OFF_TIER: &str =
    "the session is not on its expected tier; stop before work that needs it and have it re-dispatched at that tier";

const UNVERIFIED: &str = "tier unverified, not refused; the caller's rule says whether to proceed";

struct Args {
    expect: Option<String>,
    operand: Option<String>,
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — a dash-led token naming no option is a refusal,
// and `--` ends option processing
fn parse(args: &[String]) -> Result<Args, String> {
    let mut out = Args { expect: None, operand: None };
    let mut rest: Vec<&str> = Vec::new();
    let mut i = 0;
    let mut literal = false;
    while i < args.len() {
        let a = args[i].as_str();
        if literal {
            rest.push(a);
        } else if a == "--" {
            literal = true;
        } else if a == "--expect" {
            let class = args.get(i + 1).ok_or("model-verdict: --expect needs a class")?;
            if tier::rank(class).is_none() {
                return Err(format!(
                    "model-verdict: --expect names '{}', not one of {}",
                    class,
                    tier::CLASSES.join(", ")
                ));
            }
            out.expect = Some(class.clone());
            i += 1;
        } else if a.starts_with('-') {
            return Err(format!(
                "model-verdict: unrecognized option: {} — an operand beginning with \"-\" is passed after a \"--\" separator",
                a
            ));
        } else {
            rest.push(a);
        }
        i += 1;
    }
    if rest.len() > 1 {
        return Err(format!("model-verdict: one operand at most (got {})", rest.len()));
    }
    out.operand = rest.first().map(|s| s.to_string());
    Ok(out)
}

fn inputs() -> Result<Inputs, String> {
    let var = |n: &str| std::env::var(n).unwrap_or_default();
    let pwd = var("PWD");
    let here = if pwd.is_empty() { walk::cwd()? } else { pwd };
    Ok(Inputs {
        session_id: String::new(),
        harness_id: var("CLAUDE_CODE_SESSION_ID"),
        child: var("CLAUDE_CODE_CHILD_SESSION"),
        sessions_dir: walk::knob_scalar("DELEGATION_KIT_SESSIONS_DIR")?,
        config_home: var("CLAUDE_CONFIG_DIR"),
        home: var(crate::sessions::HOME_VAR),
        here,
    })
}

// spec: delegation-kit/SPEC.md §model-verdict — which transcript: a `.jsonl` path as given, an
// eight-character key through the shared inverse lookup, else the delegation-aware pick
fn transcript(operand: Option<&str>, i: &Inputs) -> Option<String> {
    if let Some(op) = operand {
        if std::path::Path::new(op).is_file() {
            return Some(op.to_string());
        }
        return Some(op).filter(|k| k.chars().count() == 8).and_then(|k| sessions::find(i, k));
    }
    let path = match sessions::delegation(i) {
        Delegation::TopLevel(id) => sessions::top_level(&sessions::sessions_dir(i), &id),
        Delegation::Delegated(path) => path,
        Delegation::Undetermined if i.harness_id.is_empty() => sessions::newest(i)?,
        Delegation::Undetermined => return None,
    };
    Some(path).filter(|p| std::path::Path::new(p).is_file())
}

// spec: delegation-kit/SPEC.md §model-verdict — the newest assistant record whose usage is not all
// zero, since the harness's synthetic records carry zero usage
fn running_model(body: &str) -> Option<String> {
    body.lines()
        .rev()
        .filter_map(stage_economics::record)
        .find(|r| r.tokens != Tokens::default())
        .map(|r| r.model)
}

pub fn verdict(args: &[String]) -> (String, i32) {
    let a = match parse(args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("{}", USAGE);
            return (String::new(), 2);
        }
    };
    let expect = a.expect.as_deref().unwrap_or("-");
    let unknown = |id: &str, session: &str, why: &str| {
        (
            format!(
                "model-verdict: id={} session={} class=none expect={} -> UNKNOWN ({}) — {}",
                id, session, expect, why, UNVERIFIED
            ),
            2,
        )
    };
    let binding = match walk::knob_array("DELEGATION_KIT_TIER_MODEL") {
        Ok(b) => b,
        Err(e) => return unknown("-", "-", &format!("the tier binding did not resolve: {}", e)),
    };
    let i = match inputs() {
        Ok(i) => i,
        Err(e) => return unknown("-", "-", &e),
    };
    let Some(path) = transcript(a.operand.as_deref(), &i) else {
        return unknown("-", a.operand.as_deref().unwrap_or("-"), "no transcript");
    };
    let session = sessions::key(&path);
    let body = match std::fs::read(&path) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => return unknown("-", &session, &format!("cannot read {}: {}", path, e)),
    };
    let Some(id) = running_model(&body) else {
        return unknown("-", &session, "no assistant record with non-zero usage");
    };
    judge(&id, &session, &binding, a.expect.as_deref())
}

// spec: delegation-kit/SPEC.md §model-verdict — the verdict table over a read id
fn judge(id: &str, session: &str, binding: &[String], expect: Option<&str>) -> (String, i32) {
    let classes = tier::classes_of(binding, id);
    let shown = if classes.is_empty() { "none".to_string() } else { classes.join(",") };
    let line = |verdict: &str, tail: &str| {
        format!(
            "model-verdict: id={} session={} class={} expect={} -> {}{}",
            id,
            session,
            shown,
            expect.unwrap_or("-"),
            verdict,
            tail
        )
    };
    let Some(want) = expect else {
        return (line("READ", ""), 0);
    };
    if binding.is_empty() {
        return (line("UNKNOWN", &format!(" (the tier binding is empty) — {}", UNVERIFIED)), 2);
    }
    if tier::bound(binding, want).is_none() {
        return (line("UNKNOWN", &format!(" (class '{}' is unbound) — {}", want, UNVERIFIED)), 2);
    }
    let floor = tier::rank(want).unwrap_or(0);
    if classes.iter().any(|c| tier::rank(c).is_some_and(|r| r <= floor)) {
        return (line("OK", ""), 0);
    }
    if classes.is_empty() {
        return (line("UNBOUND", &format!(" — {}", OFF_TIER)), 1);
    }
    (line("BELOW", &format!(" — {}", OFF_TIER)), 1)
}

pub fn run(args: &[String]) -> i32 {
    let (line, code) = verdict(args);
    if !line.is_empty() {
        println!("{}", line);
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knobenv;
    use std::path::PathBuf;

    const KIT_PREFIX: &str = "DELEGATION_KIT_";

    // spec: delegation-kit/SPEC.md §Testing — the budget verdict's hermetic contract: the whole
    // `DELEGATION_KIT_*` namespace held aside, the sandbox the gates directory, and the case's binding
    // written into the sandbox's knob file
    struct Sandbox {
        dir: PathBuf,
        held: Vec<(String, String)>,
        gates_dir: Option<String>,
        knobs: knobenv::KnobEnv,
    }

    impl Sandbox {
        fn new(tag: &str, binding: &[&str]) -> Sandbox {
            let knobs = knobenv::lock();
            let dir = std::env::temp_dir().join(format!("checkwright-model-verdict.{}.{}", tag, std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(dir.join("sessions")).expect("the sandbox must be creatable");
            let held: Vec<(String, String)> =
                std::env::vars().filter(|(n, _)| n.starts_with(KIT_PREFIX)).collect();
            for (n, _) in &held {
                knobs.remove(n);
            }
            let gates_dir = std::env::var("GATE_SDK_GATES_DIR").ok();
            knobs.set("GATE_SDK_GATES_DIR", &dir.display().to_string());
            let mut file = format!("DELEGATION_KIT_SESSIONS_DIR = {}/sessions\n", dir.display());
            for e in binding {
                file.push_str(&format!("DELEGATION_KIT_TIER_MODEL[] = {}\n", e));
            }
            std::fs::write(dir.join("delegation-config.knobs"), file).expect("the knob file must be writable");
            crate::knobs::reset(&knobs);
            Sandbox { dir, held, gates_dir, knobs }
        }

        fn transcript(&self, rel: &str, records: &[(&str, &str, u64)]) -> String {
            let path = self.dir.join(rel);
            std::fs::create_dir_all(path.parent().expect("a parent")).expect("the transcript dir");
            let mut body = String::from("{\"type\":\"user\",\"message\":{\"role\":\"user\"}}\n");
            for (id, model, out) in records {
                body.push_str(&format!(
                    "{{\"type\":\"assistant\",\"message\":{{\"id\":\"{}\",\"model\":\"{}\",\"usage\":{{\"input_tokens\":{},\"output_tokens\":{}}}}}}}\n",
                    id, model, out, out
                ));
            }
            std::fs::write(&path, body).expect("the transcript must be writable");
            path.display().to_string()
        }

        fn run(&self, args: &[&str]) -> (String, i32) {
            verdict(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            match &self.gates_dir {
                Some(g) => self.knobs.set("GATE_SDK_GATES_DIR", g),
                None => self.knobs.remove("GATE_SDK_GATES_DIR"),
            }
            for (n, v) in &self.held {
                self.knobs.set(n, v);
            }
            crate::knobs::reset(&self.knobs);
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    fn bound() -> [&'static str; 2] {
        ["judgment=big", "mechanical=vendor-small-4"]
    }

    // spec: delegation-kit/SPEC.md §model-verdict — READ, a bare read that judges nothing
    #[test]
    fn a_bare_read_names_the_model_and_exits_zero() {
        let s = Sandbox::new("read", &bound());
        let t = s.transcript("t.jsonl", &[("m1", "vendor-tiny-1", 5)]);
        let (line, code) = s.run(&[&t]);
        assert_eq!(code, 0, "{}", line);
        assert!(line.ends_with("id=vendor-tiny-1 session=t class=none expect=- -> READ"), "{}", line);
    }

    // spec: delegation-kit/SPEC.md §model-verdict — OK by exact id, by alias token, and by a stronger
    // class under a floor
    #[test]
    fn a_model_meeting_its_floor_is_ok() {
        let s = Sandbox::new("ok", &bound());
        let exact = s.transcript("exact.jsonl", &[("m1", "vendor-small-4", 5)]);
        assert_eq!(s.run(&["--expect", "mechanical", &exact]).1, 0);
        let alias = s.transcript("alias.jsonl", &[("m1", "vendor-big-5", 5)]);
        let (line, code) = s.run(&["--expect", "judgment", &alias]);
        assert_eq!(code, 0, "{}", line);
        assert!(line.contains("class=judgment expect=judgment -> OK"), "{}", line);
        assert_eq!(s.run(&["--expect", "mechanical", &alias]).1, 0, "a stronger class passes a floor");
    }

    // spec: delegation-kit/SPEC.md §model-verdict — BELOW, a match only under the floor
    #[test]
    fn a_model_under_its_floor_is_below() {
        let s = Sandbox::new("below", &bound());
        let t = s.transcript("t.jsonl", &[("m1", "vendor-small-4", 5)]);
        let (line, code) = s.run(&["--expect", "judgment", &t]);
        assert_eq!(code, 1, "{}", line);
        assert!(line.contains("-> BELOW — the session is not on its expected tier"), "{}", line);
    }

    // spec: delegation-kit/SPEC.md §model-verdict — UNBOUND, a model the binding does not name
    #[test]
    fn a_model_outside_the_binding_is_unbound() {
        let s = Sandbox::new("unbound", &bound());
        let t = s.transcript("t.jsonl", &[("m1", "vendor-tiny-1", 5)]);
        let (line, code) = s.run(&["--expect", "mechanical", &t]);
        assert_eq!(code, 1, "{}", line);
        assert!(line.contains("class=none expect=mechanical -> UNBOUND"), "{}", line);
    }

    // spec: delegation-kit/SPEC.md §model-verdict — UNKNOWN's four causes, each exit 2
    #[test]
    fn what_cannot_be_read_is_unknown() {
        let s = Sandbox::new("unknown", &["judgment=big"]);
        let empty = s.transcript("empty.jsonl", &[]);
        let (line, code) = s.run(&["--expect", "judgment", &empty]);
        assert_eq!(code, 2, "{}", line);
        assert!(line.contains("UNKNOWN (no assistant record with non-zero usage) — tier unverified"), "{}", line);
        let synthetic = s.transcript("synthetic.jsonl", &[("m1", "vendor-big-5", 0)]);
        assert_eq!(s.run(&["--expect", "judgment", &synthetic]).1, 2);
        let real = s.transcript("real.jsonl", &[("m1", "vendor-big-5", 5)]);
        let (line, code) = s.run(&["--expect", "mechanical", &real]);
        assert_eq!(code, 2, "{}", line);
        assert!(line.contains("class 'mechanical' is unbound"), "{}", line);
        drop(s);
        let off = Sandbox::new("off", &[]);
        let real = off.transcript("real.jsonl", &[("m1", "vendor-big-5", 5)]);
        let (line, code) = off.run(&["--expect", "judgment", &real]);
        assert_eq!(code, 2, "{}", line);
        assert!(line.contains("the tier binding is empty"), "{}", line);
        assert_eq!(off.run(&[&real]).1, 0, "a bare read needs no binding");
    }

    // spec: delegation-kit/SPEC.md §model-verdict — the operand forms: a path, and an eight-character
    // key naming an `agent-` transcript through the inverse lookup
    #[test]
    fn a_session_key_finds_an_agent_transcript() {
        let s = Sandbox::new("key", &bound());
        s.transcript("sessions/lead/subagents/agent-abcdef1234567.jsonl", &[("m1", "vendor-big-5", 5)]);
        let (line, code) = s.run(&["--expect", "judgment", "abcdef12"]);
        assert_eq!(code, 0, "{}", line);
        assert!(line.contains("session=abcdef12"), "{}", line);
        let (line, code) = s.run(&["zzzzzzzz"]);
        assert_eq!(code, 2, "{}", line);
        assert!(line.contains("no transcript"), "{}", line);
    }

    // spec: delegation-kit/SPEC.md §model-verdict — a model switched mid-session reads as the one now
    // running
    #[test]
    fn the_newest_record_wins() {
        let s = Sandbox::new("newest", &bound());
        let t = s.transcript("t.jsonl", &[("m1", "vendor-small-4", 5), ("m2", "vendor-big-5", 5), ("m3", "vendor-small-4", 0)]);
        let (line, _) = s.run(&[&t]);
        assert!(line.contains("id=vendor-big-5"), "{}", line);
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the shape refusal and the `--` escape
    #[test]
    fn a_dash_led_operand_refuses_and_the_escape_takes_it() {
        let flag = vec!["--help".to_string()];
        assert!(parse(&flag).is_err());
        assert!(parse(&["--expect".to_string()]).is_err());
        assert!(parse(&["--expect".to_string(), "premium".to_string()]).is_err());
        let escaped = parse(&["--".to_string(), "-t.jsonl".to_string()]).expect("the escape");
        assert_eq!(escaped.operand.as_deref(), Some("-t.jsonl"));
        assert!(parse(&["a.jsonl".to_string(), "b.jsonl".to_string()]).is_err(), "one operand at most");
    }
}
