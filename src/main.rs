use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_hanabi::prelude::*;
use std::time::Duration;

mod consts;
mod player;
mod projectile;
mod input;
mod camera;
mod swarm;
mod particles;

use consts::*;
use player::*;
use projectile::*;
use input::*;
use camera::*;
use swarm::*;
use particles::*;

#[cfg(debug_assertions)]
mod debug;

#[cfg(debug_assertions)]
use debug::DebugPlugin;

fn main() {
    let mut app = App::new();
    let _ = &app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Swarmy"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()
        )))
        .add_plugins(HanabiPlugin)
        .init_state::<AppState>()
        .register_type::<(Health, Projectile, SwarmSpawner, Player)>()
        .add_event::<PlayerMovementEvent>()
        .add_event::<PlayerAttackEvent>()
        .add_event::<CameraEvent>()
        .add_event::<PlayerDamageEvent>()
        .add_event::<PlayerDeathEvent>()
        .add_systems(Startup, (setup_particle_effects, setup).chain())
        .add_systems(Update, (
            camera_zoom, keyboard_input, mouse_input,
            mouse_wheel_input, swarmling_spawn))
        .add_systems(FixedUpdate, (
            player_aim, player_move, player_dash, player_attack,
            projectile_move, projectile_collision, swarmling_move,
            player_collision, player_take_damage, player_death, health_cleanup
        ).chain())
        .add_systems(PostUpdate, camera_follow.before(TransformSystem::TransformPropagate))
        .insert_resource(Gravity(Vec2::ZERO))
        ;

    #[cfg(debug_assertions)] // debug/dev builds only
    {
        let _ = &app.add_plugins(DebugPlugin);
    }

    let _ = &app.run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    player_dash_effect: Res<PlayerDashEffect>
    ) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player{},
        Sprite {
            image: assets.load("sprites/astronaut/astronaut.png"),
            ..default()
        },
        Health(1.),
        CollidingEntities::default(),
        ParticleEffectBundle::new(player_dash_effect.handle.clone()),
    ));
    commands.spawn((
        SwarmSpawner{
            spawn_timer: Timer::new(Duration::from_secs(SWARMLING_SPAWN_TIMER as u64), TimerMode::Repeating)
        },
        Sprite {
            image: assets.load("sprites/hatchery/hatchery.png"),
            ..default()
        },
        Transform::from_xyz(0., 200., 0.),
        RigidBody::Static,
        Collider::rectangle(84., 84.),
        Health(10.)
    ));
}

fn health_cleanup(
    mut commands: Commands,
    q_health: Query<(Entity, &Health)>,
) {
    for (e, health) in q_health.iter() {
        if health.0 < 0.1 {
            commands.entity(e).despawn();
        }
    }
}