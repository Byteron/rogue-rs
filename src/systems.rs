use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn tick(time: Res<Time>, mut query: Query<&mut MoveTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

pub fn control(
    input: Res<Input<KeyCode>>,
    tiles: Res<Tiles>,
    floor: Res<Floor>,
    solids: Query<&Solid>,
    mut query: Query<(&mut Coords, &mut MoveTimer), With<Controllable>>,
) {
    for (mut coords, mut timer) in query.iter_mut() {
        if !timer.0.finished() {
            continue;
        }

        let direction = get_direction(&input);
        let target_coords = coords.0 + direction;

        let tile = tiles.0.get(&(floor.current, target_coords)).unwrap();

        if solids.get(*tile).is_ok() || direction == IVec2::ZERO {
            continue;
        }

        timer.1 = coords.0;
        timer.2 = target_coords;
        coords.0 = target_coords;

        timer.0.reset();
    }
}

pub fn lerp(settings: Res<Settings>, mut query: Query<(&mut Transform, &MoveTimer)>) {
    for (mut transform, timer) in query.iter_mut() {
        let between_pos = timer.1.as_vec2().lerp(timer.2.as_vec2(), timer.0.percent());
        transform.translation = (between_pos * settings.tile_size.as_vec2()).extend(0.);
    }
}

pub fn get_direction(input: &Input<KeyCode>) -> IVec2 {
    let mut direction = IVec2::default();

    direction.y += if input.pressed(KeyCode::W) { 1 } else { 0 };
    direction.y += if input.pressed(KeyCode::S) { -1 } else { 0 };
    direction.x += if input.pressed(KeyCode::A) { -1 } else { 0 };
    direction.x += if input.pressed(KeyCode::D) { 1 } else { 0 };

    if direction.x != 0 && direction.y != 0 {
        direction = IVec2::ZERO;
    }

    direction
}
