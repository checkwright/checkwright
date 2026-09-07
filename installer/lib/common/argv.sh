# shellcheck shell=bash
# spec: installer/README.md §init — sourceable owner of the rule that an install's size is never bounded by the host's argv width: every git call naming the whole vendored roster is issued through here, so the batching is one implementation rather than a discipline each call site has to remember

# comment-tier-exempt: a native Windows process is handed at most 32767 command-line characters and MinGW's git is one, which is the tightest ceiling this actually runs under; the budget is a fraction of it so the fixed argv and the caller's own quoting cannot close the gap
ARGV_BATCH_BYTES=4096

argv_batched() {   # argv_batched <fixed argv>… -- <path>… -> runs '<fixed argv> -- <chunk>' once per chunk, stopping at the first non-zero
    local -a fixed=() chunk=()
    local sep=0 used=0 arg
    for arg in "$@"; do
        if (( ! sep )); then
            if [[ "$arg" == "--" ]]; then sep=1; else fixed+=("$arg"); fi
            continue
        fi
        if (( ${#chunk[@]} > 0 && used + ${#arg} + 1 > ARGV_BATCH_BYTES )); then
            "${fixed[@]}" -- "${chunk[@]}" || return $?
            chunk=(); used=0
        fi
        chunk+=("$arg"); used=$(( used + ${#arg} + 1 ))
    done
    (( ${#chunk[@]} > 0 )) || return 0
    "${fixed[@]}" -- "${chunk[@]}"
}
