use atoz_renderer::{
    layer::{Layer, LayerBuffer},
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
    rect_pipeline: RectPipeline,
    triangle_pipeline: TrianglePipeline,
    circle_pipeline: CirclePipeline,
    image_pipeline: ImagePipeline,
}

impl Context {
    pub async fn init(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();
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

        let rect_pipeline = RectPipeline::new(&device, config.format);
        let triangle_pipeline = TrianglePipeline::new(&device, config.format);
        let circle_pipeline = CirclePipeline::new(&device, config.format);
        let image_pipeline = ImagePipeline::new(&device, config.format);

        return Self {
            window,
            device,
            queue,
            surface,
            config,
            layers: vec![],
            rect_pipeline,
            triangle_pipeline,
            circle_pipeline,
            image_pipeline,
        };
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn set_min_size<S: Into<Size>>(&mut self, size: S) {
        self.window.set_min_inner_size(Some(size));
    }

    pub fn set_max_size<S: Into<Size>>(&mut self, size: S) {
        self.window.set_max_inner_size(Some(size));
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

        let viewport_group = Viewport::new(self.config.width as _, self.config.height as _)
            .get_bind_group(&self.device);

        let layer_buffers = self
            .layers
            .iter()
            .map(|layer| layer.get_all_buffers(&self.device))
            .collect::<Vec<LayerBuffer>>();

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("context.render.render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            layer_buffers.iter().for_each(|buffer| {
                self.rect_pipeline.render(
                    &mut render_pass,
                    &buffer.rect_buffer,
                    buffer.rect_count as _,
                    &viewport_group,
                );

                self.triangle_pipeline.render(
                    &mut render_pass,
                    &buffer.triangle_buffer,
                    buffer.triangle_count as _,
                    &viewport_group,
                );

                self.circle_pipeline.render(
                    &mut render_pass,
                    &buffer.circle_buffer,
                    buffer.circle_count as _,
                    &viewport_group,
                );

                buffer.image_buffers.iter().for_each(
                    |(texture, instance_buffer, instance_count)| {
                        if instance_count > &0 {
                            self.image_pipeline.render(
                                &mut render_pass,
                                instance_buffer,
                                instance_count.clone(),
                                texture,
                                &viewport_group,
                            );
                        }
                    },
                );
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
