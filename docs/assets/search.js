// Client-side docs search: filters the Liquid-emitted search.json in the browser.
// No CDN, no external service — the index ships from the same origin as the site.
(function () {
  var input = document.getElementById('site-search');
  var results = document.getElementById('search-results');
  if (!input || !results) return;

  var url = window.CK_SEARCH_URL || '/search.json';
  var index = [];
  var loaded = false;

  function load() {
    if (loaded) return Promise.resolve();
    return fetch(url)
      .then(function (r) { return r.json(); })
      .then(function (data) { index = data; loaded = true; })
      .catch(function () { index = []; loaded = true; });
  }

  function render(items) {
    results.innerHTML = '';
    items.slice(0, 12).forEach(function (it) {
      var li = document.createElement('li');
      var a = document.createElement('a');
      a.href = it.url;
      var t = document.createElement('span');
      t.textContent = it.title;
      a.appendChild(t);
      if (it.parent) {
        var crumb = document.createElement('span');
        crumb.className = 'hit-crumb';
        crumb.textContent = ' — ' + it.parent;
        a.appendChild(crumb);
      }
      li.appendChild(a);
      results.appendChild(li);
    });
  }

  function query(raw) {
    var q = raw.trim().toLowerCase();
    if (!q) { results.innerHTML = ''; return; }
    var hits = index.filter(function (it) {
      return (it.title && it.title.toLowerCase().indexOf(q) !== -1) ||
             (it.content && it.content.toLowerCase().indexOf(q) !== -1);
    });
    hits.sort(function (a, b) {
      var at = a.title.toLowerCase().indexOf(q) !== -1 ? 0 : 1;
      var bt = b.title.toLowerCase().indexOf(q) !== -1 ? 0 : 1;
      return at - bt;
    });
    render(hits);
  }

  input.addEventListener('focus', load);
  input.addEventListener('input', function () {
    load().then(function () { query(input.value); });
  });
})();
