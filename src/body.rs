use std::f32::consts::PI;

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

const DENSITY: f32 = 0.551; // g/cm^3

pub fn spawn_bodies(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_body: EventReader<SpawnBody>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for event in ev_spawn_body.read() {
        let radius = ((3.0 * event.mass / DENSITY) / (4.0 * PI)).cbrt();
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
        let mass_disparity = (motion_a.mass - motion_b.mass).abs() + 0.0001;
        let system_velocity = (motion_a.velocity + motion_b.velocity).xy();
        let system_position =
            (position_a * motion_a.mass + position_b * motion_b.mass) / total_mass;
        let relative_speed = (motion_a.velocity - motion_b.velocity).length();
        let collision_energy = 0.5 * total_mass * relative_speed.powf(2.0);

        commands.entity(entity_a).despawn();
        commands.entity(entity_b).despawn();

        if total_mass < 100.0 {
            continue;
        } else if collision_energy / mass_disparity < 10_000_000.0 {
            ev_spawn_body.send(SpawnBody {
                color: Color::Hsla {
                    saturation: 0.2 + 0.8 * rng.f32(),
                    lightness: 0.2 + 0.8 * rng.f32(),
                    hue: 360.0 * rng.f32(),
                    alpha: 1.0,
                },
                position: system_position,
                velocity: system_velocity,
                mass: total_mass,
            });
        } else {
            let n = rng.u8(4..=10);
            let mass = total_mass / f32::from(n);
            let exit_speed = relative_speed / f32::from(n).sqrt();

            for i in 0..n {
                let theta: f32 = (360.0 * f32::from(i)) / f32::from(n);

                let velocity = system_velocity
                    + exit_speed * Vec2::new(theta.cos() - theta.sin(), theta.sin() + theta.cos());

                let position = system_position
                    + threshold * Vec2::new(theta.cos() - theta.sin(), theta.sin() + theta.cos());

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
}
