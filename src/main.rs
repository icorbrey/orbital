mod body;
mod camera;
mod motion;

use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_turborand::prelude::*;
use body::{BodyPlugin, SpawnBody};
use camera::CameraPlugin;
use leafwing_input_manager::prelude::*;
use motion::MotionPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            InputManagerPlugin::<Action>::default(),
            EditorPlugin::new(),
            RngPlugin::new(),
        ))
        .add_plugins((CameraPlugin, BodyPlugin, MotionPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_random_body)
        .run();
}

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    SpawnRandomBody,
}

fn setup(
    mut ev_spawn_body: EventWriter<SpawnBody>,
    mut global_rng: ResMut<GlobalRng>,
    mut commands: Commands,
) {
    let input_map = InputMap::new([(Action::SpawnRandomBody, KeyCode::Space)]);

    commands.spawn(InputManagerBundle::with_map(input_map));
    commands.spawn(RngComponent::from(&mut global_rng));

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
    action_state: Query<&ActionState<Action>>,
    mut ev_spawn_body: EventWriter<SpawnBody>,
    mut rng: Query<&mut RngComponent>,
) {
    let action_state = action_state.single();
    let mut rng = rng.single_mut();

    if action_state.just_pressed(&Action::SpawnRandomBody) {
        ev_spawn_body.send(SpawnBody {
            velocity: 100.0 * Vec2::new(rng.f32() - 0.5, rng.f32() - 0.5),
            position: 200.0 * Vec2::new(rng.f32() - 0.5, rng.f32() - 0.5),
            mass: 1000.0 * rng.f32(),
            color: Color::Hsla {
                saturation: 0.2 + 0.8 * rng.f32(),
                lightness: 0.2 + 0.8 * rng.f32(),
                hue: 360.0 * rng.f32(),
                alpha: 1.0,
            },
        });
    }
}
