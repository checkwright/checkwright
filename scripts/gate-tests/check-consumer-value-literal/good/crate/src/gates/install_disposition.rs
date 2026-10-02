use crate::knobs::demo;

// consumer-value-exempt: a valved tracked path
const PATH: &str = "docs/guide.md";
const MARKED: &str = "## Done"; // consumer-value-exempt: a valved marked heading
// consumer-value-exempt: a valved bare multi-word heading
const BARE: &str = "Release notes index";
const KIT_OWN: &str = "demo-kit/README.md";
const ONE_WORD: &str = "Overview";
const FENCED: &str = "Fenced heading text";

#[cfg(test)]
mod tests {
    const IN_TESTS: &str = "docs/guide.md";
}
