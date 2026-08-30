pub fn crate_dir() -> &'static str {
    // spec: gate-sdk/SPEC.md §The path-dialect contract — recorded verdict: this value is handed
    // to a directory-consuming API and never composed, the Rust counterpart of a cd-consumed root.
    env!("CARGO_MANIFEST_DIR")
}
