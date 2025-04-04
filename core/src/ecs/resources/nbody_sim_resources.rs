use bevy_ecs::{system::Resource, world::World};
use glam::{Vec3, Vec4};
use wgpu::BufferUsages;

use crate::{
    ecs::components::{
        materials::unlit_diffuse_material::UnlitDiffuseMaterial, mesh_filter::BasicMeshFilter,
    },
    gpu_resources::{
        layouts::nbody_simparams_uniform_layout::NBodySimParamsUniformLayout,
        render_resources::RenderResources,
        types::{
            basic_vertex::BasicVertex, gpu_indirect_args::GpuIndirectArgs,
            gpu_particle::GpuParticle, gpu_particle_instance::GpuParticleInstance,
            gpu_sim_params::GpuSimParams,
        },
    },
    utils::buffer::{Buffer, BufferBuilder},
};

#[derive(Resource)]
pub struct NBodySimResources {
    sim_params: GpuSimParams,

    particle_buffer_a: Buffer<GpuParticle>,
    particle_buffer_b: Buffer<GpuParticle>,
    sim_params_buffer: Buffer<GpuSimParams>,
    instance_buffer: Buffer<GpuParticleInstance>,
    indirect_buffer: Buffer<GpuIndirectArgs>,

    particle_mesh_filter: BasicMeshFilter,
    particle_material: UnlitDiffuseMaterial,

    // we need two bind groups for double buffering
    bind_group_a: wgpu::BindGroup,
    bind_group_b: wgpu::BindGroup,

    frame_count: u32,
}

impl NBodySimResources {
    pub fn new(
        world: &World,
        particle_mesh_filter: BasicMeshFilter,
        particle_material: UnlitDiffuseMaterial,
    ) -> Self {
        let num_particles = 10;

        let render_resources = world.get_resource::<RenderResources>().unwrap();
        let (device, queue) = &render_resources.get_device_queue();
        let nbody_bind_group_layout = world.get_resource::<NBodySimParamsUniformLayout>().unwrap();

        let sim_params = GpuSimParams::new(0.0, num_particles, 2.0);

        // Create a buffer for the particle data
        let min_mass = 0.1;
        let max_mass = 0.11;
        let min_velocity = 0.1;
        let max_velocity = 0.11;
        let dimensions = 10.0;
        let mut rng = rand::thread_rng();

        let first_big_particle = GpuParticle {
            position: Vec4::new(0.0, 0.0, 0.0, 500.0),
            velocity: Vec4::new(0.0, 0.0, 0.0, 0.0),
        };

        let random_partictes: Vec<GpuParticle> = (0..num_particles)
            .map(|i| {
                if i == 0 {
                    first_big_particle
                } else {
                    GpuParticle::new_random(
                        &mut rng,
                        dimensions,
                        min_mass,
                        max_mass,
                        min_velocity,
                        max_velocity,
                    )
                }
            })
            .collect();

        let particle_buffer_a = BufferBuilder::<GpuParticle>::new(device)
            .label("Particle Buffer Read")
            .usage(BufferUsages::STORAGE | BufferUsages::COPY_DST)
            .queue(queue)
            .contents(&random_partictes)
            .build()
            .unwrap();

        let particle_buffer_b = BufferBuilder::<GpuParticle>::new(device)
            .label("Particle Buffer Write")
            .size(num_particles as usize)
            .usage(BufferUsages::STORAGE | BufferUsages::COPY_DST)
            .build()
            .unwrap();

        let sim_params_buffer = BufferBuilder::<GpuSimParams>::new(device)
            .label("Sim Params Buffer")
            .usage(BufferUsages::UNIFORM | BufferUsages::COPY_DST)
            .contents(&[sim_params])
            .build()
            .unwrap();

        let instance_buffer = BufferBuilder::<GpuParticleInstance>::new(device)
            .label("Instance Buffer")
            .size(num_particles as usize)
            .usage(BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::VERTEX)
            .build()
            .unwrap();

        let indirect_buffer = BufferBuilder::<GpuIndirectArgs>::new(device)
            .label("Indirect Buffer")
            .usage(BufferUsages::STORAGE | BufferUsages::INDIRECT | BufferUsages::COPY_DST)
            .queue(queue)
            .contents(&[GpuIndirectArgs::new(
                particle_mesh_filter.filter.index_count,
                0,
            )])
            .build()
            .unwrap();

        let bind_group_a = nbody_bind_group_layout.create_bind_group(
            device,
            &particle_buffer_a,
            &particle_buffer_b,
            &sim_params_buffer,
            &instance_buffer,
            &indirect_buffer,
        );
        let bind_group_b = nbody_bind_group_layout.create_bind_group(
            device,
            &particle_buffer_b,
            &particle_buffer_a,
            &sim_params_buffer,
            &instance_buffer,
            &indirect_buffer,
        );

        Self {
            sim_params,
            particle_buffer_a,
            particle_buffer_b,
            sim_params_buffer,
            instance_buffer,
            indirect_buffer,

            bind_group_a,
            bind_group_b,

            particle_mesh_filter,
            particle_material,

            frame_count: 0,
        }
    }

    pub fn get_particle_count(&self) -> u32 {
        self.sim_params.num_particles
    }

    pub fn get_vertex_buffer(&self) -> &Buffer<BasicVertex> {
        &self.particle_mesh_filter.filter.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &Buffer<u32> {
        &self.particle_mesh_filter.filter.index_buffer
    }

    pub fn get_instance_buffer(&self) -> &Buffer<GpuParticleInstance> {
        &self.instance_buffer
    }

    pub fn get_indirect_buffer(&self) -> &Buffer<GpuIndirectArgs> {
        &self.indirect_buffer
    }

    pub fn get_material_bind_group(&self) -> &wgpu::BindGroup {
        &self.particle_material.bind_group
    }

    pub fn set_frame_count(&mut self, frame_count: u32) {
        self.frame_count = frame_count;
    }

    pub fn reset_indirect_buffer(&mut self, queue: &wgpu::Queue) {
        let new_idirect_args = &[GpuIndirectArgs::new(
            self.particle_mesh_filter.filter.index_count,
            0,
        )];

        self.indirect_buffer.update(queue, new_idirect_args, 0);
    }

    pub fn set_delta_time(&mut self, queue: &wgpu::Queue, delta_time: f32) {
        self.sim_params.delta_time = delta_time;
        self.sim_params_buffer.update(queue, &[self.sim_params], 0);
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        if self.frame_count % 2 == 0 {
            &self.bind_group_a
        } else {
            &self.bind_group_b
        }
    }
}
