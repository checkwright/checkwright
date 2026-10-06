// spec: lifecycle-kit/SPEC.md §The consult inbox — the count arm: one line naming how many items
// the inbox owes the consult skill and the oldest one's date, and nothing for an empty inbox.

pub const KNOBS: &[&str] = &["LIFECYCLE_KIT_CONSULT_INBOX_FILE"];

// spec: lifecycle-kit/SPEC.md §The consult inbox — a bullet is a line opening `- `, and the oldest
// is the first, the inbox being append-only; its date is the ISO day the filing arm stamps there
pub fn line(inbox: &str) -> Option<String> {
    let bullets: Vec<&str> = inbox.lines().filter(|l| l.starts_with("- ")).collect();
    let first = bullets.first()?;
    let oldest = first
        .get(2..12)
        .filter(|d| super::kpi::is_iso_day(d))
        .unwrap_or("undated");
    Some(format!(
        "Consult inbox: {} item(s) owed to /consult, oldest {}.\n",
        bullets.len(),
        oldest
    ))
}

pub fn emit(_args: &[String]) -> Result<String, String> {
    let (inbox, _) = super::file_survey::anchored("LIFECYCLE_KIT_CONSULT_INBOX_FILE")?;
    let text = match std::fs::read(&inbox) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => return Ok(String::new()),
    };
    Ok(line(&text).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §The consult inbox — the count is the bullets and the date is the
    // first bullet's; the contract header and prose between bullets count nothing
    #[test]
    fn the_line_counts_bullets_and_dates_the_first() {
        let inbox = "# contract: the inbox\n- 2026-01-02 — one\nprose\n- 2026-03-04 — two\n";
        assert_eq!(
            line(inbox).as_deref(),
            Some("Consult inbox: 2 item(s) owed to /consult, oldest 2026-01-02.\n")
        );
    }

    // spec: lifecycle-kit/SPEC.md §The consult inbox — a first bullet carrying no date reads
    // `undated`, and an inbox with no bullet prints nothing
    #[test]
    fn an_undated_first_bullet_says_so_and_an_empty_inbox_says_nothing() {
        assert_eq!(
            line("- an item\n- 2026-03-04 — two\n").as_deref(),
            Some("Consult inbox: 2 item(s) owed to /consult, oldest undated.\n")
        );
        assert_eq!(line("# contract: the inbox\n"), None);
        assert_eq!(line(""), None);
    }
}
