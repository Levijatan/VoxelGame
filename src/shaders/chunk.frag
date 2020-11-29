#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout(location=0) in vec3 o_uvw;
layout(location=1) in vec4 v_position;

layout(location=0) out vec4 f_color;

void main() {

    f_color = vec4(o_uvw, 1);
}
