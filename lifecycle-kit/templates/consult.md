The `consult` skill — an operator strategy session whose conclusions leave the
transcript. Not an iteration stage: it invokes no `enter-stage.sh`, stamps no
`WORKFLOW-STATE.txt`, and holds no cursor, so it may run before an iteration
opens, between stages, or across an iteration boundary. Exit condition:
every ruling the operator closed in the session has landed in a governed
surface, every alternative refused along the way is recorded with the grounds it
was refused on, and every always-loaded surface those rulings stale is
corrected, flagged, or filed as work.

A consultation that ends with its rulings still in the transcript has not
exited. That is the failure this skill exists to stop — a strategy session whose
conclusions the next session re-derives from memory, or never recovers at all.

The write authority this skill leans on is **not** the skill's to grant: it
belongs to the ruling record itself, which states who may record a ruling there
and under what condition. What this skill adds on top of that authority is
**obligation**. Any session that closes a ruling with the operator is permitted
to record it; a consultation has not finished until it has.

## Session ritual

1. **Read in before proposing anything.** *<entry-reading: the surfaces a
   consultation reads on entry — name paths, not topics. Typically the record of
   closed direction, the work queue, and any context this consumer keeps outside
   the tree; a consultation that skips the last of those re-opens what is
   already settled.>*

2. **Land each ruling the moment it closes, never at exit.** The landing target
   is chosen by what the ruling is about, from *<landing-surfaces: the surfaces a
   closed ruling may land on, each with the ruling class it takes — direction and
   refusals, work items, and the contract surfaces a ruling about mechanism
   belongs in. Say which one takes a ruling that fits none of them.>* Landing at
   exit instead is the failure mode itself, one step later: a session that
   batches its rulings to the end loses all of them together when it is
   interrupted.

3. **Record a refused alternative beside the ruling that refused it, with its
   grounds.** A refusal recorded without grounds is re-proposed by the next
   session at full cost; a refusal recorded with them is already costed when that
   session reaches for it. Deletion is not disposition — an alternative weighed
   and rejected is *kept*, marked as rejected.

4. **Name what the rulings stale.** A ruling that contradicts an always-loaded
   surface leaves that surface teaching the superseded shape to every later
   session, at per-session cost. Correct it in the same session where the edit is
   small and unambiguous; where it is neither, file it as work with the ruling
   cited, so the correction is owed rather than remembered.

5. **Journal in flight, and treat a held index as latency rather than loss.**
   Durability is the obligation and this session discharges it by whichever means
   is available — the rule and its two discharge paths are
   `delegation-kit/templates/agent-execution.md`'s **Findings you will act on are
   durable before you act on them** bullet, which reaches this session directly.
   When another session holds the shared index, journal each ruling as it closes
   and commit when the index frees. Waiting is safe only because the finding is
   already durable; waiting without the journal is the transcript failure again,
   wearing a different cause.

6. **Dispatch by the delegation protocol, and select the tier every time.** The
   protocol is `delegation-kit/templates/agent-execution.md`, cited and never
   copied — a copy is a second tier of the same content, drifting from the day it
   is written. Tier selection needs its own attention here: an unselected
   dispatch inherits *this* session's tier, and a consultation runs at the
   judgment tier while most of what it dispatches is read-only research, so an
   unselected fan-out buys the most expensive tier for the cheapest work. Name
   the tier in the dispatch, downward by default, and reserve the judgment tier
   for a dispatch that actually rules.

The two named slots are where this consumer's own governance layout lives — what
a consultation reads, and where its rulings land. The kit ships the shape and
never the surfaces: a consultation has an entry read-set and an exit
landing-set, and every closed ruling reaches the landing set before the session
ends.
