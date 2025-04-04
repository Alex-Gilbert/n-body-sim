use bevy_ecs::{system::Resource, world::World};

use crate::gpu_resources::{
    layouts::nbody_simparams_uniform_layout::NBodySimParamsUniformLayout,
    render_resources::RenderResources,
};

use super::super::shaders::n_body_sim_compute::SHADER_DESCRIPTOR_COMPUTE;
use super::super::shaders::n_body_sim_compute_workgroup::SHADER_DESCRIPTOR_COMPUTE as WORKGROUP_SHADER_DESCRIPTOR_COMPUTE;

#[derive(Resource)]
pub struct NBodySimComputePipeline {
    pub compute_pipeline: wgpu::ComputePipeline,
}

impl NBodySimComputePipeline {
    pub fn new(world: &World) -> Self {
        let render_resources = world.get_resource::<RenderResources>().unwrap();
        let device = &render_resources.device;

        let nbody_sim_params_uniform_layout =
            world.get_resource::<NBodySimParamsUniformLayout>().unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("unlit_diffuse_pipeline_layout"),
            bind_group_layouts: &[&nbody_sim_params_uniform_layout.layout],
            push_constant_ranges: &[],
        });

        // let compute_shader_module = device.create_shader_module(SHADER_DESCRIPTOR_COMPUTE);
        let compute_shader_module =
            device.create_shader_module(WORKGROUP_SHADER_DESCRIPTOR_COMPUTE);

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("n-body-sim-compute-pipeline"),
            entry_point: "cs_main",
            layout: Some(&pipeline_layout),
            module: &compute_shader_module,
            compilation_options: Default::default(),
        });

        Self { compute_pipeline }
    }
}
