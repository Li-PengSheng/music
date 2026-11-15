// Compares the visible rows' image hashes before (cover-<label>-RS.json) and after (cover-<label>-S.json) the recover step.
import {readFileSync} from 'node:fs';
for(const label of process.argv.slice(2)){
 const pick=file=>JSON.parse(readFileSync(`tests/perf/out/cover-${label}-${file}.json`,'utf8')).find(r=>r.name.startsWith('S ')).rows;
 const before=new Map(pick('RS').map(r=>[r.i,r.hash]));
 const after=pick('S');
 const changed=after.filter(r=>before.has(r.i)&&before.get(r.i)!==r.hash).map(r=>r.i);
 const same=after.filter(r=>before.get(r.i)===r.hash).map(r=>r.i);
 console.log(JSON.stringify({label,changed,same}));
}
