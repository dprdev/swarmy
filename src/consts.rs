use bevy::prelude::*;

//Global constants
pub const MIN_CAMERA_ZOOM: f32 = 0.5;
pub const MAX_CAMERA_ZOOM: f32 = 5.0;
pub const CAMERA_PAN_SPEED: f32 = 1.0;
pub const DEFAULT_PROJECTILE_SPEED: f32 = 500.0;
pub const DEFAULT_SPAWN_TIMER: f32 = 1.0;
pub const SWARMLING_COLLISION_DAMAGE: f32 = 1.0;

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