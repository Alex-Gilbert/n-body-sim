// Input binding for the particle data
@export struct Particle {
    position: vec4<f32>,  // xyz = position, w = mass
    velocity: vec4<f32>,  // xyz = velocity, w = unused
}

// Instance data for rendering
struct Instance {
    position: vec4<f32>,  // xyz = position, w = size/scale
    color: vec4<f32>,     // rgba color
    velocity: vec4<f32>,  // For visual effects like trails/rotation
}

// Parameters for the simulation
@export struct SimParams {
    delta_time: f32,
    num_particles: u32,
    gravitational_constant: f32,
    softening: f32,       // To avoid numerical instability when particles get too close
    min_distance: f32,     // Threshold for instance inclusion
    max_distance: f32,     // Upper bound for instance inclusion
    _0: u32,               // Padding
    _1: u32,               // Padding
}

@export struct IndirectArgs {
    index_count: u32,
    instance_count: atomic<u32>, // for atomic append
    first_index: u32,
    vertex_offset: u32,
    first_instance: u32,
}

// Input and output bindings
@group(0) @binding(0) var<storage, read> particles: array<Particle>;
@group(0) @binding(1) var<storage, read_write> new_particles: array<Particle>;
@group(0) @binding(2) var<uniform> params: SimParams;
@group(0) @binding(3) var<storage, read_write> instance_buffer: array<Instance>;
@group(0) @binding(4) var<storage, read_write> indirect_buffer: IndirectArgs;

// @compute @workgroup_size(64)
@compute @workgroup_size(8,8,1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    // Guard against out-of-bounds work
    if (index >= params.num_particles) {
        return;
    }

    // Load the current particle
    let current_particle = particles[index];
    var new_particle = current_particle;

    // Apply N-body gravitational force
    var total_force = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = 0u; i < params.num_particles; i = i + 1u) {
        // Skip self
        if (i == index) {
            continue;
        }

        let other_particle = particles[i];
        let mass_j = other_particle.position.w;

        // Calculate distance vector
        let diff = other_particle.position.xyz - current_particle.position.xyz;
        let dist_sqr = dot(diff, diff) + params.softening;
                
        // Newton's law of universal gravitation: F = G * m1 * m2 / r^2
        let inv_dist_sqr = 1.0 / dist_sqr;
                
        // Calculate the gravitational force
        let force = params.gravitational_constant * current_particle.position.w * mass_j * inv_dist_sqr;
                
        // Accumulate force
        total_force = total_force + normalize(diff) * force;
    }

    // Update velocity based on force (F = ma, so a = F/m)
    let new_velocity = current_particle.velocity.xyz + (total_force / current_particle.position.w) * params.delta_time;
    new_particle.velocity.x = new_velocity.x;
    new_particle.velocity.y = new_velocity.y;
    new_particle.velocity.z = new_velocity.z;

    // Update position based on velocity
    let new_position = current_particle.position.xyz + new_velocity * params.delta_time;
    new_particle.position.x = new_position.x;
    new_particle.position.y = new_position.y;
    new_particle.position.z = new_position.z;

    // Store updated particle
    new_particles[index] = new_particle;

    // Determine if this particle should be included in the instance buffer for rendering
    // based on its distance from origin or other criteria
    let distance_from_origin = length(new_particle.position.xyz);

    if (distance_from_origin >= params.min_distance && distance_from_origin <= params.max_distance) {
        // Atomically append this particle's data to the instance buffer
        let old_count = atomicAdd(&indirect_buffer.instance_count, 1u);

        // Ensure we don't overflow the instance buffer
        if (old_count < arrayLength(&instance_buffer)) {
            // Create an instance based on the particle properties
            var instance: Instance;

            // Set position (xyz) and size based on mass (w)
            instance.position = vec4<f32>(
                new_particle.position.xyz, 0.5 + (new_particle.position.w * 0.00001)  // Scale size based on mass
            );

            // Set color based on velocity (faster = redder)
            let speed = length(new_particle.velocity.xyz);
            instance.color = vec4<f32>(
                min(1.0, speed / 20.0),         // R: higher with speed
                min(1.0, 0.2 + 0.8 / speed),    // G: lower with speed
                min(1.0, 0.5 / speed),          // B: lower with speed
                1.0                             // A: fully opaque
            );

            // Store velocity for visual effects
            instance.velocity = vec4<f32>(new_particle.velocity.xyz, 0.0);

            // Write the instance
            instance_buffer[old_count] = instance;
        }
    }
}
