#define CAMERA_GROUP 0
#import include/camera.wgsl

#define TEXTURE_GROUP 1
#define TEXTURE_BINDING 0
#import include/texture_sampler.wgsl as diffuse

#import include/basic_vertex.wgsl

@export struct ParticleInstance {
    @location(2) position: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) velocity: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) velocity: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    vertex: basic_vertex::BasicVertex,
    instance: ParticleInstance,
) -> VertexOutput {
    var output: VertexOutput;

    // Scale the mesh by the instance's size (stored in position.w)
    let world_position = vertex.position * instance.position.w + instance.position.xyz;

    // Transform to clip space
    output.clip_position = camera::to_clip(world_position);

    // Pass instance color and velocity to fragment shader
    output.color = instance.color;
    output.velocity = instance.velocity.xyz;
    output.tex_coords = vertex.tex_coords;

    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return diffuse::sample_2D(in.tex_coords.xy) * in.color;
}
