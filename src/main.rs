use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::{MouseButtonInput, MouseWheel, MouseScrollUnit};
use bevy::input::ButtonState;
use avian2d::{math::*, prelude::*};


mod consts;
mod player;
mod projectile;
mod input;
use consts::*;
use player::*;
use projectile::*;
use input::*;

#[cfg(debug_assertions)]
mod debug;

#[cfg(debug_assertions)]
use debug::DebugPlugin;

#[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq, Reflect)]
pub enum AppState {
    #[default]
    Start
}

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
        .register_type::<(Health, Projectile, Speed, Velocity, Displacement, Range, Building)>()
        .add_systems(Startup, setup)
        .add_systems(Update, (player_aim, player_shoot, camera_zoom))
        .add_systems(FixedUpdate, (player_move, projectile_move, projectile_despawn, print_collisions))
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
        },
        RigidBody::Kinematic,
        Collider::capsule(12.5, 20.0),
        Speed(2.0),
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

fn camera_zoom(
    mut mouse_scroll_event: EventReader<MouseWheel>,
    mut q_camera: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for event in mouse_scroll_event.read() {
        let mut projection = q_camera.single_mut();
        match event.unit {
            MouseScrollUnit::Line => {
                projection.scale -= event.y;
            }
            MouseScrollUnit::Pixel => {
                projection.scale -= event.y/100.0;
            }
        }
        projection.scale = projection.scale.clamp(MIN_CAMERA_ZOOM, MAX_CAMERA_ZOOM);
    }
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
