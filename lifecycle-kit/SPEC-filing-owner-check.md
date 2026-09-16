# SPEC amendment: filing-owner-check

Pairs `close-eviction-refiles-without-checking`.

**Every replacement text below is Not yet applied.**

## What changes

### (1) Every queue filing owes an owner lookup, made where the queue is written {design-bearing}

A stage that writes a new deferred entry first looks for an existing entry that owns
the finding. The lookup needs no similarity oracle. Every attested re-filing had an
owner that a plain grep for the slug and the subject found, and in each case the
filer never ran it. So the missing piece is a step, not a scanner.

The obligation sits where the queue is written, never at capture. §The committed gap
inbox already refuses a filing-time prompt on grounds this change leaves standing:
capture stays cheap, and the drain re-verifies. The lookup is part of that
re-verification. It binds every session that writes a deferred entry:
- the close drain's →promote;
- the first stage's intake →promote;
- each finding close files on its own account, such as a lesson →task, an eviction
  or triage finding, or a dispatch's captured debt.

Replacement text, a new paragraph in lifecycle-kit/SPEC.md §The committed gap inbox
after "The drain re-verifies; capture does not." (Not yet applied):

> **Filing looks for an owner first.** Before a session writes a new deferred
> entry, whether by a drain's or an intake's →promote or by close's own filing, it
> searches the queue file for an entry that already owns the finding. It greps the
> finding's distinctive terms (the paths, gate and knob names, and slug-shaped
> words) across every entry, the icebox included. A match is read, not counted:
> - An owner carrying the finding is a **recurrence**. It gets a `recurrence:`
>   stamp and any new axis folded into its body. No second entry is filed.
> - An adjacent entry is cited in the new entry's prose as **distinct**, with the
>   reason.
> - A done slug is a finding that recurred after its fix. It files as a new defect
>   that names the done slug.
>
> The commit that files records the lookup per entry: the terms searched, and
> `none` or the slug that matched with its reading. Then a missed owner shows as a
> bad search, not as a silent omission. This lookup does not reach capture.
> *The honest limit:* a filer who picks terms that miss the owner files a
> duplicate, and nothing reds. A duplicate pair stays conserved under
> `check-task-conservation`, so the commit-message record is the only residue a
> later reader can audit.

### (2) The two filing templates carry the step {mechanical}

**`lifecycle-kit/templates/stages/close.md`, "Where close's own captures file".**
Append after its first sentence group (Not yet applied):

> Every filing, including a drain →promote, runs the owner lookup first and records it
> in the commit message (lifecycle-kit/SPEC.md §The committed gap inbox).

The step 1 sentence that begins "When a lesson claims it was 'already filed under
<slug>'" stays. It is the →task instance of the same check, and it reads the
owner's body.

**`lifecycle-kit/templates/stages/scope.md`, second step.** After "promoted to a
queue entry, fixed inline this session, or discarded with cause in the commit
message", add (Not yet applied):

> — a promotion running the owner lookup first (lifecycle-kit/SPEC.md §The committed
> gap inbox)

## Producers and consumers

- **The lookup obligation (delta 1).**
  - Producers: close's drain step, the first stage's intake step, and close's other
    filing steps. Each is reachable at every boundary with no enabling config.
  - Consumer: the filing session itself, whose next action (stamp, cite, or file)
    turns on the result.
- **The commit-message record.** Its reader is a later session auditing a suspected
  duplicate, which learns what was searched. It is a record in history, not a field,
  so no gate reads it. That matches the drain's existing re-verification record in
  the same commit message.
- **Roster readers.** The change mints no knob, gate, tag or field, so no
  roster-holding reader is touched. `check-shim-restatement` reads the two template
  edits. Each is one sentence pointing at the SPEC and carries no grounds.
- **Point 5.** No delta narrows a corpus.
- **Point 6.** No delta obliges an enumerable corpus.

## Existing sections updated

Probe: `grep -n "promote" lifecycle-kit/templates/stages/scope.md
lifecycle-kit/templates/stages/close.md lifecycle-kit/templates/lead.md`, run
2026-09-16. It is a floor. The lead template files only to the inbox, so it takes no
edit.

- `lifecycle-kit/SPEC.md` §The committed gap inbox, the new paragraph (delta 1).
- `lifecycle-kit/templates/stages/close.md`, "Where close's own captures file"
  (delta 2).
- `lifecycle-kit/templates/stages/scope.md`, second step (delta 2).
- `docs/lifecycle-kit/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds.
- [ ] **Instruction surfaces: instruction only** — the template sentences point at
      the SPEC paragraph and carry no grounds.
- [ ] **Merged with no information lost** — the SPEC paragraph re-phrases into
      §The committed gap inbox rather than appending a second rule.
- [ ] **Amendment deleted** — none remain (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none were declared.
- [ ] **Gaps filed** — cross-component gaps discovered during the work are filed.
