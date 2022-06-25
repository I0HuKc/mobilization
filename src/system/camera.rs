use bevy::math::{Mat3, Quat, Vec2, Vec3};
use bevy::prelude::{
    App, Commands, Component, EventReader, MouseButton, Mut, PerspectiveCameraBundle, Plugin,
    Query, SystemSet, Transform,
};
use bevy::render::camera::PerspectiveProjection;
use bevy::{
    input::{
        mouse::{MouseMotion, MouseWheel},
        Input,
    },
    prelude::Res,
    window::Windows,
};
use lazy_static::lazy_static;

lazy_static! {
    /// Максимально допустимый масштаб
    static ref MAX_ZOOM: f32 = 3.0;

    /// Минимально допустимый масштаб
    static ref MIN_ZOOM: f32 = 15.0;

    static ref MIN_DELTA_Y: f32 = 0.0;
}

/// Создание сущности способную панорамировать и вращаться по орбите
#[derive(Component)]
struct PanOrbitCamera {
    /// «Точка фокуса», вокруг которой нужно вращаться.
    /// Автоматически обновляется при панорамировании камеры
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

pub struct Camera;

impl Camera {
    pub fn spawn(mut commands: Commands) {
        let translation = Vec3::new(-2.0, 2.5, 5.0);
        let radius = translation.length();

        commands
            .spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            })
            .insert(PanOrbitCamera {
                radius,
                ..Default::default()
            });
    }

    /// Панорамирования камеры
    /// Орбитальное вращение -> ЛКМ
    /// Перемещение -> ПКМ
    /// Масштабирование -> СКМ
    fn pan_orbit(
        windows: Res<Windows>,
        mut ev_motion: EventReader<MouseMotion>,
        mut ev_scroll: EventReader<MouseWheel>,
        input_mouse: Res<Input<MouseButton>>,
        mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
    ) {
        // Изменение входного сопоставление для орбиты и панорамирования
        let orbit_button = MouseButton::Right;
        let pan_button = MouseButton::Left;

        let mut pan = Vec2::ZERO;
        let mut rotation_move = Vec2::ZERO;
        let mut scroll = 0.0;
        let mut orbit_button_changed = false;

        if input_mouse.pressed(orbit_button) {
            for ev in ev_motion.iter() {
                rotation_move += ev.delta;
            }
        } else if input_mouse.pressed(pan_button) {
            // Панорамировать, только если нет вращения в данный момент
            for ev in ev_motion.iter() {
                pan += ev.delta;
            }
        }

        for ev in ev_scroll.iter() {
            scroll += ev.y;
        }

        if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
            orbit_button_changed = true;
        }

        for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
            // Проверка только на перевернутость, когда орбита началась или закончилась в этом кадре
            if orbit_button_changed {
                let up = transform.rotation * Vec3::Y;
                println!("{} > 0.8 = {}", up.y, up.y > 0.8);

                if up.y > 0.8 {
                    println!("1");
                    break;
                }

                pan_orbit.upside_down = up.y <= 0.0;
            }

            let mut any = false;
            if rotation_move.length_squared() > 0.0 {
                any = true;
                let window = window_size(&windows);
                let delta_x = {
                    let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                    if pan_orbit.upside_down {
                        -delta
                    } else {
                        delta
                    }
                };

                let delta_y = rotation_move.y / window.y * std::f32::consts::PI;

                // Отклонение
                let yaw = Quat::from_rotation_y(-delta_x);
                // Высота
                let pitch = Quat::from_rotation_x(-delta_y);

                // Вращаться вокруг глобальной оси Y
                transform.rotation = yaw * transform.rotation;
                // Вращаться вокруг глобальной оси X
                transform.rotation = transform.rotation * pitch;
            } else if pan.length_squared() > 0.0 {
                any = true;
                // Cделать расстояние панорамирования независимым от разрешения и FOV
                let window = window_size(&windows);
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
                // Перевести по локальным осям
                let right = transform.rotation * Vec3::X * -pan.x;
                let up = transform.rotation * Vec3::Y * pan.y;
                // Сделать панорамирование пропорциональным расстоянию от точки фокусировки
                let translation = (right + up) * pan_orbit.radius;
                pan_orbit.focus += translation;
            } else if scroll.abs() > 0.0 // Масштабирование
                && sor(scroll, pan_orbit.radius) > *MAX_ZOOM
                && sor(scroll, pan_orbit.radius) < *MIN_ZOOM
            {
                any = true;
                pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
                pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
            }

            if any {
                let rot_matrix = Mat3::from_quat(transform.rotation);
                transform.translation =
                    pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
            }
        }
    }
}

fn sor(scroll: f32, radius: f32) -> f32 {
    let radius = radius - (scroll * radius * 0.2);
    f32::max(radius, 0.05)
}

fn window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

impl Plugin for Camera {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label(super::Systems::Camera)
                .with_system(Camera::pan_orbit),
        );
    }
}
