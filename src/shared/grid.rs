use super::math::{Vec2i, Vec3i};
use bevy::prelude::*;

pub struct Coords(pub Vec2i);

pub struct Grid {
    pub cell_size: Vec2i,
}

impl Grid {
    pub fn map_to_world(&self, coords: Vec3i) -> Vec3i {
        coords * self.cell_size.extend(0)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            cell_size: Vec2i::new(64, 64),
        }
    }
}

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Grid::default());
    }
}
