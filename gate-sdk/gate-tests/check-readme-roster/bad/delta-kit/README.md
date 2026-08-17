# delta-kit (fixture)

A synthetic mixed-spelling kit whose roster drifted across both globs —
`check-readme-roster` must REJECT in both directions over the ported spelling:
shipped `check-delta-one` (a `.gate` descriptor) is absent from the roster, and
`check-delta-ghost` names no shipped check of either spelling.

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-delta-two     # the surviving shell declaration
   check-delta-ghost   # ported-away gate the roster never dropped
   ```
   <!-- gate-roster:end -->
