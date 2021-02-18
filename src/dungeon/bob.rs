use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::{grid::Grid, view::ViewAnchor};

pub struct Coords(pub Vec2i);

impl Coords {
    pub fn get_neighbors(&self) -> Vec<(Vec2i, Coords)> {
        let mut neighbors: Vec<(Vec2i, Coords)> = Vec::default();

        neighbors.push((Vec2i::up(), Coords(self.0 + Vec2i::up())));
        neighbors.push((Vec2i::down(), Coords(self.0 + Vec2i::down())));
        neighbors.push((Vec2i::left(), Coords(self.0 + Vec2i::left())));
        neighbors.push((Vec2i::right(), Coords(self.0 + Vec2i::right())));

        neighbors
    }
}

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

pub fn late_update(
    grid: Res<Grid>,
    mut query: Query<(&mut Position, &Coords, &Layer), Changed<Coords>>,
) {
    for (mut position, coords, layer) in query.iter_mut() {
        position.0 = grid.map_to_world(coords.0);
        position.1 = layer.0;
    }
}
