use bevy::{prelude::*, utils::HashMap};

pub struct Settings {
    pub floor_count: usize,
    pub room_count: usize,
    pub room_size: IVec2,
    pub tile_size: IVec2,
}

pub struct Tiles(pub HashMap<(usize, IVec2), Entity>);

pub struct Floor {
    pub current: usize,
}