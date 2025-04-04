use crate::define_gpu_data_type;

define_gpu_data_type!(
    super::super::shaders::render_particles::naga::types::ParticleInstance as GpuParticleInstance
);
