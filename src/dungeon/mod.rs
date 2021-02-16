mod actor;
mod bob;
mod grid;
mod player;
mod tile;
mod view;

use crate::core::{math::Vec2i, AppState, APPSTATES};
use bevy::{prelude::*, utils::HashMap};
use bob::BoardObjectBundle;

use self::{
    actor::ActorBundle,
    bob::{Coords, Layer},
    grid::Grid,
    player::Player,
    view::{View, ViewAnchor},
};

const PLAYER_MOVES: &str = "PlayerInput";
const ACTORS_MOVED: &str = "PlayerMoved";

struct StateCleanup;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Image {
    Floor,
    Wall,
    Human,
    Goblin,
}

struct Images {
    images: HashMap<Image, Handle<ColorMaterial>>,
}

impl Images {
    pub fn get(&self, image: Image) -> Handle<ColorMaterial> {
        self.images.get(&image).unwrap().clone()
    }
}

impl FromResources for Images {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = resources.get_mut::<AssetServer>().unwrap();

        let mut images: HashMap<Image, Handle<ColorMaterial>> = HashMap::default();

        images.insert(
            Image::Human,
            materials.add(assets.load("images/human.png").into()),
        );
        images.insert(
            Image::Goblin,
            materials.add(assets.load("images/goblin.png").into()),
        );
        images.insert(
            Image::Wall,
            materials.add(assets.load("images/wall.png").into()),
        );
        images.insert(
            Image::Floor,
            materials.add(assets.load("images/floor.png").into()),
        );

        Images { images }
    }
}
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

    // Actual Player Entity
    commands
        .spawn(BoardObjectBundle {
            view_anchor: ViewAnchor(Some(view)),
            coords,
            layer: Layer(0),
            ..Default::default()
        })
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
