pub fn here() -> String {
    let d = std::env::current_dir().expect("cwd");
    d.display().to_string()
}

pub fn crate_dir() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}
