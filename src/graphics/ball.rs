use crate::{graphics::{blocks::{Block, Level}, common_graphic_structs::{Coords, Dimensions, Input}}, logic::vertex::Vertex};

#[derive(Copy, Clone, Debug)]
pub struct Ball {
    pub id: Option<u32>,
    pub position: Coords,
    pub dimensions: Dimensions,
    pub offset: Option<usize>,
    pub vertices: [[f32; 2]; 6],
    pub y_movement_speed: f32,
}

impl Ball {
    pub fn new(input: Input) -> Ball {
        Ball {
            id: input.id,
            position: input.position,
            dimensions: input.dimensions,
            offset: input.offset,
            #[rustfmt::skip]
            vertices: [
                [input.position.x - input.dimensions.width, input.position.y - input.dimensions.height],
                [input.position.x + input.dimensions.width, input.position.y - input.dimensions.height],
                [input.position.x + input.dimensions.width, input.position.y + input.dimensions.height],
                [input.position.x - input.dimensions.width, input.position.y - input.dimensions.height],
                [input.position.x + input.dimensions.width, input.position.y + input.dimensions.height],
                [input.position.x - input.dimensions.width, input.position.y + input.dimensions.height]
            ],
            y_movement_speed: 0.01,
        }
    }
    fn is_colliding_with(&self, other: &Block) -> bool {
       self.position.x < other.position.x + other.dimensions.width &&
       self.position.x + self.dimensions.width > other.position.x &&
       self.position.y < other.position.y + other.dimensions.height &&
       self.position.y + self.dimensions.height > other.position.y
    }
    pub fn update(&mut self, buffer: &wgpu::Buffer, level: &mut Level, queue: &wgpu::Queue){
        for block in &mut level.blocks {
            if self.is_colliding_with(block) {
                self.y_movement_speed = -self.y_movement_speed;
                block.is_active = false;
                break;
            }
        }
        if self.position.y + self.dimensions.height >= 1.0 || self.position.y - self.dimensions.height <= -1.0 {
            self.y_movement_speed = -self.y_movement_speed
        }
        self.position.x += 0.00;
        self.position.y += self.y_movement_speed;
        let updated_ball = vec![
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y - self.dimensions.height]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y - self.dimensions.height]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y + self.dimensions.height]},
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y - self.dimensions.height]},
            Vertex { position: [self.position.x + self.dimensions.width, self.position.y + self.dimensions.height]},
            Vertex { position: [self.position.x - self.dimensions.width, self.position.y + self.dimensions.height]}
        ];
        queue.write_buffer(buffer, self.offset.unwrap() as u64 * std::mem::size_of::<Vertex>() as wgpu::BufferAddress, bytemuck::cast_slice(&updated_ball));
    }
}
