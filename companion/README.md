# Checkwright companion

Checkwright's gates over a repository whose specs another toolkit writes. This directory holds what makes the `prose` profile govern a Spec Kit or OpenSpec spec tree, and the tests that prove it.

## What it holds

- **A recipe per toolkit**, in `<toolkit>/recipe/`: the knob lines and the dropped gate that fit the profile to the toolkit's layout, applied by one block of `sh` after `init`.
- **A Spec Kit extension**, in `speckit/`, whose install command installs Checkwright and applies the Spec Kit recipe for you.
- **A fixture tree per toolkit**, in `fixtures/`, in the toolkit's own layout, with one planted defect per claimed gate. The consumer smoke installs the profile on each tree, applies the recipe, and asserts the battery green and each defect caught.
- **The tested versions**, in `toolkits.list`.

## Using it

- **Spec Kit:** add the extension from a Checkwright release and run its install command ([speckit/README.md](speckit/README.md)).
- **OpenSpec:** install the `prose` profile, then run the recipe block with the OpenSpec recipe.

Both routes, what the gates catch in a spec tree and the tested versions are on the [Spec Kit and OpenSpec](https://checkwright.dev/spec-toolkits.html) page.

## Where the design lives

[`companion/SPEC.md`](SPEC.md): the recipe format and procedure, each recipe line with the idiom it answers, the tested claim, the fixture rules, the extension's contract, the pack step, the toolkit legs and the honest limits.
