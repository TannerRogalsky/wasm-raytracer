use winit::platform::web::WindowExtWebSys;
use winit::window::Window;

pub struct WebsysWindow {
    inner: Window,
}

impl WebsysWindow {
    pub fn new(inner: Window) -> Self {
        Self { inner }
    }
}

impl super::IsWindow for WebsysWindow {
    fn swap_buffers(&self) -> Result<(), String> {
        Ok(())
    }
}

impl std::ops::Deref for WebsysWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn init_ctx(
    wb: winit::window::WindowBuilder,
    el: &winit::event_loop::EventLoop<()>,
) -> (graphics::glow::Context, WebsysWindow) {
    use wasm_bindgen::JsCast;
    let window = wb.build(&el).unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_with_node_1(&window.canvas())
        .unwrap();
    let webgl_context = window
        .canvas()
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()
        .unwrap();
    let gfx = graphics::glow::Context::from_webgl1_context(webgl_context);
    (gfx, WebsysWindow::new(window))
}
