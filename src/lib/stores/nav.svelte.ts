import {createHistory} from '../utils/history';

export type Route =
 | {view:'home'}
 | {view:'songs'}
 | {view:'recent'}
 | {view:'albums'}
 | {view:'artists'}
 | {view:'album';key:string}
 | {view:'artist';name:string}
 | {view:'playlist';id:number}
 /** Built-in “本地” collection: the whole library. */
 | {view:'local'}
 | {view:'settings'};

const history=createHistory<Route>({view:'home'});

export const nav=$state({route:history.current as Route,canBack:false,canForward:false});

function sync(){nav.route=history.current;nav.canBack=history.canBack;nav.canForward=history.canForward;}

export function go(route:Route){history.push(route);sync();}
export function back(){history.back();sync();}
export function forward(){history.forward();sync();}
export function forgetPlaylist(id:number){history.forget(r=>r.view==='playlist'&&r.id===id,{view:'songs'});sync();}

/** Stable key for per-page UI memory such as scroll position. */
export function routeKey(route:Route){
 switch(route.view){
  case 'album':return 'album:'+route.key;
  case 'artist':return 'artist:'+route.name;
  case 'playlist':return 'playlist:'+route.id;
  default:return route.view;
 }
}

/** Remembered per-route list state (scroll offset and sort) for the session. */
export const memory=new Map<string,{scroll:number;sort?:string}>();
