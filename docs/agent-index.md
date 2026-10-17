---
title: Agent index
---

# The agent index

This page holds the contract of `docs/llms.txt` and `docs/llms-full.txt`, the agent index the [roster of generated projections](generated-projections.md) lists with its regeneration command and its freshness gate.

## Inputs

The docs root is `SITE_KIT_DOCS_DIR`. The site title is the first `title:` line of its `_config.yml` and the host the first line of its `CNAME`, so a page's URL is `https://<host>/` plus its path as the Pages build serves it: `index.md` as its directory, any other page `.html` for `.md`. The summary is the product statement's plain rendering, read through that statement's own parser. The pages are the nav model `check-docs-nav-reachable` reads, one crate module both call ([§Site chrome and the nav contract](site-architecture.md#site-chrome-and-the-nav-contract)), and a title, an order or a note is a front-matter value with one pair of enclosing quotes dropped.

## llms.txt

The file opens `# <site title>`, a blank line and `> <summary>`. A section per top-level page follows in `nav_order`, `index.md` excepted since the header stands for it. The section's heading is the page's title. Its items are the page, then its nav children in `nav_child_order`, each a list item linking the page's title to its URL, followed by `: <note>` where the page sets `description:`.

`## Optional` comes last. It lists each item's generated directory siblings by title and its `nav_suffix` pairs as written, item by item in the order above, labelled `<item title> <sibling title>` or `<item title> <pair label>`. Then come the top-level pages naming `nav_children_key`, which take no section and leave their derived children unlisted. A page off the menu carries no nav key, so it reaches neither part, and no exclusion list is read.

## llms-full.txt

The file opens with a `door-contributor` declaration, because every door in it is copied from a menu page the door gate already sweeps at its source, and a generated file's doors take their declaration from the emitter ([`guard-kit/SPEC.md` §check-door-binding](guard-kit/SPEC.md#check-door-binding)). The same header follows, then `index.md` and every item outside `## Optional` in the order `llms.txt` gives them. Each body is preceded by `Source: <url>` and a blank line. A body drops its front matter and every HTML comment outside a fence, and a comment filling its lines takes them and one blank line after them.

Every inline link target outside a fence or a code span becomes absolute. A scheme-led target stays. An `#anchor` takes the page's own URL before it, and any other target resolves against the page's directory, or against the docs root where it opens with `/`: a `.md` path takes the URL rule above, any other path becomes `https://<host>/<path>`, and its anchor is kept.

## The arm and its gate

Without `--write` the arm prints `llms.txt`, and `--write` writes neither file until both render. Both refuse at exit 2 on an unreadable input, a `_config.yml` with no `title:` line, an empty `CNAME`, a listed page with no `title:`, or a `nav_suffix` pair naming no docs page. They also refuse on a link target resolving outside the docs root, and on a body carrying Liquid template syntax, which its raw source would publish. `check-llms-fresh` compares both files byte for byte with the arm's rendering, in process. It also exits 2 on an absent projection file and names each stale one. Its positional form is `check-llms-fresh [docs-root]`.
