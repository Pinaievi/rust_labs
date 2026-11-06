#[derive(Clone, Copy)]
pub enum Brand {
    Burton,
    Salomon,
    Nitro,
}

impl Brand {
    pub fn name(&self) -> &str {
        match self {
            Brand::Burton => "Burton",
            Brand::Salomon => "Salomon",
            Brand::Nitro => "Nitro",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Right,
    Left,
}

impl Direction {
    pub fn name(&self) -> &str {
        match self {
            Direction::Right => "праворуч",
            Direction::Left => "ліворуч",
        }
    }
}

pub struct SkiMuneris {
    pub id: u32,
    pub brand: Brand,
    pub shoe_size: u8,
    pub direction: Direction,
}

impl SkiMuneris {
    pub fn new(id: u32, brand: Brand, shoe_size: u8, direction: Direction) -> Self {
        SkiMuneris {
            id,
            brand,
            shoe_size,
            direction,
        }
    }
}