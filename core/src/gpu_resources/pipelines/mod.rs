use bevy_ecs::world::World;

pub mod n_body_sim_compute_pipeline;
pub mod render_particles_pipeline;
pub mod unlit_diffuse_pipeline;

pub fn initialize_pipelines(world: &mut World) {
    let unlit_diffuse_pipeline = unlit_diffuse_pipeline::UnlitDiffusePipeline::new(world);
    let n_body_sim_compute_pipeline =
        n_body_sim_compute_pipeline::NBodySimComputePipeline::new(world);
    let render_particles_pipeline = render_particles_pipeline::RenderParticlesPipeline::new(world);

    world.insert_resource(unlit_diffuse_pipeline);
    world.insert_resource(n_body_sim_compute_pipeline);
    world.insert_resource(render_particles_pipeline);
}
