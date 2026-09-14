// spec: gate-sdk/SPEC.md §lib/gate.sh — a knob has exactly one producer, and which one is a property
// of its owning kit: a static kit's knobs resolve here from its defaults table and knob files, every
// other name crosses the config bridge, and no name is ever both
use crate::knobfile::{self, Form};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub mod doctrine_kit;
pub mod lifecycle_kit;
pub mod queue_kit;
pub mod site_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Scalar,
    Indexed,
    Keyed,
}

impl Shape {
    pub fn word(self) -> &'static str {
        match self {
            Shape::Scalar => "scalar",
            Shape::Indexed => "indexed",
            Shape::Keyed => "keyed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Scalar(String),
    Indexed(Vec<String>),
    Keyed(Vec<(String, String)>),
}

impl Value {
    // spec: gate-sdk/SPEC.md §lib/gate.sh — the bridge's own serialization, so a reader parses a
    // static value exactly as it parses a bridged one
    pub fn wire(&self) -> String {
        match self {
            Value::Scalar(s) => s.clone(),
            Value::Indexed(v) => v.join("\t"),
            Value::Keyed(m) => m
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("\t"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Env,
    Local,
    Tracked,
    Default,
    Bridged,
}

// spec: gate-sdk/SPEC.md §The knob file — a derived default is a function of already-resolved knobs,
// handed the resolver so it can ask a sibling's value and where that value came from
pub type Resolve<'a> = &'a dyn Fn(&str) -> Result<(Value, Origin), String>;

pub enum Default {
    Scalar(&'static str),
    Indexed(&'static [&'static str]),
    Keyed(&'static [(&'static str, &'static str)]),
    Derived(fn(Resolve) -> Result<Value, String>),
}

pub struct Row {
    pub name: &'static str,
    pub shape: Shape,
    pub default: Default,
    // spec: gate-sdk/SPEC.md §The knob file — every name a derived default reads, so the bridge's
    // closure is known without running the derivation; empty on every other row
    pub inputs: &'static [&'static str],
}

impl Row {
    pub const fn scalar(name: &'static str, v: &'static str) -> Row {
        Row { name, shape: Shape::Scalar, default: Default::Scalar(v), inputs: &[] }
    }

    pub const fn indexed(name: &'static str, v: &'static [&'static str]) -> Row {
        Row { name, shape: Shape::Indexed, default: Default::Indexed(v), inputs: &[] }
    }

    pub const fn keyed(name: &'static str, v: &'static [(&'static str, &'static str)]) -> Row {
        Row { name, shape: Shape::Keyed, default: Default::Keyed(v), inputs: &[] }
    }

    pub const fn derived(
        name: &'static str,
        shape: Shape,
        f: fn(Resolve) -> Result<Value, String>,
        inputs: &'static [&'static str],
    ) -> Row {
        Row { name, shape, default: Default::Derived(f), inputs }
    }
}

// spec: gate-sdk/SPEC.md §The knob file — what a kit validator reads: each row's resolved value, or
// `None` for a row left at a default derived from a bridged input the reading member may not carry
pub type Values = BTreeMap<&'static str, Option<Value>>;

pub type Validator = fn(&Values) -> Vec<String>;

pub struct Kit {
    pub root: &'static str,
    pub rows: &'static [Row],
    // spec: gate-sdk/SPEC.md §The knob file — the config noun the refusal's lead line names, and the
    // check returning every finding, so a malformed config gates nothing
    pub validate: Option<(&'static str, Validator)>,
}

pub fn scalar<'a>(v: &'a Values, name: &str) -> Option<&'a str> {
    match v.get(name) {
        Some(Some(Value::Scalar(s))) => Some(s.as_str()),
        _ => None,
    }
}

pub fn indexed<'a>(v: &'a Values, name: &str) -> Option<&'a [String]> {
    match v.get(name) {
        Some(Some(Value::Indexed(e))) => Some(e.as_slice()),
        _ => None,
    }
}

pub fn keyed<'a>(v: &'a Values, name: &str) -> Option<&'a [(String, String)]> {
    match v.get(name) {
        Some(Some(Value::Keyed(m))) => Some(m.as_slice()),
        _ => None,
    }
}

impl Kit {
    pub fn stem(&self) -> &'static str {
        self.root.strip_suffix("-kit").unwrap_or(self.root)
    }

    pub fn prefix(&self) -> String {
        format!("{}_", self.root.to_ascii_uppercase().replace('-', "_"))
    }

    pub fn knob_file_var(&self) -> String {
        format!("{}KNOB_FILE", self.prefix())
    }

    fn row(&self, name: &str) -> Option<&'static Row> {
        self.rows.iter().find(|r| r.name == name)
    }
}

