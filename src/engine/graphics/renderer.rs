use glium::glutin::event_loop::EventLoop;
use glium::{Display, Surface, VertexBuffer};
use glium::program::Program;
use glium::texture::{UnsignedTexture2d, Texture2d, pixel_buffer::PixelBuffer};
use glium::framebuffer::{ColorAttachment, MultiOutputFrameBuffer, DepthRenderBuffer};
use super::{ModelsManager, Model, TexturesManager, Texture, vertex::{Vertex2d, VertexSkybox}};
use crate::engine::systems::MeshInstance;

pub struct Fbos {
    pub color: Texture2d,
    pub pick: UnsignedTexture2d,
    pub depth: DepthRenderBuffer,
}

rental! {
    mod rentals {
        use super::*;

        #[rental]
        pub struct ResolutionDependents {
            fbos: Box<Fbos>,
            framebuffer: (MultiOutputFrameBuffer<'fbos>, &'fbos Fbos),
        }
    }
}

pub struct Renderer {
    // Basic singletons
    display: Display,
    resolution: [u32; 2],
    projection: [[f32; 4]; 4],
    program_staticmesh: Program,
    program_skybox: Program,
    program_composition: Program,
    resolution_dependents: rentals::ResolutionDependents,
    instance_buffer: VertexBuffer<MeshInstance>,
    quad_vbuffer: VertexBuffer<Vertex2d>,
    skybox_model: Model<VertexSkybox>,

    models_manager: ModelsManager,
    textures_manager: TexturesManager,
    latest_pick_result: Option<u32>,
    picking_pbo: PixelBuffer<u32>,
}

fn matrix_to_floats(m: nalgebra::Matrix4<f32>) -> [[f32; 4]; 4] {
    m.into()
}

impl Renderer {
    pub fn get_display(&self) -> &Display {
        &self.display
    }

    pub fn get_pick_result(&self) -> Option<u32> {
        self.latest_pick_result
    }

    pub fn new(eventloop: &EventLoop<()>) -> Renderer {
        let display = super::window::make_window(eventloop);
        let program_staticmesh = super::shaders::staticmesh(&display);
        let program_composition = super::shaders::composition(&display);
        let program_skybox = super::shaders::static_skybox(&display);
        let resolution = crate::consts::DEFAULT_RESOLUTION;
        let models_manager = ModelsManager::new(&display);
        let textures_manager = TexturesManager::new(&display);

        let picking_pbo: PixelBuffer<u32> = PixelBuffer::new_empty(&display, 1);
        let instance_buffer = VertexBuffer::empty_dynamic(
            &display, crate::consts::DEFAULT_INSTANCE_BUFFER_SIZE
        ).unwrap();
        let quad_vbuffer = VertexBuffer::immutable(&display, &[
            Vertex2d {
                position: [-1.0, 1.0],
                texcoord: [0.0, 1.0],
            },
            Vertex2d {
                position: [1.0, 1.0],
                texcoord: [1.0, 1.0],
            },
            Vertex2d {
                position: [1.0, -1.0],
                texcoord: [1.0, 0.0],
            },
            Vertex2d {
                position: [-1.0, 1.0],
                texcoord: [0.0, 1.0],
            },
            Vertex2d {
                position: [1.0, -1.0],
                texcoord: [1.0, 0.0],
            },
            Vertex2d {
                position: [-1.0, -1.0],
                texcoord: [0.0, 0.0],
            },
        ]).unwrap();
        let resolution_dependents = Renderer::build_resolution_dependents(
            &display,
            resolution
        );

        let skybox_model = Renderer::create_skybox_vbuffer(&display);

        Renderer {
            display,
            program_staticmesh,
            program_composition,
            program_skybox,
            models_manager,
            textures_manager,
            latest_pick_result: None,
            resolution_dependents,
            instance_buffer,
            quad_vbuffer,
            skybox_model,
            picking_pbo,
            resolution,
            projection: nalgebra::Matrix4::new_perspective(
                crate::consts::DEFAULT_ASPECT_RATIO,
                crate::consts::DEFAULT_VERTICAL_FOV_DEG * std::f32::consts::PI / 180.0,
                crate::consts::DEFAULT_NEAR_CLIP,
                crate::consts::DEFAULT_FAR_CLIP,
            ).into()
        }
    }

    pub fn build_resolution_dependents(display: &Display, resolution: [u32; 2]) -> 
    rentals::ResolutionDependents {
        rentals::ResolutionDependents::new(
            Box::new(Fbos {
                color: Texture2d::empty_with_format(
                    display,
                    glium::texture::UncompressedFloatFormat::F32F32F32F32,
                    glium::texture::MipmapsOption::NoMipmap,
                    resolution[0],
                    resolution[1],
                ).unwrap(),
                pick: UnsignedTexture2d::empty_with_format(
                    display,
                    glium::texture::UncompressedUintFormat::U32,
                    glium::texture::MipmapsOption::NoMipmap,
                    resolution[0], 
                    resolution[1],
                ).unwrap(),
                depth: DepthRenderBuffer::new(
                    display,
                    glium::texture::DepthFormat::F32,
                    resolution[0],
                    resolution[1],
                ).unwrap(),
            }),
            |fbos| {
                (MultiOutputFrameBuffer::with_depth_buffer(
                    display,
                    [
                        ("color", ColorAttachment::Texture(fbos.color.main_level().into())),
                        ("pick", ColorAttachment::Texture(fbos.pick.main_level().into()))
                    ].iter().cloned(),
                    &fbos.depth
                ).unwrap(), fbos)
            }
        )
    }

