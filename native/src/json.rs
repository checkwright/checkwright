// spec: context-kit/SPEC.md §check-settings-pins — the pin-path layer: a compiled path
// expression, its evaluation over a parsed document, and the one equality rule the
// dependency's object model does not give for free
use serde_json::Value;

// spec: context-kit/SPEC.md §check-settings-pins — a refusal names the construct, because the
// gate turns it into an exit-2 message naming pin, knob and construct, and a refusal whose text
// does not name what it refused sends its reader looking through the whole line
#[derive(Debug, PartialEq)]
pub struct PathError {
    pub construct: String,
}

impl PathError {
    fn at(construct: &str) -> Self {
        PathError {
            construct: construct.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Step {
    Field(String),
    Index(i64),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    steps: Vec<Step>,
}

// spec: context-kit/SPEC.md §check-settings-pins — jq's own indexing type rules, preserved
// rather than repaired: a field step on a scalar or array, and an index step on an object, are
// errors the gate classifies as a malformed pin exactly as the shell's non-zero jq status does
#[derive(Debug, PartialEq)]
pub struct EvalError {
    pub message: String,
}

impl Path {
    // spec: context-kit/SPEC.md §check-settings-pins — the accepted grammar, and the refusal
    // of everything outside it
    pub fn compile(src: &str) -> Result<Path, PathError> {
        let b: Vec<char> = src.chars().collect();
        if b.is_empty() {
            return Err(PathError::at("empty path"));
        }
        if src == "." {
            return Ok(Path { steps: Vec::new() });
        }
        // spec: context-kit/SPEC.md §check-settings-pins — a path opens with `.`, the shell
        // form's own `[[ "$path" != .* ]]` pre-check preserved; that section owns why a
        // leading `[` cannot be admitted
        if b[0] != '.' {
            return Err(PathError::at(
                "a pin path opens with '.'; a leading '[' is a jq array literal, not an index step",
            ));
        }
        let mut steps = Vec::new();
        let mut i = 0usize;
        while i < b.len() {
            match b[i] {
                '.' => {
                    i += 1;
                    if i < b.len() && b[i] == '"' {
                        let (s, next) = quoted(&b, i)?;
                        steps.push(Step::Field(s));
                        i = next;
                        continue;
                    }
                    let start = i;
                    if i >= b.len() || !(b[i].is_ascii_alphabetic() || b[i] == '_') {
                        return Err(PathError::at(&format!(
                            "'.' not followed by an identifier or a quoted key at offset {}",
                            start
                        )));
                    }
                    while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == '_') {
                        i += 1;
                    }
                    steps.push(Step::Field(b[start..i].iter().collect()));
                }
                '[' => {
                    i += 1;
                    if i < b.len() && b[i] == '"' {
                        let (s, next) = quoted(&b, i)?;
                        i = next;
                        if i >= b.len() || b[i] != ']' {
                            return Err(PathError::at("unterminated '[\"…\"]' key step"));
                        }
                        i += 1;
                        steps.push(Step::Field(s));
                        continue;
                    }
                    let start = i;
                    if i < b.len() && b[i] == '-' {
                        i += 1;
                    }
                    let digits = i;
                    while i < b.len() && b[i].is_ascii_digit() {
                        i += 1;
                    }
                    if i == digits {
                        return Err(PathError::at(&format!(
                            "'[' is not followed by an integer or a quoted key at offset {}",
                            start
                        )));
                    }
                    let raw: String = b[start..i].iter().collect();
                    let n: i64 = raw
                        .parse()
                        .map_err(|_| PathError::at(&format!("index out of range: {}", raw)))?;
                    if i >= b.len() || b[i] != ']' {
                        return Err(PathError::at("unterminated '[…]' index step"));
                    }
                    i += 1;
                    steps.push(Step::Index(n));
                }
                c => {
                    return Err(PathError::at(&format!(
                        "'{}' at offset {} — the pins grammar is a path expression, not a jq filter",
                        c, i
                    )));
                }
            }
        }
        Ok(Path { steps })
    }

    // spec: context-kit/SPEC.md §check-settings-pins — evaluation on jq's terms: a field step on
    // null yields null, an out-of-range index yields null, and the type errors above are errors
    pub fn eval(&self, doc: &Value) -> Result<Value, EvalError> {
        let mut cur = doc.clone();
        for step in &self.steps {
            cur = match (step, &cur) {
                (_, Value::Null) => Value::Null,
                (Step::Field(k), Value::Object(m)) => m.get(k).cloned().unwrap_or(Value::Null),
                (Step::Index(n), Value::Array(a)) => {
                    let len = a.len() as i64;
                    let idx = if *n < 0 { len + *n } else { *n };
                    if idx < 0 || idx >= len {
                        Value::Null
                    } else {
                        a[idx as usize].clone()
                    }
                }
                (Step::Field(k), other) => {
                    return Err(EvalError {
                        message: format!(
                            "cannot index {} with \"{}\"",
                            type_name(other),
                            k
                        ),
                    })
                }
                (Step::Index(n), other) => {
                    return Err(EvalError {
                        message: format!("cannot index {} with number {}", type_name(other), n),
                    })
                }
            };
        }
        Ok(cur)
    }
}

fn quoted(b: &[char], at: usize) -> Result<(String, usize), PathError> {
    let mut i = at + 1;
    let mut out = String::new();
    while i < b.len() {
        match b[i] {
            '"' => return Ok((out, i + 1)),
            '\\' => {
                i += 1;
                if i >= b.len() {
                    return Err(PathError::at("trailing '\\' in a quoted key"));
                }
                match b[i] {
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    '/' => out.push('/'),
                    c => {
                        return Err(PathError::at(&format!(
                            "unsupported escape '\\{}' in a quoted key",
                            c
                        )))
                    }
                }
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    Err(PathError::at("unterminated quoted key"))
}

fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

// spec: context-kit/SPEC.md §check-settings-pins — numbers compare by their f64 value wherever
// they occur, every other shape by Value's own equality
pub fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => match (x.as_f64(), y.as_f64()) {
            (Some(p), Some(q)) => p == q,
            _ => x == y,
        },
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(p, q)| values_equal(p, q))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).map(|w| values_equal(v, w)).unwrap_or(false))
        }
        _ => a == b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grammar_accepts_exactly_its_four_step_forms_and_identity() {
        assert!(Path::compile(".").is_ok());
        assert!(Path::compile(".a").is_ok());
        assert!(Path::compile(".a.b_2").is_ok());
        assert!(Path::compile(".\"a.b\"").is_ok());
        assert!(Path::compile(".a[\"k\"]").is_ok());
        assert!(Path::compile(".a[0]").is_ok());
        assert!(Path::compile(".a[-1]").is_ok());
    }

