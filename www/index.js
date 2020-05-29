import workerSrc from "raw-loader!./worker.js";
// import * as wasm from 'wasm-executor';
import wasmExecutorPath from "../pkg/wasm_executor_bg.wasm";

const memory = new WebAssembly.Memory({
  initial:17,
  maximum:16384,
  shared:true
});

(async () => {
  const imports = {
    wbg: {
      memory,
      __wbindgen_throw: function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
      },
    },
  }
  const wasm = await WebAssembly.instantiateStreaming(fetch(wasmExecutorPath), imports);

  let i = wasm.instance.exports.load();
  let loop = () => {
    let n = wasm.instance.exports.load();
    if (i !== n) {
      console.log(n);
      i = n;
    }
    requestAnimationFrame(loop);
  }
  loop();

  worker.postMessage([wasm.module, memory]); // Start the worker.
  setInterval(() => {
    worker.postMessage("ayy");
  }, 100);
})();

const blob = new Blob([workerSrc]);

// Obtain a blob URL reference to our worker 'file'.
const blobURL = window.URL.createObjectURL(blob);
const worker = new Worker(blobURL);