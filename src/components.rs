use bevy::prelude::*;

#[derive(Component)]
pub struct Controllable;

#[derive(Component)]
pub struct Roamer;

#[derive(Component)]
pub struct MoveTween {
    pub start: IVec2,
    pub end: IVec2,
    pub timer: Timer,
}

#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coords(pub IVec2);

#[derive(Component)]
pub struct Solid;


#[derive(Component)]
pub struct HasCharacter {
    pub entity: Entity,
}