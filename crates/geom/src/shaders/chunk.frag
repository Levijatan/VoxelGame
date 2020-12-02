#version 450

layout(location=0) in vec3 o_uvw;
layout(location=1) in vec4 o_local_camera_pos_lod;
layout(location=2) in vec3 o_color;
layout(location=3) in vec3 o_local_pos;

layout(location=0) out vec4 f_color;

layout(set = 1, binding = 5) uniform texture2D ChunkUniform_voxel_texture;
layout(set = 1, binding = 6) uniform sampler ChunkUniform_voxel_texture_sampler;


// "p" point being textured
// "n" surface normal at "p"
// "k" controls the sharpness of the blending in the transitions areas
// "s" texture sampler
vec4 boxmap(in vec3 p, in vec3 n, in float k )
{
    // project+fetch
    vec4 x = texture(sampler2D(ChunkUniform_voxel_texture,  ChunkUniform_voxel_texture_sampler), p.yz );
    vec4 y = texture(sampler2D(ChunkUniform_voxel_texture,  ChunkUniform_voxel_texture_sampler), p.zx );
    vec4 z = texture(sampler2D(ChunkUniform_voxel_texture,  ChunkUniform_voxel_texture_sampler), p.xy );
    
    // blend factors
    vec3 w = pow( abs(n), vec3(k) );
    // blend and return
    return (x*w.x + y*w.y + z*w.z) / (w.x + w.y + w.z);
}


void main() {
    f_color = vec4(o_color, 1.0) * boxmap(o_uvw, o_local_pos, 100.0);
}
