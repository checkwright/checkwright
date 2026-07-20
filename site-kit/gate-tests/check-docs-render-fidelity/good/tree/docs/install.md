---
title: Install
---

# Install

Vendor the kit, then:

1. Copy the config into place.

2. Give the queue file its header and each evidence file its skeleton — the
   stage-stamp file and the lesson-disposition file
   (`LESSON_EVIDENCE_FILE`, both boundary-reset to their header).
   The queue header line:

       ## Iteration: —

   The stage-stamp skeleton:

       # contract: lifecycle-kit/SPEC.md §check-stage-evidence

       ---

   The lesson-disposition skeleton:

       # contract: lifecycle-kit/SPEC.md §check-lesson-disposition

3. Adopt the skills in your agent-skill directory.

## Verdict line

The consequence clause appends after the `` -> <verdict> `` arrow, leaving it
disjoint from the width invariant below. The doubled-backtick span is the
faithful form: the placeholder renders as escaped entities inside a code
element, so neither a stray backtick nor a raw tag reaches the scanned text.
