// spec: gate-sdk/SPEC.md §The diff renderer — the crate's one rendering of `diff`'s normal
// format: an LCS walk rather than a spawn, returning every hunk uncapped, carrying no
// consumer vocabulary and no report policy

// spec: gate-sdk/SPEC.md §The diff renderer — the freshness family's stale-report cap, held
// as the value its shell members each spelled inline. The renderer stays uncapped; the
// caller applies this.
pub(crate) const STALE_REPORT_CAP: usize = 20;

pub(crate) fn normal_diff(a: &[&str], b: &[&str]) -> Vec<String> {
    let (n, m) = (a.len(), b.len());
    let mut lcs = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lcs[i][j] = if a[i] == b[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    let range = |lo: usize, hi: usize| {
        if lo == hi {
            format!("{}", lo)
        } else {
            format!("{},{}", lo, hi)
        }
    };
    let mut out: Vec<String> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < n || j < m {
        if i < n && j < m && a[i] == b[j] {
            i += 1;
            j += 1;
            continue;
        }
        let (di, dj) = (i, j);
        while i < n && (j >= m || a[i] != b[j]) && (j >= m || lcs[i + 1][j] >= lcs[i][j + 1]) {
            i += 1;
        }
        while j < m && (i >= n || a[i] != b[j]) {
            j += 1;
        }
        let (dels, adds) = (i - di, j - dj);
        if dels > 0 && adds > 0 {
            out.push(format!(
                "{}c{}",
                range(di + 1, i),
                range(dj + 1, j)
            ));
        } else if dels > 0 {
            out.push(format!("{}d{}", range(di + 1, i), dj));
        } else {
            out.push(format!("{}a{}", di, range(dj + 1, j)));
        }
        for l in &a[di..i] {
            out.push(format!("< {}", l));
        }
        if dels > 0 && adds > 0 {
            out.push("---".to_string());
        }
        for l in &b[dj..j] {
            out.push(format!("> {}", l));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The diff renderer — the three hunk kinds in `diff`'s normal
    // format, held to the format rather than to any one consumer's live shape
    #[test]
    fn the_three_hunk_kinds_render_in_diff_normal_format() {
        assert_eq!(
            normal_diff(&["a", "b", "c"], &["a", "x", "c"]),
            vec!["2c2", "< b", "---", "> x"]
        );
        assert_eq!(normal_diff(&["a", "b"], &["a"]), vec!["2d1", "< b"]);
        assert_eq!(normal_diff(&["a"], &["a", "b"]), vec!["1a2", "> b"]);
        assert!(normal_diff(&["a", "b"], &["a", "b"]).is_empty());
    }

    #[test]
    fn a_multi_line_hunk_prints_a_range_on_each_side() {
        assert_eq!(
            normal_diff(&["a", "b", "c", "d"], &["a", "d"]),
            vec!["2,3d1", "< b", "< c"]
        );
        assert_eq!(
            normal_diff(&["a", "d"], &["a", "b", "c", "d"]),
            vec!["1a2,3", "> b", "> c"]
        );
    }
}
