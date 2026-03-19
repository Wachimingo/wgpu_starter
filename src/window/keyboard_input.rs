use winit::{
    event::ElementState,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::logic::renderer::Renderer;

pub fn handle_keyboard(state: &mut Renderer, input: PhysicalKey, key_state: ElementState) {
    match input {
        PhysicalKey::Code(KeyCode::KeyA) => {
            if key_state == ElementState::Pressed {
            } else {
            }
        }
        PhysicalKey::Code(KeyCode::KeyD) => {
            if key_state == ElementState::Pressed {
            } else {
            }
        }
        _ => {}
    }
}
