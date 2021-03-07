use bevy::math::IVec2;

pub struct Grid {
    pub cell_size: IVec2,
}

impl Grid {
    pub fn map_to_world(&self, coords: IVec2) -> IVec2 {
        coords * self.cell_size
    }
}

impl Grid {
    pub fn new(x: i32, y: i32) -> Self {
        Grid {
            cell_size: IVec2::new(x, y),
        }
    }
}
