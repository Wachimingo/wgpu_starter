use winit::{
    event_loop::{ActiveEventLoop},
};

use crate::window::window_state::WindowState;

const TIME: u64 = 1000;
const MAX_FPS: u64 = 24;
const WAIT_TIME: std::time::Duration = std::time::Duration::from_millis(TIME / MAX_FPS);

pub fn about_to_wait_request(window: &mut WindowState, event_loop: &ActiveEventLoop) {
    if let Some(state) = &window.state {
        std::thread::sleep(WAIT_TIME);
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        state.window.request_redraw();
    }
}
