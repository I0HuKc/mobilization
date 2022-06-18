mod resource;
mod system;

use bevy::prelude::*;
use bevy_config_cam::*;

use crate::resource::world::{Kind, World};

fn main() {
    let mut app = App::new();

    app.add_plugin(system::logger::Logger)
        .add_plugin(system::window::Window);

    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cl: ResMut<CamLogic>,
) {
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..Default::default()
    // });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // cube, set as target
    // cl.target = Some(
    //     commands
    //         .spawn_bundle(PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //             material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //             transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //             ..Default::default()
    //         })
    //         .id(),
    // );

    // plane

    World::gen(Kind::Basic, 5, commands, meshes, materials);
}
