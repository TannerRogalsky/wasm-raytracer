#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod websys;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{NativeWindow as Window, *};
#[cfg(target_arch = "wasm32")]
pub use websys::{WebsysWindow as Window, *};

#[cfg(not(target_arch = "wasm32"))]
use glutin as winit;

pub trait IsWindow: std::ops::Deref<Target = winit::window::Window> {
    fn swap_buffers(&self) -> Result<(), String>;
}
