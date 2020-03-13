extern crate vulkano;
extern crate vulkano_shaders;

use std::sync::Arc;

use vulkano_win;
use winit::{
    window::Window,
    event_loop::EventLoop,
}; 
use vulkano::sync::SharingMode;
use vulkano::swapchain::{Swapchain, SurfaceTransform, Surface, PresentMode, CompositeAlpha,
                         FullscreenExclusive, ColorSpace};
use vulkano::instance::{Instance, PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::framebuffer::{RenderPassAbstract, Subpass, FramebufferAbstract, Framebuffer};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract, viewport::Viewport};
use vulkano::command_buffer::DynamicState;

type SWSSwapchain = Arc<Swapchain<Window>>;
type SWSFramebuffer = Arc<dyn FramebufferAbstract + Send + Sync>;
type SWSRenderPass = Arc<dyn RenderPassAbstract + Send + Sync>;
type SWSPipeline = Arc<dyn GraphicsPipelineAbstract>;

pub struct Renderer {
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    dynamic_state: DynamicState,

    framebuffers: Vec<SWSFramebuffer>,
    render_pass: SWSRenderPass,
    pipeline: SWSPipeline,
}

impl Renderer {
    pub fn get_window(self: &Renderer) -> &Window {
        self.surface.window()
    }
}

fn pick_best_physical_device(inst: &Arc<Instance>) -> PhysicalDevice {
    // Start with any GPU
    let mut result = PhysicalDevice::enumerate(inst).next().expect("Failed to find a physical graphics device");

    // Discrete GPU is better
    for pd in PhysicalDevice::enumerate(inst) {
        if pd.ty() == PhysicalDeviceType::DiscreteGpu {
            result = pd
        }
    }

    crate::log::info(&format!("Using GPU '{}'", result.name()));
    result
}

impl Renderer {
    pub fn new(eventloop: &EventLoop<()>) -> Renderer {
        // Create instance + window
        let instance = Instance::new(
            None,
            &vulkano_win::required_extensions(),
            None).expect("Failed to create Vulkan instance");
        let surface = crate::graphics::window::make_window(eventloop, instance.clone());
        
        // Pick a physical device (i.e. GPU)
        let physical_device = pick_best_physical_device(&instance);
        let qf = physical_device.queue_families()
            .find(|&q| q.supports_graphics())
            .expect("Couldn't find a Vulkan queue family with graphics support");

        // Create device + command queues
        let (device, mut queues) = {
            Device::new(
                physical_device, 
                &Features::none(), 
                &DeviceExtensions {
                    khr_swapchain: true,
                    .. DeviceExtensions::none()
                },
                [(qf, 0.5)].iter().cloned()
            ).expect("Failed to create Vulkan device")
        };
        let queue = queues.next().expect("Device::new returned no queues");

        // Create swap chain
        let caps = surface.capabilities(physical_device)
            .expect("Failed to get surface capabilities");

        let (swapchain, swapchain_images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            caps.min_image_count, // number of buffers
            caps.supported_formats[0].0,
            caps.current_extent.unwrap_or([640, 480]),
            1, // layers of each buffer
            caps.supported_usage_flags,
            SharingMode::Exclusive,
            SurfaceTransform::Identity,
            CompositeAlpha::Opaque,
            PresentMode::Fifo,
            FullscreenExclusive::Disallowed,
            true, // Clip portions of the window that are outside the screen for performance
            ColorSpace::SrgbNonLinear
        ).expect("Failed to create swapchain");

        let render_pass = Arc::new(vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).expect("Failed to create render pass"));

        let (swapchain, framebuffers, viewports) = Renderer::window_size_dependent_setup(
            [640, 480],
            swapchain.clone(),
            render_pass.clone()
        );

        let dynamic_state = DynamicState {
            line_width: None,
            viewports: Some(viewports),
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };

        mod vs {
            vulkano_shaders::shader!{
                ty: "vertex",
                src: "
    #version 450
    layout(location = 0) in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }"
            }
        }
    
        mod fs {
            vulkano_shaders::shader!{
                ty: "fragment",
                src: "
    #version 450
    layout(location = 0) out vec4 f_color;
    void main() {
        f_color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "
            }
        }
        let vs = vs::Shader::load(device.clone()).expect("Failed to create Vertex Shader");
        let fs = fs::Shader::load(device.clone()).expect("Failed to create Fragment Shader");

        let pipeline = Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer()
        .triangle_list()
        .vertex_shader(vs.main_entry_point(), ())
        .fragment_shader(fs.main_entry_point(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .viewports_dynamic_scissors_irrelevant(1)
        .build(device.clone())
        .unwrap());

        Renderer {
            instance,
            surface,
            device, 
            queue,
            swapchain,
            framebuffers,
            render_pass,
            pipeline,
            dynamic_state,
        }
    }

    pub fn window_size_dependent_setup(
        dims: [u32; 2],
        swapchain: SWSSwapchain,
        render_pass: SWSRenderPass)
    -> (SWSSwapchain, Vec<SWSFramebuffer>, Vec<Viewport>) {
        let (new_swapchain, new_images) = match swapchain.recreate_with_dimensions(dims) {
            Ok(result) => result,
            Err(e) => panic!("Can't resize window to dims {:?}, error: {}", dims, e),
        };

        (new_swapchain,
        new_images.iter().map(|image| {
            Arc::new(
                Framebuffer::start(&render_pass.clone())
                .add(image.clone()).expect("Failed to add attachment to framebuffer")
                .build().expect("Failed to create framebuffer")
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        }).collect::<Vec<_>>(),
        vec![Viewport {
            origin: [0.0, 0.0],
            dimensions: [dims[0] as f32, dims[1] as f32],
            depth_range: 0.0 .. 1.0,
        }])
    }

    pub fn resize_window(&self, dims: [u32; 2]) {
        let (swapchain, framebuffers, viewports) = Renderer::window_size_dependent_setup(
            dims,
            self.swapchain.clone(),
            self.render_pass.clone()
        );
        self.swapchain = swapchain;
        self.framebuffers = framebuffers;
        self.dynamic_state.viewports = Some(viewports);
    }

    pub fn acquire(&self) {
        let (image_num, suboptimal, acquire_future) =
            vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();
    }
}