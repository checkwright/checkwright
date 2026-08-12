// spec: gate-sdk/SPEC.md §check-assertion-strength — a guard over a call to a script whose
// declared `# exit:` header binds a verdict token to one non-zero code must not name that token
// in its failure message while comparing no status to that token's code
use crate::walk;
use std::path::Path;

const WINDOW: usize = 8;

fn is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

fn is_blank(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | 0x0b | 0x0c)
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — the `# exit:` line's own leader:
// `^#[[:space:]]*exit:`, with the header stripped before the token grammar reads it
fn exit_header_rest(line: &str) -> Option<&str> {
    let rest = line.strip_prefix('#')?;
    let rest = rest.trim_start_matches(|c: char| (c as u32) < 128 && is_blank(c as u8));
    rest.strip_prefix("exit:")
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — a verdict token: uppercase-led, alnum,
// hyphen-joined, at least two characters (`^[A-Z][A-Z0-9]*(-[A-Z0-9]+)*$`)
fn is_verdict_token(t: &str) -> bool {
    if t.len() < 2 {
        return false;
    }
    let mut segs = t.split('-');
    let Some(first) = segs.next() else {
        return false;
    };
    let fb = first.as_bytes();
    if fb.is_empty() || !fb[0].is_ascii_uppercase() {
        return false;
    }
    if !fb[1..]
        .iter()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    {
        return false;
    }
    segs.all(|s| {
        !s.is_empty()
            && s.bytes()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    })
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — the token->code grammar: each uppercase
// token binds to the nearest preceding integer on the line; a token bound to several codes or to
// code 0 is not discriminable and is dropped
pub fn exit_map(text: &str) -> Vec<(String, i64)> {
    let mut bound: Vec<(String, i64)> = Vec::new();
    let mut dup: Vec<String> = Vec::new();
    for line in text.lines() {
        let Some(rest) = exit_header_rest(line) else {
            continue;
        };
        let mut code: i64 = -1;
        for t in rest.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-')) {
            if t.is_empty() {
                continue;
            }
            if t.bytes().all(|b| b.is_ascii_digit()) {
                code = t.parse::<i64>().unwrap_or(0);
                continue;
            }
            if code < 0 || !is_verdict_token(t) {
                continue;
            }
            match bound.iter().find(|(k, _)| k == t) {
                Some((_, c)) if *c != code => {
                    if !dup.iter().any(|d| d == t) {
                        dup.push(t.to_string());
                    }
                }
                Some(_) => {}
                None => bound.push((t.to_string(), code)),
            }
        }
    }
    // spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — the shell form emits this
    // map in `awk`'s hash order and the port sorts it, the deliberate ordering difference
    let mut out: Vec<(String, i64)> = bound
        .into_iter()
        .filter(|(t, c)| *c != 0 && !dup.iter().any(|d| d == t))
        .collect();
    out.sort();
    out
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — `bin/([A-Za-z0-9._-]+\.sh)` at its leftmost
// match: the first `bin/` whose following name-run ends in `.sh`, taking the longest such run
fn bin_callee(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    let mut at = 0usize;
    while let Some(off) = line[at..].find("bin/") {
        let start = at + off + 4;
        let mut end = start;
        while end < b.len() && (is_word(b[end]) || b[end] == b'.') {
            end += 1;
        }
        let run = &line[start..end];
        let mut cut = run.len();
        while cut >= 4 {
            if run[..cut].ends_with(".sh") {
                return Some(&run[..cut]);
            }
            cut -= 1;
        }
        at = at + off + 4;
    }
    None
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — the matchers below search *bytes*, not
// `str` slices: a scanned script is decoded lossily, so an offset one past a previous hit need
// not be a character boundary and a `str` re-slice at one would panic rather than mismatch
fn find_from(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    (from..=hay.len() - needle.len()).find(|&i| &hay[i..i + needle.len()] == needle)
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — bash's `(^|[^A-Za-z0-9_-])TOK([^…]|$)`:
// an occurrence with no word character on either side
fn names_token(text: &str, tok: &str) -> bool {
    let b = text.as_bytes();
    let t = tok.as_bytes();
    let mut at = 0usize;
    while let Some(s) = find_from(b, t, at) {
        let e = s + t.len();
        if (s == 0 || !is_word(b[s - 1])) && (e == b.len() || !is_word(b[e])) {
            return true;
        }
        at = s + 1;
    }
    false
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — `(-eq|-ne|==|!=)[[:space:]]*\"?<code>\"?`:
// the guard compared a status to the code the message claims
fn compares_code(text: &str, code: i64) -> bool {
    let digits = code.to_string();
    let d = digits.as_bytes();
    let b = text.as_bytes();
    for op in ["-eq", "-ne", "==", "!="] {
        let o = op.as_bytes();
        let mut at = 0usize;
        while let Some(s) = find_from(b, o, at) {
            let mut i = s + o.len();
            while i < b.len() && is_blank(b[i]) {
                i += 1;
            }
            if i < b.len() && b[i] == b'"' {
                i += 1;
            }
            if b.len() >= i + d.len() && &b[i..i + d.len()] == d {
                return true;
            }
            at = s + 1;
        }
    }
    false
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — `(^|[^A-Za-z0-9_-])<code>\)`: a case arm
// discriminating that code
fn has_case_arm(text: &str, code: i64) -> bool {
    let pat = format!("{})", code);
    let p = pat.as_bytes();
    let b = text.as_bytes();
    let mut at = 0usize;
    while let Some(s) = find_from(b, p, at) {
        if s == 0 || !is_word(b[s - 1]) {
            return true;
        }
        at = s + 1;
    }
    false
}

fn is_guard_shape(line: &str) -> bool {
    if line.contains("||") || line.contains("&&") || line.contains("; then") {
        return true;
    }
    let t = line.trim_start_matches(|c: char| (c as u32) < 128 && is_blank(c as u8));
    match t.strip_prefix("if") {
        Some(rest) => rest
            .chars()
            .next()
            .map(|c| c.is_whitespace())
            .unwrap_or(false),
        None => false,
    }
}

fn is_comment_line(line: &str) -> bool {
    line.trim_start_matches(|c: char| (c as u32) < 128 && is_blank(c as u8))
        .starts_with('#')
}

struct Scan {
    declaring: usize,
    findings: Vec<String>,
}

fn scan_file(path: &str, text: &str, kitroot: &str, scan: &mut Scan) -> Result<(), String> {
    let l: Vec<&str> = text.lines().collect();
    let n = l.len();
    if n == 0 {
        return Ok(());
    }

    let mut inv: Vec<usize> = Vec::new();
    for (i, line) in l.iter().enumerate() {
        let Some(name) = bin_callee(line) else {
            continue;
        };
        if std::fs::metadata(format!("{}/bin/{}", kitroot, name)).is_err() {
            continue;
        }
        inv.push(i);
    }
    if inv.is_empty() {
        return Ok(());
    }

    for k in 0..inv.len() {
        let i = inv[k];
        let Some(name) = bin_callee(l[i]) else {
            continue;
        };
        let callee = format!("{}/bin/{}", kitroot, name);
        // spec: gate-sdk/SPEC.md §Fail-closed contract — the shell form read the callee through
        // `awk` under `fail_closed`, so a callee that exists and cannot be read is exit 2 rather
        // than a member silently dropped from the map
        let callee_text = match std::fs::read(&callee) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => {
                return Err(format!("cannot read {} ({})", callee, e));
            }
        };
        let map = exit_map(&callee_text);
        if map.is_empty() {
            continue;
        }
        scan.declaring += 1;

        if !is_guard_shape(l[i]) {
            continue;
        }

        let mut stop = i + WINDOW;
        if stop > n - 1 {
            stop = n - 1;
        }
        if k + 1 < inv.len() && inv[k + 1] - 1 < stop {
            stop = inv[k + 1] - 1;
        }

        let mut cbstart = i;
        while cbstart > 0 && is_comment_line(l[cbstart - 1]) {
            cbstart -= 1;
        }

        let mut gtext = String::new();
        if stop >= cbstart {
            for line in &l[cbstart..=stop] {
                gtext.push_str(line);
                gtext.push('\n');
            }
        }
        if gtext.contains("assertion-strength-exempt:") {
            continue;
        }

        let mut wtext = String::new();
        if stop >= i {
            for line in &l[i..=stop] {
                if line.contains("echo") || line.contains("printf") {
                    wtext.push_str(line);
                    wtext.push('\n');
                }
            }
        }
        if wtext.is_empty() {
            continue;
        }

        for (tok, code) in &map {
            if !names_token(&wtext, tok) {
                continue;
            }
            if compares_code(&gtext, *code) || has_case_arm(&gtext, *code) {
                continue;
            }
            scan.findings.push(format!(
                "{}:{}: message names {} (exit {} of {}) but the guard compares no status to {}",
                path,
                i + 1,
                tok,
                code,
                name,
                code
            ));
        }
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §check-assertion-strength — `cd "$d/.." && pwd`: the callee's kit root,
// logically resolved so a bridged relative dir and an absolute one land on the same place
fn parent_abs(d: &str) -> Result<String, String> {
    let here = std::env::current_dir()
        .map_err(|e| format!("cannot read the current directory: {}", e))?
        .display()
        .to_string();
    let joined = if d.starts_with('/') {
        format!("{}/..", d)
    } else {
        format!("{}/{}/..", here.trim_end_matches('/'), d)
    };
    Ok(walk::normalize_abs(&joined))
}

pub fn run(args: &[String]) -> i32 {
    let scan_dirs: Vec<String> = if !args.is_empty() {
        args.to_vec()
    } else {
        match walk::kit_roots_abs() {
            Ok(roots) => {
                let mut v = Vec::new();
                for k in roots {
                    let smoke = format!("{}/smoke", k);
                    if Path::new(&smoke).is_dir() {
                        v.push(smoke);
                    }
                    let tests = format!("{}/gate-tests", k);
                    if Path::new(&tests).is_dir() {
                        v.push(tests);
                    }
                }
                v
            }
            Err(e) => {
                eprintln!("check-assertion-strength: {}", e);
                return 2;
            }
        }
    };

    let mut scan = Scan {
        declaring: 0,
        findings: Vec::new(),
    };
    let mut scanned = 0usize;

    for d in &scan_dirs {
        if !Path::new(d).is_dir() {
            eprintln!("check-assertion-strength: not a directory: {}", d);
            return 2;
        }
        let kitroot = match parent_abs(d) {
            Ok(k) => k,
            Err(e) => {
                eprintln!(
                    "check-assertion-strength: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        };
        let files = match walk::glob_files(Path::new(d), &["*.sh".to_string()]) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "check-assertion-strength: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        };
        for f in &files {
            let path = f.display().to_string();
            let text = match std::fs::read(f) {
                Ok(b) => String::from_utf8_lossy(&b).into_owned(),
                Err(_) => {
                    eprintln!("check-assertion-strength: unreadable script: {}", path);
                    return 2;
                }
            };
            scanned += 1;
            if let Err(e) = scan_file(&path, &text, &kitroot, &mut scan) {
                eprintln!(
                    "check-assertion-strength: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        }
    }

    if !scan.findings.is_empty() {
        println!("check-assertion-strength: guard(s) whose failure message is more specific than the");
        println!("guard behind it — the message names a verdict token the callee's declared '# exit:'");
        println!("header binds to one non-zero code, but the guard discriminates only zero from");
        println!("non-zero, so a different failure mode reports itself under the wrong name");
        println!("(gate-sdk/SPEC.md §check-assertion-strength):");
        for m in &scan.findings {
            println!("  {}", m);
        }
        println!("  help: capture the status and compare it to the code the message claims —");
        println!("        cmd && rc=0 || rc=$?   then   if [[ \"$rc\" -ne <code> ]]; then …");
        println!("  (report the observed status in the message), OR reword the message to claim only");
        println!("  what truthiness establishes, OR add a '# assertion-strength-exempt: <reason>' line");
        println!("  on a guard that establishes the outcome by other means.");
        return 1;
    }

    println!(
        "ASSERTION-STRENGTH: clean ({} script(s) scanned; {} call(s) to a script with a declared exit contract)",
        scanned, scan.declaring
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_token_grammar_binds_to_the_nearest_preceding_integer_and_drops_zero_and_duplicates() {
        let h = "#!/usr/bin/env bash\n#   exit: 0 OK / RESET-OK, 1 PAUSE, 2 STALE or unreadable\n";
        assert_eq!(
            exit_map(h),
            vec![("PAUSE".to_string(), 1), ("STALE".to_string(), 2)]
        );
        assert!(exit_map("# exit: 1 PAUSE\n# exit: 2 PAUSE\n").is_empty());
        assert!(exit_map("# exit: OK before any code\n").is_empty());
        assert!(exit_map("# no header here\n").is_empty());
    }

    #[test]
    fn a_verdict_token_is_uppercase_led_hyphen_joined_and_at_least_two_characters() {
        assert!(is_verdict_token("OK"));
        assert!(is_verdict_token("RESET-OK"));
        assert!(is_verdict_token("A1"));
        assert!(!is_verdict_token("A"));
        assert!(!is_verdict_token("Ok"));
        assert!(!is_verdict_token("fail-closed"));
        assert!(!is_verdict_token("A-"));
    }

    #[test]
    fn the_callee_is_the_leftmost_bin_path_whose_name_run_ends_in_sh() {
        assert_eq!(bin_callee("bash \"$K/bin/verdict.sh\" x"), Some("verdict.sh"));
        assert_eq!(bin_callee("bin/a.sh.bak"), Some("a.sh"));
        assert_eq!(bin_callee("bin/notascript && bin/real.sh"), Some("real.sh"));
        assert_eq!(bin_callee("no callee here"), None);
    }

    #[test]
    fn a_named_token_needs_a_non_word_character_on_each_side() {
        assert!(names_token("did not PAUSE on a reading", "PAUSE"));
        assert!(names_token("(PAUSE)", "PAUSE"));
        assert!(!names_token("PAUSED", "PAUSE"));
        assert!(!names_token("REPAUSE", "PAUSE"));
    }

    #[test]
    fn a_code_comparison_is_an_operator_then_optional_space_and_quote_then_the_digits() {
        assert!(compares_code("if [[ \"$vrc\" -ne 1 ]]; then", 1));
        assert!(compares_code("[[ $rc == \"2\" ]]", 2));
        assert!(!compares_code("exit 1", 1));
        assert!(has_case_arm("case $rc in\n  1) echo ;;\nesac", 1));
        assert!(!has_case_arm("x1) echo", 1));
    }

    #[test]
    fn the_guard_shape_is_a_conditional_rather_than_a_bare_call() {
        assert!(is_guard_shape("cmd || fail"));
        assert!(is_guard_shape("cmd && rc=0 || rc=$?"));
        assert!(is_guard_shape("if bash x; then"));
        assert!(is_guard_shape("  if [[ -n $x ]]"));
        assert!(!is_guard_shape("bash x"));
        assert!(!is_guard_shape("iffy thing"));
    }
}
