use std::path::{Path, PathBuf};

// Path-typed: the value never becomes a string, so std::path carries the dialect.
pub fn manifest_is_dir() -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR")).is_dir()
}

pub fn manifest_buf() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
