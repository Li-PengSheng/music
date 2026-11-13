export function duration(ms:number){
 const total=Math.max(0,Math.floor(ms/1000));
 const h=Math.floor(total/3600),m=Math.floor(total/60)%60,s=String(total%60).padStart(2,'0');
 return h?`${h}:${String(m).padStart(2,'0')}:${s}`:`${m}:${s}`;
}

/** First visible character for artist placeholders (works for CJK and emoji). */
export function initial(name:string){return Array.from(name.trim())[0]?.toUpperCase()??'?';}

/** Stable hue per name so placeholder tints stay consistent between sessions. */
export function nameHue(name:string){let h=0;for(const c of name)h=(h*31+c.codePointAt(0)!)%360;return h;}

export function errorText(e:unknown){
 if(e instanceof Error)return e.message;
 if(typeof e==='string')return e;
 try{return JSON.stringify(e);}catch{return String(e);}
}
