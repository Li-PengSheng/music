// Runs before the app bundle so the first paint already uses the saved theme (no light/dark flash).
// Mirrors parseTheme/resolveTheme in src/lib/utils/theme.ts.
(function(){
 var choice=null;
 try{choice=localStorage.getItem('music-mine-theme');}catch(e){/* storage unavailable: dark */}
 var theme=choice==='light'?'light':choice==='system'&&window.matchMedia&&matchMedia('(prefers-color-scheme: light)').matches?'light':'dark';
 var root=document.documentElement;
 root.dataset.theme=theme;
 root.style.colorScheme=theme;
})();
