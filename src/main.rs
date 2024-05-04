mod camera;

use bevy::prelude::*;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_scale(Vec3::new(100.0, 100.0, 0.0)),
        ..default()
    });
}
