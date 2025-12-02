#[derive(Copy, Clone, Debug)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug)]
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

pub trait HasBounds {
    fn position(&self) -> &Coords;
    fn dimensions(&self) -> &Dimensions;
}
