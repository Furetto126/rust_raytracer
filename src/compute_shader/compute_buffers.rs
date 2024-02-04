use bevy::{
    prelude::*,
    render::{MainWorld, RenderApp},
};

use crate::scene::scene::Scene;

use super::lib::buffers_interface::*;

pub struct ComputeBuffersUpdatePlugin;

#[allow(dead_code)]
pub enum BufferType {
    MainTexture = 0,
    CameraPosition = 1,
    CameraDirection = 2,
    ScreenResolution = 3,
    ScreenAspectRatio = 4,
    InverseViewMatrix = 5,
    Spheres = 6,
}

impl Plugin for ComputeBuffersUpdatePlugin {
    fn build(&self, app: &mut App) {
        init_buffers(&mut app.world);

        let render_app = app.sub_app_mut(RenderApp);
        init_buffers(&mut render_app.world);

        render_app
            .add_systems(ExtractSchedule, update_buffers)
            .insert_resource(ShaderPath("shaders/raytracer.wgsl"));
    }
}

pub fn update_buffers(mut commands: Commands, main_world: Res<MainWorld>) {
    let compute_buffers_main_world = main_world.get_resource::<ComputeBuffers>().unwrap().clone();
    commands.insert_resource(compute_buffers_main_world);
}

pub fn init_buffers(world: &mut World) {
    let compute_buffers = ComputeBuffers::new(
        vec![
            ComputeBuffer::new(BufferType::CameraPosition as u32, vec![Vec3::splat(0.0)]),
            ComputeBuffer::new(BufferType::CameraDirection as u32, vec![Vec3::splat(0.0)]),
            ComputeBuffer::new(
                BufferType::ScreenResolution as u32,
                vec![1920.0 as f32, 1080.0],
            ),
            ComputeBuffer::new(BufferType::InverseViewMatrix as u32, vec![Mat4::default()]),
            ComputeBuffer::new(BufferType::ScreenAspectRatio as u32, vec![1920.0 as f32 / 1080.0]),
            ComputeBuffer::new(BufferType::Spheres as u32, Scene::default().spheres),
        ],
        world,
    );

    world.insert_resource(compute_buffers);
}
