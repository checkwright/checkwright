// spec: queue-kit/SPEC.md §check-queue-hygiene — the queue holds only tasks, tags, and
// structure: no HTML comments, no duplicate lines, no column-0 prose, no line-number cites,
// no -S pickaxe on the queue
use crate::queue;
use std::collections::HashMap;

fn is_rule(line: &str) -> bool {
    match line.strip_prefix("---") {
        Some(rest) => rest.bytes().all(|b| b == b' ' || b == b'\t'),
        None => false,
    }
}

fn is_blank(line: &str) -> bool {
    line.bytes().all(|b| b == b' ' || b == b'\t')
}

fn is_path_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'/' || b == b'-'
}

// spec: queue-kit/SPEC.md §check-queue-hygiene — a <path>:<n> or <path>:<n>-<m> token, whose
// path ends in a dot-extension, is a stale-prone line-number citation
fn ends_with_dot_extension(token: &str) -> bool {
    match token.rfind('.') {
        Some(dot) => {
            let ext = &token[dot + 1..];
            !ext.is_empty() && ext.len() <= 4 && ext.bytes().all(|b| b.is_ascii_lowercase())
        }
        None => false,
    }
}

fn line_number_cites(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut hits = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if is_path_char(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_path_char(bytes[i]) {
                i += 1;
            }
            let path = &line[start..i];
            if ends_with_dot_extension(path) && i < bytes.len() && bytes[i] == b':' {
                let num_start = i + 1;
                let mut j = num_start;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                if j > num_start {
                    let mut end = j;
                    if j < bytes.len() && bytes[j] == b'-' {
                        let num2_start = j + 1;
                        let mut k = num2_start;
                        while k < bytes.len() && bytes[k].is_ascii_digit() {
                            k += 1;
                        }
                        if k > num2_start {
                            end = k;
                        }
                    }
                    hits.push(line[start..end].to_string());
                    i = end;
                    continue;
                }
            }
        } else {
            i += 1;
        }
    }
    hits
}

fn code_spans(line: &str) -> Vec<&str> {
    let bytes = line.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'`' {
            if let Some(rel_end) = line[i + 1..].find('`') {
                spans.push(&line[i + 1..i + 1 + rel_end]);
                i = i + 1 + rel_end + 1;
                continue;
            }
            break;
        }
        i += 1;
    }
    spans
}

// spec: queue-kit/SPEC.md §check-queue-hygiene — a `git log` span with `-S` and the queue
// file as its path is red (§The icebox tier's recovery is `-G`); queue_names carries the
// basename and the literal `<queue-file>`
fn dash_s_pickaxes<'a>(line: &'a str, queue_names: &[&str]) -> Vec<&'a str> {
    let mut hits = Vec::new();
    for span in code_spans(line) {
        if !span.contains("git log") {
            continue;
        }
        let has_dash_s = span
            .as_bytes()
            .windows(2)
            .enumerate()
            .any(|(idx, w)| {
                w == b"-S"
                    && (idx == 0 || !span.as_bytes()[idx - 1].is_ascii_alphanumeric())
            });
        if !has_dash_s {
            continue;
        }
        if queue_names.iter().any(|n| span.contains(n)) {
            hits.push(span);
        }
    }
    hits
}

