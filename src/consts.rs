use bevy::math::Vec2;
use bevy::prelude::{Component, Event, Reflect, States};

//Global constants
pub const MIN_CAMERA_ZOOM: f32 = 0.5;
pub const MAX_CAMERA_ZOOM: f32 = 5.0;

#[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq, Reflect)]
pub enum AppState {
    #[default]
    Start
}

//Custom Components
#[derive(Component, Reflect)]
pub struct Health(f32);

impl Default for Health {
    fn default() -> Self {
        Health(100.0)
    }
}

#[derive(Component, Reflect)]
pub struct Damage(f32);

#[derive(Component, Reflect)]
pub struct Building;