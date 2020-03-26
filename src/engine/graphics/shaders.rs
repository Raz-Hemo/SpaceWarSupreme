pub mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: "
#version 450
// push constants
layout(push_constant) uniform PushConstantData {
    mat4 view;
    mat4 proj;
} pc;

// vertex buffer
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 texcoord;

// instance buffer
layout(location = 3) in mat4 world_matrix;

// outputs
layout(location = 0) out vec3 outnorm;
layout(location = 1) out vec2 outtex;

void main() {
    outnorm = (world_matrix * vec4(normal, 0.0)).xyz;
    outtex = texcoord;
    gl_Position = pc.proj * pc.view * world_matrix * vec4(position, 1.0);
}"
    }
}

pub mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        src: "
#version 450
layout(location = 0) in vec3 outnorm;
layout(location = 1) in vec2 outtex;

layout(location = 0) out vec4 f_color;
void main() {
f_color = vec4(abs(outnorm), 1.0);
}
"
    }
}