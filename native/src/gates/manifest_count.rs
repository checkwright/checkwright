// spec: canon-kit/SPEC.md §check-manifest-count — no bare cardinal quantifying a governed
// collection noun in manifest prose outside an exempt site
use crate::spec;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-manifest-count: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }

    let manifests = spec::manifest_files_sorted_stripped(root)?;
    if manifests.is_empty() {
        println!("MANIFEST-COUNT: clean (0 manifest file(s) found)");
        return Ok(0);
    }

    // spec: canon-kit/SPEC.md §check-manifest-count — prose walk only: the shared driver
    // gates fences and per-site markers, these hooks judge the line and the wrapped paragraph
    struct Sink {
        grammar: spec::CountGrammar,
        out: Vec<String>,
    }
    impl spec::ProseSink for Sink {
        fn on_line(&mut self, file: &str, fnr: usize, raw: &str) {
            if let Some(hit) = self.grammar.hit(raw) {
                self.out
                    .push(format!("  {}:{}  restated collection total: {}", file, fnr, hit));
            }
        }
        fn on_pflush(&mut self, file: &str, para: &spec::Para) {
            if let Some((fnr, hit)) = spec::para_wrapped(&self.grammar, para) {
                self.out
                    .push(format!("  {}:{}  restated collection total: {}", file, fnr, hit));
            }
        }
    }
    let mut sink = Sink {
        grammar: spec::CountGrammar::resolve()?,
        out: Vec::new(),
    };
    // spec: canon-kit/SPEC.md §check-manifest-count — the second sanctioned discharge, riding
    // the same per-site window the exempt tag opens
    spec::walk_prose_multi(
        &manifests,
        &["manifest-count-exempt:", spec::MEASURED_MARKER],
        &mut sink,
    )?;
    let out = sink.out;

    if !out.is_empty() {
        println!("check-manifest-count: bare count(s) quantifying a governed collection in manifest prose — the count's owner is the collection, a restated total drifts silently:");
        println!();
        for l in &out {
            println!("{}", l);
        }
        println!("  help: reword to cite the owning collection (e.g. 'the gates in gates.list') rather than pin a total; a total worth keeping takes a '<!-- measured: <key>=<value> -->' marker on the line above, which binds it to an oracle check-measured-claim re-runs; a genuinely fixed named set joins CANON_KIT_COUNT_ALLOWED_PHRASES; a threshold/rate/partition/proportion is already exempt; else add a 'manifest-count-exempt: <reason>' comment on the line or the one above");
        return Ok(1);
    }
    println!("MANIFEST-COUNT: clean ({} manifest file(s); no bare cardinal quantifying a governed collection in prose outside an exempt site)", manifests.len());
    Ok(0)
}
