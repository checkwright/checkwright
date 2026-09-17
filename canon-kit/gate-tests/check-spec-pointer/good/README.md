# Widget fixture

Free prose can cite SPEC-widget.md §The widget contract inline in a sentence,
and the prose-citation pass resolves the heading past the trailing prose.

- An indented bullet may wrap a citation mid-heading: SPEC-widget.md §The
  widget contract reassembles across the break, the continuation line's own
  indentation stripped before the joining space rather than folded into the
  heading fragment.

## Citing without a path

A citation with no path resolves against every governed file's headings: this
file's own §Citing without a path, the sibling spec's §The widget contract, and
a sentence-shaped heading named by its lead clause, §The probe is asymmetric,
all resolve. So does a link-wrapped citation,
[the widget spec](SPEC-widget.md) §The widget contract, and a quoted path,
`SPEC-widget.md` §The widget contract. Prose about the grammar writes the
placeholder §<heading>, and a `§` quoted inside a code span is the mark itself.
