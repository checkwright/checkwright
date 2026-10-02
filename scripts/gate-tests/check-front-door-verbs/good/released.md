# Released verbs

```sh
curl -fsSL https://example.test/install.sh | sh -s -- doctor
```

```powershell
& ([scriptblock]::Create((irm https://example.test/install.ps1))) diff
```

Run `npx checkwright update`, then `checkwright uninstall` to reverse it.

A route names its verb before its flags: `sh -s -- init --profile full`. A placeholder advertises none: `checkwright <verb>`, and neither does the help arm: `checkwright --help`.

A toolkit page's fenced line stands for the reader's install line:

```sh
checkwright init --profile=prose --dry-run   # plan first
```

The gate binary's own arms, named apart from any route: `--emit env-probe`, also spelled `--emit-env-probe`, and `--run`. An installer flag named apart from its route is no arm: `--profile`.
