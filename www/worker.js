import init from 'exports-loader?wasm_bindgen!../pkg/wasm_executor';

self.onmessage = function onmessage(event) {
  const promise = init(...event.data);
  self.onmessage = async function onmessage(e) {
    const instance = await promise;
    instance.incr();
  }
}