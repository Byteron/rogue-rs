mod character;
mod controllable;
mod enemy;
mod room;
mod shared;

use bevy::prelude::*;

use character::{CharacterImages, CharacterPlugin, CharacterType};
use controllable::{Controllable, ControllablePlugin};
use enemy::{EnemyPlugin, EnemyType};
use room::RoomPlugin;
use shared::{
    events::EventsPlugin,
    grid::{Grid, GridPlugin},
    math::Vec3i,
    moveable::{MoveableBundle, MoveablePlugin},
    tween::TweenPlugin,
    view::{Sync, View, ViewAnchor, ViewPlugin},
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EventsPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(TweenPlugin)
        .add_plugin(RoomPlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(MoveablePlugin)
        .add_plugin(ControllablePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ViewPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands, grid: Res<Grid>, character_images: Res<CharacterImages>) {
    commands.spawn(OrthographicCameraBundle::new_2d());

    spawn_player(commands, &character_images, &grid);

    for i in 0..5000 {
        spawn_enemy(commands, &character_images, &grid);
    }
}

fn spawn_player(commands: &mut Commands, character_images: &CharacterImages, grid: &Grid) {
    let coords = Vec3i::new(0, 0, 0);
    let translation = grid.map_to_world(coords).as_f32();

    let player_view = commands
        .spawn(SpriteBundle {
            material: character_images
                .0
                .get(&CharacterType::Human)
                .unwrap()
                .clone(),
            transform: Transform::from_translation(translation),
            sprite: Sprite {
                size: grid.cell_size.as_f32(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(View)
        .with(Sync)
        .current_entity()
        .unwrap();

    commands
        .spawn(MoveableBundle::new(coords.x, coords.y))
        .with(Controllable)
        .with(CharacterType::Human)
        .with(Transform::from_translation(translation))
        .with(ViewAnchor(Some(player_view)));
}

fn spawn_enemy(commands: &mut Commands, character_images: &CharacterImages, grid: &Grid) {
    let coords = Vec3i::new(0, 0, 0);
    let translation = grid.map_to_world(coords).as_f32();

    let enemy_view = commands
        .spawn(SpriteBundle {
            material: character_images
                .0
                .get(&CharacterType::Goblin)
                .unwrap()
                .clone(),
            transform: Transform::from_translation(translation),
            sprite: Sprite {
                size: grid.cell_size.as_f32(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(View)
        .with(Sync)
        .current_entity()
        .unwrap();

    commands
        .spawn(MoveableBundle::new(coords.x, coords.y))
        .with(EnemyType::Goblin)
        .with(CharacterType::Goblin)
        .with(Transform::from_translation(translation))
        .with(ViewAnchor(Some(enemy_view)));
}
