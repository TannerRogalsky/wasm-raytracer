import init from 'exports-loader?wasm_bindgen!playground';

self.onmessage = function onmessage(event) {
  const promise = init(...event.data);
  self.onmessage = async function onmessage(event) {
    const instance = await promise;
    instance.child_entry_point(event.data);
  }
}