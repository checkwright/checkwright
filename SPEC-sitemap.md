# SPEC amendment: sitemap

The site serves no `sitemap.xml` and no `robots.txt`: both return 404 at the site root, and `docs/_config.yml` loads two plugins, `jekyll-relative-links` and `jekyll-optional-front-matter`. Crawlers index the site from its links alone.

This amendment adds `jekyll-sitemap` to the plugin list. The plugin writes both files, so no `robots.txt` is authored.

One queue entry pairs it: [site-sitemap-robots](TASK-QUEUE.md#site-sitemap-robots).

**The rulings.**

- **The plugin writes `robots.txt`, so the tree carries none.** jekyll-sitemap 1.4.0 adds a `robots.txt` page when the source has none, whose one line is `Sitemap:` and the sitemap's absolute URL. A hand-written file would be a second statement of that URL, which the plugin already derives from the site's URL.
- **No exclusion needs configuring.** The sitemap template lists `site.html_pages` less a page whose front matter sets `sitemap: false` and less `/404.html`. `docs/search.json` renders to a `.json` output, so it is not an HTML page and is never listed. The off-nav pages are public pages and are listed: off-nav is a menu decision, not a crawl one.
- **An XML sitemap only.** An HTML sitemap page was refused at filing, since the nav and the search already cover what one would. A crawler reads the XML file, and a reader never meets it.
- **The seam.** Repo-root only: the site's config and its architecture page. No kit surface changes.

## What changes

### (1) The plugin {mechanical}

**Not yet applied.** `docs/_config.yml`'s `plugins:` list gains `- jekyll-sitemap`, after the two it carries. The comment above the list is rewritten to one clause per plugin: *relative-links rewrites in-repo `.md` references to their built `.html`; optional-front-matter renders the front-matter-less pages through the chrome; sitemap writes `sitemap.xml` and, since the tree has none, `robots.txt` naming it.* Its first line becomes *GitHub Pages supports these; declaring them keeps a local build faithful.*, since the sitemap plugin is supported but is not one Pages enables by default.

### (2) The site-architecture sentence {mechanical}

**Not yet applied.** docs/site-architecture.md §Site chrome and the nav contract, after the first sentence: *`jekyll-sitemap` writes `sitemap.xml`, every HTML page less one setting `sitemap: false`, and a `robots.txt` naming it, so the tree carries neither file and a page leaves the sitemap only by that key.*

## Producers and consumers

Probe: `https://pages.github.com/versions.json`, read on the day of authoring, lists `"jekyll-sitemap":"1.4.0"` beside `"github-pages":"232"`. The plugin's source at tag `v1.4.0`, `lib/jekyll/jekyll-sitemap.rb`, adds `sitemap.xml` and `robots.txt` to the site's pages when the source lacks each. `lib/sitemap.xml` loops over `site.html_pages | where_exp:'doc','doc.sitemap != false' | where_exp:'doc','doc.url != "/404.html"'`, and `lib/robots.txt` reads `Sitemap: {{ "sitemap.xml" | absolute_url }}`. `git ls-files docs | grep -E 'robots|sitemap'` finds no tracked file, and `grep -rn '^sitemap:' docs` finds no page setting the key.

- **`sitemap.xml` and `robots.txt`** (delta 1). Producer: the Pages build, on each push to master. Consumer: a crawler, by fetch. No gate reads either: they exist only in the built site.
- **The absolute URL.** `absolute_url` prefixes the site's `url`. **Inferred, not probed:** that the Pages build sets `url` from the `CNAME`, through the `github-pages` gem's metadata plugin, since `docs/_config.yml` carries no `url:`. The post-push check below settles it: a relative or `http://` `Sitemap:` line means `url: https://checkwright.dev` joins `docs/_config.yml`.

## Existing sections updated

Roster probe: `git grep -n "_config.yml\|jekyll-optional-front-matter\|robots\|sitemap"` over the tracked tree, less `docs/posts/`, the generated mirrors and the queue.

- `docs/_config.yml` (delta 1).
- `docs/site-architecture.md` — §Site chrome and the nav contract (delta 2).
- `.workflow/surface-ceiling.txt` — the grown `docs/site-architecture.md` row re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling` (delta 2).

## Retired spellings

- None — the deltas add a plugin line and a sentence; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, checked by `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Served** — after the mid-iteration push's Pages run is green: `https://checkwright.dev/sitemap.xml` returns 200 and lists the home page and a release note but not `search.json`, and `https://checkwright.dev/robots.txt` returns 200 with a `Sitemap:` line naming `https://checkwright.dev/sitemap.xml`. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that check passes.
