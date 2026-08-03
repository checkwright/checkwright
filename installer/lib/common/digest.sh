# shellcheck shell=bash
# spec: installer/README.md §The gate binary — sourceable owner of the artifact digest mechanism: which hasher this host has and how a SHA-256 is taken with it. Deliberately not lock.sh's, because the two hashes answer different questions — the manifest's git hash-object is change detection over files init wrote, this one is an integrity claim against a value published outside the payload

# spec: installer/README.md §The gate binary — sha256sum first, then shasum -a 256, because stock macOS ships the second and not the first; an absent hasher is not a refusal, it is what selects the digest-unverifiable omission
digest_hasher() {   # -> the resolved hasher's name, empty when neither is on PATH
    if command -v sha256sum >/dev/null 2>&1; then
        printf 'sha256sum'
    elif command -v shasum >/dev/null 2>&1; then
        printf 'shasum'
    fi
}

digest_of() {   # $1 = file -> its SHA-256 in hex, empty when no hasher resolved
    case "$(digest_hasher)" in
        sha256sum) sha256sum -- "$1" 2>/dev/null | cut -d' ' -f1 ;;
        shasum)    shasum -a 256 -- "$1" 2>/dev/null | cut -d' ' -f1 ;;
    esac
}
