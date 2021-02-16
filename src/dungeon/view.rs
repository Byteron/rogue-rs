use bevy::prelude::*;

pub struct ViewAnchor(pub Option<Entity>);

pub struct View;

pub fn sync_views(anchors: Query<(&Transform, &ViewAnchor), Changed<Transform>>, mut views: Query<&mut Transform, With<View>>) {
    for (transform, anchor) in anchors.iter() {
        if let Some(entity) = anchor.0 {
            if let Ok(mut view_transform) = views.get_mut(entity) {
                view_transform.translation = transform.translation;
                println!("Sync!");
            }
        }
    }
}