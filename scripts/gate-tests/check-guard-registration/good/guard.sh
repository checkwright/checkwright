#!/usr/bin/env bash
guard_rule_alpha() {
    return 0
}

guard_rule_beta() {
    return 0
}

guard_rule_gamma() {
    return 0
}

guard_generic_rules() {
    local cmd="$1"
    guard_rule_alpha "$cmd"

    guard_rule_beta "$cmd"
    guard_rule_gamma "$cmd"
}
