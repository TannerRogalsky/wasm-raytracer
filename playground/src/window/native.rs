use glutin as winit;
use winit::window::Window;

type WindowContext = winit::ContextWrapper<winit::PossiblyCurrent, winit::window::Window>;

pub struct NativeWindow {
    inner: WindowContext,
}

impl NativeWindow {
    pub fn new(inner: WindowContext) -> Self {
        Self { inner }
    }
}

impl super::IsWindow for NativeWindow {
    fn swap_buffers(&self) -> Result<(), String> {
        self.inner
            .swap_buffers()
            .map_err(|e| format!("Context Error: {:?}", e))
    }
}

impl std::ops::Deref for NativeWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.inner.window()
    }
}

pub fn init_ctx(
    wb: winit::window::WindowBuilder,
    el: &winit::event_loop::EventLoop<()>,
) -> (graphics::glow::Context, NativeWindow) {
    let windowed_context = winit::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let gfx = graphics::glow::Context::from_loader_function(|s| {
        windowed_context.get_proc_address(s) as *const _
    });
    (gfx, NativeWindow::new(windowed_context))
}
