mod app;
mod window;

#[cfg(not(target_arch = "wasm32"))]
use glutin as winit;

use graphics::texture::Texture;
use graphics::vertex::Vertex;
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

#[derive(Vertex, Default, Copy, Clone, Debug)]
#[repr(packed, C)]
struct Vertex2D {
    position: [f32; 2],
    uv: [f32; 2],
}

#[wasm_bindgen]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let concurrency = 5;
    let thread_pool = build(rayon::ThreadPoolBuilder::new().num_threads(concurrency));

    let (send, recv) = std::sync::mpsc::channel();
    thread_pool.spawn(move || {
        let mut i = 2u64;
        for v in 0..10000 {
            i += v;
        }
        send.send(i).expect("failed to send");
        log::info!("what's good?");
    });

    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 100;

    let el = winit::event_loop::EventLoop::new();
    let wb = winit::window::WindowBuilder::new()
        .with_title("Viewer")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720));
    let (ctx, window) = window::init_ctx(wb, &el);

    let mut ctx = graphics::Context::new(ctx);
    ctx.set_viewport(0, 0, 1280, 720);
    // ctx.enable(graphics::Feature::DepthTest(graphics::DepthFunction::Less));
    // ctx.enable(graphics::Feature::CullFace(
    //     graphics::CullFace::Back,
    //     graphics::VertexWinding::CounterClockWise,
    // ));

    let mut app = app::App::new(WIDTH as _, HEIGHT as _);
    app.draw();

    let image = graphics::image::Image::with_data(
        &mut ctx,
        graphics::texture::TextureType::Tex2D,
        graphics::data::PixelFormat::RGB8,
        WIDTH,
        HEIGHT,
        unsafe {
            let pixels = app.pixels();
            std::slice::from_raw_parts(
                pixels.as_ptr() as *const u8,
                pixels.len() * std::mem::size_of::<raytracer::Pixel>(),
            )
        },
        graphics::image::Settings {
            mipmaps: false,
            dpi_scale: 1.0,
            slices: 1,
            filter: graphics::texture::FilterMode::Nearest,
            wrap: graphics::texture::WrapMode::Clamp,
        },
    )
    .unwrap();
    let mut quadbatch = graphics::quad_batch::QuadBatch::new(&mut ctx, 1).unwrap();
    quadbatch.push(
        graphics::quad_batch::Quad::from(graphics::viewport::Viewport::new(0., 0., 1., 1.)).map(
            |(x, y)| Vertex2D {
                position: [x, y],
                uv: [x, y],
            },
        ),
    );
    let shader = {
        const SRC: &str = include_str!("./shader.glsl");
        let (vert, frag) = graphics::shader::Shader::create_source(SRC, SRC);
        graphics::shader::Shader::new(&mut ctx, &vert, &frag).unwrap()
    };

    ctx.clear_color(1., 0., 0., 1.);
    ctx.use_shader(Some(&shader));
    ctx.bind_texture_to_unit(image.get_texture_type(), image.get_texture_key(), 0.into());

    el.run(move |e, _, cx| {
        use winit::{event::*, event_loop::ControlFlow};
        *cx = ControlFlow::Poll;

        if let Ok(i) = recv.try_recv() {
            log::info!("{}", i);
            // *cx = ControlFlow::Exit;
        }

        match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *cx = ControlFlow::Exit,
                WindowEvent::Resized(winit::dpi::PhysicalSize { width, height }) => {
                    ctx.set_viewport(0, 0, width as _, height as _);
                }
                _ => {}
            },
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                ctx.clear();
                quadbatch.draw(&mut ctx);

                use window::IsWindow;
                window.swap_buffers().expect("failed to swap buffers");
            }
            Event::LoopDestroyed => *cx = ControlFlow::Exit,
            _ => {}
        }
    });
}
