use bevy::prelude::*;

use crate::{scene::spheres::sphere::*, BufferType, ComputeBuffers};

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scene>();
        app.add_systems(Update, update_scene_buffers);
    }
}

#[derive(Resource, Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            spheres: init_spheres(),
        }
    }
}

impl Scene {
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

fn update_scene_buffers(
    mut commands: Commands,
    scene: Res<Scene>,
    mut compute_buffers: ResMut<ComputeBuffers>,
) {
    compute_buffers.set_value_at(
        BufferType::Spheres as u32,
        scene.spheres.clone(),
        &mut commands,
    );
}
