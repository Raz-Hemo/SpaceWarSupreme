use glium::{Display, Program};

pub fn staticmesh(display: &Display) -> Program {
    Program::from_source(display,
    "
#version 450
// constants
uniform mat4 proj;
uniform mat4 view;

// vertex buffer
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 texcoord;

// instance buffer
layout(location = 3) in mat4 world_matrix;
layout(location = 7) in uint entity;

// outputs
layout(location = 0) out vec3 outnorm;
layout(location = 1) out vec2 outtex;
layout(location = 2) out uint outent;

void main() {
    outnorm = (world_matrix * vec4(normal, 0.0)).xyz;
    outtex = texcoord;
    outent = entity;
    gl_Position = proj * view * world_matrix * vec4(position, 1.0);
}",
    "
    #version 450
    layout(location = 0) in vec3 outnorm;
    layout(location = 1) in vec2 outtex;
    layout(location = 2) in flat uint entity;
    
    layout(location = 0) out vec4 color;
    layout(location = 1) out uint pick;
    void main() {
        float lighting = clamp(dot(outnorm, -normalize(-vec3(-1.0, 1.0, 0.0))), 0.0, 1.0);
        color = vec4(0.2 + 0.8 * vec3(lighting), 1.0);
        pick = entity;
    }
    ",
    None
    ).unwrap()
}

pub fn composition(display: &Display) -> Program {
    Program::from_source(display,
        "
            #version 450
            in vec2 position;
            in vec2 texcoord;
            out vec2 frag_texcoord;

            void main() {
                frag_texcoord = texcoord;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
        "
            #version 450
            uniform sampler2D color;
            in vec2 frag_texcoord;
            out vec4 frag_output;

            void main() {
                frag_output = texture(color, frag_texcoord);
            }
        ",
        None)
        .unwrap()
}

pub fn static_skybox(display: &Display) -> Program {
    Program::from_source(display,
        "
            #version 450
            // constants
            uniform mat4 proj;
            uniform mat4 view;

            in vec3 position;

            out vec3 out_direction;

            void main() {
                out_direction = position;
                gl_Position = proj * view * vec4(position, 1.0);
            }
        ",
        "
            #version 450
            uniform samplerCube tex;
            
            in vec3 out_direction;

            out vec4 color;
            out uint pick;

            void main() {
                color = texture(tex, out_direction);
                pick = 0;
            }
        ",
        None)
        .unwrap()
}