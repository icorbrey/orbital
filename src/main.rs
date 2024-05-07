mod body;
mod camera;
mod motion;

use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use body::{BodyPlugin, SpawnBody};
use camera::CameraPlugin;
use motion::MotionPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EditorPlugin::default()))
        .add_plugins((CameraPlugin, BodyPlugin, MotionPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut ev_spawn_body: EventWriter<SpawnBody>) {
    ev_spawn_body.send_batch(vec![
        SpawnBody {
            position: Vec2::new(100.0, 0.0),
            velocity: Vec2::Y,
            color: Color::RED,
            mass: 100.0,
            ..default()
        },
        SpawnBody {
            position: Vec2::new(0.0, 200.0),
            color: Color::GREEN,
            velocity: Vec2::NEG_X,
            mass: 200.0,
            ..default()
        },
        SpawnBody {
            position: Vec2::new(-50.0, -50.0),
            velocity: Vec2::NEG_ONE,
            color: Color::BLUE,
            mass: 400.0,
            ..default()
        },
    ]);
}
