use crate::{
    core::{Coordinates, Grid},
    despawn::DespawnPlugin,
    dungeon,
    dungeon::*,
    player,
    player::*,
    tween::{Tween, TweenPlugin},
};

use bevy::prelude::*;

pub struct Rogue;

impl Plugin for Rogue {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(TweenPlugin)
            .add_plugin(DespawnPlugin)
            .add_event::<ExitRoomEvent>()
            .add_event::<EnterRoomEvent>()
            .add_resource(Grid::default())
            .add_startup_system(setup.system())
            .add_system(dungeon::on_exit_room.system())
            .add_system(dungeon::on_enter_room.system())
            .add_system(player::input.system());
    }
}

fn setup(
    commands: &mut Commands,
    grid: Res<Grid>,
    assets: Res<AssetServer>,
    mut events: ResMut<Events<ExitRoomEvent>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut images = Images::default();

    images.add_player(&assets, &mut materials, "images/player.png");
    images.add_tile(&assets, &mut materials, TileType::Wall, "images/wall.png");
    images.add_tile(&assets, &mut materials, TileType::Floor, "images/floor.png");
    images.add_enemy(
        &assets,
        &mut materials,
        EnemyType::Goblin,
        "images/enemy.png",
    );

    let mut state = dungeon::generate();

    events.send(ExitRoomEvent {
        direction: Coordinates::zero(),
    });

    let room = state.get_current_room();

    let center = grid.map_to_world(room.center());

    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(center),
        ..Default::default()
    });

    commands
        .spawn(SpriteBundle {
            material: images.get_player(),
            transform: Transform::from_translation(center),
            sprite: Sprite {
                size: grid.cell_size,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(Player)
        .with(Tween::new(center))
        .with(room.center());

    commands.insert_resource(images);
    commands.insert_resource(state);
}
