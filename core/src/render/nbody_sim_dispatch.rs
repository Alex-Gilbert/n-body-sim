use bevy_ecs::system::{Res, SystemState};

use crate::{
    ecs::resources::nbody_sim_resources::NBodySimResources,
    gpu_resources::{
        layouts::nbody_simparams_uniform_layout::NBodySimParamsUniformLayout,
        pipelines::n_body_sim_compute_pipeline::NBodySimComputePipeline,
        render_resources::RenderResources,
    },
};

type NBodySimDispatcherSystemState = SystemState<(
    Res<'static, NBodySimResources>,
    Res<'static, NBodySimComputePipeline>,
)>;

pub struct NBodySimDispatcher {
    pub system_state: NBodySimDispatcherSystemState,
}

impl NBodySimDispatcher {
    pub fn new(world: &mut bevy_ecs::world::World) -> Self {
        Self {
            system_state: SystemState::new(world),
        }
    }

    pub fn dispatch<'a, 'w>(
        &mut self,
        world: &'w bevy_ecs::world::World,
        compute_pass: &mut wgpu::ComputePass<'a>,
    ) where
        'w: 'a,
    {
        let (nbody_sim_resources, nbody_sim_compute_pipeline) = self.system_state.get(world);
        let (nbody_sim_resources, nbody_sim_compute_pipeline) = (
            nbody_sim_resources.into_inner(),
            nbody_sim_compute_pipeline.into_inner(),
        );

        let particle_count = nbody_sim_resources.get_particle_count();
        let dispatch_size = (particle_count + 63) / 64;
        compute_pass.set_pipeline(&nbody_sim_compute_pipeline.compute_pipeline);
        compute_pass.set_bind_group(0, nbody_sim_resources.get_bind_group(), &[]);

        compute_pass.dispatch_workgroups(dispatch_size, 1, 1);
    }
}
