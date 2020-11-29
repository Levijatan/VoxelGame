#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::single_match)]
use bevy::{input::system::exit_on_esc_system, math::Vec4, prelude::*};

const VOXEL_SIZE: f32 = 2.0;

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_uniforms: ResMut<Assets<geom::chunk::ChunkUniform>>,
) {
    let mesh = geom::chunk::ChunkMesh::default();

    let chunk_uniform = chunk_uniforms.add(geom::chunk::ChunkUniform{
        voxel_size: Vec4::new(VOXEL_SIZE, VOXEL_SIZE, VOXEL_SIZE, 1.0),
        camera_pos: Vec4::new(0.0, 0.0, 0.0, 1.0),
    });
    let test_amount = 1;
    let size = 16;
    let chunk_size = size as f32;
    let half_size = (chunk_size * test_amount as f32)/2.0;

    let mesh_handle =  meshes.add(mesh.into());

    for xc in 0..test_amount {
        for zc in 0..test_amount {
            let mut instances = Vec::new();
            for x in 0..size {
                for y in 0..size  {
                    for z in 0..size  {
                        if rand::random::<bool>() {
                            instances.push(geom::chunk::InstanceData{
                                position: Vec4::new(x as f32 * VOXEL_SIZE, y as f32 * VOXEL_SIZE, z as f32 * VOXEL_SIZE, 1.0),
                                color: Vec4::new(0.0, 0.0, 0.0, 0.0),
                            });
                        }
                    }
                }
            }
            let chunk_material = geom::chunk::ChunkMaterial{
                instances
            };

            commands
                .spawn(geom::entity::GeomBundle {
                    mesh: mesh_handle.clone(),
                    material: chunk_material,
                    uniform: chunk_uniform.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        (xc as f32).mul_add(chunk_size * VOXEL_SIZE, -half_size * VOXEL_SIZE),
                        -chunk_size * VOXEL_SIZE,
                        (zc as f32).mul_add(chunk_size * VOXEL_SIZE, -half_size * VOXEL_SIZE))
                    ),
                    ..Default::default()
                });
        }
    }

    

    let camera = control::camera::CameraBundle::default();

    


    commands
        .spawn(camera)
        .with_bundle((chunk_uniform, control::camera::MainTag{}));
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(control::input::InputPlugin)
        .add_plugin(geom::GeomPlugin)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .run();
}

