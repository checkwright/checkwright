# shellcheck shell=bash
# spec: site-kit/SPEC.md §lib/site.sh — sourced config loader + defaults for the deployment-truth gate; this repo's layout as defaults

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
declare -p SITE_KIT_RENDERER &>/dev/null \
    || SITE_KIT_RENDERER=(ruby -e 'require "kramdown"; require "kramdown-parser-gfm"; STDOUT.write(Kramdown::Document.new(STDIN.read, input: "GFM").to_html)')
