use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;

mod pool;
pub use pool::WorkerPool;

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub fn incr() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

#[wasm_bindgen]
pub fn load_state() -> u32 {
    COUNTER.load(Ordering::SeqCst)
}

#[wasm_bindgen]
pub fn module() -> JsValue {
    wasm_bindgen::module()
}

#[wasm_bindgen]
pub fn memory() -> JsValue {
    wasm_bindgen::memory()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
