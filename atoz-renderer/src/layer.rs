use wgpu::{util::DeviceExt, Device};

use crate::pipeline::image::ImageResource;

use super::pipeline::{circle::CircleInstance, rect::RectInstance, triangle::TriangleInstance};

#[derive(Debug)]
pub struct Layer {
    pub circles: Vec<CircleInstance>,
    pub rects: Vec<RectInstance>,
    pub triangles: Vec<TriangleInstance>,
    pub images: Vec<ImageResource>,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            circles: vec![],
            rects: vec![],
            triangles: vec![],
            images: vec![],
        }
    }
}

impl Layer {
    pub fn push_circle(&mut self, instance: CircleInstance) {
        self.circles.push(instance);
    }

    pub fn get_circle_buffer(&self, device: &Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("layer.circle.buffer"),
            contents: bytemuck::cast_slice(&self.circles),
            usage: wgpu::BufferUsages::VERTEX,
        });
    }

    pub fn push_rect(&mut self, instance: RectInstance) {
        self.rects.push(instance);
    }

    pub fn get_rect_buffer(&self, device: &Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("layer.rect.buffer"),
            contents: bytemuck::cast_slice(&self.rects),
            usage: wgpu::BufferUsages::VERTEX,
        });
    }

    pub fn push_triangle(&mut self, instance: TriangleInstance) {
        self.triangles.push(instance);
    }

    pub fn get_triangle_buffer(&self, device: &Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("layer.triangle.buffer"),
            contents: bytemuck::cast_slice(&self.triangles),
            usage: wgpu::BufferUsages::VERTEX,
        });
    }

    pub fn push_image(&mut self, instance: ImageResource) {
        self.images.push(instance);
    }

    pub fn get_image_buffers(&self, device: &Device) -> Vec<(wgpu::BindGroup, wgpu::Buffer, u32)> {
        return self
            .images
            .iter()
            .map(|r| {
                return (
                    r.get_bind_group(device),
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("layer.image.buffer"),
                        contents: bytemuck::cast_slice(&r.instances),
                        usage: wgpu::BufferUsages::VERTEX,
                    }),
                    r.instances.len() as u32,
                );
            })
            .collect();
    }
}
