// spec: queue-kit/SPEC.md §check-queue-prose-precondition — no active entry states a forward
// precondition in prose without a blocked-by tag (selection trusts tags, not prose)
use crate::ere::Ere;
use crate::queue;

// spec: queue-kit/SPEC.md §check-queue-prose-precondition — the three rewrites: bracket tags and
// links, then past-tense narration (`QUEUE_KIT_PRECONDITION_PAST_REGEX`, the one consumer
// calibration), then the queue's own state name spelled unbracketed; the other two are grammar
const BRACKET_RE_SRC: &str = "\\[[^]]*\\]";
const TAG_WORD_RE_SRC: &str = "design-pending";

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

const SPAN_CAP: usize = 80;

fn clause_end(c: &char) -> bool {
    matches!(c, '.' | ';' | '\n')
}

// spec: queue-kit/SPEC.md §check-queue-prose-precondition — the match widened to its enclosing
// clause, capped each side, quoted from the rewritten body rather than the author's spelling
fn fired_span(b: &str, s: usize, e: usize) -> String {
    let mut left: Vec<char> = b[..s].chars().rev().take_while(|c| !clause_end(c)).take(SPAN_CAP).collect();
    left.reverse();
    let right: String = b[e..].chars().take_while(|c| !clause_end(c)).take(SPAN_CAP).collect();
    let whole = format!("{}{}{}", left.into_iter().collect::<String>(), &b[s..e], right);
    whole.split_whitespace().collect::<Vec<_>>().join(" ")
}

// spec: queue-kit/SPEC.md §Layout and configuration — `off` strips nothing, and a value that
// does not compile is unreadable config, exit 2
fn past_tense(src: &str) -> Result<Option<Ere>, String> {
    if src == "off" {
        return Ok(None);
    }
    Ere::compile(src)
        .map(Some)
        .map_err(|e| format!("QUEUE_KIT_PRECONDITION_PAST_REGEX failed to compile: {}", e))
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
    let past_tense = past_tense(&queue::knob_scalar("QUEUE_KIT_PRECONDITION_PAST_REGEX")?)?;
    let tag_word = Ere::compile(TAG_WORD_RE_SRC)
        .map_err(|e| format!("the state-name pattern failed to compile: {}", e))?;

    let file = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?,
    };
    let text = std::fs::read_to_string(&file).map_err(|_| format!("file not found: {}", file))?;

    let mut findings: Vec<(usize, String, String)> = Vec::new();

    // spec: gate-sdk/SPEC.md §The twelfth cohort — `to_ascii_lowercase` and not `to_lowercase`:
    // the shell form's `tolower` is C-locale, and a Unicode fold can change a string's byte length,
    // desynchronizing the offsets the rewrite loop below slices with.
    let mut flush = |open: &mut Option<Entry>| -> Result<(), String> {
        let Some(e) = open.take() else { return Ok(()) };
        let b = e.body.to_ascii_lowercase();
        let b = replace_all(&bracket, &b, " ")?;
        let b = match &past_tense {
            Some(re) => replace_all(re, &b, " ")?,
            None => b,
        };
        let b = replace_all(&tag_word, &b, " ")?;
        if e.hasblock {
            return Ok(());
        }
        if let Some((s, t)) = trig.find(&b) {
            if !b.is_char_boundary(s) || !b.is_char_boundary(t) {
                return Err(format!(
                    "the trigger matched bytes {}..{} of a body those offsets do not divide — the \
                     fired-on clause could not be built; treating as failure (not clean)",
                    s, t
                ));
            }
            findings.push((e.startln, e.lead, fired_span(&b, s, t)));
        }
        Ok(())
    };

    let lines: Vec<&str> = text.lines().collect();
    for e in queue::entries(&lines, &sec) {
        if e.level != 3 || !sec.active.contains(&e.section) {
            continue;
        }
        let extent = &lines[e.start..e.end];
        flush(&mut Some(Entry {
            startln: e.start + 1,
            lead: lines[e.start].to_string(),
            body: extent.join(" "),
            hasblock: extent.iter().any(|l| carries_block_tag(l)),
        }))?;
    }

    if !findings.is_empty() {
        println!("check-queue-prose-precondition: active entry states a forward precondition in prose");
        println!("but carries no [blocked-by:] tag — selection trusts tags, so it is latently blocked");
        println!("yet mechanically pickable as 'first unblocked':");
        for (ln, lead, span) in &findings {
            println!("  {}:{}: {}", file, ln, lead);
            println!("      fired on: \"{}\"", span);
        }
        println!("  help: read the fired-on clause, then pick by its shape.");
        println!("    It states a precondition THIS entry waits on: tag the real blocker '[blocked-by: <slug>]',");
        println!("    move the entry to Deferred, or rephrase past-tense if the precondition is already met.");
        println!("    It is negated, or it names this entry as what something else waits on: the gate cannot");
        println!("    read either shape, and '[precondition-ok: <cause>]' anywhere in the entry is the answer");
        println!("    (queue-kit/SPEC.md §check-queue-prose-precondition).");
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
        let past = past_tense(&shipped_past()).unwrap().expect("the default is a pattern");
        assert_eq!(
            replace_all(&past, "c waitingonce foo landedon d", " ").unwrap(),
            "c waiting on d"
        );
    }

    fn shipped_past() -> String {
        match crate::knobs::queue_kit::KIT
            .rows
            .iter()
            .find(|r| r.name == "QUEUE_KIT_PRECONDITION_PAST_REGEX")
            .map(|r| &r.default)
        {
            Some(crate::knobs::Default::Scalar(s)) => s.to_string(),
            _ => panic!("the past-tense knob is a scalar row"),
        }
    }

    // spec: queue-kit/SPEC.md §Layout and configuration — the default strips the shipped phrase
    // set, `off` strips nothing, and a pattern that does not compile is refused
    #[test]
    fn the_past_tense_strip_is_the_knob_and_off_strips_nothing() {
        let past = past_tense(&shipped_past()).unwrap().expect("the default is a pattern");
        assert_eq!(replace_all(&past, "x once y landed z", " ").unwrap(), "x   z");
        assert!(past_tense("off").unwrap().is_none());
        assert!(past_tense("(unclosed").is_err());
    }

    // spec: queue-kit/SPEC.md §check-queue-prose-precondition — the fired-on clause
    #[test]
    fn the_fired_on_span_is_the_enclosing_clause() {
        let b = "- **s** — first sentence. this one waits on other landing; then more.";
        let at = b.find("waits on").unwrap();
        assert_eq!(fired_span(b, at, at + "waits on".len()), "this one waits on other landing");
        let b = "- **s** — lead. and it is   gated on the tail";
        let at = b.find("gated on").unwrap();
        assert_eq!(fired_span(b, at, at + "gated on".len()), "and it is gated on the tail");
        let long = format!("{} gated on x", "w".repeat(200));
        let at = long.find("gated on").unwrap();
        assert_eq!(fired_span(&long, at, at + 8).chars().count(), SPAN_CAP + " gated on x".len() - 1);
    }
}