pub const STATIC_KITS: &[&Kit] = &[
    &doctrine_kit::KIT,
    &lifecycle_kit::KIT,
    &queue_kit::KIT,
    &site_kit::KIT,
];

// spec: canon-kit/SPEC.md §check-docs-cmd — every name a static kit's reader reads: its declared
// knobs, its file locator, and the retired locator the legacy refusal still reads
pub fn static_names() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for kit in STATIC_KITS {
        out.extend(kit.rows.iter().map(|r| r.name.to_string()));
        out.push(kit.knob_file_var());
        out.push(format!("{}CONFIG_FILE", kit.prefix()));
    }
    out
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — ownership is the prefix rule the bridge's own owning-kit
// derivation uses, so a static kit owns every name its prefix spells, declared or not
pub fn owner(name: &str) -> Option<&'static Kit> {
    STATIC_KITS
        .iter()
        .copied()
        .find(|k| name.starts_with(&k.prefix()))
}

pub fn is_static(name: &str) -> bool {
    owner(name.trim_end_matches('*')).is_some()
}

// spec: gate-sdk/SPEC.md §The knob file — the gates directory is a locator, env-or-default, because
// a file cannot name its own directory
pub const GATES_DIR_DEFAULT: &str = "scripts";

fn gates_dir() -> String {
    std::env::var("GATE_SDK_GATES_DIR")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| GATES_DIR_DEFAULT.to_string())
}

type Layer = BTreeMap<String, Value>;

struct Layers {
    local: Layer,
    tracked: Layer,
}

type CacheKey = (&'static str, String, String, String);
type Loaded = Arc<Result<Layers, String>>;
type Cache = Mutex<Vec<(CacheKey, Loaded)>>;

fn cache() -> &'static Cache {
    static CACHE: Cache = Mutex::new(Vec::new());
    &CACHE
}

type Checked = Mutex<Vec<(CacheKey, Result<(), String>)>>;

fn checked() -> &'static Checked {
    static CHECKED: Checked = Mutex::new(Vec::new());
    &CHECKED
}

#[cfg(test)]
pub fn reset(_guard: &crate::knobenv::KnobEnv) {
    cache().lock().unwrap_or_else(|e| e.into_inner()).clear();
    checked().lock().unwrap_or_else(|e| e.into_inner()).clear();
}

fn cache_key(kit: &'static Kit) -> CacheKey {
    (
        kit.root,
        gates_dir(),
        std::env::var(kit.knob_file_var()).unwrap_or_default(),
        std::env::var(format!("{}CONFIG_FILE", kit.prefix())).unwrap_or_default(),
    )
}

fn layers(kit: &'static Kit) -> Loaded {
    let key = cache_key(kit);
    let mut held = cache().lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, l)) = held.iter().find(|(k, _)| *k == key) {
        return l.clone();
    }
    let l = Arc::new(load(kit, &key.1, &key.2, &key.3));
    held.push((key, l.clone()));
    l
}

// spec: gate-sdk/SPEC.md §The knob file — the legacy refusals come first, because a shell config left
// behind at upgrade would otherwise be silently ignored and drop the consumer's values
fn load(kit: &'static Kit, dir: &str, knob_file: &str, config_file: &str) -> Result<Layers, String> {
    let stem = kit.stem();
    for legacy in [
        format!("{}/{}-config.sh", dir, stem),
        format!("{}/{}-config.local.sh", dir, stem),
    ] {
        if Path::new(&legacy).exists() {
            return Err(format!(
                "{} is a shell config, and {}'s knobs are read from a knob file now — rewrite it as \
                 {}/{}-config{}.knobs (`NAME = value`, one `NAME[] = element` line per array \
                 element) and delete it (gate-sdk/SPEC.md §The knob file)",
                legacy,
                kit.root,
                dir,
                stem,
                if legacy.ends_with(".local.sh") { ".local" } else { "" }
            ));
        }
    }
    if !config_file.is_empty()
        && std::fs::metadata(config_file).map(|m| m.len() > 0).unwrap_or(false)
    {
        return Err(format!(
            "{}CONFIG_FILE names {}, a shell config — rewrite it in the knob-file grammar and point \
             {} at it instead (gate-sdk/SPEC.md §The knob file)",
            kit.prefix(),
            config_file,
            kit.knob_file_var()
        ));
    }
    let tracked_path = if knob_file.is_empty() {
        format!("{}/{}-config.knobs", dir, stem)
    } else {
        if !Path::new(knob_file).is_file() {
            return Err(format!(
                "{} names {}, which does not exist — point it at the knob file, or unset it to read \
                 {}/{}-config.knobs",
                kit.knob_file_var(),
                knob_file,
                dir,
                stem
            ));
        }
        knob_file.to_string()
    };
    let local_path = format!("{}/{}-config.local.knobs", dir, stem);
    Ok(Layers {
        local: read_layer(kit, &local_path)?,
        tracked: read_layer(kit, &tracked_path)?,
    })
}

