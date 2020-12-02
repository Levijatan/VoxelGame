#version 450

layout(location=0) out vec3 o_uvw;
layout(location=1) out vec4 o_local_camera_pos_lod;
layout(location=2) out vec3 o_color;
layout(location=3) out vec3 o_local_pos;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform ChunkUniform_voxel_size {
    vec4 voxel_size;
};

layout(set = 1, binding = 1) uniform ChunkUniform_camera_pos {
    vec4 camera_pos;
};

layout(set = 1, binding = 2) uniform ChunkUniform_center_to_edge {
    vec4 center_to_edge;
};

layout(set = 1, binding = 3) uniform Transform {
    mat4 Model;
};

struct InstanceData {
    vec4 position;
    vec3 color;
};

layout(set = 1, binding = 4) buffer ChunkInstances_instances {
    InstanceData[] instances;
};



void main() {
    uint vx = gl_VertexIndex;
    uint instance = vx >> 3;

    uvec3 xyz = uvec3(vx & 0x1, (vx & 0x4) >> 2, (vx & 0x2) >> 1);
    vec3 uvw = vec3(xyz);
    vec3 pos = uvw * 2.0 - 1.0;

    vec3 instance_pos = instances[instance].position.xyz;

    vec3 local_pos = pos.xyz * center_to_edge.xyz;
    vec3 local_camera_pos = camera_pos.xyz - instance_pos;
 
    float lod = 0.5 * log2(dot(local_camera_pos, local_camera_pos)) - 6.0;

    vec3 texel_scale_lod = voxel_size.xyz * exp2(clamp(lod, 0.0, 5.0));

    o_uvw = uvw * (vec3(1.0) - texel_scale_lod) + texel_scale_lod * 0.5;
    o_color = instances[instance].color;
    o_local_pos = local_pos;
    o_local_camera_pos_lod = vec4(local_camera_pos, lod);

    gl_Position = ViewProj * vec4(local_pos + instance_pos, 1.0);
}
