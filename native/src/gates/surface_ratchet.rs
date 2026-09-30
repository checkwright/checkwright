// spec: context-kit/SPEC.md §The surface ratchet — no governed surface above its committed
// ceiling. The gate never writes: `--ceiling` raises a row in the commit that grows the file.
use crate::emit::always_loaded;
use crate::walk;

const NAME: &str = "check-surface-ratchet";

pub fn run(_args: &[String]) -> i32 {
    let ceiling_file = match walk::knob_scalar("CONTEXT_KIT_CEILING_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };
    // spec: context-kit/SPEC.md §The surface ratchet — an absent ceiling file and an unparsable
    // row are both exit 2: a ratchet with no ceilings to read is a broken machine, not a clean
    // tree, and a row silently dropped leaves its surface silently ungoverned.
    let rows = match always_loaded::ceiling_rows(&ceiling_file) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            if let Some(door) = super::door_or_report(NAME, "--emit always-loaded --ceiling") {
                eprintln!("{}: help: write the ceilings with `{}` and commit them", NAME, door);
            }
            return 2;
        }
    };
    let governed = match always_loaded::governed() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };

    let mut findings: Vec<String> = Vec::new();
    for (file, size) in &governed {
        // spec: context-kit/SPEC.md §The surface ratchet — a row whose file is gone or is no
        // longer governed is ignored, so narrowing the governed set never reds; the match is
        // therefore from the governed side, never from the row side.
        match rows.iter().find(|(_, p)| p == file) {
            Some((ceiling, _)) if size <= ceiling => {}
            Some((ceiling, _)) => findings.push(format!(
                "{} — {}cp, above its committed ceiling of {}cp",
                file, size, ceiling
            )),
            None => findings.push(format!(
                "{} — {}cp, no ceiling row (a newly governed surface grows from nothing)",
                file, size
            )),
        }
    }

    if !findings.is_empty() {
        let Some(door) = super::door_or_report(NAME, "--emit always-loaded --ceiling") else {
            return 2;
        };
        println!(
            "SURFACE-RATCHET: {} governed surface(s) past their ceiling in {}:",
            findings.len(),
            ceiling_file
        );
        for f in &findings {
            println!("  {}", f);
        }
        println!(
            "  help: cut the surface back under its row, or — where the growth is deliberate — \
             re-stamp with `{}` and commit {} with the growth it prices",
            door, ceiling_file
        );
        return 1;
    }
    println!(
        "SURFACE-RATCHET: clean ({} governed file(s) at or below their ceiling)",
        governed.len()
    );
    0
}
