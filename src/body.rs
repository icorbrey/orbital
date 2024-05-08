use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_turborand::prelude::*;

use crate::motion::Motion;

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBody>()
            .add_systems(Update, (check_for_collisions, spawn_bodies));
    }
}

#[derive(Component, Debug, Default)]
pub struct Body;

#[derive(Event, Default)]
pub struct SpawnBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
    pub mass: f32,
}

pub fn spawn_bodies(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_body: EventReader<SpawnBody>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for event in ev_spawn_body.read() {
        let radius = event.mass / 10.0;
        commands.spawn((
            Body,
            Motion {
                velocity: event.velocity.extend(0.0),
                mass: event.mass,
                radius,
                ..default()
            },
            MaterialMesh2dBundle {
                transform: Transform::from_xyz(
                    event.position.x,
                    event.position.y,
                    -event.mass / 10000.0,
                ),
                material: materials.add(event.color),
                mesh: Mesh2dHandle(meshes.add(Circle { radius })),
                ..default()
            },
        ));
    }
}

pub fn check_for_collisions(
    bodies: Query<(Entity, &Transform, &Motion), With<Body>>,
    mut ev_spawn_body: EventWriter<SpawnBody>,
    mut rng: Query<&mut RngComponent>,
    mut commands: Commands,
) {
    let mut bodies = bodies.iter_combinations();
    let mut rng = rng.single_mut();

    while let Some([a, b]) = bodies.fetch_next() {
        let (entity_a, transform_a, motion_a) = a;
        let (entity_b, transform_b, motion_b) = b;

        let position_a = transform_a.translation.xy();
        let position_b = transform_b.translation.xy();

        let distance = position_a.distance(position_b);
        let threshold = motion_a.radius + motion_b.radius;

        if threshold < distance {
            continue;
        }

        let total_mass = motion_a.mass + motion_b.mass;
        let system_velocity = motion_a.velocity + motion_b.velocity;
        let system_position =
            (position_a * motion_a.mass + position_b * motion_b.mass) / total_mass;

        commands.entity(entity_a).despawn();
        commands.entity(entity_b).despawn();

        if total_mass < 100.0 {
            continue;
        }

        let rubble_count = rng.u8(4..=10);

        for i in 0..rubble_count {
            let theta: f32 = (360.0 * f32::from(i)) / f32::from(rubble_count);

            let mass = total_mass / f32::from(rubble_count);

            let velocity = 0.5
                * Vec2::new(
                    system_velocity.x * theta.cos() - system_velocity.y * theta.sin(),
                    system_velocity.x * theta.sin() + system_velocity.y * theta.cos(),
                );

            let position = system_position
                + Vec2::new(
                    25.0 * theta.cos() - 25.0 * theta.sin(),
                    25.0 * theta.sin() + 25.0 * theta.cos(),
                );

            ev_spawn_body.send(SpawnBody {
                color: Color::Hsla {
                    saturation: 0.2 + 0.8 * rng.f32(),
                    lightness: 0.2 + 0.8 * rng.f32(),
                    hue: 360.0 * rng.f32(),
                    alpha: 1.0,
                },
                position,
                velocity,
                mass,
                ..default()
            });
        }
    }
}
