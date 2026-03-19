use crate::{
    graphics::{
        blocks::Level,
        common_graphic_structs::{Coords, Dimensions, HasBounds, Input},
        paddle::Paddle,
    },
    logic::vertex::Vertex,
};

#[derive(Clone, Debug)]
pub struct Ball {
    pub position: Coords,
    pub dimensions: Dimensions,
    pub offset: Option<usize>,
    pub vertices: Vec<Vertex>,
    pub y_movement_speed: f32,
    pub x_movement_speed: f32,
}

impl Ball {
    pub fn new(input: Input) -> Ball {
        Ball {
            position: input.position,
            dimensions: input.dimensions,
            offset: input.offset,
            #[rustfmt::skip]
            vertices: vec![
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y - input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y - input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y + input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y - input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y + input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y + input.dimensions.height], color: [1.0,1.0,1.0,1.0]}
            ],
            y_movement_speed: 0.01,
            x_movement_speed: 0.01,
        }
    }
    fn is_colliding_with<T: HasBounds>(&self, other: &T) -> bool {
        let ax_min = self.position.x - self.dimensions.width;
        let ax_max = self.position.x + self.dimensions.width;
        let ay_min = self.position.y - self.dimensions.height;
        let ay_max = self.position.y + self.dimensions.height;

        let bx_min = other.position().x - other.dimensions().width;
        let bx_max = other.position().x + other.dimensions().width;
        let by_min = other.position().y - other.dimensions().height;
        let by_max = other.position().y + other.dimensions().height;

        ax_min < bx_max && ax_max > bx_min && ay_min < by_max && ay_max > by_min
    }
    pub fn update(
        &mut self,
        buffer: &wgpu::Buffer,
        level: &mut Level,
        queue: &wgpu::Queue,
        paddle: &Paddle,
    ) {
        for block in &mut level.blocks {
            if !block.is_active {
                continue;
            };
            if self.is_colliding_with(block) {
                self.y_movement_speed = -self.y_movement_speed;
                block.is_active = false;
                block.vertices = vec![
                    Vertex {
                        position: [
                            block.position.x - block.dimensions.width,
                            block.position.y - block.dimensions.height,
                        ],
                        color: [0.0, 0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [
                            block.position.x + block.dimensions.width,
                            block.position.y - block.dimensions.height,
                        ],
                        color: [0.0, 0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [
                            block.position.x + block.dimensions.width,
                            block.position.y + block.dimensions.height,
                        ],
                        color: [0.0, 0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [
                            block.position.x - block.dimensions.width,
                            block.position.y - block.dimensions.height,
                        ],
                        color: [0.0, 0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [
                            block.position.x + block.dimensions.width,
                            block.position.y + block.dimensions.height,
                        ],
                        color: [0.0, 0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [
                            block.position.x - block.dimensions.width,
                            block.position.y + block.dimensions.height,
                        ],
                        color: [0.0, 0.0, 0.0, 0.0],
                    },
                ];
                queue.write_buffer(
                    buffer,
                    block.offset as u64 * std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    bytemuck::cast_slice(&block.vertices),
                );
                break;
            }
        }
        if self.is_colliding_with(paddle) {
            self.y_movement_speed = -self.y_movement_speed;
        }
        // Bounce on top screen
        if self.position.y + self.dimensions.height >= 1.0 {
            self.y_movement_speed = -self.y_movement_speed
        }
        // Loose on bottom
        if self.position.y - self.dimensions.height <= -1.0 {
            println!("You lost");
            self.y_movement_speed = 0.0;
            self.x_movement_speed = 0.0
        }
        // Bounce on sides
        if self.position.x + self.dimensions.width >= 1.0
            || self.position.x - self.dimensions.width <= -1.0
        {
            self.x_movement_speed = -self.x_movement_speed;
        }
        self.position.x += self.x_movement_speed;
        self.position.y += self.y_movement_speed;
        let updated_ball = [
            Vertex {
                position: [
                    self.position.x - self.dimensions.width,
                    self.position.y - self.dimensions.height,
                ],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                position: [
                    self.position.x + self.dimensions.width,
                    self.position.y - self.dimensions.height,
                ],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                position: [
                    self.position.x + self.dimensions.width,
                    self.position.y + self.dimensions.height,
                ],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                position: [
                    self.position.x - self.dimensions.width,
                    self.position.y - self.dimensions.height,
                ],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                position: [
                    self.position.x + self.dimensions.width,
                    self.position.y + self.dimensions.height,
                ],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                position: [
                    self.position.x - self.dimensions.width,
                    self.position.y + self.dimensions.height,
                ],
                color: [1.0, 1.0, 1.0, 1.0],
            },
        ];
        queue.write_buffer(
            buffer,
            self.offset.unwrap() as u64 * std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            bytemuck::cast_slice(&updated_ball),
        );
    }
}
