use std::f32::consts::PI;

use crate::prelude::*;

pub struct SpawnBodyPlugin;
impl Plugin for SpawnBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBodyEvent>().add_systems(
            Update,
            handle_spawn_body_event.run_if(on_event::<SpawnBodyEvent>()),
        );
    }
}

#[derive(Event, Default)]
pub(crate) struct SpawnBodyEvent {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
    pub mass: f32,
}

pub(crate) const DENSITY: f32 = 0.551; // g/cm^3

fn handle_spawn_body_event(
    mut ev_spawn_body: EventReader<SpawnBodyEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for event in ev_spawn_body.read() {
        let transform = Transform::from_translation(event.position.extend(event.mass / -10000.0));
        let radius = ((3.0 * event.mass / DENSITY) / (4.0 * PI)).cbrt();
        let mesh = Mesh2dHandle(meshes.add(Circle { radius }));
        let material = materials.add(event.color);

        commands.spawn((
            Name::new("Body"),
            cleanup::OnExit,
            Body,
            Motion {
                velocity: event.velocity.extend(0.0),
                mass: event.mass,
                radius,
                ..default()
            },
            MaterialMesh2dBundle {
                transform,
                material,
                mesh,
                ..default()
            },
        ));
    }
}
