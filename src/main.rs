use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::{MouseButtonInput, MouseWheel, MouseScrollUnit};
use bevy::input::ButtonState;

mod consts;
use consts::AppState;

#[cfg(debug_assertions)]
mod debug;

#[cfg(debug_assertions)]
use debug::DebugPlugin;

fn main() {
    let mut app = App::new();
    let _ = &app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("ProjectAlpha"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<AppState>()
        .register_type::<(Health, Projectile, Speed, Velocity, Displacement, Range)>()
        .add_systems(Startup, setup)
        .add_systems(Update, (player_aim, player_shoot, camera_zoom))
        .add_systems(FixedUpdate, (player_move, projectile_move, projectile_despawn))
        ;

    #[cfg(debug_assertions)] // debug/dev builds only
    {
        let _ = &app.add_plugins(DebugPlugin);
    }

    let _ = &app.run();
}

#[derive(Component, Reflect)]
pub struct Health(f32);

impl Default for Health {
    fn default() -> Self {
        Health(100.0)
    }
}

#[derive(Component, Reflect)]
pub struct Velocity(Vec2);

#[derive(Component, Reflect)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self { Speed(1.0)}
}

#[derive(Component, Reflect)]
#[require(Sprite, Name(|| "Player"), Health, Speed)]
pub struct Player;

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
        Speed(2.0)
    ));
}

fn player_move(
    mut q_player: Query<(&mut Speed, &mut Transform), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (speed, mut transform) = q_player.single_mut();

    if keys.pressed(KeyCode::KeyW) {
        // W is being held down
        transform.translation.y += 100.0 * time.delta_secs() * speed.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        // W is being held down
        transform.translation.x += 100.0 * time.delta_secs() * speed.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        // W is being held down
        transform.translation.y -= 100.0 * time.delta_secs() * speed.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        // W is being held down
        transform.translation.x -= 100.0 * time.delta_secs() * speed.0;
    }
}

fn player_aim(
    mut q_player: Query<&mut Transform, With<Player>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>
) {
    let window = q_window.single();

    if let Some(cursor_position) = window.cursor_position() {
        let (camera, camera_global_transform) = q_camera.single();
        if let Ok(cursor_world_position) = camera.viewport_to_world_2d(camera_global_transform, cursor_position) {
            let mut player_transform = q_player.single_mut();
            // Calculate the angle between the player and the cursor
            let direction = cursor_world_position - player_transform.translation.truncate();
            let angle = direction.y.atan2(direction.x);
            // Apply the rotation to the player
            player_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

#[derive(Component, Reflect)]
#[require(Sprite, Speed, Displacement, Range, Name(|| "Projectile"))]
pub struct Projectile{}

#[derive(Component, Reflect)]
pub struct Displacement(f32);

impl Default for Displacement {
    fn default() -> Self { Displacement(0.0) }
}

#[derive(Component, Reflect)]
pub struct Range(f32);

impl Default for Range {
    fn default() -> Self { Range(10.0) }
}

#[derive(Component, Reflect)]
pub struct Damage(f32);

fn player_shoot(
    mut commands: Commands,
    mut mouse_btn_event: EventReader<MouseButtonInput>,
    assets: Res<AssetServer>,
    q_player: Query<&Transform, With<Player>>
) {
    for ev in mouse_btn_event.read() {
        if (ev.button == MouseButton::Left) &&  (ev.state == ButtonState::Pressed) {
            let transform = q_player.single();
            let direction = get_direction(transform.rotation.to_euler(EulerRot::XYZ).2
            );
            commands.spawn((
                Projectile {},
                Sprite {
                    image: assets.load("sprites/projectiles/missile.png"),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                    rotation: transform.rotation,
                    ..default()
                },
                Velocity(direction),
                Speed(5.0)
            ));
        }
    }
}

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

fn get_direction(z_rotation: f32) -> Vec2 {
    let aim_x = z_rotation.cos();
    let aim_y = z_rotation.sin();
    Vec2::new(aim_x, aim_y).normalize()
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

fn camera_zoom(
    mut mouse_scroll_event: EventReader<MouseWheel>,
    mut q_camera: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for event in mouse_scroll_event.read() {
        let mut projection = q_camera.single_mut();
        match event.unit {
            MouseScrollUnit::Line => {
                projection.scale += event.y;
            }
            MouseScrollUnit::Pixel => {
                projection.scale += (event.y/100.0);
            }
        }
    }
}
