# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-16 scope — Which kit SPECs still carry this project's ruling provenance, and at what sites (seam-sweep remainder sizing)?
- corpus: canon-kit/SPEC.md doctrine-kit/SPEC.md drift-kit/SPEC.md site-kit/SPEC.md evidence-kit/SPEC.md gate-sdk/SPEC.md queue-kit/SPEC.md lifecycle-kit/SPEC.md context-kit/SPEC.md guard-kit/SPEC.md delegation-kit/SPEC.md
- oracle: live-slug scan (TASK-QUEUE.md lead-line slugs of 12+ chars, grep -o -w -F over each kit SPEC); grep -c for ISO dates over the four owing kits; grep for operator/lead-authority markers and TRAJECTORY.md, BRIEF.local.md, CLAUDE.md section pointers
- rev: 3d4a45565bc2d3b4614d450bb9fe194c398fcc3f
- finding: Slug hits: canon 3, drift 3, site 1, gate-sdk 1 (accretion). Dated hits: drift 8, site 2, canon 1, doctrine 1, evidence 0. Pointer hits: canon-kit one TRAJECTORY.md citation. Owing kits: canon, doctrine, drift, site; evidence-kit clean. Seam gate unbuilt.
- inferred: dated-marker hits include legitimate dated measurements (delta 2 exclusion), so dated counts are an upper bound

## 2026-09-16 spec — Which kit SPECs cite a queue slug that is no longer live, and which queue slugs collide with mechanism names (seam-sweep remainder, retired-slug half)?
- corpus: canon-kit/SPEC.md context-kit/SPEC.md delegation-kit/SPEC.md doctrine-kit/SPEC.md drift-kit/SPEC.md evidence-kit/SPEC.md gate-sdk/SPEC.md guard-kit/SPEC.md lifecycle-kit/SPEC.md queue-kit/SPEC.md site-kit/SPEC.md
- oracle: every slug of 12+ chars ever on a TASK-QUEUE.md lead line (git log -p TASK-QUEUE.md), matched per kit SPEC line with [A-Za-z0-9_-] as word chars
- rev: 9db4b3dce2a482eb49d769595a0ef87b1317f2a5
- finding: Beyond the live-slug hits: shell-gate-tail-port in canon-kit, site-kit and evidence-kit; deprecation-lifecycle and upgrade-path in lifecycle-kit; doctrine-rule-lockstep in context-kit; faithful-artifact-verification in site-kit. Mechanism-name collisions in history: check-spec-pointer, doctrine-kit, enforcement-map (no finding; they shape the slug arm's exclusion). evidence-kit is not clean.
- inferred: site-kit faithful-artifact-verification use may be an ordinary term rather than a pointer
