use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::bob::Coords;

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
    mut coordinates: Query<&mut Coords, With<Solid>>,
    mut movers: Query<(Entity, &mut Step), With<Solid>>,
    bodies: Query<Entity, With<Solid>>,
) {
    for (entity, mut step) in movers.iter_mut() {
        if step.direction == Vec2i::zero() {
            continue;
        }

        let target_coords: Coords;

        let coords = coordinates.get_mut(entity).unwrap();
        target_coords = Coords(coords.0 + step.direction);
        drop(coords);

        let mut colliding = false;

        for entity in bodies.iter() {
            let other_coords = coordinates.get_mut(entity).unwrap();

            if target_coords.0 == other_coords.0 {
                colliding = true;
                step.direction = Vec2i::zero();
                break;
            }
        }

        let mut coords = coordinates.get_mut(entity).unwrap();

        if !colliding {
            coords.0 += step.direction;
            step.direction = Vec2i::zero();
        }
    }
}
