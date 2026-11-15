import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {parseTheme,resolveTheme,THEME_KEY} from '../src/lib/utils/theme.ts';

const read=f=>readFileSync(f,'utf8');

/** Run public/theme-init.js against a fake document, as WebView2 does before the bundle loads. */
function boot(stored,systemLight){
 const root={dataset:{},style:{}};
 const store=stored===undefined?{}:{[THEME_KEY]:stored};
 const context={
  localStorage:{getItem:k=>k in store?store[k]:null},
  matchMedia:q=>({matches:q==='(prefers-color-scheme: light)'?systemLight:!systemLight}),
  document:{documentElement:root},
 };
 context.window=context;
 vm.runInNewContext(read('public/theme-init.js'),context);
 return root;
}

test('existing users and unknown values default to dark',()=>{
 for(const v of [null,undefined,'','Dark','blue'])assert.equal(parseTheme(v),'dark');
 assert.equal(parseTheme('light'),'light');assert.equal(parseTheme('system'),'system');
 assert.equal(boot(undefined,true).dataset.theme,'dark','no saved choice stays dark even on a light system');
});

test('resolved theme follows the choice, and the system only for 跟随系统',()=>{
 assert.equal(resolveTheme('dark',false),'dark');
 assert.equal(resolveTheme('light',true),'light');
 assert.equal(resolveTheme('system',true),'dark');
 assert.equal(resolveTheme('system',false),'light');
});

test('pre-render script agrees with resolveTheme for every saved choice (no flash)',()=>{
 for(const choice of ['dark','light','system'])for(const systemLight of [true,false]){
  const root=boot(choice,systemLight);
  const expected=resolveTheme(choice,!systemLight);
  assert.equal(root.dataset.theme,expected,`${choice} / system ${systemLight?'light':'dark'}`);
  assert.equal(root.style.colorScheme,expected);
 }
 const html=read('index.html');
 const init=html.indexOf('<script src="/theme-init.js"></script>');
 assert.ok(init>0&&init<html.indexOf('</head>')&&init<html.indexOf('type="module"'),'classic script in <head>, before the bundle');
 assert.doesNotMatch(html,/<script>[^<]/,'no inline script: CSP is default-src self');
});

test('choice persists locally and in desktop preferences, and system changes are followed',()=>{
 const prefs=read('src/lib/stores/preferences.ts');
 assert.match(prefs,/localStorage\.setItem\(THEME_KEY,choice\)/);
 assert.match(prefs,/desktopPreferences\(null,null,choice\)/,'Rust applies it to the window on the next start');
 assert.match(prefs,/addEventListener\('change',onMedia\)/);
 assert.match(prefs,/onWindowThemeChanged\(t=>\{if\(ui\.theme==='system'\)paint\(t\);\}\)/);
 assert.doesNotMatch(prefs,/reduceMotion=.*theme|theme.*reduceMotion=/,'theme never touches reduce-motion');
 const services=read('src/lib/services/music.ts');
 assert.match(services,/getCurrentWindow\(\)\.setTheme\(choice==='system'\?null:choice\)/);
 assert.match(read('src-tauri/capabilities/default.json'),/"core:window:allow-set-theme"/);
 assert.match(read('src-tauri/tauri.conf.json'),/"theme":"Dark","backgroundColor":"#121414"/);
 const desktop=read('src-tauri/src/platform/desktop.rs');
 assert.match(desktop,/w\.set_theme\(window_theme\(&preferences\.theme\)\)\?/);
 const settings=read('src/lib/views/SettingsView.svelte');
 assert.match(settings,/<h3 id="set-appearance">外观<\/h3>/);
 for(const label of ['深色','浅色','跟随系统'])assert.match(settings,new RegExp(`label:'${label}'`));
});

test('both themes share one set of tokens and components',()=>{
 const tokens=read('src/lib/styles/tokens.css');
 const [dark,light]=tokens.split(':root[data-theme=light]');
 const names=block=>new Set([...block.matchAll(/(--[\w-]+):/g)].map(m=>m[1]));
 const darkNames=names(dark);
 for(const name of names(light))assert.ok(darkNames.has(name),`${name} overrides an existing token`);
 for(const [name,value] of [['--bg','#f3f5f1'],['--bg-sidebar','#e9ede7'],['--surface','#fafbf8'],['--surface-hover','#e2e8e0'],['--text','#29342c'],['--accent','#52705a']])
  assert.match(light,new RegExp(`${name}:${value};`,'i'));
 const css=read('src/app.css').replace(/\/\*[\s\S]*?\*\//g,'');
 const colours=[...css.matchAll(/#[0-9a-f]{3,8}\b|rgba?\((?!var\()[^)]*\)/gi)].map(m=>m[0].replace(/\s+/g,''));
 const allowed=new Set(['#000','rgba(0,0,0,.5)','rgba(0,0,0,.4)']);
 assert.deepEqual(colours.filter(c=>!allowed.has(c)),[],'only masks and shadows over artwork stay literal');
});
