mod bob;
mod grid;
mod images;
mod physics;
mod player;
mod tile;
mod view;

use self::{
    bob::{Coords, Layer},
    grid::Grid,
    images::{Image, Images},
    physics::{Body, KinematicBodyBundle},
    player::Player,
    tile::Tile,
    view::{View, ViewAnchor},
};
use crate::core::{math::Vec2i, AppState, APPSTATES};
use bevy::prelude::*;
use bob::BoardObjectBundle;
use rand::Rng;

const PLAYER_MOVES: &str = "PlayerInput";
const ACTORS_MOVED: &str = "PlayerMoved";
const PHYSICS_COLLISION: &str = "PhysicsCollision";
const PHYSICS_STEP: &str = "PhysicsStep";

struct StateCleanup;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Grid::new(64, 64))
            .init_resource::<Images>()
            .on_state_enter(APPSTATES, AppState::Dungeon, setup.system())
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                player::movement.system().label(PLAYER_MOVES),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                physics::step_timer_tick.system().before(PHYSICS_COLLISION),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                physics::collision
                    .system()
                    .label(PHYSICS_COLLISION)
                    .after(PLAYER_MOVES),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                physics::step
                    .system()
                    .label(PHYSICS_STEP)
                    .after(PHYSICS_COLLISION),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                physics::cleanup.system().after(PHYSICS_STEP),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                bob::update_position
                    .system()
                    .label(ACTORS_MOVED)
                    .after(PHYSICS_STEP),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                view::sync_views.system().after(ACTORS_MOVED),
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
        .with_bundle(KinematicBodyBundle::solid())
        .with(Player)
        .with(StateCleanup);
}

fn spawn_tile(commands: &mut Commands, grid: &Grid, images: &Images, coords: Coords) {
    let mut rng = rand::thread_rng();
    let solid: bool;

    // Tile's View
    let view: Entity;

    if rng.gen_bool(0.1) {
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
