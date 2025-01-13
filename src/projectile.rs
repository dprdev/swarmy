use avian2d::prelude::*;
use avian2d::math::Scalar;
use bevy::prelude::*;
use crate::consts::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Projectile"), RigidBody(projectile_rigidbody), Collider(projectile_collider), CollidingEntities)]
pub struct Projectile{
    displacement: Scalar,
    range: Scalar,
    damage: Scalar,
    speed: Scalar,
}

impl Default for Projectile {
    fn default() -> Projectile {
        Projectile {
            displacement: 0.0,
            range: 500.0,
            damage: 1.0,
            speed: DEFAULT_PROJECTILE_SPEED,
        }
    }
}

fn projectile_rigidbody() -> RigidBody {
    RigidBody::Kinematic
}

fn projectile_collider() -> Collider {
    Collider::rectangle(2., 2.)
}

pub fn projectile_move(
    mut commands: Commands,
    mut q_projectile: Query<(Entity, &Transform, &mut LinearVelocity, &mut Projectile)>,
    time: Res<Time>,
) {
    for (entity, projectile_transform, mut velocity, mut projectile) in q_projectile.iter_mut() {
        let forward = projectile_transform.rotation * Vec3::X;
        velocity.0 = forward.truncate() * projectile.speed;
        projectile.displacement += velocity.length() * time.delta_secs();
        if projectile.displacement > projectile.range {
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_collision(
    mut commands: Commands,
    q_projectile: Query<(Entity, &Projectile, &CollidingEntities)>,
    mut q_health: Query<&mut Health, With<Collider>>
) {
    for (projectile_entity, projectile, colliding_entities) in q_projectile.iter() {
        if projectile.displacement > projectile.range {
            commands.entity(projectile_entity).despawn();
            continue;
        }
        if !colliding_entities.is_empty() {
            for colliding_entity in colliding_entities.iter() {
                if let Ok(mut health) = q_health.get_mut(*colliding_entity) {
                    health.0 -= projectile.damage;
                }
            }
            commands.entity(projectile_entity).despawn();
        }
    }
}