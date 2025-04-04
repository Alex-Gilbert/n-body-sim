use bevy_ecs::system::{Res, SystemState};

use crate::{
    ecs::resources::nbody_sim_resources::NBodySimResources,
    gpu_resources::pipelines::render_particles_pipeline::RenderParticlesPipeline,
};

type NBodySimRendererSystemState = SystemState<(
    Res<'static, NBodySimResources>,
    Res<'static, RenderParticlesPipeline>,
)>;

pub struct NBodySimRenderer {
    pub system_state: NBodySimRendererSystemState,
}

impl NBodySimRenderer {
    pub fn new(world: &mut bevy_ecs::world::World) -> Self {
        Self {
            system_state: SystemState::new(world),
        }
    }

    pub fn render<'a, 'w>(
        &mut self,
        world: &'w bevy_ecs::world::World,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) where
        'w: 'a,
    {
        let (nbody_sim_resources, render_particles_pipeline) = self.system_state.get(world);
        let (nbody_sim_resources, render_particles_pipeline) = (
            nbody_sim_resources.into_inner(),
            render_particles_pipeline.into_inner(),
        );

        render_pass.set_pipeline(&render_particles_pipeline.render_pipeline);
        render_pass.set_bind_group(1, nbody_sim_resources.get_material_bind_group(), &[]);

        render_pass.set_vertex_buffer(0, nbody_sim_resources.get_vertex_buffer().slice());
        render_pass.set_vertex_buffer(1, nbody_sim_resources.get_instance_buffer().slice());
        render_pass.set_index_buffer(
            nbody_sim_resources.get_index_buffer().slice(),
            wgpu::IndexFormat::Uint32,
        );

        render_pass.draw_indexed_indirect(&nbody_sim_resources.get_indirect_buffer().buffer, 0);
    }
}
