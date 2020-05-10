use crate::engine::prelude::*;
use glium::glutin::event_loop::EventLoop;
use glium::{Display, Surface, VertexBuffer};
use glium::program::Program;
use glium::uniforms::{UniformBuffer, UniformValue};
use glium::texture::{UnsignedTexture2d, Texture2d, pixel_buffer::PixelBuffer, CompressedSrgbTexture2d,
    CompressedTexture2d};
use glium::framebuffer::{ColorAttachment, MultiOutputFrameBuffer, DepthRenderBuffer};
use super::{ModelsManager, Model, TexturesManager, Texture, vertex::{Vertex2d, VertexSkybox}};
use crate::engine::systems::MeshInstance;

pub struct Fbos {
    pub color: Texture2d,
    pub pick: UnsignedTexture2d,
    pub depth: DepthRenderBuffer,
}

struct PointLight {
    pos: [f32; 3],
    color: [f32; 3],
}

struct Lights {
    pointlights: [PointLight; consts::DEFAULT_MAX_LIGHTS]
}

struct PbrUniforms<'a> {
    view: nalgebra::Matrix4<f32>,
    proj: nalgebra::Matrix4<f32>,
    albedo: &'a CompressedSrgbTexture2d,
    metallic_rough: &'a CompressedTexture2d,
    normal_map: &'a CompressedTexture2d,
    ao: &'a CompressedTexture2d,

    lights: &'a Lights,
    camera_position: nalgebra::Point3<f32>,
    exposure: f32,
}
impl<'a> glium::uniforms::Uniforms for PbrUniforms<'a> {
    fn visit_values<'b, F: FnMut(&str, UniformValue<'b>)>(&'b self, mut f: F) {
        f("view", UniformValue::Mat4(self.view.into()));
        f("proj", UniformValue::Mat4(self.proj.into()));
        let sampler = Some(glium::uniforms::SamplerBehavior{
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Linear,
            minify_filter: glium::uniforms::MinifySamplerFilter::LinearMipmapLinear,
            ..Default::default()
        });
        f("albedo", UniformValue::CompressedSrgbTexture2d(self.albedo, sampler));
        f("metallic_rough", UniformValue::CompressedTexture2d(self.metallic_rough, sampler));
        f("normal_map", UniformValue::CompressedTexture2d(self.normal_map, sampler));
        f("ao", UniformValue::CompressedTexture2d(self.ao, sampler));
        f("cameraPosition", UniformValue::Vec3(point_to_floats(self.camera_position.into())));
        f("exposure", UniformValue::Float(self.exposure));
        for i in 0..consts::DEFAULT_MAX_LIGHTS {
            f(&format!("pointlights[{}].pos", i), UniformValue::Vec3(self.lights.pointlights[i].pos));
            f(&format!("pointlights[{}].color", i), UniformValue::Vec3(self.lights.pointlights[i].color));
        }
    }
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
    projection: nalgebra::Matrix4<f32>,
    program_pbr: Program,
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

    pub light_pos: [f32; 3],
}

fn matrix_to_floats(m: nalgebra::Matrix4<f32>) -> [[f32; 4]; 4] {
    m.into()
}

fn vector_to_floats(v: nalgebra::Vector3<f32>) -> [f32; 3] {
    v.into()
}

