use avian2d::math::Scalar;
use bevy::input::{ButtonInput, ButtonState};
use bevy::input::mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel};
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Event)]
pub enum PlayerMovementAction {
    Move(Vec2),
}

#[derive(Event)]
pub enum PlayerAttackAction {
    PrimaryFire,
}

#[derive(Event)]
pub enum CameraAction {
    Zoom(f32),
}

/// Sends [`PlayerMovementAction`] events based on keyboard input.
pub fn keyboard_input(
    mut movement_event_writer: EventWriter<PlayerMovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.is_changed() {
        let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
        let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);

        let horizontal = right as i8 - left as i8;
        let vertical = up as i8 - down as i8;
        let direction = Vec2::new(horizontal as Scalar, vertical as Scalar);
        info!("Moved!");
        movement_event_writer.send(PlayerMovementAction::Move(direction));
    }
}

pub fn mouse_input(
    mut mouse_btn_event: EventReader<MouseButtonInput>,
    mut attack_event_writer: EventWriter<PlayerAttackAction>
) {
    for event in mouse_btn_event.read() {
        if event.state == ButtonState::Pressed {
            match event.button {
                MouseButton::Left => {attack_event_writer.send(PlayerAttackAction::PrimaryFire);},
                _ => continue
            }
        }
    }
}

pub fn mouse_wheel_input(
    mut mouse_wheel_event: EventReader<MouseWheel>,
    mut camera_event_writer: EventWriter<CameraAction>
) {
    for event in mouse_wheel_event.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                camera_event_writer.send(CameraAction::Zoom(event.y));
            },
            MouseScrollUnit::Pixel => {
                camera_event_writer.send(CameraAction::Zoom(event.y/100.));
            }
        }
    }
}