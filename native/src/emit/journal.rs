// spec: lifecycle-kit/SPEC.md §The journal arm — the append affordance: the caller's own stamp names
// the journal, so a stage session journals without spelling the path its entry printed.
use crate::stages;
use crate::walk;

pub const KNOBS: &[&str] = &[
    "LIFECYCLE_KIT_STATE_FILE",
    "LIFECYCLE_KIT_STAGES",
    "LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN",
    "LIFECYCLE_KIT_LEAD_JOURNAL_FILE",
    "GATE_SDK_TMP_DIR",
];

const LEAD: &str = "--lead";

pub const USAGE: &str = "usage: --emit journal [--] \"<text>\"\n       --emit journal --lead [--] \"<text>\"\n  appends the text, or standard input when no text is given, to the resume journal of the stage the calling session entered, or with --lead as the first argument to the lead journal --enter-stage --open-lead-journal opened; \"--\" admits a text beginning with \"-\"";

// spec: lifecycle-kit/SPEC.md §The journal arm — verbatim is the bytes: the blank test reads a
// lossy view and the append never does, one closing newline added where the text lacks one; a
// text with nothing but whitespace is refused, never appended
fn shape(text: &[u8]) -> Result<Vec<u8>, String> {
    if String::from_utf8_lossy(text).trim().is_empty() {
        return Err(format!("the text is empty, so nothing was appended\n{}", USAGE));
    }
    let mut body = text.to_vec();
    if !body.ends_with(b"\n") {
        body.push(b'\n');
    }
    Ok(body)
}

// spec: lifecycle-kit/SPEC.md §The journal arm — refused in a linked worktree, where the scratch
// dir is that worktree's own and no reader of the main checkout's journal sees the line
fn worktree_refusal(main: Option<&str>) -> Result<(), String> {
    match main {
        None => Ok(()),
        Some(m) => Err(format!(
            "this checkout is a linked worktree of `{}`, so a journal line written here never \
             reaches the journal the main checkout's readers open. Hand the finding back to your \
             dispatcher in your report.",
            m
        )),
    }
}

// spec: lifecycle-kit/SPEC.md §The journal arm — an absent state file carries no stamp, so it
// reaches the refusal naming the id and the remedy; any other read failure stays an I/O error
fn stamps(anchored: &str) -> Result<String, String> {
    if std::path::Path::new(anchored).exists() {
        super::read_text(anchored)
    } else {
        Ok(String::new())
    }
}

// spec: lifecycle-kit/SPEC.md §The journal arm — the caller: the derived id where a stamp carries
// it, else the first id down the derivation's own scan that one does, so a live child's newer
// transcript does not unseat the stamped session that dispatched it
fn caller(state_text: &str, derived: &str, scan: &[String], stages: &[String]) -> Option<(String, String)> {
    std::iter::once(derived.to_string())
        .chain(scan.iter().map(|p| crate::sessions::key(p)))
        .find_map(|id| stages::caller_stage(state_text, &id, stages).map(|s| (id, s)))
}

fn text_of(args: &[String]) -> Result<Vec<u8>, String> {
    let fields = super::file_survey::positionals(args, "text").map_err(|e| format!("{}\n{}", e, USAGE))?;
    match fields {
        [] => {
            use std::io::Read;
            let mut buf = Vec::new();
            std::io::stdin()
                .read_to_end(&mut buf)
                .map_err(|e| format!("cannot read standard input: {}", e))?;
            Ok(buf)
        }
        [one] => Ok(one.clone().into_bytes()),
        _ => Err(format!("one text is taken, and {} were given\n{}", fields.len(), USAGE)),
    }
}

fn append(path: &str, body: &[u8]) -> Result<(), String> {
    use std::io::Write;
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut f| f.write_all(body))
        .map_err(|e| format!("cannot append to {}: {}", path, e))
}

// spec: lifecycle-kit/SPEC.md §The journal arm — the lead form's own refusals: an absent journal
// and a disposed last segment, each naming the opener as the remedy
fn lead_refusal(path: &str, text: Option<&str>) -> Result<(), String> {
    const REMEDY: &str = "run --enter-stage --open-lead-journal first";
    match text {
        None => Err(format!("the lead journal {} does not exist, so nothing was appended — {}", path, REMEDY)),
        Some(t) if super::enter_stage::lead_journal_last_disposed(t) => Err(format!(
            "the last segment of the lead journal {} is disposed (its last non-empty line is {}), and a              line after the mark would turn it back into an undisposed one, so nothing was appended — {}",
            path,
            super::enter_stage::DISPOSITION_MARK,
            REMEDY
        )),
        Some(_) => Ok(()),
    }
}

