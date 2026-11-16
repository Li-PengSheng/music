// Runs before the app bundle so the first paint already uses the saved theme (no light/dark flash).
// Mirrors parseTheme/resolveTheme in src/lib/utils/theme.ts.
(function () {
  var choice = null;
  var palette = null;
  try {
    choice = localStorage.getItem('music-mine-theme');
    palette = localStorage.getItem('music-mine-palette');
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
  root.style.colorScheme = theme;
})();
