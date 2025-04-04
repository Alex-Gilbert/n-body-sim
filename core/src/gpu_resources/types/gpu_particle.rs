use glam::{Vec3, Vec4};

use crate::define_gpu_data_type;

define_gpu_data_type!(
    super::super::shaders::n_body_sim_compute::naga::types::Particle as GpuParticle
);

impl GpuParticle {
    pub fn new_random(
        rng: &mut impl rand::Rng,
        dimensions: f32,
        min_mass: f32,
        max_mass: f32,
        min_velocity: f32,
        max_velocity: f32,
    ) -> Self {
        let mass = rng.gen_range(min_mass..max_mass);
        let position = Vec3::new(
            rng.gen_range(-dimensions..dimensions),
            rng.gen_range(-dimensions..dimensions),
            rng.gen_range(-dimensions..dimensions),
        );
        let velocity = rng.gen_range(min_velocity..max_velocity);
        let mut velocity_vector = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize();
        velocity_vector *= velocity;

        Self {
            position: Vec4::new(position.x, position.y, position.z, mass),
            velocity: Vec4::new(velocity_vector.x, velocity_vector.y, velocity_vector.z, 0.0),
        }
    }
}
