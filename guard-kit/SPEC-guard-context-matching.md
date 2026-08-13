# SPEC amendment: guard-context-matching

Two queue entries, one surface, one root: **`guard-glyph-match-context-blind`**
and **`exit-echo-decoration-guard-vs-habit`**. Both are paired to this file
rather than to one amendment each, because neither is a bug in the rule that
fires it. Both are the absence of one principled lexical view of the command,
and fixing them separately would build that view twice or, worse, once and a
half.

This amendment designs the primitive and the rule changes that consume it. It
does not restate the triage criterion (§The triage criterion), the primitive
contracts (§The guard framework), or the ordering rationale each rule already
carries (§The generic ruleset). It deliberately does **not** touch
`guard-command-prefix-wrapper`, which the operator ruled out of this iteration:
transparent command prefixes are a *grant* question about a different segment
position, and nothing below widens toward it.

**The premises were probed by running the guard, not by reading the entries.**
Three of the two entries' stated grounds did not survive. Correcting them is
part of the design, because a fix built on the entry's mechanism would fix
something the tree does not do.

## What the probes found

### The glyph entry is confirmed, and wider than it says

Its mechanism claim is exact. `guard_rule_brace_glyph` strips **single-quoted
spans only** before matching, and `guard_rule_expansion` does the same for its
first check. Executed against `scripts/bash-guard.sh`: a double-quoted POSIX ERE
quantifier in a working `grep` exits 2 with *write out the brace expansion*; the
single-quoted respelling exits 0 and is **auto-allowed** by the read-only
pipeline rule. Two spellings of one semantically identical command sit at
opposite ends of the verdict range. A quoted heredoc body carrying a brace, and
one carrying a command substitution, each exit 2 — and neither rule reads the
heredoc delimiter, which was quoted, so the shell itself guarantees the body
does not expand.

**What the entry understates is `guard_rule_abs_prefix`, which strips
nothing at all** — it greps the raw command for the repo-root prefix. Reproduced
live: a `printf` that only echoes prose, with the path inside single quotes and
never executed, is blocked. For the absolute-path third of the entry **not even
the single-quote escape exists**. Four other rules also match the raw string.

### The decoration entry's grounds are three-quarters false

- **`sort` is in `GUARD_KIT_RO_BINS`**, ninth of twenty-two, and has been since
  before the friction-kit rename — so this was wrong when written, not stale.
  `find … | sort` auto-allows today. The entry's "second contributor" is a
  one-item finding: `xargs`, which really is absent.
- **The newline claim is true of the regex and false of the behavior.**
  `guard_split_compound`'s separator class carries no newline, but it *emits*
  segments as lines and every consumer reads lines, so a newline already present
  in the input is already a boundary before the substitution runs. Executed:
  `ls` and an `xargs` call joined by a newline are segmented and **blocked** by
  the decorated-allowlist rule; a `grep`-then-`ls` newline blob is **auto-allowed**
  by the read-only pipeline rule. The entry's dominant class is substantially
  resolved already.
- **The `; echo EXIT:$?` reasoning is wrong.** The entry argues the `$?` is an
  expansion and therefore cannot be resolved to an allowlist entry. The
  expansion rule's pattern does not match `$?` — nor `$1`, `$@`, `$#`, `$$` —
  so that shape falls through silently and the guard never sees it as an
  expansion at all.

### The sizing moved, and the residue is a different failure mode

`bash guard-kit/bin/scan-prompts.sh` at the authoring rev reports **23 prompting
calls across 15 patterns from 111 logged fall-throughs**, with `bash <script>`
invocations at the top and no read-blob class in the profile at all. The entry's
headline is *roughly 78 of this iteration's 106 prompting calls*, dated
2026-08-06, with grep/find/cat/ls blobs dominant. That evidence is stale by a
large factor, exactly as the newline finding predicts.

What is not stale is friction. The authoring session alone met the guard four
times — an expansion inside a loop, a `sed` read, an absolute path in a
redirect target, and an allowlisted script decorated with a pipe into `head`.
Every one was a **guard block on a benign command**, not a harness prompt. That
is a different failure mode from the one the entry describes, it is the one the
SWOT names as converting enforcement into bypass and distrust, and it is what
the deltas below actually address. The reframing was escalated rather than
taken: this amendment designs the mechanism, and whether the decoration entry's
*asserted behavior* is restated in those terms is not a spec-stage call.

## The seam, ruled

**Mechanism to guard-kit, rule content to the consumer, rosters to config** —
the split the kit already runs on, applied to everything below.

The normalizer, the inert-region classes and every rule change land in
`guard-kit/lib/guard.sh` and `guard-kit/SPEC.md`. They encode **harness and
shell behavior**, which is the kit's stated subject. Nothing in them names a
path, a project tool, or a repo layout.

