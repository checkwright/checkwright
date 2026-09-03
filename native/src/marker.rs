// spec: gate-sdk/SPEC.md §lib/inject.sh — the crate's marker-block half: one reader shared by
// every block consumer, and a writer that replaces a block in place. The shell library keeps its
// own copy for its remaining shell callers; this is the compiled counterpart, not its retirement.
use crate::fresh;

// spec: gate-sdk/SPEC.md §lib/inject.sh — whole-line equality on each marker, the lines strictly
// between them, and the trailing newlines a `$(…)` capture strips. An indented or embedded
// spelling opens nothing, which is what keeps a marker from being forgeable by prose.
pub fn read_block(text: &str, begin: &str, end: &str) -> String {
    let mut inb = false;
    let mut out: Vec<&str> = Vec::new();
    for line in fresh::file_lines(text) {
        if line == begin {
            inb = true;
            continue;
        }
        if line == end {
            inb = false;
            continue;
        }
        if inb {
            out.push(line);
        }
    }
    out.join("\n").trim_end_matches('\n').to_string()
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the marker pair's positions, whole-line. Returned rather
// than asserted so the writer states its own refusal wording for each malformed shape.
fn bounds(lines: &[&str], begin: &str, end: &str) -> (Option<usize>, Option<usize>) {
    (
        lines.iter().position(|l| *l == begin),
        lines.iter().position(|l| *l == end),
    )
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — replace the span between an existing marker pair, the
// file otherwise byte-untouched. Deliberately **tightens** the shell original, which appends a
// fresh block on an absent begin marker where this refuses.
pub fn write_block(path: &str, begin: &str, end: &str, body: &str) -> Result<(), String> {
    let text = fresh::read_captured(path)?;
    let lines = fresh::file_lines(&text);
    let (b, e) = bounds(&lines, begin, end);
    let (b, e) = match (b, e) {
        (Some(b), Some(e)) if b < e => (b, e),
        (Some(b), Some(e)) => {
            return Err(format!(
                "{}: the end marker precedes the begin marker (lines {} and {}) — refusing to \
                 guess the block bounds",
                path,
                e + 1,
                b + 1
            ))
        }
        (Some(_), None) => {
            return Err(format!(
                "{}: begin marker present but end marker missing — refusing to guess the block \
                 bounds",
                path
            ))
        }
        (None, Some(_)) => {
            return Err(format!(
                "{}: end marker present but begin marker missing — refusing to guess the block \
                 bounds",
                path
            ))
        }
        (None, None) => {
            return Err(format!(
                "{}: no {} marker — refusing to append a fresh block, which would corrupt a \
                 hand-authored page and read back as staleness",
                path, begin
            ))
        }
    };

    let mut out: Vec<String> = lines[..=b].iter().map(|s| s.to_string()).collect();
    for line in fresh::file_lines(body) {
        out.push(line.to_string());
    }
    out.extend(lines[e..].iter().map(|s| s.to_string()));
    let mut rendered = out.join("\n");
    if text.ends_with('\n') {
        rendered.push('\n');
    }
    std::fs::write(path, rendered).map_err(|e| format!("cannot write {}: {}", path, e))
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the installer half: `inject_marker_block`'s
// append-on-absent disposition, which is correct for a seeding installer and is exactly what
// `write_block` above refuses on behalf of a generator. Two writers, two misses, one module.
// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the action word is the return value
// because the caller reports it; the begin-without-end refusal is exit 2 at that caller.
pub fn install_block(path: &str, begin: &str, end: &str, body: &str) -> Result<String, String> {
    let text = fresh::read_captured(path)?;
    let lines = fresh::file_lines(&text);
    let mut block = String::new();
    block.push_str(begin);
    block.push('\n');
    block.push_str(body);
    if !body.ends_with('\n') {
        block.push('\n');
    }
    block.push_str(end);
    block.push('\n');

    // spec: gate-sdk/SPEC.md §lib/inject.sh — the presence test is whole-line, the module's own
    // stated rule; the shell form gated the same replace on a substring grep, so a marker
    // occurring inside prose sent it down a replace path that matched nothing and reported it
    if !lines.contains(&begin) {
        let mut out = text;
        out.push('\n');
        out.push_str(&block);
        std::fs::write(path, out).map_err(|e| format!("cannot write {}: {}", path, e))?;
        return Ok("appended".to_string());
    }
    if !lines.contains(&end) {
        return Err(format!(
            "{}: begin marker present but end marker missing — refusing to guess the block bounds",
            path
        ));
    }

    let mut out = String::new();
    let mut skip = false;
    for line in &lines {
        if *line == begin {
            skip = true;
            out.push_str(&block);
            continue;
        }
        if *line == end {
            skip = false;
            continue;
        }
        if !skip {
            out.push_str(line);
            out.push('\n');
        }
    }
    std::fs::write(path, out).map_err(|e| format!("cannot write {}: {}", path, e))?;
    Ok("replaced".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const B: &str = "<!-- x:begin -->";
    const E: &str = "<!-- x:end -->";

    #[test]
    fn the_block_is_the_lines_strictly_between_whole_line_markers() {
        let text = format!("head\n{}\na\nb\n{}\ntail\n", B, E);
        assert_eq!(read_block(&text, B, E), "a\nb");
        assert_eq!(read_block("no markers here\n", B, E), "");
        assert_eq!(read_block(&format!("  {}\na\n", B), B, E), "");
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — the three malformed shapes the writer refuses rather
    // than guessing at, which is the whole reason it diverges from the shell original
    #[test]
    fn the_writer_refuses_absent_unbalanced_and_reversed_markers() {
        let dir = std::env::temp_dir().join(format!("marker-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let case = |name: &str, content: &str| -> String {
            let p = dir.join(name).display().to_string();
            std::fs::write(&p, content).expect("write");
            p
        };

        let absent = case("absent.md", "hand authored\n");
        let err = write_block(&absent, B, E, "new").expect_err("absent markers must refuse");
        assert!(err.contains("refusing to append"), "{}", err);
        assert_eq!(
            fresh::read_captured(&absent).unwrap(),
            "hand authored\n",
            "a refused write leaves the file byte-untouched"
        );

        let unbalanced = case("unbalanced.md", &format!("a\n{}\nb\n", B));
        assert!(write_block(&unbalanced, B, E, "new")
            .expect_err("a begin with no end must refuse")
            .contains("end marker missing"));

        let reversed = case("reversed.md", &format!("{}\nmid\n{}\n", E, B));
        assert!(write_block(&reversed, B, E, "new")
            .expect_err("a reversed pair must refuse")
            .contains("precedes"));

        let ok = case("ok.md", &format!("head\n{}\nold\n{}\ntail\n", B, E));
        write_block(&ok, B, E, "one\ntwo").expect("a well-formed pair is replaced");
        assert_eq!(
            fresh::read_captured(&ok).unwrap(),
            format!("head\n{}\none\ntwo\n{}\ntail\n", B, E)
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — the installer writer's three shapes against the
    // shell original's bytes: append on an absent begin marker, replace in place on a pair, and
    // refuse a begin with no end. The blank line before an appended block is the shell form's.
    #[test]
    fn the_installer_writer_appends_replaces_and_refuses_on_the_shell_forms_bytes() {
        let dir = std::env::temp_dir().join(format!("install-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let case = |name: &str, content: &str| -> String {
            let p = dir.join(name).display().to_string();
            std::fs::write(&p, content).expect("write");
            p
        };

        let fresh_file = case("fresh.md", "hand authored\n");
        assert_eq!(
            install_block(&fresh_file, B, E, "one\ntwo\n"),
            Ok("appended".to_string())
        );
        assert_eq!(
            fresh::read_captured(&fresh_file).unwrap(),
            format!("hand authored\n\n{}\none\ntwo\n{}\n", B, E)
        );
        // spec: gate-sdk/SPEC.md §lib/inject.sh — a re-run replaces rather than duplicating,
        // which is the idempotence its installer callers rely on
        let before = fresh::read_captured(&fresh_file).unwrap();
        assert_eq!(
            install_block(&fresh_file, B, E, "one\ntwo\n"),
            Ok("replaced".to_string())
        );
        assert_eq!(fresh::read_captured(&fresh_file).unwrap(), before);

        let unbalanced = case("unbalanced.md", &format!("a\n{}\nb\n", B));
        assert!(install_block(&unbalanced, B, E, "new")
            .expect_err("a begin with no end must refuse")
            .contains("end marker missing"));

        let empty = case("empty.md", "");
        assert_eq!(
            install_block(&empty, B, E, "x\n"),
            Ok("appended".to_string())
        );
        assert_eq!(
            fresh::read_captured(&empty).unwrap(),
            format!("\n{}\nx\n{}\n", B, E)
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
