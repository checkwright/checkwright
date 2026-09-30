// spec: guard-kit/SPEC.md §check-door-binding — a comment naming bash gate-sdk/bin/run-gates.sh --emit graph is no output
pub fn help() {
    // door-contributor: a contributor's regenerator, run from this repository's own checkout
    println!("  help: regenerate — bash gate-sdk/bin/run-gates.sh --emit graph");
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_test_literal_is_no_output() {
        assert!("bash gate-sdk/bin/run-gates.sh --emit graph".contains("--emit"));
    }
}
