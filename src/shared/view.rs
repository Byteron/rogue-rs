use bevy::prelude::*;

pub struct ViewAnchor(pub Option<Entity>);

pub struct View;

pub struct Sync;

pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(sync.system());
    }
}

fn sync(anchors: Query<(&Transform, &ViewAnchor)>, mut views: Query<&mut Transform, (With<View>, With<Sync>)>) {
    for (transform, anchor) in anchors.iter() {
        if let Some(entity) = anchor.0 {
            if let Ok(mut view_transform) = views.get_mut(entity) {
                view_transform.translation = transform.translation;
            }
        }
    }
}