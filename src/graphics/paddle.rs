use crate::graphics::common_graphic_structs::{Coords, Dimensions, HasBounds, Input};
use crate::logic::vertex::Vertex;

pub struct Paddle {
    pub position: Coords,
    pub dimensions: Dimensions,
    pub vertices: Vec<Vertex>,
    pub movement_speed: f32,
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
    pub fn move_paddle(&mut self, input: i8) {
        self.position.x += self.movement_speed * (input as f32);
        if self.position.x + self.dimensions.width >= 1.0 || self.position.x - self.dimensions.width<= -1.0 {
            self.position.x -= self.movement_speed * (input as f32);
            return
        }
        self.vertices = vec![
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y - self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y - self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y + self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y - self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y + self.dimensions.height], color: [1.0,1.0,1.0,1.0]},
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y + self.dimensions.height], color: [1.0,1.0,1.0,1.0]}
        ];
    }
    pub fn update(&mut self, buffer: &wgpu::Buffer, queue: &wgpu::Queue) {
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
