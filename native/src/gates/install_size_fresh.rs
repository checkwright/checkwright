// spec: installer/SPEC.md §The install size — the page's block carries the arm's rendering,
// compared in process; a positional emission stands in for the index's
use crate::emit::install_size;
use crate::exit::Fault;
use crate::fresh;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-size-fresh: {}", e);
            e.status()
        }
    }
}

fn rule(args: &[String]) -> Result<i32, Fault> {
    let page = fresh::positional_or_knob(args, 0, install_size::PAGE_KNOB).map_err(Fault::refused)?;
    let (want, source) = match args.get(1).filter(|a| !a.is_empty()) {
        Some(e) => {
            let text = fresh::read_captured(e).map_err(|err| Fault::reading(e, format!("{}: {}", e, err)))?;
            (format!("\n{}\n", text.trim_matches('\n')), e.to_string())
        }
        None => (install_size::render().map_err(Fault::input)?, "the index".to_string()),
    };
    let text = fresh::read_captured(&page).map_err(|e| Fault::reading(&page, format!("{}: {}", page, e)))?;
    let blk = install_size::block(&page, &text).map_err(Fault::input)?;
    if blk.actual != want {
        println!(
            "check-install-size-fresh: 1 stale block — {}:{} differs from the figure {} derives:",
            page, blk.line, source
        );
        fresh::print_capped_diff(&blk.actual, &want);
        println!("  help: regenerate — {}", super::door_command("--emit install-size --write").map_err(Fault::refused)?);
        return Ok(1);
    }
    println!("INSTALL-SIZE-FRESH: clean ({} carries the figure {} derives)", page, source);
    Ok(0)
}