    // spec: context-kit/SPEC.md §check-settings-pins — everything outside the grammar is refused
    // loudly, which is what makes the narrowing a guard rather than a silent mis-scan
    #[test]
    fn every_out_of_subset_construct_is_refused_by_name() {
        for src in [
            ".a | .b",
            ".a?",
            ".[]",
            ".a[1:2]",
            "map(.x)",
            ".a + 1",
            "",
            ".1a",
            ".a[",
            ".\"unterminated",
            "[\"k\"]",
            "[0]",
        ] {
            let e = Path::compile(src);
            assert!(e.is_err(), "{} should be refused", src);
            assert!(
                !e.unwrap_err().construct.is_empty(),
                "{}'s refusal must name the construct",
                src
            );
        }
    }

    // spec: context-kit/SPEC.md §check-settings-pins — jq's indexing type rules, preserved
    #[test]
    fn indexing_follows_jqs_own_type_rules() {
        let doc: Value = serde_json::from_str(r#"{"a":{"b":1},"s":"x","arr":[1,2,3]}"#).unwrap();
        assert_eq!(Path::compile(".a.b").unwrap().eval(&doc).unwrap(), Value::from(1));
        assert_eq!(Path::compile(".nope").unwrap().eval(&doc).unwrap(), Value::Null);
        assert_eq!(
            Path::compile(".nope.deeper").unwrap().eval(&doc).unwrap(),
            Value::Null
        );
        assert_eq!(
            Path::compile(".arr[-1]").unwrap().eval(&doc).unwrap(),
            Value::from(3)
        );
        assert_eq!(
            Path::compile(".arr[9]").unwrap().eval(&doc).unwrap(),
            Value::Null
        );
        assert!(Path::compile(".s.k").unwrap().eval(&doc).is_err());
        assert!(Path::compile(".arr.k").unwrap().eval(&doc).is_err());
        assert!(Path::compile(".a[0]").unwrap().eval(&doc).is_err());
    }

    #[test]
    fn numbers_compare_by_their_f64_value_at_every_depth() {
        let one: Value = serde_json::from_str("1").unwrap();
        let one_point_oh: Value = serde_json::from_str("1.0").unwrap();
        assert_ne!(one, one_point_oh, "the premise: Value::eq separates them");
        assert!(values_equal(&one, &one_point_oh));
        let a: Value = serde_json::from_str(r#"{"k":[1,{"n":2}]}"#).unwrap();
        let b: Value = serde_json::from_str(r#"{"k":[1.0,{"n":2.0}]}"#).unwrap();
        assert!(values_equal(&a, &b));
        let c: Value = serde_json::from_str(r#"{"k":[1,{"n":3}]}"#).unwrap();
        assert!(!values_equal(&a, &c));
    }
}
