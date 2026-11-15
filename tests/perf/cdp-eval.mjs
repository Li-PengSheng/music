// Evaluates one expression in the running WebView over CDP: node tests/perf/cdp-eval.mjs <file-with-expression> [port]
import {readFileSync} from 'node:fs';
const [file,port='9333']=process.argv.slice(2);
const page=(await (await fetch(`http://127.0.0.1:${port}/json`)).json()).find(t=>t.type==='page');
const ws=new WebSocket(page.webSocketDebuggerUrl);
await new Promise((ok,fail)=>{ws.onopen=ok;ws.onerror=fail;});
ws.onmessage=({data})=>{const m=JSON.parse(data);if(m.id===1){console.log(JSON.stringify(m.result?.result?.value??m.result?.exceptionDetails??m.error,null,1));ws.close();}};
ws.send(JSON.stringify({id:1,method:'Runtime.evaluate',params:{expression:readFileSync(file,'utf8'),awaitPromise:true,returnByValue:true}}));
