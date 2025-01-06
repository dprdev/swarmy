use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseButtonInput;
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
        .register_type::<Health>()
        .add_systems(Startup, setup)
        .add_systems(Update, (player_aim, player_shoot))
        .add_systems(FixedUpdate, player_move)
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
#[require(Sprite, Name(|| "Projectile"), Speed)]
pub struct Projectile;

fn player_shoot(
    mut commands: Commands,
    mut mouse_btn_event: EventReader<MouseButtonInput>,
    assets: Res<AssetServer>,
    q_player: Query<&Transform, With<Player>>
) {
    for ev in mouse_btn_event.read() {
        let transform = q_player.single();
        if (ev.button == MouseButton::Left) &&  (ev.state == ButtonState::Pressed) {
            commands.spawn((
                Projectile,
                Sprite {
                    image: assets.load("sprites/projectiles/missile.png"),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                    rotation: transform.rotation,
                    ..default()
                },
                Speed(5.0)
            ));
        }
    }
}