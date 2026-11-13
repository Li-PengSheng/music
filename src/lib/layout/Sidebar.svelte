<script lang="ts">
 import {House,Clock3,Music2,Disc3,MicVocal,ListMusic,Plus,Settings2} from 'lucide-svelte';
 import {nav,go} from '../stores/nav.svelte';
 import {library} from '../stores/library.svelte';
 import {openMenu} from '../stores/ui.svelte';
 import {playlistMenu,newPlaylist} from '../actions';
 import {tooltip} from '../components/tooltip';

 let active=$derived(nav.route.view);
 let libraryActive=$derived(active==='songs'||active==='albums'||active==='artists'||active==='album'||active==='artist');
 const top=[{view:'home',label:'音乐空间',icon:House},{view:'recent',label:'最近添加',icon:Clock3}] as const;
 let sections=$derived([
  {view:'songs',label:'歌曲',icon:Music2,count:library.summary.tracks,match:['songs']},
  {view:'albums',label:'专辑',icon:Disc3,count:library.summary.albums.length,match:['albums','album']},
  {view:'artists',label:'艺术家',icon:MicVocal,count:library.summary.artists.length,match:['artists','artist']},
 ] as const);
</script>

<aside class="sidebar" aria-label="侧边栏">
 <button type="button" class="brand" onclick={()=>go({view:'home'})} aria-label="Music Mine 音乐空间">
  <span class="brand-mark" aria-hidden="true"><Music2 size={18} strokeWidth={2.4}/></span>
  <span class="brand-name">Music Mine</span>
 </button>

 <nav class="nav" aria-label="主导航">
  <ul class="nav-group">
   {#each top as item (item.view)}
    <li><button type="button" class="nav-item" class:active={active===item.view} aria-current={active===item.view?'page':undefined} onclick={()=>go({view:item.view})}>
     <item.icon size={18}/><span class="nav-label">{item.label}</span>
    </button></li>
   {/each}
  </ul>

  <div class="nav-heading" class:active-group={libraryActive}>音乐库</div>
  <ul class="nav-group">
   {#each sections as item (item.view)}
    {@const selected=(item.match as readonly string[]).includes(active)}
    <li><button type="button" class="nav-item" class:active={selected} aria-current={selected?'page':undefined} onclick={()=>go({view:item.view})}>
     <item.icon size={18}/><span class="nav-label">{item.label}</span>{#if library.loaded}<span class="nav-count">{item.count}</span>{/if}
    </button></li>
   {/each}
  </ul>

  <div class="nav-heading nav-heading-row">
   <span>我的歌单</span>
   <button type="button" class="icon-btn icon-btn-sm" aria-label="新建歌单" use:tooltip={'新建歌单'} onclick={()=>newPlaylist()}><Plus size={16}/></button>
  </div>
  <ul class="nav-group nav-playlists" aria-label="我的歌单">
   {#each library.summary.playlists as list (list.id)}
    {@const selected=nav.route.view==='playlist'&&nav.route.id===list.id}
    <li><button type="button" class="nav-item" class:active={selected} aria-current={selected?'page':undefined}
     onclick={()=>go({view:'playlist',id:list.id})}
     oncontextmenu={(e)=>openMenu(e,playlistMenu(list),'歌单「'+list.name+'」操作')}>
     <ListMusic size={18}/><span class="nav-label" use:tooltip={{text:list.name,overflow:true}}>{list.name}</span><span class="nav-count">{list.count}</span>
    </button></li>
   {:else}
    <li><button type="button" class="nav-item nav-item-muted" onclick={()=>newPlaylist()}><Plus size={18}/><span class="nav-label">新建歌单</span></button></li>
   {/each}
  </ul>
 </nav>

 <div class="sidebar-footer">
  <button type="button" class="nav-item" class:active={active==='settings'} aria-current={active==='settings'?'page':undefined} onclick={()=>go({view:'settings'})}>
   <Settings2 size={18}/><span class="nav-label">设置</span>
  </button>
 </div>
</aside>
