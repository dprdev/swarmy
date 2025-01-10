use avian2d::prelude::*;
use avian2d::math::Scalar;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Projectile"), RigidBody(projectile_rigidbody), Collider(projectile_collider))]
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
            speed: 1000.0,
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
    mut q_projectile: Query<(&Transform, &mut LinearVelocity, &mut Projectile)>,
    time: Res<Time>,
) {
    for (projectile_transform, mut velocity,mut projectile) in q_projectile.iter_mut() {
        let forward = projectile_transform.rotation * Vec3::X;
        velocity.0 = forward.truncate() * projectile.speed;
        projectile.displacement += velocity.length() * time.delta_secs();
    }
}

pub fn projectile_despawn(
    mut commands: Commands,
    q_projectile: Query<(Entity, &Projectile)>
) {
    for (entity, projectile) in q_projectile.iter() {
        if projectile.displacement > projectile.range {
            commands.entity(entity).despawn();
        }
    }
}