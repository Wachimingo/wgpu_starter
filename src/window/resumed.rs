use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::logic::renderer::Renderer;
use crate::window::window_state::WindowState;

pub fn resume(window: &mut WindowState, event_loop: &ActiveEventLoop) {
    let new_window = event_loop
        .create_window(Window::default_attributes())
        .unwrap();
    window.state = Some(Renderer::new(new_window));
}
