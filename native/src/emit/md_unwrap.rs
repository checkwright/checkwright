// spec: canon-kit/SPEC.md §check-md-unwrapped — `--emit md-unwrap [--write] <file>…`, the gate's
// remedy: every soft break the gate's scanner finds is joined onto its predecessor
use crate::exit::Fault;
use crate::gates::md_unwrapped;

pub const KNOBS: &[&str] = &["CANON_KIT_UNWRAP_DECLARATION_LEADS"];

pub const USAGE: &str = "usage: --emit md-unwrap [--write] <file>…";

pub fn emit(args: &[String]) -> Result<String, Fault> {
    let write = args.first().is_some_and(|a| a == "--write");
    let files: Vec<&String> = args.iter().skip(usize::from(write)).collect();
    if files.is_empty() || files.iter().any(|f| f.starts_with("--")) {
        return Err(Fault::usage(USAGE));
    }
    let leads = md_unwrapped::declaration_leads().map_err(Fault::refused)?;
    let mut out = String::new();
    let mut failed: Vec<String> = Vec::new();
    let mut wrote_badly = false;
    for f in files {
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                failed.push(format!("{}: {}", f, e));
                continue;
            }
        };
        let joined = md_unwrapped::unwrap(&text, &leads);
        if !write {
            out.push_str(&joined);
        } else if joined != text {
            if let Err(e) = std::fs::write(f, &joined) {
                failed.push(format!("{}: {}", f, e));
                wrote_badly = true;
            } else if std::fs::read_to_string(f).ok().as_deref() != Some(joined.as_str()) {
                failed.push(format!("{}: the written text does not read back", f));
                wrote_badly = true;
            }
        }
    }
    if !failed.is_empty() {
        let message = format!("md-unwrap: could not rewrite:\n  {}", failed.join("\n  "));
        return Err(if wrote_badly { Fault::tool(message) } else { Fault::input(message) });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: canon-kit/SPEC.md §check-md-unwrapped — a malformed argv refuses before the knob read,
    // so none reaches a file
    #[test]
    fn a_malformed_argv_refuses_with_the_usage() {
        let _knobs = crate::knobenv::lock();
        for args in [&[][..], &["--write"], &["--help"], &["a.md", "--b.md"]] {
            let argv: Vec<String> = args.iter().map(|a| a.to_string()).collect();
            let fault = emit(&argv).expect_err("a malformed argv must refuse");
            assert_eq!(fault.message, USAGE, "{:?}", args);
            assert_eq!(fault.status(), crate::exit::USAGE, "{:?}", args);
        }
    }
}
