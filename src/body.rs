use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::motion::Motion;

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBody>()
            .add_systems(Update, spawn_bodies);
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
        commands.spawn((
            Body,
            Motion {
                velocity: event.velocity.extend(0.0),
                mass: event.mass,
                ..default()
            },
            MaterialMesh2dBundle {
                transform: Transform::from_xyz(event.position.x, event.position.y, -event.mass),
                material: materials.add(event.color),
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: event.mass / 10.0,
                })),
                ..default()
            },
        ));
    }
}
