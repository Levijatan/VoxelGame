#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::single_match)]

use bevy::{input::system::exit_on_esc_system, math::Vec4, prelude::*};

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_uniforms: ResMut<Assets<geom::chunk::ChunkUniform>>,
    asset_server: Res<AssetServer>,
) {
    let test_amount = 1;
    let size = 16;
    let chunk_size = size as f32;
    let voxel_size = 1.0 / chunk_size;
    let voxel_diag = 0.5 * voxel_size * chunk_size;
    let half_size = chunk_size/2.0;

    let mesh = geom::chunk::ChunkMesh::new(size);

    let texture_handle = asset_server.load("cube-normal.png");

    let chunk_uniform = chunk_uniforms.add(geom::chunk::ChunkUniform{
        voxel_size: Vec4::new(voxel_size, voxel_size, voxel_size, 1.0),
        camera_pos: Vec4::new(0.0, 0.0, 0.0, 1.0),
        center_to_edge: Vec4::new(voxel_diag, voxel_diag, voxel_diag, 1.0),
        voxel_texture: texture_handle,
    });


    let mesh_handle =  meshes.add(mesh.into());

    for xc in 0..test_amount {
        for zc in 0..test_amount {
            let mut instances = Vec::new();
            for x in 8..size {
                for y in 8..size  {
                    for z in 8..size  {
                            instances.push(geom::chunk::InstanceData{
                                position: Vec4::new(x as f32, y as f32, z as f32, 1.0),
                                color: Color::GREEN,
                            });
                    }
                }
            }
            let chunk_material = geom::chunk::ChunkInstances{
                instances,
            };

            commands
                .spawn(geom::entity::GeomBundle {
                    mesh: mesh_handle.clone(),
                    instances: chunk_material,
                    uniform: chunk_uniform.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        (xc as f32).mul_add(chunk_size * voxel_size, -half_size * voxel_size),
                        -chunk_size * voxel_size,
                        (zc as f32).mul_add(chunk_size * voxel_size, -half_size * voxel_size))
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

#[bevy_main]
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(control::input::InputPlugin)
        .add_plugin(geom::GeomPlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

