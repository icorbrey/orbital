use bevy::{app::Plugin, input::mouse::MouseButtonInput, prelude::*};

pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<InputStore>()
            .add_event::<InputOccured>()
            .add_systems(Update, intercept_mouse);
    }
}

#[derive(Event)]
pub enum InputOccured {
    Move { position: Vec2 },
    Click { position: Vec2, button: MouseButton },
}

#[derive(Default, Resource)]
pub struct InputStore {
    pub position: Vec2,
}

fn intercept_mouse(
    mut ev_mouse_button_input: EventReader<MouseButtonInput>,
    mut ev_input_occured: EventWriter<InputOccured>,
    mut ev_cursor_moved: EventReader<CursorMoved>,
    mut input_store: ResMut<InputStore>,
) {
    for event in ev_mouse_button_input.read() {
        ev_input_occured.send(InputOccured::Click {
            position: input_store.position,
            button: event.button,
        });
    }

    for event in ev_cursor_moved.read() {
        input_store.position = event.position;
        ev_input_occured.send(InputOccured::Move {
            position: event.position,
        });
    }
}
