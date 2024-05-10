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
        .add_plugins((EditorPlugin::new(), RngPlugin::new()))
        .add_plugins((BodyPlugin, CameraPlugin, MotionPlugin, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut ev_spawn_body: EventWriter<SpawnBodyEvent>,
    mut global_rng: ResMut<GlobalRng>,
    mut commands: Commands,
) {
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
