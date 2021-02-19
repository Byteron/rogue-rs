use std::time::Instant;

use bevy::{prelude::*, render::render_graph::base::MainPass};

use crate::core::math::Vec2i;

use super::{
    bob::{Coords, Layer},
    grid::Grid,
    player::Player,
    room::Room,
};

pub fn sync(
    grid: Res<Grid>,
    players: Query<&Coords, With<Player>>,
    mut query: Query<(&mut Transform, &Coords, &Layer)>,
) {
    let start = Instant::now();

    let center = players.iter().next().unwrap();
    let extents = Vec2i::new(9, 5);

    let room = Room {
        position: center.0 - extents,
        size: extents * 2,
    };

    for (mut transform, coords, layer) in query.iter_mut() {
        if room.contains(coords.0) {
            let position = grid.map_to_world(coords.0);

            transform.translation = Vec3::new(position.x as f32, position.y as f32, layer.0 as f32);
        }
    }

    println!("SyncFrame: {:?}", start.elapsed(),);
}

pub fn update(
    commands: &mut Commands,
    views: Query<&Sprite>,
    players: Query<&Coords, With<Player>>,
    mut query: Query<(Entity, &Transform, &Coords, &Handle<ColorMaterial>)>,
) {
    let start = Instant::now();

    for center in players.iter() {
        let extents = Vec2i::new(10, 6);

        let room = Room {
            position: center.0 - extents,
            size: extents * 2,
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
                }
            } else {
                if let Err(_) = views.get(entity) {
                    commands.insert(
                        entity,
                        SpriteBundle {
                            material: material.clone(),
                            transform: *transform,
                            sprite: Sprite {
                                size: Vec2::new(64.0, 64.0),
                                resize_mode: SpriteResizeMode::Manual,
                            },
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    println!(
        "ViewFrame: {:?}, Sprites: {}",
        start.elapsed(),
        query.iter().count()
    );
}
