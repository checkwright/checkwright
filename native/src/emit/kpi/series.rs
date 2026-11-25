// spec: drift-kit/SPEC.md §The external series form — a data member: read, validated whole, and
// rendered in the plugin row grammar so the collator parses one grammar for every member.
use super::{civil_days, read, today_iso};

const FIELDS: &[&str] = &["label", "section", "source", "unit", "trend"];
const REQUIRED: &[&str] = &["label", "section", "source"];

struct Point {
    date: String,
    days: i64,
    value: String,
}

struct Series {
    fields: Vec<(String, String)>,
    points: Vec<Point>,
}

impl Series {
    fn field(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.as_str())
            .filter(|v| !v.is_empty())
    }
}

pub fn run(path: &str, name: &str, trend: bool) -> Option<String> {
    let text = read(path)?;
    Some(render(&text, name, civil_days(&today_iso()), trend))
}

fn render(text: &str, name: &str, today: Option<i64>, trend: bool) -> String {
    let s = match parse(text) {
        Ok(s) => s,
        Err(_) if trend => return String::new(),
        Err((line, defect)) => {
            return format!("lead\t{}\tn/a (series line {}: {})\n", name, line, defect)
        }
    };
    let last = s.points.last().expect("parse owes at least one point");
    if trend {
        return match s.field("trend") {
            Some(key) => format!("{} {}\n", key, last.value),
            None => String::new(),
        };
    }
    let unit = s.field("unit").map(|u| format!(" {}", u)).unwrap_or_default();
    let slope = match s.points.len().checked_sub(2).map(|i| &s.points[i]) {
        Some(prev) => format!("{} since {}; ", delta(&last.value, &prev.value), prev.date),
        None => String::new(),
    };
    let age = today.map_or_else(|| "?".to_string(), |t| (t - last.days).max(0).to_string());
    format!(
        "{}\t{}\t{}{} ({}external: {}, as of {}, {}d old)\n",
        s.field("section").unwrap_or("lead"),
        s.field("label").unwrap_or(name),
        last.value,
        unit,
        slope,
        s.field("source").unwrap_or(""),
        last.date,
        age
    )
}

fn header(line: &str) -> Option<(&str, &str)> {
    let (name, value) = line.split_once(':')?;
    let mut c = name.chars();
    let lead = c.next()?.is_ascii_alphabetic();
    (lead && c.all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'))
        .then(|| (name, value.trim_matches([' ', '\t'])))
}

fn is_number(v: &str) -> bool {
    let v = v.strip_prefix('-').unwrap_or(v);
    let (int, frac) = match v.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (v, None),
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    digits(int) && frac.map_or(true, digits)
}

fn parse(text: &str) -> Result<Series, (usize, String)> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut s = Series { fields: Vec::new(), points: Vec::new() };
    let mut first_point: Option<usize> = None;
    for (i, line) in text.lines().enumerate() {
        let n = i + 1;
        let bare = line.trim_matches([' ', '\t']);
        if bare.is_empty() || bare.starts_with('#') {
            continue;
        }
        if let Some((name, value)) = header(line) {
            if first_point.is_some() {
                return Err((n, "header after a point".to_string()));
            }
            if !FIELDS.contains(&name) {
                return Err((n, format!("unknown field {}", name)));
            }
            if s.fields.iter().any(|(f, _)| f == name) {
                return Err((n, format!("repeated field {}", name)));
            }
            let value = value.replace('\t', " ");
            if REQUIRED.contains(&name) && value.is_empty() {
                return Err((n, format!("required field {} missing or empty", name)));
            }
            if name == "section" && value != "lead" && value != "lag" {
                return Err((n, format!("section {} is neither lead nor lag", value)));
            }
            if name == "trend" && (value.is_empty() || !value.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')) {
                return Err((n, format!("trend key {:?} is not letters, digits and -", value)));
            }
            s.fields.push((name.to_string(), value));
            continue;
        }
        let point = line
            .split_once('\t')
            .filter(|(_, v)| is_number(v))
            .and_then(|(d, v)| civil_days(d).map(|days| (d, days, v)));
        let (date, days, value) = match point {
            Some(p) => p,
            None => return Err((n, "not a date, a tab and a value".to_string())),
        };
        if let Some(prev) = s.points.last() {
            if days <= prev.days {
                return Err((n, format!("date {} not after {}", date, prev.date)));
            }
        }
        first_point.get_or_insert(n);
        s.points.push(Point { date: date.to_string(), days, value: value.to_string() });
    }
    let due = first_point.unwrap_or(text.lines().count() + 1);
    if let Some(f) = REQUIRED.iter().find(|f| s.field(f).is_none()) {
        return Err((due, format!("required field {} missing or empty", f)));
    }
    if s.points.is_empty() {
        return Err((due, "no point".to_string()));
    }
    Ok(s)
}

