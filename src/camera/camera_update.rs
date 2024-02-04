use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::{
    compute_shader::{compute_buffers::BufferType, lib::buffers_interface::ComputeBuffers},
    update_window_buffers,
    scene::{
        scene::Scene,
        spheres::*
    }
};

use self::sphere::Sphere;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneCamera>();
        app.add_systems(
            Update,
            (
                update_camera,
                move_camera,
                rotate_camera,
                update_camera_buffers,
                update_window_buffers,
                test,
            ),
        );
    }
}

#[derive(Resource, Copy, Clone)]
pub struct SceneCamera {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub inverse_view_matrix: Mat4,
}

impl Default for SceneCamera {
    fn default() -> Self {
        SceneCamera {
            position: Vec3::splat(0.0),
            front: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            inverse_view_matrix: Mat4::default(),
        }
    }
}

fn update_camera(mut camera: ResMut<SceneCamera>) {
    camera.right = Vec3::normalize(Vec3::cross(camera.front, camera.up));
    camera.inverse_view_matrix = Mat4::inverse(&Mat4::look_at_lh(
        camera.position,
        camera.position + camera.front,
        camera.up,
    ));
}

fn move_camera(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut camera: ResMut<SceneCamera>,
) {
    let move_button = MouseButton::Middle;
    if input_mouse.pressed(move_button) {
        let cursor_offset: Vec2 = ev_motion.iter().map(|ev| ev.delta).sum();

        let camera_speed = 0.01;
        let tangent = Vec3::normalize(Vec3::new(camera.front.z, 0.0, -camera.front.x));
        let bitangent = Vec3::cross(camera.front, tangent);

        let movement = (tangent * cursor_offset.x + bitangent * cursor_offset.y) * camera_speed;
        camera.position += movement;
    } else {
        let scroll_offset: f32 = ev_scroll.iter().map(|ev| ev.y).sum();
        let zoom_speed = 5.0;

        let camera_clone = camera.clone();
        camera.position += camera_clone.front * scroll_offset * zoom_speed;
    }

    ev_motion.clear();
    ev_scroll.clear();
}

fn rotate_camera(
    mut ev_motion: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut camera: ResMut<SceneCamera>,
) {
    let rotate_button = MouseButton::Right;
    if !input_mouse.pressed(rotate_button) {
        return;
    }

    let mouse_sensitivity = 0.0001;
    let cursor_offset = ev_motion.iter().map(|ev| ev.delta).sum::<Vec2>() * -mouse_sensitivity;
    let rotation_speed = 5.0;

    camera.front = Vec3::normalize(camera.front);
    let camera_clone = camera.clone();

    let original_front = Vec3::new(0.0, 0.0, 1.0);

    camera.front +=
        (camera_clone.right * cursor_offset.x - camera_clone.up * cursor_offset.y) * rotation_speed;
    camera.front = Vec3::normalize(camera.front);

    let angle = Vec3::dot(camera.front, original_front).acos() * 180.0 / PI;
    println!("ANGLE IS: {angle}");
}

fn test(
    mut commands: Commands,
    input_mouse: Res<Input<MouseButton>>,
    mut compute_buffers: ResMut<ComputeBuffers>,
    camera: Res<SceneCamera>,
    mut scene: ResMut<Scene>,
) {
    /*if input_mouse.pressed(MouseButton::Left) {
        /*scene.add_sphere(
            Sphere::new(camera.position, 2.0)
        );*/
        scene.spheres.push(Sphere::new(camera.position + Vec3::new(0.0, 0.0, 2.0), 0.1));
    }*/
}

fn update_camera_buffers(
    mut commands: Commands,
    mut compute_buffers: ResMut<ComputeBuffers>,
    camera: Res<SceneCamera>,
) {
    compute_buffers.set_value_at(
        BufferType::CameraPosition as u32,
        vec![camera.position],
        &mut commands,
    );

    compute_buffers.set_value_at(
        BufferType::CameraDirection as u32,
        vec![camera.front],
        &mut commands,
    );

    compute_buffers.set_value_at(
        BufferType::InverseViewMatrix as u32,
        vec![camera.inverse_view_matrix],
        &mut commands,
    );

    //println!("CAMERA POSITION IS: {}", camera.position);
    //println!("CAMERA DIRECTION IS: {}", camera.front);
}
