use bevy::prelude::{Plugin, WindowDescriptor};

pub struct Window;

impl Plugin for Window {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(WindowDescriptor {
            title: "Voxland".to_string(),
            ..Default::default()
        });
    }
}
