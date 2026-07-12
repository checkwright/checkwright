# doctrine-kit — the experience-packaging rung

The delivery doctrine the other kits enforce piecemeal — the cross-kit practice
rules a session must hold to work this methodology — becomes one kit-shipped,
customer-deliverable rules file, installed by reference into a consumer's
always-loaded agent file and held there by a gate. The other kits own the
*mechanisms*; doctrine-kit owns the *statements* that name why those mechanisms
exist, at one tier, in one place.

The kit is a new remit rather than a fold into canon-kit: the doctrine spans kit
charges — enforcement-first is gate-sdk-flavoured, load-trigger residency is
context-kit-flavoured, content-tiering is canon-kit-flavoured — and canon-kit's
own out-of-scope boundary holds that a consumer's tier contract is not
canon-kit's to own.

## The doctrine deliverable

`DOCTRINE.md` is the deliverable. It carries each cross-kit rule as its
statement, why it holds under coding-agent work, and a pointer to the kit
mechanism that enforces it; the mechanism and its knob rosters live in the cited
kit SPEC, never restated in the doctrine. The rule *statements* are the
doctrine's to own — a governed surface elsewhere cites a rule by name and links
the doctrine rather than restating it.

The roster carries two registers: methodology-maintenance rules, which govern
how the methodology's own surfaces stay honest and bear on every surface edit,
and engineering-craft rules, which govern how the work built under the
methodology is written and are triggered by test, rename, git-rewrite,
config-edit, and dispatch work. The
maintenance register forms the installed digest; the craft register lives behind
the link, load-triggered — the doctrine applying its own load-trigger residency
rule to itself.

The file is referenced in place, never copy-installed. A consumer vendors the
kit and links the vendored path; re-vendoring the kit *is* the doctrine upgrade.
A copied doctrine drifts the moment the source moves and the copy does not; a
linked one cannot. This is point-never-restate applied to the doctrine mechanism
itself.

`DOCTRINE.md` joins the consumer's spec manifest (this repo wires it into
`scripts/canon-config.sh`), so its links and commands resolve under the
canon-kit doc gates like any governed page.

## install-doctrine

`bin/install-doctrine.sh` writes the reference block into the always-loaded
agent file, idempotently. The block is bounded by fixed marker lines
(`<!-- doctrine-kit:begin -->` … `<!-- doctrine-kit:end -->`); a run replaces
the content between the markers when they are present and appends the block when
they are not, so re-running never duplicates and a marker set left in place is
updated where it sits. A begin marker without its end is a malformed target: the
installer refuses (exit 2) rather than guess the block bounds. The agent file
must already exist — the installer edits an always-loaded file, it does not mint
one — so a missing target is exit 2.

The block is the always-loaded shape applied to the doctrine itself: a one-line
digest of the methodology-maintenance rules plus a markdown link to the doctrine
file. The engineering-craft register is not digested — it is load-triggered and
reached through the link, so the always-loaded surface carries only what bears
on every edit. The installer is the single source of the block text, so a manual
insertion for a harness-less consumer copies what the tool would emit; the README
documents that manual path. The installed digest names every methodology rule,
so a fresh consumer is in per-rule lockstep out of the box (installer and gate
agree on the `## Delivery doctrine` heading — that agreement is part of
check-doctrine-registration's contract). Trimming a rule the consumer does not
keep resident stays legal, but rides a declared-trim marker rather than a silent
deletion: the gate asserts name-lockstep modulo declared trims (§check-doctrine-registration
assertion B).

Positional overrides `install-doctrine.sh [agent-file [doctrine-file]]` let a
smoke or a fixture point both paths at a scratch tree without touching consumer
config; unset, they fall to the knob defaults.

## check-doctrine-registration

Invariant, in four assertions: the configured agent file (A) carries a markdown
link to the configured doctrine file and (B, C) holds its methodology-rule digest
in per-rule lockstep with the doctrine, and the doctrine's craft register (D)
tags every rule with exactly one stage-routing trailer. The digest is the surface the
always-loaded-shape rule requires, and a re-vendored `DOCTRINE.md` that adds or
renames a methodology-maintenance rule staling every consumer's digest *by
construction* — on the exact path the kit advertises as its upgrade story — is
the drift-prone-surface-that-must-exist case where a gate is owed (the
enforcement-first weighing). Extending the existing gate rather than adding one
keeps the gate count flat; the coupling is unchanged (the `# graph:` manifest
already couples the agent file and the doctrine file).

- **Assertion A (link).** The gate greps the agent file for a
  `](<doctrine-file>` link token; absent, it is a finding with the install
  remedy. It asserts link *presence* only — that a session loading the agent
  file is pointed at the doctrine, not that the link was followed or the
  doctrine read (the honest limit: a link is not a read) — and leaves
  link-target resolution to the consumer's doc gates (canon-kit's
  `check-md-refs` over the manifest).
