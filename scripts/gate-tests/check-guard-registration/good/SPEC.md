# Fixture guard spec

## The generic ruleset

Each rule is one `guard_rule_<name>` function, dispatched in roster order.

1. **First rule** (`guard_rule_alpha`) — blocked.
   A continuation line citing rule 2 by number.

   An indented paragraph that repeats `guard_rule_alpha`.
2. **Second rule** (`guard_rule_beta`) — advised.
3. **Third rule** (`guard_rule_gamma`) — granted.

**Fall-through logging closes every call and is no rule.**

## Testing

1. A later numbered list outside the section is not the roster.
