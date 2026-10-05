// Copy button on every docs code block: copies the block's text as authored, wrapped lines whole.
// No CDN, no external service — the script ships from the same origin as the site.
(function () {
  if (!navigator.clipboard || !navigator.clipboard.writeText) return;

  function text(pre) {
    var code = pre.querySelector('code');
    if (code) return code.textContent;
    var out = '';
    pre.childNodes.forEach(function (n) {
      if (!(n.classList && n.classList.contains('ck-copy'))) out += n.textContent;
    });
    return out;
  }

  document.querySelectorAll('.markdown-body pre').forEach(function (pre) {
    var button = document.createElement('button');
    button.type = 'button';
    button.className = 'ck-copy';
    button.textContent = 'Copy';
    button.setAttribute('aria-label', 'Copy this code block to the clipboard');
    var timer = null;
    button.addEventListener('click', function () {
      navigator.clipboard.writeText(text(pre)).then(function () {
        button.textContent = 'Copied';
        clearTimeout(timer);
        timer = setTimeout(function () { button.textContent = 'Copy'; }, 2000);
      });
    });
    pre.classList.add('ck-copyable');
    pre.appendChild(button);
  });
})();