`scripts/bash-guard.sh` keeps every project rule it has — the hook-bypass block,
the harness-scratchpad path, the `git clean -x` block, the scratch-run steer.
Those are this repo's rule content and stay in the consumer copy, where
`check-template-copy-parity` reads the `# copy-divergence:` declaration that
sanctions them. The copy gains the normalizer's behavior by calling it, not by
carrying a second copy of it.

`xargs` joining `GUARD_KIT_RO_BINS`' default is a **default value change on an
existing kit knob**, not new rule content: the roster's twenty-two members are
read-only POSIX text tools, `xargs` is one, and the knob has been consumer-
overridable since it shipped. No project vocabulary enters the kit, and the
provenance seam is untouched.

## What changes

### The primitive

**(1) `guard_skeleton <cmd> <inert-class>…` — one context-aware normalizer, and
the only place a rule learns what part of a command is live.** [design-bearing]
It returns the command with every region of the named inert classes replaced by
a placeholder token, leaving everything else byte-identical. Three classes,
which is the whole domain:

- **`sq`** — single-quoted spans.
- **`dq`** — double-quoted spans.
- **`hd`** — heredoc bodies, from the line after an opener to its terminator
  line.

Each rule names the classes inert **for it**, in one argument, at one call site.
That is the design's whole content: the classes were never the disagreement —
the five ad-hoc dialects below all agree that *some* regions are inert — the
disagreement was that each rule decided privately and none recorded why.

**Placeholder, never deletion, and this is a correctness point rather than
taste.** Three rules already substitute `SQ`/`DQ` tokens while six delete the
span outright, and deleting **fuses adjacent tokens**: a pattern glued to its
flag loses the boundary between them and the residue reads as one word that was
never in the command. Every deleting site carries that latent misparse today.
The normalizer substitutes, so the skeleton has the same token count and the
same statement structure as the command it models.

**The heredoc class answers the objection the entry raises against it.** The
entry says the cheap approximations are wrong in both directions — blanking
everything after a `<<` blinds the guard to real commands in the same call,
while stripping only single quotes still refuses a double-quoted mention. The
normalizer does neither. It marks the **body extent** inert and leaves the
opener line and everything after the terminator fully live, so a heredoc-bearing
call is still matched on its executable text. That extent is decidable without a
shell parser because the terminator is a literal token alone on its line, which
is the property the class rests on.

**Two bounds are stated rather than discovered later.** A heredoc with an
**unquoted** delimiter does expand, so `hd` is not inert for the expansion rule
even though it is inert for every glyph rule — which is why classes are declared
per rule and not fixed per region. And the normalizer models quoting, not shell
semantics: a construct that survives its scan and is not one of the three
classes is treated as live, which is the fail-toward-matching direction and the
one a guard should err in.

**(2) The five dialects collapse onto it, and the collapse is the fix for both
entries.** [design-bearing] Today one file carries five incompatible
strippings: nothing stripped, single-quotes-deleted, both-classes-deleted,
both-classes-placeheld, and a sixth in the consumer copy. No gate holds them in
agreement and nothing could, because there is no statement of what they should
agree on. After this delta every rule calls the normalizer with its declared
classes and the divergence has no place left to live.

The rules that strip **nothing** are the ones this changes most, and they are
the glyph entry's understated half: the compound-`cd` rule, the `git -C` root
rule, the scratch-redirect rule, the absolute-prefix rule, and the truncate
auto-allow all currently match raw. Each gets `sq dq hd` — none of the five
tests for an expansion, so none has the expansion rule's reason to keep
double-quoted spans live.

**(3) The glyph rule's inherited asymmetry is corrected, with the rationale that
does not transfer.** [design-bearing] §The generic ruleset justifies the
single-quote-only strip **for the expansion rule** and the justification is
sound: inside single quotes `$` is literal, while a double-quoted `"$x"` still
expands and must stay visible. The brace rule inherited the same strip without
restating a rationale, and the rationale does not carry: `{` is a **matcher
glyph, not a shell expansion**, so a double-quoted brace is exactly as inert as
a single-quoted one. The brace rule's classes are `sq dq hd`; the expansion
rule's stay `sq` for its expansion check and `sq dq` for its assignment check,
unchanged and now written down as a deliberate difference rather than an
accident of two adjacent lines.

This is what stops a working POSIX quantifier being refused. It is also what
stops the *wrong corrective*: the guard's own output contract requires a block
message to name the offending pattern **and the corrective form**, and *write
out the brace expansion, spell the members* is inapplicable to a quantifier that
has no members and to a heredoc body that has nothing to respell. A block whose
corrective cannot be followed fails that contract however right its verdict.

### The decoration residue

