use bevy::{prelude::{Draw, GlobalTransform, Handle, Mesh, Transform}, render::{render_graph::base::MainPass, pipeline::{PipelineSpecialization, RenderPipelines, RenderPipeline}}, ecs::Bundle};

use crate::chunk::{ChunkMaterial, ChunkUniform};

#[derive(Bundle)]
pub struct GeomBundle {
    pub mesh: Handle<Mesh>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub material: ChunkMaterial,
    pub uniform: Handle<ChunkUniform>,
}

impl Default for GeomBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                crate::PIPELINE_HANDLE,
                PipelineSpecialization::default(),
            )]),
            mesh: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            material: Default::default(),
            uniform: Default::default(),
        }
    }
}