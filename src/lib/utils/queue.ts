import type {Track} from '../services/music';

/** True when two queue snapshots list the same tracks with the same displayed metadata, in order. */
export function sameTracks(a:readonly Track[],b:readonly Track[]){
 if(a===b)return true;
 if(a.length!==b.length)return false;
 for(let i=0;i<a.length;i++){
  const x=a[i],y=b[i];
  if(x.path!==y.path||x.title!==y.title||x.artist!==y.artist||x.album!==y.album||x.duration_ms!==y.duration_ms)return false;
 }
 return true;
}
