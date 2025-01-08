use avian2d::math::Scalar;
use bevy::input::{ButtonInput, ButtonState};
use bevy::input::mouse::MouseButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Event, EventReader, EventWriter, KeyCode, MouseButton, Res};

#[derive(Event)]
pub enum MovementAction {
    Move(Vec2),
}

#[derive(Event)]
pub enum AttackAction {
    PrimaryFire,
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vec2::new(horizontal as Scalar, vertical as Scalar);

    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }
}

fn mouse_input(
    mut mouse_btn_event: EventReader<MouseButtonInput>,
    mut attack_event_writer: EventWriter<AttackAction>
) {
    for event in mouse_btn_event.read() {
        if event.state == ButtonState::Pressed {
            match event.button {
                MouseButton::Left => {attack_event_writer.send(AttackAction::PrimaryFire);},
                _ => continue
            }
        }
    }
}