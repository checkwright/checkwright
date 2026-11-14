// spec: gate-sdk/SPEC.md §check-hook-knobs-fresh — every hook page's block carries the arm's
// rendering, compared in process, and every tree-free member has a page
use crate::emit::hook_knobs;
use crate::fresh;
use crate::hook;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-hook-knobs-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let dirs: Vec<String> = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => vec![a.clone()],
        None => crate::walk::knob_words(hook_knobs::PAGES_KNOB)?,
    };
    if dirs.is_empty() {
        println!("HOOK-KNOBS-FRESH: clean ({} is empty — no hook page is governed, 0 checked)", hook_knobs::PAGES_KNOB);
        return Ok(0);
    }
    let dir = dirs.iter().map(|d| fresh::strip_trailing_slash(d)).collect::<Vec<_>>().join(" ");
    let first = fresh::strip_trailing_slash(&dirs[0]).to_string();
    let pages = hook_knobs::pages(&dirs)?;
    let mut red = 0;
    let mut stale = false;
    for (page, member) in &pages {
        if !hook_knobs::is_member(member) {
            println!("check-hook-knobs-fresh: {} names no hook member", page);
            println!("  help: name the page <member>.md after a member `--hook` lists, or move it out of {}", dir);
            red += 1;
            continue;
        }
        let text = fresh::read_captured(page).map_err(|e| format!("{}: {}", page, e))?;
        let blk = match hook_knobs::block(page, &text) {
            Ok(b) => b,
            Err(e) => {
                println!("check-hook-knobs-fresh: {}", e);
                println!("  help: give the page one hook-knobs marker pair, then regenerate the block");
                red += 1;
                continue;
            }
        };
        let want = hook_knobs::render(member)?;
        if blk.actual != want {
            println!("check-hook-knobs-fresh: stale block — {}:{} differs from the table {}'s knobs derive:", page, blk.line, member);
            fresh::print_capped_diff(&blk.actual, &want);
            stale = true;
            red += 1;
        }
    }
    for member in hook::tree_free_members() {
        if !pages.iter().any(|(_, m)| m == member) {
            println!("check-hook-knobs-fresh: tree-free member {} has no page — {}/{}.md is owed", member, first, member);
            println!("  help: add the page with a hook-knobs marker pair, then regenerate the block");
            red += 1;
        }
    }
    if stale {
        println!("  help: regenerate — {}", super::door_command("--emit hook-knobs --write")?);
    }
    if red > 0 {
        return Ok(1);
    }
    println!(
        "HOOK-KNOBS-FRESH: clean ({} page(s) under {} carry the tables their members' knobs derive)",
        pages.len(),
        dir
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tree_free_member_with_no_page_reds_and_a_missing_dir_refuses() {
        let d = crate::proc::temp_base().join(format!("hook-knobs-fresh-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let rc = rule(&[d.display().to_string()]);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(rc, Ok(1));
        assert!(rule(&[d.display().to_string()]).is_err());
    }
}
