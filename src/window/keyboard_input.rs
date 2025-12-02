use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{logic::renderer::Renderer};

pub fn handle_keyboard(state:&mut Renderer, input: PhysicalKey) {
    match input {
        PhysicalKey::Code(KeyCode::KeyA) => {
            state.paddle.move_paddle(-1);
        }
        PhysicalKey::Code(KeyCode::KeyD) => {
            state.paddle.move_paddle(1);
        }
        _ => {}
    }
}
