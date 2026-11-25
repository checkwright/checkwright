// spec: gate-sdk/SPEC.md §The non-gate arm — the harness-integration arms: non-gate arms
// whose named caller is the coding harness, so their channels are the harness's rather than the
// gate output contract's. `--hook <member>` is the one dispatching arm of that sub-class.
use crate::exit;
use serde_json::Value;

pub mod antigravity;
pub mod binding;
pub mod budget;
pub mod dispatch;
pub mod escalation;
pub mod model_verdict;
pub mod observe;
pub mod patch;
pub mod poll;
pub mod session_context;
pub mod shell_guard;
pub mod statusline;
pub mod stop_liveness;
pub mod usage;
pub mod verdict;
pub mod wakeup;
pub mod workflow_state;

// spec: gate-sdk/SPEC.md §The non-gate arm — the parse is `Option` rather than a `Value` that
// might be null, because an absent, empty or unparseable payload is one condition with one
// consequence: the member's own degraded path, the path each shell member had for a missing jq.
pub struct Payload {
    bytes: Vec<u8>,
    value: Option<Value>,
}

impl Payload {
    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub type HookFn = fn(&Payload) -> i32;

// spec: gate-sdk/SPEC.md §The harness-integration arm — the member table: the single roster the
// arm dispatches on and the unknown-member refusal prints. Each row's knob slice is exactly what
// that member's shell original read, less the knobs a compiled member cannot want; then the owning
// kit, held to the member module's leading `spec:` binding by a unit test below, and the tree-free
// declaration: every input the member's rule reads can resolve absolute; then the event the member
// registers under, a binding spelling; last, the tools a before-tool-call member registers on.
pub type Row = (
    &'static str,
    HookFn,
    &'static [&'static str],
    &'static str,
    bool,
    &'static str,
    &'static [&'static str],
);

pub const HOOKS: &[Row] = &[
    // spec: delegation-kit/SPEC.md §usage-verdict — the rule runs inside the hook process, so the row
    // declares the rule's own reads rather than a path to it, one roster for both callers
    (
        "agent-budget-guard",
        |p| budget::run(p.value()),
        verdict::KNOBS,
        "delegation-kit",
        true,
        binding::EVENT_BEFORE_TOOL_CALL,
        &[binding::TOOL_DISPATCH],
    ),
    (
        "agent-dispatch-guard",
        |p| dispatch::run(p.value()),
        &[
            "DELEGATION_KIT_READONLY_TYPES",
            "DELEGATION_KIT_MUTATING_TYPES",
            "DELEGATION_KIT_REQUIRE_TIER",
            "DELEGATION_KIT_AGENT_DIR",
            "DELEGATION_KIT_TIER_MODEL",
            "GATE_SDK_NATIVE_BIN",
        ],
        "delegation-kit",
        true,
        binding::EVENT_BEFORE_TOOL_CALL,
        &[binding::TOOL_DISPATCH],
    ),
    (
        "subagent-stop-liveness",
        |p| stop_liveness::run(p.value()),
        &[
            "DELEGATION_KIT_STOP_LOG",
            "DELEGATION_KIT_STOP_WAIT",
            "DELEGATION_KIT_LIVENESS_CMD",
            "GATE_SDK_TMP_DIR",
        ],
        "delegation-kit",
        true,
        binding::EVENT_CHILD_TURN_END,
        &[],
    ),
    (
        "escalation-guard",
        |p| escalation::run(p.value()),
        &[],
        "guard-kit",
        true,
        binding::EVENT_BEFORE_TOOL_CALL,
        &[binding::TOOL_MESSAGE],
    ),
    // spec: gate-sdk/SPEC.md §The harness-integration arm — tree-bound: it runs the binary's arms
    // over the tree
    (
        "session-context",
        session_context::run,
        session_context::KNOBS,
        "context-kit",
        false,
        binding::EVENT_SESSION_START,
        &[],
    ),
    // spec: gate-sdk/SPEC.md §The harness-integration arm — tree-bound: it asks git for the main
    // checkout and reads the project's settings
    (
        "shell-guard",
        shell_guard::run,
        crate::guard::host::KNOBS,
        "guard-kit",
        false,
        binding::EVENT_BEFORE_TOOL_CALL,
        &[binding::TOOL_BASH, binding::TOOL_POWERSHELL],
    ),
    (
        "wakeup-guard",
        |p| wakeup::run(p.value()),
        &["GUARD_KIT_WAKEUP_LOG"],
        "guard-kit",
        true,
        binding::EVENT_BEFORE_TOOL_CALL,
        &[binding::TOOL_WAKE, binding::TOOL_CRON],
    ),
    // spec: gate-sdk/SPEC.md §The harness-integration arm — tree-bound: it judges the repository's
    // stamp file
    (
        "workflow-state-guard",
        |p| workflow_state::run(p.value()),
        &[
            "LIFECYCLE_KIT_STAGE_SESSION_TYPES",
            "LIFECYCLE_KIT_STATE_FILE",
            "LIFECYCLE_KIT_STAGES",
            "GATE_SDK_TMP_DIR",
            "GATE_SDK_NATIVE_BIN",
        ],
        "lifecycle-kit",
        false,
        binding::EVENT_BEFORE_TOOL_CALL,
        &[binding::TOOL_WRITE, binding::TOOL_EDIT],
    ),
];

