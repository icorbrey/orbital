mod body;
mod camera;

use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use body::SpawnBody;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EditorPlugin::default()))
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut ev_spawn_body: EventWriter<SpawnBody>) {
    ev_spawn_body.send_batch(vec![
        SpawnBody {
            position: Vec2::new(10.0, 0.0),
            color: Color::RED,
            mass: 100.0,
            ..default()
        },
        SpawnBody {
            position: Vec2::new(0.0, 20.0),
            color: Color::GREEN,
            mass: 200.0,
            ..default()
        },
        SpawnBody {
            position: Vec2::new(-5.0, -5.0),
            color: Color::BLUE,
            mass: 400.0,
            ..default()
        },
    ]);
}
