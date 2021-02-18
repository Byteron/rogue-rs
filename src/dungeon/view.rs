use bevy::prelude::*;

use super::{
    actor::ActorType,
    bob::{Coords, Layer},
    grid::Grid,
    images::{ActorImages, TileImages},
    tile::TileType,
    StateCleanup,
};

pub struct ViewAnchor(pub Option<Entity>);

pub struct View;

pub fn sync_views(
    grid: Res<Grid>,
    anchors: Query<(&Coords, &Layer, &ViewAnchor), Or<(Changed<Coords>, Changed<ViewAnchor>)>>,
    mut views: Query<&mut Transform, With<View>>,
) {
    for (coords, layer, anchor) in anchors.iter() {
        if let Some(entity) = anchor.0 {
            if let Ok(mut view_transform) = views.get_mut(entity) {
                let position = grid.map_to_world(coords.0);
                view_transform.translation =
                    Vec3::new(position.x as f32, position.y as f32, layer.0 as f32);
            }
        }
    }
}

pub fn spawn_views(
    commands: &mut Commands,
    grid: Res<Grid>,
    actor_images: Res<ActorImages>,
    tile_images: Res<TileImages>,
    actors: Query<(Entity, &ActorType), Added<ActorType>>,
    tiles: Query<(Entity, &TileType), Added<TileType>>,
    mut anchors: Query<&mut ViewAnchor, Added<ViewAnchor>>,
) {
    for (entity, actor) in actors.iter() {
        let view = create_view(commands, &grid, actor_images.get(*actor));
        if let Ok(mut anchor) = anchors.get_mut(entity) {
            anchor.0 = Some(view);
        }
    }

    for (entity, tile) in tiles.iter() {
        let view = create_view(commands, &grid, tile_images.get(*tile));
        if let Ok(mut anchor) = anchors.get_mut(entity) {
            anchor.0 = Some(view);
        }
    }
}

fn create_view(commands: &mut Commands, grid: &Grid, material: Handle<ColorMaterial>) -> Entity {
    commands
        .spawn(SpriteBundle {
            material,
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
