#[derive(Copy, Clone)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

pub struct Input {
    pub position: Coords,
    pub dimensions: Dimensions,
    pub offset: Option<usize>,
    pub id: Option<u32>,
}
