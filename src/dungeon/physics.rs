use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::bob::Coords;

pub struct Body {
    solid: bool,
}

impl Body {
    pub fn new(solid: bool) -> Self {
        Body { solid }
    }

    pub fn solid() -> Self {
        Body { solid: true }
    }

    pub fn hollow() -> Self {
        Body { solid: true }
    }
}

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
    pub body: Body,
}

impl KinematicBodyBundle {
    pub fn new(solid: bool) -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            body: Body { solid },
        }
    }

    pub fn solid() -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            body: Body::solid(),
        }
    }

    pub fn hollow() -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            body: Body::hollow(),
        }
    }
}

pub fn movement(
    mut coordinates: Query<&mut Coords, With<Body>>,
    mut movers: Query<(Entity, &mut Step, &Body)>,
    bodies: Query<(Entity, &Body)>,
) {
    for (entity, mut step, body) in movers.iter_mut() {
        if !body.solid || step.direction == Vec2i::zero() {
            continue;
        }

        let target_coords: Coords;

        let coords = coordinates.get_mut(entity).unwrap();
        target_coords = Coords(coords.0 + step.direction);
        drop(coords);

        let mut colliding = false;

        for (entity, body) in bodies.iter() {
            let other_coords = coordinates.get_mut(entity).unwrap();

            if body.solid && target_coords.0 == other_coords.0 {
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
