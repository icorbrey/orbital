use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::body::{Body, Mass};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::focus_on_center_of_mass);
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CameraMovement {
    Pan,
}

/// The actual, for realsies camera. Not the fake, for fakesies camera that the
/// editor plugin uses.
#[derive(Component)]
pub struct TheActualForRealsiesCamera;

impl CameraPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn((TheActualForRealsiesCamera, Camera2dBundle::default()));
    }

    fn focus_on_center_of_mass(
        mut cameras: Query<&mut Transform, (With<TheActualForRealsiesCamera>, Without<Body>)>,
        bodies: Query<(&Mass, &Transform), With<Body>>,
    ) {
        let mut camera_transform = cameras.single_mut();

        if bodies.is_empty() {
            camera_transform.translation.x = 0.0;
            camera_transform.translation.y = 0.0;
        } else {
            let total_mass: f32 = bodies.iter().map(|(mass, _)| mass.0).sum();
            let center_of_mass = bodies
                .iter()
                .map(|(mass, transform)| {
                    let Vec3 { x, y, .. } = transform.translation;
                    Vec2::new(x * mass.0, y * mass.0)
                })
                .sum::<Vec2>()
                / total_mass;

            camera_transform.translation.x = center_of_mass.x;
            camera_transform.translation.y = center_of_mass.y;
        }
    }
}
