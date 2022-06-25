use bevy::prelude::{Plugin, WindowDescriptor};
use bevy::window::{PresentMode, WindowMode};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(WindowDescriptor {
            present_mode: PresentMode::Mailbox,
            mode: WindowMode::SizedFullscreen,
            title: "Unistone".to_string(),
            ..Default::default()
        });
    }
}
