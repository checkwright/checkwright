// spec: docs/agent-index.md §The arm and its gate — both agent
// index files equal the arm's rendering byte for byte, compared in process
use crate::emit::llms;
use crate::emit::product_statement as ps;
use crate::fresh;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-llms-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = fresh::strip_trailing_slash(&fresh::positional_or_knob(args, 0, llms::ROOT_KNOB)?).to_string();
    if !fresh::is_dir(&root) {
        return Err(format!("not a directory: {}", root));
    }
    let source = crate::walk::knob_scalar(ps::SOURCE_KNOB)?;
    let r = llms::render(&root, &source)?;
    let (index, full) = llms::paths(&root);
    let mut stale: Vec<&str> = Vec::new();
    for (path, want) in [(index.as_str(), &r.index), (full.as_str(), &r.full)] {
        let have = fresh::read_captured(path).map_err(|e| format!("{}: {}", path, e))?;
        if &have != want {
            stale.push(path);
        }
    }
    if !stale.is_empty() {
        println!("check-llms-fresh: {} agent index file(s) differ from their rendering:", stale.len());
        for path in &stale {
            println!("  {}: stale — differs from the rendering of the nav and the product statement", path);
        }
        println!("  help: regenerate — {}", super::door_command("--emit llms --write")?);
        return Ok(1);
    }
    println!("LLMS-FRESH: clean ({} and {} match their rendering)", index, full);
    Ok(0)
}