pub fn run(args: &[String]) -> i32 {
    let leads = match queue::knob_array("QUEUE_KIT_PROSE_LEADS") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-hygiene: {}", e);
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-queue-hygiene: {}", e);
                return 2;
            }
        },
    };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-queue-hygiene: file not found: {}", file);
            return 2;
        }
    };

    let basename = std::path::Path::new(&file)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| file.clone());
    let queue_names: Vec<&str> = vec![basename.as_str(), "<queue-file>"];

    let mut html: Vec<String> = Vec::new();
    let mut prose: Vec<String> = Vec::new();
    let mut dup: Vec<String> = Vec::new();
    let mut linecite: Vec<String> = Vec::new();
    let mut pickaxe: Vec<String> = Vec::new();
    let mut seen: HashMap<&str, usize> = HashMap::new();

    for (i, line) in text.lines().enumerate() {
        let fnr = i + 1;
        if line.contains("<!--") || line.contains("-->") {
            html.push(format!("{}:{}: {}", file, fnr, line));
        }

        for cite in line_number_cites(line) {
            linecite.push(format!("{}:{}: {}", file, fnr, cite));
        }
        for span in dash_s_pickaxes(line, &queue_names) {
            pickaxe.push(format!("{}:{}: `{}`", file, fnr, span));
        }

        // spec: queue-kit/SPEC.md §check-queue-hygiene — every column-0 line must be a
        // heading, a '- ' bullet, '---', or a configured QUEUE_KIT_PROSE_LEADS lead
        let col0 = match line.bytes().next() {
            Some(b) => b != b' ' && b != b'\t',
            None => false,
        };
        if col0 {
            let ok = line.starts_with('#')
                || matches!(line.as_bytes().get(1), Some(&c) if line.starts_with('-') && (c == b' ' || c == b'\t'))
                || is_rule(line)
                || leads.iter().any(|l| !l.is_empty() && line.starts_with(l.as_str()));
            if !ok {
                prose.push(format!("{}:{}: {}", file, fnr, line));
            }
        }

        if !is_blank(line) && !is_rule(line) {
            match seen.get(line) {
                Some(first) => dup.push(format!(
                    "{}:{}: {} (first seen at line {})",
                    file, fnr, line, first
                )),
                None => {
                    seen.insert(line, fnr);
                }
            }
        }
    }

    if !html.is_empty()
        || !prose.is_empty()
        || !dup.is_empty()
        || !linecite.is_empty()
        || !pickaxe.is_empty()
    {
        if !html.is_empty() {
            println!("check-queue-hygiene: HTML comment(s) in the queue (provenance belongs in git history):");
            for x in &html {
                println!("  {}", x);
            }
            println!("  help: delete the comment; record the why in the commit message, not the queue.");
        }
        if !dup.is_empty() {
            if !html.is_empty() {
                println!();
            }
            println!("check-queue-hygiene: exact-duplicate line(s) (copy-paste artifact):");
            for x in &dup {
                println!("  {}", x);
            }
            println!("  help: remove the duplicate; if two tasks genuinely share wording, differentiate them.");
        }
        if !prose.is_empty() {
            if !html.is_empty() || !dup.is_empty() {
                println!();
            }
            println!("check-queue-hygiene: column-0 prose (every column-0 line must be a heading,");
            println!("a '- ' bullet, '---', or a configured QUEUE_KIT_PROSE_LEADS lead):");
            for x in &prose {
                println!("  {}", x);
            }
            println!("  help: indent the prose to a continuation line under its bullet, or (for a");
            println!("        recurring protocol lead) add its token to QUEUE_KIT_PROSE_LEADS.");
        }
        if !linecite.is_empty() {
            if !html.is_empty() || !dup.is_empty() || !prose.is_empty() {
                println!();
            }
            println!("check-queue-hygiene: line-number citation(s) (goes stale on any edit above it):");
            for x in &linecite {
                println!("  {}", x);
            }
            println!("  help: cite by a §Section, a symbol beside its path, or a quoted literal —");
            println!("        never by line number.");
        }
        if !pickaxe.is_empty() {
            if !html.is_empty() || !dup.is_empty() || !prose.is_empty() || !linecite.is_empty() {
                println!();
            }
            println!("check-queue-hygiene: -S pickaxe over the queue file (blind to an eviction that");
            println!("keeps the slug):");
            for x in &pickaxe {
                println!("  {}", x);
            }
            println!("  help: use -G (queue-kit/SPEC.md §The icebox tier); -S stays correct over other files.");
        }
        return 1;
    }

    println!(
        "QUEUE-HYGIENE: clean (no HTML comments, no duplicate lines, no column-0 prose, no \
         line-number cites, no -S pickaxe on the queue in {})",
        file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_horizontal_rule_is_exempt_from_both_prose_and_duplicate_arms() {
        assert!(is_rule("---"));
        assert!(is_rule("---   "));
        assert!(!is_rule("--- x"));
        assert!(!is_rule("--"));
    }

    #[test]
    fn blankness_is_whitespace_only_not_merely_empty() {
        assert!(is_blank(""));
        assert!(is_blank("  \t "));
        assert!(!is_blank(" x"));
    }

    #[test]
    fn a_line_number_cite_needs_a_dot_extension_path_and_reads_the_dash_range() {
        assert_eq!(
            line_number_cites("see native/src/gates/foo.rs:12-34 for the loop"),
            vec!["native/src/gates/foo.rs:12-34".to_string()]
        );
        assert_eq!(
            line_number_cites("see native/src/gates/foo.rs:12 for the loop"),
            vec!["native/src/gates/foo.rs:12".to_string()]
        );
        assert!(line_number_cites("prose says lines 642 and 669, not a cite").is_empty());
        assert!(line_number_cites("see [blocked-by: beta-feature] for the dep").is_empty());
        assert!(line_number_cites("`README.md`:16-17 has a backtick between path and colon")
            .is_empty());
    }

    #[test]
    fn a_dash_s_pickaxe_reds_only_within_a_code_span_naming_the_queue_file() {
        let names = ["TASK-QUEUE.md", "<queue-file>"];
        assert_eq!(
            dash_s_pickaxes(
                "recovery is `git log -p -S'<slug>' -- TASK-QUEUE.md`.",
                &names
            )
            .len(),
            1
        );
        assert_eq!(
            dash_s_pickaxes(
                "recovery is `git log -p -S'<slug>' -- <queue-file>`.",
                &names
            )
            .len(),
            1
        );
        assert!(dash_s_pickaxes(
            "recoverable via `git log -p -S'a phrase' -- .workflow/gap-inbox.md`.",
            &names
        )
        .is_empty());
        assert!(dash_s_pickaxes("no code span at all, just git log -S mentioned", &names)
            .is_empty());
    }
}
