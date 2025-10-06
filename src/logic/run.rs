use winit::event_loop::EventLoop;

use crate::window::window_state::WindowState;

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut window_state = WindowState::new();
    let _ = event_loop.run_app(&mut window_state);
}
