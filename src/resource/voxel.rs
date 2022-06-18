use rand::{prelude::ThreadRng, Rng};

use bevy::{
    pbr::{PbrBundle, StandardMaterial},
    prelude::{shape, Assets, Color, Mesh, ResMut, Transform},
};

pub const VOXEL_SIZE: f32 = 0.3;

pub struct Voxel;

impl Voxel {
    pub(super) fn basic_voxel(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> PbrBundle {
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: VOXEL_SIZE })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        }
    }

    pub(super) fn calc_position(iter_pos: i8, default_pos: f32) -> f32 {
        if iter_pos != 0 {
            VOXEL_SIZE * (iter_pos as f32)
        } else {
            default_pos
        }
    }

    pub(super) fn random_color(rng: &mut ThreadRng) -> Color {
        Color::rgb(
            rng.gen_range(0.1..1.0),
            rng.gen_range(0.1..1.0),
            rng.gen_range(0.1..1.0),
        )
    }
}
