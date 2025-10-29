use winit::keyboard::{KeyCode, PhysicalKey};
use winit::{
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::window::window_state::WindowState;

pub fn window_event(
    window: &mut WindowState,
    event_loop: &ActiveEventLoop,
    window_id: WindowId,
    event: WindowEvent,
) {
    if let Some(state) = &mut window.state {
        let window = state.window();
        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    let _ = state.render();
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                } => {}
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::KeyA),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    state.vertex[0].position[0] -= 0.01;
                    state.vertex[1].position[0] -= 0.01;
                    state.vertex[2].position[0] -= 0.01;
                    state.vertex[3].position[0] -= 0.01;
                    state.vertex[4].position[0] -= 0.01;
                    state.vertex[5].position[0] -= 0.01;
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::KeyD),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    state.vertex[0].position[0] += 0.01;
                    state.vertex[1].position[0] += 0.01;
                    state.vertex[2].position[0] += 0.01;
                    state.vertex[3].position[0] += 0.01;
                    state.vertex[4].position[0] += 0.01;
                    state.vertex[5].position[0] += 0.01;
                }
                _ => {}
            }
        }
    }
}
