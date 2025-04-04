use bevy_ecs::system::{Query, Res, ResMut};

use crate::{
    ecs::resources::{
        nbody_sim_resources::NBodySimResources, screen_parameters::ScreenParameters, time::Time,
    },
    gpu_resources::render_resources::RenderResources,
};

pub fn update_n_body_sim_bindings(
    render_resources: Res<RenderResources>,
    time: Res<Time>,
    mut n_body_sim_resources: ResMut<NBodySimResources>,
) {
    n_body_sim_resources.set_frame_count(time.frame_count as u32);
    n_body_sim_resources.set_delta_time(&render_resources.queue, time.delta_time);
    n_body_sim_resources.reset_indirect_buffer(&render_resources.queue);
}
