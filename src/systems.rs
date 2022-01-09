use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;
use rand::Rng;

pub fn tick(time: Res<Time>, mut query: Query<&mut MoveTween>) {
    for mut tween in query.iter_mut() {
        tween.timer.tick(time.delta());
    }
}

pub fn control(
    mut events: EventWriter<MovedEvent>,
    input: Res<Input<KeyCode>>,
    tiles: Res<Tiles>,
    floor: Res<Floor>,
    solid_tiles: Query<&Solid>,
    mut char_slots: Query<&mut CharacterSlot>,
    mut query: Query<(Entity, &mut Coords, &mut MoveTween), With<Controllable>>,
) {
    for (player, mut coords, mut tween) in query.iter_mut() {
        if !tween.timer.finished() {
            continue;
        }

        let direction = get_direction(&input);
        let target_coords = coords.0 + direction;

        let target_tile = tiles
            .0
            .get(&(target_coords.x, target_coords.y, floor.current))
            .unwrap();

        if solid_tiles.get(*target_tile).is_ok()
            || char_slots.get(*target_tile).unwrap().entity.is_some()
            || direction == IVec2::ZERO
        {
            continue;
        }

        let tile = tiles
            .0
            .get(&(coords.0.x, coords.0.y, floor.current))
            .unwrap();

        char_slots.get_mut(*tile).unwrap().entity = None;
        char_slots.get_mut(*target_tile).unwrap().entity = Some(player);

        tween.start = coords.0;
        tween.end = target_coords;
        tween.timer.reset();

        coords.0 = target_coords;

        events.send(MovedEvent);
    }
}

pub fn roam(
    tiles: Res<Tiles>,
    floor: Res<Floor>,
    solid_tiles: Query<&Solid>,
    mut char_slots: Query<&mut CharacterSlot>,
    mut events: EventReader<MovedEvent>,
    mut query: Query<(Entity, &mut Coords, &mut MoveTween), With<Roamer>>,
) {
    for _ in events.iter() {
        for (roamer, mut coords, mut tween) in query.iter_mut() {
            if !tween.timer.finished() {
                continue;
            }

            let mut rng = rand::thread_rng();

            let direction = IVec2::new(rng.gen_range(-1..2), rng.gen_range(-1..2));
            let target_coords = coords.0 + direction;

            let target_tile = tiles
                .0
                .get(&(target_coords.x, target_coords.y, floor.current))
                .unwrap();

            if solid_tiles.get(*target_tile).is_ok()
                || char_slots.get(*target_tile).unwrap().entity.is_some()
                || direction == IVec2::ZERO
            {
                continue;
            }

            let tile = tiles
                .0
                .get(&(coords.0.x, coords.0.y, floor.current))
                .unwrap();

            char_slots.get_mut(*tile).unwrap().entity = None;
            char_slots.get_mut(*target_tile).unwrap().entity = Some(roamer);

            tween.start = coords.0;
            tween.end = target_coords;
            tween.timer.reset();

            coords.0 = target_coords;
        }
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
