// spec: drift-kit/SPEC.md §The price-coverage arm — which running model ids the price table cannot
// price, and, where the consumer opts in, whether the pricing page's section moved since it was
// transcribed. Advisory: exit 0 on every reading, 2 on an operand or a configuration refusal.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` although its exit grammar is the
// `--emit-` collapse: its page half spawns a consumer's fetch, and that family is fence-safe.
use super::kpi::price_table_age::header_value;
use super::stage_economics::{usage_by_model, Prices, Tokens};
use crate::proc;
use crate::programs::{self, Program};
use crate::walk;

pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_PRICE_TABLE",
    "DRIFT_KIT_SESSIONS_DIR",
    "DRIFT_KIT_PRICE_COVERAGE_DAYS",
    "DRIFT_KIT_PRICE_PAGE_CMD",
    "DRIFT_KIT_PRICE_PAGE_SECTION",
    "DRIFT_KIT_PRICE_PAGE_TIMEOUT",
];

pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        eprintln!(
            "checkwright-gates: --price-coverage: takes no argument (got: {}) — usage: run-gates.sh --price-coverage",
            a
        );
        return 2;
    }
    match report() {
        Ok(out) => {
            print!("{}", out);
            0
        }
        Err(e) => {
            eprintln!("checkwright-gates: --price-coverage: {}", e);
            2
        }
    }
}

fn report() -> Result<String, String> {
    let table_path = walk::knob_scalar("DRIFT_KIT_PRICE_TABLE")?;
    let days: u64 = walk::knob_scalar("DRIFT_KIT_PRICE_COVERAGE_DAYS")?
        .parse()
        .map_err(|_| "DRIFT_KIT_PRICE_COVERAGE_DAYS is not a non-negative integer".to_string())?;
    let table = std::fs::read(&table_path)
        .ok()
        .map(|b| String::from_utf8_lossy(&b).into_owned());
    let prices = table.as_deref().map(Prices::parse);
    let var = |n: &str| std::env::var(n).unwrap_or_default();
    let pwd = var("PWD");
    let inputs = crate::sessions::Inputs {
        session_id: String::new(),
        harness_id: String::new(),
        child: String::new(),
        sessions_dir: walk::knob_scalar("DRIFT_KIT_SESSIONS_DIR")?,
        config_home: var("CLAUDE_CONFIG_DIR"),
        home: var("HOME"),
        here: if pwd.is_empty() { walk::cwd()? } else { pwd },
    };
    let dir = crate::sessions::sessions_dir(&inputs);
    let today = super::kpi::today_iso();

    let window = window(&inputs, days);
    let ids = used_ids(&window);
    let mut out = format!(
        "price-coverage: {} model id(s) with usage in {} transcript(s) of the last {}d\n",
        ids.len(),
        window.len(),
        days
    );
    if !std::path::Path::new(&dir).is_dir() {
        out.push_str(&format!("  no sessions dir {} — set DRIFT_KIT_SESSIONS_DIR\n", dir));
    } else {
        let unpriced: Vec<&str> = ids
            .iter()
            .filter(|id| prices.as_ref().map_or(true, |p| p.in_force(id, &today).is_none()))
            .map(String::as_str)
            .collect();
        if unpriced.is_empty() {
            out.push_str("  all priced\n");
        } else if prices.is_none() {
            out.push_str(&format!(
                "  unpriced: {} — no price table ({})\n",
                unpriced.join(", "),
                table_path
            ));
        } else {
            out.push_str(&format!(
                "  unpriced: {} — no row in force today in {}\n",
                unpriced.join(", "),
                table_path
            ));
        }
    }
    out.push_str(&format!("price-page: {}\n", page(table.as_deref().unwrap_or(""))?));
    Ok(out)
}

// spec: drift-kit/SPEC.md §The price-coverage arm — the shared two-tier population narrowed by
// modification time, `0` reading every transcript
fn window(inputs: &crate::sessions::Inputs, days: u64) -> Vec<String> {
    let cutoff = std::time::SystemTime::now()
        .checked_sub(std::time::Duration::from_secs(days.saturating_mul(86_400)));
    crate::sessions::every_transcript(inputs)
        .into_iter()
        .filter(|f| {
            days == 0
                || std::fs::metadata(f)
                    .and_then(|m| m.modified())
                    .is_ok_and(|when| cutoff.map_or(true, |c| when >= c))
        })
        .collect()
}

// spec: drift-kit/SPEC.md §The price-coverage arm — the meter's own reader, summed per id across
// the window; an id counts only where its summed counts are not all zero, in first-seen order
fn used_ids(transcripts: &[String]) -> Vec<String> {
    let mut order: Vec<String> = Vec::new();
    let mut sums: std::collections::HashMap<String, Tokens> = std::collections::HashMap::new();
    for f in transcripts {
        let Ok(b) = std::fs::read(f) else { continue };
        for (model, t) in usage_by_model(&String::from_utf8_lossy(&b)) {
            let s = sums.entry(model.clone()).or_insert_with(|| {
                order.push(model.clone());
                Tokens::default()
            });
            s.input += t.input;
            s.output += t.output;
            s.cache_read += t.cache_read;
            s.cache_write += t.cache_write;
        }
    }
    order.retain(|m| sums[m] != Tokens::default());
    order
}

