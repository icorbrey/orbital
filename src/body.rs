use bevy::{
    app::Plugin,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBody>().add_systems(Update, spawn_body);
    }
}

#[derive(Event)]
pub struct SpawnBody {
    pub position: Vec2,
}

fn spawn_body(
    mut ev_spawn_body: EventReader<SpawnBody>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for event in ev_spawn_body.read() {
        commands.spawn(MaterialMesh2dBundle {
            transform: Transform::from_xyz(event.position.x, event.position.y, 0.0),
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 25.0 })),
            material: materials.add(Color::WHITE),
            ..default()
        });
    }
}
