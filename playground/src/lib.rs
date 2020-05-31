use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let pool = wasm_executor::WorkerPool::new(5).expect("pool creation failed");
    pool.run(|| {
        log::info!("threadz!");
        wasm_executor::incr();
    })
    .expect("run failed");
}
