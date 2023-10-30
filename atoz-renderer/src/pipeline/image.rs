use std::mem;

use bytemuck::{Pod, Zeroable};
use image::{DynamicImage, GenericImageView};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device, Queue, RenderPass, RenderPipeline,
    Sampler, Texture, TextureView,
};

use crate::viewport::Viewport;

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct ImageVertex {
    pub position: [f32; 2],
}

impl ImageVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        };
    }
}

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct ImageInstance {
    pub start: [f32; 2],
    pub size: [f32; 2],
}

impl ImageInstance {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![1 => Float32x2, 2 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        };
    }

    pub fn get_vertex_buffer(device: &Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("image.vertex"),
            contents: bytemuck::cast_slice(&IMAGE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
    }

    pub fn get_index_buffer(device: &Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("image.index"),
            contents: bytemuck::cast_slice(&IMAGE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
    }

    pub fn new(start: [f32; 2], size: [f32; 2]) -> Self {
        return Self { start, size };
    }
}

const IMAGE_VERTICES: [ImageVertex; 4] = [
    ImageVertex {
        position: [0.0, 0.0],
    },
    ImageVertex {
        position: [0.0, 1.0],
    },
    ImageVertex {
        position: [1.0, 1.0],
    },
    ImageVertex {
        position: [1.0, 0.0],
    },
];

const IMAGE_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

#[derive(Debug)]
pub struct ImageResource {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub instances: Vec<ImageInstance>,
}

impl ImageResource {
    pub fn new(image: DynamicImage, device: &Device, queue: &Queue) -> Self {
        let data = image.to_rgba8();
        let dimension = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimension.0,
            height: dimension.1,
            depth_or_array_layers: 1,
        };

        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("image.texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimension.0),
                rows_per_image: Some(dimension.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            instances: vec![],
        }
    }

    fn layout(device: &Device) -> BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("image.layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
    }

    pub fn get_bind_group(&self, device: &Device) -> BindGroup {
        return device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("image.group"),
            layout: &Self::layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
    }

    pub fn push_instance(&mut self, instance: ImageInstance) {
        self.instances.push(instance);
    }
}

#[derive(Debug)]
pub struct ImagePipeline {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl ImagePipeline {
    pub fn new(device: &Device, format: wgpu::TextureFormat) -> Self {
        let image_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("image.shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader/image.wgsl").into()),
        });

        let vertex_buffer = ImageInstance::get_vertex_buffer(device);
        let index_buffer = ImageInstance::get_index_buffer(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("image.pipeline.layout"),
            bind_group_layouts: &[&Viewport::layout(device), &ImageResource::layout(device)],
            push_constant_ranges: &[],
        });

        let pipeline: RenderPipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("image.pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &image_shader,
                    entry_point: "vs_main",
                    buffers: &[ImageVertex::desc(), ImageInstance::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &image_shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        return Self {
            pipeline,
            vertex_buffer,
            index_buffer,
        };
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        instance_buffer: &'a Buffer,
        instance_count: u32,
        texture: &'a BindGroup,
        viewport: &'a BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, viewport, &[]);
        render_pass.set_bind_group(1, texture, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..IMAGE_INDICES.len() as u32, 0, 0..instance_count);
    }
}
