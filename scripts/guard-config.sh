# shellcheck shell=bash
# spec: guard-kit/SPEC.md §Layout and configuration — this repo overrides one guard-kit knob; templates/guard-config.sh lists them and lib/guard.sh holds the defaults

# spec: guard-kit/SPEC.md §compare-settings-allow — witnesses that a local-overlay glob is too broad; probes, not a roster, so no completeness is claimed
# shellcheck disable=SC2034  # consumed by guard-kit/lib/guard.sh after sourcing
GUARD_KIT_BREADTH_PROBES=(
    "Bash(git reset --hard)"
    "Bash(git clean -fd)"
    "Bash(git push --force)"
    "Bash(git rm -r src)"
    "Bash(git checkout -- TASK-QUEUE.md)"
    "Bash(git stash clear)"
    "Bash(gh repo delete owner/repo --yes)"
    "Bash(gh api -X DELETE /repos/owner/repo)"
    "Bash(gh release delete v0.24.0 --yes)"
    "Bash(gh secret delete TOKEN)"
    "Bash(git worktree remove --force .)"
)

# spec: guard-kit/SPEC.md §compare-settings-allow — breadth ruled intended; exact-string keyed, so re-spelling or narrowing the glob returns it to the narrowing candidates
# shellcheck disable=SC2034  # consumed by guard-kit/lib/guard.sh after sourcing
declare -A GUARD_KIT_BREADTH_DECLARED=(
    ["Bash(git worktree *)"]="worktree isolation is mandated rather than optional here — agent-dispatch-guard refuses a read-only-type dispatch that does not carry it — and the harness both creates and auto-cleans those trees, so the force-removal this breadth reaches destroys only disposable child state"
    ["Bash(gh api *)"]="operator-ruled 2026-08-23 out of guard-grant-review, whose review walked all 105 committed Bash( entries and found nothing to narrow among the four gh globs. The ops surface this serves is repo-settings desired state, which is read and reconciled through arbitrary api paths and methods, so no glob narrower than the binary bounds it — a Bash rule's * spans / and .. by the vendor's own matcher semantics, which is why the narrowing question is a breadth ruling rather than a glob-authoring one"
    ["Bash(gh repo *)"]="operator-ruled 2026-08-23 out of guard-grant-review, on the same review. The destructive form this reaches is gh repo delete, guarded host-side by the delete_repo scope a stock gh login does not carry, so the breadth reaches a command the credential must separately be entitled to run"
    ["Bash(gh release *)"]="operator-ruled 2026-08-23 out of guard-grant-review, on the same review. This is the sharpest of the four — gh release delete destroys one of this project's two shipping transports — and it is declared rather than narrowed because the close stage cuts, edits and verifies releases itself (RELEASING.md steps 4-6) across subcommands no fixed glob enumerates"
    ["Bash(gh secret *)"]="operator-ruled 2026-08-23 out of guard-grant-review, on the same review. Secret rotation is the ops workflow it serves, and a rotation is a set-then-verify pair whose argument shape is the secret name, which is exactly what a glob cannot bound"
)
