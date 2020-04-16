#version 450

layout(location=0) in vec3 position;
layout(location=1) in vec2 texcoord;
layout(location=2) in vec3 normal;
layout(location=3) in vec3 tangent;

// instance buffer
layout(location = 4) in mat4 world_matrix;
layout(location = 8) in uint entity;

layout(location=0) out vec3 v_worldpos;
layout(location=1) out vec2 v_tex;
layout(location=2) out vec3 v_tangent;
layout(location=3) out vec3 v_norm;
layout(location=4) out uint v_ent;

// constants
uniform mat4 proj;
uniform mat4 view;

void main() {
    v_tex = texcoord;
    v_ent = entity;
    v_tangent = (world_matrix * vec4(tangent, 0.0)).xyz;
    v_norm = (world_matrix * vec4(normal, 0.0)).xyz;
    v_worldpos = (world_matrix * vec4(position, 1.0)).xyz;
    gl_Position = proj * view * vec4(v_worldpos, 1.0);
}