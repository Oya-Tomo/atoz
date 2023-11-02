pub mod layer;
pub mod pipeline;
pub mod viewport;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use env_logger;
    use wgpu::InstanceDescriptor;
    use winit::event_loop::EventLoopBuilder;
    use winit::{
        dpi::PhysicalSize,
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
        platform::wayland::EventLoopBuilderExtWayland,
        window::WindowBuilder,
    };

    use crate::layer::Layer;
    use crate::pipeline::image::{ImageInstance, ImagePipeline, ImageResource};
    use crate::{
        pipeline::{
            circle::{CircleInstance, CirclePipeline},
            rect::{RectInstance, RectPipeline},
            triangle::{TriangleInstance, TrianglePipeline},
        },
        viewport::Viewport,
    };

    #[test]
    fn test_pipeline() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        env_logger::init();
        let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let _ = window.set_inner_size(PhysicalSize::new(800, 600));

        let size = window.inner_size();
        let mut viewport = Viewport::new(size.width as f32, size.height as f32);

        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = rt
            .block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .unwrap();

        let (device, queue) = rt
            .block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            ))
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let resize = |device: &wgpu::Device,
                      surface: &wgpu::Surface,
                      config: &mut wgpu::SurfaceConfiguration,
                      new_size: winit::dpi::PhysicalSize<u32>| {
            if new_size.width > 0 && new_size.height > 0 {
                config.width = new_size.width;
                config.height = new_size.height;
                surface.configure(&device, &config);
            }
        };

        let mut draw_layer = Layer::default();
        draw_layer.push_circle(CircleInstance::fill(
            [120.0, 120.0],
            70.0,
            [1.0, 0.0, 0.0, 1.0],
        ));
        draw_layer.push_rect(RectInstance::fill(
            [100.0, 260.0],
            [200.0, 100.0],
            [5.0, 5.0, 5.0, 5.0],
            [0.0, 0.0, 1.0, 1.0],
        ));
        draw_layer.push_triangle(TriangleInstance::fill(
            [330.0, 70.0],
            [250.0, 200.0],
            [410.0, 200.0],
            [0.0, 1.0, 0.0, 1.0],
        ));

        let image_bytes = include_bytes!("test/assets/rust.png");
        let mut image_res = ImageResource::new(
            image::load_from_memory(image_bytes).unwrap(),
            &device,
            &queue,
        );
        image_res.push_instance(ImageInstance::new([100.0, 100.0], [200.0, 200.0]));
        draw_layer.push_image(image_res);

        let mut circle_pipeline = CirclePipeline::new(&device, surface_format);
        let mut rect_pipeline = RectPipeline::new(&device, surface_format);
        let mut triangle_pipeline = TrianglePipeline::new(&device, surface_format);
        let mut image_pipeline = ImagePipeline::new(&device, surface_format);

        let render = |device: &wgpu::Device,
                      queue: &wgpu::Queue,
                      surface: &wgpu::Surface,
                      circle_pipeline: &mut CirclePipeline,
                      rect_pipeline: &mut RectPipeline,
                      triangle_pipeline: &mut TrianglePipeline,
                      image_pipeline: &mut ImagePipeline,
                      layer: &Layer,
                      viewport_bind_group: &wgpu::BindGroup| {
            let output = surface.get_current_texture().unwrap();
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            let rect_instance_buffer = layer.get_rect_buffer(device);
            let circle_instance_buffer = layer.get_circle_buffer(device);
            let triangle_instance_buffer = layer.get_triangle_buffer(device);
            let image_instance_buffers = layer.get_image_buffers(device);

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                rect_pipeline.render(
                    &mut render_pass,
                    &rect_instance_buffer,
                    layer.rects.len() as _,
                    viewport_bind_group,
                );

                circle_pipeline.render(
                    &mut render_pass,
                    &circle_instance_buffer,
                    layer.circles.len() as _,
                    viewport_bind_group,
                );

                triangle_pipeline.render(
                    &mut render_pass,
                    &triangle_instance_buffer,
                    layer.triangles.len() as _,
                    viewport_bind_group,
                );

                image_instance_buffers.iter().for_each(
                    |(texture, instance_buffer, instance_count)| {
                        image_pipeline.render(
                            &mut render_pass,
                            instance_buffer,
                            instance_count.clone(),
                            texture,
                            viewport_bind_group,
                        )
                    },
                );
            }

            queue.submit(std::iter::once(encoder.finish()));
            output.present();
        };

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    resize(&device, &surface, &mut config, *physical_size);
                    viewport =
                        Viewport::new(physical_size.width as f32, physical_size.height as f32);
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size,
                } => {
                    resize(&device, &surface, &mut config, **new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let i = Instant::now();
                render(
                    &device,
                    &queue,
                    &surface,
                    &mut circle_pipeline,
                    &mut rect_pipeline,
                    &mut triangle_pipeline,
                    &mut image_pipeline,
                    &draw_layer,
                    &mut viewport.get_bind_group(&device),
                );
                println!("{}", i.elapsed().as_millis());
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        });
    }
}
