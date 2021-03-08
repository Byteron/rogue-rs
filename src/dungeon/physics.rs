use bevy::{prelude::*, utils::HashSet};

use super::bob::Position;

pub struct MoveEvent {
    pub entity: Entity,
    pub direction: IVec2,
}

#[derive(Default)]
pub struct PhysicsState {
    set: HashSet<IVec2>,
}

impl PhysicsState {
    pub fn is_blocked(&self, coords: IVec2) -> bool {
        self.set.contains(&coords)
    }

    fn block(&mut self, coords: IVec2) {
        self.set.insert(coords);
    }

    fn unblock(&mut self, coords: IVec2) {
        self.set.remove(&coords);
    }

    fn clear(&mut self) {
        self.set.clear();
    }
}

pub struct Solid;

#[derive(Default)]
pub struct Velocity(IVec2);

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
    mut positions: Query<&mut Velocity, With<Solid>>,
) {
    for event in event_reader.iter() {
        let mut velocity = positions.get_mut(event.entity).unwrap();
        velocity.0 = event.direction;
    }
}

pub fn update(
    mut state: ResMut<PhysicsState>,
    mut positions: Query<&mut Position, With<Solid>>,
    mut velocities: Query<(Entity, &mut Velocity), With<Solid>>,
) {
    state.clear();

    for position in positions.iter_mut() {
        state.block(position.0);
    }

    for (entity, mut velocity) in velocities.iter_mut() {
        let mut position = positions.get_mut(entity).unwrap();
        let target_position = position.0 + velocity.0;

        if !state.is_blocked(target_position) {
            state.unblock(position.0);
            state.block(target_position);
            position.0 = target_position;
        }

        velocity.0 = IVec2::ZERO;
    }
}
