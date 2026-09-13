// spec: gate-sdk/SPEC.md §The knob file — the line grammar a static kit's consumer file is written
// in; the shape a knob takes is its owner's declaration, so this module parses forms and the kit
// table in `knobs` refuses a form against a shape
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Form {
    Scalar,
    Indexed,
    Keyed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub lno: usize,
    pub name: String,
    pub form: Form,
    pub value: String,
}

fn blank(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn is_name(s: &str) -> bool {
    let mut chars = s.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_uppercase())
        && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

fn refuse(file: &str, lno: usize, what: &str) -> String {
    format!(
        "{}:{}: {} — write `NAME = value`, `NAME[] = element` or `NAME[key] = value` \
         (gate-sdk/SPEC.md §The knob file)",
        file, lno, what
    )
}

pub fn parse(text: &str, file: &str) -> Result<Vec<Entry>, String> {
    let mut out: Vec<Entry> = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let lno = idx + 1;
        let t = line.trim_matches(blank);
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let Some((head, value)) = t.split_once('=') else {
            return Err(refuse(file, lno, "a line with no `=`"));
        };
        let head = head.trim_matches(blank);
        let value = value.trim_matches(blank);
        if value.contains('\t') {
            return Err(refuse(file, lno, "a value carrying a tab"));
        }
        let (name, form) = match head.split_once('[') {
            None => (head, Form::Scalar),
            Some((name, rest)) => {
                let Some(key) = rest.strip_suffix(']') else {
                    return Err(refuse(file, lno, &format!("the head `{}` does not close its `[`", head)));
                };
                if key.is_empty() {
                    (name, Form::Indexed)
                } else if key.contains(']') || key.contains('\t') {
                    return Err(refuse(file, lno, &format!("the key `{}` carries a `]` or a tab", key)));
                } else {
                    (name, Form::Keyed(key.to_string()))
                }
            }
        };
        if !is_name(name) {
            return Err(refuse(file, lno, &format!("`{}` is not a SCREAMING_SNAKE knob name", name)));
        }
        out.push(Entry {
            lno,
            name: name.to_string(),
            form,
            value: value.to_string(),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(line: &str) -> Entry {
        parse(line, "f").expect("parses").remove(0)
    }

    #[test]
    fn the_three_forms_parse_with_blanks_trimmed_and_the_value_verbatim() {
        assert_eq!(one("  A_B = x y  ").form, Form::Scalar);
        assert_eq!(one("A_B = x y").value, "x y");
        assert_eq!(one("A[] = el").form, Form::Indexed);
        assert_eq!(one("A[k.v] = 1").form, Form::Keyed("k.v".to_string()));
        let hazards = one(r#"A[] = require "k"; $x # not a comment = still data 'q'"#);
        assert_eq!(hazards.value, r#"require "k"; $x # not a comment = still data 'q'"#);
        assert_eq!(one("A =").value, "");
    }

    #[test]
    fn blank_and_comment_lines_are_ignored() {
        assert!(parse("\n   \n# c\n   # indented\n", "f").expect("parses").is_empty());
    }

    #[test]
    fn every_malformed_line_is_refused_with_its_file_and_line() {
        for bad in ["just words", "a = lower", "A[ = x", "A[k]x = 1", "A = a\tb", "= v", "A[a]b] = 1"] {
            let e = parse(&format!("# ok\n{}\n", bad), "cfg.knobs").expect_err(bad);
            assert!(e.starts_with("cfg.knobs:2: "), "{:?} gave {}", bad, e);
        }
    }
}
