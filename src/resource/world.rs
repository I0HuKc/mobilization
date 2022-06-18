use bevy::prelude::*;
use rand::Rng;
use std::rc::Rc;

use crate::resource::voxel::Voxel;

use super::voxel;

pub enum Kind {
    Basic,
    Flat,
}

pub struct World {
    size: usize,
    kind: Kind,
}

impl World {
    pub fn gen(
        kind: Kind,
        size: usize,

        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mut rng = rand::thread_rng();

        for x in 0..size as i8 {
            let x_pos = Voxel::calc_position(x, 0.0);

            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: voxel::VOXEL_SIZE,
                })),
                material: materials.add(Voxel::random_color(&mut rng).into()),
                transform: Transform::from_xyz(x_pos, 0.0, 0.0),
                ..Default::default()
            });

            for y in 0..size as i8 {
                let y_pos = Voxel::calc_position(-y, -voxel::VOXEL_SIZE);

                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube {
                        size: voxel::VOXEL_SIZE,
                    })),
                    material: materials.add(Voxel::random_color(&mut rng).into()),
                    transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                    ..Default::default()
                });

                for z in 0..size as i8 {
                    commands.spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube {
                            size: voxel::VOXEL_SIZE,
                        })),
                        material: materials.add(Voxel::random_color(&mut rng).into()),
                        transform: Transform::from_xyz(
                            x_pos,
                            y_pos,
                            Voxel::calc_position(z, voxel::VOXEL_SIZE),
                        ),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

mod chunk {
    pub const CHUNK_SIZE: u8 = 10;

    pub struct Chunk;

    impl Chunk {
        pub fn gen() {
            todo!()
        }
    }
}
