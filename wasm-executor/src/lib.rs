use wasm_bindgen::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct ThreadPool {
    workers: Vec<web_sys::Worker>
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            workers: vec![]
        }
    }
}

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub fn incr() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

#[wasm_bindgen]
pub fn load() -> u32 {
    COUNTER.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
