import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,readdirSync,statSync} from 'node:fs';
import {join} from 'node:path';
import {LOCAL_COLLECTION,localQuery,shadowsBuiltIn} from '../src/lib/utils/collections.ts';
import {playlistNameError} from '../src/lib/utils/playlist-name.ts';

const read=f=>readFileSync(f,'utf8');
function sources(dir){return readdirSync(dir).flatMap(n=>{const p=join(dir,n);return statSync(p).isDirectory()?sources(p):/\.(svelte|ts)$/.test(n)?[p]:[];});}

test('built-in 本地 is a stable route over the whole library, not a stored playlist',()=>{
 assert.equal(LOCAL_COLLECTION.view,'local');
 const q=localQuery('title','周');
 assert.deepEqual(q,{sort:'title',search:'周'});
 assert.ok(!('playlist' in q)&&!('favorite' in q),'no membership filter: imports and rescans show up automatically');
 const nav=read('src/lib/stores/nav.svelte.ts');
 assert.match(nav,/\{view:'local'\}/);
 const sidebar=read('src/lib/layout/Sidebar.svelte');
 assert.match(sidebar,/go\(\{view:'local'\}\)/,'opened by route kind');
 assert.match(sidebar,/library\.summary\.tracks\}<\/span>/,'count is the library size from the summary');
 const songs=read('src/lib/views/SongsView.svelte');
 assert.match(songs,/case 'local':return localQuery\(sort,''\)/);
 assert.match(songs,/route\.view==='songs'\|\|route\.view==='local'\|\|route\.view==='artist'/,'sortable');
});

test('built-in collection can be played and shuffled but never renamed or deleted',()=>{
 const actions=read('src/lib/actions.ts');
 const menu=actions.slice(actions.indexOf('export function localMenu'),actions.indexOf('export async function newPlaylist'));
 assert.match(menu,/播放全部/);assert.match(menu,/随机播放/);
 assert.doesNotMatch(menu,/deletePlaylist|renamePlaylist|删除|重命名/);
 const services=read('src/lib/services/music.ts');
 assert.doesNotMatch(services,/'local'/,'no IPC ever addresses the built-in collection');
});

test('a user playlist named 本地 keeps its data; the name is reserved only for new names',()=>{
 const lists=[{id:7,name:'本地'},{id:8,name:'夜晚'}];
 assert.equal(shadowsBuiltIn(' 本地 '),true);assert.equal(shadowsBuiltIn('本地音乐'),false);
 assert.equal(playlistNameError('本地',lists,7),null,'existing same-name playlist can be saved unchanged');
 assert.match(playlistNameError('本地',[{id:8,name:'夜晚'}]),/系统歌单/,'cannot create a new one');
 assert.match(playlistNameError('本地',[{id:8,name:'夜晚'}],8),/系统歌单/,'cannot rename into it');
 assert.equal(playlistNameError('夜晚 2',lists),null,'ordinary playlists unaffected');
 const sidebar=read('src/lib/layout/Sidebar.svelte');
 assert.match(sidebar,/\{#each library\.summary\.playlists as list \(list\.id\)\}/,'user playlists still listed by ID');
 assert.match(sidebar,/playlistMenu\(list\)/,'and keep rename/delete');
});

test('favourites removed from every UI surface without touching stored data',()=>{
 for(const file of sources('src')){
  const text=read(file);
  assert.doesNotMatch(text,/收藏|favoriteTrack|setFavorite|favoritesOnly|library_favorite|\bHeart\b/,file);
 }
 const songs=read('src/lib/views/SongsView.svelte');
 assert.doesNotMatch(songs,/favorite/);
 const rust=read('src-tauri/src/catalog/collections.rs');
 assert.match(rust,/CREATE|favorites/,'database layer and stored favourites are left in place');
 const db=read('src-tauri/src/catalog/db.rs');
 assert.match(db,/favorites/,'schema keeps the favourites table for compatibility');
});