// spec: drift-kit/SPEC.md §The price-coverage arm — the page half's one verdict, `off` where the
// command knob is empty; the arm reads the stored hash and never writes it
fn page(table: &str) -> Result<String, String> {
    let cmd = walk::knob_array("DRIFT_KIT_PRICE_PAGE_CMD")?;
    if cmd.is_empty() {
        return Ok("off".to_string());
    }
    let heading = walk::knob_scalar("DRIFT_KIT_PRICE_PAGE_SECTION")?;
    if heading.is_empty() {
        return Ok("n/a (no section heading configured)".to_string());
    }
    let secs: u64 = walk::knob_scalar("DRIFT_KIT_PRICE_PAGE_TIMEOUT")?
        .parse()
        .ok()
        .filter(|s| *s > 0)
        .ok_or_else(|| "DRIFT_KIT_PRICE_PAGE_TIMEOUT is not a positive integer".to_string())?;
    let program = Program::consumer("DRIFT_KIT_PRICE_PAGE_CMD", cmd[0].as_str());
    let rest: Vec<&str> = cmd[1..].iter().map(String::as_str).collect();
    let body = match proc::run_bounded_capture(&program, &rest, secs) {
        Err(e) => {
            let cause = e.split(" — ").next().unwrap_or(&e).to_string();
            return Ok(format!("n/a (fetch failed: {})", cause));
        }
        Ok(None) => return Ok(format!("n/a (fetch failed: timed out after {}s)", secs)),
        Ok(Some((code, _))) if code != 0 => return Ok(format!("n/a (fetch failed: exit {})", code)),
        Ok(Some((_, bytes))) => String::from_utf8_lossy(&bytes).into_owned(),
    };
    let Some(section) = section(&body, &heading) else {
        return Ok("n/a (section heading not found)".to_string());
    };
    let hash = hash_object(section.as_bytes())?;
    let as_of = header_value(table, "priced-as-of").unwrap_or_else(|| "?".to_string());
    Ok(match header_value(table, "price-page-hash") {
        None => format!("n/a (no price-page-hash: header) — record: # price-page-hash: {}", hash),
        Some(stored) if stored == hash => format!("unchanged since priced-as-of {}", as_of),
        Some(_) => format!(
            "CHANGED since priced-as-of {} — re-verify the rows, then record: # price-page-hash: {}",
            as_of, hash
        ),
    })
}

fn level(line: &str) -> usize {
    line.bytes().take_while(|b| *b == b'#').count()
}

// spec: drift-kit/SPEC.md §The price-coverage arm — the section: the first line equal to the
// heading, up to the line before the next heading with as many or fewer `#`; CRLF reads as LF
fn section(page: &str, heading: &str) -> Option<String> {
    let lines: Vec<&str> = page.split('\n').map(|l| l.strip_suffix('\r').unwrap_or(l)).collect();
    let start = lines.iter().position(|l| *l == heading)?;
    let top = level(heading);
    let mut out = String::new();
    for (i, l) in lines.iter().enumerate().skip(start) {
        let n = level(l);
        if i > start && n > 0 && n <= top.max(1) && l[n..].starts_with([' ', '\t']) {
            break;
        }
        if i + 1 == lines.len() && l.is_empty() {
            break;
        }
        out.push_str(l);
        out.push('\n');
    }
    Some(out)
}

fn hash_object(bytes: &[u8]) -> Result<String, String> {
    let done = proc::run_with_stdin(&programs::GIT, &["hash-object", "--stdin"], bytes)?;
    done.stdout()
        .map(|o| String::from_utf8_lossy(o).trim().to_string())
        .filter(|h| !h.is_empty())
        .ok_or_else(|| "git hash-object --stdin failed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §The price-coverage arm — the section runs to the next heading of the
    // same or a higher level, a deeper one staying inside, and CRLF hashes as LF
    #[test]
    fn the_section_stops_at_the_next_heading_of_its_level_or_higher() {
        let page = "# Top\n## Model pricing\n| m | 1 |\n### Detail\nx\n## Other\ny\n";
        assert_eq!(
            section(page, "## Model pricing").as_deref(),
            Some("## Model pricing\n| m | 1 |\n### Detail\nx\n")
        );
        let crlf = page.replace('\n', "\r\n");
        assert_eq!(section(&crlf, "## Model pricing"), section(page, "## Model pricing"));
        let last = "## Model pricing\n| m | 1 |\n# Next\n";
        assert_eq!(section(last, "## Model pricing").as_deref(), Some("## Model pricing\n| m | 1 |\n"));
        assert_eq!(section(page, "## Absent"), None);
        assert_eq!(
            section("## Model pricing\n#hashtag\n", "## Model pricing").as_deref(),
            Some("## Model pricing\n#hashtag\n"),
            "a `#` run with no space after it is no heading"
        );
    }

    // spec: drift-kit/SPEC.md §The price-coverage arm — an id counts only where its summed counts
    // are not all zero, in first-seen order across the window
    #[test]
    fn a_zero_usage_id_is_never_counted() {
        let dir = std::env::temp_dir().join(format!("price-coverage-test.{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let line = |id: &str, model: &str, n: u64| {
            format!(
                "{{\"type\":\"assistant\",\"message\":{{\"id\":\"{}\",\"model\":\"{}\",\
                 \"usage\":{{\"input_tokens\":{}}}}}}}\n",
                id, model, n
            )
        };
        let a = dir.join("a.jsonl");
        let b = dir.join("b.jsonl");
        std::fs::write(&a, format!("{}{}", line("1", "beta", 1), line("2", "<synthetic>", 0))).expect("a");
        std::fs::write(&b, format!("{}{}", line("3", "alpha", 2), line("4", "beta", 1))).expect("b");
        let ids = used_ids(&[a.display().to_string(), b.display().to_string()]);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(ids, vec!["beta".to_string(), "alpha".to_string()]);
    }
}
