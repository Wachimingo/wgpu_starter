use pollster::FutureExt;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::logic::{render_pipeline::{create_render_pipeline, RenderPipelineInput}, renderer::Renderer, vertex::{Vertex, VERTICES}};

pub fn init<'a>(window_arc: Window) -> Renderer<'a> {
    let window = std::sync::Arc::new(window_arc);
    let size = window.inner_size();
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });
    let surface = instance.create_surface(window.clone()).unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).block_on().unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        label: None,
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
    }).block_on().unwrap();
    let surface_capabilities = surface.get_capabilities(&adapter);
    let surface_format = surface_capabilities.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(surface_capabilities.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_capabilities.present_modes[0],
        alpha_mode: surface_capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/render.wgsl").into())
    });
    let render_pipeline = create_render_pipeline(RenderPipelineInput {
        device: &device,
        config: &config,
        shader: &shader,
    });
    Renderer { window, surface, device, queue, config, size, render_pipeline, vertex_buffer }
}
