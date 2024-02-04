use super::buffers_setup::{setup, ComputeRenderStartPlugin};
use bevy::prelude::*;

pub struct ComputeBuffersPlugin;

impl Plugin for ComputeBuffersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComputeRenderStartPlugin)
            .add_systems(Startup, setup);
    }
}

#[derive(Clone)]
pub struct ComputeBuffer {
    pub binding: u32,
    pub bytes: Vec<u8>,
}

#[derive(Resource, Clone)]
pub struct ComputeBuffers(pub Vec<ComputeBuffer>);

#[derive(Resource)]
pub struct ShaderPath(pub &'static str);
