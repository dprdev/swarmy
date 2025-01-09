use avian2d::prelude::*;
use avian2d::math::Scalar;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Projectile"), RigidBody, Collider(projectile_collider))]
pub struct Projectile{
    displacement: f32,
    range: f32,
    damage: f32
}

fn projectile_collider() -> Collider {
    Collider::rectangle(2., 2.)
}

pub fn projectile_move(
    mut q_projectile: Query<&mut Projectile>,
    time: Res<Time>,
) {
    for (mut projectile) in q_projectile.iter_mut() {
        //TODO
    }
}

pub fn projectile_despawn(
    mut commands: Commands,
    mut q_projectile: Query<Entity, With<Projectile>>
) {
    for (entity) in q_projectile.iter_mut() {
        //TODO
    }
}