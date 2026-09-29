# installer

| verb | asks | reads |
| --- | --- | --- |
| `init` | vendor | the payload |
| `doctor` | check | the toolchain |
| `diff` | compare | the manifest |
| `update` | upgrade | the manifest |
| `uninstall` | reverse | the manifest |
| `demo` | show | the payload |

| flag | verbs | means |
| --- | --- | --- |
| `--profile` | `init`, `update` | vendor this profile |
| `--recipe` | `init`, `update` | apply this recipe |
| `--no-recipe` | `init`, `update` | clear the recipes |
| `--dry-run` | `init`, `update`, `uninstall` | print the plan |
| `--force` | `init`, `update`, `uninstall` | overwrite |
| `--no-commit` | `init`, `update`, `uninstall` | leave the commit |
