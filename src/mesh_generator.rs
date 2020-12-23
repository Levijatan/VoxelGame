use bevy::ecs::Entity;

#[derive(Default)]
pub struct MeshGeneratorState {
    current_shape_index: i32,
    chunk_mesh_entites: Vec<Entity>,
}

