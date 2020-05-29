self.onmessage = function onmessage(event) {
  const [module, memory] = event.data;
  const imports = {
    wbg: {
      memory,
      __wbindgen_throw: function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
      }
    },
  }
  const promise = WebAssembly.instantiate(module, imports);
  self.onmessage = async function onmessage(e) {
    const instance = await promise;
    // self.postMessage(e.data);
    instance.exports.incr();
  }
}