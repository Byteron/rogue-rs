use crate::core::math::Vec2i;

pub struct Grid {
    pub cell_size: Vec2i,
}

impl Grid {
    pub fn map_to_world(&self, coords: Vec2i) -> Vec2i {
        coords * self.cell_size
    }
}

impl Grid {
    pub fn new(x: i32, y: i32) -> Self {
        Grid {
            cell_size: Vec2i::new(x, y),
        }
    }
}
