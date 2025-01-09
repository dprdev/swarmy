use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::window::PrimaryWindow;
use avian2d::prelude::*;

use crate::consts::*;
use crate::input::*;
use crate::projectile::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Player"), Health, Collider(player_collider), RigidBody(player_rigidbody))]
pub struct Player;

fn player_collider() -> Collider {
    Collider::capsule(12.5, 20.0)
}

fn player_rigidbody() -> RigidBody {
    RigidBody::Kinematic
}

pub fn player_move(
    mut q_player: Query<(&mut LinearVelocity), With<Player>>,
    mut player_movement_event_reader: EventReader<PlayerMovementAction>,
    time: Res<Time>,
) {
    for event in player_movement_event_reader.read() {
        match event {
            PlayerMovementAction::Move(direction) => {
                let mut linear_velocity = q_player.single_mut();
                linear_velocity.0 = *direction * time.delta_secs() * 10000.0;
            }
        }
    }
}

pub fn player_aim(
    mut q_player: Query<(&mut Transform), With<Player>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>
) {
    let window = q_window.single();

    if let Some(cursor_position) = window.cursor_position() {
        let (camera, camera_global_transform) = q_camera.single();
        if let Ok(cursor_world_position) = camera.viewport_to_world_2d(camera_global_transform, cursor_position) {
            let mut player_transform = q_player.single_mut();
            // Calculate the angle between the player and the cursor
            let direction = cursor_world_position - player_transform.translation.truncate();
            let aim_angle = direction.y.atan2(direction.x);
            player_transform.rotation.z = aim_angle;
            info!("aim angle: {}", aim_angle);
        }
    }
}

pub fn player_attack(
    mut commands: Commands,
    assets: Res<AssetServer>,
    q_player: Query<&Transform, With<Player>>,
    mut player_attack_event_reader: EventReader<PlayerAttackAction>
) {
    for event in player_attack_event_reader.read() {
        match event {
            PlayerAttackAction::PrimaryFire => {
                //test
            }
        }
    }
}

fn get_direction(z_rotation: f32) -> Vec2 {
    let aim_x = z_rotation.cos();
    let aim_y = z_rotation.sin();
    Vec2::new(aim_x, aim_y).normalize()
}

fn floats_are_equal(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}