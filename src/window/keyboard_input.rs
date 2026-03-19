use winit::{
    event::ElementState,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::logic::renderer::Renderer;

pub fn handle_keyboard(state: &mut Renderer, input: PhysicalKey, key_state: ElementState) {
    match input {
        PhysicalKey::Code(KeyCode::KeyA) => {
            if key_state == ElementState::Pressed {
                state.paddle.should_move = true;
                state.paddle.move_paddle(-state.paddle.movement_speed);
            } else {
                state.paddle.should_move = false;
                state.paddle.move_paddle(0.0);
            }
        }
        PhysicalKey::Code(KeyCode::KeyD) => {
            if key_state == ElementState::Pressed {
                state.paddle.should_move = true;
                state.paddle.move_paddle(state.paddle.movement_speed);
            } else {
                state.paddle.should_move = false;
                state.paddle.move_paddle(0.0);
            }
        }
        _ => {}
    }
}