    pub fn resize_window(&mut self, dims: [u32; 2]) {
        self.get_display().gl_window().window().set_inner_size(winit::dpi::LogicalSize::new(dims[0], dims[1]));
        self.resolution = dims;
        self.projection = nalgebra::Matrix4::new_perspective(
            (dims[0] as f32) / (dims[1] as f32),
            crate::consts::DEFAULT_VERTICAL_FOV_DEG * std::f32::consts::PI / 180.0,
            crate::consts::DEFAULT_NEAR_CLIP,
            crate::consts::DEFAULT_FAR_CLIP,
        ).into();
        self.resolution_dependents = Renderer::build_resolution_dependents(&self.display, dims);
    }

    pub fn draw_frame(
        &mut self,
        framebuilder: &super::FrameBuilder,
        camera: &dyn super::Camera,
        mouse_coords: [u32; 2] // for picking
    ) {
        // drawing a frame
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        self.resolution_dependents.rent_mut(|(fb, _fbos)| {
            fb.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        });

        // Skybox must be drawn first
        if let Some(sb) = &framebuilder.skybox {
            if let Texture::Cubemap(cm) = self.textures_manager.get(sb) {
                let proj = self.projection;
                let model = &self.skybox_model;
                let program = &self.program_skybox;
                let sb_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: false,
                        .. Default::default()
                    },
                    backface_culling: glium::BackfaceCullingMode::CullClockwise,
                    .. Default::default()
                };
                self.resolution_dependents.rent_mut(|(fb, _)| {
                    fb.draw(
                        &model.vertices,
                        &model.indices,
                        program,
                        &uniform!{
                            view: matrix_to_floats(camera.get_view_at_origin()),
                            proj: proj,
                            tex: cm.sampled().wrap_function(
                                // Without this there are weird 1-pixel flashing seams
                                glium::uniforms::SamplerWrapFunction::BorderClamp),
                        },
                        &sb_params
                    ).unwrap();
                });
            }
        }

        for (model, insts) in framebuilder.meshes.iter() {
            {
                let mut map = self.instance_buffer.map_write();
                for (index, inst) in insts.iter().enumerate() {
                    map.set(index, *inst);
                }
            }

            let model_data = self.models_manager.get(model);
            let ibufslice = self.instance_buffer.slice(0..insts.len()).unwrap();
            let program = &self.program_staticmesh;
            let proj = self.projection;
            self.resolution_dependents.rent_mut(|(fb, _)| {
                fb.draw(
                    (&model_data.vertices, ibufslice.per_instance().unwrap()),
                    &model_data.indices,
                    program,
                    &uniform!{view: matrix_to_floats(camera.get_view()), proj: proj},
                    &params
                ).unwrap();
            });
        }

        // Determine pick output
        self.resolution_dependents.rent(|(_fb, fbos)| {
            fbos.pick.main_level()
            .first_layer()
            .into_image(None).unwrap()
            .raw_read_to_pixel_buffer(&glium::Rect {
                left: crate::utils::clamp(mouse_coords[0] as u32, 0, self.resolution[0] - 1),
                bottom: crate::utils::clamp(
                    self.resolution[1] as i32 - mouse_coords[1] as i32,
                    0i32,
                    self.resolution[1] as i32 - 1
                ) as u32,
                width: 1,
                height: 1,
            }, &self.picking_pbo);
        });
        self.latest_pick_result = Some(self.picking_pbo.read().unwrap()[0]);
        if let Some(0) = self.latest_pick_result {
            self.latest_pick_result = None;
        }

        let mut target = self.display.draw();
        target.clear_color_srgb_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        self.resolution_dependents.rent(|(_fb, fbos)| {
            target.draw(
                &self.quad_vbuffer,
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program_composition,
                &uniform! {
                    color: &fbos.color,
                },
                &Default::default()
            ).unwrap();
        });
        target.finish().unwrap();
    }

    pub fn get_supported_resolutions(&self) -> Vec<[u32; 2]> {
        let max_size = self.get_display().gl_window().window().current_monitor().size();
        vec![[max_size.width as u32, max_size.height as u32]]
    }

    pub fn load_model(&mut self, m: &str) -> crate::utils::SWSResult<()> {
        self.models_manager.try_load(&self.display, m)
    }

    pub fn load_texture(&mut self, t: &str) -> crate::utils::SWSResult<()> {
        self.textures_manager.try_load(&self.display, t)
    }
    
    pub fn load_cubemap(&mut self, cm: &str) -> crate::utils::SWSResult<()> {
        self.textures_manager.try_load_cubemap(&self.display, cm)
    }
}