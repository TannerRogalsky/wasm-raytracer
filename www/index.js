import main from 'imports-loader?WorkerTemplate=../www/worker.js!exports-loader?wasm_bindgen!playground';
import wasmExecutorPath from "file-loader!../pkg/playground_bg.wasm";
main(wasmExecutorPath).then((wasm) => {
  wasm.main();
});