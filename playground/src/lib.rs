use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let concurrency = 5;

    let pool = wasm_executor::WorkerPool::new(concurrency).expect("pool creation failed");
    pool.run(|| {
        log::info!("threadzz!");
    })
    .expect("run failed");

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
        .build()
        .unwrap();
    pool.run(move || {
        thread_pool.install(|| {
            log::info!("what's good?");
        });
    })
    .expect("run failed");
}
