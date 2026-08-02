# fixture runner doc

1. Run the verification suites:

   <!-- battery-roster:begin -->
   ```bash
   bash bin/run-alpha.sh           # alpha
   bash bin/run-beta.sh            # beta — the env prefix is normalized away
   bash bin/run-gamma.sh --deep    # gamma — arguments are part of the command
   ```
   <!-- battery-roster:end -->

The indented markers, the fence lines, and the annotation clauses are all
outside what the parity compare reads.
