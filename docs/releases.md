---
title: Releases
nav_order: 6
---

# Release notes

Every tagged release carries one dated note — the phase-B checklist the
[upgrade contract](install.md) names, its bump chosen by that page's
§Versioning criteria. The list below derives at render time from the notes
themselves: a note joins by carrying the `release:` front-matter key it
already carries for the upgrade tooling, so there is no second copy to
maintain and no index to regenerate. The GitHub Release for each tag points
at the same note.

{% assign notes = site.pages | where_exp: "p", "p.release" | sort: "path" | reverse %}
<ul>
{% for p in notes %}
  <li><a href="{{ p.url | relative_url }}">{{ p.release }}</a></li>
{% endfor %}
</ul>
