use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_hanabi::prelude::*;
use bevy::window::PrimaryWindow;

use crate::consts::*;
use crate::DashParticleEffect;
use crate::input::*;
use crate::projectile::*;
use crate::swarm::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Player"), Health, Collider(player_collider), RigidBody(player_rigidbody), Dash)]
pub struct Player {}

#[derive(Component, Reflect)]
pub struct Dash {
    is_dashing: bool,
    direction: Vec2,
    duration: f32,
    elapsed: f32,
    cooldown: f32,
}

impl Default for Dash {
    fn default() -> Self {
        Dash {
            is_dashing: false,
            direction: Vec2::ZERO,
            duration: PLAYER_DASH_DURATION,
            elapsed: 0.0,
            cooldown: PLAYER_DASH_COOLDOWN
        }
    }
}

#[derive(Event)]
pub enum PlayerDamageEvent {
    Collision(f32),
}

#[derive(Event)]
pub enum PlayerDeathEvent {
    Death,
}

fn player_collider() -> Collider {
    Collider::capsule(15.0, 20.0)
}

fn player_rigidbody() -> RigidBody {
    RigidBody::Kinematic
}

pub fn player_move(
    mut q_player: Query<(&mut LinearVelocity, &mut Dash),With<Player>>,
    mut player_movement_event_reader: EventReader<PlayerMovementEvent>,
    time: Res<Time>,
) {
    for event in player_movement_event_reader.read() {
        match event {
            PlayerMovementEvent::Move(direction) => {
                if let Ok((mut linear_velocity, _)) = q_player.get_single_mut() {
                    linear_velocity.0 = *direction * PLAYER_MOVEMENT_SPEED;
                }
            },
            PlayerMovementEvent::Dash(direction) => {
                if let Ok((_, mut dash))  = q_player.get_single_mut() {
                    if dash.cooldown <= 0.0 {
                        dash.direction = *direction;
                        dash.is_dashing = true;
                    }
                }
            }
        }
    }
}

pub fn player_dash(
    mut commands: Commands,
    mut q_player: Query<(&mut LinearVelocity, &mut Dash, &mut EffectInitializers), With<Player>>,
    time: Res<Time>
) {
    for (mut linear_velocity, mut dash, mut e_initializers) in q_player.get_single_mut() {
        if dash.is_dashing {
            dash.elapsed += time.delta_secs();
            if dash.elapsed > dash.duration {
                dash.is_dashing = false;
                dash.direction = Vec2::ZERO;
                dash.elapsed = 0.0;
                dash.cooldown = PLAYER_DASH_COOLDOWN;
                e_initializers.reset();
                continue;
            }
            let progress = dash.elapsed / dash.duration;
            let curve = EasingCurve::new(
                dash.direction * PLAYER_DASH_SPEED, Vec2::ZERO, EaseFunction::SineOut);
            let vel_2d = curve.sample(progress).unwrap();
            linear_velocity.x = vel_2d.x;
            linear_velocity.y = vel_2d.y;
        } else if dash.cooldown > 0.0 {
            dash.cooldown -= time.delta_secs();
            dash.cooldown = dash.cooldown.clamp(0.0, PLAYER_DASH_COOLDOWN);
        }
    }
}

pub fn player_aim(
    mut q_player: Query<&mut Transform, With<Player>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>
) {
    if let Ok(window) = q_window.get_single() {
        if let Some(cursor_position) = window.cursor_position() {
            let (camera, camera_global_transform) = q_camera.single();
            if let Ok(cursor_world_position) = camera.viewport_to_world_2d(camera_global_transform, cursor_position) {
                if let Ok(mut player_transform) = q_player.get_single_mut() {
                    // Calculate the angle between the player and the cursor
                    let direction = cursor_world_position - player_transform.translation.truncate();
                    let aim_angle = direction.y.atan2(direction.x);
                    player_transform.rotation = Quat::from_rotation_z(aim_angle);
                }
            }
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
                if let Ok(player_transform) = q_player.get_single() {
                    let forward = player_transform.rotation * Vec3::X;
                    let spawn_offset = forward * 25.;
                    let projectile_translation = player_transform.translation + spawn_offset;
                    commands.spawn((
                        Projectile::default(),
                        Sprite {
                            image: assets.load("sprites/projectiles/star_tiny.png"),
                            ..default()
                        },
                        Transform {
                            translation: projectile_translation,
                            rotation: player_transform.rotation,
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

pub fn player_collision(
    q_player: Query<&CollidingEntities, With<Player>>,
    mut q_swarmling: Query<Entity, With<Swarmling>>,
    mut player_damage_event_writer: EventWriter<PlayerDamageEvent>
) {
    if let Ok(colliding_entities) = q_player.get_single() {
        for entity in colliding_entities.iter() {
            if let Ok(mut health) = q_swarmling.get_mut(*entity) {
                player_damage_event_writer.send(PlayerDamageEvent::Collision(SWARMLING_COLLISION_DAMAGE));
            }
        }
    }
}

pub fn player_take_damage(
    mut q_player: Query<&mut Health, With<Player>>,
    mut player_damage_event_reader: EventReader<PlayerDamageEvent>,
    mut player_death_event_writer: EventWriter<PlayerDeathEvent>
) {
    for event in player_damage_event_reader.read() {
        match event {
            PlayerDamageEvent::Collision(damage) => {
                if let Ok(mut player_health) = q_player.get_single_mut() {
                    player_health.0 -= damage;
                    info!("Player health: {}", player_health.0);
                    if player_health.0 <= 0.1 {
                        player_death_event_writer.send(PlayerDeathEvent::Death);
                    }
                }
            }
        }
    }
}

pub fn player_death(
    mut player_death_event_reader: EventReader<PlayerDeathEvent>
) {
    for event in player_death_event_reader.read() {
        info!("PLAYER IS DEAD, YOU NOOB");
    }
}
