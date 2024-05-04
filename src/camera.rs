use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraMovement>::default())
            .add_systems(Startup, Self::setup)
            .add_systems(Update, Self::pan);
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CameraMovement {
    Pan,
}

impl CameraPlugin {
    const PAN_RATE: f32 = 0.5;

    /// Adds a 2D camera to the scene and adds the input map.
    fn setup(mut commands: Commands) {
        let input_map = InputMap::new([(CameraMovement::Pan, DualAxis::mouse_motion())]);
        commands
            .spawn(Camera2dBundle::default())
            .insert(InputManagerBundle::with_map(input_map));
    }

    /// Pans the camera when the mouse moves.
    fn pan(mut query: Query<(&mut Transform, &ActionState<CameraMovement>), With<Camera2d>>) {
        let (mut transform, action_state) = query.single_mut();
        let direction = action_state.axis_pair(&CameraMovement::Pan).unwrap();

        transform.translation.x -= Self::PAN_RATE * direction.x();
        transform.translation.y += Self::PAN_RATE * direction.y();
    }
}
