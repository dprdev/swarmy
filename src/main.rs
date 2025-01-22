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

use consts::*;
use player::*;
use projectile::*;
use input::*;
use camera::*;
use swarm::*;

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
        .add_systems(Startup, (setup, setup_effect))
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
    assets: Res<AssetServer>
    ) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player{},
        Sprite {
            image: assets.load("sprites/astronaut/astronaut.png"),
            ..default()
        },
        Health(1.),
        CollidingEntities::default()
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

fn setup_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands
) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::X),
        radius: module.lit(2.),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocityCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::X),
        speed: module.lit(6.),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(10.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -3., 0.));
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        Spawner::rate(5.0.into()),
        // Move the expression module into the asset
        module
    )
        .with_name("TestEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        // Render the particles with a color gradient over their
        // lifetime. This maps the gradient key 0 to the particle spawn
        // time, and the gradient key 1 to the particle death (10s).
        .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands
        .spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(Vec3::Y),
            ..Default::default()
        });
}