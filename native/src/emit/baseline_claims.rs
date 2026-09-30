// spec: evidence-kit/SPEC.md §The baseline-claims arm — each baseline row as a `measured:` oracle
// line, `baseline-<suite>--<scenario>`⇥`<status>`, in the file's order
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Emit`: the contract is a document and every
// failure is exit 2; it declares evidence-kit's baseline knob alone, so a canon-kit command knob
// naming it cannot recurse
use crate::evidence;
use crate::walk;

pub const KNOBS: &[&str] = &["EVIDENCE_KIT_BASELINE_FILE"];

pub fn emit(_args: &[String]) -> Result<String, String> {
    let path = walk::knob_scalar("EVIDENCE_KIT_BASELINE_FILE")?;
    let text = super::read_text(&path)?;
    lines(&text)
}

// spec: evidence-kit/SPEC.md §The baseline-claims arm — each part lowercased, every character
// outside `[a-z0-9]` a hyphen, so the key is always slug-shaped
fn part(s: &str) -> String {
    s.chars()
        .map(|c| {
            let c = c.to_ascii_lowercase();
            if c.is_ascii_lowercase() || c.is_ascii_digit() {
                c
            } else {
                '-'
            }
        })
        .collect()
}

fn lines(text: &str) -> Result<String, String> {
    let mut seen: Vec<(String, String)> = Vec::new();
    let mut out = String::new();
    for row in evidence::data_lines(text) {
        let f: Vec<&str> = row.split_whitespace().collect();
        if f.len() < 3 {
            return Err(format!(
                "baseline row has fewer than three fields (<suite> <scenario> <status>): {}",
                row.trim()
            ));
        }
        let key = format!("baseline-{}--{}", part(f[0]), part(f[1]));
        if let Some((_, first)) = seen.iter().find(|(k, _)| *k == key) {
            return Err(format!(
                "two baseline rows map to the key {}: '{}' and '{}'",
                key,
                first,
                row.trim()
            ));
        }
        out.push_str(&format!("{}\t{}\n", key, f[2]));
        seen.push((key, row.trim().to_string()));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_maps_to_a_slug_key_and_its_status_alone() {
        let text = "# contract: header\n\ninstaller_smoke build pass\ngates check-graph fail some-slug reproduces-at=abc\n  # comment\n";
        assert_eq!(
            lines(text).unwrap(),
            "baseline-installer-smoke--build\tpass\nbaseline-gates--check-graph\tfail\n"
        );
    }

    #[test]
    fn a_colliding_pair_and_a_short_row_refuse() {
        let err = lines("a_b x pass\na-b x fail\n").unwrap_err();
        assert!(err.contains("baseline-a-b--x") && err.contains("a_b x pass") && err.contains("a-b x fail"), "{}", err);
        assert!(lines("gates check-graph\n").unwrap_err().contains("fewer than three fields"));
    }
}
