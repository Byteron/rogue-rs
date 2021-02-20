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
    physics::{KinematicBodyBundle, PhysicsState, Solid},
    player::Player,
    room::Room,
    tile::TileType,
};
use crate::{core::math::Vec2i, AppState};
use actor::ActorType;
use ai::{AiTickEvent, GoblinAi};
use bevy::prelude::*;
use bob::BoardObjectBundle;
use combat::Attitude;
use rand::Rng;
use view::Viewshed;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub enum Stage {
    Update,
    PhysicsUpdate,
    ViewUpdate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Tick,
    Input,
    Ai,
    Combat,
}

struct StateCleanup;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AiTickEvent>()
            .insert_resource(Grid::new(64, 64))
            .insert_resource(PhysicsState::default())
            .init_resource::<ActorImages>()
            .init_resource::<TileImages>()
            .on_state_enter(Stage::Update, AppState::Dungeon, setup.system())
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                actor::tick.system().label(Label::Tick),
            )
            // Payer Input
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                player::input
                    .system()
                    .label(Label::Input)
                    .after(Label::Tick),
            )
            // AI
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                ai::goblin_ai_movement
                    .system()
                    .label(Label::Ai)
                    .after(Label::Input),
            )
            // Combat
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                combat::attack
                    .system()
                    .label(Label::Combat)
                    .after(Label::Ai),
            )
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                combat::death.system().after(Label::Combat),
            )
            // Movement
            .on_state_update(
                Stage::PhysicsUpdate,
                AppState::Dungeon,
                physics::update.system(),
            )
            // View
            .on_state_update(Stage::ViewUpdate, AppState::Dungeon, view::update.system())
            .on_state_update(Stage::ViewUpdate, AppState::Dungeon, view::sync.system())
            .on_state_exit(
                Stage::Update,
                AppState::Dungeon,
                crate::core::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn setup(
    commands: &mut Commands,
    grid: Res<Grid>,
    actor_images: Res<ActorImages>,
    tile_images: Res<TileImages>,
) {
    let room = Room {
        position: Vec2i::new(0, 0),
        size: Vec2i::new(200, 200),
    };

    commands
        .spawn(OrthographicCameraBundle {
            transform: Transform::from_translation(
                grid.map_to_world(room.center()).extend(1000).as_f32(),
            ),
            ..OrthographicCameraBundle::new_2d()
        })
        .with(Viewshed {
            size: Vec2i::new(19, 11),
        })
        .with(Coords(room.center()))
        .with(StateCleanup);

    spawn_player(
        commands,
        Coords(room.center()),
        actor_images.get(ActorType::Human),
    );

    let mut rng = rand::thread_rng();

    for coords in room.coords().iter_mut() {
        if room.is_door(*coords) {
            spawn_tile(
                commands,
                Coords(*coords),
                tile_images.get(TileType::Floor),
                TileType::Floor,
                false,
            );
        } else if room.is_entrance(*coords) {
            spawn_tile(
                commands,
                Coords(*coords),
                tile_images.get(TileType::Floor),
                TileType::Floor,
                false,
            );
        } else if room.is_border(*coords) {
            spawn_tile(
                commands,
                Coords(*coords),
                tile_images.get(TileType::Wall),
                TileType::Wall,
                true,
            );
        } else if rng.gen_bool(0.1) {
            spawn_tile(
                commands,
                Coords(*coords),
                tile_images.get(TileType::Wall),
                TileType::Wall,
                true,
            );
        } else {
            spawn_tile(
                commands,
                Coords(*coords),
                tile_images.get(TileType::Floor),
                TileType::Floor,
                false,
            );
        }
    }

    for coords in room.coords().iter_mut() {
        if rng.gen_bool(0.1)
            && !room.is_border(*coords)
            && !room.is_entrance(*coords)
            && !room.is_door(*coords)
            && !room.is_center(*coords)
        {
            spawn_enemy(
                commands,
                Coords(*coords),
                actor_images.get(ActorType::Goblin),
                ActorType::Goblin,
            );
        }
    }
}

fn spawn_player(commands: &mut Commands, coords: Coords, material: Handle<ColorMaterial>) {
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
        .with(material)
        .with(Player)
        .with(StateCleanup);
}

fn spawn_tile(
    commands: &mut Commands,
    coords: Coords,
    material: Handle<ColorMaterial>,
    tile_type: TileType,
    solid: bool,
) {
    // Actual Tile Entity
    let tile = commands
        .spawn(BoardObjectBundle {
            coords,
            layer: Layer(0),
            ..Default::default()
        })
        .with(material)
        .with(tile_type)
        .with(StateCleanup)
        .current_entity()
        .unwrap();

    if solid {
        commands.insert_one(tile, Solid);
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    coords: Coords,
    material: Handle<ColorMaterial>,
    actor_type: ActorType,
) {
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
        .with(material)
        .with(GoblinAi)
        .with(StateCleanup);
}
