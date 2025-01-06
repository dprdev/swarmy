use bevy::prelude::{Reflect, States};

#[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq, Reflect)]
pub enum AppState {
    #[default]
    Start
}