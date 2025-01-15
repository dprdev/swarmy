use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use avian2d::prelude::*;
use crate::consts::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_plugins(StateInspectorPlugin::<AppState>::default());
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 42.0,
                    font: default(),
                    font_smoothing:FontSmoothing::default(),
                },
                text_color: Color::srgb(1.0, 0.0, 0.0),
                enabled: true,
            },
        });
        app.add_plugins(PhysicsDebugPlugin::default());
        // app.add_systems(PostProcessCollisions, print_collisions);
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
