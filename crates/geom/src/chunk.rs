use std::ops::Deref;

use bevy::{asset::{Assets, Handle}, core::Byteable, render::mesh::Indices, ecs::{ResMut, Query}, math::Vec4, render::{
        renderer::{RenderResources, RenderResource},
        camera::Camera,
        mesh::Mesh,
    }, transform::components::GlobalTransform, type_registry::TypeUuid};
use building_blocks::core::{Point3i, PointN};

pub const NUM_CUBE_VERTICES: usize = 8;
pub const NUM_CUBE_INDICES: usize = 3 * 3 * 2;

const CUBE_INDICIES: [u32; 36] = [
    0, 2, 1, 2, 3, 1,
    5, 4, 1, 1, 4, 0,
    0, 4, 6, 0, 6, 2,
    6, 5, 7, 6, 4, 5,
    2, 6, 3, 6, 7, 3,
    7, 1, 3, 7, 5, 1,
];

pub struct ChunkCoord(Point3i);

impl Deref for ChunkCoord {
    type Target = Point3i;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn chunk_indices(chunk_shape: Point3i) -> Vec<u32> {
    let max_voxels = chunk_shape.x() * chunk_shape.y() * chunk_shape.z();
    let num_chunk_indicies = max_voxels as usize * NUM_CUBE_INDICES;

    let indices: Vec<u32> = (0..num_chunk_indicies)
        .map(|i| {
            let cube = i / NUM_CUBE_INDICES;
            let cube_local = i % NUM_CUBE_INDICES;
            CUBE_INDICIES[cube_local] + cube as u32 * NUM_CUBE_VERTICES as u32
        }).collect();
    println!("amount indices: {}", indices.len());
    indices
}

#[derive(RenderResources, RenderResource, Default, TypeUuid)]
#[uuid = "d10816cf-cd32-404e-92dd-0f72400ecc4b"]
#[render_resources(from_self)]
pub struct ChunkUniform {
    pub voxel_size: Vec4,
    pub camera_pos: Vec4,
}

unsafe impl Byteable for ChunkUniform {}

#[derive(Debug)]
pub struct InstanceData {
    pub position: Vec4,
    pub color: Vec4,
}

unsafe impl Byteable for InstanceData {}

#[derive(RenderResources, RenderResource, Default, TypeUuid)]
#[uuid = "57dc3500-27af-41af-9c49-acfd87e66330"]
pub struct ChunkMaterial {
    #[render_resources(buffer)]
    pub instances: Vec<InstanceData>,
}

unsafe impl Byteable for ChunkMaterial {}

pub fn chunk_uniform_camera(
    mut chunk_uniforms: ResMut<Assets<ChunkUniform>>,
    query: Query<(&Camera, &GlobalTransform, &Handle<ChunkUniform>)>,
) {
    for (camera, transform, chunk_uniform_handle) in query.iter() {
        if let Some(name) = camera.name.as_ref() {
            if name == "Camera3d" {
                let chunk_uniform = chunk_uniforms.get_mut(chunk_uniform_handle).unwrap();
                chunk_uniform.camera_pos = transform.translation.extend(1.0); 
            }
        }
    }
}

pub struct ChunkMesh {
    pub shape: Point3i,
}

impl Default for ChunkMesh {
    fn default() -> Self {
        Self {
            shape: PointN([16; 3]),
        }
    }
}

impl From<ChunkMesh> for Mesh {
    fn from(chunk: ChunkMesh) -> Self {
        let mut mesh = Self::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        let indices = chunk_indices(chunk.shape);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}