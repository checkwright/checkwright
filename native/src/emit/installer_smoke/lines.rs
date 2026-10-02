// spec: installer/SPEC.md §The consumer smoke — the terminator's one owner: every child's
// line-oriented output is split here, exactly one trailing carriage return dropped per line, and the
// count declared on the green path as on the red
use super::say;

pub(super) struct Read {
    pub lines: Vec<String>,
    pub stripped: usize,
}

// spec: installer/SPEC.md §The consumer smoke — the split alone, with no declaration, so the unit
// test reads the reader's own contract
pub(super) fn split(text: &str) -> Read {
    let mut lines = Vec::new();
    let mut stripped = 0;
    if !text.is_empty() {
        let body = text.strip_suffix('\n').unwrap_or(text);
        for raw in body.split('\n') {
            match raw.strip_suffix('\r') {
                Some(cut) => {
                    stripped += 1;
                    lines.push(cut.to_string());
                }
                None => lines.push(raw.to_string()),
            }
        }
    }
    Read { lines, stripped }
}

pub(super) fn declaration(what: &str, read: &Read) -> Option<String> {
    (read.stripped > 0).then(|| {
        format!(
            "{}: the stream delivered {} of {} line(s) ending in a carriage return, each dropped as a line terminator",
            what,
            read.stripped,
            read.lines.len()
        )
    })
}

// spec: installer/SPEC.md §The consumer smoke — the reader every arm calls: the lines, and the
// declaration printed where a strip fired
pub(super) fn of(what: &str, bytes: &[u8]) -> Vec<String> {
    let read = split(&String::from_utf8_lossy(bytes));
    if let Some(d) = declaration(what, &read) {
        say(&d);
    }
    read.lines
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The consumer smoke — the synthetic CRLF stream: one carriage return
    // off each CRLF line, none off a bare LF one, every record kept, and the count declared
    #[test]
    fn a_crlf_stream_loses_one_terminator_byte_per_line_and_says_how_many() {
        let read = split("a\r\nb\nc\r\n");
        assert_eq!(read.lines, vec!["a", "b", "c"]);
        assert_eq!(read.stripped, 2);
        assert_eq!(
            declaration("self-test", &read).as_deref(),
            Some("self-test: the stream delivered 2 of 3 line(s) ending in a carriage return, each dropped as a line terminator")
        );
    }

    // spec: installer/SPEC.md §The consumer smoke — exactly one is taken, so a doubled carriage
    // return still reaches the value and its shape test
    #[test]
    fn a_doubled_carriage_return_keeps_one() {
        let read = split("a\r\r\n");
        assert_eq!(read.lines, vec!["a\r"]);
        assert_eq!(read.stripped, 1);
    }

    // spec: installer/SPEC.md §The consumer smoke — an LF stream declares nothing, an unterminated
    // last line is a record, and an empty stream is no records
    #[test]
    fn an_lf_stream_declares_nothing() {
        let read = split("a\nb");
        assert_eq!(read.lines, vec!["a", "b"]);
        assert!(declaration("x", &read).is_none());
        assert!(split("").lines.is_empty());
    }
}
