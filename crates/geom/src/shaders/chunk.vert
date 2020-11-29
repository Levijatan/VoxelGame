#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout(location=0) out vec3 o_uvw;
layout(location=1) out vec4 v_position;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform ChunkUniform {
    vec4 ChunkUniform_voxel_size;
    vec4 ChunkUniform_camera_pos;
};

struct InstanceData {
    vec4 position;
    vec4 color;
};

layout(std140, set = 1, binding = 1) buffer ChunkMaterial_instances {
    InstanceData[] instances;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    uint vx = gl_VertexIndex;
    uint instance = vx >> 3;

    vec4 instance_pos = instances[instance].position;
    vec3 local_camera_pos = ChunkUniform_camera_pos.xyz - (Model * instance_pos).xyz;

    uvec3 xyz = uvec3(vx & 0x1, (vx & 0x4) >> 2, (vx & 0x2) >> 1);

    if (local_camera_pos.x > 0) xyz.x = 1 - xyz.x;
    if (local_camera_pos.y > 0) xyz.y = 1 - xyz.y;
    if (local_camera_pos.z > 0) xyz.z = 1 - xyz.z;

    vec3 uvw = vec3(xyz);
    vec3 pos = uvw * 2.0 - 1.0;
    
    vec3 local_pos = pos.xyz;
    
    float lod = 0.5 * log2(dot(local_camera_pos, local_camera_pos)) - 6.0;
    vec3 texel_scale_lod = ChunkUniform_voxel_size.xyz * exp2(clamp(lod, 0.0, 5.0));

    o_uvw = uvw * (vec3(1.0) - texel_scale_lod) + texel_scale_lod * 0.5;

    v_position = vec4(local_camera_pos, lod);
    gl_Position = ViewProj * Model * vec4(local_pos + instance_pos.xyz, 1.0);
}
