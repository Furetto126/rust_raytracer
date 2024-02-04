use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDimension, TextureFormat, TextureUsages,
    },
};

use crate::{
    compute_shader::lib::buffers_setup::ComputeImage, BufferType, ComputeBuffers,
    ResizedWindowEvent, WindowSize,
};

pub fn update_window_buffers(
    mut commands: Commands,
    mut compute_buffers: ResMut<ComputeBuffers>,
    resolution: Res<WindowSize>,
) {
    let resolution = resolution.0;
    let aspect_ratio = resolution.x / resolution.y;

    compute_buffers.set_value_at(
        BufferType::ScreenResolution as u32,
        vec![resolution],
        &mut commands,
    );
    compute_buffers.set_value_at(
        BufferType::ScreenAspectRatio as u32,
        vec![aspect_ratio],
        &mut commands,
    );
}

pub fn update_camera_texture_size(
    mut compute_image: ResMut<ComputeImage>,
    images: Option<ResMut<Assets<Image>>>,
    resolution: Res<WindowSize>,
    mut sprites: Query<(&mut Sprite, &mut Handle<Image>)>,
    mut ev_window_resized: EventReader<ResizedWindowEvent>,
) {
    if !ev_window_resized.is_empty() {
        if let Some(mut images) = images {
            let mut image_texture = Image::new_fill(
                Extent3d {
                    width: resolution.0.x as u32,
                    height: resolution.0.y as u32,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &[0, 0, 0, 255],
                TextureFormat::Rgba8Unorm,
            );
            image_texture.texture_descriptor.usage = TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING
                | TextureUsages::TEXTURE_BINDING;
            let image_handle = images.add(image_texture);

            let mut sprite = sprites.single_mut();
            sprite.0.custom_size = Some(resolution.0);
            *sprite.1 = image_handle.clone();

            *compute_image = ComputeImage(image_handle);
        } else {
            println!("AAAAAAAA FUCK SOMETHING'S WRONG HERE HELP RIGGED");
        }
    }

    ev_window_resized.clear();
}
