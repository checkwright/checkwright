// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-deprecated-surface: live deprecation-marker lines over the comment surface canon-kit's check-deprecation-task scans
use super::{na, read, Ctx};
use crate::spec;

const LABEL: &str = "deprecated surface";
const MARKERS: &str = "CANON_KIT_DEPRECATION_MARKERS";

// spec: drift-kit/SPEC.md §Bundled KPIs — a marker line is one the joined alternation matches, the
// pattern check-deprecation-task compiles from the same roster
pub fn marker_lines(texts: &[String], markers: &[String]) -> Result<usize, String> {
    let re = spec::compile_pattern(&markers.join("|"), MARKERS)?;
    Ok(texts
        .iter()
        .map(|t| t.lines().filter(|l| re.find(l).is_some()).count())
        .sum())
}

pub fn run(_ctx: &Ctx, trend: bool) -> Option<String> {
    let markers = spec::knob_array_pub(MARKERS).ok()?;
    if markers.is_empty() {
        return na("lead", LABEL, "no CANON_KIT_DEPRECATION_MARKERS roster", trend);
    }
    let surface = spec::comment_surface(".", false).ok()?;
    let texts: Vec<String> = surface.iter().filter_map(|f| read(f)).collect();
    let count = marker_lines(&texts, &markers).ok()?;
    if trend {
        return Some(if count > 0 {
            format!("deprecated-surface {}\n", count)
        } else {
            String::new()
        });
    }
    Some(format!(
        "lead\t{}\t{} live marker(s) — decommission or re-justify at the next major\n",
        LABEL, count
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the count is lines, so two markers on one line are one
    // and a line no marker matches is none
    #[test]
    fn a_line_counts_once_however_many_markers_it_carries() {
        let texts = vec![
            "# DEPRECATED: a\nplain\n# LEGACY and DEPRECATED\n".to_string(),
            "# LEGACY: b\n".to_string(),
        ];
        let markers = vec!["DEPRECATED".to_string(), "LEGACY".to_string()];
        assert_eq!(marker_lines(&texts, &markers), Ok(3));
        assert_eq!(marker_lines(&[], &markers), Ok(0));
    }
}
