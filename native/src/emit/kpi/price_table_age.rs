// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-price-table-age: age of the price table's priced-as-of: header and time to its optional prices-valid-through: expiry
use super::{date_epoch, is_iso_day, now_epoch, read, Ctx};

// spec: drift-kit/SPEC.md §Bundled KPIs — `awk '^#[[:space:]]*<field>:'` then the value up to the
// first whitespace: a commented header field, read positionally so a trailing note is not the value.
pub fn header_value(text: &str, field: &str) -> Option<String> {
    let want = format!("{}:", field);
    for line in text.lines() {
        let rest = match line.strip_prefix('#') {
            Some(r) => r.trim_start_matches([' ', '\t']),
            None => continue,
        };
        if let Some(v) = rest.strip_prefix(&want) {
            let v = v.trim_start_matches([' ', '\t']);
            let end = v.find([' ', '\t']).unwrap_or(v.len());
            return Some(v[..end].to_string());
        }
    }
    None
}

fn day_epoch(v: &str) -> Option<i64> {
    if !is_iso_day(v) {
        return None;
    }
    date_epoch(v)
}

fn age_row(out: &mut String, trend: bool, value: String) {
    if !trend {
        out.push_str(&format!("lead\tprice table age\t{}\n", value));
    }
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let mut out = String::new();

    let text = match read(&ctx.price_table) {
        Some(t) => t,
        None => {
            age_row(&mut out, trend, "n/a (no price table)".to_string());
            return Some(out);
        }
    };

    let now = now_epoch();
    match header_value(&text, "priced-as-of") {
        None => age_row(&mut out, trend, "n/a (no priced-as-of: header)".to_string()),
        Some(p) => match day_epoch(&p) {
            None => age_row(
                &mut out,
                trend,
                "n/a (unparseable priced-as-of date)".to_string(),
            ),
            Some(ps) => {
                let days = ((now - ps) / 86400).max(0);
                if trend {
                    return Some(format!("price {}d\n", days));
                }
                age_row(&mut out, trend, format!("priced {}d ago (as-of {})", days, p));
            }
        },
    }
    if trend {
        return Some(out);
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — the expiry is measured from *today's* local midnight
    // rather than from now, so "expires in 0d" means today rather than some hours from now.
    let today = date_epoch(&super::today_iso()).unwrap_or(now);
    let expiry = match header_value(&text, "prices-valid-through") {
        None => "n/a (no prices-valid-through: header)".to_string(),
        Some(t) => match day_epoch(&t) {
            None => "n/a (unparseable prices-valid-through date)".to_string(),
            Some(ts) => {
                let left = (ts - today) / 86400;
                if left >= 0 {
                    format!("expires in {}d (through {})", left, t)
                } else {
                    format!("EXPIRED {}d ago — re-verify (through {})", -left, t)
                }
            }
        },
    };
    out.push_str(&format!("lead\tprice table expiry\t{}\n", expiry));
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the header is a commented field and its value ends
    // at the first whitespace, so a trailing note on the same line is not part of the date
    #[test]
    fn a_header_value_is_the_token_after_the_field_and_stops_at_whitespace() {
        let t = "#  priced-as-of: 2026-08-01  (checked by hand)\n# prices-valid-through:2026-08-31\nmodel\t1\n";
        assert_eq!(header_value(t, "priced-as-of").as_deref(), Some("2026-08-01"));
        assert_eq!(
            header_value(t, "prices-valid-through").as_deref(),
            Some("2026-08-31")
        );
        assert_eq!(header_value(t, "absent-field"), None);
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — an uncommented line carrying the same text is table
    // data, not a header, which is what the `#` anchor buys
    #[test]
    fn an_uncommented_line_is_table_data_and_never_a_header() {
        assert_eq!(header_value("priced-as-of: 2026-08-01\n", "priced-as-of"), None);
    }
}
