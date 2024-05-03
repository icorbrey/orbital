mod body;
mod input;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use body::BodyPlugin;
use input::{InputHandlerPlugin, InputOccured};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InputHandlerPlugin, BodyPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, move_cursor)
        .run();
}

#[derive(Component)]
struct Cursor;

fn setup(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Cursor,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 25.0 })),
            material: materials.add(Color::PINK),
            ..default()
        },
    ));
}

fn move_cursor(
    mut ev_input_occured: EventReader<InputOccured>,
    mut cursor: Query<(&Cursor, &mut Transform)>,
    windows: Query<&Window>,
) {
    for event in ev_input_occured.read() {
        match event {
            InputOccured::Move { position } => {
                for window in windows.iter() {
                    let width = window.width();
                    let height = window.height();

                    let x = position.x - width / 2.0;
                    let y = -position.y + height / 2.0;

                    for (_, mut transform) in cursor.iter_mut() {
                        transform.translation.x = x;
                        transform.translation.y = y;
                    }
                }
            }
            InputOccured::Click { .. } => {}
        }
    }
}
