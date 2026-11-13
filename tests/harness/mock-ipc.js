// Browser-only layout harness. Loaded exclusively by tests/harness/index.html; the app never imports it.
// It answers Tauri commands with a synthetic in-memory library so lists, dialogs and the queue can be
// inspected in a normal browser. Playback state never advances on its own.
const TOTAL=Number(new URLSearchParams(location.search).get('tracks')??471);
const tracks=Array.from({length:TOTAL},(_,i)=>({
 id:i+1,path:`C:\\Fixture\\${String(i+1).padStart(3,'0')}.flac`,title:`测试歌曲 ${i+1}`,artist:`艺术家 ${i%17}`,album:`专辑 ${i%40}`,
 album_artist:'',genre:'',year:2020,track_number:(i%12)+1,disc_number:null,duration_ms:180000+i*997,added_at:TOTAL-i,album_source:'tag',
}));
const playlists=[{id:7,name:'本地',count:0},{id:8,name:'夜晚',count:2}];
const members={7:[],8:[3,5]};
const queue=tracks.slice(0,12).map(({path,title,artist,album,duration_ms})=>({path,title,artist,album,duration_ms}));
let snapshot={revision:1,track_id:queue[0].path,current_track:queue[0],playback_state:'paused',position_ms:42000,duration_ms:queue[0].duration_ms,
 volume:.7,playback_mode:'sequential',lyrics_offset_ms:0,output_device_id:'Fixture output',error:null,queue};
const prefs={close_to_tray:false,queue_open:false,theme:null};
const log=[];
window.__mockLog=log;
window.__mockSnapshot=()=>snapshot;
function page(q){
 let list=tracks;
 if(q.playlist!=null)list=(members[q.playlist]??[]).map(id=>tracks[id-1]);
 if(q.search){const s=q.search.toLowerCase();list=list.filter(t=>(t.title+t.artist+t.album).toLowerCase().includes(s));}
 if(q.artist)list=list.filter(t=>t.artist===q.artist);
 if(q.sort==='title')list=[...list].sort((a,b)=>a.title.localeCompare(b.title)||a.id-b.id);
 const offset=q.offset??0,limit=q.limit??100;
 return {tracks:list.slice(offset,offset+limit),total:list.length,favorites:[]};
}
const handlers={
 bridge_probe:a=>({nonce:a.nonce,backend:'rust',snapshot}),
 player_get_state:()=>snapshot,
 desktop_preferences:a=>{if(a.closeToTray!=null)prefs.close_to_tray=a.closeToTray;if(a.queueOpen!=null)prefs.queue_open=a.queueOpen;if(a.theme!==undefined&&a.theme!==null)prefs.theme=a.theme;return {...prefs};},
 media_status:()=>'Fixture',
 library_progress:()=>({running:false,visited:0,imported:0,current:'',cancelled:false,errors:[]}),
 library_summary:()=>({tracks:tracks.length,albums:[],artists:[],folders:['C:\\Fixture'],playlists:playlists.map(p=>({...p,count:members[p.id].length}))}),
 library_query:a=>page(a.query),
 library_home:()=>({recent:tracks.slice(0,6),artists:[]}),
 library_cover:()=>null,
 library_track:a=>tracks[a.id-1],
 library_audio_info:()=>({format:'Flac',sample_rate:44100,channels:2,bitrate:900}),
 library_playlist:a=>{if(a.name){if(playlists.some(p=>p.name===a.name.trim()))throw `已存在名为「${a.name.trim()}」的歌单`;const id=Math.max(...playlists.map(p=>p.id))+1;playlists.push({id,name:a.name.trim(),count:0});members[id]=[];return id;}members[a.playlistId].push(a.trackId);return null;},
 library_playlist_edit:()=>null,
 library_queue_tracks:()=>snapshot,
 library_play:()=>snapshot,
 library_queue:()=>snapshot,
 player_lyrics:()=>({lines:Array.from({length:30},(_,i)=>({time_ms:i*4000,text:`歌词第 ${i+1} 行`})),offset_ms:0,synced:true,source:'fixture.lrc'}),
 player_artwork:()=>null,
 player_configure:()=>snapshot,
 player_play:()=>snapshot,
 player_pause:()=>snapshot,
 player_set_volume:async a=>{await new Promise(r=>setTimeout(r,40));snapshot={...snapshot,volume:Math.min(1,Math.max(0,a.volume)),revision:snapshot.revision+1};return snapshot;},
};
let callbackId=0;
window.isTauri=true;
window.__TAURI_INTERNALS__={
 metadata:{currentWindow:{label:'main'},currentWebview:{windowLabel:'main',label:'main'}},
 transformCallback:(cb)=>{const id=++callbackId;window['_'+id]=cb;return id;},
 unregisterCallback:(id)=>{delete window['_'+id];},
 invoke:async(cmd,args={})=>{
  log.push({cmd,args});
  const h=handlers[cmd];
  if(cmd.startsWith('plugin:'))return null;
  if(!h)throw `mock: unhandled command ${cmd}`;
  return structuredClone(await h(args));
 },
};
