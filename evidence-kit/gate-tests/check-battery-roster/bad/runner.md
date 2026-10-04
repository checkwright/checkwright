# fixture runner doc

1. Run the verification suites:

   <!-- battery-roster:begin -->
   ```bash
   bash bin/run-alpha.sh           # alpha
   bash bin/run-delta.sh           # a suite validate no longer runs
   ./bin/cw --run-gate-tests stale/gate-tests   # a door line naming a retired suite
   ```
   <!-- battery-roster:end -->

`bash bin/run-beta.sh` appears here, outside the markers, for a different
rhetorical job — so beta is undocumented and gamma is missing outright, while
the delta line and the stale door line resolve to no configured suite.
