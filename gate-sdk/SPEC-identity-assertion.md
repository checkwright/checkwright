# SPEC amendment: identity-assertion

A verification backstop for the fresh-clone gap: an agent that commits or
pushes under the wrong git identity fails silently — misattribution is
unpurgeable without a SHA-breaking history rewrite, and the wrong-SSH-key
symptom is a misleading "Repository not found". Multi-identity setups
(work + personal accounts) make this common for the integrator/consultant
audience. **Scope fence:** the identity *mapping* stays git's job
(`includeIf`, `core.sshCommand`) — this feature only verifies that the
mapping actually applied in this clone.

## What changes

- **Component ruling: gate-sdk**, not the guard kit. The queue entry left
  this open ("likely friction-kit (guard) or the setup story"); ruled
  gate-sdk because (a) guards intercept tool calls, while this is a
  repo-state assertion — gate territory; (b) the guard kit's
  registers-no-gates property is worth keeping; (c) the setup-story hook
  point, `install-hooks.sh`, is gate-sdk's.
- **New gate `gate-sdk/checks/check-identity.sh`** (skeleton-derived, four
  contracts, `good/`+`bad/` fixtures). Invariant: every expectation in the
  identity manifest matches the local clone —
  - `email <expected>` lines match `git config user.email` exactly;
  - `remote-host <remote> <host>` lines match the host part of
    `git remote get-url <remote>` by exact string (an SSH host alias is
    matched as the alias — that is the point: the alias *is* the identity
    selector in multi-identity setups). A configured remote that does not
    exist is red.
  - All comparisons are local reads — cheap, FP-free, no network.
- **New manifest convention `scripts/identity.conf`** — line-based
  `key value…`, `#` comments; knob `GATE_SDK_IDENTITY_FILE` (default
  `$GATE_SDK_GATES_DIR/identity.conf`). Optional consumer config
  (`graph-vocab` pattern): absent ⇒ pass with a note; present-but-malformed
  line ⇒ red (fail-closed).
- **Tier: precommit** (via the `# graph:` manifest), covering the
  commit-identity half without any stage skill. **Plus the
  apply-and-verify rung:** `install-hooks.sh` runs this gate once,
  immediately after enabling hooks, so a fresh clone learns about a
  mismatch at setup time — before the first commit, and covering the
  push-identity half (remote-host) at the moment it is cheapest to fix. No
  pre-push hook is added: gate-sdk generates only the pre-commit hook, and
  widening that surface is not justified by a check the setup rung and
  precommit tier already cover.
- **Registered in this repo's `scripts/gates.list`**; this repo commits an
  `identity.conf` expecting the maintainer identity.

## Producers and consumers

- **Producer:** the generated pre-commit hook / `run-gates.sh` (every
  commit), and `install-hooks.sh` (once, at opt-in) — both reachable
  wherever the consumer registers the gate and commits the manifest.
- **Consumer:** the committing operator/agent via the gate output
  contract; at install time, the operator running `install-hooks.sh` (its
  exit status surfaces the mismatch).
- **Manifest fields:** `email`'s value is read by the user.email
  comparison; `remote-host`'s two values by the remote-URL host
  comparison — each at the gate's single scan transition. No other reader;
  no new persistent state.

## Existing sections updated

At merge into gate-sdk/SPEC.md:
- new `### check-identity` per-gate contract section;
- the knob table gains `GATE_SDK_IDENTITY_FILE`;
- the `install-hooks.sh` section gains the apply-and-verify sentence (run
  `check-identity` after enabling `core.hooksPath`);
- the consumer-layout section lists `identity.conf` as optional consumer
  config beside `graph-vocab.sh` and `core-files.list`.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
