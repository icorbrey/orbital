pub mod prelude;

use crate::prelude::{body::*, *};

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, spawn_random_body);
    }
}

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlayerAction {
    SpawnRandomBody,
}

#[derive(Component)]
pub(crate) struct Player;

fn setup(mut commands: Commands) {
    let input_map = InputMap::new([(PlayerAction::SpawnRandomBody, KeyCode::Space)]);

    commands.spawn((
        Name::new("Player"),
        cleanup::OnExit,
        InputManagerBundle::with_map(input_map),
    ));
}

fn spawn_random_body(
    action_state: Query<&ActionState<PlayerAction>>,
    mut ev_spawn_body: EventWriter<SpawnBodyEvent>,
    mut rng: Query<&mut RngComponent>,
) {
    let action_state = action_state.single();
    let mut rng = rng.single_mut();

    if action_state.just_pressed(&PlayerAction::SpawnRandomBody) {
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
