use crate::graphics::common_graphic_structs::Input;
use crate::logic::vertex::Vertex;

pub struct Paddle {
    pub vertices: Vec<Vertex>,
}

impl Paddle {
    pub fn new(input: Input) -> Paddle {
        Paddle {
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
