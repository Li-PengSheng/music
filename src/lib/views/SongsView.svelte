<script lang="ts">
 import {Play,Shuffle,ListEnd,Ellipsis,Search,Music2,FolderPlus,ListMusic} from 'lucide-svelte';
 import TrackList from '../components/TrackList.svelte';
 import EmptyState from '../components/EmptyState.svelte';
 import Select from '../components/Select.svelte';
 import Cover from '../components/Cover.svelte';
 import Artwork from '../components/Artwork.svelte';
 import {nav,memory,routeKey,go} from '../stores/nav.svelte';
 import {library} from '../stores/library.svelte';
 import {ui,openMenu} from '../stores/ui.svelte';
 import {control} from '../stores/player.svelte';
 import {setMode,desktopAvailable,type LibraryQuery} from '../services/music';
 import {playCollection,playlistMenu,albumMenu,artistMenu,importFolder} from '../actions';
 import {LOCAL_COLLECTION,localQuery} from '../utils/collections';

 let route=$derived(nav.route);
 let key=$derived(routeKey(route));

 let search=$state(ui.query);
 $effect(()=>{const q=ui.query;if(q===search)return;const t=setTimeout(()=>search=q,180);return ()=>clearTimeout(t);});

 let sorts=$state<Record<string,string>>({});
 let sort=$derived(sorts[key]??memory.get(key)?.sort??(route.view==='artist'?'album':'added'));
 function changeSort(next:string){sorts[key]=next;memory.set(key,{scroll:0,...memory.get(key),sort:next});}

 let album=$derived(route.view==='album'?library.summary.albums.find(a=>a.key===route.key)??null:null);
 let artist=$derived(route.view==='artist'?library.summary.artists.find(a=>a.name===route.name)??null:null);
 let playlist=$derived(route.view==='playlist'?library.summary.playlists.find(p=>p.id===route.id)??null:null);

 let base=$derived.by(():LibraryQuery=>{
  switch(route.view){
   case 'album':return {album:route.key,sort:'album'};
   case 'artist':return {artist:route.name,sort};
   case 'playlist':return {playlist:route.id};
   case 'recent':return {sort:'added'};
   case 'local':return localQuery(sort,'');
   default:return {sort};
  }
 });
 let query=$derived<LibraryQuery>({...base,search:search.trim()});
 let sortable=$derived(route.view==='songs'||route.view==='local'||route.view==='artist');

 let total=$state(0);let loaded=$state(false);
 let title=$derived(route.view==='songs'?'歌曲':route.view==='recent'?'最近添加':route.view==='local'?LOCAL_COLLECTION.name:album?.name??artist?.name??playlist?.name??'');

 async function shufflePlay(){await control(()=>setMode('shuffle'));await playCollection(base);}
 function headerMenu(e:MouseEvent){
  const target=e.currentTarget as HTMLElement;
  if(playlist)openMenu(target,playlistMenu(playlist),'歌单操作');
  else if(album)openMenu(target,albumMenu(album),'专辑操作');
  else if(artist)openMenu(target,artistMenu(artist),'艺术家操作');
 }
 const sortOptions=[{value:'added',label:'最近添加'},{value:'title',label:'标题'},{value:'artist',label:'艺术家'},{value:'album',label:'专辑与曲序'},{value:'duration',label:'时长'}];
</script>

