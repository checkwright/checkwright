// spec: queue-kit/SPEC.md §check-queue-prose-precondition — no active entry states a forward
// precondition in prose without a blocked-by tag (selection trusts tags, not prose)
use crate::ere::Ere;
use crate::queue;

// spec: queue-kit/SPEC.md §check-queue-prose-precondition — the two rewrites, patterns baked
// literally into this member's own source rather than resolved from consumer config: bracket tags
// and links come out of the prose, then past-tense narration does
const BRACKET_RE_SRC: &str = "\\[[^]]*\\]";
const PAST_TENSE_RE_SRC: &str =
    "(once|when|after)[^.,;]*(landed|shipped|merged|resolved|completed|was [a-z]+ed)";

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — awk's `gsub`, as a caller-side loop over the
// engine's leftmost-longest `find`, which *is* gsub's match rule; the recorded promotion trigger
// for lifting it into the engine is that section's.
fn replace_all(re: &Ere, subject: &str, replacement: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = subject;
    while let Some((s, e)) = re.find(rest) {
        if !rest.is_char_boundary(s) || !rest.is_char_boundary(e) {
            return Err(format!(
                "a match spanned bytes {}..{} of a subject those offsets do not divide — the \
                 rewrite could not be built; treating as failure (not clean)",
                s, e
            ));
        }
        out.push_str(&rest[..s]);
        out.push_str(replacement);
        if e > s {
            rest = &rest[e..];
            continue;
        }
        // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — an empty match advances one character.
        // Neither pattern above can match empty; the rule is stated anyway because the failure it
        // has is a silent infinite loop rather than a wrong answer.
        if s >= rest.len() {
            rest = "";
            break;
        }
        let mut n = s + 1;
        while n < rest.len() && !rest.is_char_boundary(n) {
            n += 1;
        }
        out.push_str(&rest[s..n]);
        rest = &rest[n..];
    }
    out.push_str(rest);
    Ok(out)
}

// spec: queue-kit/SPEC.md §check-queue-prose-precondition — awk's `/^-[[:space:]]/`: a new
// top-level active entry, so the `-` is at column zero and an indented bullet is continuation
fn is_top_level_entry(line: &str) -> bool {
    let b = line.as_bytes();
    b.first() == Some(&b'-')
        && matches!(b.get(1), Some(&c) if c == b' ' || c == b'\t' || c == 0x0b || c == 0x0c || c == b'\r')
}

fn carries_block_tag(line: &str) -> bool {
    line.contains("[blocked-by:") || line.contains("[precondition-ok:")
}

struct Entry {
    startln: usize,
    lead: String,
    body: String,
    hasblock: bool,
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-queue-prose-precondition: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let sec = queue::Sections::active_and_deferred()?;
    let trig_src = queue::knob_scalar("QUEUE_KIT_PRECONDITION_REGEX")?;
    let trig = Ere::compile(&trig_src)
        .map_err(|e| format!("QUEUE_KIT_PRECONDITION_REGEX failed to compile: {}", e))?;
    let bracket = Ere::compile(BRACKET_RE_SRC)
        .map_err(|e| format!("the bracket pattern failed to compile: {}", e))?;
    let past_tense = Ere::compile(PAST_TENSE_RE_SRC)
        .map_err(|e| format!("the past-tense pattern failed to compile: {}", e))?;