**(4) `xargs` joins the `GUARD_KIT_RO_BINS` default.** [mechanical] The one
surviving half of the entry's second-contributor claim. `sort` is already there
and the entry was wrong about it; the correction rides delta (8).

**(5) The read-only pipeline rule gains the banner tolerance its two neighbours
already have.** [design-bearing] The bare-`find` and bare-`cat` rules both
tolerate a literal `echo`/`printf` banner between reads, on the stated ground
that a banner is the natural separator of a batched read. The read-only pipeline
rule has no such tolerance and `echo` is not a roster binary, so a
grep-banner-ls sequence falls through where the same sequence without the banner
is granted. `_guard_is_banner` already exists and is already used by both
neighbours; this is the third caller. The asymmetry is the accident, not the
tolerance.

**(6) The read-only pipeline rule's lead predicate widens from a roster binary
to a roster binary *or a bare committed allow entry*.** [design-bearing] This is
the shape the authoring session met live: an allowlisted script piped into
`head`. The pipeline rule declines because the lead is not a roster binary; the
decorated-allowlist rule then **blocks**, and the agent is steered to re-issue a
command whose only decoration was a read-only reduction of its own output.

Widening the lead predicate is the right repair rather than weakening the
decorated-allowlist rule, and the ordering is why. Grant, not fall-through, is
what actually removes the friction, and the decorated-allowlist rule's own text
refuses to grant — correctly, since granting there would bless segments the
allowlist never reviewed. The pipeline rule is already the tree's sanctioned
place for a grant of exactly this shape, it is already placed **before** the
decorated-allowlist rule so a granted pipeline never reaches it, and the safety
argument composes without extension: the lead is statically allowlisted, so it
was reviewed; every tail segment leads with a roster binary, so it is read-only;
every redirect is `/dev/null` or an fd-dup, unchanged. Nothing ungranted becomes
granted that either half would not already have granted alone.

The **bare** qualifier is load-bearing and is taken verbatim from the
decorated-allowlist rule's own reasoning: only an exact `Bash(<cmd>)` entry with
no glob qualifies as a lead, because a glob-headed family coexists with
allowlisted decorators and would admit far more than the reviewed command.

### Corrections to the record

**(7) The generic ruleset's rule 6 and rule 7 entries gain the class
declaration.** [mechanical] Each numbered rule's prose states which inert
classes it declares and why, so the next rule author reads the decision instead
of copying the adjacent line. The ordering constraints each rule already carries
are untouched.

**(8) Both queue entries' falsified grounds are corrected in place at
promotion.** [mechanical] The decoration entry loses the `sort` claim, the
newline-segmentation conclusion and the `$?`-as-expansion argument, and gains
the measured re-sizing. The glyph entry gains the absolute-prefix widening —
that rule strips nothing, so the respelling escape its body implies does not
exist for that third. Corrections are made **where the sentences stand**; no
superseded claim is left beside the sentence that corrects it.

### What is deliberately not done

**(9) No registration gate for the rule roster, and the refusal is reasoned.**
[design-bearing] The generic ruleset is a numbered SPEC roster of named
`lib/guard.sh` functions dispatched in a fixed order, and **nothing holds the
three in lockstep** — the SPEC never names the `guard_rule_*` prefix or the
dispatcher, and the only statement of the convention is a code comment pointing
at a section that does not contain it. A `check-guard-ruleset-registration`
gate on the three-way correspondence is the obvious analogue of the lifecycle
and doctrine registration gates.

It is not taken here, for two reasons that are about scope rather than merit.
The convention it would enforce is currently unwritten, and gating an unwritten
convention means authoring it in the same motion — a governed name of its own,
on a surface `guard-command-prefix-wrapper` also writes to, which the operator
ruled out of this iteration. And the correspondence it would hold is the roster
of *rules*, while this amendment adds a **primitive** and changes existing
rules' arguments; the gate would be green before and after every delta here, so
landing it with this unit would buy nothing this unit needs. The convention is
stated in prose by delta (7) for the classes, and the gap is filed with its
cost rather than skipped.

## Producers and consumers

**`guard_skeleton` — producer.** Defined in `guard-kit/lib/guard.sh` beside the
other primitives, called by every rule that needs a lexical view. It is a pure
function of its arguments: no config read, no subprocess, no global. Its
consumers are enumerable and enumerated — the compound-`cd`, `git -C` root,
scratch-redirect, absolute-prefix, expansion, brace-glyph, `sed`-file,
`find`-glob, `cat`-file, `git grep`, truncate-scratch, read-only pipeline,
decorated-allowlist, git-rewrite and `rm`-tracked rules, plus the consumer
copy's project-rule prologue. Every one of those is a call site that exists
today with a private stripping; none is a new caller invented to give the
primitive a reader.

