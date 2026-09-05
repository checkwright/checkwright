// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the canonical stamp id by the fixed source
// order, first hit wins, every source ending in the same normalization. That section owns the
// contract; `crate::sessions` holds the one implementation the drift meters read too.
// spec: gate-sdk/SPEC.md §The non-gate arm — the roster is empty and must stay empty: neither
// name this arm reads is defined in lifecycle-kit's `lib/stages.sh`, so a declared row would
// fail-close through the config bridge's undeclared-knob refusal on every invocation.
use crate::sessions::{key, resolve, Inputs};

pub const KNOBS: &[&str] = &[];

fn var(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the derivation order and its exit-2 refusals are
// `resolve`'s; what this arm owes on top of them is the normalization every source ends in, which
// `key` is over a resolved path and the identity over a bare id.
pub fn derive(i: &Inputs) -> Result<String, String> {
    Ok(key(&resolve(i)?))
}

// spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the process cwd is an *input* to source 3's
// default. bash's `pwd` prints the logical path it carries in `PWD` where `current_dir()` returns
// the physical one, so `PWD` is read where set and the crate's crosser answers otherwise.
pub fn emit(_args: &[String]) -> Result<String, String> {
    let pwd = var("PWD");
    let here = if pwd.is_empty() {
        crate::walk::cwd()?
    } else {
        pwd
    };
    let inputs = Inputs {
        session_id: var("LIFECYCLE_KIT_SESSION_ID"),
        harness_id: var("CLAUDE_CODE_SESSION_ID"),
        child: var("CLAUDE_CODE_CHILD_SESSION"),
        sessions_dir: var("LIFECYCLE_KIT_SESSIONS_DIR"),
        config_home: var("CLAUDE_CONFIG_DIR"),
        home: var("HOME"),
        here,
    };
    Ok(format!("{}\n", derive(&inputs)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::{normalize, sessions_dir};
    use std::path::Path;

    fn inputs(dir: &str) -> Inputs {
        Inputs {
            session_id: String::new(),
            harness_id: String::new(),
            child: String::new(),
            sessions_dir: dir.to_string(),
            config_home: String::new(),
            home: String::new(),
            here: String::new(),
        }
    }

    fn scratch(tag: &str) -> String {
        let dir = std::env::temp_dir().join(format!(
            "checkwright-session-id.{}.{}",
            tag,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("the scratch sessions dir must be creatable");
        dir.display().to_string()
    }

    fn transcript(path: &str, epoch: u64) {
        let p = Path::new(path);
        std::fs::create_dir_all(p.parent().expect("a transcript has a parent"))
            .expect("the transcript's directory must be creatable");
        std::fs::write(p, "").expect("the transcript must be writable");
        let status = std::process::Command::new("touch")
            .arg("-d")
            .arg(format!("@{}", epoch))
            .arg(path)
            .status()
            .expect("touch must be spawnable");
        assert!(status.success(), "touch could not set the mtime on {}", path);
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the normalization is the one step every
    // source ends in, so the `agent-` strip is asserted on the token and not on a substring of it
    #[test]
    fn the_normalization_strips_an_agent_token_then_takes_eight_characters() {
        assert_eq!(normalize("agent-deadbeefcafe0000"), "deadbeef");
        assert_eq!(normalize("abcdef0123456789"), "abcdef01");
        assert_eq!(normalize("agentic-0123456789"), "agentic-");
        assert_eq!(normalize("short"), "short");
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the sessions-dir default: the cwd's every
    // non-alphanumeric character maps to `-`, and `CLAUDE_CONFIG_DIR` displaces `~/.claude`
    #[test]
    fn the_default_sessions_dir_slugs_the_cwd_under_the_config_home() {
        let mut i = inputs("");
        i.home = "/h".to_string();
        i.here = "/a/b-c.d".to_string();
        assert_eq!(sessions_dir(&i), "/h/.claude/projects/-a-b-c-d");
        i.config_home = "/cfg".to_string();
        assert_eq!(sessions_dir(&i), "/cfg/projects/-a-b-c-d");
        i.sessions_dir = "/over".to_string();
        assert_eq!(sessions_dir(&i), "/over");
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — sources 1 and 2, and source 2's skip: the
    // override outranks the harness uuid, and the harness uuid is taken directly only when
    // `CLAUDE_CODE_CHILD_SESSION` is unset
    #[test]
    fn the_override_outranks_the_harness_uuid_which_the_child_flag_skips() {
        let dir = scratch("order");
        transcript(&format!("{}/9999888877776666.jsonl", dir), 1_000);
        let mut i = inputs(&dir);
        i.session_id = "agent-deadbeefcafe0000".to_string();
        i.harness_id = "abcdef0123456789".to_string();
        assert_eq!(derive(&i).expect("source 1 refused"), "deadbeef");
        i.session_id = String::new();
        assert_eq!(derive(&i).expect("source 2 refused"), "abcdef01");
        i.child = "1".to_string();
        assert!(
            derive(&i).is_err(),
            "the child flag must narrow the scan to the env uuid's own subagents/ rather than \
             falling through to source 3's widened glob, which the seeded top-level transcript \
             would have answered"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — source 3's widened glob, and the mtime rule
    // it selects by: the newest transcript wins across both tiers, and a tie leaves the
    // glob-sorted earlier candidate standing rather than being order-dependent
    #[test]
    fn the_widened_scan_takes_the_newest_across_both_tiers_and_keeps_ties_stable() {
        let dir = scratch("widened");
        transcript(&format!("{}/11112222-3333.jsonl", dir), 1_000);
        transcript(&format!("{}/lead/subagents/agent-99998888aaaa.jsonl", dir), 2_000);
        let i = inputs(&dir);
        assert_eq!(derive(&i).expect("the widened scan refused"), "99998888");
        transcript(&format!("{}/aaaabbbb-3333.jsonl", dir), 2_000);
        assert_eq!(
            derive(&i).expect("the widened scan refused"),
            "aaaabbbb",
            "a top-level transcript tying the subagent's mtime must win, the top-level glob \
             running first and `-nt` replacing only on a strictly newer mtime"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the child-narrowed scan excludes the lead's
    // own top-level transcript from the candidate set even when it out-mtimes every subagent's,
    // and the spurious-flag fall-back reads that same top-level file only on an *empty* scan
    #[test]
    fn a_dispatched_child_narrows_to_its_leads_subagents_and_falls_back_when_the_scan_is_empty() {
        let dir = scratch("narrowed");
        let lead = "11112222-3333-4444-5555-666677778888";
        transcript(&format!("{}/{}/subagents/agent-aaaabbbb0000.jsonl", dir, lead), 1_000);
        transcript(&format!("{}/{}.jsonl", dir, lead), 9_000);
        let mut i = inputs(&dir);
        i.child = "1".to_string();
        i.harness_id = lead.to_string();
        assert_eq!(
            derive(&i).expect("the narrowed scan refused"),
            "aaaabbbb",
            "a newer top-level lead transcript entered the narrowed candidate set"
        );

        let spur = "ccccdddd-1111-2222-3333-444455556666";
        std::fs::create_dir_all(format!("{}/{}/subagents", dir, spur))
            .expect("the empty subagents dir must be creatable");
        transcript(&format!("{}/{}.jsonl", dir, spur), 1_000);
        i.harness_id = spur.to_string();
        assert_eq!(derive(&i).expect("the spurious-flag fall-back refused"), "ccccdddd");

        i.harness_id = "no-such-uuid".to_string();
        assert!(
            derive(&i).is_err(),
            "an empty narrowed scan with no top-level transcript must refuse"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the two refusals name the sessions dir and
    // the knob that redirects it, which is the whole remedy a caller gets
    #[test]
    fn both_refusals_name_the_sessions_dir_and_its_knob() {
        let dir = scratch("refusals");
        let absent = format!("{}/nope", dir);
        let err = derive(&inputs(&absent)).expect_err("an absent sessions dir must refuse");
        assert!(err.contains(&absent) && err.contains("LIFECYCLE_KIT_SESSIONS_DIR"), "{}", err);
        let err = derive(&inputs(&dir)).expect_err("an empty sessions dir must refuse");
        assert!(err.contains("no transcript (*.jsonl)") && err.contains(&dir), "{}", err);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
