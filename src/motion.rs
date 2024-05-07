use bevy::prelude::*;

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (compute_gravity, tick_motion).chain());
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Motion {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
}

const GRAV_CONST: f32 = 4.0;

fn compute_gravity(mut query: Query<(&Transform, &mut Motion)>) {
    let mut bodies = query.iter_combinations_mut();
    while let Some([a, b]) = bodies.fetch_next() {
        let (transform_a, mut motion_a) = a;
        let (transform_b, mut motion_b) = b;

        let mass_a = motion_a.mass;
        let mass_b = motion_b.mass;

        let interval = transform_b.translation - transform_a.translation;
        let distance = interval.length();

        let gravity = GRAV_CONST * interval.normalize() * mass_a * mass_b / distance.powf(2.0);

        motion_a.acceleration += gravity / mass_a;
        motion_b.acceleration -= gravity / mass_b;
    }
}

fn tick_motion(mut query: Query<(&mut Transform, &mut Motion)>, time: Res<Time>) {
    for (mut transform, mut motion) in query.iter_mut() {
        let acceleration = motion.acceleration;

        motion.velocity += acceleration;
        motion.acceleration = Vec3::ZERO;

        println!("{:?}", motion);

        transform.translation += motion.velocity * time.delta_seconds();
    }
}
