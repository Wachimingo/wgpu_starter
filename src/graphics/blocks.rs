use crate::{
    graphics::common_graphic_structs::{Coords, Dimensions, Input},
    logic::vertex::Vertex,
};

pub struct Block {
    pub id: u32,
    pub offset: usize,
    pub vertices: Vec<Vertex>,
}

impl Block {
    pub fn new(input: Input) -> Block {
        Block {
            id: input.id.unwrap(),
            offset: input.offset.unwrap(),
            #[rustfmt::skip]
            vertices: vec![
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y - input.dimensions.height]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y - input.dimensions.height]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y + input.dimensions.height]},
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y - input.dimensions.height]},
                Vertex { position: [input.position.x + input.dimensions.width, input.position.y + input.dimensions.height]},
                Vertex { position: [input.position.x - input.dimensions.width, input.position.y + input.dimensions.height]}
            ],
        }
    }
}

pub struct Level {
    pub position: Coords,
    pub dimensions: Dimensions,
    pub blocks: Vec<Block>,
}

pub struct Grid {
    pub x: f32,
    pub y: f32,
}

pub struct LevelInput<'a> {
    pub position: Coords,
    pub dimensions: Dimensions,
    pub vertex: &'a [Vertex],
    pub number_of_blocks: u32,
}

impl Level {
    pub fn generate_level(input: LevelInput) -> Level {
        let mut blocks = Vec::new();
        let mut i: f32 = 0.0;
        let mut j: f32 = 0.0;
        let mut amount: u32 = 0;
        let scale = 0.4;
        // let size: f32 = input.dimensions.width / input.number_of_blocks as f32 * 1.0;
        let size: f32 = 0.1;
        while amount < input.number_of_blocks {
            let x = input.position.x + (i * size);
            let y = input.position.y - (j * size);
            blocks.push(self::Block::new(Input {
                id: Some(i as u32),
                offset: Some(input.vertex.len()),
                position: Coords { x, y },
                dimensions: Dimensions {
                    width: size * scale,
                    height: size * scale,
                },
            }));
            i += 1.0;
            if x > 0.9 {
                j += 1.0;
                i = 0.0;
            }
            amount += 1;
        }
        Level {
            position: input.position,
            dimensions: input.dimensions,
            blocks,
        }
    }
    pub fn update_vertex_buffer(&self, vertex: &mut Vec<Vertex>) {
        for block in &self.blocks {
            vertex.extend(block.vertices.clone());
        }
    }
}
