---
title: Install
---

# Install

Vendor the kit, then:

1. Copy the config into place.

2. Give the queue file its header and each evidence file its skeleton — the
   stage-stamp file and the lesson-disposition file
   (`LESSON_EVIDENCE_FILE`, both boundary-reset to their header):

   ```
   ## Iteration: —
   ```

   ```
   # contract: lifecycle-kit/SPEC.md §check-stage-evidence

   ---

   ```

   ```
   # contract: lifecycle-kit/SPEC.md §check-lesson-disposition
   ```

3. Adopt the skills in your agent-skill directory.

## Verdict line

The consequence clause appends after the ` -> <verdict>` arrow, leaving it
disjoint from the width invariant below. A single-backtick span whose content
carries an angle-bracket placeholder is severed by kramdown's block-before-span
parse: the span never closes, so the page leaks a stray backtick *and* a raw
`<verdict>` tag into rendered text — both symptoms of assertion 1.
