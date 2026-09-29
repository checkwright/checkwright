# installer, as the pinned release carries it

| verb | asks | reads |
| --- | --- | --- |
| `init` | vendor | the payload |
| `doctor` | check | the toolchain |
| `diff` | compare | the manifest |
| `update` | upgrade | the manifest |
| `uninstall` | reverse | the manifest |

| flag | verbs | means |
| --- | --- | --- |
| `--profile` | `init`, `update` | vendor this profile |
| `--dry-run` | `init`, `update`, `uninstall` | print the plan |
| `--force` | `init`, `update`, `uninstall` | overwrite |
| `--no-commit` | `init`, `update`, `uninstall` | leave the commit |
