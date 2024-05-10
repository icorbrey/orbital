use crate::prelude::*;

pub(crate) struct GravityPlugin;
impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, compute_gravity);
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
