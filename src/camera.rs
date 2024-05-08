use bevy::prelude::*;

use crate::{body::Body, motion::Motion};

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
        mut cameras: Query<
            (&mut Transform, &mut OrthographicProjection),
            (With<TheActualForRealsiesCamera>, Without<Body>),
        >,
        bodies: Query<(&Transform, &Motion), With<Body>>,
    ) {
        let (mut camera_transform, mut projection) = cameras.single_mut();

        let mut weighted_position = Vec2::default();
        let mut total_mass = 0.0;

        let mut min = Vec2::default();
        let mut max = Vec2::default();

        for (transform, motion) in bodies.iter() {
            weighted_position += transform.translation.xy() * motion.mass;
            total_mass += motion.mass;

            if transform.translation.x < min.x {
                min.x = transform.translation.x;
            }

            if max.x < transform.translation.x {
                max.x = transform.translation.x;
            }

            if transform.translation.y < min.y {
                min.y = transform.translation.y;
            }

            if max.y < transform.translation.y {
                max.y = transform.translation.y;
            }
        }

        let center_of_mass = weighted_position / total_mass;

        camera_transform.translation.x = center_of_mass.x;
        camera_transform.translation.y = center_of_mass.y;

        projection.scale = 0.9 + 0.0005 * (min).distance(max);
    }
}
