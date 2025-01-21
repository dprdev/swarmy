use avian2d::math::Vector;
use bevy::prelude::*;
use avian2d::prelude::*;
use bevy::ecs::query::QuerySingleError;
use bevy::math::ops::sin;
use fastnoise_lite::{FastNoiseLite, NoiseType};
use crate::consts::*;
use crate::player::*;

#[derive(Resource)]
pub struct SwarmNoise {
    pub value: FastNoiseLite
}

impl Default for SwarmNoise {
    fn default() -> Self {
        SwarmNoise {
            value: FastNoiseLite::new(),
        }
    }
}

#[derive(Component)]
#[require(Sprite, Name(|| "Swarmling"), Health, Collider(swarmling_collider), RigidBody(swarmling_rigidbody))]
pub struct Swarmling{
    state: BehaviorState,
    path_noise: FastNoiseLite
}

impl Default for Swarmling{
    fn default() -> Self {
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(NoiseType::Perlin));
        noise.set_frequency(Some(0.2));
        noise.seed = fastrand::i32(0..100000);
        Swarmling {
            state: BehaviorState::default(),
            path_noise: noise
        }
    }
}

#[derive(Reflect, Default)]
enum BehaviorState{
    #[default]
    WanderAlone,
    WanderSwarm,
    FleeAlone,
    FleeSwarm,
    AttackAlone,
    AttackSwarm,
}

#[derive(Component, Reflect)]
#[require(Name(|| "SwarmSpawner"))]
pub struct SwarmSpawner{
    pub spawn_timer: Timer
}

fn swarmling_collider() -> Collider {
    Collider::circle(5.)
}

fn swarmling_rigidbody() -> RigidBody {
    RigidBody::Dynamic
}

pub fn swarmling_spawn(
    mut commands: Commands,
    mut q_swarm_spawner: Query<(&mut SwarmSpawner, &Transform)>,
    assets: Res<AssetServer>,
    time: Res<Time>
) {
    for (mut spawner, transform) in q_swarm_spawner.iter_mut() {
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            let mut swarmling_transform = transform.clone();
            swarmling_transform.translation.y += 55.;
            commands.spawn((
                Swarmling::default(),
                Sprite {
                    image: assets.load("sprites/bug/bug.png"),
                    ..default()
                },
                swarmling_transform,
                ExternalImpulse::new(Vector::new(0., SWARMLING_LAUNCH_SPEED))
            ));
        }
    }
}

pub fn swarmling_move(
    mut q_swarmling: Query<(&mut Transform, &mut LinearVelocity, &mut ExternalImpulse, &Swarmling), (Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<Swarmling>)>,
    time: Res<Time>
) {
    let player_transform = q_player.get_single();
    for (swarmling_transform, mut linear_velocity, mut external_impulse, swarmling) in q_swarmling.iter_mut() {
        match swarmling.state {
            BehaviorState::WanderAlone => {
                match player_transform {
                    Ok(player_transform) => {
                        external_impulse.x += swarmling.path_noise.get_noise_2d(time.elapsed_secs(), 0.0) * SWARMLING_SPEED;
                        external_impulse.y += swarmling.path_noise.get_noise_2d(0.0, time.elapsed_secs()) * SWARMLING_SPEED;
                    }
                    Err(_) => {}
                }
            },
            BehaviorState::FleeAlone => {},
            BehaviorState::AttackAlone => {},
            BehaviorState::AttackSwarm => {
                if let Ok(player_transform) = q_player.get_single() {
                    let direction = player_transform.translation - swarmling_transform.translation;
                    let normalized_direction = direction.normalize_or_zero().truncate();
                    linear_velocity.x += sin(normalized_direction.x) * SWARMLING_SPEED;
                    linear_velocity.y += sin(normalized_direction.y) * SWARMLING_SPEED;
                } else {
                    linear_velocity.x += sin(linear_velocity.x) * 50.;
                    linear_velocity.y += sin(linear_velocity.y) * 50.;
                };
            }
            _ => {}
        }
    }
}