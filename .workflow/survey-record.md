# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-29 scope — Which deferred or icebox entries pair (moot or reshape) with the companion-prerequisite unit set's edited names?
- corpus: TASK-QUEUE.md
- oracle: awk '/^## Deferred/{d=1} /^## Done/{d=0} d' TASK-QUEUE.md | grep -n -o -E '^### [a-z0-9-]+|checkwright\.lock|profiles\.list|--profile|--recipe|run-smoke\.sh|speckit\.checkwright|companion/'
- rev: cbabc1020701235aacd5977405fdc9facfc5ed32
- finding: Over the Deferred and Icebox sections: consumer-smoke-windows-residue mooted by compiled-consumer-smoke-driver (replaces the script) if that lands first; install-gate-selection reshaped by verify-workflow-decoupling (whether selection may drop gate-sdk) and front-door-flag-grammar (new init flags advertised before the pinned release); speckit-extension-full-profile reshaped by companion-spec-to-code-gates (the gates the full tier carries) and install-gate-selection; spec-brevity-residue's installer slice reshaped by install-gate-selection (same sections); companion-spec-to-code-gates independent; adoption-prompt-templates reshaped by all three
- inferred: whether each pairing moots or reshapes is judged from the bodies, not the grep