// spec: drift-kit/SPEC.md §The external series form — exact decimal arithmetic over digit strings,
// so no value the grammar admits is rounded or overflows
fn magnitude(v: &str, scale: usize) -> (bool, Vec<u8>) {
    let neg = v.starts_with('-');
    let v = v.trim_start_matches('-');
    let (int, frac) = v.split_once('.').unwrap_or((v, ""));
    let mut digits: Vec<u8> = int.bytes().chain(frac.bytes()).map(|b| b - b'0').collect();
    digits.resize(digits.len() + scale - frac.len(), 0);
    digits.reverse();
    while digits.len() > 1 && digits.last() == Some(&0) {
        digits.pop();
    }
    let zero = digits.iter().all(|d| *d == 0);
    (neg && !zero, digits)
}

fn cmp_mag(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

fn add_mag(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut carry = 0;
    for i in 0..a.len().max(b.len()) {
        let s = a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0) + carry;
        out.push(s % 10);
        carry = s / 10;
    }
    if carry > 0 {
        out.push(carry);
    }
    out
}

fn sub_mag(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut borrow = 0;
    for (i, &d) in a.iter().enumerate() {
        let take = b.get(i).unwrap_or(&0) + borrow;
        borrow = u8::from(d < take);
        out.push(d + 10 * borrow - take);
    }
    while out.len() > 1 && out.last() == Some(&0) {
        out.pop();
    }
    out
}

fn fraction_len(v: &str) -> usize {
    v.split_once('.').map_or(0, |(_, f)| f.len())
}

