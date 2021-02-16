use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::{grid::Grid, view::ViewAnchor};

pub struct Coords(pub Vec2i);

pub struct Layer(pub i32);

pub struct Position(pub Vec2i, pub i32);

#[derive(Bundle)]
pub struct BoardObjectBundle {
    pub coords: Coords,
    pub layer: Layer,
    pub position: Position,
    pub view_anchor: ViewAnchor,
}

impl Default for BoardObjectBundle {
    fn default() -> Self {
        BoardObjectBundle {
            coords: Coords(Vec2i::zero()),
            layer: Layer(0),
            position: Position(Vec2i::zero(), 0),
            view_anchor: ViewAnchor(None),
        }
    }
}

pub fn update_position(
    grid: Res<Grid>,
    mut query: Query<(&mut Position, &Coords, &Layer), Changed<Coords>>,
) {
    for (mut position, coords, layer) in query.iter_mut() {
        position.0 = grid.map_to_world(coords.0);
        position.1 = layer.0;
    }
}