**The inert-class tokens — producer and named reader.** `sq`, `dq` and `hd` are
arguments, not config: they are shell grammar rather than consumer vocabulary,
so they are kit literals and introduce no knob. Their reader is the normalizer's
own dispatch, and each of the three has at least one rule declaring it — `sq`
by the expansion rule, `dq` by the brace rule and the assignment check, `hd` by
both. A class with no rule declaring it would be removed; none is.

**`GUARD_KIT_RO_BINS` — an existing producer, one element added.** Its reader is
the read-only pipeline rule, unchanged. Its default is stated in
§Layout and configuration as a family rather than an enumeration, so no SPEC
roster changes; `check-knob-default-coupling` holds the literal against the
SPEC's stated default and both sides agree after delta (4).

**The read-only pipeline rule's settings read — a new read by an existing
reader.** Delta (6) makes that rule read `GUARD_KIT_SETTINGS`, which it does not
today. The knob exists and the decorated-allowlist rule already reads it, so no
knob is introduced — but the **fail-open contract travels with the read** and is
restated for the new reader: no `jq`, no settings file, or a parse error and the
lead-widening silently declines, leaving the rule exactly as it behaves now.
A grant that depends on a settings read must never turn a missing settings file
into a grant, and declining is the only direction that cannot.

**No new field, and the removals are the interesting half.** Nothing here adds a
field to any record. What is *removed* is six private stripping expressions, and
their readers are the six rules that own them — each converted in the same delta
that removes it, which is why delta (2) is one delta rather than six.

**The narrowing check, run on the reader whose red condition is not monotone.**
Deltas (3), (5) and (6) all **narrow** what the guard refuses, and the reader
that matters is the decision table, `guard-kit/guard-tests/cases.tsv`. Its red
condition is a **verdict mismatch in either direction**, not a violation count,
so it is not monotone under narrowing and cannot be cleared by inspection: every
existing case whose command carries a double-quoted brace, a heredoc, a banner
segment, or an allowlisted lead with a read-only tail changes verdict and its
expected column must be re-derived rather than assumed still correct. This is
the delta most likely to be mistaken for a no-op sweep.

**Fixtures, and a coverage hole named because the amendment would otherwise
inherit it.** Every generic rule owes at least one firing and one non-firing
case, so each of deltas (3), (5) and (6) adds a pair, and the heredoc and
double-quoted-glyph shapes get cases of their own since they are the entries'
subject. The hole: the decision-table runner drives
`guard-kit/templates/bash-guard.sh`, **not** the consumer copy, so the four
project rules in `scripts/bash-guard.sh` have no behavioral coverage at all and
`check-template-copy-parity` checks their declaration shape rather than what
they do. Delta (2) changes those rules' matching by giving them the normalizer,
so the amendment adds template-level cases for the normalizer's behavior and
files the consumer-copy coverage hole rather than quietly relying on it being
someone else's.

## Existing sections updated

- **guard-kit/SPEC.md §The guard framework** — owned by deltas (1) and (2). The
  normalizer joins the primitive contracts with its three classes, the
  placeholder-not-deletion rule and the two stated bounds. The existing
  `guard_split_compound` contract gains the newline clarification delta (8)
  corrects the entry on: the separator class carries no newline, and one is
  already a boundary because consumers read the emitted lines.
- **guard-kit/SPEC.md §The generic ruleset** — owned by deltas (3), (5), (6)
  and (7). Rules 6 and 7 gain their class declarations and the statement that
  their difference is deliberate; rule 13 gains the banner tolerance and the
  widened lead predicate with its fail-open clause; rule 14's placement
  rationale gains the sentence explaining why the grant lives in 13.
- **guard-kit/SPEC.md §Layout and configuration** — owned by delta (4), the
  `GUARD_KIT_RO_BINS` default.
- **guard-kit/SPEC.md §Testing** — owned by the fixture obligations above.
- **TASK-QUEUE.md `exit-echo-decoration-guard-vs-habit`** and
  **`guard-glyph-match-context-blind`** — owned by delta (8).

## Definition of Done

- [ ] **Causal completeness** — the normalizer's every consumer is a converted
      existing call site, each inert class has a declaring rule, and the
      settings read added in delta (6) carries its fail-open clause.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper section; the rationale for rule 6 and rule 7 differing survives as
      prose a reader can act on.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`). Both paired entries move together,
      since one file serves both.
- [ ] **Removals propagated** — all six private stripping expressions gone, no
      rule matching a raw command string, and `bash guard-kit/bin/run-guard-tests.sh`
      green with every re-derived verdict.
- [ ] **Gaps filed** — the rule-roster registration gap of delta (9) and the
      consumer-copy fixture hole filed with their costs, never flagged and
      skipped.
