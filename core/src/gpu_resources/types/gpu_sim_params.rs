use crate::define_gpu_data_type;

define_gpu_data_type!(
    super::super::shaders::n_body_sim_compute::naga::types::SimParams as GpuSimParams
);

impl GpuSimParams {
    pub fn new(delta_time: f32, num_particles: u32, gravitational_constant: f32) -> Self {
        Self {
            delta_time,
            num_particles,
            gravitational_constant,
            softening: 0.1,
            min_distance: 1.0,
            max_distance: 100.0,
            _0: 0,
            _1: 0,
        }
    }
}
