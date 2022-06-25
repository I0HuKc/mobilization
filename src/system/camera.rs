use bevy::prelude::{
    App, Camera as BevyCamera, EventReader, KeyCode, Plugin, Query, ResMut, SystemSet, Transform,
    With,
};
use bevy::{
    input::{
        mouse::{MouseScrollUnit, MouseWheel},
        Input,
    },
    prelude::Res,
    window::Windows,
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
    /// Обычная скорость передвижения камеры
    static ref MOOVE_SPEED: f32 = 0.05;

    /// Ускореннопе передвижение камеры
    /// Если курсор находится близко к краю экрана, скорость движения камеры увеличивается
    static ref ACCELERATED_MOOVE_SPEED: f32 = 0.08;
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
    fn keyboard_test(
        mut camera: Query<&mut Transform, With<BevyCamera>>,
        keys: ResMut<Input<KeyCode>>,
    ) {
        for mut transform in camera.iter_mut() {
            if keys.pressed(KeyCode::Up) {
                transform.translation.z -= *MOOVE_SPEED;
            }

            if keys.pressed(KeyCode::Down) {
                transform.translation.z += *MOOVE_SPEED;
            }

            if keys.pressed(KeyCode::Right) {
                transform.translation.x += *MOOVE_SPEED;
            }

            if keys.pressed(KeyCode::Left) {
                transform.translation.x -= *MOOVE_SPEED;
            }
        }
    }

    fn movement(mut camera: Query<&mut Transform, With<BevyCamera>>, windows: Res<Windows>) {
        let window = windows.get_primary().unwrap();

        if let Some(position) = window.cursor_position() {
            for mut transform in camera.iter_mut() {
                println!("X - {}, Y - {}", position.x, position.y);
                match position {
                    // Движение влево
                    pos if pos.x < 100 as f32 => {
                        transform.translation.x -= *MOOVE_SPEED;
                    }

                    // Движение вправо
                    pos if pos.x > window.width() - 100 as f32 => {
                        transform.translation.x += *MOOVE_SPEED;
                    }

                    // Движение вниз
                    pos if pos.y < 50 as f32 => match pos.y {
                        y if y < 25 as f32 => transform.translation.z += *ACCELERATED_MOOVE_SPEED,

                        _ => transform.translation.z += *MOOVE_SPEED,
                    },

                    // Движение вверх
                    pos if pos.y > window.height() - 50 as f32 => match pos.y {
                        y if y > window.height() - 25 as f32 => {
                            transform.translation.z -= *ACCELERATED_MOOVE_SPEED
                        }

                        _ => transform.translation.z -= *MOOVE_SPEED,
                    },

                    _ => (),
                }
            }
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label(super::Systems::Camera)
                .with_system(CameraPlugin::zoom)
                .with_system(CameraPlugin::movement)
                .with_system(CameraPlugin::keyboard_test),
        );
    }
}
