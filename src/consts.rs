use bevy::prelude::*;

//Global constants
pub const CAMERA_ZOOM_MIN: f32 = 0.5;
pub const CAMERA_ZOOM_MAX: f32 = 5.0;
pub const PROJECTILE_SPEED: f32 = 500.0;
pub const SWARMLING_SPAWN_TIMER: f32 = 1.0;
pub const SWARMLING_COLLISION_DAMAGE: f32 = 1.0;
pub const SWARMLING_SPEED: f32 = 50.0;
pub const SWARMLING_LAUNCH_SPEED: f32 = 1500.0;
pub const PLAYER_MOVEMENT_SPEED: f32 = 100.;
pub const PLAYER_DASH_SPEED: f32 = 1000.;
pub const PLAYER_DASH_DURATION: f32 = 0.5;
pub const PLAYER_DASH_COOLDOWN: f32 = 1.0;

#[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq, Reflect)]
pub enum AppState {
    #[default]
    Start
}

//Custom Components
#[derive(Component, Reflect, Debug)]
#[require(Name(|| "Health"))]
pub struct Health(pub f32);

impl Default for Health {
    fn default() -> Self {
        Health(2.0)
    }
}