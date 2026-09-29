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

## 2026-09-29 align — Do the four iteration amendments hold against the tree?
- corpus: native installer companion docs
- oracle: read-site greps and one audit sweep
- rev: ed9968eb9528b23fc6165ce231e6d361cb658192
- finding: Quoted sources hold; EreCapture lacks a from-offset capture (fixed in the amendment as capture_from); rosters gained companion Recipes and Fixtures, ere.rs, release-declaration rows; two wording fixes
- inferred: none

## 2026-09-29 build — Which facts do other tracked surfaces cite into installer/SPEC.md's install-surface sections (§The verbs, §init, §What init seeds, §Payload recipes, §Profiles, §The manifest)?
- corpus: installer native/src gate-sdk/SPEC.md companion/SPEC.md plugin/SPEC.md queue-kit/SPEC.md context-kit/SPEC.md docs/site-architecture.md
- oracle: git grep -nE '(§|SPEC\.md ?§? ?)(The verbs|init|What init seeds|Payload recipes|Profiles|The manifest)|#(the-verbs|init|what-init-seeds|payload-recipes|profiles|the-manifest)' -- . ':!docs/installer' ':!docs/posts'
- rev: 8ba692ff267507e9b3f024753b0b9ac6eae82bc2
- finding: Per-section keep-lists recorded in the installer-install-brevity commit; every cited fact survives the pass. Dangling: §The verbs lacked the -h/--help-answers-on-its-own rule and the refusal-prefix idiom (7 crate sites, main.rs, §demo cite it), now stated there; native/src/installer/recipe.rs cited §Profiles for the profile-keyed roster, which §What init seeds states, now repointed. update's verbatim forwarding stays in §update.
- inferred: Rust spec: comments were read to their first ~250 characters, so a keep fact built from a comment head alone is inferred.
