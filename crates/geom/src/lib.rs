use bevy::{
    asset::{Assets, Handle, AddAsset},
    app::{Plugin, AppBuilder},
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{RenderGraph, AssetRenderResourcesNode, base, RenderResourcesNode},
        shader::{Shader, ShaderStages, ShaderStage},
    },
    type_registry::TypeUuid,
    ecs::ResMut,
};

pub mod chunk;
pub mod entity;
pub mod map;

pub const PIPELINE_HANDLE: Handle<PipelineDescriptor>
    = Handle::weak_from_u64(PipelineDescriptor::TYPE_UUID, 17626092015219607069);

const VERTEX_SHADER: &str = include_str!("./shaders/chunk.vert");
const FRAGMENT_SHADER: &str = include_str!("./shaders/chunk.frag");


pub struct GeomPlugin;

impl Plugin for GeomPlugin
{
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<chunk::ChunkUniform>()
            .add_asset::<map::ChunkMap3U32>()
            .add_startup_system(setup)
            .add_system(chunk::chunk_uniform_camera);
    }
}

fn setup(
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    render_graph.add_system_node("chunk_uniform", AssetRenderResourcesNode::<chunk::ChunkUniform>::new(false));
    render_graph.add_node_edge("chunk_uniform", base::node::MAIN_PASS).unwrap();

    render_graph.add_system_node("chunk_material", RenderResourcesNode::<chunk::ChunkMaterial>::new(false));
    render_graph.add_node_edge("chunk_material", base::node::MAIN_PASS).unwrap();

    let mut pipeline = PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    });
    
    if let Some(state) = pipeline.rasterization_state.as_mut() {
        state.cull_mode = bevy::render::pipeline::CullMode::None;
    }
    pipelines.set_untracked(
        PIPELINE_HANDLE,
        pipeline,
    )
}