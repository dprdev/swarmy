use bevy::math::Vec2;
use bevy::prelude::{Component, Event, Reflect, States};

//Global constants
pub const MIN_CAMERA_ZOOM: f32 = 0.5;
pub const MAX_CAMERA_ZOOM: f32 = 5.0;

//Custom Components
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

#[derive(Component, Reflect)]
pub struct Building;