    let file = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?,
    };
    let text = std::fs::read_to_string(&file).map_err(|_| format!("file not found: {}", file))?;

    let mut findings: Vec<(usize, String)> = Vec::new();
    let mut open: Option<Entry> = None;
    let mut inq = false;

    // spec: gate-sdk/SPEC.md §The twelfth cohort — `to_ascii_lowercase` and not `to_lowercase`:
    // the shell form's `tolower` is C-locale, and a Unicode fold can change a string's byte length,
    // desynchronizing the offsets the rewrite loop below slices with.
    let mut flush = |open: &mut Option<Entry>| -> Result<(), String> {
        let Some(e) = open.take() else { return Ok(()) };
        let b = e.body.to_ascii_lowercase();
        let b = replace_all(&bracket, &b, " ")?;
        let b = replace_all(&past_tense, &b, " ")?;
        if trig.is_match(&b) && !e.hasblock {
            findings.push((e.startln, e.lead));
        }
        Ok(())
    };

    for (i, line) in text.lines().enumerate() {
        let fnr = i + 1;
        if queue::is_section_line(line) {
            let name = queue::heading_name(line).unwrap_or("");
            inq = sec.active.iter().any(|a| a == name);
            flush(&mut open)?;
            continue;
        }
        if !inq {
            continue;
        }
        if is_top_level_entry(line) {
            flush(&mut open)?;
            open = Some(Entry {
                startln: fnr,
                lead: line.to_string(),
                body: line.to_string(),
                hasblock: carries_block_tag(line),
            });
            continue;
        }
        if let Some(e) = open.as_mut() {
            e.body.push(' ');
            e.body.push_str(line);
            if carries_block_tag(line) {
                e.hasblock = true;
            }
        }
    }
    flush(&mut open)?;

    if !findings.is_empty() {
        println!("check-queue-prose-precondition: active entry states a forward precondition in prose");
        println!("but carries no [blocked-by:] tag — selection trusts tags, so it is latently blocked");
        println!("yet mechanically pickable as 'first unblocked':");
        for (ln, lead) in &findings {
            println!("  {}:{}: {}", file, ln, lead);
        }
        println!("  help: tag the real blocker '[blocked-by: <slug>]', or move the entry to the");
        println!("        Deferred section, or rephrase past-tense if the precondition is already");
        println!("        met, or opt out with '[precondition-ok: <reason>]' anywhere in the entry.");
        return Ok(1);
    }

    println!(
        "QUEUE-PROSE-PRECONDITION: clean (no untagged forward precondition in the active sections of {})",
        file
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the three cases the fixture pair cannot
    // reach: no match, consecutive matches with nothing lost between them, and the empty-match
    // advance that is the loop's only non-termination mode
    #[test]
    fn the_substitution_loop_reproduces_gsub() {
        let bracket = Ere::compile(BRACKET_RE_SRC).unwrap();
        assert_eq!(
            replace_all(&bracket, "no tags here", " ").unwrap(),
            "no tags here"
        );
        assert_eq!(
            replace_all(&bracket, "a[x][y]b", " ").unwrap(),
            "a  b"
        );
        let star = Ere::compile("x*").unwrap();
        assert_eq!(replace_all(&star, "ab", "-").unwrap(), "-a-b-");
        assert_eq!(replace_all(&star, "", "-").unwrap(), "-");
    }

    // spec: queue-kit/SPEC.md §check-queue-prose-precondition — the rewrite replaces with a space,
    // and a space can bridge text that was not adjacent, which is why the port builds the string
    // rather than testing whether the trigger's span falls outside the stripped spans
    #[test]
    fn a_replacement_space_can_bridge_text_that_was_not_adjacent() {
        let bracket = Ere::compile(BRACKET_RE_SRC).unwrap();
        assert_eq!(
            replace_all(&bracket, "a gated[x]on b", " ").unwrap(),
            "a gated on b"
        );
        let past = Ere::compile(PAST_TENSE_RE_SRC).unwrap();
        assert_eq!(
            replace_all(&past, "c waitingonce foo landedon d", " ").unwrap(),
            "c waiting on d"
        );
    }

    // spec: queue-kit/SPEC.md §check-queue-prose-precondition — awk's `/^-[[:space:]]/`, which
    // an indented continuation bullet does not match
    #[test]
    fn only_a_column_zero_bullet_opens_an_entry() {
        assert!(is_top_level_entry("- **slug** — prose"));
        assert!(!is_top_level_entry("  - a continuation bullet"));
        assert!(!is_top_level_entry("-no-space"));
    }
}
