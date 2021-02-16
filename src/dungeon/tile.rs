use bevy::prelude::*;

pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
}

impl Default for TileBundle {
    fn default() -> Self {
        TileBundle { tile: Tile }
    }
}
