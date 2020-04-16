use glium::{Display, Program};

pub fn pbr(display: &Display) -> Program {
    Program::from_source(display,
        include_str!("./pbr.vert"),
        include_str!("./pbr.frag"),
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
            out vec4 fragColor;

            void main() {
                fragColor = texture(color, frag_texcoord);
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

            out vec4 fragColor;
            out uint fragPick;

            void main() {
                fragColor = texture(tex, out_direction);
                fragPick = 0;
            }
        ",
        None)
        .unwrap()
}