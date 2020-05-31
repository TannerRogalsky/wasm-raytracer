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
    
    let el = winit::event_loop::EventLoop::new();
    let (send, recv) = std::sync::mpsc::channel();

    thread_pool.spawn(move || {
        let mut i = 2u64;
        for v in 0..10000 {
            i += v;
        }
        send.send(i).expect("failed to send");
        log::info!("what's good?");
    });

    el.run(move |_e, _, cx| {
        use winit::event_loop::ControlFlow;
        *cx = ControlFlow::Poll;

        if let Ok(i) = recv.try_recv() {
            log::info!("{}", i);
            *cx = ControlFlow::Exit;
        }
    });
}
