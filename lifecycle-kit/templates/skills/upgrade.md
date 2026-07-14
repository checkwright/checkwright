The `upgrade` skill — the phase-B disposition walk for a consumer moving their
vendored kits from one release to the next. Not an iteration stage: it invokes
no `enter-stage.sh`, stamps no `WORKFLOW-STATE.txt`, and runs in any session at
an upgrade. Phase A (the deterministic wholesale kit-sync) and this phase-B walk
are the two halves of the upgrade contract — the install guide §The upgrade
contract owns the shape, and `gate-sdk/bin/upgrade-smoke.sh` proves it
mechanically where the consumer has the smoke. Exit condition: the full battery
is green or every red carries a stamped disposition.

## Session ritual

1. **Run phase A.** Replace the vendored kit directories wholesale at the
   target release and regenerate the generated artifacts, the install guide's
   phase-A steps. Where the upgrade smoke is available, run it first: a green
   verdict is the mechanical proof that the sync is deterministic and the red
   set is declared, before you disposition anything by hand.
2. **Read the target release note.** The note whose front-matter `release:`
   names the target version carries two checklists. Register every gate its
   Tightened gates section names into *<gates-list: where this consumer
   registers its gates — the gates.list path when non-default; a new release's
   gate reaches an upgrading consumer only through this declaration, never
   through the phase-A sync, so registering it here is the delivery step.>*, and
   apply each Renamed knobs rename to your own config.
3. **Run the full battery; disposition every red.** Each red gate is one of two
   dispositions — **fix-the-tree** (the surface moved; migrate your code to the
   new contract) or **exempt-with-cause** (a gate you shadow or omit from your
   registry, the cause stamped) — and never a weakening of the gate. Stamp one
   disposition line per red into *<disposition-evidence: this consumer's
   evidence/disposition stamp path, committed on the upgrade commit.>*.
4. **Judge the semantic residual — the ungateable audit.** For every skill you
   bind through a shim, set the target template's slots beside your shim's slot
   fills. A changed slot set reds `check-skill-binding` and a verbatim copy reds
   `check-shim-restatement`, but the residual — a fill you worded to cover what
   the new template's slot now means to own — clears both gates and cannot be
   gated. The upgrade is that class's audit cadence: surface each changed
   template slot against your fill and delete the wording the template now
   carries for you.

The disposition stamp is operator evidence riding the upgrade commit; nothing
in the kit reads it. The two named slots hold this consumer's residue — where
its gates register and where its dispositions land.
