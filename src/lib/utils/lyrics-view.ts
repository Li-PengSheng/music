// U+2009 is the bilingual separator verified in Ailee - No No No.lrc.
// Ordinary whitespace and language changes are never treated as translation markers.
export function displayParts(text: string): string[] { return text.split('\u2009'); }
export function groupLines(lines: {time_ms:number;text:string}[], synced:boolean) {
 const groups: {time_ms:number;parts:string[]}[]=[];
 for(const line of lines){const last=groups.at(-1);const parts=displayParts(line.text);if(synced&&last?.time_ms===line.time_ms)last.parts.push(...parts);else groups.push({time_ms:line.time_ms,parts});}
 return groups;
}
export function activeLine(lines:{time_ms:number}[],position:number){let lo=0,hi=lines.length;while(lo<hi){const mid=(lo+hi)>>>1;if(lines[mid].time_ms<=position)lo=mid+1;else hi=mid;}return lo-1;}
export function scrollTarget(top:number,height:number,viewport:number,total:number){return Math.max(0,Math.min(total-viewport,top+height/2-viewport*.4));}
