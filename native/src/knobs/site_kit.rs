// spec: site-kit/SPEC.md §Knob defaults — site-kit's static knob table
use super::{Default, Kit, Origin, Resolve, Row, Shape, Value};

const RENDERER: &[&str] = &[
    "ruby",
    "-e",
    r#"require "kramdown"; require "kramdown-parser-gfm"; STDOUT.write(Kramdown::Document.new(STDIN.read, input: "GFM").to_html)"#,
];

const RENDERER_BATCH: &[&str] = &[
    "ruby",
    "-e",
    r#"require "kramdown"; require "kramdown-parser-gfm"; d = STDIN.read.split("\x00", -1); d.pop; d.each { |s| STDOUT.write(Kramdown::Document.new(s, input: "GFM").to_html); STDOUT.write("\x00") }"#,
];

// spec: site-kit/SPEC.md §Knob defaults — the one conditional default: a consumer who pinned
// SITE_KIT_RENDERER is never handed this unpinned batch oracle in its place
fn renderer_batch(resolve: Resolve) -> Result<Value, String> {
    let (_, origin) = resolve("SITE_KIT_RENDERER")?;
    Ok(Value::Indexed(if origin == Origin::Default {
        RENDERER_BATCH.iter().map(|s| s.to_string()).collect()
    } else {
        Vec::new()
    }))
}

pub const KIT: Kit = Kit {
    root: "site-kit",
    rows: &[
        Row {
            name: "SITE_KIT_CNAME",
            shape: Shape::Scalar,
            default: Default::Scalar("docs/CNAME"),
            inputs: &[],
        },
        Row {
            name: "SITE_KIT_SCAN_ROOT",
            shape: Shape::Scalar,
            default: Default::Scalar("."),
            inputs: &[],
        },
        Row {
            name: "SITE_KIT_DOCS_DIR",
            shape: Shape::Scalar,
            default: Default::Scalar("docs"),
            inputs: &[],
        },
        Row {
            name: "SITE_KIT_ALIASES",
            shape: Shape::Indexed,
            default: Default::Indexed(&[]),
            inputs: &[],
        },
        Row {
            name: "SITE_KIT_EXEMPT_PATHS",
            shape: Shape::Indexed,
            default: Default::Indexed(&["*/gate-tests/*", "*docs/posts/*"]),
            inputs: &[],
        },
        Row {
            name: "SITE_KIT_RENDERER",
            shape: Shape::Indexed,
            default: Default::Indexed(RENDERER),
            inputs: &[],
        },
        Row {
            name: "SITE_KIT_RENDERER_BATCH",
            shape: Shape::Indexed,
            default: Default::Derived(renderer_batch),
            inputs: &["SITE_KIT_RENDERER"],
        },
    ],
    validate: None,
    open_family: false,
    retired: &[],
};

#[cfg(test)]
mod tests {
    use super::super::{reset, wire};
    use crate::knobenv;

    // spec: site-kit/SPEC.md §Knob defaults — the fill rule, both halves: an unpinned renderer arms
    // the batch default, a pinned one gets an empty batch knob
    #[test]
    fn the_batch_default_fills_only_behind_the_default_renderer() {
        let env = knobenv::lock();
        let d = std::env::temp_dir().join(format!("checkwright-site-fill.{}", std::process::id()));
        std::fs::create_dir_all(&d).expect("scratch");
        env.set("GATE_SDK_GATES_DIR", &d.display().to_string());
        env.remove("SITE_KIT_KNOB_FILE");
        env.remove("SITE_KIT_CONFIG_FILE");
        reset(&env);
        let batch = wire("SITE_KIT_RENDERER_BATCH").unwrap().unwrap();
        assert!(batch.starts_with("ruby\t-e\t") && batch.contains(r#"split("\x00", -1)"#), "{}", batch);
        std::fs::write(d.join("site-config.knobs"), "SITE_KIT_RENDERER[] = true\n").expect("write");
        reset(&env);
        assert_eq!(wire("SITE_KIT_RENDERER_BATCH").unwrap().unwrap(), "");
        std::fs::write(d.join("site-config.knobs"), "SITE_KIT_RENDERER[] = true\nSITE_KIT_RENDERER_BATCH[] = cat\n").expect("write");
        reset(&env);
        assert_eq!(wire("SITE_KIT_RENDERER_BATCH").unwrap().unwrap(), "cat");
        let _ = std::fs::remove_dir_all(&d);
        env.remove("GATE_SDK_GATES_DIR");
        reset(&env);
    }
}
