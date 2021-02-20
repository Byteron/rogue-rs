use std::time::Instant;

use bevy::{
    prelude::*,
    render::{camera::Camera, render_graph::base::MainPass},
};

use crate::core::math::Vec2i;

pub struct Viewshed {
    pub size: Vec2i,
}

use super::{
    bob::{Coords, Layer},
    grid::Grid,
    room::Room,
};

pub fn sync(
    grid: Res<Grid>,
    players: Query<(&Coords, &Viewshed), With<Camera>>,
    mut query: Query<(&mut Transform, &Coords, &Layer)>,
) {
    for (center, viewshed) in players.iter() {
        let room = Room {
            position: center.0 - viewshed.size / Vec2i::new(2, 2),
            size: viewshed.size,
        };

        for (mut transform, coords, layer) in query.iter_mut() {
            if room.contains(coords.0) {
                let position = grid.map_to_world(coords.0);

                transform.translation =
                    Vec3::new(position.x as f32, position.y as f32, layer.0 as f32);
            }
        }
    }
}

pub fn update(
    commands: &mut Commands,
    grid: Res<Grid>,
    views: Query<&Sprite>,
    players: Query<(&Coords, &Viewshed), With<Camera>>,
    mut query: Query<(Entity, &Transform, &Coords, &Handle<ColorMaterial>)>,
) {
    let start = Instant::now();

    let mut visible = 0;
    let mut removed = 0;
    let mut added = 0;

    for (center, viewshed) in players.iter() {
        let room = Room {
            position: center.0 - viewshed.size / Vec2i::new(2, 2),
            size: viewshed.size,
        };

        for (entity, transform, coords, material) in query.iter_mut() {
            if !room.contains(coords.0) {
                if let Ok(_) = views.get(entity) {
                    commands.remove_one::<Sprite>(entity);
                    commands.remove_one::<Handle<Mesh>>(entity);
                    commands.remove_one::<Visible>(entity);
                    commands.remove_one::<MainPass>(entity);
                    commands.remove_one::<RenderPipelines>(entity);
                    commands.remove_one::<Draw>(entity);

                    removed += 1;
                }
            } else {
                visible += 1;
                if let Err(_) = views.get(entity) {
                    commands.insert(
                        entity,
                        SpriteBundle {
                            material: material.clone(),
                            transform: *transform,
                            sprite: Sprite {
                                size: grid.cell_size.as_f32(),
                                resize_mode: SpriteResizeMode::Manual,
                            },
                            ..Default::default()
                        },
                    );

                    added += 1;
                }
            }
        }
    }

    println!(
        "ViewFrame: {:?}, Sprites: {}, Added: {}, Removed: {}, Visible: {}",
        start.elapsed(),
        query.iter().count(),
        added,
        removed,
        visible,
    );
}
