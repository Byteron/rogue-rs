use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn tick(time: Res<Time>, mut query: Query<&mut MoveTween>) {
    for mut tween in query.iter_mut() {
        tween.timer.tick(time.delta());
    }
}

pub fn control(
    input: Res<Input<KeyCode>>,
    tiles: Res<Tiles>,
    floor: Res<Floor>,
    solids: Query<&Solid>,
    mut query: Query<(&mut Coords, &mut MoveTween), With<Controllable>>,
) {
    for (mut coords, mut tween) in query.iter_mut() {
        if !tween.timer.finished() {
            continue;
        }

        let direction = get_direction(&input);
        let target_coords = coords.0 + direction;

        let tile = tiles
            .0
            .get(&(target_coords.x, target_coords.y, floor.current))
            .unwrap();

        if solids.get(*tile).is_ok() || direction == IVec2::ZERO {
            continue;
        }

        tween.start = coords.0;
        tween.end = target_coords;
        tween.timer.reset();

        coords.0 = target_coords;
    }
}

pub fn lerp(settings: Res<Settings>, mut query: Query<(&mut Transform, &MoveTween)>) {
    for (mut transform, tween) in query.iter_mut() {
        let between_pos = tween
            .start
            .as_vec2()
            .lerp(tween.end.as_vec2(), tween.timer.percent());
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
