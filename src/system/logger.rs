use bevy::log::{Level, LogSettings};
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::Plugin};

pub struct Logger;

impl Plugin for Logger {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut diag = LogDiagnosticsPlugin::default();
        diag.debug = true;

        app.insert_resource(LogSettings {
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,libracity=trace".to_string(),
        })
        .add_plugin(diag);
    }
}
