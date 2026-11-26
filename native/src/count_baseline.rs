// spec: gate-sdk/SPEC.md §The count baseline — `<count><TAB><path>` rows, one per path, read by a
// ceiling gate and an exact-count gate alike
use crate::exit::{self, Fault};
use std::collections::BTreeMap;

pub type Counts = BTreeMap<String, usize>;

// spec: gate-sdk/SPEC.md §The count baseline — an absent configured baseline refuses and an
// unreadable or unparseable one is unparseable input
pub fn read(path: &str) -> Result<Counts, Fault> {
    parse(path, &exit::read_configured(path, "count baseline")?)
}

pub fn parse(path: &str, text: &str) -> Result<Counts, Fault> {
    let mut rows = Counts::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let row = line
            .split_once('\t')
            .filter(|(count, file)| !count.is_empty() && count.bytes().all(|c| c.is_ascii_digit()) && !file.is_empty())
            .and_then(|(count, file)| count.parse::<usize>().ok().map(|c| (c, file)));
        let Some((count, file)) = row else {
            return Err(Fault::input(format!("{}:{}: not a `<count><TAB><path>` row", path, n + 1)));
        };
        let file = file.strip_prefix("./").unwrap_or(file);
        if rows.insert(file.to_string(), count).is_some() {
            return Err(Fault::input(format!("{}:{}: {} is given a second row", path, n + 1, file)));
        }
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The count baseline — comments and blanks skipped; a malformed row and
    // a repeated path are unparseable, each naming its line
    #[test]
    fn rows_parse_and_malformed_ones_name_their_line() {
        let rows = parse("b", "# reviewed\n\n2\ta.sh\n0\t./lib/x.sh\n").expect("parses");
        assert_eq!(rows.get("a.sh"), Some(&2));
        assert_eq!(rows.get("lib/x.sh"), Some(&0));
        for (text, line) in [("2 a.sh\n", 1), ("\n-1\ta\n", 2), ("1\ta\n2\ta\n", 2), ("x\ta\n", 1), ("3\t\n", 1)] {
            let fault = parse("b", text).expect_err(text);
            assert_eq!(fault.status(), exit::INPUT, "{:?}", text);
            assert!(fault.message.starts_with(&format!("b:{}:", line)), "{:?}: {}", text, fault.message);
        }
    }
}
