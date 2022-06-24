use bevy::input::{
    mouse::{MouseScrollUnit, MouseWheel},
    Input,
};
use bevy::prelude::{
    App, Camera as BevyCamera, EventReader, KeyCode, Plugin, Query, ResMut, Transform, With,
};
use lazy_static::lazy_static;

lazy_static! {
    /// Максимально допустимый масштаб
    static ref MAX_ZOOM_VALUE: f32 = 1.5;

    /// Минимально допустимый масштаб
    static ref MIN_ZOOM_VALUE: f32 = 0.5;

    /// Единица масштабирования
    static ref ZOOM_STEP: f32 = 0.08;
}

lazy_static! {
    static ref MOOVE_STEP: f32 = 0.05;
}

pub struct CameraPlugin;

impl CameraPlugin {
    /// Масштабирование камеры отосительно игрового поля
    fn zoom(
        mut scroll_evr: EventReader<MouseWheel>,
        mut camera: Query<&mut Transform, With<BevyCamera>>,
    ) {
        for ev in scroll_evr.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    for mut transform in camera.iter_mut() {
                        if ev.y > 0.0 && transform.scale.z <= *MAX_ZOOM_VALUE {
                            transform.scale.z += *ZOOM_STEP;
                        }

                        if ev.y < 0.0 && transform.scale.z >= *MIN_ZOOM_VALUE {
                            transform.scale.z -= *ZOOM_STEP;
                        }
                    }
                }

                MouseScrollUnit::Pixel => (),
            }
        }
    }

    /// Передвижения камеры с использованием клавиатуры
    fn movement_by_keyboard(
        mut camera: Query<&mut Transform, With<BevyCamera>>,
        keys: ResMut<Input<KeyCode>>,
    ) {
        for mut transform in camera.iter_mut() {
            if keys.pressed(KeyCode::Up) {
                transform.translation.z -= *MOOVE_STEP;
            }

            if keys.pressed(KeyCode::Down) {
                transform.translation.z += *MOOVE_STEP;
            }

            if keys.pressed(KeyCode::Right) {
                transform.translation.x += *MOOVE_STEP;
            }

            if keys.pressed(KeyCode::Left) {
                transform.translation.x -= *MOOVE_STEP;
            }
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(CameraPlugin::zoom);
        app.add_system(CameraPlugin::movement_by_keyboard);
    }
}
