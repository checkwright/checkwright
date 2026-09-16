// spec: SPEC-widget.md §The widget contract — the crate arm's pointer resolves
// spec: vendored-kit/SPEC.md §Anything — withheld, not dangling: the kit root resolves
// as a directory and only its SPEC file is absent, so this is counted and not a finding
pub fn widget() -> usize {
    1
}
