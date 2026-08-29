// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-settings-local: entry count of the untracked local permission overlay
use super::{na, read, Ctx};
use serde_json::Value;

const LABEL: &str = "settings.local";

// spec: drift-kit/SPEC.md §Bundled KPIs — the shell member's `jq '[.permissions.allow // [],
// .deny // [], .ask // []] | add | length'`: the three lists summed, a missing or non-list arm
// counting nothing, and an unparseable document degrading rather than counting zero.
pub fn overlay_entries(text: &str) -> Option<usize> {
    let doc: Value = serde_json::from_str(text).ok()?;
    let perms = doc.get("permissions");
    Some(
        ["allow", "deny", "ask"]
            .iter()
            .filter_map(|k| perms.and_then(|p| p.get(k)).and_then(|v| v.as_array()))
            .map(|a| a.len())
            .sum(),
    )
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.settings_local) {
        Some(t) => t,
        None => return na("lead", LABEL, "no local overlay", trend),
    };
    let n = match overlay_entries(&text) {
        Some(n) => n,
        None => return na("lead", LABEL, "unreadable overlay", trend),
    };
    if trend {
        return Some(if n > 0 {
            format!("local {}\n", n)
        } else {
            String::new()
        });
    }
    Some(format!("lead\t{}\t{} local override(s)\n", LABEL, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the three arms sum and an absent one contributes
    // nothing, which is what `// []` bought in the shell form
    #[test]
    fn the_three_permission_arms_sum_and_an_absent_arm_counts_nothing() {
        assert_eq!(
            overlay_entries(r#"{"permissions":{"allow":["a","b"],"deny":["c"]}}"#),
            Some(3)
        );
        assert_eq!(overlay_entries(r#"{"permissions":{}}"#), Some(0));
        assert_eq!(overlay_entries("{}"), Some(0));
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — an unreadable overlay degrades in its own value
    // rather than counting zero, because zero is a claim the document does not support
    #[test]
    fn an_unparseable_overlay_is_not_a_count_of_zero() {
        assert_eq!(overlay_entries("{not json"), None);
        assert_eq!(overlay_entries(r#"{"permissions":{"allow":"a"}}"#), Some(0));
    }
}
