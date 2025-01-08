use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
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
            },
            );
        }
    }
}
