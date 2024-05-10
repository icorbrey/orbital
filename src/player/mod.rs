pub mod prelude;

use crate::prelude::*;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
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
