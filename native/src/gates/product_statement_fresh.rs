// spec: docs/generated-projections.md §Generated projections and their freshness gates — every
// product-statement site carries the rendering the source gives it, compared in process
use crate::exit::Fault;
use crate::emit::product_statement as ps;
use crate::fresh;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-product-statement-fresh: {}", e);
            e.status()
        }
    }
}

fn rule(args: &[String]) -> Result<i32, Fault> {
    let source = fresh::positional_or_knob(args, 0, ps::SOURCE_KNOB).map_err(Fault::refused)?;
    let source = source.as_str();
    let sites: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        crate::walk::knob_array(ps::SITES_KNOB).map_err(Fault::refused)?
    };
    let st = ps::parse(source, &fresh::read_captured(source).map_err(|e| Fault::reading(source, format!("{}: {}", source, e)))?).map_err(Fault::input)?;

    let mut findings: Vec<String> = Vec::new();
    for site in &sites {
        let text = fresh::read_captured(site).map_err(|e| Fault::reading(site, format!("{}: {}", site, e)))?;
        for (slot, want) in ps::check(site, &text, &st).map_err(Fault::input)? {
            if slot.actual != want {
                findings.push(format!(
                    "  {}:{}: stale {} — carries {}, the source renders {}",
                    site,
                    slot.line,
                    slot.field,
                    slot.actual.trim(),
                    want.trim()
                ));
            }
        }
    }
    if !findings.is_empty() {
        println!(
            "check-product-statement-fresh: {} slot(s) differ from the product statement in {}:",
            findings.len(),
            source
        );
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: regenerate — {}", super::door_command("--emit product-statement --write").map_err(Fault::refused)?);
        return Ok(1);
    }
    println!(
        "PRODUCT-STATEMENT-FRESH: clean ({} sites carry the product statement in {})",
        sites.len(),
        source
    );
    Ok(0)
}
