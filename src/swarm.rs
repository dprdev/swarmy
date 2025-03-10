use crate::consts::*;
use crate::particles::*;
use crate::player::*;
use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_hanabi::{EffectInitializers, ParticleEffect, ParticleEffectBundle};
use fastnoise_lite::{FastNoiseLite, NoiseType};

#[derive(Resource)]
pub struct SwarmNoise {
    pub value: FastNoiseLite,
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
pub struct Swarmling {
    state: BehaviorState,
    path_noise: FastNoiseLite,
}

impl Default for Swarmling {
    fn default() -> Self {
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(NoiseType::Perlin));
        noise.set_frequency(Some(0.2));
        noise.seed = fastrand::i32(0..100000);
        Swarmling {
            state: BehaviorState::default(),
            path_noise: noise,
        }
    }
}

#[derive(Reflect, Default)]
enum BehaviorState {
    #[default]
    Wander,
    Flee,
    Attack,
}

#[derive(Component, Reflect)]
pub struct SwarmSpawner {
    pub spawn_timer: Timer,
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
    swarmling_death_effect: Res<SwarmlingDeathEffect>,
    assets: Res<AssetServer>,
    time: Res<Time>,
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
                ExternalImpulse::new(Vector::new(0., SWARMLING_LAUNCH_SPEED)),
                ParticleEffectBundle {
                    effect: ParticleEffect::new(swarmling_death_effect.handle.clone()),
                    transform: swarmling_transform, // Use your transform here
                    ..default()
                },
            ));
        }
    }
}

pub fn swarmling_move(
    mut q_swarmling: Query<
        (
            &mut ExternalImpulse,
            &Swarmling,
        ),
        Without<Player>,
    >,
    q_player: Query<&Transform, (With<Player>, Without<Swarmling>)>,
    time: Res<Time>,
) {
    if let Ok(_player_transform) = q_player.get_single() {
        for (mut external_impulse, swarmling) in
            q_swarmling.iter_mut()
        {
            match swarmling.state {
                BehaviorState::Wander => {
                    external_impulse.x +=
                        swarmling.path_noise.get_noise_2d(time.elapsed_secs(), 0.0)
                            * SWARMLING_SPEED;
                    external_impulse.y +=
                        swarmling.path_noise.get_noise_2d(0.0, time.elapsed_secs())
                            * SWARMLING_SPEED;
                }
                BehaviorState::Flee => {}
                BehaviorState::Attack => {}
            }
        }
    }
}

pub fn swarmling_behavior_control(
    mut q_swarmling: Query<&mut Swarmling, Changed<Health>>,
    q_player: Query<&Transform, With<Player>>,
) {
    let player_transform = q_player.get_single().unwrap();
    for swarmling in q_swarmling.iter_mut() {
        //TODO
    }
}

pub fn swarmling_death(
    mut q_swarmling: Query<
        (Entity, &Health, &mut EffectInitializers),
        (With<Swarmling>, Changed<Health>),
    >,
    mut commands: Commands,
) {
    for (entity, health, mut initializers) in q_swarmling.iter_mut() {
        if health.0 <= 0.1 {
            initializers.reset();
        }
    }
}