fn point_to_floats(p: nalgebra::Point3<f32>) -> [f32; 3] {
    p.coords.into()
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
        let program_pbr = super::shaders::pbr(&display);
        let program_composition = super::shaders::composition(&display);
        let program_skybox = super::shaders::static_skybox(&display);
        let resolution = consts::DEFAULT_RESOLUTION;
        let models_manager = ModelsManager::new(&display);
        let textures_manager = TexturesManager::new(&display);

        let picking_pbo: PixelBuffer<u32> = PixelBuffer::new_empty(&display, 1);
        let instance_buffer = VertexBuffer::empty_dynamic(
            &display, consts::DEFAULT_INSTANCE_BUFFER_SIZE
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
            program_pbr,
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
            light_pos: [0.4f32, 0.7, 0.25],
            projection: nalgebra::Matrix4::new_perspective(
                consts::DEFAULT_ASPECT_RATIO,
                consts::DEFAULT_VERTICAL_FOV_DEG * std::f32::consts::PI / 180.0,
                consts::DEFAULT_NEAR_CLIP,
                consts::DEFAULT_FAR_CLIP,
            )
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
                        ("fragColor", ColorAttachment::Texture(fbos.color.main_level().into())),
                        ("fragPick", ColorAttachment::Texture(fbos.pick.main_level().into()))
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
            consts::DEFAULT_VERTICAL_FOV_DEG * std::f32::consts::PI / 180.0,
            consts::DEFAULT_NEAR_CLIP,
            consts::DEFAULT_FAR_CLIP,
        );
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
            if let Some(Texture::Cubemap(cm)) = self.textures_manager.get(sb) {
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
                        &model.primitives[0].vertices,
                        &model.primitives[0].indices,
                        program,
                        &uniform!{
                            view: matrix_to_floats(camera.get_view_at_origin()),
                            proj: Into::<[[f32; 4]; 4]>::into(proj),
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
                if insts.len() > consts::DEFAULT_INSTANCE_BUFFER_SIZE {
                    panic!("Too many instances of one model!")
                }
                let mut map = self.instance_buffer.map_write();
                for (index, inst) in insts.iter().enumerate() {
                    map.set(index, *inst);
                }
            }

            let model_data = self.models_manager.get(model);
            let ibufslice = self.instance_buffer.slice(0..insts.len()).unwrap();
            let program = &self.program_pbr;
            let proj = self.projection;
            let texture_manager = &self.textures_manager;
            
            let mut lights = Lights {
                pointlights: [
                    PointLight {
                        pos: self.light_pos,
                        color: [400.0f32, 400.0, 400.0]//[4.5f32, 7.0, 450.0]
                    },
                    PointLight {
                        pos: self.light_pos,
                        color: [400.0f32, 400.0, 400.0]//[4.5f32, 450.0, 30.0]
                    },
                ]
            };
            lights.pointlights[1].pos[0] *= -1.0;

            self.resolution_dependents.rent_mut(|(fb, _)| {
                for p in model_data.primitives.iter() {
                    fb.draw(
                        (&p.vertices, ibufslice.per_instance().unwrap()),
                        &p.indices,
                        program,
                        &PbrUniforms {
                            view: camera.get_view(),
                            proj: proj,
                            albedo: &p.albedo.as_ref().unwrap_or(texture_manager.get_default_albedo()),
                            metallic_rough: &p.metal_roughness.as_ref().unwrap_or(texture_manager.get_default_rough_metal()),
                            normal_map: &p.normalmap.as_ref().unwrap_or(texture_manager.get_default_normal()),
                            ao: &p.occlusion.as_ref().unwrap_or(texture_manager.get_default_occ()),

                            lights: &lights,
                            camera_position: camera.get_world_position(),
                            exposure: 1.0f32, // between 1 and 3?  
                        },
                        &params
                    ).unwrap();
                }
            });
        }

        // Determine pick output
        self.resolution_dependents.rent(|(_fb, fbos)| {
            fbos.pick.main_level()
            .first_layer()
            .into_image(None).unwrap()
            .raw_read_to_pixel_buffer(&glium::Rect {
                left: utils::clamp(mouse_coords[0] as u32, 0, self.resolution[0] - 1),
                bottom: utils::clamp(
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

    pub fn load_model(&mut self, m: &str) -> anyhow::Result<()> {
        self.models_manager.try_load(&self.display, m)
    }

    pub fn load_texture(&mut self, t: &str) -> anyhow::Result<()> {
        self.textures_manager.try_load(&self.display, t)
    }
    
    pub fn load_cubemap(&mut self, cm: &str) -> anyhow::Result<()> {
        self.textures_manager.try_load_cubemap(&self.display, cm)
    }
}