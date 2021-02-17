mod actor;
mod ai;
mod bob;
mod combat;
mod grid;
mod images;
mod physics;
mod player;
mod tile;
mod view;

use self::{
    actor::ActorBundle,
    bob::{Coords, Layer},
    combat::CombatBundle,
    grid::Grid,
    images::{Image, Images},
    physics::{Body, KinematicBodyBundle},
    player::Player,
    tile::Tile,
    view::{View, ViewAnchor},
};
use crate::core::{math::Vec2i, AppState, APPSTATES};
use ai::{AiTickEvent, GoblinAi};
use bevy::prelude::*;
use bob::BoardObjectBundle;
use rand::Rng;

const TIMER_TICK: &str = "TimerTick";
const PLAYER_INPUT: &str = "PlayerInput";
const AI: &str = "Ai";
const COMBAT: &str = "Combat";
const MOVEMENT: &str = "Movement";
const MOVED: &str = "PlayerMoved";

struct StateCleanup;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AiTickEvent>()
            .insert_resource(Grid::new(64, 64))
            .init_resource::<Images>()
            .on_state_enter(APPSTATES, AppState::Dungeon, setup.system())
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                actor::tick.system().label(TIMER_TICK),
            )
            // Payer Input
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                player::movement_input
                    .system()
                    .label(PLAYER_INPUT)
                    .after(TIMER_TICK),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                player::combat_input
                    .system()
                    .after(TIMER_TICK)
                    .before(PLAYER_INPUT),
            )
            // AI
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                ai::goblin_ai_movement
                    .system()
                    .label(AI)
                    .after(PLAYER_INPUT),
            )
            // Combat
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                combat::attack.system().label(COMBAT).after(PLAYER_INPUT),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                combat::death.system().after(COMBAT),
            )
            // Movement
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                physics::movement
                    .system()
                    .label(MOVEMENT)
                    .after(PLAYER_INPUT),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                bob::update_position.system().label(MOVED).after(MOVEMENT),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                view::sync_views.system().after(MOVED),
            )
            .on_state_exit(
                APPSTATES,
                AppState::Dungeon,
                crate::core::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn setup(commands: &mut Commands, grid: Res<Grid>, images: Res<Images>) {
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .with(StateCleanup);

    spawn_player(commands, &images, &grid);

    for y in -10..=10 {
        for x in -10..=10 {
            spawn_tile(commands, &grid, &images, Coords(Vec2i::new(x, y)));
        }
    }

    for y in -9..=9 {
        for x in -9..=9 {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.1) {
                spawn_enemy(commands, &grid, &images, Coords(Vec2i::new(x, y)));
            }
        }
    }
}

fn spawn_player(commands: &mut Commands, images: &Images, grid: &Grid) {
    // Player's View
    let view = create_view(commands, &grid, images, Image::Human);

    // Actual Player Entity
    commands
        .spawn(BoardObjectBundle {
            view_anchor: ViewAnchor(Some(view)),
            layer: Layer(10),
            ..Default::default()
        })
        .with_bundle(ActorBundle::default())
        .with_bundle(CombatBundle::default())
        .with_bundle(KinematicBodyBundle::solid())
        .with(Player)
        .with(StateCleanup);
}

fn spawn_tile(commands: &mut Commands, grid: &Grid, images: &Images, coords: Coords) {
    let mut rng = rand::thread_rng();
    let solid: bool;

    // Tile's View
    let view: Entity;

    if coords.0.x == 10 || coords.0.y == 10 || coords.0.x == -10 || coords.0.y == -10 {
        view = create_view(commands, &grid, images, Image::Wall);
        solid = true;
    } else if rng.gen_bool(0.1) {
        view = create_view(commands, &grid, images, Image::Wall);
        solid = true;
    } else {
        view = create_view(commands, &grid, images, Image::Floor);
        solid = false;
    }

    // Actual Tile Entity
    commands
        .spawn(BoardObjectBundle {
            view_anchor: ViewAnchor(Some(view)),
            coords,
            layer: Layer(0),
            ..Default::default()
        })
        .with(Body::new(solid))
        .with(Tile)
        .with(StateCleanup);
}

fn spawn_enemy(commands: &mut Commands, grid: &Grid, images: &Images, coords: Coords) {
    // Enemy's View
    let view = create_view(commands, &grid, images, Image::Goblin);

    // Actual Enemy Entity
    commands
        .spawn(BoardObjectBundle {
            view_anchor: ViewAnchor(Some(view)),
            coords,
            layer: Layer(10),
            ..Default::default()
        })
        .with_bundle(ActorBundle::default())
        .with_bundle(CombatBundle::default())
        .with_bundle(KinematicBodyBundle::solid())
        .with(GoblinAi)
        .with(StateCleanup);
}

fn create_view(commands: &mut Commands, grid: &Grid, images: &Images, image: Image) -> Entity {
    commands
        .spawn(SpriteBundle {
            material: images.get(image),
            transform: Transform::from_translation(Vec3::zero()),
            sprite: Sprite {
                size: grid.cell_size.as_f32(),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(View)
        .with(StateCleanup)
        .current_entity()
        .unwrap()
}
