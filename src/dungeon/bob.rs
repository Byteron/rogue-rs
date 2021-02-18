use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::view::ViewAnchor;

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

#[derive(Bundle)]
pub struct BoardObjectBundle {
    pub coords: Coords,
    pub layer: Layer,
    pub view_anchor: ViewAnchor,
}

impl Default for BoardObjectBundle {
    fn default() -> Self {
        BoardObjectBundle {
            coords: Coords(Vec2i::zero()),
            layer: Layer(0),
            view_anchor: ViewAnchor(None),
        }
    }
}
