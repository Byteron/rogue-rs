use bevy::{app::startup_stage, prelude::*};

use crate::{
    combat::{Health, Strength},
    dungeon::RoomExitedEvent,
    enemies::Enemies,
    grid::{Grid, Vec2i},
    rogue::GameState,
    rooms::{Rooms, TileType, Tiles},
    tween::{Tween, TweenMode},
};

pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(startup_stage::POST_STARTUP, setup.system())
            .add_system(movement.system());
    }
}

fn setup(
    commands: &mut Commands,
    assets: Res<AssetServer>,
    grid: Res<Grid>,
    state: Res<GameState>,
    rooms: Res<Rooms>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let room = rooms.0.get(&state.current_room).unwrap();

    let translation = grid.map_to_world(room.center());

    commands
        .spawn(SpriteBundle {
            material: materials.add(assets.load("images/player.png").into()),
            transform: Transform::from_translation(translation),
            sprite: Sprite {
                size: grid.cell_size,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(Player)
        .with(Health::new(30))
        .with(Strength(8))
        .with(Tween::new(translation))
        .with(room.center());
}

fn movement(
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    state: Res<GameState>,
    rooms: Res<Rooms>,
    tiles: Res<Tiles>,
    enemies: Res<Enemies>,
    mut events: ResMut<Events<RoomExitedEvent>>,
    mut players: Query<(&mut Vec2i, &mut Tween), With<Player>>,
) {
    for (mut coords, mut tween) in players.iter_mut() {
        if !tween.finished() {
            continue;
        }

        let direction = get_input_direction(&input);

        if direction == Vec2i::zero() {
            continue;
        }

        let room = rooms.0.get(&state.current_room).unwrap();

        let from_coords = *coords;
        let to_coords = *coords + direction;

        if room.is_exit(to_coords) {
            events.send(RoomExitedEvent { direction });
            tween.from = grid.map_to_world(from_coords);
            tween.to = grid.map_to_world(to_coords);
            tween.start(0.15, TweenMode::Move);

            *coords = to_coords;
        } else if let Some(_) = enemies.0.get(&to_coords.extend(state.current_level)) {
        } else if let Some(tile) = tiles.0.get(&to_coords.extend(state.current_level)) {
            if *tile == TileType::Wall {
                continue;
            }

            tween.from = grid.map_to_world(from_coords);
            tween.to = grid.map_to_world(to_coords);
            tween.start(0.15, TweenMode::Move);

            *coords = to_coords;
        }
    }
}

pub fn get_input_direction(input: &Input<KeyCode>) -> Vec2i {
    let mut direction = Vec2i::zero();

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
