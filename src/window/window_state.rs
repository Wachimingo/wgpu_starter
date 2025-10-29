use winit::event_loop::ActiveEventLoop;

use crate::logic::renderer::Renderer;
use crate::window::{
    about_to_wait::about_to_wait_request, resumed::resume, window_event::window_event,
};

pub struct WindowState<'a> {
    pub state: Option<Renderer<'a>>,
}

impl<'a> WindowState<'a> {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl<'a> winit::application::ApplicationHandler for WindowState<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        resume(self, event_loop);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        window_event(self, event_loop, window_id, event);
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        about_to_wait_request(self, event_loop);
    }
}
