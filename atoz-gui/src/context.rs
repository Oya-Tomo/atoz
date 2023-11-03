use atoz_renderer::{
    layer::Layer,
    pipeline::{
        circle::CirclePipeline, image::ImagePipeline, rect::RectPipeline,
        triangle::TrianglePipeline,
    },
    viewport::Viewport,
};
use wgpu::{
    InstanceDescriptor, RenderPassColorAttachment, RenderPassDescriptor, SurfaceConfiguration,
    TextureViewDescriptor,
};
use winit::{
    dpi::Size,
    event_loop::EventLoop,
    window::{WindowBuilder, WindowId},
};

#[derive(Debug)]
pub struct Context {
    window: winit::window::Window,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    layers: Vec<Layer>,
}

impl Context {
    pub async fn init<S: Into<Size>>(event_loop: &EventLoop<()>, inner_size: S) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        window.set_inner_size(inner_size);
        let size = window.inner_size();

        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        return Self {
            window,
            device,
            queue,
            surface,
            config,
            layers: vec![],
        };
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn get_config(&self) -> SurfaceConfiguration {
        return self.config.clone();
    }

    pub fn get_window_id(&self) -> WindowId {
        return self.window.id();
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn push_layers(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn clear_layers(&mut self) {
        self.layers.clear();
    }

    pub fn render(&self) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("context.render.encoder"),
            });

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let rect_pipeline = RectPipeline::new(&self.device, self.config.format);
        let triangle_pipeline = TrianglePipeline::new(&self.device, self.config.format);
        let circle_pipeline = CirclePipeline::new(&self.device, self.config.format);
        let image_pipeline = ImagePipeline::new(&self.device, self.config.format);

        let viewport_group = Viewport::new(self.config.width as _, self.config.height as _)
            .get_bind_group(&self.device);

        for layer in &self.layers {
            let rect_buffer = layer.get_rect_buffer(&self.device);
            let rect_count = layer.rects.len();

            let triangle_buffer = layer.get_triangle_buffer(&self.device);
            let triangle_count = layer.triangles.len();

            let circle_buffer = layer.get_circle_buffer(&self.device);
            let circle_count = layer.circles.len();

            let image_buffer = layer.get_image_buffers(&self.device);

            {
                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("context.render.render_pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                if rect_count > 0 {
                    rect_pipeline.render(
                        &mut render_pass,
                        &rect_buffer,
                        rect_count as _,
                        &viewport_group,
                    );
                }

                if triangle_count > 0 {
                    triangle_pipeline.render(
                        &mut render_pass,
                        &triangle_buffer,
                        triangle_count as _,
                        &viewport_group,
                    );
                }

                if circle_count > 0 {
                    circle_pipeline.render(
                        &mut render_pass,
                        &circle_buffer,
                        circle_count as _,
                        &viewport_group,
                    );
                }

                image_buffer
                    .iter()
                    .for_each(|(texture, instance_buffer, instance_count)| {
                        if instance_count > &0 {
                            image_pipeline.render(
                                &mut render_pass,
                                instance_buffer,
                                instance_count.clone(),
                                texture,
                                &viewport_group,
                            );
                        }
                    });
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
