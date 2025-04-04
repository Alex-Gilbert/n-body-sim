use crate::define_gpu_data_type;

define_gpu_data_type!(
    super::super::shaders::n_body_sim_compute::naga::types::IndirectArgs as GpuIndirectArgs
);


impl GpuIndirectArgs {
    pub fn new(index_count: u32, instance_count: u32) -> Self {
        Self {
            index_count,
            instance_count,
            first_index: 0,
            vertex_offset: 0,
            first_instance: 0,
        }
    }
}