fn read_layer(kit: &'static Kit, path: &str) -> Result<Layer, String> {
    if !Path::new(path).is_file() {
        return Ok(Layer::new());
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    layer(kit, path, &text)
}

// spec: gate-sdk/SPEC.md §The knob file — a knob named in a file is replaced whole, and a line whose
// form disagrees with the declared shape is refused with its file and line
fn layer(kit: &'static Kit, path: &str, text: &str) -> Result<Layer, String> {
    let mut out = Layer::new();
    let mut emptied: Vec<String> = Vec::new();
    let at = |lno: usize, what: String| format!("{}:{}: {}", path, lno, what);
    for e in knobfile::parse(text, path)? {
        let Some(row) = kit.row(&e.name) else {
            return Err(at(
                e.lno,
                format!(
                    "{} is not a {} knob — `--emit knob-roster` lists the names this file may set",
                    e.name, kit.root
                ),
            ));
        };
        let twice = || at(e.lno, format!("{} is given twice — keep one line for it", e.name));
        let shape_word = row.shape.word();
        let mismatch = || {
            at(
                e.lno,
                format!(
                    "{} is declared {} and this line is not that form — write {}",
                    e.name,
                    shape_word,
                    match row.shape {
                        Shape::Scalar => format!("`{} = value`", e.name),
                        Shape::Indexed => format!("one `{}[] = element` line per element", e.name),
                        Shape::Keyed => format!("one `{}[key] = value` line per pair", e.name),
                    }
                ),
            )
        };
        match (row.shape, &e.form) {
            (Shape::Scalar, Form::Scalar) => {
                if out.contains_key(&e.name) {
                    return Err(twice());
                }
                out.insert(e.name.clone(), Value::Scalar(e.value.clone()));
            }
            (Shape::Indexed | Shape::Keyed, Form::Scalar) if e.value.is_empty() => {
                if out.contains_key(&e.name) {
                    return Err(twice());
                }
                emptied.push(e.name.clone());
                let v = if row.shape == Shape::Indexed {
                    Value::Indexed(Vec::new())
                } else {
                    Value::Keyed(Vec::new())
                };
                out.insert(e.name.clone(), v);
            }
            (Shape::Indexed, Form::Indexed) => {
                if emptied.contains(&e.name) {
                    return Err(twice());
                }
                match out
                    .entry(e.name.clone())
                    .or_insert_with(|| Value::Indexed(Vec::new()))
                {
                    Value::Indexed(v) => v.push(e.value.clone()),
                    _ => return Err(mismatch()),
                }
            }
            (Shape::Keyed, Form::Keyed(key)) => {
                if emptied.contains(&e.name) {
                    return Err(twice());
                }
                match out
                    .entry(e.name.clone())
                    .or_insert_with(|| Value::Keyed(Vec::new()))
                {
                    Value::Keyed(m) => {
                        if m.iter().any(|(k, _)| k == key) {
                            return Err(twice());
                        }
                        m.push((key.clone(), e.value.clone()));
                        m.sort();
                    }
                    _ => return Err(mismatch()),
                }
            }
            _ => return Err(mismatch()),
        }
    }
    Ok(out)
}

fn default_value(row: &'static Row, resolve: Resolve) -> Result<Value, String> {
    Ok(match &row.default {
        Default::Scalar(s) => Value::Scalar(s.to_string()),
        Default::Indexed(v) => Value::Indexed(v.iter().map(|s| s.to_string()).collect()),
        Default::Keyed(m) => {
            let mut pairs: Vec<(String, String)> =
                m.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            pairs.sort();
            Value::Keyed(pairs)
        }
        Default::Derived(f) => f(resolve)?,
    })
}

fn undeclared(kit: &Kit, name: &str) -> String {
    format!(
        "{} is not a {} knob — `--emit knob-roster` lists the names it declares",
        name, kit.root
    )
}

fn static_row(name: &str) -> Result<(&'static Kit, &'static Row), String> {
    let kit = owner(name).ok_or_else(|| format!("{} is not a statically owned knob", name))?;
    let row = kit.row(name).ok_or_else(|| undeclared(kit, name))?;
    Ok((kit, row))
}

// spec: gate-sdk/SPEC.md §The knob file — the precedence, highest first: the environment for a
// scalar only, the local overlay, the tracked file; `None` leaves the kit default to the caller
fn layered(kit: &'static Kit, row: &'static Row) -> Result<Option<(Value, Origin)>, String> {
    let l = layers(kit);
    let l = l.as_ref().as_ref().map_err(String::clone)?;
    if row.shape == Shape::Scalar {
        if let Ok(v) = std::env::var(row.name) {
            return Ok(Some((Value::Scalar(v), Origin::Env)));
        }
    }
    if let Some(v) = l.local.get(row.name) {
        return Ok(Some((v.clone(), Origin::Local)));
    }
    if let Some(v) = l.tracked.get(row.name) {
        return Ok(Some((v.clone(), Origin::Tracked)));
    }
    Ok(None)
}

fn lookup(name: &str) -> Result<(Value, Origin), String> {
    let (kit, row) = static_row(name)?;
    match layered(kit, row)? {
        Some(v) => Ok(v),
        None => Ok((default_value(row, &input)?, Origin::Default)),
    }
}

// spec: gate-sdk/SPEC.md §The knob file — a derivation's resolver: a static sibling through the
// table, a bridged input through the one reader function, whose absence is its ordinary refusal
fn input(name: &str) -> Result<(Value, Origin), String> {
    if owner(name).is_some() {
        return lookup(name);
    }
    crate::walk::knob_scalar(name).map(|v| (Value::Scalar(v), Origin::Bridged))
}

// spec: gate-sdk/SPEC.md §The knob file — a row whose default reads a bridged input, directly or
// through a static sibling, which the validator skips while that row sits at its default
fn reaches_bridged(row: &'static Row) -> bool {
    row.inputs.iter().any(|n| match owner(n) {
        None => true,
        Some(k) => k.row(n).is_some_and(reaches_bridged),
    })
}

// spec: gate-sdk/SPEC.md §The knob file — the kit validator runs once per process per kit, at its
// first resolution, and refuses with every finding under the kit's malformed-config lead line
fn validated(kit: &'static Kit) -> Result<(), String> {
    let Some((noun, check)) = kit.validate else {
        return Ok(());
    };
    let key = cache_key(kit);
    if let Some((_, r)) = checked()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .find(|(k, _)| *k == key)
    {
        return r.clone();
    }
    let run = || -> Result<(), String> {
        let mut values = Values::new();
        for row in kit.rows {
            let v = match layered(kit, row)? {
                Some((v, _)) => Some(v),
                None if reaches_bridged(row) => None,
                None => Some(default_value(row, &input)?),
            };
            values.insert(row.name, v);
        }
        let findings = check(&values);
        if findings.is_empty() {
            return Ok(());
        }
        Err(format!(
            "{}: malformed {} — the gates cannot run:\n  {}",
            kit.root,
            noun,
            findings.join("\n  ")
        ))
    };
    let r = run();
    checked()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push((key, r.clone()));
    r
}

pub fn resolve(name: &str) -> Result<(Value, Origin), String> {
    let (kit, _) = static_row(name)?;
    validated(kit)?;
    lookup(name)
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the one function every knob read resolves through: a
// statically owned name from its kit, any other from its `GATE_SDK_KNOB_<NAME>` variable, whose
// absence is the caller's to report
pub fn wire(name: &str) -> Result<Option<String>, String> {
    if owner(name).is_some() {
        return resolve(name).map(|(v, _)| Some(v.wire()));
    }
    Ok(std::env::var(format!("GATE_SDK_KNOB_{}", name)).ok())
}

// spec: gate-sdk/SPEC.md §The non-gate arm — one line per element in the roster's grammar, and one
// line with an empty third field for an empty collection
fn render(out: &mut String, row: &'static Row, v: Value) {
    let lines: Vec<String> = match v {
        Value::Scalar(s) => vec![s],
        Value::Indexed(e) => e,
        Value::Keyed(m) => m.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect(),
    };
    if lines.is_empty() {
        out.push_str(&format!("{}\t{}\t\n", row.name, row.shape.word()));
    }
    for l in lines {
        out.push_str(&format!("{}\t{}\t{}\n", row.name, row.shape.word(), l));
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — `--emit knob-roster`: every statically owned knob with
// its shape and default, the default rendered with every sibling at its own default and a bridged
// input as `${NAME}`, because gate-sdk's default for that input is not the crate's to spell
pub fn roster() -> Result<String, String> {
    fn defaults_only(name: &str) -> Result<(Value, Origin), String> {
        if owner(name).is_none() {
            return Ok((Value::Scalar(format!("${{{}}}", name)), Origin::Bridged));
        }
        let (_, row) = static_row(name)?;
        Ok((default_value(row, &defaults_only)?, Origin::Default))
    }
    let mut out = String::new();
    for kit in STATIC_KITS {
        for row in kit.rows {
            let (v, _) = defaults_only(row.name)?;
            render(&mut out, row, v);
        }
    }
    Ok(out)
}

pub fn emit(_args: &[String]) -> Result<String, String> {
    roster()
}

// spec: gate-sdk/SPEC.md §The non-gate arm — `--emit knob-values`: the resolved value of each named
// static knob in the invoking tree, in the roster's grammar; its declared set is its argv's closure
pub const ARGV_STATIC_KNOBS: &str = "@argv-static-knobs";

pub fn values(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("needs at least one static knob name — usage: --emit knob-values <NAME>...".to_string());
    }
    let mut out = String::new();
    for name in args {
        let (_, row) = static_row(name)?;
        let (v, _) = resolve(name)?;
        render(&mut out, row, v);
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the argv names a static table declares, spelled as its
// rows spell them, so the arm's declared set can be closed like any member's
pub fn declared_names(args: &[String]) -> Vec<&'static str> {
    args.iter()
        .filter_map(|a| owner(a).and_then(|k| k.row(a)).map(|r| r.name))
        .collect()
}

fn bridged_inputs(row: &'static Row, out: &mut Vec<&str>) {
    for i in row.inputs {
        match owner(i) {
            None => {
                if !out.contains(i) {
                    out.push(i);
                }
            }
            Some(k) => {
                if let Some(r) = k.row(i) {
                    bridged_inputs(r, out);
                }
            }
        }
    }
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — `--knobs` answers the knobs a member needs the bridge to
// carry: its non-static names, plus the bridged inputs its static names' derivations reach, so a
// static name never reaches a kit library and a derived default never meets an absent input
pub fn bridged<'a>(names: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
    let names: Vec<&'a str> = names.into_iter().collect();
    let mut out: Vec<&'a str> = names.iter().copied().filter(|n| !is_static(n)).collect();
    for n in names.iter().filter(|n| is_static(n)) {
        let Some(kit) = owner(n.trim_end_matches('*')) else {
            continue;
        };
        match n.strip_suffix('*') {
            Some(stem) => {
                for r in kit.rows.iter().filter(|r| r.name.starts_with(stem)) {
                    bridged_inputs(r, &mut out);
                }
            }
            None => {
                if let Some(r) = kit.row(n) {
                    bridged_inputs(r, &mut out);
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knobenv;

    struct Scratch(std::path::PathBuf);

    impl Scratch {
        fn new(tag: &str) -> Scratch {
            let d = std::env::temp_dir().join(format!("checkwright-knobs-{}.{}", tag, std::process::id()));
            let _ = std::fs::remove_dir_all(&d);
            std::fs::create_dir_all(&d).expect("scratch");
            Scratch(d)
        }
        fn write(&self, rel: &str, body: &str) -> String {
            let p = self.0.join(rel);
            std::fs::write(&p, body).expect("write");
            p.display().to_string()
        }
        fn dir(&self) -> String {
            self.0.display().to_string()
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn clean(env: &knobenv::KnobEnv, dir: &str) {
        env.set("GATE_SDK_GATES_DIR", dir);
        for k in STATIC_KITS {
            env.remove(&k.knob_file_var());
            env.remove(&format!("{}CONFIG_FILE", k.prefix()));
            for r in k.rows {
                env.remove(r.name);
            }
        }
        reset(env);
    }

    fn restore(env: &knobenv::KnobEnv) {
        env.remove("GATE_SDK_GATES_DIR");
        reset(env);
    }

    #[test]
    fn precedence_is_env_then_local_then_tracked_then_default() {
        let env = knobenv::lock();
        let s = Scratch::new("precedence");
        clean(&env, &s.dir());
        assert_eq!(resolve("DOCTRINE_KIT_AGENT_FILE").unwrap(), (Value::Scalar("CLAUDE.md".into()), Origin::Default));
        s.write("doctrine-config.knobs", "DOCTRINE_KIT_AGENT_FILE = TRACKED.md\n");
        reset(&env);
        assert_eq!(resolve("DOCTRINE_KIT_AGENT_FILE").unwrap().1, Origin::Tracked);
        s.write("doctrine-config.local.knobs", "DOCTRINE_KIT_AGENT_FILE = LOCAL.md\n");
        reset(&env);
        assert_eq!(resolve("DOCTRINE_KIT_AGENT_FILE").unwrap(), (Value::Scalar("LOCAL.md".into()), Origin::Local));
        env.set("DOCTRINE_KIT_AGENT_FILE", "ENV.md");
        assert_eq!(resolve("DOCTRINE_KIT_AGENT_FILE").unwrap(), (Value::Scalar("ENV.md".into()), Origin::Env));
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn an_indexed_knob_takes_no_environment_override_and_is_replaced_whole() {
        let env = knobenv::lock();
        let s = Scratch::new("indexed");
        clean(&env, &s.dir());
        env.set("SITE_KIT_EXEMPT_PATHS", "from-env");
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap().unwrap(), "*/gate-tests/*\t*docs/posts/*");
        s.write("site-config.knobs", "SITE_KIT_EXEMPT_PATHS[] = only one\n");
        reset(&env);
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap().unwrap(), "only one");
        s.write("site-config.knobs", "SITE_KIT_EXEMPT_PATHS =\n");
        reset(&env);
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap().unwrap(), "");
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn every_file_refusal_names_its_file_and_line() {
        let env = knobenv::lock();
        let s = Scratch::new("refusals");
        let undeclared_line = format!("{}NOPE = 1\n", site_kit::KIT.prefix());
        for (body, want) in [
            (undeclared_line.as_str(), "not a site-kit knob"),
            ("SITE_KIT_CNAME[] = x\n", "is declared scalar"),
            ("SITE_KIT_ALIASES = x\n", "is declared indexed"),
            ("SITE_KIT_ALIASES[k] = x\n", "is declared indexed"),
            ("SITE_KIT_CNAME = a\nSITE_KIT_CNAME = b\n", "given twice"),
            ("SITE_KIT_ALIASES =\nSITE_KIT_ALIASES[] = a\n", "given twice"),
            ("GATE_SDK_TMP_DIR = x\n", "not a site-kit knob"),
            ("site_kit_cname = x\n", "SCREAMING_SNAKE"),
        ] {
            clean(&env, &s.dir());
            let p = s.write("site-config.knobs", &format!("# header\n{}", body));
            reset(&env);
            let e = resolve("SITE_KIT_CNAME").expect_err(body);
            let lno = body.lines().count() + 1;
            assert!(e.starts_with(&format!("{}:{}: ", p, lno)), "{:?}: {}", body, e);
            assert!(e.contains(want), "{:?}: {}", body, e);
        }
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn the_legacy_configs_and_a_set_missing_knob_file_are_refused() {
        let env = knobenv::lock();
        let s = Scratch::new("legacy");
        clean(&env, &s.dir());
        s.write("site-config.local.sh", "SITE_KIT_CNAME=x\n");
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("site-config.local.knobs"));
        std::fs::remove_file(s.0.join("site-config.local.sh")).unwrap();
        s.write("site-config.sh", "SITE_KIT_CNAME=x\n");
        reset(&env);
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("is a shell config"));
        std::fs::remove_file(s.0.join("site-config.sh")).unwrap();

        let empty = s.write("empty.sh", "");
        env.set("SITE_KIT_CONFIG_FILE", &empty);
        reset(&env);
        assert!(resolve("SITE_KIT_CNAME").is_ok(), "an empty pinned shell config is inert");
        let full = s.write("full.sh", "SITE_KIT_CNAME=x\n");
        env.set("SITE_KIT_CONFIG_FILE", &full);
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("SITE_KIT_KNOB_FILE"));
        env.remove("SITE_KIT_CONFIG_FILE");

        env.set("SITE_KIT_KNOB_FILE", &format!("{}/absent.knobs", s.dir()));
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("does not exist"));
        let pinned = s.write("pinned.knobs", "SITE_KIT_CNAME = PINNED\n");
        env.set("SITE_KIT_KNOB_FILE", &pinned);
        assert_eq!(wire("SITE_KIT_CNAME").unwrap().unwrap(), "PINNED");
        clean(&env, &s.dir());
        restore(&env);
    }

    static KEYED_KIT: Kit = Kit {
        root: "probe-kit",
        rows: &[Row::keyed("PROBE_KIT_MAP", &[])],
        validate: None,
    };

    #[test]
    fn a_keyed_knob_takes_pairs_sorted_and_refuses_a_repeated_key_or_an_element() {
        let got = layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[b] = 2 = two\nPROBE_KIT_MAP[a] = 1\n").expect("parses");
        assert_eq!(got["PROBE_KIT_MAP"].wire(), "a=1\tb=2 = two");
        assert!(layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[a] = 1\nPROBE_KIT_MAP[a] = 2\n").unwrap_err().starts_with("f:2: "));
        assert!(layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[] = 1\n").unwrap_err().contains("is declared keyed"));
        assert_eq!(layer(&KEYED_KIT, "f", "PROBE_KIT_MAP =\n").expect("empties")["PROBE_KIT_MAP"].wire(), "");
    }

    #[test]
    fn an_undeclared_static_name_is_an_error_and_a_bridged_name_reads_the_wire() {
        let env = knobenv::lock();
        assert!(wire(&format!("{}NOT_A_KNOB", site_kit::KIT.prefix())).unwrap_err().contains("not a site-kit knob"));
        env.set("GATE_SDK_KNOB_PROBE_ROUTED", "a\tb");
        assert_eq!(wire("PROBE_ROUTED").unwrap().as_deref(), Some("a\tb"));
        env.remove("GATE_SDK_KNOB_PROBE_ROUTED");
        assert_eq!(wire("PROBE_ROUTED").unwrap(), None);
    }

    #[test]
    fn the_roster_prints_one_line_per_element_and_an_empty_default_as_an_empty_field() {
        let r = roster().expect("renders");
        assert!(r.contains("SITE_KIT_CNAME\tscalar\tdocs/CNAME\n"));
        assert!(r.contains("SITE_KIT_ALIASES\tindexed\t\n"));
        assert!(r.contains("SITE_KIT_EXEMPT_PATHS\tindexed\t*/gate-tests/*\nSITE_KIT_EXEMPT_PATHS\tindexed\t*docs/posts/*\n"));
        for line in r.lines() {
            assert_eq!(line.split('\t').count(), 3, "{:?}", line);
        }
    }

    #[test]
    fn a_static_name_is_never_bridged() {
        assert_eq!(
            bridged(["SITE_KIT_CNAME", "GATE_PRUNE_DIRS", "DOCTRINE_KIT_AGENT_FILE"]),
            vec!["GATE_PRUNE_DIRS"]
        );
    }

    // spec: gate-sdk/SPEC.md §The knob file — the closure carries a derived default's bridged inputs,
    // transitively through a static sibling, once each and never the static name itself
    #[test]
    fn the_bridged_closure_reaches_a_derived_defaults_inputs_through_its_siblings() {
        assert_eq!(
            bridged(["LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS", "GATE_SDK_QUEUE_FILE", "QUEUE_KIT_QUEUE_FILE"]),
            vec!["GATE_SDK_QUEUE_FILE"]
        );
        assert_eq!(
            bridged(["LIFECYCLE_KIT_STATE_FILE", "LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN", "LIFECYCLE_KIT_STAGES"]),
            vec!["GATE_SDK_WORKFLOW_DIR", "GATE_SDK_TMP_DIR"]
        );
    }

    // spec: gate-sdk/SPEC.md §The knob file — the closure's soundness rests on each derived row's
    // declared inputs, so every derivation runs under a recording resolver and asks nothing undeclared
    #[test]
    fn every_derived_row_reads_only_its_declared_inputs() {
        let mut derived = 0usize;
        for kit in STATIC_KITS {
            for row in kit.rows {
                let Default::Derived(f) = row.default else {
                    assert!(row.inputs.is_empty(), "{} declares inputs but derives nothing", row.name);
                    continue;
                };
                derived += 1;
                let asked = std::cell::RefCell::new(Vec::<String>::new());
                let record = |n: &str| -> Result<(Value, Origin), String> {
                    asked.borrow_mut().push(n.to_string());
                    let (_, r) = match static_row(n) {
                        Ok(v) => v,
                        Err(_) => return Ok((Value::Scalar("in".into()), Origin::Bridged)),
                    };
                    Ok((default_value(r, &|_| Ok((Value::Scalar("in".into()), Origin::Bridged)))?, Origin::Default))
                };
                f(&record).unwrap_or_else(|e| panic!("{}: {}", row.name, e));
                for n in asked.borrow().iter() {
                    assert!(row.inputs.contains(&n.as_str()), "{} reads {}, which its row does not declare", row.name, n);
                }
            }
        }
        assert!(derived > 0, "no derived row to hold");
    }

    // spec: gate-sdk/SPEC.md §The knob file — skipping a bridged-derived row at its default is sound
    // only because every kit default passes its own validator
    #[test]
    fn every_kit_default_passes_its_validator() {
        for kit in STATIC_KITS {
            let Some((_, check)) = kit.validate else {
                continue;
            };
            let mut values = Values::new();
            for row in kit.rows {
                let v = default_value(row, &|n| match static_row(n) {
                    Ok((_, r)) => Ok((default_value(r, &|b| Ok((Value::Scalar(format!("dir-{}", b)), Origin::Bridged)))?, Origin::Default)),
                    Err(_) => Ok((Value::Scalar(format!("dir-{}", n)), Origin::Bridged)),
                })
                .expect("a default renders");
                values.insert(row.name, Some(v));
            }
            assert_eq!(check(&values), Vec::<String>::new(), "{}'s defaults fail its validator", kit.root);
        }
    }

    // spec: gate-sdk/SPEC.md §The knob file — a malformed file value is refused at the kit's first
    // read with every finding, and a bridged-derived row at its default is not asked for its input
    #[test]
    fn a_malformed_config_is_refused_at_first_resolution_with_every_finding() {
        let env = knobenv::lock();
        let s = Scratch::new("validate");
        clean(&env, &s.dir());
        env.remove("GATE_SDK_KNOB_GATE_SDK_WORKFLOW_DIR");
        env.remove("GATE_SDK_KNOB_GATE_SDK_TMP_DIR");
        assert_eq!(resolve("LIFECYCLE_KIT_FIRST_STAGE").unwrap().0, Value::Scalar("scope".into()));
        s.write("lifecycle-config.knobs", "LIFECYCLE_KIT_FIRST_STAGE = nope\nLIFECYCLE_KIT_SHIM_NGRAM = x\n");
        reset(&env);
        let e = resolve("LIFECYCLE_KIT_STAGES").unwrap_err();
        assert!(e.starts_with("lifecycle-kit: malformed stage-machine config"), "{}", e);
        assert!(e.contains("'nope' is not in LIFECYCLE_KIT_STAGES") && e.contains("'x' is not a positive integer"), "{}", e);
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn knob_values_prints_the_resolved_value_and_refuses_an_unowned_name() {
        let env = knobenv::lock();
        let s = Scratch::new("values");
        clean(&env, &s.dir());
        s.write("queue-config.knobs", "QUEUE_KIT_TRACKS[] = a\nQUEUE_KIT_TRACKS[] = b\nQUEUE_KIT_HORIZONS[] = now\n");
        reset(&env);
        let args = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(
            values(&args(&["QUEUE_KIT_TRACKS", "QUEUE_KIT_PROSE_SURFACE_GLOBS"])).unwrap(),
            "QUEUE_KIT_TRACKS\tindexed\ta\nQUEUE_KIT_TRACKS\tindexed\tb\nQUEUE_KIT_PROSE_SURFACE_GLOBS\tindexed\t\n"
        );
        assert!(values(&args(&["GATE_PRUNE_DIRS"])).is_err());
        assert!(values(&[format!("{}NOPE", queue_kit::KIT.prefix())]).is_err());
        assert_eq!(declared_names(&args(&["LIFECYCLE_KIT_STATE_FILE", "NOPE"])), vec!["LIFECYCLE_KIT_STATE_FILE"]);
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn the_roster_renders_a_bridged_input_by_name() {
        let r = roster().expect("renders");
        assert!(r.contains("LIFECYCLE_KIT_STATE_FILE\tscalar\t${GATE_SDK_WORKFLOW_DIR}/WORKFLOW-STATE.txt\n"), "{}", r);
        assert!(r.contains("LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS\tindexed\t${GATE_SDK_QUEUE_FILE}\n"), "{}", r);
        assert!(r.contains("LIFECYCLE_KIT_PREDECESSOR\tkeyed\talign=scope\n"), "{}", r);
    }

    // spec: gate-sdk/SPEC.md §The knob file — the gates-directory default is spelled here and in
    // lib/gate.sh, and no single owner can hold a locator until gate-sdk migrates
    #[test]
    fn the_gates_dir_default_agrees_with_the_shell_library() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let out = std::process::Command::new("env")
            .args(["-u", "GATE_SDK_GATES_DIR", "bash", "-c", "source gate-sdk/lib/gate.sh; gate_sdk_gates_dir"])
            .current_dir(&root)
            .output()
            .expect("cannot run the shell library");
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), GATES_DIR_DEFAULT);
    }
}
