// spec: companion/SPEC.md §The support table — the page's block carries the arm's rendering,
// compared in process
use crate::emit::support_table as st;
use crate::fresh;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-support-table-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let dir = fresh::positional_or_knob(args, 0, st::COMPANION_KNOB)?;
    let page = fresh::positional_or_knob(args, 1, st::PAGE_KNOB)?;
    let (dir, page) = (dir.as_str(), page.as_str());
    let want = st::render(dir)?;
    let text = fresh::read_captured(page).map_err(|e| format!("{}: {}", page, e))?;
    let blk = st::block(page, &text)?;
    if blk.actual != want {
        println!(
            "check-support-table-fresh: 1 stale block — {}:{} differs from the table {} derives:",
            page, blk.line, dir
        );
        fresh::print_capped_diff(&blk.actual, &want);
        println!("  help: regenerate — {}", super::door_command("--emit support-table --write")?);
        return Ok(1);
    }
    println!(
        "SUPPORT-TABLE-FRESH: clean ({} carries the table {} derives)",
        page, dir
    );
    Ok(0)
}
