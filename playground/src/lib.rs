mod app;
mod window;

#[cfg(not(target_arch = "wasm32"))]
use glutin as winit;

use graphics::texture::{Texture, TextureUpdate};
use graphics::vertex::Vertex;
use rand::prelude::*;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
fn build_thread_pool() -> rayon::ThreadPool {
    let concurrency = web_sys::window()
        .unwrap()
        .navigator()
        .hardware_concurrency() as usize;
    let pool = wasm_executor::WorkerPool::new(concurrency).expect("pool creation failed");
    rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .spawn_handler(move |thread| Ok(pool.run(|| thread.run()).unwrap()))
        .build()
        .unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn build_thread_pool() -> rayon::ThreadPool {
    let concurrency = num_cpus::get();
    rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .build()
        .unwrap()
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

    let thread_pool = build_thread_pool();

    const WIDTH: u32 = 720;
    const HEIGHT: u32 = 480;

    let el = winit::event_loop::EventLoop::new();
    let wb = winit::window::WindowBuilder::new()
        .with_title("Viewer")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720));
    let (ctx, window) = window::init_ctx(wb, &el);

    let mut ctx = graphics::Context::new(ctx);
    {
        let scale = window.scale_factor();
        let width = 1280. * scale;
        let height = 720. * scale;
        ctx.set_viewport(0, 0, width as _, height as _);
    }
    ctx.enable(graphics::Feature::CullFace(
        graphics::CullFace::Back,
        graphics::VertexWinding::CounterClockWise,
    ));

    let app = app::App::new(WIDTH as _, HEIGHT as _);
    let pixels = {
        let mut pixels = (0..WIDTH * HEIGHT)
            .map(|i| {
                let x = (i % WIDTH) as usize;
                let y = ((i / WIDTH) % HEIGHT) as usize;
                (x, y)
            })
            .collect::<Vec<_>>();
        let mut rng = SmallRng::seed_from_u64(0);
        pixels.shuffle(&mut rng);
        pixels
    };
    let mut pixel_data = vec![raytracer::Pixel::default(); (WIDTH * HEIGHT) as usize];

    let image = graphics::image::Image::with_data(
        &mut ctx,
        graphics::texture::TextureType::Tex2D,
        graphics::data::PixelFormat::RGB8,
        WIDTH,
        HEIGHT,
        unsafe {
            let pixels = &pixel_data;
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
                uv: [x, 1.0 - y],
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

    let (sender, recv) = std::sync::mpsc::channel();
    thread_pool.spawn(move || {
        pixels
            .into_par_iter()
            .for_each_with(sender, |sender, (x, y)| {
                let mut rng = SmallRng::seed_from_u64(0);
                let pixel = app.draw(x, y, &mut rng);
                sender
                    .send(((x, y), pixel))
                    .expect("failed to send, the main thread is probably dead");
            });
    });

    el.run(move |e, _, cx| {
        use winit::{event::*, event_loop::ControlFlow};
        *cx = ControlFlow::Poll;

        match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *cx = ControlFlow::Exit,
                WindowEvent::Resized(winit::dpi::PhysicalSize { width, height }) => {
                    let scale = window.scale_factor();
                    let width = width as f64 * scale;
                    let height = height as f64 * scale;
                    ctx.set_viewport(0, 0, width as _, height as _);
                }
                _ => {}
            },
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                let mut updated = false;
                for ((x, y), pixel) in recv.try_iter() {
                    updated = true;
                    let i = x + WIDTH as usize * y;
                    pixel_data[i] = pixel;
                }

                if updated {
                    ctx.set_texture_data(
                        image.get_texture_key(),
                        image.get_texture_info(),
                        image.get_texture_type(),
                        Some(unsafe {
                            let pixels = &pixel_data;
                            std::slice::from_raw_parts(
                                pixels.as_ptr() as *const u8,
                                pixels.len() * std::mem::size_of::<raytracer::Pixel>(),
                            )
                        }),
                    );
                }

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
