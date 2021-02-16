use bevy::prelude::*;

use super::bob::Position;

pub struct ViewAnchor(pub Option<Entity>);

pub struct View;

pub fn sync_views(
    anchors: Query<(&Position, &ViewAnchor), Changed<Position>>,
    mut views: Query<&mut Transform, With<View>>,
) {
    for (position, anchor) in anchors.iter() {
        if let Some(entity) = anchor.0 {
            if let Ok(mut view_transform) = views.get_mut(entity) {
                view_transform.translation = position.0.extend(position.1).as_f32();
                println!("Sync!");
            }
        }
    }
}
