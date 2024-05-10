use bevy::prelude::*;

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (compute_gravity, tick_motion).chain());
    }
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct Motion {
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

const GRAV_CONST: f32 = 4.0;

fn compute_gravity(mut query: Query<(&Transform, &mut Motion)>) {
    let mut bodies = query.iter_combinations_mut();
    while let Some([a, b]) = bodies.fetch_next() {
        let (transform_a, mut motion_a) = a;
        let (transform_b, mut motion_b) = b;

        let interval = transform_b.translation - transform_a.translation;
        let distance = interval.length();

        let mut gravity =
            GRAV_CONST * interval.normalize() * motion_a.mass * motion_b.mass / distance.powf(2.0);
        gravity.z = 0.0;

        motion_a.exert_force(gravity);
        motion_b.exert_force(-1.0 * gravity);
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