// spec: lifecycle-kit/SPEC.md §The journal arm — the lead form: the knob names the file, so no
// stamp is read and no id derived
fn emit_lead(body: &[u8]) -> Result<String, String> {
    let spelled = super::enter_stage::lead_journal_spelled(
        &walk::knob_scalar("GATE_SDK_TMP_DIR")?,
        &walk::knob_scalar("LIFECYCLE_KIT_LEAD_JOURNAL_FILE")?,
    );
    let path = super::enter_stage::repo_anchored(&spelled)?;
    let text = std::fs::read(&path).ok().map(|b| String::from_utf8_lossy(&b).into_owned());
    lead_refusal(&path, text.as_deref())?;
    append(&path, body)?;
    Ok(format!("journal: {} lead\n", path))
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let lead = args.first().map(String::as_str) == Some(LEAD);
    let body = shape(&text_of(&args[usize::from(lead)..])?)?;
    worktree_refusal(walk::main_checkout_root().as_deref())?;
    if lead {
        return emit_lead(&body);
    }

    let inputs = super::session_id::inputs()?;
    let derived = super::session_id::derive(&inputs)?;
    let state = walk::knob_scalar("LIFECYCLE_KIT_STATE_FILE")?;
    let state_text = stamps(&super::enter_stage::repo_anchored(&state)?)?;
    let scan = crate::sessions::scan_newest_first(&inputs);
    let Some((id, stage)) = caller(&state_text, &derived, &scan, &stages::stages()?) else {
        return Err(format!(
            "no stage stamp in {} carries this session's id ({}), so it has no journal to append \
             to — run --enter-stage <stage> first",
            state, derived
        ));
    };

    let spelled =
        super::enter_stage::journal_path(&walk::knob_scalar("LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN")?, &stage);
    let path = super::enter_stage::repo_anchored(&spelled)?;
    if let Some(dir) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
    }
    append(&path, &body)?;
    Ok(format!("journal: {} {} {}\n", path, stage, id))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §The journal arm — the closing newline is added once and never
    // doubled, a multi-line text keeps its inner lines, and `DONE` can stand as a last line
    #[test]
    fn a_closing_newline_is_added_once_and_never_doubled() {
        assert_eq!(shape(b"a finding").as_deref(), Ok(&b"a finding\n"[..]));
        assert_eq!(shape(b"a finding\n").as_deref(), Ok(&b"a finding\n"[..]));
        assert_eq!(shape(b"one\ntwo").as_deref(), Ok(&b"one\ntwo\n"[..]));
        assert_eq!(shape(b"DONE").as_deref(), Ok(&b"DONE\n"[..]));
    }

    #[test]
    fn a_byte_that_is_no_utf8_is_kept_and_never_replaced() {
        assert_eq!(shape(b"a\xffb").as_deref(), Ok(&b"a\xffb\n"[..]));
    }

    #[test]
    fn an_absent_state_file_reads_as_no_stamp() {
        assert_eq!(stamps("/no/such/dir/stamps.txt").as_deref(), Ok(""));
    }

    // spec: lifecycle-kit/SPEC.md §The journal arm — a stamped derived id is taken with no walk,
    // an unstamped one yields to the newest stamped candidate, and no stamped candidate is no caller
    #[test]
    fn an_unstamped_derived_id_walks_the_scan_to_the_first_stamped_one() {
        let stages: Vec<String> = ["scope", "build"].iter().map(|s| s.to_string()).collect();
        let state = "---\ndemo scope aaaaaaaa 2026-06-01 none\ndemo build bbbbbbbb 2026-06-02 none\n";
        let scan: Vec<String> = ["agent-cccccccc00", "agent-bbbbbbbb00", "agent-aaaaaaaa00"]
            .iter()
            .map(|n| format!("/s/lead/subagents/{}.jsonl", n))
            .collect();
        let got = |derived: &str, scan: &[String]| caller(state, derived, scan, &stages);
        assert_eq!(got("aaaaaaaa", &scan), Some(("aaaaaaaa".to_string(), "scope".to_string())));
        assert_eq!(got("cccccccc", &scan), Some(("bbbbbbbb".to_string(), "build".to_string())));
        assert_eq!(got("cccccccc", &scan[..1]), None);
        assert_eq!(got("cccccccc", &[]), None);
    }

    #[test]
    fn an_empty_or_blank_text_is_refused() {
        for t in ["", "\n", "  \t\n"] {
            let err = shape(t.as_bytes()).expect_err("a blank text was shaped");
            assert!(err.contains("usage: --emit journal"), "{}", err);
        }
    }

    #[test]
    fn a_linked_worktree_is_refused_naming_the_main_checkout() {
        assert!(worktree_refusal(None).is_ok());
        let err = worktree_refusal(Some("/main")).expect_err("a linked worktree was admitted");
        assert!(err.contains("`/main`"), "{}", err);
    }

    #[test]
    fn the_lead_form_refuses_an_absent_journal_and_a_disposed_last_segment() {
        let absent = lead_refusal("/s/lead.md", None).expect_err("an absent journal was admitted");
        assert!(absent.contains("--open-lead-journal"), "{}", absent);
        let disposed = lead_refusal("/s/lead.md", Some("## lead-journal opened after k\nprose\nDISPOSED\n\n"))
            .expect_err("a disposed last segment was admitted");
        assert!(disposed.contains("--open-lead-journal") && disposed.contains("DISPOSED"), "{}", disposed);
        assert!(lead_refusal("/s/lead.md", Some("")).is_ok());
        assert!(lead_refusal("/s/lead.md", Some("early\nDISPOSED\n## lead-journal opened after k\nprose\n")).is_ok());
    }

    #[test]
    fn a_second_text_and_a_dash_led_one_are_refused_and_a_separator_admits_it() {
        let argv = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert!(text_of(&argv(&["one", "two"])).is_err());
        assert!(text_of(&argv(&["--list"])).is_err());
        assert_eq!(text_of(&argv(&["--", "-led"])).as_deref(), Ok(&b"-led"[..]));
    }
}
