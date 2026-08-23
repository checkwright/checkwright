// spec: gate-sdk/SPEC.md §check-gate-fail-closed — every awk/jq capture in the gate family
// handles subprocess exit status, so a crashed parser cannot false-green as "clean"
use crate::fresh;
use crate::walk;

// spec: gate-sdk/SPEC.md §check-gate-fail-closed — awk's `[[:space:]]` in the C locale, the
// class the shell form's parser regexes are written against
fn space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

// spec: gate-sdk/SPEC.md §check-gate-fail-closed — `/(^|[^A-Za-z])(awk|jq)[[:space:]]/`: the
// program name at a non-letter boundary and followed by whitespace, so `gawk`/`jqx` do not count
fn isparser(line: &str) -> bool {
    let b = line.as_bytes();
    for w in [&b"awk"[..], &b"jq"[..]] {
        let mut i = 0usize;
        while i + w.len() < b.len() {
            if &b[i..i + w.len()] == w
                && (i == 0 || !b[i - 1].is_ascii_alphabetic())
                && space(b[i + w.len()])
            {
                return true;
            }
            i += 1;
        }
    }
    false
}

// spec: gate-sdk/SPEC.md §check-gate-fail-closed — the four satisfying shapes: the shared
// helper, a `$?` capture, a `|| {` guard, and the per-capture exemption marker
fn sat(line: &str) -> bool {
    if line.contains("fail_closed") || line.contains("=$?") || line.contains("fail-closed-exempt") {
        return true;
    }
    let b = line.as_bytes();
    let mut i = 0usize;
    while i + 1 < b.len() {
        if b[i] == b'|' && b[i + 1] == b'|' {
            let mut j = i + 2;
            while j < b.len() && space(b[j]) {
                j += 1;
            }
            if j < b.len() && b[j] == b'{' {
                return true;
            }
        }
        i += 1;
    }
    false
}

