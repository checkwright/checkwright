// spec: docs/site-architecture.md §Generated projections and their freshness gates — every
// product-statement site carries the rendering the source gives it, compared in process
use crate::emit::product_statement as ps;
use crate::fresh;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-product-statement-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let source = fresh::positional(args, 0, ps::SOURCE);
    let sites: Vec<&str> = if args.len() > 1 {
        args[1..].iter().map(String::as_str).collect()
    } else {
        ps::SITES.to_vec()
    };
    let st = ps::parse(source, &fresh::read_captured(source).map_err(|e| format!("{}: {}", source, e))?)?;

    let mut findings: Vec<String> = Vec::new();
    for site in &sites {
        let text = fresh::read_captured(site).map_err(|e| format!("{}: {}", site, e))?;
        let (slot, want) = ps::check(site, &text, &st)?;
        if slot.actual != want {
            findings.push(format!(
                "  {}:{}: stale — carries {}, the source renders {}",
                site,
                slot.line,
                slot.actual.trim(),
                want.trim()
            ));
        }
    }
    if !findings.is_empty() {
        println!(
            "check-product-statement-fresh: {} site(s) differ from the product statement in {}:",
            findings.len(),
            source
        );
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: regenerate — bash gate-sdk/bin/run-gates.sh --emit product-statement --write");
        return Ok(1);
    }
    println!(
        "PRODUCT-STATEMENT-FRESH: clean ({} sites carry the product statement in {})",
        sites.len(),
        source
    );
    Ok(0)
}
