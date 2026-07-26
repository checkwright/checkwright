# shellcheck shell=bash
# spec: installer/README.md §The manifest — sourceable owner of the checkwright.lock schema: the wire key, the field accessors, and how a recorded content hash is obtained, so the verb that writes the manifest and the verbs that read it share one definition instead of a copy each

# spec: installer/README.md §The manifest — the versioned wire key; a build refuses a schema it does not know rather than guessing at an unknown shape
CHECKWRIGHT_LOCK_SCHEMA="checkwright-lock v1"
CHECKWRIGHT_LOCK_FILE="checkwright.lock"

lock_path() {   # $1 = repo root -> the manifest's path within it
    printf '%s/%s' "${1%/}" "$CHECKWRIGHT_LOCK_FILE"
}

lock_schema_ok() {   # $1 = manifest path -> 0 iff .schema is the key this build knows
    local got
    got="$(jq -r 'if type == "object" then (.schema // "") else "" end' "$1" 2>/dev/null)" || return 1
    [[ "$got" == "$CHECKWRIGHT_LOCK_SCHEMA" ]]
}

lock_field() {   # $1 = manifest path, $2 = field name -> its value, arrays space-joined, empty when absent
    jq -r --arg f "$2" '
        (if type == "object" then getpath([$f]) else null end)
        | if . == null then "" elif type == "array" then join(" ") else tostring end
    ' "$1" 2>/dev/null
}

# spec: installer/README.md §The manifest — git's object hash, never sha256sum: macOS ships shasum instead, and git is already a floor-contract member, so the manifest's integrity story stays inside the toolchain the contract asserts
lock_hash() {   # $1 = file -> the content hash a files[] entry records
    git hash-object -- "$1"
}
