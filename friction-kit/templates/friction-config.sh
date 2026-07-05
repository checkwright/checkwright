# shellcheck shell=bash
# friction-kit consumer config. Copy into your gates dir (default scripts/), or
# point FRICTION_KIT_CONFIG_FILE elsewhere, and uncomment any knob you want to
# override. lib/guard.sh sources this first, then fills every unset knob with
# the platform-value default shown here.

# Per-iteration scratch logs (gitignore both, even where .workflow/ is committed).
# FRICTION_KIT_LOG="${GATE_SDK_WORKFLOW_DIR:-.workflow}/prompt-friction.log"
# FRICTION_KIT_WAKEUP_LOG="${GATE_SDK_WORKFLOW_DIR:-.workflow}/wakeup-attempts.log"

# Settings surfaces the triage tools read.
# FRICTION_KIT_SETTINGS=".claude/settings.json"
# FRICTION_KIT_SETTINGS_LOCAL=".claude/settings.local.json"

# Read-only repo scripts eligible for the absolute→relative rewrite (rule 4).
# Globs, matched against the script basename or its repo-relative path.
# FRICTION_KIT_RO_SCRIPTS=("check-*.sh")

# Gitignored scratch dirs named in the rule-3 corrective message.
# FRICTION_KIT_SCRATCH_DIRS=(".tmp")

# Read-only pipeline roster for the rule-8 auto-allow (leading binary per pipe
# segment). Add only binaries with no write side effect.
# FRICTION_KIT_RO_BINS=(grep egrep fgrep rg head tail cat wc sort uniq cut tr nl rev tac paste comm column diff jq find ls)
