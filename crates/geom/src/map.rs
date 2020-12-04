use bevy::{asset::Handle, ecs::Bundle, render::mesh::Mesh, reflect::TypeUuid};
use building_blocks::{prelude::Chunk3, core::Extent3i, core::Point3i, prelude::ChunkMap3};

pub trait MapType{
    fn type_name(&self) -> &'static str;
    fn generate_chunk(&self, key: &Point3i, extent: &Extent3i) -> Chunk3<u32, ()>;
}

pub struct Active;

#[derive(TypeUuid)]
#[uuid = "2ffcf7e9-c318-484f-9c33-366da0605bc8"]
pub struct ChunkMap3U32(ChunkMap3<u32>);

#[derive(Bundle)]
pub struct MapBundle {
    pub storage: Handle<ChunkMap3U32>,
    pub name: &'static str,
    pub chunk_mesh: Handle<Mesh>,
    pub chunk_shape: Point3i,
    pub m_type: Handle<Box<dyn MapType>>,
}