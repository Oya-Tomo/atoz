use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device, RenderPass, RenderPipeline};

use crate::viewport::Viewport;

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct TriangleVertex {
    pub index: u32,
}

impl TriangleVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Uint32];

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
pub struct TriangleInstance {
    pub point1: [f32; 2],
    pub point2: [f32; 2],
    pub point3: [f32; 2],
    pub thickness: f32,
    pub fill_color: [f32; 4],
    pub line_color: [f32; 4],
}

impl TriangleInstance {
    const ATTRIBUTES: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        1 => Float32x2,
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32,
        5 => Float32x4,
        6 => Float32x4,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        };
    }

    pub fn get_vertex_buffer(device: &Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("triangle_instance.vertex"),
            contents: bytemuck::cast_slice(&TRIANGLE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
    }

    pub fn get_index_buffer(device: &Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("triangle_instance.index"),
            contents: bytemuck::cast_slice(&TRIANGLE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
    }

    pub fn new(
        point1: [f32; 2],
        point2: [f32; 2],
        point3: [f32; 2],
        thickness: u32,
        fill_color: [f32; 4],
        line_color: [f32; 4],
    ) -> Self {
        return Self {
            point1,
            point2,
            point3,
            thickness: thickness as f32,
            fill_color,
            line_color,
        };
    }

    pub fn fill(point1: [f32; 2], point2: [f32; 2], point3: [f32; 2], color: [f32; 4]) -> Self {
        return Self {
            point1,
            point2,
            point3,
            thickness: 1.0,
            fill_color: color,
            line_color: color,
        };
    }

    pub fn outline(
        point1: [f32; 2],
        point2: [f32; 2],
        point3: [f32; 2],
        thickness: u32,
        color: [f32; 4],
    ) -> Self {
        assert!(
            thickness > 0,
            "TriangleInstance : thickness must be bigger than 0."
        );
        return Self {
            point1,
            point2,
            point3,
            thickness: thickness as f32,
            fill_color: [0.0, 0.0, 0.0, 0.0],
            line_color: color,
        };
    }
}

const TRIANGLE_VERTICES: [TriangleVertex; 3] = [
    TriangleVertex { index: 0 },
    TriangleVertex { index: 1 },
    TriangleVertex { index: 2 },
];

const TRIANGLE_INDICES: [u16; 3] = [0, 1, 2];

#[derive(Debug)]
pub struct TrianglePipeline {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl TrianglePipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let triangle_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle.shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader/triangle.wgsl").into()),
        });

        let vertex_buffer = TriangleInstance::get_vertex_buffer(device);
        let index_buffer = TriangleInstance::get_index_buffer(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("triangle.pipeline.layout"),
            bind_group_layouts: &[&Viewport::layout(device)],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle.pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &triangle_shader,
                entry_point: "vs_main",
                buffers: &[TriangleVertex::desc(), TriangleInstance::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &triangle_shader,
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
        viewport: &'a BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, viewport, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..TRIANGLE_INDICES.len() as u32, 0, 0..instance_count);
    }
}
