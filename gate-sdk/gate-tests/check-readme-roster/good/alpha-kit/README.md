# alpha-kit (fixture)

A synthetic kit whose README registers both shipped gates — the parity
`check-readme-roster` must ACCEPT. The roster block nests in a list item (the
real READMEs' indented shape) and carries annotation clauses after each name.

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-alpha-one   # the first synthetic gate
   check-alpha-two   # the second, annotated like the real rosters
   ```
   <!-- gate-roster:end -->

Prose outside the markers naming check-alpha-ghost must not join the roster.
