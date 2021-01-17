use std::time::Duration;

use bevy::prelude::*;

use crate::{
    core::{Coordinates, Grid},
    rogue::*,
};

pub struct Player;

pub struct StepTimer {
    pub from: Vec3,
    pub to: Vec3,
    pub timer: Timer,
}

impl StepTimer {
    pub fn value(&self) -> Vec3 {
        self.from.lerp(self.to, self.timer.percent())
    }
}

impl Default for StepTimer {
    fn default() -> Self {
        StepTimer {
            from: Vec3::zero(),
            to: Vec3::zero(),
            timer: Timer::new(Duration::from_secs_f32(0.15), false),
        }
    }
}

pub fn step(mut query: Query<(&mut Transform, &StepTimer), With<Player>>) {
    for (mut transform, step_timer) in query.iter_mut() {
        if step_timer.timer.finished() {
            continue;
        }
        
        transform.translation = step_timer.value();
    }
}

pub fn input(
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    room: Res<Room>,
    time: Res<Time>,
    mut query: Query<(&mut Coordinates, &mut StepTimer), With<Player>>,
) {
    for (mut coords, mut step_timer) in query.iter_mut() {
        if !step_timer.timer.finished() {
            step_timer.timer.tick(time.delta_seconds());
            continue;
        }

        let direction = get_input_direction(&input);

        if direction != Coordinates::zero() {
            if let Some(tile) = room.tiles.get(&(*coords + direction)) {
                match tile {
                    Tile::Wall => {
                        // nothing
                    }
                    Tile::Floor => {
                        step_timer.from = grid.map_to_world(*coords);
                        *coords += direction;
                        step_timer.to = grid.map_to_world(*coords);
                        step_timer.timer.reset();
                    }
                }
            }
        }
    }
}

fn get_input_direction(input: &Input<KeyCode>) -> Coordinates {
    let mut direction = Coordinates::zero();

    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        direction.y += 1;
    } else if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        direction.y -= 1;
    } else if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        direction.x += 1;
    } else if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        direction.x -= 1;
    }

    direction
}
