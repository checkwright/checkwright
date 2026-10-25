// spec: gate-sdk/SPEC.md §lib/inject.sh — the marker-block module: one reader shared by every block
// consumer, a writer that replaces a block in place, an installer that appends when absent, and a
// remover. It is the whole of the mechanism; nothing holds a second copy.
use crate::fresh;

// spec: gate-sdk/SPEC.md §lib/inject.sh — a line as a marker or blank-line test reads it: one
// trailing carriage return dropped, so a CRLF-converted file keeps its blocks
pub fn bare(line: &str) -> &str {
    line.strip_suffix('\r').unwrap_or(line)
}

fn is(line: &str, marker: &str) -> bool {
    bare(line) == marker
}

pub fn has(lines: &[&str], marker: &str) -> bool {
    lines.iter().any(|l| is(l, marker))
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — whole-line equality on each marker, the lines strictly
// between them, and the trailing newlines a `$(…)` capture strips. An indented or embedded
// spelling opens nothing, which is what keeps a marker from being forgeable by prose.
pub fn read_block(text: &str, begin: &str, end: &str) -> String {
    let mut inb = false;
    let mut out: Vec<&str> = Vec::new();
    for line in fresh::file_lines(text) {
        if is(line, begin) {
            inb = true;
            continue;
        }
        if is(line, end) {
            inb = false;
            continue;
        }
        if inb {
            out.push(line);
        }
    }
    out.join("\n").trim_end_matches('\n').to_string()
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — every writer reads and rewrites the file as bytes, so a
// byte outside the block that is not UTF-8 is written back as it was read
fn read_bytes(path: &str) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|_| fresh::fail_closed("cat", Some(1)))
}

fn byte_lines(text: &[u8]) -> Vec<&[u8]> {
    if text.is_empty() {
        return Vec::new();
    }
    text.strip_suffix(b"\n").unwrap_or(text).split(|&c| c == b'\n').collect()
}

