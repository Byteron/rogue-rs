mod actor;
mod grid;
mod player;
mod view;

use crate::core::{APPSTATES, AppState};
use bevy::prelude::*;

use self::{actor::ActorBundle, grid::Grid, player::Player, view::{View, ViewAnchor}};

const PLAYER_MOVES: &str = "PlayerInput";
const ACTORS_MOVED: &str = "PlayerMoved";

struct StateCleanup;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Grid::new(64, 64))
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
                actor::update_position.system().after(PLAYER_MOVES).label(ACTORS_MOVED),
            )
            .on_state_update(
                APPSTATES,
                AppState::Dungeon,
                view::sync_views.system().after(ACTORS_MOVED))
            .on_state_exit(
                APPSTATES,
                AppState::Dungeon,
                crate::core::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn setup(
    commands: &mut Commands,
    grid: Res<Grid>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .with(StateCleanup);

    let view = spawn_player_view(commands, &mut materials, &assets, &grid);
    spawn_player(commands, view);
}

fn spawn_player(
    commands: &mut Commands,
    view: Entity
) {
    commands
        .spawn(ActorBundle::default())
        .with(Player)
        .with(ViewAnchor(Some(view)))
        .with(StateCleanup);
}

fn spawn_player_view(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    assets: &AssetServer,
    grid: &Grid,
) -> Entity {
    commands
    .spawn(SpriteBundle {
        material: materials.add(assets.load("images/human.png").into()),
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
