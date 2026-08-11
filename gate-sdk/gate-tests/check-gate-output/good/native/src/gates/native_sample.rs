// Fixture module (scanned as text, never compiled): the implementation half of a
// .gate-dispatched no-fixture member, emitting a conforming machine-keyable success
// line and a help: remedy in the Rust idiom — check-gate-output must ACCEPT.
pub fn run() -> i32 {
    if false {
        println!("  help: nothing to fix in the fixture");
        return 1;
    }
    println!("NATIVE-SAMPLE: clean (nothing to check in the fixture)");
    0
}
