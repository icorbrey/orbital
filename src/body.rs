use bevy::prelude::*;

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBody>()
            .add_systems(Update, (spawn_bodies, calculate_body_movement));
    }
}

#[derive(Component, Debug, Default)]
pub struct Body;

#[derive(Bundle, Debug, Default)]
pub struct BodyBundle {
    pub body: Body,
    pub mass: Mass,
    pub velocity: Velocity,
    pub transform: Transform,
}

#[derive(Component, Debug, Default)]
pub struct Mass(pub f32);

#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec2);

fn calculate_body_movement(mut bodies: Query<(&mut Transform, &Mass, &Velocity), With<Body>>) {
    let mut body_pairs = bodies.iter_combinations_mut();
    while let Some([mut a, mut b]) = body_pairs.fetch_next() {
        let (mut t_a, m_a, mut v_a) = a;
        let (mut t_b, m_b, mut v_b) = b;

        todo!("Finish gravity")
    }
}

#[derive(Event, Default)]
pub struct SpawnBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
    pub mass: f32,
}

fn spawn_bodies(mut ev_spawn_body: EventReader<SpawnBody>, mut commands: Commands) {
    for event in ev_spawn_body.read() {
        commands.spawn(BodyBundle {
            mass: Mass(event.mass),
            velocity: Velocity(event.velocity),
            transform: Transform::from_xyz(event.position.x, event.position.y, 0.0),
            ..default()
        });
    }
}
