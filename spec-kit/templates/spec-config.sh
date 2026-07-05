# shellcheck shell=bash
# Consumer spec config for spec-kit (spec-kit/SPEC.md §Layout and
# configuration). Copy into your gates dir as spec-config.sh (or point
# SPEC_KIT_CONFIG_FILE at it). Every knob is optional: anything left unset keeps
# the kit default shown here. A malformed config exits 2 — a broken grammar must
# not gate anything.

# The canonical spec filename and the amendment-file glob.
#SPEC_KIT_SPEC_NAME=SPEC.md
#SPEC_KIT_AMENDMENT_GLOB='SPEC-*.md'

# The governed queue file (repo-root-relative; the queue-facing gate also takes
# it as $1). Shares queue-kit's default; the two knobs are independent.
#SPEC_KIT_QUEUE_FILE=TASK-QUEUE.md

# Active-queue sections whose entries require a [spec:] ref (feature sections),
# and the broader active set where a [needs-spec] tag is a violation. Plain text
# spliced into a '^## (…)$' regex — avoid regex metacharacters. Cross-kit note:
# queue-kit's QUEUE_KIT_ACTIVE_SECTIONS carries the same default; a consumer
# renaming its sections sets both (independent knobs).
#SPEC_KIT_FEATURE_SECTIONS=("New Features")
#SPEC_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

# The parked section: every entry there requires [needs-spec].
#SPEC_KIT_DEFERRED_SECTION=Deferred

# The Definition-of-Done heading and how many a canonical spec may carry.
# 'exactly-one' (platform default) flags a spec with none; 'at-most-one' allows
# zero — set it for a reference-spec corpus whose specs legitimately carry no DoD.
#SPEC_KIT_DOD_HEADING="Definition of Done"
#SPEC_KIT_DOD_MODE=exactly-one

# The code-derivable heading set and the fenced-density budget (percent) above
# which such a section is a code dump. The pointer regex names the one-line index
# reference that exempts a shed section (your index tooling's marker).
#SPEC_KIT_BANNED_HEADINGS=("Directory Structure" "Public API" "Cargo.toml Dependencies")
#SPEC_KIT_DERIVABLE_DENSITY=60
#SPEC_KIT_DERIVABLE_POINTER_REGEX='pub-index|proto-index'

# check-spec-embedded-source calibration. SPEC_KIT_EMBED_LANGS is the scanned
# fence-language → source mapping: one 'kind|fence-alias,…|file-glob,…' entry per
# language family. A fence whose language is not a listed alias (or is in
# SPEC_KIT_EMBED_ILLUSTRATIVE) is treated as illustrative and skipped. The wire
# kind is the one fence an amendment may embed as a not-yet-merged contract delta.
#SPEC_KIT_EMBED_THRESHOLD=0.70
#SPEC_KIT_EMBED_MINLINES=8
#SPEC_KIT_EMBED_LANGS=(
#    "rs|rust,rs|*.rs"
#    "toml|toml|*.toml"
#    "sql|sql|*.sql"
#    "sh|bash,sh|*.sh"
#    "yaml|yaml,yml|*.yaml,*.yml"
#    "ts|typescript,ts,tsx|*.ts,*.tsx"
#    "rego|rego|*.rego"
#    "proto|proto,protobuf|*.proto"
#    "dockerfile|dockerfile|Dockerfile"
#)
#SPEC_KIT_EMBED_ILLUSTRATIVE=("json")
#SPEC_KIT_EMBED_WIRE_KIND=proto

# check-surface-duplication topology. The glossary owns the canonical
# definitions; the listed non-glossary surfaces are scanned for foreign
# bold-lead-in definitions (every component SPEC.md is added automatically).
# Register the gate only where the glossary exists — it exits 2 otherwise.
#SPEC_KIT_GLOSSARY_FILE=GLOSSARY.md
#SPEC_KIT_DUP_SURFACES=("VISION.md")
