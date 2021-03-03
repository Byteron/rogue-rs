use crate::core::math::Vec2i;
use bevy::{prelude::*, utils::HashSet};

use super::bob::Coords;

pub struct MoveEvent {
    pub entity: Entity,
    pub position: Vec2i,
}

pub struct PhysicsState {
    set: HashSet<Vec2i>,
}

impl PhysicsState {
    pub fn is_blocked(&self, coords: Vec2i) -> bool {
        self.set.contains(&coords)
    }

    fn block(&mut self, coords: Vec2i) {
        self.set.insert(coords);
    }

    fn unblock(&mut self, coords: Vec2i) {
        self.set.remove(&coords);
    }

    fn clear(&mut self) {
        self.set.clear();
    }
}

impl Default for PhysicsState {
    fn default() -> Self {
        PhysicsState {
            set: HashSet::default(),
        }
    }
}


pub struct Solid;

pub struct Velocity(Vec2i);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vec2i::zero())
    }
}

#[derive(Bundle)]
pub struct KinematicBodyBundle {
    pub velocity: Velocity,
    pub body: Solid,
}

impl Default for KinematicBodyBundle {
    fn default() -> Self {
        KinematicBodyBundle {
            velocity: Velocity::default(),
            body: Solid,
        }
    }
}

pub fn move_event(
    mut event_reader: EventReader<MoveEvent>,
    mut coordinates: Query<&mut Coords, With<Solid>>,
) {
    for event in event_reader.iter() {
        let mut coords = coordinates.get_mut(event.entity).unwrap();
        coords.0 = event.position;
    }
}

pub fn update_state(
    mut state: ResMut<PhysicsState>,
    mut coordinates: Query<&mut Coords, With<Solid>>,
) {
    state.clear();

    for coords in coordinates.iter_mut() {
        state.block(coords.0);
    }
}