fn delta(latest: &str, previous: &str) -> String {
    let scale = fraction_len(latest).max(fraction_len(previous));
    let (na, a) = magnitude(latest, scale);
    let (nb, b) = magnitude(previous, scale);
    let (neg, mut m) = if na != nb {
        (na, add_mag(&a, &b))
    } else if cmp_mag(&a, &b).is_ge() {
        (na, sub_mag(&a, &b))
    } else {
        (!na, sub_mag(&b, &a))
    };
    m.resize(m.len().max(scale + 1), 0);
    let neg = neg && m.iter().any(|d| *d != 0);
    let text: String = m.iter().rev().map(|d| char::from(b'0' + d)).collect();
    let (int, frac) = text.split_at(text.len() - scale);
    let sign = if neg { '-' } else { '+' };
    if scale == 0 {
        format!("{}{}", sign, int)
    } else {
        format!("{}{}.{}", sign, int, frac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = "label: velocity\nsection: lead\nsource: sprint-board export\nunit: pts\ntrend: vel\n2026-11-01\t21\n2026-11-15\t18.5\n";

    fn day(d: &str) -> Option<i64> {
        civil_days(d)
    }

    // spec: drift-kit/SPEC.md §The external series form — the row: latest, unit, delta at the wider
    // precision, the provenance mark and the age in whole civil days
    #[test]
    fn a_valid_series_renders_its_latest_point_its_delta_and_its_provenance() {
        assert_eq!(
            render(VALID, "kpi-velocity", day("2026-11-25"), false),
            "lead\tvelocity\t18.5 pts (-2.5 since 2026-11-01; external: sprint-board export, as of 2026-11-15, 10d old)\n"
        );
        assert_eq!(render(VALID, "kpi-velocity", day("2026-11-25"), true), "vel 18.5\n");
    }

    #[test]
    fn a_one_point_series_has_no_delta_and_an_unkeyed_series_no_trend() {
        let t = "label: lessons\nsection: lag\nsource: retro log\n2026-11-20\t4\n";
        assert_eq!(
            render(t, "kpi-lessons", day("2026-11-20"), false),
            "lag\tlessons\t4 (external: retro log, as of 2026-11-20, 0d old)\n"
        );
        assert_eq!(render(t, "kpi-lessons", day("2026-11-20"), true), "");
    }

    #[test]
    fn a_latest_date_after_today_reads_zero_days_old() {
        let t = "label: x\nsection: lead\nsource: s\n2026-12-01\t1\n";
        assert!(render(t, "kpi-x", day("2026-11-25"), false).ends_with("as of 2026-12-01, 0d old)\n"));
    }

    // spec: drift-kit/SPEC.md §The external series form — comments, blank lines, CRLF ends and a
    // leading byte-order mark are not the grammar's business
    #[test]
    fn comments_blank_lines_crlf_and_a_bom_are_tolerated() {
        let t = "\u{feff}# exported nightly\r\nlabel: q age\r\n\r\nsection: lag\r\nsource: tracker\r\n  # note\r\n2026-11-01\t3\r\n";
        assert_eq!(
            render(t, "kpi-q", day("2026-11-02"), false),
            "lag\tq age\t3 (external: tracker, as of 2026-11-01, 1d old)\n"
        );
    }

    fn defect(t: &str) -> String {
        render(t, "kpi-bad", day("2026-11-25"), false)
    }

    // spec: drift-kit/SPEC.md §The external series form — every defect class refuses the file whole,
    // in the lead section under the member's name, at the line it names
    #[test]
    fn each_defect_class_refuses_the_series_whole_at_its_line() {
        let h = "label: v\nsection: lead\nsource: s\n";
        let cases: &[(String, &str)] = &[
            ("section: lead\nsource: s\n2026-11-01\t1\n".into(), "3: required field label missing or empty"),
            ("label:\nsection: lead\nsource: s\n2026-11-01\t1\n".into(), "1: required field label missing or empty"),
            (format!("{}colour: red\n2026-11-01\t1\n", h), "4: unknown field colour"),
            (format!("{}label: w\n2026-11-01\t1\n", h), "4: repeated field label"),
            ("label: v\nsection: middle\n".into(), "2: section middle is neither lead nor lag"),
            (format!("{}trend: v el\n2026-11-01\t1\n", h), "4: trend key \"v el\" is not letters, digits and -"),
            (format!("{}2026-11-01\t1\nunit: pts\n", h), "5: header after a point"),
            (format!("{}2026-11-01 1\n", h), "4: not a date, a tab and a value"),
            (format!("{}2026-02-31\t1\n", h), "4: not a date, a tab and a value"),
            (format!("{}2026-11-01\t1e3\n", h), "4: not a date, a tab and a value"),
            (format!("{}2026-11-01\t1.\n", h), "4: not a date, a tab and a value"),
            (format!("{}2026-11-01\t1\t\n", h), "4: not a date, a tab and a value"),
            (format!("{}2026-11-02\t1\n2026-11-02\t2\n", h), "5: date 2026-11-02 not after 2026-11-02"),
            (format!("{}2026-11-02\t1\n2026-11-01\t2\n", h), "5: date 2026-11-01 not after 2026-11-02"),
            (h.to_string(), "4: no point"),
            (String::new(), "1: required field label missing or empty"),
        ];
        for (text, want) in cases {
            assert_eq!(defect(text), format!("lead\tkpi-bad\tn/a (series line {})\n", want), "{:?}", text);
            assert_eq!(render(text, "kpi-bad", day("2026-11-25"), true), "", "{:?}", text);
        }
    }

    // spec: drift-kit/SPEC.md §The external series form — the delta carries the wider fraction
    // precision of its operands, a sign always, and never a negative zero
    #[test]
    fn the_delta_is_exact_at_the_wider_precision() {
        for (latest, previous, want) in [
            ("18.5", "21", "-2.5"),
            ("21", "18.5", "+2.5"),
            ("3", "3", "+0"),
            ("3.00", "3", "+0.00"),
            ("-1.25", "0.5", "-1.75"),
            ("0.5", "-1.25", "+1.75"),
            ("-2", "-5", "+3"),
            ("-5", "-2", "-3"),
            ("0.1", "0.2", "-0.1"),
            ("100000000000000000000000000000000000000001", "1", "+100000000000000000000000000000000000000000"),
            ("-0", "0", "+0"),
        ] {
            assert_eq!(delta(latest, previous), want, "{} - {}", latest, previous);
        }
    }
}
