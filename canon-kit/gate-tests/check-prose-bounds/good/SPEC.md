# widget-toolkit — a bounded spec

## The scan

The scan reads one file per kit root. It stops at the first unreadable file and exits 2.

A short roster follows, each bullet stating its own rule:

- `WIDGET_ROOT` names the tree the scan starts from.
- `WIDGET_DEPTH` bounds how far below it the walk descends.
- `WIDGET_PRUNE` lists the directories the walk never enters.

Every knob is read by this gate alone, which the lead-in above says once for the whole roster.

<!-- prose-bound-exempt: the fixture's valved keep, one sentence deliberately left long to prove the valve suppresses it at the site -->
This sentence is deliberately long, because it demonstrates that a valve on the line above suppresses the sentence-length finding at this one site, and so it keeps running well past the bound of forty-five words that the gate applies to every other sentence in the governed corpus, without any finding at all.

| a table row may run as long as it likes, since tables are held out of every assertion the gate makes over its governed prose corpus, and this one proves it | x |
|---|---|

```text
A fenced block is shown grammar, so this long line is never read as a sentence even though it runs well past the bound of forty-five words that the gate applies to every other sentence in the corpus it governs.
```

<!-- roster:begin -->
A generated region is byte-gated elsewhere, so this long line is never read as a sentence even though it runs well past the bound of forty-five words that the gate applies to every other sentence in its corpus.
<!-- roster:end -->
