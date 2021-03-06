mod action;
mod ai;
mod bob;
mod damage;
mod grid;
mod images;
mod physics;
mod player;
mod room;
mod tween;

use self::{action::Actions, ai::{GoblinAi, TickEvent}, bob::{Coords, Layer, SpatialMap}, damage::{AttackState, Damage, DamageEvent, Damageable}, grid::Grid, images::Images, physics::{KinematicBodyBundle, MoveEvent, PhysicsState, Solid}, player::Controllable, room::Room, tween::TweenPlugin};
use crate::{core::math::Vec2i, AppState};
use bevy::prelude::*;
use bob::BoardObjectBundle;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub enum Stage {
    Update,
    DamageUpdate,
    PhysicsUpdate,
    SyncUpdate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Input,
    Ai,
    Actions,
    Movement,
    Tweening,
    DamageEvent,
    DamageUpdate,
}

struct StateCleanup;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TweenPlugin)
            .add_event::<DamageEvent>()
            .add_event::<MoveEvent>()
            .add_event::<TickEvent>()
            .insert_resource(Grid::new(64, 64))
            .insert_resource(Actions::default())
            .insert_resource(SpatialMap::default())
            .insert_resource(AttackState::default())
            .insert_resource(PhysicsState::default())
            .init_resource::<Images>()
            .on_state_enter(Stage::Update, AppState::Dungeon, setup.system())
            // Payer Input
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                player::input
                    .system()
                    .label(Label::Input)
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
            // Commands
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                action::actions
                    .system()
                    .label(Label::Actions)
                    .after(Label::Ai),
            )
            // Combat
            .on_state_update(
                Stage::DamageUpdate,
                AppState::Dungeon,
                damage::damage.system().label(Label::DamageEvent),
            )
            .on_state_update(
                Stage::DamageUpdate,
                AppState::Dungeon,
                damage::update_state
                    .system()
                    .label(Label::DamageUpdate)
                    .after(Label::DamageEvent),
            )
            .on_state_update(
                Stage::DamageUpdate,
                AppState::Dungeon,
                damage::death.system().after(Label::DamageUpdate),
            )
            // Movement
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                physics::move_event
                    .system()
                    .label(Label::Movement)
                    .after(Label::Actions),
            )
            .on_state_update(
                Stage::Update,
                AppState::Dungeon,
                tween::tween.system()
                    .system()
                    .label(Label::Tweening)
                    .after(Label::Movement)
            )
            .on_state_update(
                Stage::PhysicsUpdate,
                AppState::Dungeon,
                physics::update_state.system(),
            )
            // View
            .on_state_update(Stage::SyncUpdate, AppState::Dungeon, bob::update_position.system())
            // .on_state_update(Stage::SyncUpdate, AppState::Dungeon, bob::update_spatial_map.system())
            .on_state_exit(
                Stage::Update,
                AppState::Dungeon,
                crate::core::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn setup(mut commands: Commands, images: Res<Images>) {
    let room = Room {
        position: Vec2i::new(0, 0),
        size: Vec2i::new(100, 100),
    };

    spawn_player(&mut commands, Coords(room.center()), images.get("Human"));

    let mut rng = rand::thread_rng();

    for coords in room.coords().iter_mut() {
        if room.is_door(*coords) {
            spawn_tile(&mut commands, Coords(*coords), images.get("Floor"), false);
        } else if room.is_entrance(*coords) {
            spawn_tile(&mut commands, Coords(*coords), images.get("Floor"), false);
        } else if room.is_border(*coords) {
            spawn_tile(&mut commands, Coords(*coords), images.get("Wall"), true);
        } else if rng.gen_bool(0.1) {
            spawn_tile(&mut commands, Coords(*coords), images.get("Wall"), true);
        } else {
            spawn_tile(&mut commands, Coords(*coords), images.get("Floor"), false);
        }
    }

    for coords in room.coords().iter_mut() {
        if rng.gen_bool(0.1)
            && !room.is_border(*coords)
            && !room.is_entrance(*coords)
            && !room.is_door(*coords)
            && !room.is_center(*coords)
        {
            spawn_enemy(&mut commands, Coords(*coords), images.get("Goblin"));
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
        .with_bundle(SpriteBundle {
            material,
            transform: Transform::default(),
            sprite: Sprite {
                size: Vec2::new(64.0, 64.0),
                resize_mode: SpriteResizeMode::Manual,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_bundle(KinematicBodyBundle::default())
        .with_bundle(OrthographicCameraBundle::new_2d())
        .with(Damageable::new(200))
        .with(Damage::new(12))
        .with(Controllable)
        .with(StateCleanup);
}

fn spawn_tile(
    commands: &mut Commands,
    coords: Coords,
    material: Handle<ColorMaterial>,
    solid: bool,
) {
    // Actual Tile Entity
    let tile = commands
        .spawn(BoardObjectBundle {
            coords,
            layer: Layer(0),
            ..Default::default()
        })
        .with_bundle(SpriteBundle {
            material,
            transform: Transform::default(),
            sprite: Sprite {
                size: Vec2::new(64.0, 64.0),
                resize_mode: SpriteResizeMode::Manual,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Damageable::new(10))
        .with(StateCleanup)
        .current_entity()
        .unwrap();

    if solid {
        commands.insert(tile, Solid);
    }
}

fn spawn_enemy(commands: &mut Commands, coords: Coords, material: Handle<ColorMaterial>) {
    // Actual Enemy Entity
    commands
        .spawn(BoardObjectBundle {
            coords,
            layer: Layer(10),
            ..Default::default()
        })
        .with_bundle(SpriteBundle {
            material,
            transform: Transform::default(),
            sprite: Sprite {
                size: Vec2::new(64.0, 64.0),
                resize_mode: SpriteResizeMode::Manual,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_bundle(KinematicBodyBundle::default())
        .with(Damageable::new(20))
        .with(GoblinAi)
        .with(StateCleanup);
}
