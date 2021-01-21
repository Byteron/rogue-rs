use crate::{
    core::{Coordinates, Grid},
    dungeon,
    dungeon::*,
    images::{Images, ImagesPlugin},
    player,
    player::*,
    tween::{Tween, TweenPlugin},
};

use bevy::prelude::*;

pub struct Rogue;

impl Plugin for Rogue {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(ImagesPlugin)
            .add_plugin(TweenPlugin)
            .add_plugin(DungeonPlugin)
            .add_resource(Grid::default())
            .add_startup_system(setup.system())
            .add_system(player::movement.system())
            .add_system(player::combat.system());
    }
}

fn setup(
    commands: &mut Commands,
    grid: Res<Grid>,
    images: Res<Images>,
    mut events: ResMut<Events<ExitRoomEvent>>,
) {
    let mut state = dungeon::generate();

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

    commands.insert_resource(state);

    events.send(ExitRoomEvent {
        direction: Coordinates::zero(),
    });
}
