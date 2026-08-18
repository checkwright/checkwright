#!/usr/bin/env bash
# The `gh-account` kind's graded absence postures, which the good/bad pair cannot
# express: the pair fixes the match/mismatch axis, and this holds the three
# verdicts gate-sdk/SPEC.md §check-identity grades an absent, unreadable or
# host-block-less CLI config into — plus the `users`/`user` key collision, whose
# whole point is that a wrong implementation still passes the pair.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
SANDBOX="$(mktemp -d)"
trap 'chmod -R u+rwX "$SANDBOX" 2>/dev/null; rm -rf "$SANDBOX"' EXIT

fails=0

printf 'dev@example.com\n' >"$SANDBOX/git-config-email"
printf 'origin git@github.com:example/example.git\n' >"$SANDBOX/git-remotes"

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=hosts-file path  $5=manifest body
    local label="$1" want="$2" sub="$3" hosts="$4" body="$5"
    printf '%s\n' "$body" >"$SANDBOX/identity.conf"
    local out rc
    # spec: gate-sdk/SPEC.md §check-identity — the knobs are the ported member's only
    # redirection, so a case sets them where it used to pass `--fixture <dir>`
    out="$(
        gate_env \
            GATE_SDK_IDENTITY_FILE="$SANDBOX/identity.conf" \
            GATE_SDK_GIT_EMAIL_FILE="$SANDBOX/git-config-email" \
            GATE_SDK_GIT_REMOTES_FILE="$SANDBOX/git-remotes" \
            GATE_SDK_GH_HOSTS_FILE="$hosts" \
            GATE_SDK_GH_HOST="host.example"
        gate_run check-identity "$DIR/checks" 2>&1
    )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# The whole manifest, so the two established kinds stay clean and the account
# kind is the only axis each case moves.
MANIFEST='email dev@example.com
remote-host origin github.com
gh-account example-dev'

# Posture 1 — absent hosts file: clean, and the fail-open caveat is named in the
# clean line rather than left for a reader to infer from a bare `clean`.
check_case "absent-clean-with-caveat" 0 "unverified" "$SANDBOX/nonexistent-hosts.yml" "$MANIFEST"

# The count in that clean line excludes the expectation it could not verify — a
# clean claiming three matches where one was never read is the false clean the
# caveat exists to prevent.
check_case "absent-count-excludes-unverified" 0 "2 expectation(s) match" \
    "$SANDBOX/nonexistent-hosts.yml" "$MANIFEST"

# Posture 2 — present and unreadable: fail-closed. A directory at the path is
# present and unreadable whatever uid runs the battery, where a chmod-000 file
# is readable by root and would make this case pass vacuously in a container.
mkdir -p "$SANDBOX/hosts-is-a-dir"
check_case "unreadable-fail-closed" 2 "not readable" "$SANDBOX/hosts-is-a-dir" "$MANIFEST"

# Posture 3 — present, no block for the configured host: a violation. The
# manifest says the account should be one thing and this machine is not logged
# in to that host at all, which is inside the manifest's authority.
cat >"$SANDBOX/other-host-only.yml" <<'EOF'
elsewhere.example:
    user: example-dev
EOF
check_case "no-host-block-violation" 1 "carries no block for host.example" \
    "$SANDBOX/other-host-only.yml" "$MANIFEST"

# An unrecognized shape is fail-closed rather than clean — the one posture that
# keeps a CLI config-format change from silently retiring the assertion.
cat >"$SANDBOX/no-user-key.yml" <<'EOF'
host.example:
    git_protocol: ssh
EOF
check_case "unrecognized-shape-fail-closed" 2 "no active-account key" \
    "$SANDBOX/no-user-key.yml" "$MANIFEST"

# The collision the parse is ruled over: `users` is the map of accounts available
# on this machine and the CLI writes it *before* the active-account key, so a
# startswith or substring match on `user` reads a structural key as a login. A
# login literally spelled `user` inside that map is reachable the same way, which
# is why the match is on the exact key token at the block's own indent.
cat >"$SANDBOX/collision.yml" <<'EOF'
host.example:
    users:
        example-dev:
            oauth_token: fixture-token
        user:
            oauth_token: fixture-token
    git_protocol: ssh
    user: example-dev
EOF
check_case "users-map-is-not-read-as-a-login" 0 "3 expectation(s) match" \
    "$SANDBOX/collision.yml" "$MANIFEST"

# The field count is fail-closed by the arm the manifest already had, not by a
# new one — a `gh-account` line carrying a host as well as a login is malformed.
check_case "wrong-field-count-fail-closed" 2 "malformed line(s)" \
    "$SANDBOX/collision.yml" 'gh-account host.example example-dev'

if [[ "$fails" -gt 0 ]]; then
    echo "check-identity.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-identity.test.sh: clean (absent clean-with-caveat, count excludes unverified, unreadable fail-closed, no-host-block red, unrecognized-shape fail-closed, users-map collision, wrong-field-count fail-closed, 7 cases)"
exit 0
