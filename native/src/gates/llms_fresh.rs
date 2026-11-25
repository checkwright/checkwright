// spec: docs/agent-index.md §The arm and its gate — both agent
// index files equal the arm's rendering byte for byte, compared in process
use crate::exit::Fault;
use crate::emit::llms;
use crate::emit::product_statement as ps;
use crate::fresh;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-llms-fresh: {}", e);
            e.status()
        }
    }
}

fn rule(args: &[String]) -> Result<i32, Fault> {
    let root = fresh::strip_trailing_slash(&fresh::positional_or_knob(args, 0, llms::ROOT_KNOB).map_err(Fault::refused)?).to_string();
    if !fresh::is_dir(&root) {
        return Err(Fault::refused(format!("not a directory: {}", root)));
    }
    let source = crate::walk::knob_scalar(ps::SOURCE_KNOB).map_err(Fault::refused)?;
    let r = llms::render(&root, &source).map_err(Fault::input)?;
    let (index, full) = llms::paths(&root);
    let mut stale: Vec<&str> = Vec::new();
    for (path, want) in [(index.as_str(), &r.index), (full.as_str(), &r.full)] {
        let have = fresh::read_captured(path).map_err(|e| Fault::input(format!("{}: {}", path, e)))?;
        if &have != want {
            stale.push(path);
        }
    }
    if !stale.is_empty() {
        println!("check-llms-fresh: {} agent index file(s) differ from their rendering:", stale.len());
        for path in &stale {
            println!("  {}: stale — differs from the rendering of the nav and the product statement", path);
        }
        println!("  help: regenerate — {}", super::door_command("--emit llms --write").map_err(Fault::refused)?);
        return Ok(1);
    }
    println!("LLMS-FRESH: clean ({} and {} match their rendering)", index, full);
    Ok(0)
}
