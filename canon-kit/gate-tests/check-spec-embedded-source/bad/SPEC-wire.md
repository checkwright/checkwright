# SPEC amendment: wire

The wire-kind exemption is scoped to the configured wire kind, **not** to
amendments generally. This amendment quotes the tracked implementation in a
`bash` fence rather than a wire fence, so the exemption does not reach it and the
block fires — which is the assertion this file exists to make.

```bash
#!/usr/bin/env bash
set -euo pipefail
alpha_one="first line of widget logic"
beta_two="second line of widget logic"
gamma_three="third line of widget logic"
delta_four="fourth line of widget logic"
epsilon_five="fifth line of widget logic"
zeta_six="sixth line of widget logic"
echo "widget processing complete"
```
