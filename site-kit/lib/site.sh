# shellcheck shell=bash
# spec: site-kit/SPEC.md §lib/site.sh — sourced config loader + defaults for the deployment-truth gate; this repo's layout as defaults
# no-port: gate-sdk/SPEC.md §The kit-library port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope. This library is the config bridge's sole resolver for the SITE_KIT_* knobs: gate-sdk/SPEC.md §lib/gate.sh rules exactly one place a knob's value is computed, and the bridge computes it by sourcing this file, so a crate-side resolver would be the second producer criterion 6 refuses. Its only non-bridge reader left in the tree is its own gate-test, so the bridge is very nearly the whole of its live role. Structural, not a sizing judgment.

_st_cfg="${SITE_KIT_CONFIG_FILE:-}"
if [[ -n "$_st_cfg" ]]; then
    [[ -f "$_st_cfg" ]] || {
        echo "site-kit: SITE_KIT_CONFIG_FILE not found: $_st_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_st_cfg"
else
    _st_cfg="${GATE_SDK_GATES_DIR:-scripts}/site-config.sh"
    if [[ -f "$_st_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_st_cfg"
    fi
fi
unset _st_cfg

[[ -v SITE_KIT_CNAME ]]     || SITE_KIT_CNAME="docs/CNAME"
[[ -v SITE_KIT_SCAN_ROOT ]] || SITE_KIT_SCAN_ROOT="."
[[ -v SITE_KIT_DOCS_DIR ]]  || SITE_KIT_DOCS_DIR="docs"

declare -p SITE_KIT_ALIASES &>/dev/null || SITE_KIT_ALIASES=()
declare -p SITE_KIT_EXEMPT_PATHS &>/dev/null \
    || SITE_KIT_EXEMPT_PATHS=("*/gate-tests/*" "*docs/posts/*")
if declare -p SITE_KIT_RENDERER &>/dev/null; then
    _st_renderer_overridden=1
else
    _st_renderer_overridden=0
    SITE_KIT_RENDERER=(ruby -e 'require "kramdown"; require "kramdown-parser-gfm"; STDOUT.write(Kramdown::Document.new(STDIN.read, input: "GFM").to_html)')
fi

# spec: site-kit/SPEC.md §lib/site.sh — the one conditional default: filling the batch knob for a consumer who pinned SITE_KIT_RENDERER would replace their pinned oracle with this unpinned one and report clean against a parser build they rejected
if ! declare -p SITE_KIT_RENDERER_BATCH &>/dev/null; then
    if [[ "$_st_renderer_overridden" -eq 0 ]]; then
        SITE_KIT_RENDERER_BATCH=(ruby -e 'require "kramdown"; require "kramdown-parser-gfm"; d = STDIN.read.split("\x00", -1); d.pop; d.each { |s| STDOUT.write(Kramdown::Document.new(s, input: "GFM").to_html); STDOUT.write("\x00") }')
    else
        SITE_KIT_RENDERER_BATCH=()
    fi
fi
unset _st_renderer_overridden
