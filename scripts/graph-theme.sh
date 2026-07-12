# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §check-graph — this repo's consumer graph-theme: inline the docs-site visual tokens + chrome (light+dark) into docs/check-graph.html so the generated artifact reads as the same site; the self-contained-artifact rule bars linking the site stylesheet, so the Primer-shaped tokens are inlined and the mark is an inline SVG (no asset href for assertion F to police)

graph_theme_css() {
    cat <<'CSS'
    :root {
      color-scheme: light dark;
      --ck-bg: #ffffff; --ck-fg: #1f2328; --ck-muted: #656d76;
      --ck-border: #d0d7de; --ck-sidebar: #f6f8fa; --ck-accent: #9a6f16;
      --ck-header-bg: #16202b; --ck-header-fg: #e6edf3; --ck-gold: #c19a3d;
      --ck-link: #0969da; --ck-code-bg: #f6f8fa;
    }
    @media (prefers-color-scheme: dark) {
      :root:not([data-theme]) {
        --ck-bg: #0d1117; --ck-fg: #e6edf3; --ck-muted: #8b949e;
        --ck-border: #30363d; --ck-sidebar: #161b22; --ck-accent: #d3a84a;
        --ck-link: #58a6ff; --ck-code-bg: #161b22; color-scheme: dark;
      }
    }
    :root[data-theme="dark"] {
      --ck-bg: #0d1117; --ck-fg: #e6edf3; --ck-muted: #8b949e;
      --ck-border: #30363d; --ck-sidebar: #161b22; --ck-accent: #d3a84a;
      --ck-link: #58a6ff; --ck-code-bg: #161b22; color-scheme: dark;
    }
    * { box-sizing: border-box; }
    body { margin: 0; background: var(--ck-bg); color: var(--ck-fg);
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
      line-height: 1.5; }
    a { color: var(--ck-link); }
    .site-header { display: flex; align-items: center; gap: 0.6rem; padding: 0.6rem 1.25rem;
      background: var(--ck-header-bg); color: var(--ck-header-fg);
      border-top: 4px solid var(--ck-gold); position: sticky; top: 0; z-index: 20; }
    .site-header .brand { display: flex; align-items: center; gap: 0.6rem;
      color: var(--ck-header-fg); text-decoration: none; font-weight: 600; font-size: 1.15rem; }
    .site-header .brand svg { width: 30px; height: 30px; display: block; }
    .site-header .spacer { flex: 1; }
    .site-header .tagline { color: var(--ck-header-fg); opacity: 0.7; font-size: 0.85rem; }
    header:not(.site-header) { padding: 1rem 1.5rem; border-bottom: 1px solid var(--ck-border); }
    header:not(.site-header) h1 { margin: 0 0 .35rem; font-size: 1.25rem; }
    header:not(.site-header) p { margin: .25rem 0; max-width: 68rem; }
    header:not(.site-header) code { background: var(--ck-code-bg); border-radius: 4px;
      padding: 0.1em 0.35em; font-size: 0.9em; }
    .status { font-size: .85rem; color: var(--ck-muted); }
    main { padding: 1rem 1.5rem; }
    .legend { margin-bottom: .75rem; font-size: .9rem; }
    .legend .valve { display: inline-block; width: 22px; border-top: 3px solid #d97706; }
    .viewport { border: 1px solid var(--ck-border); border-radius: 8px; padding: 16px;
      background: var(--ck-sidebar); overflow: auto; height: 78vh; cursor: grab; }
    .viewport.grabbing { cursor: grabbing; }
    .viewport svg { transform-origin: 0 0; max-width: none; }
    .hint { margin: .5rem 0 0; font-size: .8rem; color: var(--ck-muted); }
    .site-footer { border-top: 1px solid var(--ck-border); color: var(--ck-muted);
      padding: 1.25rem 1.5rem; font-size: 0.85rem; }
    .site-footer a { color: var(--ck-muted); }
CSS
}

graph_theme_header() {
    cat <<'HTML'
  <script>
    // Honor the docs site's persisted checkwright-theme choice before first
    // paint so the graph chrome matches the rest of the site (shared origin,
    // shared localStorage key owned by docs/_layouts/default.html).
    (function () {
      try {
        var t = localStorage.getItem('checkwright-theme');
        if (t === 'light' || t === 'dark') document.documentElement.setAttribute('data-theme', t);
      } catch (e) {}
    })();
  </script>
  <header class="site-header">
    <a class="brand" href="https://checkwright.dev/">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 240 240" aria-hidden="true">
        <g transform="translate(120 120) scale(1.4) translate(-103 -65.9) rotate(-45 86 114)">
          <path fill="#5B6B7F" fill-rule="evenodd" d="M86,96 H204 V114 H86 Z M124,96 h3 v9 h-3 z M136,96 h3 v6 h-3 z M148,96 h3 v9 h-3 z M160,96 h3 v6 h-3 z M172,96 h3 v9 h-3 z M184,96 h3 v6 h-3 z M196,96 h3 v9 h-3 z"/>
          <path fill="#C19A3D" fill-rule="evenodd" d="M86,44 H112 V114 H86 Z M105,58 a6,6 0 1 1 -12,0 a6,6 0 1 1 12,0 z"/>
        </g>
      </svg>
      <span>Checkwright</span>
    </a>
    <span class="spacer"></span>
    <span class="tagline">Check-coupling graph</span>
  </header>
HTML
}

graph_theme_footer() {
    cat <<'HTML'
  <footer class="site-footer">
    Checkwright — governed by the same kits it ships.
    <a href="https://github.com/checkwright/checkwright">Source on GitHub</a>.
  </footer>
HTML
}
