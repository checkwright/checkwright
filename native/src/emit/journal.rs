// spec: lifecycle-kit/SPEC.md §The journal arm — the append affordance: the caller's own stamp names
// the journal, so a stage session journals without spelling the path its entry printed.
use crate::stages;
use crate::walk;

pub const KNOBS: &[&str] = &[
    "LIFECYCLE_KIT_STATE_FILE",
    "LIFECYCLE_KIT_STAGES",
    "LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN",
];

pub const USAGE: &str = "usage: --emit journal [--] \"<text>\"\n  appends the text, or standard input when no text is given, to the resume journal of the stage the calling session entered; \"--\" admits a text beginning with \"-\"";

// spec: lifecycle-kit/SPEC.md §The journal arm — verbatim, one closing newline added where the
// text lacks one; a text with nothing but whitespace is refused, never appended
fn shape(text: &str) -> Result<String, String> {
    if text.trim().is_empty() {
        return Err(format!("the text is empty, so nothing was appended\n{}", USAGE));
    }
    Ok(if text.ends_with('\n') { text.to_string() } else { format!("{}\n", text) })
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

fn anchor(p: &str) -> Result<String, String> {
    let root = match walk::toplevel_opt()? {
        Some(t) => t,
        None => walk::cwd()?,
    };
    Ok(walk::abs_against(&root, p))
}

fn text_of(args: &[String]) -> Result<String, String> {
    let fields = super::file_survey::positionals(args, "text").map_err(|e| format!("{}\n{}", e, USAGE))?;
    match fields {
        [] => {
            use std::io::Read;
            let mut buf = Vec::new();
            std::io::stdin()
                .read_to_end(&mut buf)
                .map_err(|e| format!("cannot read standard input: {}", e))?;
            Ok(String::from_utf8_lossy(&buf).into_owned())
        }
        [one] => Ok(one.clone()),
        _ => Err(format!("one text is taken, and {} were given\n{}", fields.len(), USAGE)),
    }
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let body = shape(&text_of(args)?)?;
    worktree_refusal(walk::main_checkout_root().as_deref())?;

    let id = super::session_id::emit(&[])?.trim_end().to_string();
    let state = walk::knob_scalar("LIFECYCLE_KIT_STATE_FILE")?;
    let state_text = super::read_text(&anchor(&state)?)?;
    let Some(stage) = stages::caller_stage(&state_text, &id, &stages::stages()?) else {
        return Err(format!(
            "no stage stamp in {} carries this session's id ({}), so it has no journal to append \
             to — run --enter-stage <stage> first",
            state, id
        ));
    };

    let spelled =
        super::enter_stage::journal_path(&walk::knob_scalar("LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN")?, &stage);
    let path = anchor(&spelled)?;
    if let Some(dir) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
    }
    {
        use std::io::Write;
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .and_then(|mut f| f.write_all(body.as_bytes()))
            .map_err(|e| format!("cannot append to {}: {}", spelled, e))?;
    }
    Ok(format!("journal: {} {} {}\n", spelled, stage, id))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §The journal arm — the closing newline is added once and never
    // doubled, a multi-line text keeps its inner lines, and `DONE` can stand as a last line
    #[test]
    fn a_closing_newline_is_added_once_and_never_doubled() {
        assert_eq!(shape("a finding").as_deref(), Ok("a finding\n"));
        assert_eq!(shape("a finding\n").as_deref(), Ok("a finding\n"));
        assert_eq!(shape("one\ntwo").as_deref(), Ok("one\ntwo\n"));
        assert_eq!(shape("DONE").as_deref(), Ok("DONE\n"));
    }

    #[test]
    fn an_empty_or_blank_text_is_refused() {
        for t in ["", "\n", "  \t\n"] {
            let err = shape(t).expect_err("a blank text was shaped");
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
    fn a_second_text_and_a_dash_led_one_are_refused_and_a_separator_admits_it() {
        let argv = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert!(text_of(&argv(&["one", "two"])).is_err());
        assert!(text_of(&argv(&["--list"])).is_err());
        assert_eq!(text_of(&argv(&["--", "-led"])).as_deref(), Ok("-led"));
    }
}
