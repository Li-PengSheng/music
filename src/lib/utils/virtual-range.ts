/** Visible row window for a fixed-height list inside a scroll container. */
export function visibleRange(scrollTop:number,viewport:number,listTop:number,rowHeight:number,total:number,overscan=8){
 if(total<=0||rowHeight<=0)return {start:0,end:0};
 const first=Math.floor((scrollTop-listTop)/rowHeight);
 const last=Math.ceil((scrollTop-listTop+viewport)/rowHeight);
 const start=Math.max(0,Math.min(total,first-overscan));
 const end=Math.max(start,Math.min(total,last+overscan));
 return {start,end};
}

/** Page numbers (of `size` rows) needed to render rows [start, end). */
export function pagesFor(start:number,end:number,size:number){
 const pages:number[]=[];
 if(end<=start)return pages;
 for(let p=Math.floor(start/size);p<=Math.floor((end-1)/size);p++)pages.push(p);
 return pages;
}

/** Scroll offset that brings row `index` fully into view, or null when already visible. */
export function scrollToRow(index:number,scrollTop:number,viewport:number,listTop:number,rowHeight:number,padTop=0){
 const top=listTop+index*rowHeight;
 const bottom=top+rowHeight;
 if(top-padTop<scrollTop)return top-padTop;
 if(bottom>scrollTop+viewport)return bottom-viewport;
 return null;
}
