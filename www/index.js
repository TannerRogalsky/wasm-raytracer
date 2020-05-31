// import main from 'playground';
import wasmExecutorPath from "file-loader!../pkg/wasm_executor_bg.wasm";
// main(wasmExecutorPath);

import init from 'exports-loader?wasm_bindgen!../pkg/wasm_executor';
import Worker from 'worker-loader!./worker.js';

const memory = new WebAssembly.Memory({
  initial:17,
  maximum:16384,
  shared:true
});
const worker = new Worker();

init(wasmExecutorPath, memory).then((instance) => {
  let i = instance.load_state();
  let loop = () => {
    let n = instance.load_state();
    if (i !== n) {
      console.log(n);
      i = n;
    }
    requestAnimationFrame(loop);
  }
  loop();

  worker.postMessage([init.__wbindgen_wasm_module, memory]); // Start the worker.
  setInterval(() => {
    worker.postMessage("ayy");
  }, 100);
})

// import workerSrc from "raw-loader!./worker.js";
// // import * as wasm from 'wasm-executor';
// import wasmExecutorPath from "file-loader!../pkg/wasm_executor_bg.wasm";

// const memory = new WebAssembly.Memory({
//   initial:17,
//   maximum:16384,
//   shared:true
// });

// (async () => {
//   const imports = {
//     wbg: {
//       memory,
//       __wbindgen_throw: function(arg0, arg1) {
//         throw new Error(getStringFromWasm0(arg0, arg1));
//       },
//     },
//   }
//   const wasm = await WebAssembly.instantiateStreaming(fetch(wasmExecutorPath), imports);

//   let i = wasm.instance.exports.load();
//   let loop = () => {
//     let n = wasm.instance.exports.load();
//     if (i !== n) {
//       console.log(n);
//       i = n;
//     }
//     requestAnimationFrame(loop);
//   }
//   loop();

//   worker.postMessage([wasm.module, memory]); // Start the worker.
//   setInterval(() => {
//     worker.postMessage("ayy");
//   }, 100);
// })();

// const blob = new Blob([workerSrc]);

// // Obtain a blob URL reference to our worker 'file'.
// const blobURL = window.URL.createObjectURL(blob);
// const worker = new Worker(blobURL);