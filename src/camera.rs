use bevy::prelude::*;

use crate::body::Body;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::focus_on_bodies);
    }
}

/// The actual, for realsies camera. Not the fake, for fakesies camera that the
/// editor plugin uses.
#[derive(Component)]
pub struct TheActualForRealsiesCamera;

impl CameraPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn((TheActualForRealsiesCamera, Camera2dBundle::default()));
    }

    fn focus_on_bodies(
        mut cameras: Query<&mut Transform, (With<TheActualForRealsiesCamera>, Without<Body>)>,
        bodies: Query<&Transform, With<Body>>,
    ) {
        let mut camera_transform = cameras.single_mut();

        if bodies.is_empty() {
            camera_transform.translation.x = 0.0;
            camera_transform.translation.y = 0.0;
        } else {
        }
    }
}
