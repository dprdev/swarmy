use avian2d::math::Vector;
use bevy::prelude::*;
use avian2d::prelude::*;
use crate::consts::*;
use crate::player::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Swarmling"), Health, Collider(swarmling_collider), RigidBody(swarmling_rigidbody))]
pub struct Swarmling{}

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
                Swarmling {},
                Sprite {
                    image: assets.load("sprites/bug/bug.png"),
                    ..default()
                },
                swarmling_transform,
                ExternalImpulse::new(Vector::new(0., 50000.))
            ));
        }
    }
}

pub fn swarmling_move(
    mut q_swarmling: Query<(&mut Transform, &mut LinearVelocity), (With<Swarmling>, Without<Player>)>,
    q_player: Query<(&Transform), (With<Player>, Without<Swarmling>)>
) {
    for (mut swarmling_transform, mut linear_velocity) in q_swarmling.iter_mut() {
        if let Ok(player_transform) = q_player.get_single() {
            let player_position = player_transform.translation;
            let direction = player_position - swarmling_transform.translation;
            let normalized_direction = direction.normalize_or_zero();
            let speed = 10.;
            linear_velocity.0 += normalized_direction.truncate() * speed;
        }
    }
}