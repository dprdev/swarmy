use avian2d::math::Vector;
use bevy::prelude::*;
use avian2d::prelude::*;
use bevy::math::ops::sin;
use crate::consts::*;
use crate::player::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Swarmling"), Health, Collider(swarmling_collider), RigidBody(swarmling_rigidbody))]
pub struct Swarmling{
    state: BehaviorState
}

impl Default for Swarmling{
    fn default() -> Self {
        Swarmling {
            state: BehaviorState::default()
        }
    }
}

#[derive(Reflect, Default)]
enum BehaviorState{
    WanderAlone,
    WanderSwarm,
    FleeAlone,
    FleeSwarm,
    #[default]
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
                ExternalImpulse::new(Vector::new(0., 25000.))
            ));
        }
    }
}

pub fn swarmling_move(
    mut q_swarmling: Query<(&mut Transform, &mut LinearVelocity, &mut ExternalImpulse, &Swarmling), (Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<Swarmling>)>
) {
    let player_transform = q_player.get_single();
    for (swarmling_transform, mut linear_velocity, mut external_impulse, swarmling) in q_swarmling.iter_mut() {
        match swarmling.state {
            BehaviorState::WanderAlone => {
                match player_transform {
                    Ok(player_transform) => {},
                    _ => {}
                }
            },
            BehaviorState::FleeAlone => {},
            BehaviorState::AttackAlone => {
                if let Ok(player_transform) = q_player.get_single() {
                    let direction = player_transform.translation - swarmling_transform.translation;
                    let normalized_direction = direction.normalize_or_zero().truncate();
                    let speed = 10.;
                    linear_velocity.x += sin(normalized_direction.x) * speed;
                    linear_velocity.y += sin(normalized_direction.y) * speed;
                } else {
                    linear_velocity.x += sin(linear_velocity.x) * 50.;
                    linear_velocity.y += sin(linear_velocity.y) * 50.;
                };
            },
            _ => {}
        }
    }
}