mod body;
mod camera;
mod motion;

use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use body::{BodyPlugin, SpawnBody};
use camera::CameraPlugin;
use leafwing_input_manager::prelude::*;
use motion::MotionPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EditorPlugin::default()))
        .add_plugins((CameraPlugin, BodyPlugin, MotionPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_random_body)
        .run();
}

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    SpawnRandomBody,
}

fn setup(mut commands: Commands, mut ev_spawn_body: EventWriter<SpawnBody>) {
    commands.spawn(InputManagerBundle::with_map(InputMap::new([(
        Action::SpawnRandomBody,
        KeyCode::Space,
    )])));

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

fn spawn_random_body(
    mut ev_spawn_body: EventWriter<SpawnBody>,
    query: Query<&ActionState<Action>>,
) {
    let action_state = query.single();

    // if action_state.just_pressed(&Action::SpawnRandomBody) {
    //     ev_spawn_body.send(SpawnBody {
    //         position: (),
    //         velocity: (),
    //         color: (),
    //         mass: (),
    //     });
    // }
}
