use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, Device};

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct Viewport {
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn new(width: f32, height: f32) -> Self {
        return Self { width, height };
    }

    pub fn layout(device: &Device) -> BindGroupLayout {
        return device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("viewport.layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
    }

    pub fn get_buffer(&mut self, device: &Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Viewport Buffer"),
            contents: bytemuck::cast_slice(&[self.clone()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
    }

    pub fn get_bind_group(&mut self, device: &Device) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport.group"),
            layout: &Self::layout(device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.get_buffer(device).as_entire_binding(),
            }],
        })
    }
}
