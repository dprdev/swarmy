use avian2d::prelude::*;
use avian2d::math::Scalar;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Projectile"), RigidBody, Collider(projectile_collider))]
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
            range: 10.0,
            damage: 1.0,
            speed: 5.0,
        }
    }
}

fn projectile_collider() -> Collider {
    Collider::rectangle(2., 2.)
}

pub fn projectile_move(
    mut q_projectile: Query<& Transform, &mut Projectile>,
    time: Res<Time>,
) {
    for (mut projectile) in q_projectile.iter_mut() {
        let direction = get_direction_from_rotation(projectile.rotation.z);
        //TODO
}

pub fn projectile_despawn(
    mut commands: Commands,
    mut q_projectile: Query<Entity, With<Projectile>>
) {
    for (entity) in q_projectile.iter_mut() {
        //TODO
    }
}

fn get_direction_from_rotation(z_rotation: f32) -> Vec2 {
    let aim_x = z_rotation.cos();
    let aim_y = z_rotation.sin();
    Vec2::new(aim_x, aim_y).normalize()
}