- **Assertion B (doctrine → digest).** Every rule name under the doctrine's
  `## Methodology-maintenance rules` section — the bold text of each numbered
  rule, trailing period dropped (e.g. `Content-tiering / SSOT`) — appears as a
  bold digest lead-in (`- **<name>**`) in the agent file's digest section.
  Engineering-craft rules are exempt: they live behind the link by the
  doctrine's own two-register design, so the gate scans only the
  methodology section for the required set. The consumer's right to reject a
  rule survives as a *declared* trim: a
  `<!-- doctrine-digest-trim: <rule name> — <reason> -->` line inside the
  digest section satisfies assertion B for that rule; a silent omission stays
  red. Declared-not-silent is the reconciliation — the re-vendor moment
  surfaces every added or renamed rule, and the consumer's decision (adopt the
  bullet or trim it with cause) is recorded beside the digest it governs.
- **Assertion C (digest → doctrine).** Every bold bullet lead-in in the digest
  section matches a methodology-maintenance rule name — a digest line with no
  owning rule is a rule stated nowhere the doctrine governs.
- **Assertion D (craft-trailer coverage).** Every numbered rule under the
  doctrine's `## Engineering-craft rules` section carries exactly one `*Stages:*`
  trailer matching the tag grammar (§stage-rules). A craft rule with no trailer,
  or two, or a malformed value is a finding: a re-vendored `DOCTRINE.md` that adds
  an untagged craft rule reddens here instead of silently dropping out of the
  stage routing the emitter derives. Stage-name *validity* is deliberately
  unasserted — doctrine-kit does not depend on lifecycle-kit's stage config, and
  the emitter's empty-output posture already covers an unknown stage; the gate
  holds only that the trailer is present and well-formed.

Section resolution fails closed. The digest section is the agent-file heading
named by `DOCTRINE_KIT_DIGEST_SECTION` (Layout and configuration); a configured
heading matching nothing exits 2 — a renamed digest section must not disarm the
gate into passing an empty set. The doctrine-side headings
(`Methodology-maintenance rules` and `Engineering-craft rules`) are kit
mechanism, not config: the kit ships `DOCTRINE.md`, so it owns those names, and
either one's absence is likewise exit 2 (the gate cannot certify the digest or
the craft trailers against an unreadable rule set). A missing agent
or doctrine file is fail-closed for the same reason, as is a grep or awk that
errors rather than simply not-matching.

The gate satisfies the four gate-sdk contracts (gate-sdk/SPEC.md §The gate
model): the single `DOCTRINE-REGISTRATION: clean` success line and a `help:`
remedy on each finding path (output); exit 2 on an unreadable file, an
unresolved section, or an errored capture (fail-closed); a `good/`+`bad/`
fixture pair under `gate-tests/` — the pair carries the lockstep-clean and the
digest-missing-a-rule cases (both craft-tagged so assertion D passes there), and
a sibling `*.test.sh` drives the extra-line, declared-trim, link-absent,
craft-untagged, craft-malformed, and four fail-closed cases the one-pair harness
cannot hold (fixture-pair); and registration in this repo's `gates.list` where
its own always-loaded file is the scan target (self-lint). Positional form
`check-doctrine-registration.sh [agent-file [doctrine-file]]` lets the fixtures
point at a synthetic agent and doctrine file.

## stage-rules

The engineering-craft rules are load-triggered — behind the doctrine link, not
digested (§The doctrine deliverable) — and a *stage* is itself a load trigger.
`bin/stage-rules.sh <stage>` closes that gap: it derives, for a given stage, the
craft rules that bear on it and emits one pointer line per hit, so a session
entering a stage is reminded of the craft rules to follow *before* the matching
action, without the always-loaded surface carrying the prose.