// spec: gate-sdk/SPEC.md §The non-gate arm — the sentinel `--hook`'s own declared roster carries,
// standing for one member's knobs where the arm's argv names a member and for the union over the
// table where it does not
pub const EVERY_HOOK_KNOB: &str = "@every-hook-knob";

pub fn members() -> Vec<&'static str> {
    HOOKS.iter().map(|r| r.0).collect()
}

pub fn owner(member: &str) -> Option<&'static str> {
    HOOKS.iter().find(|r| r.0 == member).map(|r| r.3)
}

// spec: context-kit/SPEC.md §check-settings-paths — a hook's tokens: `command` split on ASCII
// whitespace, or, in the exec form, `command` followed by each `args` element verbatim
pub fn command_tokens(hook: &Value) -> Vec<&str> {
    let cmd = hook.get(binding::COMMAND).and_then(Value::as_str).unwrap_or("");
    match hook.get(binding::WIRING_ARGS).and_then(Value::as_array) {
        Some(args) => std::iter::once(cmd)
            .chain(args.iter().filter_map(Value::as_str))
            .collect(),
        None => cmd.split_ascii_whitespace().collect(),
    }
}

// spec: context-kit/SPEC.md §check-settings-paths — the command token is not always argv[0]: a
// command may lead with `env NAME=VALUE ...` before the interpreter, and a `bash`/`sh` interpreter
// word is then skipped too. One walk for the grant reader, the hook reader and the enforcement map.
pub fn command_index(tok: &[&str]) -> Option<usize> {
    let mut i = 0usize;
    if tok.first() == Some(&"env") {
        i = 1;
        while i < tok.len() && is_assignment(tok[i]) {
            i += 1;
        }
    }
    if matches!(tok.get(i), Some(&"bash") | Some(&"sh")) {
        i += 1;
    }
    (i < tok.len()).then_some(i)
}

fn is_assignment(t: &str) -> bool {
    let Some(at) = t.find('=') else { return false };
    let b = t.as_bytes();
    at > 0 && (b[0].is_ascii_alphabetic() || b[0] == b'_')
}

pub const FRONT_ENDS: [&str; 2] = ["run-gates.sh", "run-gates.ps1"];

// spec: guard-kit/SPEC.md §The recommended allowlist — the binary's placeholder, the spelling a kit
// template carries before the adopter's merge
pub const BIN_PLACEHOLDER: &str = "@GATE_SDK_NATIVE_BIN@";

// spec: gate-sdk/SPEC.md §The harness-integration arm — the Windows executable suffix, which a
// committed registration may carry whatever host reads it
const WINDOWS_EXE: &str = ".exe";

#[derive(Debug, PartialEq)]
pub enum Registration<'a> {
    NotAMember,
    Member {
        name: &'a str,
        observe: bool,
        binding: Option<&'a str>,
        knobs: Option<&'a str>,
    },
    Malformed {
        name: &'a str,
        token: &'a str,
    },
    NoOperand,
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the arm's grammar, printed by every refusal
// of a token outside it
pub const GRAMMAR: &str = "--hook <member> [--observe] [--binding <name>] [--knobs <absolute file>]";
pub const OBSERVE_FLAG: &str = "--observe";
pub const BINDING_FLAG: &str = "--binding";
pub const KNOBS_FLAG: &str = "--knobs";

#[derive(Debug, PartialEq)]
pub struct Tail<'a> {
    pub observe: bool,
    pub binding: Option<&'a str>,
    pub knobs: Option<&'a str>,
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the one parser of the tokens after the
// member, for the arm and every registration reader: each optional token in order, at most once,
// and the first token past them is the refusal's. An operand-taking token naming nothing reads empty.
pub fn tail<'a>(rest: &[&'a str]) -> Result<Tail<'a>, &'a str> {
    let observe = rest.first() == Some(&OBSERVE_FLAG);
    let mut i = usize::from(observe);
    let mut operand = |flag: &str| {
        if rest.get(i) != Some(&flag) {
            return None;
        }
        let v = rest.get(i + 1).copied().unwrap_or("");
        i += 2;
        Some(v)
    };
    let binding = operand(BINDING_FLAG);
    let knobs = operand(KNOBS_FLAG);
    match rest.get(i) {
        Some(token) => Err(token),
        None => Ok(Tail { observe, binding, knobs }),
    }
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the members a `--knobs` file can serve,
// derived from the table's tree-free column
pub fn tree_free_members() -> Vec<&'static str> {
    HOOKS.iter().filter(|r| r.4).map(|r| r.0).collect()
}

pub fn knobs(member: &str) -> Option<&'static [&'static str]> {
    HOOKS.iter().find(|r| r.0 == member).map(|r| r.2)
}

