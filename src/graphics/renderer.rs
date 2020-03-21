extern crate vulkano;
extern crate vulkano_shaders;

use std::sync::Arc;

use vulkano_win;
use winit::{
    window::Window,
    event_loop::EventLoop,
}; 
use vulkano::sync::{SharingMode, GpuFuture, FlushError};
use vulkano::swapchain::{
    display::Display,
    AcquireError, Swapchain, SurfaceTransform, Surface, PresentMode, CompositeAlpha,
    FullscreenExclusive, ColorSpace};
use vulkano::instance::{InstanceExtensions, Instance, PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::framebuffer::{RenderPassAbstract, Subpass, FramebufferAbstract, Framebuffer};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract, viewport::Viewport};
use vulkano::command_buffer::{DynamicState, AutoCommandBufferBuilder};

type SWSSwapchain = Arc<Swapchain<Window>>;
type SWSFramebuffer = Arc<dyn FramebufferAbstract + Send + Sync>;
type SWSRenderPass = Arc<dyn RenderPassAbstract + Send + Sync>;
type SWSPipeline = Arc<dyn GraphicsPipelineAbstract>;

#[derive(Default, Debug, Clone)]
struct UIVertex {
    position: [f32; 2],
}
vulkano::impl_vertex!(UIVertex, position);

const MAX_FRAMES_IN_QUEUE: usize = 2;

pub enum DrawResult {
    Success,
    ResizeNeeded,
}

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

    previous_frame_ends: Vec<Box<dyn GpuFuture>>,
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
            &InstanceExtensions {
                //khr_display: true,
                .. vulkano_win::required_extensions()
            },
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

        let (swapchain, _swapchain_images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            caps.min_image_count, // number of buffers
            caps.supported_formats[0].0,
            // Start with 640x480. Immediately resized later
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
        .vertex_input_single_buffer::<UIVertex>()
        .triangle_list()
        .vertex_shader(vs.main_entry_point(), ())
        .fragment_shader(fs.main_entry_point(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .viewports_dynamic_scissors_irrelevant(1)
        .build(device.clone())
        .unwrap());

        let mut previous_frame_ends = Vec::new();
        for i in 0..MAX_FRAMES_IN_QUEUE {
            previous_frame_ends.push(Box::new(vulkano::sync::now(device.clone())) 
                                 as Box<dyn GpuFuture>);
        }

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
            previous_frame_ends,
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
                Framebuffer::start(render_pass.clone())
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

    pub fn resize_window(&mut self, dims: [u32; 2]) {
        let (swapchain, framebuffers, viewports) = Renderer::window_size_dependent_setup(
            dims,
            self.swapchain.clone(),
            self.render_pass.clone()
        );
        self.swapchain = swapchain;
        self.framebuffers = framebuffers;
        self.dynamic_state.viewports = Some(viewports);
    }

    pub fn draw_frame(&mut self) -> DrawResult {
        // Wait for one of the previous frames to finish by dropping it
        self.previous_frame_ends.remove(0).cleanup_finished();

        let (image_num, suboptimal, acquire_future) =
            match vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(e) => panic!("Acquire swapchain failed: {:?}", e),
        };
        if suboptimal {
            return DrawResult::ResizeNeeded
        }

        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family())
            .expect("Failed to create command buffer builder")
            .begin_render_pass(self.framebuffers[image_num].clone(), false, vec![[0.5, 0.0, 0.0, 1.0].into()]).expect("Failed to begin render pass")
            //.draw(self.pipeline.clone(), &self.dynamic_state, vertex_buffer.clone(), (), ()).unwrap()
            .end_render_pass().expect("Failed to end render pass")
            .build().expect("Failed to build command buffer");

        let future = vulkano::sync::now(self.device.clone())
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
            .expect("Failed to add command buffer")
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        match future {
            Ok(f) => {
                self.previous_frame_ends.push(Box::new(f) as Box<dyn GpuFuture>);
                DrawResult::Success
            }
            Err(FlushError::OutOfDate) => {
                self.previous_frame_ends.push(Box::new(vulkano::sync::now(self.device.clone())));
                DrawResult::ResizeNeeded
            }
            Err(e) => {
                self.previous_frame_ends.push(Box::new(vulkano::sync::now(self.device.clone())));
                panic!("Draw frame failed: {:?}", e);
            }
        }
    }

    pub fn get_supported_resolutions(&self) -> Vec<[u32; 2]> {
        let mut result: Vec<[u32; 2]> = Vec::new();
        for display in Display::enumerate(pick_best_physical_device(&self.instance)) {
            for mode in display.display_modes() {
                result.push(mode.visible_region());
            }
        }

        result
    }
}