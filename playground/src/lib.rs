use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
fn build<T>(builder: rayon::ThreadPoolBuilder<T>) -> rayon::ThreadPool {
    let pool = wasm_executor::WorkerPool::new(5).expect("pool creation failed");
    builder
        .spawn_handler(move |thread| Ok(pool.run(|| thread.run()).unwrap()))
        .build()
        .unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn build(builder: rayon::ThreadPoolBuilder) -> rayon::ThreadPool {
    builder.build().unwrap()
}

#[wasm_bindgen]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let concurrency = 5;
    let thread_pool = build(rayon::ThreadPoolBuilder::new().num_threads(concurrency));
    thread_pool.spawn(|| {
        log::info!("what's good?");
    });
    log::info!("ayy");
}
