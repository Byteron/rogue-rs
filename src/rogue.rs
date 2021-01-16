use crate::{
    core,
    core::{Coordinates, Tween},
    player,
    player::*,
    tile_map::*,
};

use bevy::prelude::*;

use rand::Rng;

pub struct Rogue;

impl Plugin for Rogue {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(TileMapPlugin)
            .add_startup_system(setup.system())
            .add_system(core::tween_ticks.system())
            .add_system(player::input.system())
            .add_system(player::step.system());
    }
}

fn setup(
    commands: &mut Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map: ResMut<TileMap>
) {
    generate_room(&mut map, &assets, &mut materials);

    commands
        .spawn(SpriteBundle {
            material: materials.add(assets.load("images/player.png").into()),
            transform: Transform::from_translation(Vec3::zero()),
            sprite: Sprite {
                size: map.cell_size,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(Player)
        .with(Coordinates::zero())
        .with(TileMapCursor)
        .with_bundle(Camera2dBundle::default())
        .with(Tween::default());
}

fn generate_room(map: &mut TileMap, assets: &AssetServer, materials: &mut Assets<ColorMaterial>) {
    let floor_index = map
        .tile_set
        .create_tile(materials.add(assets.load("images/floor.png").into()));
    let wall_index = map
        .tile_set
        .create_tile(materials.add(assets.load("images/wall.png").into()));

    let extents = 100;
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
