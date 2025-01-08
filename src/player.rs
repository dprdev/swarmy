use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::window::PrimaryWindow;

use crate::consts::*;
use crate::input::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Player"), Health, Speed)]
pub struct Player;

pub fn player_move(
    mut q_player: Query<(&mut Transform), With<Player>>,
    mut evt_movement: EventReader<MovementAction>,
    time: Res<Time>,
) {
    for action in evt_movement.read() {
        match action {
            MovementAction::Move(direction) => {
                let mut transform = q_player.single_mut();
                transform.translation.x += 100.0 * direction.x * time.delta_secs();
                transform.translation.y += 100.0 * direction.y * time.delta_secs();
            }
        }
    }
}

pub fn player_aim(
    mut q_player: Query<&mut Transform, With<Player>>,
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
            let angle = direction.y.atan2(direction.x);
            // Apply the rotation to the player
            player_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

#[derive(Event)]
pub enum PlayerAction {
    Shoot
}

pub fn player_shoot(
    mut commands: Commands,
    assets: Res<AssetServer>,
    q_player: Query<&Transform, With<Player>>,
    ev_player_attack: EventReader<AttackAction>
) {
    for ev in ev_player_attack.read() {
        if (ev.button == MouseButton::Left) &&  (ev.state == ButtonState::Pressed) {
            let transform = q_player.single();
            let direction = get_direction(transform.rotation.to_euler(EulerRot::XYZ).2
            );
            commands.spawn((
                Projectile {},
                Sprite {
                    image: assets.load("sprites/projectiles/missile.png"),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                    rotation: transform.rotation,
                    ..default()
                },
                Velocity(direction),
                Speed(5.0)
            ));
        }
    }
}

fn get_direction(z_rotation: f32) -> Vec2 {
    let aim_x = z_rotation.cos();
    let aim_y = z_rotation.sin();
    Vec2::new(aim_x, aim_y).normalize()
}