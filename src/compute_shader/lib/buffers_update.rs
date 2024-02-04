use bevy::{
    prelude::*,
    render::{render_asset::RenderAssets, render_resource::*, renderer::RenderDevice},
};
use bytemuck::Pod;

use super::buffers_interface::*;
use super::buffers_setup::*;

// Run when the buffers get updated
pub fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<super::buffers_setup::ComputePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    raytracer_image: Res<ComputeImage>,
    render_device: Res<RenderDevice>,
    compute_buffers: Res<ComputeBuffers>,
) {
    let view = gpu_images.get(&raytracer_image.0).unwrap();
    let texture_bind_group_entry = BindGroupEntry {
        binding: 0,
        resource: BindingResource::TextureView(&view.texture_view),
    };
    let mut bind_group_entries = vec![texture_bind_group_entry];

    // Compute buffers setup
    // -------------------
    let mut data_buffers = vec![];
    for buffer in compute_buffers.0.iter().cloned() {
        let buffer_data = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: &buffer.bytes,
            usage: BufferUsages::STORAGE,
        });
        data_buffers.push(buffer_data);
    }

    for i in 0..data_buffers.len() {
        let bind_group_entry = BindGroupEntry {
            binding: compute_buffers.0[i].binding,
            resource: data_buffers[i].as_entire_binding(),
        };
        bind_group_entries.push(bind_group_entry);
    }

    // Final bind group setup
    // ----------------------
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.texture_bind_group_layout,
        entries: bind_group_entries.as_slice(),
    });

    commands.insert_resource(ComputeBindGroup(bind_group));
}

impl ComputeBuffer {
    pub fn new<T>(binding: u32, vector: Vec<T>) -> Self
    where
        T: Pod,
    {
        ComputeBuffer {
            binding,
            bytes: bytemuck::cast_slice(&vector).to_vec(),
        }
    }
}

impl ComputeBuffers {
    pub fn new(value: Vec<ComputeBuffer>, world: &mut World) -> Self {
        ComputeBuffers(value)
    }

    pub fn set_value_at<T>(&mut self, binding: u32, new_value: Vec<T>, commands: &mut Commands)
    where
        T: Pod,
    {
        for buffer in self.0.iter_mut() {
            if buffer.binding == binding {
                *buffer = ComputeBuffer::new(binding, new_value);
                break;
            }
        }
    }
}

#[derive(Resource)]
pub struct ComputeBindGroup(pub BindGroup);
