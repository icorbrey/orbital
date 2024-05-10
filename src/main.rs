mod body;
mod camera;
mod motion;
mod player;
mod prelude;

use crate::prelude::{body::*, camera::*, motion::*, player::*, *};

mod cleanup {
    use crate::prelude::*;

    #[derive(Component)]
    pub struct OnExit;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            InputManagerPlugin::<Action>::default(),
            EditorPlugin::new(),
            RngPlugin::new(),
        ))
        .add_plugins((BodyPlugin, CameraPlugin, MotionPlugin, PlayerPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_random_body)
        .run();
}

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    SpawnRandomBody,
}

fn setup(
    mut ev_spawn_body: EventWriter<SpawnBodyEvent>,
    mut global_rng: ResMut<GlobalRng>,
    mut commands: Commands,
) {
    let input_map = InputMap::new([(Action::SpawnRandomBody, KeyCode::Space)]);

    commands.spawn((
        Name::new("Input Manager"),
        cleanup::OnExit,
        InputManagerBundle::with_map(input_map),
    ));
    commands.spawn((
        Name::new("Random Number Generator"),
        cleanup::OnExit,
        RngComponent::from(&mut global_rng),
    ));

    ev_spawn_body.send_batch(vec![
        SpawnBodyEvent {
            position: Vec2::new(100.0, 0.0),
            velocity: 50.0 * Vec2::Y,
            color: Color::RED,
            mass: 10000.0,
            ..default()
        },
        SpawnBodyEvent {
            position: Vec2::new(0.0, 200.0),
            velocity: 50.0 * Vec2::NEG_X,
            color: Color::GREEN,
            mass: 20000.0,
            ..default()
        },
        SpawnBodyEvent {
            position: Vec2::new(-50.0, -50.0),
            velocity: 50.0 * Vec2::NEG_ONE,
            color: Color::BLUE,
            mass: 40000.0,
            ..default()
        },
    ]);
}

fn spawn_random_body(
    action_state: Query<&ActionState<Action>>,
    mut ev_spawn_body: EventWriter<SpawnBodyEvent>,
    mut rng: Query<&mut RngComponent>,
) {
    let action_state = action_state.single();
    let mut rng = rng.single_mut();

    if action_state.just_pressed(&Action::SpawnRandomBody) {
        ev_spawn_body.send(SpawnBodyEvent {
            velocity: 100.0 * Vec2::new(rng.f32() - 0.5, rng.f32() - 0.5),
            position: 200.0 * Vec2::new(rng.f32() - 0.5, rng.f32() - 0.5),
            mass: 1000000.0 * rng.f32(),
            color: Color::Hsla {
                saturation: 0.2 + 0.8 * rng.f32(),
                lightness: 0.2 + 0.8 * rng.f32(),
                hue: 360.0 * rng.f32(),
                alpha: 1.0,
            },
        });
    }
}
