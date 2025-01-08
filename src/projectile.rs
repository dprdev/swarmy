use bevy::prelude::{Commands, Component, Entity, Query, Reflect, Res, Time, Transform, With};
use crate::{Displacement, Range, Speed, Velocity};

#[derive(Component, Reflect)]
#[require(Sprite, Speed, Displacement, Range, Name(|| "Projectile"))]
pub struct Projectile{}

fn projectile_move(
    mut q_projectile: Query<(&mut Transform, &mut Displacement, &Velocity, &Speed), With<Projectile>>,
    time: Res<Time>,
) {
    for (mut transform, mut displacement, velocity, speed) in q_projectile.iter_mut() {
        transform.translation.x += 100.0 * velocity.0.x * speed.0 * time.delta_secs();
        transform.translation.y += 100.0 * velocity.0.y * speed.0 * time.delta_secs();
        displacement.0 += speed.0 * time.delta_secs();
    }
}

fn projectile_despawn(
    mut commands: Commands,
    mut q_projectile: Query<(Entity, &Displacement, &Range), With<Projectile>>
) {
    for (entity, displacement, range) in q_projectile.iter_mut() {
        if displacement.0 > range.0 {
            commands.entity(entity).despawn();
        }
    }
}