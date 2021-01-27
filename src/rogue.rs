use crate::{
    combat::CombatPlugin,
    dungeon::{self, DungeonPlugin, RoomExitedEvent},
    enemies::{Enemies, EnemiesPlugin},
    grid::{Grid, GridPlugin, Vec2i, Vec3i},
    player::PlayerPlugin,
    rooms::{Rooms, RoomsPlugin, Tiles},
    tween::TweenPlugin,
};

use bevy::prelude::*;

pub struct GameState {
    pub current_level: i32,
    pub current_room: Vec3i,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            current_level: 0,
            current_room: Vec3i::zero(),
        }
    }
}

pub struct Rogue;

impl Plugin for Rogue {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(TweenPlugin)
            .add_plugin(GridPlugin)
            .add_plugin(RoomsPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(DungeonPlugin)
            .add_plugin(CombatPlugin)
            .add_resource(GameState::default())
            .add_startup_system(setup.system());
    }
}

fn setup(
    commands: &mut Commands,
    grid: Res<Grid>,
    mut state: ResMut<GameState>,
    mut rooms: ResMut<Rooms>,
    mut tiles: ResMut<Tiles>,
    mut enemies: ResMut<Enemies>,
    mut events: ResMut<Events<RoomExitedEvent>>,
) {
    let start = dungeon::generate(commands, &mut rooms, &mut tiles, &mut enemies);
    let room = rooms.0.get(&start).unwrap();

    let translation = grid.map_to_world(room.center());

    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(translation),
        ..Default::default()
    });

    state.current_room = start;
    events.send(RoomExitedEvent {
        direction: Vec2i::zero(),
    });
}
