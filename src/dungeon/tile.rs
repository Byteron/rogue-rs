use bevy::prelude::*;

use super::physics::Body;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Bundle)]
pub struct TileBundle {
    tile_type: TileType,
    body: Body,
}

impl TileBundle {
    pub fn new(tile_type: TileType, solid: bool) -> Self {
        TileBundle {
            tile_type,
            body: Body::new(solid),
        }
    }
}

impl Default for TileBundle {
    fn default() -> Self {
        TileBundle {
            tile_type: TileType::Floor,
            body: Body::hollow(),
        }
    }
}