{#if route.view==='album'||route.view==='artist'||route.view==='playlist'}
 <header class="detail-hero">
  <div class="hero-art" class:round={route.view==='artist'}>
   {#if album}<Cover id={album.cover_id} alt={album.name} size={360}/>
   {:else if route.view==='artist'}<Artwork kind="artist" alt={title}/>
   {:else}<Artwork kind="playlist" alt={title}/>{/if}
  </div>
  <div class="hero-info">
   <span class="hero-kind">{route.view==='album'?'专辑':route.view==='artist'?'艺术家':'歌单'}</span>
   <h2 class="hero-title">{title||(route.view==='playlist'?'歌单不存在':'未知')}</h2>
   <p class="hero-meta">
    {#if album&&album.artist}<button type="button" class="link" onclick={()=>go({view:'artist',name:album!.artist})}>{album.artist}</button><span aria-hidden="true"> · </span>{/if}
    {loaded?total:(album?.count??artist?.count??playlist?.count??0)} 首歌曲
   </p>
   <div class="hero-actions">
    <button type="button" class="btn btn-primary" disabled={!total} onclick={()=>playCollection(base)}><Play size={16} fill="currentColor"/>播放</button>
    <button type="button" class="btn btn-secondary" disabled={!total} onclick={shufflePlay}><Shuffle size={16}/>随机播放</button>
    <button type="button" class="btn btn-ghost" disabled={!total} onclick={()=>playCollection(base,false,title)}><ListEnd size={16}/>加入队列</button>
    {#if album||artist||playlist}<button type="button" class="icon-btn" aria-label="更多操作" aria-haspopup="menu" onclick={headerMenu}><Ellipsis size={18}/></button>{/if}
   </div>
  </div>
 </header>
{:else}
 <header class="page-header">
  <div>
   {#if route.view==='local'}<span class="hero-kind">系统歌单 · 自动包含音乐库中的全部歌曲</span>{/if}
   <h2 class="page-title">{title}</h2>
   <p class="page-meta">{loaded?`${total} 首歌曲`:'正在读取…'}{#if search.trim()} · 搜索“{search.trim()}”{/if}</p>
  </div>
  <div class="page-actions">
   <button type="button" class="btn btn-primary" disabled={!total} onclick={()=>playCollection({...base,search:search.trim()})}><Play size={16} fill="currentColor"/>播放全部</button>
   <button type="button" class="btn btn-secondary" disabled={!total} onclick={shufflePlay}><Shuffle size={16}/>随机播放</button>
  </div>
 </header>
 {#if route.view==='songs'||route.view==='local'}
  <div class="toolbar">
   <span class="toolbar-spacer"></span>
   <span class="toolbar-label">排序</span><Select label="歌曲排序" value={sort} options={sortOptions} onchange={changeSort}/>
  </div>
 {/if}
{/if}

{#if !desktopAvailable}
 <EmptyState icon={Music2} title="请在 Music Mine 桌面应用中使用" message="浏览器预览无法读取本地音乐库。"/>
{:else if library.loaded&&!library.summary.tracks}
 <EmptyState icon={FolderPlus} title="音乐库还是空的" message="导入你的 MP3、FLAC、WAV 文件夹。Music Mine 只读取文件，不会修改音乐或标签。">
  <button type="button" class="btn btn-primary" onclick={importFolder}><FolderPlus size={16}/>导入音乐文件夹</button>
 </EmptyState>
{:else if route.view==='playlist'&&!playlist&&library.loaded}
 <EmptyState icon={ListMusic} title="这个歌单已不存在" message="它可能已被删除。"/>
{:else}
 {#key key}
  <TrackList {query} memoryKey={key} playlistId={playlist?.id??null} numbering={route.view==='album'?'track':'index'} {sortable}
   onsort={sortable?changeSort:undefined} ontotal={(t,l)=>{total=t;loaded=l;}}/>
 {/key}
 {#if loaded&&!total}
  {#if search.trim()}<EmptyState icon={Search} title="没有找到匹配的歌曲" message={`没有与“${search.trim()}”匹配的歌曲。试试歌曲名、艺术家或专辑的其他关键词。`}><button type="button" class="btn btn-ghost" onclick={()=>ui.query=''}>清除搜索</button></EmptyState>
  {:else if route.view==='playlist'}<EmptyState icon={ListMusic} title="歌单还是空的" message="在任意歌曲上点右键，选择“添加到歌单”。"><button type="button" class="btn btn-ghost" onclick={()=>go({view:'songs'})}>浏览歌曲</button></EmptyState>
  {:else}<EmptyState icon={Music2} title="这里还没有歌曲"/>{/if}
 {/if}
{/if}
