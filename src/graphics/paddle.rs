use crate::graphics::common_graphic_structs::{Coords, Dimensions, HasBounds, Input};
use crate::logic::vertex::Vertex;

pub struct Paddle {
    pub position: Coords,
    pub dimensions: Dimensions,
    pub vertices: Vec<Vertex>,
    pub movement_speed: f32,
    pub current_movement_speed: f32,
    pub should_move: bool,
}

pub struct PaddleInput {
    pub base_input: Input,
    pub movement_speed: f32,
}

impl Paddle {
    pub fn new(PaddleInput { base_input: Input { position, dimensions, ..}, movement_speed}: PaddleInput) -> Paddle {
        Paddle {
            position,
            dimensions,
            movement_speed,
            current_movement_speed: movement_speed,
            should_move: false,
            #[rustfmt::skip]
            vertices: vec![
                Vertex { position: [position.x - dimensions.width, position.y - dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [position.x + dimensions.width, position.y - dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [position.x + dimensions.width, position.y + dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [position.x - dimensions.width, position.y - dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [position.x + dimensions.width, position.y + dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [position.x - dimensions.width, position.y + dimensions.height], color: [1.0,1.0,1.0,1.0]}
            ],
        }
    }
    pub fn move_paddle(&mut self, input: f32) {
        if !self.should_move { return }
        self.current_movement_speed = input;
    }
    pub fn update(&mut self, buffer: &wgpu::Buffer, queue: &wgpu::Queue) {
        if !self.should_move { return }

        if self.current_movement_speed < 0.0 && self.position.x - self.dimensions.width - self.movement_speed < -1.0 { return }
        if self.current_movement_speed > 0.0 && self.position.x + self.dimensions.width + self.movement_speed > 1.0 { return }

        self.position.x += self.current_movement_speed;
        self.vertices = vec![
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y - self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y - self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y + self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y - self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y + self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y + self.dimensions.height], color: [1.0,1.0,1.0,1.0]}
        ];
        queue.write_buffer(buffer, 0, bytemuck::cast_slice(&self.vertices));
    }
}

impl HasBounds for Paddle {
    fn position(&self) -> &Coords {
        &self.position
    }
    fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
}
