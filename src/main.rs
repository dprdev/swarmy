use bevy::prelude::*;
use avian2d::prelude::*;

mod consts;
mod player;
mod projectile;
mod input;
mod camera;

use consts::*;
use player::*;
use projectile::*;
use input::*;
use camera::*;

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
            PhysicsPlugins::default()
        ))
        .init_state::<AppState>()
        .register_type::<(Health, Projectile, Building)>()
        .add_event::<PlayerMovementEvent>()
        .add_event::<PlayerAttackEvent>()
        .add_event::<CameraEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_zoom, keyboard_input, mouse_input, mouse_wheel_input))
        .add_systems(FixedUpdate, (player_aim, player_move, player_attack, projectile_move, projectile_cleanup).chain())
        .add_systems(PostProcessCollisions, print_collisions)
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
        Player,
        Sprite {
            image: assets.load("sprites/astronaut/astronaut.png"),
            ..default()
        }
    ));
    commands.spawn((
        Building,
        Name::new("Building".to_string()),
        Sprite {
            image: assets.load("sprites/hatchery/hatchery.png"),
            ..default()
        },
        Transform::from_xyz(0., 200., 0.),
        RigidBody::Static,
        Collider::rectangle(84., 84.)
    ));
}

fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {} and {} are colliding",
            contacts.entity1,
            contacts.entity2,
     );
    }
}
