pub mod gravity;
pub mod prelude;

use self::gravity::GravityPlugin;
use crate::prelude::*;

pub(crate) struct MotionPlugin;
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GravityPlugin)
            .add_systems(Update, tick_motion);
    }
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub(crate) struct Motion {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub radius: f32,
    pub mass: f32,
}

impl Motion {
    pub fn exert_force(&mut self, force: Vec3) {
        self.acceleration += force / self.mass;
    }
}

fn tick_motion(mut query: Query<(&mut Transform, &mut Motion)>, time: Res<Time>) {
    for (mut transform, mut motion) in query.iter_mut() {
        let acceleration = motion.acceleration;

        motion.velocity += acceleration;
        motion.acceleration = Vec3::ZERO;

        transform.translation += motion.velocity * time.delta_seconds();
    }
}
