mod actor;
mod bob;
mod grid;
mod player;
mod tile;
mod view;

use self::{
    actor::ActorBundle,
    bob::{Coords, Layer},
    grid::Grid,
    player::Player,
    tile::Tile,
    view::{View, ViewAnchor},
};
use crate::core::{
    images::{Image, Images},
    math::Vec2i,
    AppState, APPSTATES,
};
use bevy::prelude::*;
use bob::BoardObjectBundle;

const PLAYER_MOVES: &str = "PlayerInput";
const ACTORS_MOVED: &str = "PlayerMoved";

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
                actor::step_timer_tick.system().before(PLAYER_MOVES),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                bob::update_position
                    .system()
                    .after(PLAYER_MOVES)
                    .label(ACTORS_MOVED),
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
        .with_bundle(ActorBundle::default())
        .with(Player)
        .with(StateCleanup);
}

fn spawn_tile(commands: &mut Commands, grid: &Grid, images: &Images, coords: Coords) {
    // Tile's View
    let view = create_view(commands, &grid, images, Image::Floor);

    // Actual Tile Entity
    commands
        .spawn(BoardObjectBundle {
            view_anchor: ViewAnchor(Some(view)),
            coords,
            layer: Layer(0),
            ..Default::default()
        })
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
