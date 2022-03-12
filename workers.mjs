const url=new URL('hjson.wasm',location.href).href;
await (await fetch(url)).arrayBuffer();
const src=name=>`(async()=>{
  const mod=await WebAssembly.compileStreaming(await fetch('${url}',{cache:'force-cache'}));
  const wasm=(await WebAssembly.instantiate(mod)).exports;
  const malloc=wasm.__wbindgen_malloc;const free=wasm.__wbindgen_free;
  const f=data=>{
    const ptr=malloc(data.length);
    new Uint8Array(wasm.memory.buffer).set(data,ptr);
    wasm.${name}(8,ptr,data.length);
    const arr=new Int32Array(wasm.memory.buffer);
    const ptr2=arr[2];const len2=arr[3];
    const result=new Uint8Array(wasm.memory.buffer).subarray(arr[2],arr[2]+arr[3]).slice();
    free(ptr2,len2);
    return result;
  };
  onmessage=async msg=>{
    postMessage(f(msg.data));
  };
  postMessage('ready');
})();`;
const [minifyWorker,indentWorker]=await Promise.all(['minify','indent'].map(name=>new Promise(resolve=>{
  const worker=new Worker(URL.createObjectURL(new Blob([src(name)],{type:'application/javascript'})));
  worker.onmessage=msg=>{
    if(msg.data==='ready'){
      worker.onmessage=null;
      resolve(worker);
    }
  };
})));

export {minifyWorker,indentWorker};
