# Fixture: marked fences that do not run as declared

<!-- fence-runnable -->
```bash
false
```

<!-- fence-runnable -->
```bash
printf '%s\n' "$never_assigned"
```

<!-- fence-runnable -->
```bash
curl -fsSL https://example.invalid/install.sh
```

<!-- fence-runnable -->

```bash
true
```

<!-- fence-runable -->
```bash
true
```

<!-- fence-runnable: exit=one -->
```bash
true
```
