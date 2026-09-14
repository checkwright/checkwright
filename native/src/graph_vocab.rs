// spec: gate-sdk/SPEC.md §check-graph — the consumer's graph vocabulary, a document in the knob-file
// line grammar read by `check-graph` and `--emit graph` through this one reader
use crate::knobfile::{self, Form};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vocab {
    pub vocab: Vec<String>,
    pub leading: Vec<String>,
    pub lagging: Vec<String>,
    pub layers: Vec<String>,
    pub layer_rules: Vec<String>,
    pub layer_default: String,
}

const INDEXED: &[&str] = &["GRAPH_VOCAB", "GRAPH_LEADING", "GRAPH_LAGGING", "GRAPH_LAYERS", "GRAPH_LAYER_RULES"];
const LAYER_DEFAULT: &str = "GRAPH_LAYER_DEFAULT";

impl Vocab {
    fn empty() -> Vocab {
        Vocab {
            vocab: Vec::new(),
            leading: Vec::new(),
            lagging: Vec::new(),
            layers: Vec::new(),
            layer_rules: Vec::new(),
            layer_default: "surfaces".to_string(),
        }
    }

    fn slot(&mut self, name: &str) -> &mut Vec<String> {
        match name {
            "GRAPH_VOCAB" => &mut self.vocab,
            "GRAPH_LEADING" => &mut self.leading,
            "GRAPH_LAGGING" => &mut self.lagging,
            "GRAPH_LAYERS" => &mut self.layers,
            _ => &mut self.layer_rules,
        }
    }
}

// spec: gate-sdk/SPEC.md §check-graph — the path `GATE_SDK_GRAPH_VOCAB` resolves to, read with the
// legacy shell vocabulary beside it refused first
pub fn read(path: &str, gates_dir: &str) -> Result<Vocab, String> {
    let legacy = format!("{}/graph-vocab.sh", gates_dir.trim_end_matches('/'));
    if Path::new(&legacy).exists() {
        return Err(format!(
            "{} is a shell vocabulary, and the graph vocabulary is read from a knob-file document now — \
             rewrite it as {} (one `GRAPH_LAYER_RULES[] = <prefix>:<layer>` line per rule, one \
             `NAME[] = element` line per element, `GRAPH_LAYER_DEFAULT = <layer>`) and delete it \
             (gate-sdk/SPEC.md §check-graph)",
            legacy, path
        ));
    }
    if !Path::new(path).is_file() {
        return Ok(Vocab::empty());
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    parse(&text, path)
}

pub fn parse(text: &str, path: &str) -> Result<Vocab, String> {
    let mut out = Vocab::empty();
    let mut seen: Vec<String> = Vec::new();
    let mut emptied: Vec<String> = Vec::new();
    let at = |lno: usize, what: String| format!("{}:{}: {}", path, lno, what);
    for e in knobfile::parse(text, path)? {
        if e.name == LAYER_DEFAULT {
            if e.form != Form::Scalar {
                return Err(at(e.lno, format!("{} is a scalar — write `{} = <layer>`", e.name, e.name)));
            }
            if seen.contains(&e.name) {
                return Err(at(e.lno, format!("{} is given twice — keep one line for it", e.name)));
            }
            seen.push(e.name.clone());
            out.layer_default = e.value;
            continue;
        }
        if !INDEXED.contains(&e.name.as_str()) {
            return Err(at(
                e.lno,
                format!(
                    "{} is not a graph vocabulary name — the file declares {} and {}",
                    e.name,
                    INDEXED.join(", "),
                    LAYER_DEFAULT
                ),
            ));
        }
        match e.form {
            Form::Indexed if !emptied.contains(&e.name) => {
                seen.push(e.name.clone());
                out.slot(&e.name).push(e.value);
            }
            Form::Scalar if e.value.is_empty() && !seen.contains(&e.name) => {
                seen.push(e.name.clone());
                emptied.push(e.name.clone());
            }
            Form::Indexed | Form::Scalar if seen.contains(&e.name) => {
                return Err(at(e.lno, format!("{} is given twice — keep one line for it", e.name)));
            }
            _ => {
                return Err(at(
                    e.lno,
                    format!("{} is indexed — write one `{}[] = element` line per element", e.name, e.name),
                ))
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-graph — an absent file leaves every indexed name empty and the
    // default layer `surfaces`, which disables the checks each drives
    #[test]
    fn an_absent_vocabulary_is_empty_with_the_default_layer() {
        let dir = std::env::temp_dir().join(format!("cw-graph-vocab-absent-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch");
        let d = dir.display().to_string();
        assert_eq!(read(&format!("{}/graph-vocab.knobs", d), &d).expect("absent reads"), Vocab::empty());
        std::fs::write(dir.join("graph-vocab.sh"), "GRAPH_LAYERS=()\n").expect("legacy");
        assert!(read(&format!("{}/graph-vocab.knobs", d), &d).unwrap_err().contains("is a shell vocabulary"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_six_names_parse_in_file_order() {
        let v = parse(
            "GRAPH_LAYER_RULES[] = gate-sdk/:k_sdk\nGRAPH_LAYER_RULES[] = queue-kit/:k_queue\nGRAPH_LAYERS[] = k_sdk:gate-sdk\nGRAPH_LAYER_DEFAULT = k_shared\nGRAPH_VOCAB =\n",
            "v",
        )
        .expect("parses");
        assert_eq!(v.layer_rules, vec!["gate-sdk/:k_sdk", "queue-kit/:k_queue"]);
        assert_eq!(v.layers, vec!["k_sdk:gate-sdk"]);
        assert_eq!(v.layer_default, "k_shared");
        assert!(v.vocab.is_empty());
    }

    // spec: gate-sdk/SPEC.md §check-graph — each shape refusal and an undeclared name name the file and
    // line, and the file takes no reference
    #[test]
    fn every_refusal_names_its_file_and_line() {
        for (body, want) in [
            ("GRAPH_LAYERS = x\n", "is indexed"),
            ("GRAPH_LAYERS[k] = x\n", "is indexed"),
            ("GRAPH_LAYERS[] <- GRAPH_VOCAB\n", "is indexed"),
            ("GRAPH_LAYER_DEFAULT[] = x\n", "is a scalar"),
            ("GRAPH_LAYER_DEFAULT = a\nGRAPH_LAYER_DEFAULT = b\n", "given twice"),
            ("GRAPH_VOCAB =\nGRAPH_VOCAB[] = a\n", "given twice"),
            ("GRAPH_SOMETHING[] = x\n", "not a graph vocabulary name"),
        ] {
            let e = parse(&format!("# h\n{}", body), "vocab.knobs").expect_err(body);
            assert!(e.starts_with("vocab.knobs:"), "{:?}: {}", body, e);
            assert!(e.contains(want), "{:?}: {}", body, e);
        }
    }
}
