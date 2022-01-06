use bevy::prelude::*;

#[derive(Component)]
pub struct Controllable;

#[derive(Component)]
pub struct MoveTimer(pub Timer, pub IVec2, pub IVec2);

#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coords(pub IVec2);

#[derive(Component)]
pub struct Solid;