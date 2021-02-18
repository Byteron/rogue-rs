mod actor;
mod ai;
mod bob;
mod combat;
mod grid;
mod images;
mod physics;
mod player;
mod room;
mod tile;
mod view;

use self::{
    actor::ActorBundle,
    bob::{Coords, Layer},
    combat::CombatBundle,
    grid::Grid,
    images::{ActorImages, TileImages},
    physics::{KinematicBodyBundle, Solid},
    player::Player,
    room::Room,
    tile::TileType,
};
use crate::{
    core::math::Vec2i, AppState, APPSTATE_UPDATE, PHYSICS_UPDATE, VIEW_STARTUP, VIEW_UPDATE,
};
use actor::ActorType;
use ai::{AiTickEvent, GoblinAi};
use bevy::prelude::*;
use bob::BoardObjectBundle;
use combat::Attitude;
use rand::Rng;

const TIMER_TICK: &str = "TimerTick";
const PLAYER_INPUT: &str = "PlayerInput";
const AI: &str = "Ai";
const COMBAT: &str = "Combat";

struct StateCleanup;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AiTickEvent>()
            .insert_resource(Grid::new(64, 64))
            .init_resource::<ActorImages>()
            .init_resource::<TileImages>()
            .on_state_enter(APPSTATE_UPDATE, AppState::Dungeon, setup.system())
            .on_state_update(
                APPSTATE_UPDATE,
                AppState::Dungeon,
                actor::tick.system().label(TIMER_TICK),
            )
            // Payer Input
            .on_state_update(
                APPSTATE_UPDATE,
                AppState::Dungeon,
                player::movement_input
                    .system()
                    .label(PLAYER_INPUT)
                    .after(TIMER_TICK),
            )
            .on_state_update(
                APPSTATE_UPDATE,
                AppState::Dungeon,
                player::combat_input
                    .system()
                    .after(TIMER_TICK)
                    .before(PLAYER_INPUT),
            )
            // AI
            .on_state_update(
                APPSTATE_UPDATE,
                AppState::Dungeon,
                ai::goblin_ai_movement
                    .system()
                    .label(AI)
                    .after(PLAYER_INPUT),
            )
            // Combat
            .on_state_update(
                APPSTATE_UPDATE,
                AppState::Dungeon,
                combat::attack.system().label(COMBAT).after(AI),
            )
            .on_state_update(
                APPSTATE_UPDATE,
                AppState::Dungeon,
                combat::death.system().after(COMBAT),
            )
            // Movement
            .on_state_update(PHYSICS_UPDATE, AppState::Dungeon, physics::update.system())
            .on_state_update(VIEW_STARTUP, AppState::Dungeon, view::spawn_views.system())
            .on_state_update(VIEW_UPDATE, AppState::Dungeon, view::sync_views.system())
            .on_state_exit(
                APPSTATE_UPDATE,
                AppState::Dungeon,
                crate::core::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn setup(commands: &mut Commands, grid: Res<Grid>) {
    let room = Room {
        position: Vec2i::new(0, 0),
        size: Vec2i::new(19, 11),
    };

    commands
        .spawn(OrthographicCameraBundle {
            transform: Transform::from_translation(
                grid.map_to_world(room.center()).extend(1000).as_f32(),
            ),
            ..OrthographicCameraBundle::new_2d()
        })
        .with(StateCleanup);

    spawn_player(commands, Coords(room.center()));

    let mut rng = rand::thread_rng();

    for coords in room.coords().iter_mut() {
        if room.is_door(*coords) {
            spawn_tile(commands, Coords(*coords), TileType::Floor, false);
        } else if room.is_entrance(*coords) {
            spawn_tile(commands, Coords(*coords), TileType::Floor, false);
        } else if room.is_border(*coords) {
            spawn_tile(commands, Coords(*coords), TileType::Wall, true);
        } else if rng.gen_bool(0.1) {
            spawn_tile(commands, Coords(*coords), TileType::Wall, true);
        } else {
            spawn_tile(commands, Coords(*coords), TileType::Floor, false);
        }
    }

    for coords in room.coords().iter_mut() {
        if rng.gen_bool(0.1)
            && !room.is_border(*coords)
            && !room.is_entrance(*coords)
            && !room.is_door(*coords)
            && !room.is_center(*coords)
        {
            spawn_enemy(commands, Coords(*coords), ActorType::Goblin);
        }
    }
}

fn spawn_player(commands: &mut Commands, coords: Coords) {
    // Actual Player Entity
    commands
        .spawn(BoardObjectBundle {
            coords,
            layer: Layer(10),
            ..Default::default()
        })
        .with_bundle(ActorBundle::default())
        .with_bundle(CombatBundle::new(100, 12, Attitude::Neutral))
        .with_bundle(KinematicBodyBundle::default())
        .with(Player)
        .with(StateCleanup);
}

fn spawn_tile(commands: &mut Commands, coords: Coords, tile_type: TileType, solid: bool) {
    // Actual Tile Entity
    let tile = commands
        .spawn(BoardObjectBundle {
            coords,
            layer: Layer(0),
            ..Default::default()
        })
        .with(tile_type)
        .with(StateCleanup)
        .current_entity()
        .unwrap();

    if solid {
        commands.insert_one(tile, Solid);
    }
}

fn spawn_enemy(commands: &mut Commands, coords: Coords, actor_type: ActorType) {
    // Actual Enemy Entity
    commands
        .spawn(BoardObjectBundle {
            coords,
            layer: Layer(10),
            ..Default::default()
        })
        .with_bundle(ActorBundle::new(actor_type))
        .with_bundle(CombatBundle::new(20, 3, Attitude::Hostile))
        .with_bundle(KinematicBodyBundle::default())
        .with(GoblinAi)
        .with(StateCleanup);
}