fn bare_bytes(line: &[u8]) -> &[u8] {
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn is_line(line: &[u8], marker: &str) -> bool {
    bare_bytes(line) == marker.as_bytes()
}

fn has_line(lines: &[&[u8]], marker: &str) -> bool {
    lines.iter().any(|l| is_line(l, marker))
}

fn write_bytes(path: &str, out: &[u8]) -> Result<(), String> {
    std::fs::write(path, out).map_err(|e| format!("cannot write {}: {}", path, e))
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the one well-formedness test: once a begin marker is
// present, each begin closes at an end before the next begin and each end closes an open begin
fn paired(path: &str, lines: &[&[u8]], begin: &str, end: &str) -> Result<(), String> {
    if !has_line(lines, begin) {
        return Ok(());
    }
    let mut open: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        if is_line(line, begin) {
            if let Some(o) = open {
                return Err(format!(
                    "{}: a second begin marker at line {} inside the block opened at line {} — refusing to guess the block bounds",
                    path,
                    i + 1,
                    o + 1
                ));
            }
            open = Some(i);
        } else if is_line(line, end) && open.take().is_none() {
            return Err(format!(
                "{}: the end marker at line {} precedes its begin marker — refusing to guess the block bounds",
                path,
                i + 1
            ));
        }
    }
    if open.is_some() {
        return Err(format!(
            "{}: begin marker present but end marker missing — refusing to guess the block bounds",
            path
        ));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — replace the span between an existing marker pair, the
// file otherwise byte-untouched. Deliberately **tightens** the shell original, which appends a
// fresh block on an absent begin marker where this refuses.
pub fn write_block(path: &str, begin: &str, end: &str, body: &str) -> Result<(), String> {
    let text = read_bytes(path)?;
    let lines = byte_lines(&text);
    if !has_line(&lines, begin) {
        return Err(if has_line(&lines, end) {
            format!("{}: end marker present but begin marker missing — refusing to guess the block bounds", path)
        } else {
            format!(
                "{}: no {} marker — refusing to append a fresh block, which would corrupt a hand-authored page and read back as staleness",
                path, begin
            )
        });
    }
    paired(path, &lines, begin, end)?;
    let b = lines.iter().position(|l| is_line(l, begin)).unwrap_or_default();
    let e = b + lines[b..].iter().position(|l| is_line(l, end)).unwrap_or_default();

    let mut out: Vec<&[u8]> = lines[..=b].to_vec();
    out.extend(fresh::file_lines(body).into_iter().map(str::as_bytes));
    out.extend_from_slice(&lines[e..]);
    let mut rendered = out.join(&b'\n');
    if text.ends_with(b"\n") {
        rendered.push(b'\n');
    }
    write_bytes(path, &rendered)
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the installer half: `inject_marker_block`'s
// append-on-absent disposition, which is correct for a seeding installer and is exactly what
// `write_block` above refuses on behalf of a generator. Two writers, two misses, one module.
// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the action word is the return value
// because the caller reports it; the begin-without-end refusal is exit 2 at that caller.
pub fn install_block(path: &str, begin: &str, end: &str, body: &str) -> Result<String, String> {
    let text = read_bytes(path)?;
    let lines = byte_lines(&text);
    paired(path, &lines, begin, end)?;
    let mut block: Vec<u8> = Vec::new();
    block.extend_from_slice(begin.as_bytes());
    block.push(b'\n');
    block.extend_from_slice(body.as_bytes());
    if !body.ends_with('\n') {
        block.push(b'\n');
    }
    block.extend_from_slice(end.as_bytes());
    block.push(b'\n');

    // spec: gate-sdk/SPEC.md §lib/inject.sh — the presence test is whole-line, the module's own
    // stated rule; the shell form gated the same replace on a substring grep, so a marker
    // occurring inside prose sent it down a replace path that matched nothing and reported it
    if !has_line(&lines, begin) {
        let mut out = text.clone();
        out.push(b'\n');
        out.extend_from_slice(&block);
        write_bytes(path, &out)?;
        return Ok("appended".to_string());
    }

    let mut out: Vec<u8> = Vec::new();
    let mut skip = false;
    for line in &lines {
        if is_line(line, begin) {
            skip = true;
            out.extend_from_slice(&block);
            continue;
        }
        if is_line(line, end) {
            skip = false;
            continue;
        }
        if !skip {
            out.extend_from_slice(line);
            out.push(b'\n');
        }
    }
    if !terminated(&text) {
        out.pop();
    }
    write_bytes(path, &out)?;
    Ok("replaced".to_string())
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — a rebuild ends terminated iff its input did, so a block
// another writer left as an unterminated last line stays byte-identical
fn terminated(text: &[u8]) -> bool {
    text.is_empty() || text.ends_with(b"\n")
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the writers' refusal, readable before a caller's first
// write
pub fn installable(path: &str, begin: &str, end: &str) -> Result<(), String> {
    let text = read_bytes(path)?;
    paired(path, &byte_lines(&text), begin, end)
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the removal half beside its two siblings: the marker pair
// and everything between it go, the rest of the file byte-identical. An absent begin marker is a
// no-op reporting `false`; a target that does not pair is malformed on the writers' own terms.
pub fn remove_block(path: &str, begin: &str, end: &str) -> Result<bool, String> {
    let text = read_bytes(path)?;
    let lines = byte_lines(&text);
    if !has_line(&lines, begin) {
        return Ok(false);
    }
    paired(path, &lines, begin, end)?;
    let mut out: Vec<u8> = Vec::new();
    let mut skip = false;
    let mut last_kept = false;
    for line in &lines {
        last_kept = false;
        if is_line(line, begin) {
            skip = true;
            continue;
        }
        if is_line(line, end) {
            skip = false;
            continue;
        }
        if !skip {
            out.extend_from_slice(line);
            out.push(b'\n');
            last_kept = true;
        }
    }
    // spec: gate-sdk/SPEC.md §lib/inject.sh — only a surviving last line carries the missing
    // terminator; where the block owned it, the line before keeps its newline
    if !terminated(&text) && last_kept {
        out.pop();
    }
    write_bytes(path, &out)?;
    Ok(true)
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the installer writer's inverse: `remove_block`'s removal
// plus the separator `install_block`'s append put before the begin marker, so a file the
// installer appended to reads its pre-install bytes again
pub fn uninstall_block(path: &str, begin: &str, end: &str) -> Result<bool, String> {
    let text = read_bytes(path)?;
    let lines = byte_lines(&text);
    let Some(b) = lines.iter().position(|l| is_line(l, begin)) else {
        return Ok(false);
    };
    paired(path, &lines, begin, end)?;
    let head = if b > 0 && bare_bytes(lines[b - 1]).is_empty() { b - 1 } else { b };
    let mut out: Vec<u8> = Vec::new();
    for line in &lines[..head] {
        out.extend_from_slice(line);
        out.push(b'\n');
    }
    let rest = &lines[b..];
    let mut tail = rest.iter().position(|l| is_line(l, end)).map_or(&[][..], |i| &rest[i + 1..]);
    // spec: gate-sdk/SPEC.md §lib/inject.sh — a newline this block's append put on an unterminated
    // line stays while lines follow, and a blank line directly after the block goes in its place
    if head == b && b > 0 && tail.first().is_some_and(|l| bare_bytes(l).is_empty()) {
        tail = &tail[1..];
    }
    for line in tail {
        out.extend_from_slice(line);
        out.push(b'\n');
    }
    // spec: gate-sdk/SPEC.md §lib/inject.sh — an append onto an unterminated last line wrote the
    // newline that terminates it rather than a blank line, so that newline goes while the block
    // still ends the file, with the carriage return a CRLF conversion put before it
    if tail.is_empty() {
        if head == b && b > 0 {
            out.pop();
            if out.ends_with(b"\r") {
                out.pop();
            }
        }
    } else if !terminated(&text) {
        out.pop();
    }
    write_bytes(path, &out)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §lib/inject.sh — what the installer writer appended, its inverse takes
    // back byte for byte, a re-run is a no-op, and a half-open pair refuses
    #[test]
    fn the_installer_writers_inverse_restores_the_pre_install_bytes() {
        let p = std::env::temp_dir()
            .join(format!("cw-marker-uninstall-{}.txt", std::process::id()))
            .display()
            .to_string();
        for before in ["", "# a comment\n", "kept\n\n", "unterminated", "two\nlines"] {
            std::fs::write(&p, before).expect("cannot seed");
            install_block(&p, B, E, "one\n").expect("the block was not installed");
            install_block(&p, B, E, "two\n").expect("the block was not replaced");
            assert_eq!(uninstall_block(&p, B, E), Ok(true));
            assert_eq!(fresh::read_captured(&p).expect("cannot read back"), before, "from {:?}", before);
            assert_eq!(uninstall_block(&p, B, E), Ok(false));
        }
        // spec: gate-sdk/SPEC.md §lib/inject.sh — a line added after the block keeps the newline
        // the append put on an unterminated line, since dropping it would join the two
        std::fs::write(&p, "unterminated").expect("cannot seed");
        install_block(&p, B, E, "one\n").expect("the block was not installed");
        let mut grown = fresh::read_captured(&p).expect("cannot read back");
        grown.push_str("added\n");
        std::fs::write(&p, grown).expect("cannot grow");
        assert_eq!(uninstall_block(&p, B, E), Ok(true));
        assert_eq!(fresh::read_captured(&p).expect("cannot read back"), "unterminated\nadded\n");
        std::fs::write(&p, format!("head\n{}\na\n", B)).expect("cannot seed");
        assert!(uninstall_block(&p, B, E).is_err());
        std::fs::remove_file(&p).ok();
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — two blocks installed in turn read the pre-install bytes
    // again whichever order they are removed in, an unterminated last line included
    #[test]
    fn two_blocks_removed_in_either_order_restore_the_pre_install_bytes() {
        let p = std::env::temp_dir()
            .join(format!("cw-marker-two-{}.txt", std::process::id()))
            .display()
            .to_string();
        for before in ["", "# a comment\n", "kept\n\n", "keep", "two\nlines"] {
            for (first, second) in [((B, E), (B2, E2)), ((B2, E2), (B, E))] {
                std::fs::write(&p, before).expect("cannot seed");
                install_block(&p, B, E, "a\n").expect("the first block was not installed");
                install_block(&p, B2, E2, "b\n").expect("the second block was not installed");
                assert_eq!(uninstall_block(&p, first.0, first.1), Ok(true));
                assert_eq!(uninstall_block(&p, second.0, second.1), Ok(true));
                assert_eq!(
                    fresh::read_captured(&p).expect("cannot read back"),
                    before,
                    "from {:?}, {} removed first",
                    before,
                    first.0
                );
            }
        }
        std::fs::remove_file(&p).ok();
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — another block whose end marker is an unterminated last
    // line keeps its bytes through each rebuild of this one
    #[test]
    fn an_unterminated_last_line_stays_unterminated_through_every_rebuild() {
        let p = std::env::temp_dir()
            .join(format!("cw-marker-unterminated-{}.txt", std::process::id()))
            .display()
            .to_string();
        let seed = format!("head\n{}\nold\n{}\n\n{}\nb\n{}", B, E, B2, E2);
        std::fs::write(&p, &seed).expect("cannot seed");
        assert_eq!(install_block(&p, B, E, "new\n"), Ok("replaced".to_string()));
        assert_eq!(
            fresh::read_captured(&p).expect("cannot read back"),
            format!("head\n{}\nnew\n{}\n\n{}\nb\n{}", B, E, B2, E2)
        );
        assert_eq!(uninstall_block(&p, B, E), Ok(true));
        assert_eq!(fresh::read_captured(&p).expect("cannot read back"), format!("head\n{}\nb\n{}", B2, E2));
        std::fs::write(&p, &seed).expect("cannot reseed");
        assert_eq!(remove_block(&p, B, E), Ok(true));
        assert_eq!(fresh::read_captured(&p).expect("cannot read back"), format!("head\n\n{}\nb\n{}", B2, E2));
        std::fs::remove_file(&p).ok();
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — a CRLF-converted file keeps its block: the installer
    // writer replaces rather than appending a second, the half-open refusal reads, and the inverse
    // restores the converted pre-install bytes
    #[test]
    fn a_marker_line_with_a_trailing_carriage_return_still_matches() {
        let p = std::env::temp_dir()
            .join(format!("cw-marker-crlf-{}.txt", std::process::id()))
            .display()
            .to_string();
        let read = || fresh::read_captured(&p).expect("cannot read back");
        for before in ["keep\r\n", "keep"] {
            std::fs::write(&p, before).expect("cannot seed");
            install_block(&p, B, E, "one\n").expect("the block was not installed");
            std::fs::write(&p, read().replace("\r\n", "\n").replace('\n', "\r\n")).expect("cannot convert");
            assert_eq!(installable(&p, B, E), Ok(()));
            assert_eq!(install_block(&p, B, E, "two\n"), Ok("replaced".to_string()));
            assert_eq!(read().matches(B).count(), 1, "a second block was appended from {:?}", before);
            assert_eq!(read_block(&read(), B, E), "two");
            assert_eq!(uninstall_block(&p, B, E), Ok(true));
            assert_eq!(read(), before);
        }
        std::fs::write(&p, format!("a\r\n{}\r\nb\r\n{}\r\nc\r\n", B, E)).expect("cannot seed");
        assert_eq!(remove_block(&p, B, E), Ok(true));
        assert_eq!(read(), "a\r\nc\r\n");
        std::fs::write(&p, format!("a\r\n{}\r\nb\r\n", B)).expect("cannot seed");
        assert!(installable(&p, B, E).is_err(), "a CRLF half-open pair was not refused");
        std::fs::remove_file(&p).ok();
    }

    const B: &str = "<!-- x:begin -->";
    const E: &str = "<!-- x:end -->";
    const B2: &str = "<!-- y:begin -->";
    const E2: &str = "<!-- y:end -->";

    // spec: gate-sdk/SPEC.md §lib/inject.sh — the removal is idempotent and leaves the rest of the
    // file alone; a begin marker without its end refuses rather than guessing the bounds.
    #[test]
    fn a_removal_is_idempotent_and_refuses_a_half_open_pair() {
        let p = std::env::temp_dir()
            .join(format!("cw-marker-remove-{}.md", std::process::id()))
            .display()
            .to_string();
        std::fs::write(&p, format!("head\n{}\na\n{}\ntail\n", B, E)).expect("cannot seed");
        assert_eq!(remove_block(&p, B, E), Ok(true));
        assert_eq!(
            std::fs::read_to_string(&p).expect("cannot read back"),
            "head\ntail\n"
        );
        assert_eq!(remove_block(&p, B, E), Ok(false));
        std::fs::write(&p, format!("head\n{}\na\n", B)).expect("cannot seed");
        assert!(remove_block(&p, B, E).is_err());
        std::fs::remove_file(&p).ok();
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — a stray end ahead of the begin, a second begin left
    // open after a complete block and a begin inside an open block each refuse every writer, the
    // file untouched; an end with no begin anywhere is no block, so the removal stays a no-op
    #[test]
    fn a_target_whose_markers_do_not_pair_refuses_every_writer() {
        let p = std::env::temp_dir()
            .join(format!("cw-marker-pairing-{}.md", std::process::id()))
            .display()
            .to_string();
        for (seed, said) in [
            (format!("head\n{}\n{}\nowned\n", E, B), "precedes its begin marker"),
            (format!("{}\na\n{}\nkept\n{}\nowned\n", B, E, B), "end marker missing"),
            (format!("{}\na\n{}\nb\n{}\n", B, B, E), "a second begin marker at line 3"),
        ] {
            std::fs::write(&p, &seed).expect("cannot seed");
            let refusals = [
                remove_block(&p, B, E).map(|_| ()),
                uninstall_block(&p, B, E).map(|_| ()),
                install_block(&p, B, E, "x\n").map(|_| ()),
                installable(&p, B, E),
                write_block(&p, B, E, "x"),
            ];
            for r in refusals {
                assert!(r.as_ref().is_err_and(|e| e.contains(said)), "{:?} over {:?}", r, seed);
            }
            assert_eq!(fresh::read_captured(&p).expect("cannot read back"), seed, "a refusal wrote");
        }
        std::fs::write(&p, format!("head\n{}\n", E)).expect("cannot seed");
        assert_eq!(remove_block(&p, B, E), Ok(false));
        std::fs::remove_file(&p).ok();
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — bytes that are not UTF-8 outside the block come back
    // as they were read, through every writer
    #[test]
    fn bytes_outside_the_block_that_are_not_utf8_survive_every_writer() {
        let p = std::env::temp_dir()
            .join(format!("cw-marker-bytes-{}.md", std::process::id()))
            .display()
            .to_string();
        let head: &[u8] = b"caf\xe9 \xff\xfe\n";
        let tail: &[u8] = b"\x80tail\n";
        let seeded = |body: &[u8]| -> Vec<u8> { [head, format!("{}\n", B).as_bytes(), body, format!("{}\n", E).as_bytes(), tail].concat() };
        let read = || std::fs::read(&p).expect("cannot read back");
        std::fs::write(&p, seeded(b"old\n")).expect("cannot seed");
        write_block(&p, B, E, "new").expect("a well-formed pair is replaced");
        assert_eq!(read(), seeded(b"new\n"));
        assert_eq!(install_block(&p, B, E, "again\n"), Ok("replaced".to_string()));
        assert_eq!(read(), seeded(b"again\n"));
        assert_eq!(remove_block(&p, B, E), Ok(true));
        assert_eq!(read(), [head, tail].concat());
        std::fs::write(&p, head).expect("cannot seed");
        assert_eq!(install_block(&p, B, E, "x\n"), Ok("appended".to_string()));
        assert_eq!(uninstall_block(&p, B, E), Ok(true));
        assert_eq!(read(), head);
        std::fs::remove_file(&p).ok();
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — an unterminated input stays unterminated only while its
    // last line survives the removal; a block owning that line leaves the line before it terminated
    #[test]
    fn a_removed_block_that_ended_the_file_unterminated_leaves_the_line_before_it_terminated() {
        let p = std::env::temp_dir()
            .join(format!("cw-marker-tail-{}.md", std::process::id()))
            .display()
            .to_string();
        for (before, after) in [
            (format!("head\n{}\nbody\n{}", B, E), "head\n".to_string()),
            (format!("head\r\n{}\r\nbody\r\n{}", B, E), "head\r\n".to_string()),
            (format!("{}\nbody\n{}", B, E), String::new()),
            (format!("head\n{}\nbody\n{}\ntail", B, E), "head\ntail".to_string()),
            (format!("head\n{}\nbody\n{}\ntail\n", B, E), "head\ntail\n".to_string()),
        ] {
            std::fs::write(&p, &before).expect("cannot seed");
            assert_eq!(remove_block(&p, B, E), Ok(true));
            assert_eq!(fresh::read_captured(&p).expect("cannot read back").as_bytes(), after.as_bytes(), "from {:?}", before);
        }
        std::fs::remove_file(&p).ok();
    }

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
        // spec: gate-sdk/SPEC.md §lib/inject.sh — the same refusal, read ahead of any write
        assert_eq!(installable(&unbalanced, B, E), install_block(&unbalanced, B, E, "new").map(|_| ()));
        assert_eq!(installable(&fresh_file, B, E), Ok(()));

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
