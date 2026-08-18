# spec: gate-sdk/SPEC.md §check-identity — the case reaches the rule through the
# actual-source knobs, which is what makes the pair a parity oracle for the live arm
# rather than for a fixture-only second code path.
GATE_SDK_IDENTITY_FILE="identity.conf"
GATE_SDK_GIT_EMAIL_FILE="git-config-email"
GATE_SDK_GIT_REMOTES_FILE="git-remotes"
# spec: gate-sdk/SPEC.md §check-identity — the account kind's actual is the CLI's
# persisted hosts file, and the case pins it: left to derive, the case would read
# the operator's real config and stop being hermetic.
GATE_SDK_GH_HOSTS_FILE="gh-hosts.yml"