// spec: gate-sdk/SPEC.md §check-gate-fail-closed — `/[A-Za-z_][A-Za-z0-9_]*="\$\(/` minus the
// arithmetic form `="$((`: an assignment opening a command substitution starts a capture block
fn opens_capture(line: &str) -> bool {
    if line.contains("=\"$((") {
        return false;
    }
    let b = line.as_bytes();
    let pat = b"=\"$(";
    let mut i = 0usize;
    while i + pat.len() <= b.len() {
        if &b[i..i + pat.len()] == pat {
            let mut j = i;
            while j > 0 && (b[j - 1].is_ascii_alphanumeric() || b[j - 1] == b'_') {
                j -= 1;
            }
            // spec: gate-sdk/SPEC.md §check-gate-fail-closed — the ERE may start anywhere in the
            // identifier run, so one `[A-Za-z_]` inside it is what the leading atom needs
            if b[j..i]
                .iter()
                .any(|c| c.is_ascii_alphabetic() || *c == b'_')
            {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn is_comment(line: &str) -> bool {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i < b.len() && space(b[i]) {
        i += 1;
    }
    i < b.len() && b[i] == b'#'
}

fn is_blank(line: &str) -> bool {
    line.as_bytes().iter().all(|c| space(*c))
}

struct Block {
    start: usize,
    open: String,
    hasparser: bool,
    satisfied: bool,
    heredoc: bool,
    quotes: usize,
}

// spec: gate-sdk/SPEC.md §check-gate-fail-closed — one file's scan, the awk program's per-FNR
// state machine: a capture block opens on an assignment, accumulates the satisfying shapes, and
// closes on a `)"` seen at an even single-quote count
fn scan_file(path: &str, text: &str, out: &mut Vec<String>) {
    let mut blk: Option<Block> = None;
    let mut pend = false;
    for (idx, line) in fresh::file_lines(text).iter().enumerate() {
        let fnr = idx + 1;
        if blk.is_none() {
            if is_comment(line) {
                if line.contains("fail-closed-exempt") {
                    pend = true;
                }
                continue;
            }
            if is_blank(line) {
                pend = false;
                continue;
            }
            if opens_capture(line) {
                blk = Some(Block {
                    start: fnr,
                    open: (*line).to_string(),
                    hasparser: false,
                    satisfied: pend,
                    heredoc: false,
                    quotes: 0,
                });
                pend = false;
            } else {
                pend = false;
                continue;
            }
        }
        let Some(b) = blk.as_mut() else { continue };
        b.quotes += line.matches('\'').count();
        if isparser(line) {
            b.hasparser = true;
        }
        if sat(line) {
            b.satisfied = true;
        }
        if line.contains("<<<") {
            b.heredoc = true;
        }
        if line.contains(")\"") && b.quotes % 2 == 0 {
            if b.hasparser && !b.heredoc && !b.satisfied {
                out.push(format!(
                    "  {}:{}: awk/jq capture branches on output without a fail-closed status check:\n      {}",
                    path, b.start, b.open
                ));
            }
            blk = None;
        }
    }
}

pub fn run(args: &[String]) -> i32 {
    let dirs: Vec<String> = if !args.is_empty() {
        args.to_vec()
    } else {
        let gates_dir = match walk::knob_scalar("GATE_SDK_GATES_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-gate-fail-closed: {}", e);
                return 2;
            }
        };
        let roots = match walk::kit_roots_abs() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-gate-fail-closed: {}", e);
                return 2;
            }
        };
        let mut d = vec![gates_dir];
        d.extend(roots.into_iter().map(|r| format!("{}/checks", r)));
        d
    };

    // spec: gate-sdk/SPEC.md §check-gate-fail-closed — `shopt -s nullglob; gates+=("$d"/check-*.sh)`:
    // pathname expansion per dir, in dir order, with a non-matching pattern contributing nothing
    let mut gates: Vec<String> = Vec::new();
    for d in &dirs {
        gates.extend(walk::glob_entries(&format!(
            "{}/check-*.sh",
            d.trim_end_matches('/')
        )));
    }

    // spec: gate-sdk/SPEC.md §check-gate-fail-closed — the empty corpus is two different states and
    // the descriptor count over the same resolved dirs is what tells them apart; that section owns
    // why globbing both spellings keeps the refusal's original trigger rather than approximating it.
    if gates.is_empty() {
        let ported: usize = dirs
            .iter()
            .map(|d| walk::glob_entries(&format!("{}/check-*.gate", d.trim_end_matches('/'))).len())
            .sum();
        if ported == 0 {
            eprintln!(
                "check-gate-fail-closed: no check-*.sh and no check-*.gate found under: {}",
                dirs.join(" ")
            );
            eprintln!("  A tree carrying no gate declaration of either spelling has not finished a");
            eprintln!("  port — it resolved no gates directory. A gate that cannot run is not clean.");
            eprintln!("  help: check GATE_SDK_GATES_DIR and the kit roots it derives the rest from.");
            return 2;
        }
        println!(
            "GATE-FAIL-CLOSED: clean (0 shell gate(s) to scan, {} .gate-dispatched member(s) — the defect has no representation left in this corpus)",
            ported
        );
        return 0;
    }

    let mut findings: Vec<String> = Vec::new();
    for g in &gates {
        let text = match std::fs::read(g) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(_) => {
                eprintln!(
                    "check-gate-fail-closed: {}",
                    fresh::fail_closed("awk", Some(2))
                );
                return 2;
            }
        };
        scan_file(g, &text, &mut findings);
    }

    if !findings.is_empty() {
        println!("check-gate-fail-closed: gate(s) capture a parser's output but branch on");
        println!("emptiness without checking the subprocess status (gate-sdk/SPEC.md");
        println!("§Fail-closed contract — a crash would false-green as 'clean'):");
        println!();
        for f in &findings {
            println!("{}", f);
        }
        println!();
        println!("  help: capture the status and route it through the shared helper —");
        println!("        out=\"$(awk '…' \"$FILE\")\"; st=$?");
        println!("        fail_closed \"$st\" check-<name> awk");
        println!("  (source gate-sdk/lib/gate.sh near the top), or annotate a");
        println!("  genuinely-safe capture with '# fail-closed-exempt: <reason>'.");
        return 1;
    }

    println!("GATE-FAIL-CLOSED: clean (every awk/jq capture in the gate family handles subprocess status)");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_parser_boundary_rejects_a_longer_word_and_needs_trailing_space() {
        assert!(isparser("out=\"$(awk '{print}' f)\""));
        assert!(isparser("x=\"$(jq -r . f)\""));
        assert!(!isparser("gawk '{print}'"));
        assert!(!isparser("jqx -r ."));
        assert!(!isparser("awk"));
    }

    #[test]
    fn the_four_satisfying_shapes_are_recognised_and_nothing_else_is() {
        assert!(sat("fail_closed \"$st\" X awk"));
        assert!(sat("st=$?"));
        assert!(sat("cmd || { echo no; exit 2; }"));
        assert!(sat("# fail-closed-exempt: safe"));
        assert!(!sat("cmd || echo no"));
        assert!(!sat("[[ -n \"$out\" ]] && exit 1"));
    }

    // spec: gate-sdk/SPEC.md §check-gate-fail-closed — the arithmetic form is not a capture, and
    // the identifier atom may start anywhere inside a run the digits opened
    #[test]
    fn a_capture_opener_is_an_assignment_and_never_the_arithmetic_form() {
        assert!(opens_capture("out=\"$(awk '{print}' f)\""));
        assert!(opens_capture("  _x9=\"$(cmd)\""));
        assert!(opens_capture("9x=\"$(cmd)\""));
        assert!(!opens_capture("n=\"$((i + 1))\""));
        assert!(!opens_capture("=\"$(cmd)\""));
        assert!(!opens_capture("out='$(cmd)'"));
    }

    // spec: gate-sdk/SPEC.md §check-gate-fail-closed — a naked capture reds and the three
    // discharges clear it, over the multi-line block shape the state machine exists for
    #[test]
    fn a_naked_multiline_capture_reds_and_each_discharge_clears_it() {
        let naked = "set -u\nout=\"$(awk '\n  {print}\n' \"$F\")\"\n[[ -n \"$out\" ]] && exit 1\n";
        let mut hits = Vec::new();
        scan_file("g.sh", naked, &mut hits);
        assert_eq!(hits.len(), 1, "{:?}", hits);
        assert!(hits[0].starts_with("  g.sh:2: awk/jq capture branches"), "{}", hits[0]);

        for cleared in [
            "out=\"$(awk '{print}' \"$F\")\"; st=$?\n",
            "out=\"$(awk '{print}' \"$F\")\" || { exit 2; }\n",
            "# fail-closed-exempt: reason\nout=\"$(awk '{print}' \"$F\")\"\n",
            "out=\"$(awk '{print}' <<<\"$s\")\"\n",
            "out=\"$(sed -n 1p \"$F\")\"\n",
        ] {
            let mut h = Vec::new();
            scan_file("g.sh", cleared, &mut h);
            assert!(h.is_empty(), "{} -> {:?}", cleared, h);
        }
    }

    // spec: gate-sdk/SPEC.md §check-gate-fail-closed — the empty corpus is two states, and the
    // discriminator is exercised directly because no fixture pair can carry either: a committed
    // case cannot be a tree whose gates directory resolves to nothing
    #[test]
    fn an_empty_corpus_is_a_finished_port_or_a_misconfiguration_and_never_both() {
        let base = std::env::temp_dir().join(format!("checkwright-gfc.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let ported = base.join("ported");
        let empty = base.join("empty");
        std::fs::create_dir_all(&ported).expect("cannot make the scratch dir");
        std::fs::create_dir_all(&empty).expect("cannot make the scratch dir");
        std::fs::write(ported.join("check-alpha.gate"), "# graph: couples=x\n")
            .expect("cannot write the descriptor");

        assert_eq!(
            run(&[ported.display().to_string()]),
            0,
            "a dir carrying descriptors and no shell gate reds, so a finished port reads as a \
             failure and the port's own success is what breaks the battery"
        );
        assert_eq!(
            run(&[empty.display().to_string()]),
            2,
            "a dir carrying no declaration of either spelling passed, so the misconfiguration \
             the refusal exists for is no longer caught anywhere"
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    // spec: gate-sdk/SPEC.md §check-gate-fail-closed — the exemption marker is consumed by the
    // next capture alone: a blank line or an unrelated statement drops it
    #[test]
    fn a_pending_exemption_does_not_survive_a_blank_or_a_statement() {
        for text in [
            "# fail-closed-exempt: reason\n\nout=\"$(awk '{print}' f)\"\n",
            "# fail-closed-exempt: reason\necho hi\nout=\"$(awk '{print}' f)\"\n",
        ] {
            let mut h = Vec::new();
            scan_file("g.sh", text, &mut h);
            assert_eq!(h.len(), 1, "{} -> {:?}", text, h);
        }
    }
}
