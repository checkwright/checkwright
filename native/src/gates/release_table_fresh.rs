// spec: installer/SPEC.md §The releases table — the page's block carries the arm's rendering,
// compared in process
use crate::emit::release_table;
use crate::fresh;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-release-table-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let posts = fresh::positional_or_knob(args, 0, release_table::POSTS_KNOB)?;
    let page = fresh::positional_or_knob(args, 1, release_table::PAGE_KNOB)?;
    let (posts, page) = (posts.as_str(), page.as_str());
    let want = release_table::render(posts, page)?;
    let text = fresh::read_captured(page).map_err(|e| format!("{}: {}", page, e))?;
    let blk = release_table::block(page, &text)?;
    if blk.actual != want {
        println!(
            "check-release-table-fresh: 1 stale block — {}:{} differs from the table {} derives:",
            page, blk.line, posts
        );
        fresh::print_capped_diff(&blk.actual, &want);
        println!("  help: regenerate — {}", super::door_command("--emit release-table --write")?);
        return Ok(1);
    }
    println!("RELEASE-TABLE-FRESH: clean ({} carries the table {} derives)", page, posts);
    Ok(0)
}
