use crate::core::math::{Rect2i, Vec2i};
use bevy::{prelude::*, utils::HashSet};
use std::time::Instant;

use super::{bob::Coords, Viewshed};

pub struct PhysicsState {
    collider: HashSet<Vec2i>,
}

impl PhysicsState {
    pub fn is_blocked(&self, coords: Vec2i) -> bool {
        self.collider.contains(&coords)
    }

    fn block(&mut self, coords: Vec2i) {
        self.collider.insert(coords);
    }

    fn unblock(&mut self, coords: Vec2i) {
        self.collider.remove(&coords);
    }

    fn clear(&mut self) {
        self.collider.clear();
    }
}

impl Default for PhysicsState {
    fn default() -> Self {
        PhysicsState {
            collider: HashSet::default(),
        }
    }
}

pub struct Solid;

pub struct Step {
    pub direction: Vec2i,
}

impl Default for Step {
    fn default() -> Self {
        Step {
            direction: Vec2i::zero(),
        }
    }
}

#[derive(Bundle)]
pub struct KinematicBodyBundle {
    pub step: Step,
    pub body: Solid,
}

impl Default for KinematicBodyBundle {
    fn default() -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            body: Solid,
        }
    }
}

pub fn update(
    mut state: ResMut<PhysicsState>,
    mut coordinates: Query<&mut Coords, With<Solid>>,
    mut movers: Query<(Entity, &mut Step), With<Solid>>,
) {
    let start = Instant::now();

    state.collider.clear();

    for coords in coordinates.iter_mut() {
        state.collider.insert(coords.0);
    }

    for (entity, mut step) in movers.iter_mut() {
        if step.direction == Vec2i::zero() {
            continue;
        }

        let mut coords = coordinates.get_mut(entity).unwrap();
        let target_coords = Coords(coords.0 + step.direction);

        if !state.collider.contains(&target_coords.0) {
            state.collider.remove(&coords.0);
            state.collider.insert(target_coords.0);

            coords.0 = target_coords.0;
        }

        step.direction = Vec2i::zero();
    }

    println!("PysicsFrame: {:?}", start.elapsed());
}
