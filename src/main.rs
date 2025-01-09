use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::{MouseButtonInput, MouseWheel, MouseScrollUnit};
use bevy::input::ButtonState;
use avian2d::{math::*, prelude::*};

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
        .add_event::<PlayerMovementAction>()
        .add_event::<PlayerAttackAction>()
        .add_event::<CameraAction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (player_attack, camera_zoom, keyboard_input, mouse_input, mouse_wheel_input))
        .add_systems(FixedUpdate, (player_move, player_aim, projectile_move, projectile_despawn, print_collisions))
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
        Sprite {
            image: assets.load("sprites/hatchery/hatchery.png"),
            ..default()
        },
        Transform::from_xyz(0., 200., 0.),
        RigidBody::Static,
        Collider::rectangle(200., 200.)
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
