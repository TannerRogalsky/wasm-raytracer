use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let pool = wasm_executor::WorkerPool::new(5).expect("pool creation failed");
    pool.run(|| {
        log::info!("threadz!");
    })
    .expect("run failed");
}