pub fn event(member: &str) -> Option<&'static str> {
    HOOKS.iter().find(|r| r.0 == member).map(|r| r.5)
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the seventh column in the settings file's
// spelling, its tools joined by `|`, empty for an event that takes no matcher
pub fn matcher(member: &str) -> Option<String> {
    HOOKS.iter().find(|r| r.0 == member).map(|r| r.6.join("|"))
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a registration's argv ends where the shell
// syntax its command carries begins, a redirection, a pipe, a list operator or a comment
fn shell_syntax(token: &str) -> bool {
    token.starts_with(['#', ')']) || token.contains([';', '|', '&', '<', '>'])
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the three command-token spellings: a front
// end's, the binary's own package name with or without an executable suffix, and its placeholder
fn names_the_arm(base: &str) -> bool {
    let bin = env!("CARGO_PKG_NAME");
    FRONT_ENDS.contains(&base)
        || base == BIN_PLACEHOLDER
        || base == bin
        || base.strip_suffix(WINDOWS_EXE) == Some(bin)
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the registration grammar's one parser: a
// command token whose file name is one of the three spellings, `--hook`, the member, then its tail
pub fn registration<'a>(tok: &[&'a str]) -> Registration<'a> {
    let Some(i) = command_index(tok) else {
        return Registration::NotAMember;
    };
    let base = tok[i]
        .trim_matches(|c| c == '"' || c == '\'')
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("");
    if !names_the_arm(base) || tok.get(i + 1) != Some(&"--hook") {
        return Registration::NotAMember;
    }
    let Some(&name) = tok.get(i + 2) else {
        return Registration::NoOperand;
    };
    let rest: Vec<&'a str> = tok[i + 3..].iter().copied().take_while(|t| !shell_syntax(t)).collect();
    match tail(&rest) {
        Ok(Tail { knobs: Some(_), .. }) if HOOKS.iter().any(|r| r.0 == name && !r.4) => {
            Registration::Malformed { name, token: KNOBS_FLAG }
        }
        Ok(Tail { observe, binding, knobs }) => Registration::Member { name, observe, binding, knobs },
        Err(token) => Registration::Malformed { name, token },
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — stdout is the hook-JSON envelope, serialized rather
// than composed by hand: this retires `agent-dispatch-guard.sh`'s degraded arm, which kept its
// advisory literals free of any character JSON must escape by convention alone.
pub fn advise(msg: &str) -> i32 {
    println!("{}", advise_envelope(msg));
    0
}

pub fn advise_envelope(msg: &str) -> String {
    binding::context_envelope(msg)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the escaper the hand-written envelope never had. The
// envelope's own braces stay a literal so the key order matches the shell guard's original advise
// envelope, which a JSON object does not carry but a reader diffing two substrates' output does.
pub fn quote(s: &str) -> String {
    serde_json::Value::String(s.to_string()).to_string()
}

// spec: gate-sdk/SPEC.md §The non-gate arm — stderr is the member's block text, which the harness
// shows, and exit 2 is the harness's block signal; the `<name>: ` prefix matches the shell guard's.
pub fn block(name: &str, msg: &str) -> i32 {
    eprintln!("{}: {}", name, msg);
    exit::HOOK_BLOCK
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the member's own face of the fail-open
// rule: a guard that cannot run declines rather than wedging the session. The front-end holds the
// cause it can see; this holds the one only the member can, an unresolvable declared knob.
pub fn decline(name: &str, reason: &str, payload: Option<&Value>) -> i32 {
    let text = format!(
        "{}: {} — the rule could not be enforced on this call and the call was allowed.",
        name, reason
    );
    println!("{}", decline_envelope(&text, payload));
    eprintln!("{}", text);
    0
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the firing event's envelope: the advise
// envelope reaches the model on `PreToolUse`; elsewhere the universal `systemMessage` reaches the
// operator, a turn-end event having no model channel that does not refuse the stop
pub fn decline_envelope(text: &str, payload: Option<&Value>) -> String {
    if field(payload, &[binding::EVENT_NAME]) == binding::EVENT_BEFORE_TOOL_CALL {
        advise_envelope(text)
    } else {
        binding::notice_envelope(text)
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — one field of the payload by object path, rendered as
// `jq -r '<path> // ""'` renders it: a string bare, a number by its own spelling, and null, false
// or absent as empty, `//` being an alternative operator that fires on false too.
pub fn field(payload: Option<&Value>, path: &[&str]) -> String {
    let mut cur = match payload {
        Some(v) => v,
        None => return String::new(),
    };
    for step in path {
        match cur.get(step) {
            Some(next) => cur = next,
            None => return String::new(),
        }
    }
    match cur {
        Value::String(s) => s.clone(),
        Value::Null | Value::Bool(false) => String::new(),
        other => other.to_string(),
    }
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the decline for a tree with no repository
// the arm may read: the front end's notice for that case, a literal with no interpolated value
const NO_TREE_NOTICE: &str = "checkwright-gates: git refuses or skips the repository this session stands in, or answers none, so every hook guard in this tree is off and each guarded call is allowed. git status prints its reason, or git --git-dir=<that .git> status for a skipped one; where git answers none, run the session inside a repository, or register a tree-free member";

thread_local! {
    static TREE_FREE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static KNOB_FILE: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
    static KNOB_FAULT: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

// spec: gate-sdk/SPEC.md §The knob file — the `--knobs` operand, the only file layer while it is set
pub fn knob_file() -> Option<String> {
    KNOB_FILE.with(|k| k.borrow().clone())
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the fault of a `--knobs` file the arm cannot
// use, carried to the member, which takes it through its own route at its first act
pub fn knob_fault() -> Option<String> {
    KNOB_FAULT.with(|k| k.borrow().clone())
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a fail-open member's route for the carried
// fault: the decline, naming `--knobs`, the file and the line at fault
pub fn decline_on_fault(name: &str, payload: Option<&Value>) -> Option<i32> {
    knob_fault().map(|fault| decline(name, &fault, payload))
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the tree-free mark: set where a tree-free
// member runs in place with no repository, read by the knob layers and the member's path checks
pub fn tree_free() -> bool {
    TREE_FREE.with(std::cell::Cell::get)
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the observe mark an observed run sets in its
// child's environment, read by the one member write a trial skips
pub const OBSERVED_VAR: &str = "CHECKWRIGHT_HOOK_OBSERVED";

pub fn observed() -> bool {
    std::env::var_os(OBSERVED_VAR).is_some_and(|v| !v.is_empty())
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — the two peer arms raise the mark for
// themselves where no repository answers
pub(crate) fn raise_tree_free() {
    TREE_FREE.with(|m| m.set(true));
}

#[cfg(test)]
pub fn set_tree_free(on: bool) {
    TREE_FREE.with(|m| m.set(on));
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a member's case under the tree-free mark:
// every kit knob the environment carries held aside, the `--knobs` file this case writes, or none,
// the layer, and the carried fault, each restored when the case drops it
#[cfg(test)]
pub struct Marked {
    env: crate::knobenv::KnobEnv,
    held: Vec<(String, String)>,
    file: Option<std::path::PathBuf>,
}

#[cfg(test)]
impl Marked {
    pub fn new(tag: &str, knob_text: Option<&str>) -> Marked {
        let env = crate::knobenv::lock();
        let held: Vec<(String, String)> =
            std::env::vars().filter(|(n, _)| crate::knobs::owner(n).is_some()).collect();
        for (n, _) in &held {
            env.remove(n);
        }
        let file = knob_text.map(|text| {
            let f = std::env::temp_dir().join(format!("checkwright-marked.{}.{}.knobs", tag, std::process::id()));
            std::fs::write(&f, text).expect("the case's knob file must be writable");
            f
        });
        TREE_FREE.with(|m| m.set(true));
        KNOB_FILE.with(|k| *k.borrow_mut() = file.as_ref().map(|f| f.display().to_string()));
        crate::knobs::reset(&env);
        Marked { env, held, file }
    }

    pub fn set(&self, name: &str, value: &str) {
        self.env.set(name, value);
        crate::knobs::reset(&self.env);
    }

    pub fn carry(&self, fault: &str) {
        KNOB_FAULT.with(|k| *k.borrow_mut() = Some(fault.to_string()));
    }
}

#[cfg(test)]
impl Drop for Marked {
    fn drop(&mut self) {
        TREE_FREE.with(|m| m.set(false));
        KNOB_FILE.with(|k| *k.borrow_mut() = None);
        KNOB_FAULT.with(|k| *k.borrow_mut() = None);
        for (n, _) in std::env::vars().filter(|(n, _)| crate::knobs::owner(n).is_some()) {
            self.env.remove(&n);
        }
        for (n, v) in &self.held {
            self.env.set(n, v);
        }
        crate::knobs::reset(&self.env);
        if let Some(f) = &self.file {
            let _ = std::fs::remove_file(f);
        }
    }
}

// spec: installer/SPEC.md §The user install — each tree-free member's knob reads as its run takes
// them, the fault it would carry through its own route, for the case firing it under its file
#[cfg(test)]
pub fn knob_reads(member: &str) -> Result<(), String> {
    if let Some(fault) = knob_fault() {
        return Err(fault);
    }
    match member {
        "subagent-stop-liveness" => stop_liveness::inputs().map(drop),
        "escalation-guard" => Ok(()),
        "wakeup-guard" => wakeup::record_target().map(drop),
        "agent-budget-guard" => verdict::config_fault().map_or(Ok(()), Err),
        "agent-dispatch-guard" => dispatch::tier_inputs().map(drop),
        other => panic!("tree-free member {} names no knob read in this function", other),
    }
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a path-valued knob under the mark must be
// absolute; a relative value, a default included, is the member's decline reason
pub fn tree_free_path(name: &str, value: &str) -> Result<(), String> {
    if !tree_free() || crate::walk::host_absolute(value) {
        return Ok(());
    }
    Err(format!(
        "{} is '{}', and a run with no repository takes absolute paths only — set it to an absolute path",
        name, value
    ))
}

fn no_tree(line: &str) -> i32 {
    eprintln!("{}", line);
    println!("{}", binding::notice_envelope(NO_TREE_NOTICE));
    0
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's dispatch: the member name is argv, never the
// hook event name, so the port is a one-for-one substitution of a `command` value and every
// `matcher` in the consumer's settings is left untouched.
pub fn run(args: &[String]) -> i32 {
    let row = args.first().and_then(|m| HOOKS.iter().find(|r| r.0 == m));
    let rest: Vec<&str> = args.get(1..).unwrap_or_default().iter().map(String::as_str).collect();
    let parsed = tail(&rest);
    if let (Some(member), Ok(Tail { observe, binding, knobs: Some(file) })) = (args.first(), &parsed) {
        return under_knobs(member, row, *observe, *binding, file);
    }
    match crate::walk::toplevel_opt() {
        Ok(Some(top)) => {
            // spec: gate-sdk/SPEC.md §Fail-closed contract — cwd-write-exempt: the arm's own process
            // enters the tree it guards, and no test reaches `run()`
            if let Err(e) = std::env::set_current_dir(&top) {
                return no_tree(&format!("checkwright-gates: cannot enter the work tree at {}: {}", top, e));
            }
        }
        Ok(None) => match (row, &parsed) {
            (Some(&(name, f, _, _, true, _, _)), Ok(Tail { observe, binding: None, knobs: None })) => {
                TREE_FREE.with(|m| m.set(true));
                return if *observe { observe::run(name, &read_input()) } else { f(&read_input()) };
            }
            _ => return no_tree("checkwright-gates: not inside a git repository"),
        },
        Err(e) => return no_tree(&format!("checkwright-gates: {}", e)),
    }
    let Some(member) = args.first() else {
        eprintln!("checkwright-gates: --hook needs a member name — the hook could not run");
        eprintln!("  help: this binary carries: {}", members().join(", "));
        return exit::HOOK_BLOCK;
    };
    let Some(&(_, f, _, _, _, _, _)) = row else {
        return no_member(member);
    };
    let Tail { observe, binding, .. } = match parsed {
        Ok(t) => t,
        Err(token) => {
            eprintln!(
                "checkwright-gates: token outside the --hook grammar: {} — the hook could not run",
                token
            );
            eprintln!("  help: the grammar: {}", GRAMMAR);
            return exit::HOOK_BLOCK;
        }
    };
    match binding_of(binding) {
        Ok(None) if observe => observe::run(member, &read_input()),
        Ok(None) => f(&read_input()),
        Ok(Some(_)) => antigravity::run(member, observe, read_input().bytes()),
        Err(given) => no_binding(&given),
    }
}

fn no_member(member: &str) -> i32 {
    eprintln!("checkwright-gates: no such hook member: {} — the hook could not run", member);
    eprintln!("  help: this binary carries: {}", members().join(", "));
    exit::HOOK_BLOCK
}

fn no_binding(given: &str) -> i32 {
    eprintln!("checkwright-gates: no such hook binding: {} — the hook could not run", given);
    eprintln!("  help: this binary carries: {}", BINDINGS.join(", "));
    exit::HOOK_BLOCK
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — `--knobs` puts the arm under the tree-free
// mark wherever it stands, and a file it cannot use is a fault carried to the member; an observed
// run has no member in process to carry it to, so it declines as an unwritable log does
fn under_knobs(member: &str, row: Option<&Row>, observe: bool, binding: Option<&str>, file: &str) -> i32 {
    let Some(&(name, f, _, _, free, _, _)) = row else {
        return no_member(member);
    };
    if !free {
        eprintln!(
            "checkwright-gates: token outside the --hook grammar for a member that reads a repository: {} \
             on {} — the hook could not run",
            KNOBS_FLAG, name
        );
        eprintln!("  help: {} serves the tree-free members only: {}", KNOBS_FLAG, tree_free_members().join(", "));
        return exit::HOOK_BLOCK;
    }
    if let Err(given) = binding_of(binding) {
        return no_binding(&given);
    }
    let input = read_input();
    if binding.is_some() {
        return decline(
            name,
            "a bound call keeps the decline until a binding is shown to run tree-free",
            input.value(),
        );
    }
    TREE_FREE.with(|m| m.set(true));
    KNOB_FILE.with(|k| *k.borrow_mut() = Some(file.to_string()));
    let fault = crate::knobs::check_hook_file(file).err().map(|e| format!("{} {}", KNOBS_FLAG, e));
    match fault {
        Some(fault) if observe => decline(name, &fault, input.value()),
        _ if observe => observe::run(name, &input),
        fault => {
            KNOB_FAULT.with(|k| *k.borrow_mut() = fault);
            f(&input)
        }
    }
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the foreign bindings the arm's adapter
// speaks, a closed set
pub const BINDINGS: &[&str] = &[antigravity::NAME];

// spec: gate-sdk/SPEC.md §The harness-integration arm — absent is the shipped binding, and a
// `--binding` naming nothing in the set, or nothing at all, is refused naming what was given
fn binding_of(given: Option<&str>) -> Result<Option<&'static str>, String> {
    let Some(given) = given else { return Ok(None) };
    BINDINGS
        .iter()
        .find(|b| **b == given)
        .map(|b| Some(*b))
        .ok_or_else(|| if given.is_empty() { "(none named)".to_string() } else { given.to_string() })
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the payload read whole and parsed once; a read error,
// an empty body and a body that is not JSON collapse to `None`, the member's degraded input.
pub fn read_input() -> Payload {
    use std::io::Read;
    let mut bytes = Vec::new();
    if std::io::stdin().read_to_end(&mut bytes).is_err() {
        return Payload { bytes: Vec::new(), value: None };
    }
    let value = if bytes.iter().all(u8::is_ascii_whitespace) {
        None
    } else {
        serde_json::from_slice(&bytes).ok()
    };
    Payload { bytes, value }
}

pub fn read_payload() -> Option<Value> {
    read_input().value
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the UTC stamp the log line carries,
// computed rather than spawned: `date -u` asks no question about the operator's zone.
pub fn utc_stamp(epoch: i64) -> String {
    let days = epoch.div_euclid(86_400);
    let secs = epoch.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y,
        m,
        d,
        secs / 3600,
        (secs % 3600) / 60,
        secs % 60
    )
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — Howard Hinnant's civil-from-days, the
// inverse of `emit::trajectory`'s days-from-civil, which is the crate's existing half of this pair.
pub(crate) fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The non-gate arm — the table is the single roster, so a member
    // resolves only under its own name and the roster the refusal prints is that same table
    #[test]
    fn a_member_resolves_only_under_its_own_name() {
        assert!(members().contains(&"agent-budget-guard"));
        assert!(!members().contains(&"agent-budget-guards"));
        assert!(!members().contains(&"PreToolUse"));
        assert_eq!(members().len(), HOOKS.len());
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a decline speaks through the firing
    // event's envelope and never through a verdict: exit 0, and the event read off the payload
    #[test]
    fn a_decline_writes_the_firing_events_envelope() {
        let doc = |ev: &str| -> Value {
            serde_json::from_str(&format!(r#"{{"hook_event_name":"{}"}}"#, ev)).unwrap()
        };
        let pre = doc("PreToolUse");
        let adv: Value =
            serde_json::from_str(&decline_envelope("m: \"k\" unset", Some(&pre))).unwrap();
        assert_eq!(adv["hookSpecificOutput"]["hookEventName"], "PreToolUse");
        assert_eq!(adv["hookSpecificOutput"]["additionalContext"], "m: \"k\" unset");
        assert!(adv.get("systemMessage").is_none());
        for p in [Some(&doc("SubagentStop")), None] {
            let sys: Value = serde_json::from_str(&decline_envelope("t", p)).unwrap();
            assert_eq!(sys, serde_json::json!({"systemMessage": "t"}));
        }
        assert_eq!(decline("m", "r", None), 0);
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — every row weighed: five members read
    // nothing a repository holds, and three read the tree, each row's comment saying why
    #[test]
    fn five_members_are_tree_free_and_three_read_the_tree() {
        let mut free = tree_free_members();
        free.sort_unstable();
        assert_eq!(
            free,
            ["agent-budget-guard", "agent-dispatch-guard", "escalation-guard", "subagent-stop-liveness", "wakeup-guard"]
        );
        let mut bound: Vec<&str> = HOOKS.iter().filter(|r| !r.4).map(|r| r.0).collect();
        bound.sort_unstable();
        assert_eq!(bound, ["session-context", "shell-guard", "workflow-state-guard"]);
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a before-tool-call row names the tools it
    // registers on, and a row on an event that takes no matcher names none
    #[test]
    fn every_before_tool_call_row_carries_a_matcher() {
        for (name, _, _, _, _, event, tools) in HOOKS {
            assert_eq!(*event == binding::EVENT_BEFORE_TOOL_CALL, !tools.is_empty(), "{}'s matcher", name);
        }
        assert_eq!(matcher("wakeup-guard").as_deref(), Some("ScheduleWakeup|CronCreate"));
        assert_eq!(matcher("subagent-stop-liveness").as_deref(), Some(""));
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — the project-scope wiring guard-kit ships
    // registers each member it carries under that member's matcher
    #[test]
    fn the_guard_kit_wiring_template_registers_each_member_under_its_matcher() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../guard-kit/templates/settings-hooks.json");
        let text = std::fs::read_to_string(&path).expect("guard-kit's wiring template must read");
        let doc: Value = serde_json::from_str(&text).expect("guard-kit's wiring template must parse");
        let mut seen = 0;
        for (event, groups) in doc[binding::WIRING_HOOKS].as_object().expect("the template's hooks object") {
            for group in groups.as_array().into_iter().flatten() {
                let on = group.get(binding::WIRING_MATCHER).and_then(Value::as_str).unwrap_or("");
                for hook in group[binding::WIRING_HOOKS].as_array().into_iter().flatten() {
                    let Registration::Member { name, .. } = registration(&command_tokens(hook)) else { continue };
                    assert_eq!(event.as_str(), event_of(name), "{}'s event", name);
                    assert_eq!(Some(on.to_string()), matcher(name), "{}'s matcher", name);
                    seen += 1;
                }
            }
        }
        assert!(seen > 0, "the template registers no member");
    }

    fn event_of(name: &str) -> &'static str {
        event(name).unwrap_or_else(|| panic!("{} is no member", name))
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — under the mark a relative path-valued
    // knob is the decline reason, naming the knob and its value, a context-dependent spelling
    // relative; with no mark any value passes
    #[test]
    fn a_relative_path_knob_is_refused_under_the_mark_alone() {
        let (host, foreign) = if cfg!(windows) { ("C:/var/scratch", "/var/scratch") } else { ("/var/scratch", "C:/var/scratch") };
        assert_eq!(tree_free_path("GATE_SDK_TMP_DIR", ".tmp"), Ok(()));
        set_tree_free(true);
        let relative = tree_free_path("GATE_SDK_TMP_DIR", ".tmp");
        let absolute = tree_free_path("GATE_SDK_TMP_DIR", host);
        let context = [foreign, "\\var\\scratch"].map(|v| tree_free_path("GATE_SDK_TMP_DIR", v));
        set_tree_free(false);
        let e = relative.expect_err("a relative value under the mark");
        assert!(e.contains("GATE_SDK_TMP_DIR is '.tmp'") && e.contains("absolute paths only"), "{}", e);
        assert_eq!(absolute, Ok(()));
        for r in context {
            assert!(r.is_err_and(|e| e.contains("absolute paths only")), "a context-dependent spelling passed");
        }
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — the peer arms' raise is the mark every
    // knob layer reads; the stray `scripts/` case it closes is driven end to end by the kit's gate
    // test, since a unit test cannot move the process's working directory
    #[test]
    fn the_peer_arms_raise_is_the_mark() {
        assert!(!tree_free());
        raise_tree_free();
        let raised = tree_free();
        set_tree_free(false);
        assert!(raised);
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — the owner column is checked against the
    // member module rather than transcribed: each row's owner is the kit its module's leading
    // `spec:` binding names, and a member missing from this module list fails rather than passes
    #[test]
    fn each_members_owner_is_the_kit_its_module_binds_to() {
        let modules: &[(&str, &str)] = &[
            ("agent-budget-guard", include_str!("budget.rs")),
            ("agent-dispatch-guard", include_str!("dispatch.rs")),
            ("subagent-stop-liveness", include_str!("stop_liveness.rs")),
            ("escalation-guard", include_str!("escalation.rs")),
            ("session-context", include_str!("session_context.rs")),
            ("shell-guard", include_str!("shell_guard.rs")),
            ("wakeup-guard", include_str!("wakeup.rs")),
            ("workflow-state-guard", include_str!("workflow_state.rs")),
        ];
        for (name, _, _, owner, _, _, _) in HOOKS {
            let src = modules
                .iter()
                .find(|(m, _)| m == name)
                .unwrap_or_else(|| panic!("member {} names no module in this test", name))
                .1;
            let kit = src
                .lines()
                .next()
                .and_then(|l| l.strip_prefix("// spec: "))
                .and_then(|l| l.split('/').next())
                .unwrap_or_else(|| panic!("member {}'s module leads with no spec: binding", name));
            assert_eq!(kit, *owner, "member {}'s owner", name);
        }
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — the parser every reader of a
    // registration calls: a front end, the binary or its placeholder as the command token, then
    // `--hook`; a renamed binary gets no member read
    #[test]
    fn a_registration_names_its_member_through_each_command_spelling() {
        let exec = |json: &str| -> Value { serde_json::from_str(json).unwrap() };
        let bin = env!("CARGO_PKG_NAME");
        for command in [
            format!("${{CLAUDE_PROJECT_DIR}}/native/target/release/{}", bin),
            format!("C:/tools/{}.exe", bin),
            format!("${{CLAUDE_PROJECT_DIR}}/{}", BIN_PLACEHOLDER),
        ] {
            let hook = exec(&format!(r#"{{"command":"{}","args":["--hook","wakeup-guard"]}}"#, command));
            assert_eq!(
                registration(&command_tokens(&hook)),
                member("wakeup-guard"),
                "{}",
                command
            );
        }
        let renamed =
            exec(r#"{"command":"${CLAUDE_PROJECT_DIR}/bin/my-gates","args":["--hook","wakeup-guard"]}"#);
        assert_eq!(registration(&command_tokens(&renamed)), Registration::NotAMember);
        let bare = exec(&format!(r#"{{"command":"bin/{}","args":["--hook"]}}"#, bin));
        assert_eq!(registration(&command_tokens(&bare)), Registration::NoOperand);
        let t = |s: &'static str| s.split_ascii_whitespace().collect::<Vec<_>>();
        assert_eq!(
            registration(&t("bash gate-sdk/bin/run-gates.sh --hook shell-guard")),
            member("shell-guard")
        );
        assert_eq!(
            registration(&t("env A=1 bash \"${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh\" --hook x")),
            member("x")
        );
        assert_eq!(
            registration(&t("bash gate-sdk/bin/run-gates.sh --hook")),
            Registration::NoOperand
        );
        assert_eq!(
            registration(&t("bash \"$r/gate-sdk/bin/run-gates.sh\" --hook shell-guard --binding antigravity")),
            Registration::Member { name: "shell-guard", observe: false, binding: Some("antigravity"), knobs: None }
        );
        assert_eq!(
            registration(&t("bash gate-sdk/bin/run-gates.sh --run")),
            Registration::NotAMember
        );
        assert_eq!(
            registration(&t("bash scripts/x.sh --hook shell-guard")),
            Registration::NotAMember
        );
        assert_eq!(registration(&t("")), Registration::NotAMember);
        let exec: Value = serde_json::from_str(
            r#"{"command":"bash","args":["gate-sdk/bin/run-gates.sh","--hook","a b"]}"#,
        )
        .unwrap();
        assert_eq!(registration(&command_tokens(&exec)), member("a b"));
    }

    fn member(name: &str) -> Registration<'_> {
        Registration::Member { name, observe: false, binding: None, knobs: None }
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a registration's tail is the arm's
    // grammar: the observe flag, the binding, a token outside both, and the shell syntax after argv
    #[test]
    fn a_registrations_tail_reads_through_the_arms_grammar() {
        let read = |tail: &str| -> Vec<String> {
            format!("bash gate-sdk/bin/run-gates.sh --hook shell-guard {}", tail)
                .split_ascii_whitespace()
                .map(String::from)
                .collect()
        };
        let reg = |toks: &[String]| -> String {
            format!("{:?}", registration(&toks.iter().map(String::as_str).collect::<Vec<_>>()))
        };
        let shown = |r: Registration| format!("{:?}", r);
        assert_eq!(
            reg(&read("--observe")),
            shown(Registration::Member { name: "shell-guard", observe: true, binding: None, knobs: None })
        );
        assert_eq!(
            reg(&read("--observe --binding antigravity")),
            shown(Registration::Member { name: "shell-guard", observe: true, binding: Some("antigravity"), knobs: None })
        );
        assert_eq!(
            reg(&read("--obsrve")),
            shown(Registration::Malformed { name: "shell-guard", token: "--obsrve" })
        );
        assert_eq!(
            reg(&read("--binding antigravity --observe")),
            shown(Registration::Malformed { name: "shell-guard", token: "--observe" })
        );
        assert_eq!(
            reg(&read("--observe 2>/dev/null || true")),
            shown(Registration::Member { name: "shell-guard", observe: true, binding: None, knobs: None })
        );
        assert_eq!(reg(&read("; echo x")), shown(member("shell-guard")));
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — no token is the shipped binding, a set
    // member is its adapter, and an unknown or missing name is refused naming what was given
    #[test]
    fn the_binding_token_selects_an_adapter_from_the_closed_set() {
        let of = |s: &[&'static str]| tail(s).map(|t| binding_of(t.binding));
        assert_eq!(of(&[]), Ok(Ok(None)));
        assert_eq!(of(&["--binding", "antigravity"]), Ok(Ok(Some("antigravity"))));
        assert_eq!(of(&["--binding", "cursor"]), Ok(Err("cursor".to_string())));
        assert_eq!(of(&["--binding"]), Ok(Err("(none named)".to_string())));
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — each optional token in its place, at
    // most once; the first token past them is the one the refusal names
    #[test]
    fn the_tail_takes_each_token_once_and_in_order() {
        assert_eq!(tail(&["--observe"]), Ok(Tail { observe: true, binding: None, knobs: None }));
        assert_eq!(
            tail(&["--observe", "--binding", "antigravity"]),
            Ok(Tail { observe: true, binding: Some("antigravity"), knobs: None })
        );
        assert_eq!(tail(&["--observe", "--observe"]), Err("--observe"));
        assert_eq!(tail(&["--binding", "antigravity", "--observe"]), Err("--observe"));
        assert_eq!(tail(&["--verbose"]), Err("--verbose"));
        assert_eq!(
            tail(&["--observe", "--binding", "antigravity", "--knobs", "/k"]),
            Ok(Tail { observe: true, binding: Some("antigravity"), knobs: Some("/k") })
        );
        assert_eq!(tail(&["--knobs", "/k"]), Ok(Tail { observe: false, binding: None, knobs: Some("/k") }));
        assert_eq!(tail(&["--knobs", "/k", "--observe"]), Err("--observe"));
        assert_eq!(tail(&["--knobs", "/k", "--binding", "antigravity"]), Err("--binding"));
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a registration carries its `--knobs`
    // path, and `--knobs` on a member that reads a repository is outside the grammar
    #[test]
    fn a_registration_carries_its_knob_file_on_a_tree_free_member_alone() {
        let t = |s: &'static str| s.split_ascii_whitespace().collect::<Vec<_>>();
        assert_eq!(
            registration(&t("/u/bin/checkwright-gates --hook subagent-stop-liveness --observe --knobs /u/k")),
            Registration::Member { name: "subagent-stop-liveness", observe: true, binding: None, knobs: Some("/u/k") }
        );
        assert_eq!(
            registration(&t("/u/bin/checkwright-gates --hook shell-guard --knobs /u/k")),
            Registration::Malformed { name: "shell-guard", token: KNOBS_FLAG }
        );
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — every tree-free row registers under an
    // event the binding table names, the one a printed registration takes
    #[test]
    fn every_tree_free_row_carries_a_binding_event() {
        let events = [binding::EVENT_BEFORE_TOOL_CALL, binding::EVENT_SESSION_START, binding::EVENT_CHILD_TURN_END];
        for name in tree_free_members() {
            assert!(events.contains(&event_of(name)), "{}", name);
        }
        assert_eq!(event("session-context"), Some(binding::EVENT_SESSION_START));
        assert_eq!(event("no-such-member"), None);
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the computed stamp is the one
    // `date -u +%Y-%m-%dT%H:%M:%SZ` prints, checked at an epoch whose civil value is known
    #[test]
    fn the_utc_stamp_is_the_shell_form_of_the_same_instant() {
        assert_eq!(utc_stamp(0), "1970-01-01T00:00:00Z");
        assert_eq!(utc_stamp(1_756_656_000), "2025-08-31T16:00:00Z");
        assert_eq!(utc_stamp(1_709_164_800), "2024-02-29T00:00:00Z");
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — an absent payload and a payload missing the field
    // are one condition for a reader, which is what makes the degraded path a single branch
    #[test]
    fn a_missing_field_and_a_missing_payload_read_alike() {
        let doc: Value = serde_json::from_str(r#"{"tool_input":{"to":"main"},"n":1}"#)
            .expect("the fixture must parse");
        assert_eq!(field(Some(&doc), &["tool_input", "to"]), "main");
        assert_eq!(field(Some(&doc), &["tool_input", "message"]), "");
        assert_eq!(field(None, &["tool_input", "to"]), "");
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — a number reads as its own spelling, never as
    // empty: the harness sends `used_percentage` as a number, and a reader that took only strings
    // rendered every gauge at zero while a string-spelled fixture agreed with it
    #[test]
    fn a_number_field_reads_as_jq_renders_it() {
        let doc: Value = serde_json::from_str(
            r#"{"context_window":{"used_percentage":42.7},"n":1,"t":true,"f":false,"z":null}"#,
        )
        .expect("the fixture must parse");
        assert_eq!(field(Some(&doc), &["context_window", "used_percentage"]), "42.7");
        assert_eq!(field(Some(&doc), &["n"]), "1");
        assert_eq!(field(Some(&doc), &["t"]), "true");
        assert_eq!(field(Some(&doc), &["f"]), "", "jq's // fires on false too");
        assert_eq!(field(Some(&doc), &["z"]), "");
    }
}
