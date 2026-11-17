// Apply saved appearance before the bundle/first paint, including the local font family.
// Mirrors the parsers in src/lib/utils/theme.ts.
(function () {
  var choice = null;
  var palette = null;
  var font = null;
  try {
    choice = localStorage.getItem('music-mine-theme');
    palette = localStorage.getItem('music-mine-palette');
    font = localStorage.getItem('music-mine-font');
  } catch (e) {
    /* storage unavailable: light */
  }
  var theme =
    choice === 'dark'
      ? 'dark'
      : choice === 'system' &&
          window.matchMedia &&
          matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';
  var root = document.documentElement;
  root.dataset.theme = theme;
  root.dataset.palette =
    palette === 'cinnabar' || palette === 'azure' || palette === 'amber' ? palette : 'celadon';
  root.dataset.font =
    font === 'puhuiti' || font === 'serif' || font === 'kaiti' || font === 'system'
      ? font
      : 'harmony';
  root.style.colorScheme = theme;
})();
