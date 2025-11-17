use crate::{
    graphics::common_graphic_structs::{Coords, Dimensions, Input},
    logic::vertex::Vertex,
};

pub struct Block {
    pub id: u32,
    pub offset: usize,
    pub vertices: Vec<Vertex>,
    pub position: Coords,
    pub dimensions: Dimensions,
    pub is_active: bool,
}

impl Block {
    pub fn new(input: Input) -> Block {
        Block {
            id: input.id.unwrap(),
            offset: input.offset.unwrap(),
            position: input.position,
            dimensions: input.dimensions,
            is_active: true,
            #[rustfmt::skip]
            vertices: vec![
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y - input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y - input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y + input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y - input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y + input.dimensions.height], color: [1.0,1.0,1.0,1.0]},
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y + input.dimensions.height], color: [1.0,1.0,1.0,1.0]}
            ],
        }
    }
}

pub struct Level {
    pub blocks: Vec<Block>,
}

pub struct Grid {
    pub x: f32,
    pub y: f32,
}

pub struct LevelInput<'a> {
    pub position: Coords,
    pub vertex: &'a mut Vec<Vertex>,
    pub number_of_blocks: u32,
    pub block_size: f32,
}

impl Level {
    pub fn generate_level(input: LevelInput) -> Level {
        let mut blocks = Vec::new();
        let mut i: f32 = 0.0;
        let mut j: f32 = 0.0;
        let mut amount: u32 = 0;
        let scale = 0.4;
        while amount < input.number_of_blocks {
            let x = input.position.x + (i * input.block_size);
            let y = input.position.y - (j * input.block_size);
            let block = self::Block::new(Input {
                id: Some(i as u32),
                offset: Some(input.vertex.len()),
                position: Coords { x, y },
                dimensions: Dimensions {
                    width: input.block_size * scale,
                    height: input.block_size * scale,
                },
            });
            input.vertex.extend(block.vertices.clone());
            blocks.push(block);
            i += 1.0;
            if x > 0.9 {
                j += 1.0;
                i = 0.0;
            }
            amount += 1;
        }
        Level {
            blocks,
        }
    }
    pub fn update_vertex_buffer(&self, vertex: &mut Vec<Vertex>) {
        for block in &self.blocks {
            if block.is_active {
                vertex.extend(block.vertices.clone());
            }
        }
    }
}
