use bevy::prelude::*;

use crate::core::math::Vec2i;

pub struct Coords(pub Vec2i);

impl Coords {
    pub fn get_neighbors(&self) -> [(Vec2i, Coords); 4] {
        [
            (Vec2i::up(), Coords(self.0 + Vec2i::up())),
            (Vec2i::down(), Coords(self.0 + Vec2i::down())),
            (Vec2i::left(), Coords(self.0 + Vec2i::left())),
            (Vec2i::right(), Coords(self.0 + Vec2i::right())),
        ]
    }
}

pub struct Layer(pub i32);

#[derive(Bundle)]
pub struct BoardObjectBundle {
    pub coords: Coords,
    pub transform: Transform,
    pub layer: Layer,
}

impl Default for BoardObjectBundle {
    fn default() -> Self {
        BoardObjectBundle {
            coords: Coords(Vec2i::zero()),
            transform: Transform::default(),
            layer: Layer(0),
        }
    }
}
