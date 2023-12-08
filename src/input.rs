// pub mod prelude{

// };
use crate::prelude::*;
use bevy::input::{
    keyboard::KeyboardInput,
    mouse::{MouseButtonInput, MouseMotion},
};
pub mod events;
pub mod gamepad;
pub mod keyboard;
pub mod mapper;
pub mod mouse;
pub mod state;
use crate::input::{
    events::*,
    gamepad::GamepadAxis,
    mapper::{InputMapper, InputMapperPlugin},
    mouse::MouseAxis,
};
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum InputState {
    #[default]
    Menu,
    Person,
    Ship,
}
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputMapperPlugin::<InputState>::new())
            .add_state::<InputState>()
            .add_systems(Startup, bind_keys)
            .add_systems(Update, handle_input);
    }
}
fn bind_keys(mut im: ResMut<InputMapper<InputState>>) {
    // On default Scenario, pressing Space or Gamepad South triggers jump action.
    im.bind_keyboard_key_press(InputState::Person, KeyCode::Space, "jump")
        .bind_gamepad_button_press(InputState::Person, GamepadButtonType::South, "jump")
        // On swimming Scenario/State, pressing Space or Gamepad South triggers swim_up action.
        // Here we bind gamepad's right stick and mouse movements to camera.
        .bind_gamepad_axis_move(
            InputState::Person,
            GamepadAxis::NegativeRightStickX,
            "look_left",
        )
        .bind_gamepad_axis_move(
            InputState::Person,
            GamepadAxis::PositiveRightStickX,
            "look_right",
        )
        .bind_gamepad_axis_move(
            InputState::Person,
            GamepadAxis::NegativeRightStickY,
            "look_down",
        )
        .bind_gamepad_axis_move(
            InputState::Person,
            GamepadAxis::PositiveRightStickY,
            "look_up",
        )
        .bind_mouse_axis_move(InputState::Person, MouseAxis::NegativeX, "look_left")
        .bind_mouse_axis_move(InputState::Person, MouseAxis::PositiveX, "look_right")
        .bind_mouse_axis_move(InputState::Person, MouseAxis::PositiveY, "look_down")
        .bind_mouse_axis_move(InputState::Person, MouseAxis::NegativeY, "look_up")
        .bind_keyboard_key_press(InputState::Menu, KeyCode::Escape, "pause");
}
fn handle_input(
    mut action_active: EventReader<InputActionActive>,
    mut action_started: EventReader<InputActionStarted>,
    mut action_continuing: EventReader<InputActionContinuing>,
    mut action_finished: EventReader<InputActionFinished>,
) {
    for ev in action_active.read() {
        info!("Action Active: {}, {}", ev.0, ev.1);
    }
    for ev in action_started.read() {
        info!("Action Started: {}, {}", ev.0, ev.1);
    }
    for ev in action_continuing.read() {
        info!("Action Continuing: {}, {}", ev.0, ev.1);
    }
    for ev in action_finished.read() {
        info!("Action Finished: {}", ev.0);
    }
}
