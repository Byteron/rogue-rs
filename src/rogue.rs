use std::time::Duration;

use crate::tile_map::*;
use crate::{
    components::*,
    core::{Coordinates, Tween},
};
use bevy::prelude::*;

use rand::Rng;

pub struct Grid {
    pub cell_size: Vec2,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            cell_size: Vec2::new(64.0, 64.0),
        }
    }
}

pub struct Rogue;

impl Plugin for Rogue {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(TileMapPlugin)
            .add_startup_system(setup.system())
            .add_system(player_input.system())
            .add_system(player_step.system())
            .add_system(tween.system());
    }
}

fn setup(
    commands: &mut Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut map = TileMap::default();

    generate_room(&mut map, &assets, &mut materials);

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            material: materials.add(assets.load("images/player.png").into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            sprite: Sprite {
                size: map.cell_size,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(Player)
        .with(Coordinates::zero())
        .with(Tween::default());

    commands.spawn(()).with(map);

    commands.insert_resource(Grid::default());
}

fn tween(time: Res<Time>, mut query: Query<&mut Tween>) {
    for mut tween in query.iter_mut() {
        tween.tick(time.delta_seconds());
    }
}

pub fn player_step(mut query: Query<(&mut Transform, &Tween), With<Player>>) {
    for (mut transform, tween) in query.iter_mut() {
        transform.translation = tween.value();
    }
}

pub fn player_input(
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    mut query: Query<(&mut Coordinates, &mut Tween), With<Player>>,
) {
    for (mut coords, mut tween) in query.iter_mut() {
        if !tween.finished() {
            continue;
        }

        let direction = get_input_direction(&input);

        if direction != Coordinates::zero() {
            let cell_size = grid.cell_size;
            let start = *coords;

            *coords += direction;

            tween.start(
                (start.to_vec() * cell_size).extend(0.0),
                (coords.to_vec() * cell_size).extend(0.0),
                Duration::from_secs_f32(0.2),
            );
        }
    }
}

fn get_input_direction(input: &Input<KeyCode>) -> Coordinates {
    let mut direction = Coordinates::zero();

    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        direction.y += 1;
    }

    if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        direction.y -= 1;
    }

    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        direction.x += 1;
    }

    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        direction.x -= 1;
    }

    direction
}

fn generate_room(map: &mut TileMap, assets: &AssetServer, materials: &mut Assets<ColorMaterial>) {
    let floor_index = map
        .tile_set
        .create_tile(materials.add(assets.load("images/floor.png").into()));
    let wall_index = map
        .tile_set
        .create_tile(materials.add(assets.load("images/wall.png").into()));

    let extents = 5;
    let mut rng = rand::thread_rng();

    for y in -extents..=extents {
        for x in -extents..=extents {
            if x == -extents || x == extents || y == -extents || y == extents {
                map.set_cell(Coordinates::new(x, y), wall_index);
            } else {
                if rng.gen_bool(0.3) {
                    map.set_cell(Coordinates::new(x, y), wall_index);
                } else {
                    map.set_cell(Coordinates::new(x, y), floor_index);
                }
            }
        }
    }
}
