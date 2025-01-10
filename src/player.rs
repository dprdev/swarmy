use bevy::prelude::*;
use avian2d::prelude::*;
use bevy::window::PrimaryWindow;

use crate::consts::*;
use crate::input::*;
use crate::projectile::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Player"), Health, Collider(player_collider), RigidBody(player_rigidbody))]
pub struct Player;

fn player_collider() -> Collider {
    Collider::capsule(15.0, 20.0)
}

fn player_rigidbody() -> RigidBody {
    RigidBody::Kinematic
}

pub fn player_move(
    mut q_player: Query<&mut LinearVelocity, With<Player>>,
    mut player_movement_event_reader: EventReader<PlayerMovementEvent>,
    time: Res<Time>,
) {
    for event in player_movement_event_reader.read() {
        match event {
            PlayerMovementEvent::Move(direction) => {
                let mut linear_velocity = q_player.single_mut();
                linear_velocity.0 = *direction * time.delta_secs() * 10000.0;
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
            let aim_angle = direction.y.atan2(direction.x);
            player_transform.rotation = Quat::from_rotation_z(aim_angle);
        }
    }
}

pub fn player_attack(
    mut commands: Commands,
    assets: Res<AssetServer>,
    q_player: Query<&Transform, With<Player>>,
    mut player_attack_event_reader: EventReader<PlayerAttackEvent>
) {
    for event in player_attack_event_reader.read() {
        match event {
            PlayerAttackEvent::PrimaryFire => {
                let player_transform = q_player.single();
                let forward = player_transform.rotation * Vec3::X;
                let spawn_offset = forward * 50.;
                let projectile_translation = player_transform.translation + spawn_offset;
                commands.spawn((
                    Projectile::default(),
                    Sprite {
                        image: assets.load("sprites/projectiles/missile.png"),
                        ..default()
                    },
                    Transform {
                        translation: projectile_translation,
                        rotation: player_transform.rotation,
                        ..default()
                    }
                ));
            }
        }
    }
}