# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-29 scope — Which deferred or icebox entries pair (moot or reshape) with the companion-prerequisite unit set's edited names?
- corpus: TASK-QUEUE.md
- oracle: awk '/^## Deferred/{d=1} /^## Done/{d=0} d' TASK-QUEUE.md | grep -n -o -E '^### [a-z0-9-]+|checkwright\.lock|profiles\.list|--profile|--recipe|run-smoke\.sh|speckit\.checkwright|companion/'
- rev: cbabc1020701235aacd5977405fdc9facfc5ed32
- finding: Over the Deferred and Icebox sections: consumer-smoke-windows-residue mooted by compiled-consumer-smoke-driver (replaces the script) if that lands first; install-gate-selection reshaped by verify-workflow-decoupling (whether selection may drop gate-sdk) and front-door-flag-grammar (new init flags advertised before the pinned release); speckit-extension-full-profile reshaped by companion-spec-to-code-gates (the gates the full tier carries) and install-gate-selection; spec-brevity-residue's installer slice reshaped by install-gate-selection (same sections); companion-spec-to-code-gates independent; adoption-prompt-templates reshaped by all three
- inferred: whether each pairing moots or reshapes is judged from the bodies, not the grep

## 2026-09-29 spec — Which spec-to-code checks do the pinned Spec Kit and OpenSpec formats make possible, and which does the toolkit already own?
- corpus: companion/toolkits.list companion/fixtures
- oracle: install the toolkits.list pins into .tmp (pip specify-cli, npm @fission-ai/openspec); specify init --here --non-interactive --integration claude --script sh --ignore-agent-tools and read .specify/templates plus the speckit skills; openspec validate --all --strict and openspec archive <change> -y over planted MODIFIED, REMOVED, ADDED and RENAMED deltas on a copy of the OpenSpec layout fixture
- rev: 445055c5f4910aac12339288416a1fb295be6b64
- finding: Spec Kit tasks read '- [ ] T005 [P] [US1] desc in src/x.py' (bare path after in, some pathless; ticked [X] or [x]); [USn] labels map to '### User Story n' spec headings; FR-/SC- IDs appear in tasks only in the optional converge phase; plan.md's structure is a text-fence tree of intended paths; analyze is agent prose. OpenSpec tasks '- [ ] 1.1 text' carry no paths; validate --strict exits 0 on MODIFIED-absent, RENAMED-absent-source and ADDED-existing (INFO only) and REMOVED-absent (silent); archive -y refuses the first three and archives REMOVED-absent as already removed. Extension commands receive free-text ARGUMENTS.
- inferred: that OpenSpec doctor and status inspect no code (unrun)
