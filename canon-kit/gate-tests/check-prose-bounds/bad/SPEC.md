# widget-toolkit — a spec over its bounds

## The scan

The scan reads one file per kit root: it stops at the first unreadable file; it exits 2 on that file; it names the file and the reason; it never retries, because a retry would hide a transient fault the next run cannot reproduce, and the caller reads the exit status alone.

## Knobs

- `WIDGET_ROOT` names the start of the walk, and per the provenance seam it never lands as a kit literal.
- `WIDGET_DEPTH` bounds the descent, and per the provenance seam it never lands as a kit literal.
- `WIDGET_PRUNE` lists skipped directories, and per the provenance seam it never lands as a kit literal.