**The tag grammar (single source: the rule owns its stage).** Each rule under
`## Engineering-craft rules` in `DOCTRINE.md` carries a machine-parsable trailer
line, `*Stages:* <stage>[, <stage>…]` — a comma list of lowercase stage tokens
naming the kit's default stage vocabulary — or `*Stages:* —` for a rule that
routes to no stage. The mapping lives on the rule, so a re-vendored `DOCTRINE.md`
carries its own routing and no consumer-side stage↔rule table exists to drift;
`check-doctrine-registration` assertion D holds every craft rule to exactly one
well-formed trailer.

**The emitter.** `bin/stage-rules.sh <stage> [doctrine-file]` scans the craft
section for rules whose `*Stages:*` line names the given stage and prints one
pointer line each — rule number, name, and the doctrine path — so the reader
follows the link to the rule body. An unknown stage name yields empty output:
the tags name the kit-default stages, so a consumer with a renamed stage set
gets no routing rather than wrong routing (the stated honest limit — a stage
remap knob is deferred until such a consumer exists). It sources
`lib/doctrine.sh` for the doctrine path and takes the same positional override
the gate and installer do; a missing doctrine file is exit 2, an absent craft
section or a no-match stage is empty output.

**The surfacing seam.** The emitter is derived data with no standing tier of its
own; context-kit's session-context hook is its named consumer, emitting the
current stage's pointer block when the emitter is vendored
(context-kit/SPEC.md §The session-context hook, the drift-line seam precedent).

## lib/doctrine.sh

The sourced config loader shared by the installer and the gate: it loads
`DOCTRINE_KIT_CONFIG_FILE` (or the gates-dir `doctrine-config.sh` when that env
is unset), then a gitignored `.local.sh` overlay beside it, then fills each
knob's default — so the installer and the gate read one resolved configuration.
It carries no gate logic: structure stays in the check, values in config,
defaults here.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `doctrine-kit/`); its
gate is registered in the consumer's `gates.list` by name and resolves through
gate-sdk's multi-kit path. Config follows the kit pattern: an optional
`doctrine-config.sh` in the gates dir (or a `DOCTRINE_KIT_CONFIG_FILE`
elsewhere) overrides any knob; defaults fill what the consumer left unset. Knobs:

- `DOCTRINE_KIT_AGENT_FILE` — the always-loaded file the installer edits and the
  gate scans, default `CLAUDE.md`.
- `DOCTRINE_KIT_DOCTRINE_FILE` — the link target the installer writes and the
  gate asserts, default `doctrine-kit/DOCTRINE.md`.
- `DOCTRINE_KIT_DIGEST_SECTION` — the agent-file heading whose bullet list the
  gate reads as the methodology-rule digest (assertions B and C), default
  `## Delivery doctrine`. The default is `install-doctrine.sh`'s installed block
  heading, so a zero-config consumer that installed via the tool is green out of
  the box; a consumer that renamed the heading repoints this knob (a rename that
  leaves it stale exits 2 rather than passing an empty set).
- `DOCTRINE_KIT_CONFIG_FILE` — the loader override; when set it is sourced if it
  exists, and a `.local.sh` sibling sources last for private overlay values.

The defaults are this repo's own layout, so this repo runs the kit on itself
with no config file: `CLAUDE.md` is the always-loaded agent file, and its
`## Delivery doctrine` reference block links `doctrine-kit/DOCTRINE.md`.

## Out of scope

Folding the doctrine into canon-kit — ruled out above. Copy-install of the
doctrine file — a copied doctrine drifts; the reference is the mechanism.
Per-rule enable/disable knobs — the doctrine is one document, and a consumer
that rejects a rule declares a trim marker beside its digest rather than toggling
a knob. A standing consultation step in place of packaging — the always-loaded
anti-pattern this load-triggered kit exists to replace. A point-of-use guard
pushing a craft rule at the triggering action (a Write-seam `guard_advise`) —
demand-gated and out of scope: guard-kit's seam is Bash-only, the craft-rule
triggers ride the harness Write/Edit tools the guard never sees, and the
stage-routed surfacing (§stage-rules) already reaches every stage session, so a
new Write-seam mechanism carries one tentative consumer and no live demand. The kit holds no opinion
on *which* methodology rules a consumer keeps resident versus trims with cause;
it ships the statements and the wiring, and the consumer's always-loaded budget
rules the digest — but a trim is declared, not silent, so the gate holds the
resident set and the doctrine in name-lockstep modulo those declarations.
