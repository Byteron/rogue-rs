use bevy::{prelude::*, utils::HashMap};

pub struct Settings {
    pub floor_count: usize,
    pub room_count: usize,
    pub room_size: IVec2,
    pub tile_size: IVec2,
}

pub struct Tiles(pub HashMap<(i32, i32, usize), Entity>);

pub struct Floor {
    pub current: usize,
}
