# Released verbs

```sh
curl -fsSL https://example.test/install.sh | sh -s -- doctor
```

```powershell
& ([scriptblock]::Create((irm https://example.test/install.ps1))) diff
```

Run `npx checkwright update`, then `checkwright uninstall` to reverse it.

A flag route advertises the default verb: `sh -s -- --profile full`. A placeholder advertises none: `checkwright <verb>`.
